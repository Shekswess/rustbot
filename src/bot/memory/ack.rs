//! Say the speaker's words back from the listener's perspective.
#[path = "ack_opening.rs"]
mod ack_opening;
#[path = "ack_plan.rs"]
mod ack_plan;
use crate::bot::{Memory, Reading, compose, understanding::line::possessive_me};
use fancy_regex::Regex as Fancy;
use regex::{Captures, Regex};
use std::{borrow::Cow, sync::LazyLock};
#[path = "ack_context.rs"]
mod context;
#[path = "ack_data.rs"]
mod data;
pub(crate) use context::Topics;
use data::*;

fn has(words: &[&str], word: &str) -> bool {
    words.binary_search(&word).is_ok()
}
fn lookup<'a>(table: &'a [(&str, &str)], word: &str) -> Option<&'a str> {
    table
        .binary_search_by_key(&word, |p| p.0)
        .ok()
        .map(|i| table[i].1)
}
fn upper_first(s: &str) -> String {
    let mut chars = s.chars();
    chars
        .next()
        .map_or_else(String::new, |c| c.to_uppercase().chain(chars).collect())
}
fn case(original: &str, replacement: &str) -> String {
    if original.starts_with(char::is_uppercase) {
        upper_first(replacement)
    } else {
        replacement.into()
    }
}

static QUOTED: LazyLock<Fancy> =
    LazyLock::new(|| Fancy::new(r#""[^"]*"|“[^”]*”|(?<!\w)'[^']*'(?!\w)"#).unwrap());
static RELATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b([Mm]y|[Oo]ur) ([a-z]+s) ([A-Za-z0-9']+)").unwrap());
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new("[a-z']+").unwrap());
static BOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b([a-z']+) +\b(you|your)\b").unwrap());
static AFTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[\s,]*([a-z']+)").unwrap());
static ME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bme\b").unwrap());
static PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(i'm|i am|i've|i have|i was not|i wasn't|i wasnt|i was|i were|we wasnt|we was|im|ive|my|mine|myself|me|we|our|ourselves|us|i)\b").unwrap()
});
static APOSTROPHES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?i)\b(?:{})\b",
        APOSTROPHE.iter().map(|x| x.0).collect::<Vec<_>>().join("|")
    ))
    .unwrap()
});

fn relation_possessive(line: &str) -> Cow<'_, str> {
    RELATION.replace_all(line, |c: &Captures<'_>| {
        let stem = &c[2][..c[2].len() - 1];
        let thing = c[3].to_ascii_lowercase();
        let with_somebody = has(WHEN_OR_WHERE, &thing)
            && line[..c.get(0).unwrap().start()]
                .split_whitespace()
                .next_back()
                .is_some_and(|w| has(WITH_SOMEBODY, &w.to_ascii_lowercase()));
        if !has(RELATIONS, stem) || has(NOT_OWNED, &thing) || with_somebody {
            c[0].to_string()
        } else {
            format!("{} {stem}'s {}", &c[1], &c[3])
        }
    })
}

fn person(word: &str) -> &'static str {
    match word {
        "i'm" | "im" => "you're",
        "i am" => "you are",
        "i've" | "ive" => "you've",
        "i have" => "you have",
        "i was not" => "you were not",
        "i wasn't" | "i wasnt" | "we wasnt" => "you weren't",
        "i was" | "i were" | "we was" => "you were",
        "my" | "our" => "your",
        "mine" => "yours",
        "myself" => "yourself",
        "ourselves" => "yourselves",
        _ => "you",
    }
}

fn flip(part: &str) -> String {
    let part = relation_possessive(part);
    let part = relation_possessive(&part);
    let low = part.to_lowercase();
    let mine = WORD
        .find_iter(&low)
        .map(|m| m.as_str())
        .find(|w| has(SUBJECT_HEADS, w))
        .is_some_and(|w| has(FIRST_PERSON_HEADS, w));
    let marked = if mine {
        BOT.replace_all(&part, |c: &Captures<'_>| {
            let v = c[1].to_ascii_lowercase();
            if !has(BOT_OBJECT, &v)
                || (matches!(&*v, "thank" | "thanks")
                    && !AFTER
                        .captures(&part[c.get(0).unwrap().end()..])
                        .is_some_and(|c| matches!(&c[1], "for" | "again" | "so" | "too")))
            {
                return c[0].to_string();
            }
            let suffix =
                if c[2].eq_ignore_ascii_case("your") || c[2].starts_with(char::is_uppercase) {
                    'M'
                } else {
                    'm'
                };
            format!("{} \0{suffix}", &c[1])
        })
    } else {
        Cow::Borrowed(part.as_ref())
    };
    let fixed = if ME.is_match(&marked) {
        let mut words: Vec<_> = WORD
            .find_iter(&marked.to_lowercase())
            .map(|m| m.as_str().to_owned())
            .collect();
        let positions: Vec<_> = words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| (w == "me").then_some(i))
            .collect();
        possessive_me(&mut words);
        let mut positions = positions.into_iter();
        ME.replace_all(&marked, |c: &Captures<'_>| {
            let m = c.get(0).unwrap();
            if positions.next().is_some_and(|i| words[i] == "my")
                && !marked[m.end()..].trim_start().starts_with(',')
            {
                case(m.as_str(), "your")
            } else {
                m.as_str().into()
            }
        })
    } else {
        Cow::Borrowed(marked.as_ref())
    };
    let line = PERSON.replace_all(&fixed, |c: &Captures<'_>| {
        case(&c[0], person(&c[0].to_ascii_lowercase()))
    });
    let line = line.replace("\0m", "me").replace("\0M", "my");
    APOSTROPHES
        .replace_all(&line, |c: &Captures<'_>| {
            case(
                &c[0],
                lookup(APOSTROPHE, &c[0].to_ascii_lowercase()).unwrap_or(&c[0]),
            )
        })
        .into_owned()
}

pub(crate) fn you(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut start = 0;
    for quoted in QUOTED.find_iter(line).flatten() {
        out.push_str(&flip(&line[start..quoted.start()]));
        out.push_str(quoted.as_str());
        start = quoted.end();
    }
    out.push_str(&flip(&line[start..]));
    out
}

macro_rules! pattern {
    ($name:ident, $kind:ty, $pattern:expr) => {
        static $name: LazyLock<$kind> = LazyLock::new(|| <$kind>::new($pattern).unwrap());
    };
}
pattern!(TWO_TO, Fancy, TWO_TO_PATTERN);
pattern!(TWO_PLACE, Fancy, TWO_PLACE_PATTERN);
pattern!(TWO_VERB, Fancy, TWO_VERB_PATTERN);
pattern!(STARS, Regex, COUNTED_STARS_PATTERN);
pattern!(ONE_STAR, Regex, ONE_STAR_PATTERN);
pattern!(LETTERS, Fancy, COUNTED_LETTERS_PATTERN);
pattern!(DEGREE, Regex, A_DEGREE_PATTERN);
pattern!(CLASS, Regex, DEGREE_CLASS_PATTERN);
pattern!(SLASH, Regex, WITH_SLASH_PATTERN);
const COUNTS: [&str; 13] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve",
];

fn spoken(line: &str) -> String {
    let line = STARS.replace_all(line, |c: &Captures<'_>| {
        let i = c["count"].parse::<usize>().unwrap();
        if i == 1 {
            "an A*".into()
        } else {
            format!("{} A*s", COUNTS[i])
        }
    });
    let line = ONE_STAR.replace_all(&line, "an A*");
    let line = LETTERS.replace_all(&line, |c: &fancy_regex::Captures<'_>| {
        format!(
            "{} {}s",
            COUNTS[c["count"].parse::<usize>().unwrap()],
            c["letter"].to_uppercase()
        )
    });
    let line = if DEGREE.is_match(&line) {
        CLASS.replace_all(&line, "${first}:${second}")
    } else {
        Cow::Borrowed(line.as_ref())
    };
    let line = SLASH.replace_all(&line, "with ");
    let line = TWO_TO.replace_all(&line, |c: &fancy_regex::Captures<'_>| {
        format!("{}to", &c[0][..c[0].len() - 1])
    });
    let line = TWO_PLACE.replace_all(&line, "to ");
    let line = TWO_VERB.replace_all(&line, "to ");
    let words: Vec<_> = line.split_whitespace().collect();
    let bare: Vec<_> = words
        .iter()
        .map(|w| {
            w.trim_matches(['.', ',', ';', ':', '!', '?'])
                .to_lowercase()
        })
        .collect();
    words
        .iter()
        .enumerate()
        .map(|(i, &word)| {
            let before = i.checked_sub(1).map_or("", |j| bare[j].as_str());
            let after = bare.get(i + 1).map_or("", String::as_str);
            let replacement = match bare[i].as_str() {
                "yer" => Some(if after.is_empty() || has(YES_AFTER, after) {
                    "yeah"
                } else {
                    "your"
                }),
                "its" if has(ITS_OWNS_AFTER, before) => None,
                b => lookup(SPOKEN, b),
            };
            replacement.map_or_else(
                || word.to_owned(),
                |new| {
                    format!(
                        "{new}{}",
                        &word[word.trim_end_matches(['.', ',', ';', ':', '!', '?']).len()..]
                    )
                },
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pattern!(NO_THEN, Fancy, NO_THEN_A_LINE_ONLY_PATTERN);
pattern!(NO_LEAD, Fancy, NO_LEAD_PATTERN);
fn strip_lead(line: &str) -> &str {
    let mut line = line.trim_matches([' ', ',', ';', ':', '-']);
    if let Ok(Some(m)) = NO_THEN.find(line) {
        line = &line[m.end()..]
    }
    loop {
        if let Ok(Some(m)) = NO_LEAD.find(line) {
            line = &line[m.end()..];
            continue;
        }
        let low = line.to_ascii_lowercase();
        let Some(lead) = LEAD_IN.iter().find(|&&lead| {
            low.strip_prefix(lead).is_some_and(|s| {
                s.is_empty() || s.starts_with([' ', ',', ';', ':', '-', '!', '?', '.'])
            })
        }) else {
            return line;
        };
        line = line[lead.len()..].trim_start_matches([' ', ',', ';', ':', '-', '!', '?', '.']);
    }
}
pattern!(CONTENT_WORD, Regex, WORD_PATTERN);
pattern!(LINK, Regex, OPENS_WITH_A_LINK_PATTERN);
pattern!(MONEY, Regex, MONEY_ONLY_PATTERN);
pattern!(STOP, Fancy, STOP_PATTERN);
pattern!(POLITE, Regex, POLITE_PATTERN);
pattern!(PICK, Regex, PICK_FRAME_PATTERN);
pattern!(KIN, Fancy, KIN_POSSESSIVE_PATTERN);
pattern!(GOT, Regex, HAVE_GOT_PATTERN);
pattern!(COUNT, Regex, A_COUNT_NOT_A_STATE_PATTERN);
pattern!(WHOS, Regex, r"(?i)\bwhos\b");

fn echo(clean: &str) -> Option<String> {
    let body = strip_lead(clean);
    if !CONTENT_WORD
        .find_iter(&body.to_lowercase())
        .any(|m| !has(EMPTY_WORDS, m.as_str()))
        || LINK.is_match(body)
            && body.split_whitespace().count() <= 3
            && !MONEY.is_match(body.trim_matches([' ', '.', ',', '!', '?']))
    {
        return None;
    }
    let clauses;
    let mut said = body;
    if body.chars().count() > 90 {
        clauses = STOP
            .split(body)
            .filter_map(Result::ok)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .take(2)
            .collect::<Vec<_>>()
            .join(", ");
        if !clauses.is_empty() {
            said = &clauses;
        }
        if let Some((i, _)) = said.char_indices().nth(90) {
            let chunk = &said[..i];
            said = chunk
                .rsplit_once(' ')
                .map(|(s, _)| s.trim_end_matches([' ', ',', ';', ':', '-']))
                .filter(|s| s.contains(' '))
                .unwrap_or(chunk);
        }
    }
    let said = POLITE.replace(
        said.trim_matches([' ', ',', ';', ':', '-', '!', '?', '.', '—']),
        "",
    );
    let said = super::filler::without_filler(said.trim_matches([' ', ',', ';', ':', '-']));
    let mut tail = said.as_str();
    while let Some(cut) = FILLER_TAIL.iter().find(|s| {
        tail.to_lowercase()
            .trim_matches([' ', ',', ';', ':', '-'])
            .ends_with(**s)
            && tail.split_whitespace().count() > s.split_whitespace().count()
    }) {
        for _ in cut.split_whitespace() {
            tail = tail
                .trim_end()
                .rsplit_once(char::is_whitespace)?
                .0
                .trim_end();
        }
    }
    if let Some(m) = PICK.find(tail) {
        let rest = tail[m.end()..].trim_matches([' ', ',', ';', ':', '-', '.', '!', '?', '—']);
        if !rest.is_empty() {
            tail = rest;
        }
    }
    let line = you(&spoken(tail));
    let line = KIN.replace_all(&line, "${1}'s");
    let tidy = line
        .split_whitespace()
        .enumerate()
        .map(|(index, word)| {
            let low = word.to_lowercase();
            if matches!(
                low.as_str(),
                "you" | "your" | "you're" | "you've" | "we" | "our" | "yourself"
            ) || index == 0 && has(FIRST_WORD, &low)
            {
                low
            } else {
                word.into()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    if tidy.is_empty() {
        return None;
    }
    let back = GOT.replace_all(&tidy, "${1}'ve got");
    let back = if let Some(c) = COUNT.captures(&back)
        && !has(UNITS, &c[2].to_ascii_lowercase())
    {
        Cow::Owned(format!("you've got {}", &back[c.get(1).unwrap().start()..]))
    } else {
        back
    };
    Some(WHOS.replace_all(&back, "who's").into_owned())
}

pattern!(HABIT, Regex, HABIT_PATTERN);
pattern!(GOING, Regex, GOING_PATTERN);
pattern!(GOING_BAD, Regex, GOING_BAD_PATTERN);
pattern!(OBLIGED, Regex, OBLIGED_PATTERN);
pattern!(TROUBLE, Fancy, TROUBLE_PATTERN);
pattern!(FEELING, Fancy, FEELING_PATTERN);
pattern!(BODY, Fancy, BODY_PATTERN);
pattern!(A_FEELING, Fancy, A_FEELING_PATTERN);
pattern!(WOULD_HAVE_BEEN, Fancy, WOULD_HAVE_BEEN_PATTERN);
pattern!(DAY_ASK, Fancy, A_DAY_ASKED_PATTERN);
pattern!(DAY_PAST, Fancy, DAY_BEEN_AND_GONE_PATTERN);
pattern!(DAY_STATE, Fancy, DAY_JUST_A_STATE_PATTERN);
pattern!(FIGURE, Fancy, A_FIGURE_PATTERN);
pattern!(TURNED_DOWN, Fancy, A_DOING_TURNED_DOWN_PATTERN);
pattern!(DOING, Fancy, A_DOING_PATTERN);
pattern!(SORRY_DAY, Fancy, NOT_A_DAY_TO_LOOK_FORWARD_TO_PATTERN);
pattern!(FAR, Fancy, DAY_FAR_PATTERN);
pattern!(SOON, Fancy, DAY_SOON_PATTERN);
pattern!(STATE, Fancy, STATE_PATTERN);
pattern!(HAVE, Fancy, HAVE_THING_PATTERN);
pattern!(PAST, Fancy, PLACED_IN_THE_PAST_PATTERN);
pattern!(PLAN, Fancy, PLAN_PATTERN);

fn coming(line: &str) -> Option<&'static [&'static str]> {
    if [&*DAY_ASK, &*DAY_PAST, &*DAY_STATE, &*FIGURE, &*TURNED_DOWN]
        .iter()
        .any(|r| r.is_match(line).unwrap_or(false))
        || !DOING.is_match(line).unwrap_or(false)
    {
        return None;
    }
    [
        (&*SORRY_DAY, SORRY_TAIL),
        (&*FAR, FAR_TAIL),
        (&*SOON, SOON_TAIL),
    ]
    .into_iter()
    .find_map(|(re, pool)| re.is_match(line).unwrap_or(false).then_some(pool))
}

fn states_a_fact(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    let kin = lower
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|w| matches!(w[0], "my" | "our") && has(RELATIONS, w[1]));
    (kin || PLAN.is_match(line).unwrap_or(false))
        && (STATE.is_match(line).unwrap_or(false) || line.chars().count() <= 30)
        && !TROUBLE.is_match(line).unwrap_or(false)
}

static PET_POSSESSION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)^(?:i|we) *(?:(?:'ve|ve) (?:already )?got|(?:already )?(?:have(?: got)?|got|own)) ",
        r"(?:(?:already |also )?)(a|an|one|two|three|four|five|six|seven|eight|nine|ten|\d{1,2}) ",
        r"(dogs?|puppy|puppies|cats?|kittens?|rabbits?|hamsters?|guinea pigs?|parrots?|goldfish|fish|horses?)[.!]*$",
    ))
    .expect("a pet possession with no additional information")
});

const PET_TAIL_ONE: &[&str] = &[
    "{Thing} — how long has it been with you?",
    "{Thing}. What's it like to live with?",
    "{Thing} — is it good company?",
];

const PET_TAIL_MANY: &[&str] = &[
    "{Thing} — do they get on?",
    "{Thing}. Who's in charge?",
    "{Thing} — how long have they been with you?",
];

fn pet_possession(line: &str, memory: &Memory) -> Option<String> {
    let line = line.to_lowercase().replace('’', "'");
    let found = PET_POSSESSION.captures(&line)?;
    let count = match &found[1] {
        "a" | "an" | "one" | "1" => "a",
        "0" => return None,
        "2" => "two",
        "3" => "three",
        "4" => "four",
        "5" => "five",
        "6" => "six",
        "7" => "seven",
        "8" => "eight",
        "9" => "nine",
        "10" => "ten",
        other => other,
    };
    let animal = &found[2];
    let plural = matches!(animal, "puppies") || animal.ends_with('s');
    if (count == "a") == plural {
        return None;
    }
    let pool = if count == "a" {
        PET_TAIL_ONE
    } else {
        PET_TAIL_MANY
    };
    let named = format!("{count} {animal}");
    Some(compose::pick(pool, memory, &line).replace("{Thing}", &upper_first(&named)))
}

fn possession(line: &str) -> bool {
    HAVE.captures(line)
        .ok()
        .flatten()
        .and_then(|c| c.get(1))
        .is_some_and(|m| {
            let thing = m.as_str().trim_matches(['-', '\'']).to_ascii_lowercase();
            !thing.is_empty() && !has(NOT_A_THING, &thing) && !thing.chars().all(char::is_numeric)
        })
        && !HABIT.is_match(line)
        && !PAST.is_match(line).unwrap_or(false)
}

fn ending(text: &str) -> String {
    text.to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn render(pool: &[&str], thing: &str, food: &str, memory: &Memory, avoid_ending: bool) -> String {
    let recent = memory
        .turns
        .last()
        .map(|t| ending(&t.bot))
        .unwrap_or_default();
    let fresh: Vec<_> = if avoid_ending && !recent.is_empty() {
        pool.iter()
            .copied()
            .filter(|frame| {
                let tail = frame
                    .split_once('}')
                    .map_or("", |(_, after)| after.split('{').next().unwrap_or(""));
                let tail = ending(tail);
                tail.is_empty() || !recent.contains(&tail)
            })
            .collect()
    } else {
        Vec::new()
    };
    let chosen = compose::pick(if fresh.is_empty() { pool } else { &fresh }, memory, "");
    chosen
        .replace("{Thing}", &upper_first(thing))
        .replace("{thing}", thing)
        .replace("{food}", food)
}

pub(crate) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    use super::{FoodAck, food_ack};
    use crate::bot::understanding::kinds::Kind;
    let clean = reading.raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if reading.kind != Kind::Statement
        || clean.contains('?')
        || crate::bot::modules::the_detail_the_thread_adds::heavy::bad_news(
            &reading.line.plain.to_lowercase(),
        )
        || compose::loss_reader::reads_as_loss(&clean)
        || context::resolved_worry(&clean, memory)
        || reading.features.suggestion.is_some()
        || reading.context_fields.errand.is_some()
        || [&*BODY, &*A_FEELING, &*WOULD_HAVE_BEEN]
            .iter()
            .any(|r| r.is_match(&clean).unwrap_or(false))
    {
        return None;
    }
    let body = strip_lead(&clean);
    if let Some(reply) = pet_possession(body, memory) {
        return Some(reply);
    }
    let mut food = "that";
    let mut additive = None;
    let mut avoid_ending = true;
    let contextual_frames = |family: bool, ordinary| {
        if context::heavy(memory) {
            if family {
                context::HEAVY_FRAMES
            } else {
                context::STILL_FRAMES
            }
        } else {
            ordinary
        }
    };
    let food_ack = food_ack(&clean);
    if food_ack.is_none() && memory.stated_plans.current_from(&clean) {
        let thing = echo(&clean)?;
        if let Some(pool) = ack_plan::pool(&clean, true, memory) {
            let pool: Vec<_> = pool.iter().map(std::borrow::Cow::as_ref).collect();
            return Some(render(&pool, &thing, food, memory, true));
        }
        if !context::heavy(memory)
            && coming(body).is_none()
            && let Some(response) = ack_opening::reply(&thing, &clean, memory)
        {
            return Some(response);
        }
    }
    let frames = match food_ack {
        Some(FoodAck::RelativeAllergy {
            relation,
            allergens,
        }) if !relation.is_empty() && !allergens.is_empty() => {
            contextual_frames(true, OTHER_LIMIT_FRAMES)
        }
        Some(FoodAck::Allergy(allergens)) if !allergens.is_empty() => FOOD_FRAMES,
        Some(FoodAck::Additive(more)) if memory.personal.own_has_allergy() => {
            additive = Some(format!("you're allergic to {} too", more.join(" and ")));
            FOOD_FRAMES
        }
        Some(FoodAck::Diet(diet)) => {
            if diet.contains("alcohol") {
                DRINK_FRAMES
            } else {
                FOOD_FRAMES
            }
        }
        Some(FoodAck::Condition(conditions)) => {
            food = match conditions.first()?.as_str() {
                "coeliac" | "celiac" | "gluten intolerant" => "gluten",
                "lactose intolerant" => "dairy",
                _ => return None,
            };
            CONDITION_FRAMES
        }
        _ if memory.introduced_now || (!reading.context_fields.routine && states_a_fact(body)) => {
            let family = body
                .to_lowercase()
                .split_whitespace()
                .any(|w| has(RELATIONS, w));
            contextual_frames(family, coming(body).unwrap_or(FRAMES))
        }
        _ if context::detail(reading, &clean, memory) => {
            contextual_frames(false, coming(body).unwrap_or(FRAMES))
        }
        _ if possession(body) => {
            avoid_ending = false;
            FRAMES
        }
        _ => {
            let who = body
                .split_whitespace()
                .next()?
                .trim_matches([',', ';', ':'])
                .to_lowercase();
            if !matches!(
                who.as_str(),
                "i" | "i'm"
                    | "im"
                    | "i've"
                    | "ive"
                    | "i'd"
                    | "we"
                    | "we're"
                    | "were"
                    | "my"
                    | "our"
            ) || OBLIGED.is_match(body)
                || TROUBLE.is_match(body).unwrap_or(false)
                || FEELING.is_match(body).unwrap_or(false)
            {
                return None;
            }
            avoid_ending = false;
            if HABIT.is_match(body) {
                HABIT_TAIL
            } else if GOING.is_match(body) && !GOING_BAD.is_match(body) {
                GOING_TAIL
            } else {
                return None;
            }
        }
    };
    let thing = additive.or_else(|| echo(&clean))?;
    Some(render(frames, &thing, food, memory, avoid_ending))
}

#[cfg(test)]
mod tests {
    use super::*;
    mod cases {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/ack.rs"
        ));
    }
    fn compare(cases: &[(&str, &str)], read: impl Fn(&str) -> Option<String>) {
        let diffs: Vec<_> = cases
            .iter()
            .filter_map(|&(input, wanted)| {
                let result = read(input);
                (result.as_deref() != Some(wanted)).then_some((input, wanted, result))
            })
            .collect();
        assert!(
            diffs.is_empty(),
            "{} differences in {} cases: {:#?}",
            diffs.len(),
            cases.len(),
            diffs
        );
    }
    #[test]
    fn uses_the_right_ending_for_a_plan_filed_on_this_turn() {
        for (line, wanted) in [
            (
                "well im looking at an oven on Saturday",
                Some("You're looking at an oven on Saturday — how are you feeling about it?"),
            ),
            (
                "im looking at a shower next month",
                Some("You're looking at a shower next month. That'll come round fast."),
            ),
            ("im looking at an electric blanket for the winter", None),
        ] {
            let mut memory = Memory::default();
            memory.stated_plans.observe(line);
            assert_eq!(
                reply(&Reading::new(line), &memory).as_deref(),
                wanted,
                "{line}"
            );
        }
    }

    #[test]
    fn perspective_and_quoted_words() {
        compare(cases::PERSPECTIVE, |s| Some(you(s)))
    }
    #[test]
    fn echoes_the_original_line() {
        compare(cases::ECHO, echo)
    }
    #[test]
    fn acknowledges_a_filed_fact() {
        compare(cases::FIRST, |s| {
            let reading = Reading::new(s);
            let mut memory = Memory::default();
            crate::bot::memory::observe(&reading, &mut memory);
            reply(&reading, &memory)
        })
    }
}
