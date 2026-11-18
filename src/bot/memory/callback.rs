//! Recall the latest news about a person already mentioned in the conversation.
use crate::bot::{Memory, compose};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    ASK,
    r"^(?:(?:and|so|but)\s+)?(?:how(?:'s|\s+is|\s+are|\s+was|\s+were)\s+(?P<who>he|she|they|him|her|them)\s*(?:doing|getting\s+on|going|coping|holding\s+up|been)?|how\s+(?:has|have)\s+(?P<been>he|she|they)\s+been|is\s+(?P<better>he|she|they)\s+(?:any\s+)?better)\s*(?:now|today|then|since|at\s+the\s+moment)?\s*\?*\s*$"
);
pattern!(
    NAMED_ASK,
    r"^(?:(?:and|so)\s+)?(?:how(?:'s|\s+is|\s+was)\s+(?:(?:your|my)\s+)?(?P<who>[a-z]{2,})|any\s+news\s+(?:about|on|of)\s+(?:(?:your|my)\s+)?(?P<news>[a-z]{2,})|how\s+did\s+it\s+go\s+with\s+(?:(?:your|my)\s+)?(?P<went>[a-z]{2,}))\s*(?:doing|getting\s+on|now|then|today)?\s*\?*\s*$"
);
pattern!(RELATIVE, r"\b(?:my|our)\s+([a-z]{3,})\b");
pattern!(
    AGE,
    r"^(?:is|was|turned|is\s+now|just\s+turned)\s+\d{1,3}\b"
);
pattern!(
    MEDICAL_EPISODE,
    r"(?i)\b(?:hospital|hospitalis\w*|hospitaliz\w*|surgery|surgical|operation|a&?e|casualty|intensive care|icu|coronary care|hospice|care home|nursing home|the ward|on a ward|in a ward)\b"
);

// Source vocabulary from memory.callback.THE_RELATIONS and memory.facts.HE/SHE.
const RELATIONS: &[&str] = &[
    "aunt",
    "auntie",
    "aunty",
    "bf",
    "bfs",
    "boyfriend",
    "brother",
    "cousin",
    "dad",
    "daughter",
    "father",
    "fiance",
    "fiancee",
    "friend",
    "gf",
    "gfs",
    "girlfriend",
    "gran",
    "grandad",
    "grandchild",
    "granddad",
    "granddaughter",
    "grandfather",
    "grandma",
    "grandmom",
    "grandmother",
    "grandmum",
    "grandpa",
    "grandson",
    "granny",
    "hubby",
    "husband",
    "mam",
    "missus",
    "mom",
    "mother",
    "mum",
    "nan",
    "nana",
    "nanna",
    "neighbor",
    "neighbour",
    "nephew",
    "niece",
    "parents",
    "partner",
    "pops",
    "sister",
    "son",
    "stepbrother",
    "stepchild",
    "stepdad",
    "stepdaughter",
    "stepfather",
    "stepmom",
    "stepmother",
    "stepmum",
    "stepsister",
    "stepson",
    "twins",
    "uncle",
    "wife",
];

fn pronoun(relation: &str) -> &'static str {
    match relation {
        "boss" | "boy" | "boyfriend" | "brother" | "brother in law" | "brother in laws"
        | "colleague" | "cousin" | "dad" | "father" | "father in law" | "father in laws"
        | "fiance" | "flatmate" | "friend" | "godson" | "grandad" | "granddad" | "grandfather"
        | "grandpa" | "grandson" | "housemate" | "husband" | "landlady" | "landlord" | "lodger"
        | "manager" | "mate" | "neighbor" | "neighbour" | "nephew" | "partner" | "roommate"
        | "son" | "son in law" | "stepbrother" | "stepdad" | "stepfather" | "stepson" | "uncle" => {
            "he"
        }
        "aunt" | "auntie" | "aunty" | "daughter" | "daughter in law" | "fiancee" | "girl"
        | "girlfriend" | "goddaughter" | "gran" | "granddaughter" | "grandma" | "grandmom"
        | "grandmother" | "grandmum" | "granny" | "mom" | "mother" | "mother in law"
        | "mother in laws" | "mum" | "nan" | "nana" | "nanna" | "niece" | "sister"
        | "sister in law" | "sister in laws" | "stepdaughter" | "stepmom" | "stepmother"
        | "stepmum" | "stepsister" | "wife" => "she",
        _ => "they",
    }
}

struct Person<'a> {
    relation: &'static str,
    name: Option<&'a str>,
    age: Option<u8>,
}

fn words(line: &str) -> impl Iterator<Item = &str> {
    line.split(crate::text::is_space)
        .filter(|part| !part.is_empty())
}

fn flat_lower(line: &str) -> String {
    words(line).collect::<Vec<_>>().join(" ").to_lowercase()
}

fn last_person(memory: &Memory) -> Option<Person<'_>> {
    if let Some((relation, name, age)) = memory.personal.family_people().next_back() {
        return Some(Person {
            relation,
            name,
            age,
        });
    }
    let mut named = Vec::new();
    for turn in &memory.turns {
        for found in RELATIVE.captures_iter(&flat_lower(&turn.user)) {
            if let Ok(index) = RELATIONS.binary_search(&&found[1]) {
                let relative = RELATIONS[index];
                if !named.contains(&relative) {
                    named.push(relative);
                }
            }
        }
    }
    Some(Person {
        relation: named.pop()?,
        name: None,
        age: None,
    })
}

fn latest_news(person: &Person<'_>, memory: &Memory) -> Option<String> {
    let name = person.name.map(str::to_lowercase);
    let people = [
        Some(person.relation),
        name.as_deref(),
        Some(pronoun(person.relation)),
    ];
    let mentions = people.map(|reference| {
        reference.map(|word| {
            Regex::new(&format!(r"\b(?:my|our|the)?\s*{}\b", regex::escape(word)))
                .expect("escaped relative or name")
        })
    });
    for turn in memory
        .turns
        .iter()
        .rev()
        .filter(|turn| !turn.user.contains('?'))
    {
        let line = flat_lower(&turn.user);
        for mention in mentions.iter().flatten() {
            let Some(found) = mention.find(&line) else {
                continue;
            };
            let rest = line[found.end()..].trim_matches([' ', ',', '.', ';', ':', '!', '-']);
            let mut parts = rest.split(' ');
            let verb = parts.next()?;
            if matches!(
                verb,
                "is" | "was"
                    | "had"
                    | "has"
                    | "have"
                    | "went"
                    | "got"
                    | "kept"
                    | "said"
                    | "wants"
                    | "wanted"
                    | "needs"
                    | "keeps"
                    | "lives"
                    | "does"
                    | "did"
                    | "came"
                    | "started"
                    | "turned"
                    | "seems"
            ) && !AGE.is_match(rest)
                && parts.take(9).count() < 9
            {
                return Some(rest.to_owned());
            }
        }
    }
    None
}

fn capitalize(line: &str) -> String {
    let mut chars = line.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first
        .to_uppercase()
        .chain(chars.flat_map(char::to_lowercase))
        .collect()
}

fn choose(mut pool: Vec<String>, memory: &Memory) -> String {
    let fresh: Vec<_> = pool
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            (!memory
                .turns
                .iter()
                .any(|turn| words(&turn.bot).eq(line.split(' '))))
            .then_some(index)
        })
        .collect();
    let mut rng = memory.rng.borrow_mut();
    let index = if fresh.is_empty() {
        compose::draw(&mut rng.0, pool.len())
    } else {
        fresh[compose::draw(&mut rng.0, fresh.len())]
    };
    pool.swap_remove(index)
}

struct Update<'a> {
    person: Person<'a>,
    news: Option<String>,
}

fn update<'a>(raw: &str, memory: &'a Memory) -> Option<Update<'a>> {
    if memory.turns.is_empty() {
        return None;
    }
    // The upstream expressions only enter this lane for an ask on these words.
    let first = words(raw).next()?;
    if !["how", "how's", "is", "any", "so", "and", "but"]
        .iter()
        .any(|word| first.eq_ignore_ascii_case(word))
    {
        return None;
    }
    let ask = flat_lower(raw);
    let pointed = ASK.captures(&ask);
    let named = pointed
        .is_none()
        .then(|| NAMED_ASK.captures(&ask))
        .flatten();
    let (groups, pronominal) = if let Some(groups) = pointed {
        (groups, true)
    } else {
        (named?, false)
    };
    let word = groups.iter().skip(1).flatten().next()?.as_str();
    if !pronominal && RELATIONS.binary_search(&word).is_err() {
        return None;
    }
    if MEDICAL_EPISODE.is_match(raw)
        || memory
            .turns
            .iter()
            .rev()
            .filter(|turn| words(&turn.user).next().is_some())
            .take(6)
            .any(|turn| {
                let line = words(&turn.user).collect::<Vec<_>>().join(" ");
                MEDICAL_EPISODE.is_match(&line)
            })
    {
        return None;
    }
    let person = last_person(memory)?;
    let pronoun = pronoun(person.relation);
    if pronominal {
        if matches!(word, "he" | "she") && word != pronoun {
            return None;
        }
    } else if person.relation != word && person.relation != word.trim_end_matches('s') {
        return None;
    }
    let news = latest_news(&person, memory);
    (news.is_some() || person.age.is_some_and(|age| age > 0)).then_some(Update { person, news })
}

pub(super) fn has_update(raw: &str, memory: &Memory) -> bool {
    update(raw, memory).is_some()
}

pub(crate) fn reply(raw: &str, memory: &Memory) -> Option<String> {
    let Update { person, news } = update(raw, memory)?;
    let who = person
        .name
        .map_or_else(|| format!("your {}", person.relation), capitalize);
    let who_cap = capitalize(&who);
    if let Some(news) = news {
        let pool = [
            format!("You said {who} {news}."),
            format!("Last you said, {who} {news}."),
            format!("{who_cap} {news}, you said."),
        ]
        .into_iter()
        .flat_map(|lead| {
            [
                "Any word since then?",
                "Any news since?",
                "Has there been any change?",
                "Anything new since?",
            ]
            .into_iter()
            .map(move |since| format!("{lead} {since}"))
        })
        .collect();
        return Some(choose(pool, memory));
    }
    let age = person.age.filter(|age| *age > 0)?;
    let pronoun = pronoun(person.relation);
    Some(choose(
        vec![
            format!("{who_cap} is {age} - what has {pronoun} been saying?"),
            format!(
                "{who_cap} is {age}, and that is all I have. What is {pronoun} up to at the moment?"
            ),
            format!("{who_cap} is {age} - has {pronoun} been in touch?"),
        ],
        memory,
    ))
}

#[cfg(test)]
#[test]
fn holds_a_relative_and_leaves_medical_episodes_to_their_reader() {
    use crate::bot::{Bot, Exchange};

    let mut bot = Bot::new(Some("0")).unwrap();
    assert_eq!(reply("How is she doing?", &bot.memory), None);
    bot.memory.turns.push(Exchange {
        user: "My mum started a new job.".into(),
        bot: String::new(),
        module: "",
    });
    assert_eq!(reply("How is he doing?", &bot.memory), None);
    assert_eq!(
        reply("How is she doing?", &bot.memory).as_deref(),
        Some("Last you said, your mum started a new job. Has there been any change?")
    );
    bot.memory.turns.push(Exchange {
        user: "She is in hospital.".into(),
        bot: String::new(),
        module: "",
    });
    assert_eq!(reply("Any news about mum?", &bot.memory), None);
}

#[cfg(test)]
#[test]
fn repeated_callback_questions_keep_the_named_person_in_view() {
    let mut bot = crate::Bot::new(Some("0")).unwrap();
    bot.turn("My mum started a new job.");
    bot.turn("How is she doing?");
    bot.turn("Any news about mum?");
    let turn = bot.turn("How has she been?");
    assert_eq!(turn.module, Some("callback_about"));
    assert_eq!(
        turn.reply,
        "You said your mum started a new job. Any word since then?"
    );
}
