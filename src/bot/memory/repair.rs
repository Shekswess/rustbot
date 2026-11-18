//! Keep a corrected subject in the conversation after the reply has been chosen.
use super::done_topic;
use crate::bot::{Memory, Reading};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    REPAIR,
    r"(?i)^\W*(?:(?:no|nah|nope|sorry|actually|oh|wait|hmm|well|ok|okay|right|yes)[,\s]+)*(?:i\s+(?:meant|mean)|i\s+did\s+mean|we\s+meant|meant|rather)\b[,\s:]+(?P<thing>\S.*?)\W*$"
);
pattern!(
    BARE_REPAIR,
    r"(?i)^\W*(?:(?:no|nah|nope|sorry|actually|oh|wait)[,\s]+)+(?P<thing>[^.!?]+?)\W*$"
);
pattern!(
    POSSESSION_REPAIR,
    r"(?i)^\W*(?:(?:no|nah|nope|sorry|actually|oh|wait)[,\s]+)+(?:i|we)(?:'ve|ve|\s+have|\s+have\s+got|\s+got)\s+(?:got\s+)?(?P<thing>[^.!?]+?)\W*$"
);
pattern!(
    CONTRAST,
    r"(?i)\b(?:not|instead|isn'?t|wasn'?t|isnt|wasnt)\b"
);
pattern!(VALUE, r"\d");
pattern!(TO_SAY, r"^to\s+(?:say\s+|mean\s+)?");
pattern!(WORD, r"[a-z0-9']+");
pattern!(RELATION_WORD, r"[a-z']+");
pattern!(
    DETERMINER,
    r"(?i)^(?:a|an|the|my|our|your|his|her|their|this|that|these|those)\b"
);
pattern!(
    DETERMINER_WITH_SPACE,
    r"(?i)^(?:a|an|the|my|our|your|his|her|their|this|that|these|those)\s+"
);
pattern!(
    ASK_OPENS,
    concat!(
        r"(?i)^(?:how|what|why|when|where|which|who|whose|whom)\b",
        r"|^(?:can|could|should|would|will|shall|do|does|did|is|are|was|were|have|has|had|am|may",
        r"|might|must|need)\s+(?:i|we|you|they|it|he|she|that|this|there)\b"
    )
);
static INTENTION: LazyLock<Fancy> = LazyLock::new(|| {
    Fancy::new(r"(?i)\b(?:i|we)\s+meant\s+to\s+(?!say\b)").expect("valid intention pattern")
});

const NOT_A_SWAP: &str = "thanks thank worries worry problem problems idea matter matters clue doubt chance way luck bother hurry rush need point sense reason question questions drama fault apologies apology offence offense harm damage wonder boat lot bit little";
const VERB: &str = concat!(
    "is are was were am be been being do does did done doing have has had having ",
    "can could should would will shall may might must ",
    "want wants wanted need needs needed got get gets make makes made take takes took ",
    "give gives gave go goes went gone like likes liked love loves loved hate hates hated ",
    "feel feels felt hurt hurts think thinks thought know knows knew said says say ",
    "tell tells told see sees saw seen try tries tried use uses used work works worked ",
    "live lives lived play plays played eat eats ate drink drinks drank buy buys bought ",
    "sell sells sold pay pays paid find finds found lose loses lost break breaks broke ",
    "fix fixes fixed start starts started stop stops stopped keep keeps kept put puts ",
    "run runs ran walk walks walked cook cooks cooked clean cleans cleaned"
);
const ASK: &str = "should shall can could would will may might must do does did is are was were am have has had what whats how when where which who why whose";
const POINTER: &str = "it that this these those them one other the a an something anything nothing whatever thing things what which same more less first second last next another both all here there";
const REMARK: &str = "i im id ill ive we you he she they it its that there this just only also really very so whatever anyway honestly";
const GRAMMAR: &str = concat!(
    "a an the my our your his her their its some any this that these those it them ",
    "is are was were am be been being do does did done doing have has had having ",
    "can could should would will shall may might must ",
    "and or but so then than too also just very really quite more most less least ",
    "for with without about at in on of to from by into onto over under again ",
    "what whats how when where which who why whose way much many long ",
    "good best great nice better bad worse ok okay please thanks thank ",
    "today tomorrow tonight yesterday now soon later weekend weekends weekday weekdays ",
    "one ones thing things stuff lot lots bit little"
);
const CONTENT_EXCLUDED: &str = "a an the my our your his her their its some any this that";
const DATES: &str = concat!(
    "january february march april may june july august september october november december ",
    "jan feb mar apr jun jul aug sep sept oct nov dec ",
    "monday tuesday wednesday thursday friday saturday sunday weekend"
);
const RELATIONS: &str = concat!(
    "aunt auntie aunty bf bfs boyfriend brother cousin dad daughter father fiance fiancee friend ",
    "gf gfs girlfriend gran grandad grandchild granddad granddaughter grandfather grandma grandmom ",
    "grandmother grandmum grandpa grandson granny hubby husband missus mom mother mum nan nana ",
    "nanna neighbor neighbour nephew niece parents partner sister son stepbrother stepchild ",
    "stepdad stepdaughter stepfather stepmom stepmother stepmum stepsister stepson twins uncle wife"
);

fn has(group: &str, word: &str) -> bool {
    group.split_ascii_whitespace().any(|part| part == word)
}

fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    WORD.find_iter(text)
        .map(|word| word.as_str().trim_matches('\''))
}

fn phrase<'a>(regex: &Regex, text: &'a str) -> Option<&'a str> {
    regex.captures(text)?.name("thing").map(|hit| hit.as_str())
}

fn the_thing(text: &str) -> Option<String> {
    let text = flat(text);
    if CONTRAST.is_match(&text) || INTENTION.is_match(&text).unwrap_or(false) {
        return None;
    }
    let explicit = phrase(&REPAIR, &text).or_else(|| phrase(&POSSESSION_REPAIR, &text));
    let bare = explicit.is_none();
    let named = explicit.or_else(|| phrase(&BARE_REPAIR, &text))?;
    let trimmed = TO_SAY.replace(named.trim(), "");
    let thing = trimmed.trim_matches([' ', ',', '.', '!', '?', ';', ':', '"', '\'']);
    if thing.is_empty() || VALUE.is_match(thing) {
        return None;
    }
    let lower = thing.to_lowercase();
    let words: Vec<_> = words(&lower).collect();
    let first = *words.first()?;
    if words.iter().all(|word| has(POINTER, word))
        || has(REMARK, first)
        || has(ASK, first)
        || bare
            && (!DETERMINER.is_match(thing)
                || words
                    .iter()
                    .filter(|word| !has(GRAMMAR, word))
                    .all(|word| has(NOT_A_SWAP, word))
                || words.iter().any(|word| has(VERB, word)))
    {
        return None;
    }
    let last = *words.last()?;
    if last != first
        && has(POINTER, last)
        && words
            .iter()
            .all(|word| has(POINTER, word) || has(GRAMMAR, word))
    {
        return None;
    }
    Some(thing.to_owned())
}

fn different(old: &str, thing: &str) -> bool {
    let held = old.to_lowercase();
    words(&thing.to_lowercase()).any(|word| {
        !word.is_empty() && !has(CONTENT_EXCLUDED, word) && !words(&held).any(|old| old == word)
    })
}

fn relation(word: &str) -> Option<&str> {
    let word = word.trim_matches('\'');
    if has(RELATIONS, word) {
        return Some(word);
    }
    let base = word
        .strip_suffix("'s")
        .or_else(|| word.strip_suffix('s'))
        .unwrap_or(word);
    if has(RELATIONS, base) {
        return Some(base);
    }
    base.strip_suffix('\'').filter(|base| has(RELATIONS, base))
}

fn named_person(old: &str, thing: &str) -> Option<String> {
    let lower = thing.to_lowercase();
    let people: Vec<_> = RELATION_WORD
        .find_iter(&lower)
        .filter_map(|word| relation(word.as_str()))
        .collect();
    if people.is_empty() {
        return None;
    }
    RELATION_WORD
        .find_iter(&old.to_lowercase())
        .filter_map(|word| relation(word.as_str()))
        .find(|person| !people.contains(person))
        .map(str::to_owned)
}

fn named_thing(line: &str) -> Option<String> {
    let reading = Reading::new(line);
    let mut entities: Vec<_> = reading
        .entities
        .iter()
        .filter(|entity| matches!(entity.kind, "thing" | "animal" | "food" | "place"))
        .collect();
    entities.sort_by_key(|entity| entity.at);
    let first = entities.first()?;
    let mut name = flat(&first.text.to_lowercase());
    let mut next = first.at + name.split_whitespace().count();
    for entity in &entities[1..] {
        if entity.at != next {
            return None;
        }
        let word = flat(&entity.text.to_lowercase());
        next += word.split_whitespace().count();
        name.push(' ');
        name.push_str(&word);
    }
    flat(&line.to_lowercase()).contains(&name).then_some(name)
}

fn tail_word(line: &str, pointer: bool) -> Option<String> {
    let lower = line.to_lowercase();
    let mut words: Vec<_> = words(&lower).collect();
    while let Some(word) = words.pop() {
        if word.len() < 3 || has(GRAMMAR, word) || has(POINTER, word) {
            continue;
        }
        let date = has(DATES, word);
        if pointer && (date || has(VERB, word)) {
            continue;
        }
        if !pointer && (date || word.bytes().all(|byte| byte.is_ascii_digit())) {
            return None;
        }
        return done_topic::keywords(word).next().map(|_| word.to_owned());
    }
    None
}

fn refine(thing: &str, line: &str) -> Option<(String, String)> {
    let lower = thing.to_lowercase();
    let mut words: Vec<_> = words(&lower).collect();
    if !matches!(words.pop(), Some("one" | "ones")) {
        return None;
    }
    words.retain(|word| word.len() >= 2 && !has(GRAMMAR, word) && !has(POINTER, word));
    if words.is_empty() {
        return None;
    }
    let noun = tail_word(line, true)?;
    words.push(&noun);
    let refined = words.join(" ");
    Some((noun, refined))
}

fn swapped(line: &str, old: &str, thing: &str) -> Option<String> {
    let line = flat(line);
    let thing = thing.trim();
    let suffix = if has(RELATIONS, old) { "(?:'s|s)?" } else { "" };
    let pattern = Regex::new(&format!(r"(?i)\b{}{suffix}\b", regex::escape(old))).ok()?;
    let found = pattern.find(&line)?;
    let before = &line[..found.start()];
    let new = if before.split_whitespace().next().is_some() {
        DETERMINER_WITH_SPACE.replace(thing, "")
    } else {
        std::borrow::Cow::Borrowed(thing)
    };
    let joined = format!("{before}{new}{}", &line[found.end()..]);
    let mut result = Vec::<&str>::new();
    for word in joined.split_whitespace() {
        if result
            .last()
            .is_none_or(|prior| prior.to_lowercase() != word.to_lowercase())
        {
            result.push(word);
        }
    }
    let result = result.join(" ");
    (result.to_lowercase() != line.to_lowercase()).then_some(result)
}

fn state_word(line: &str, tail: &str) -> bool {
    let pattern = format!(
        r"(?i)\b(?:is|are|was|were|isn'?t|aren'?t|wasn'?t|weren'?t|feels?|seems?|looks?|gets?|got|goes|went|has|have|had|keeps?|stays?|stops?)\s+{}\b",
        regex::escape(tail)
    );
    Regex::new(&pattern).is_ok_and(|pattern| pattern.is_match(line))
}

struct Correction {
    old: String,
    new: String,
    text: String,
}

fn correction(text: &str, prior: &str) -> Option<Correction> {
    if prior.is_empty() {
        return None;
    }
    let thing = the_thing(text)?;
    if ASK_OPENS.is_match(&thing) || thing.ends_with('?') {
        return None;
    }
    let tail = tail_word(prior, false);
    let (candidates, new) = if let Some((old, refined)) = refine(&thing, prior) {
        (vec![old], refined)
    } else {
        (
            [
                named_person(prior, &thing),
                named_thing(prior),
                tail.clone(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            thing,
        )
    };
    let (old, text) = candidates
        .into_iter()
        .filter(|old| different(old, &new))
        .find_map(|old| swapped(prior, &old, &new).map(|text| (old, text)))?;
    let text = tail
        .filter(|tail| *tail != old && !state_word(prior, tail) && different(tail, &new))
        .and_then(|tail| swapped(prior, &tail, &new))
        .unwrap_or(text);
    Some(Correction { old, new, text })
}

pub(in crate::bot) fn after(text: &str, memory: &mut Memory) {
    let Some(prior) = memory.turns.last() else {
        return;
    };
    let Some(correction) = correction(text, &prior.user) else {
        return;
    };
    memory.doneness_topics.correct(&correction.text);
    memory.ack_topics.correct(correction.text);
    memory
        .stated_plans
        .correct_subject(|plan| swapped(plan, &correction.old, &correction.new));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn corrected_topic_and_plan_survive_a_reply_from_another_module() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let original = "i need to pick a paint colour for the hall";
        let repair = "i meant the kitchen";
        bot.turn(original);
        let result = bot.turn(repair);
        assert_eq!(result.module, Some("greeting"));
        assert_eq!(bot.memory.turns[0].user, original);
        assert_eq!(bot.memory.turns[1].user, repair);
        let topic = Some("i need to pick a paint colour for the kitchen");
        assert_eq!(bot.memory.doneness_topics.current_text(), topic);
        assert_eq!(
            bot.memory.ack_topics.stored_topic(&bot.memory.turns).next(),
            topic
        );
        let plan = "Pick a paint colour for the kitchen";
        assert_eq!(bot.memory.stated_plans.latest(), Some(plan));
        assert_eq!(bot.memory.stated_plans.plans().collect::<Vec<_>>(), [plan]);

        let recall = bot.turn("and whats my target again");
        assert_eq!(recall.module, Some("a_fact_asked_again"));
        assert_eq!(
            recall.reply,
            "You said your target is to pick a paint colour for the kitchen."
        );
    }

    #[test]
    fn the_topic_keeps_the_source_shape_and_preserves_the_thing_that_was_broken() {
        let room = correction("no the dishwasher", "the washing machine is broken").unwrap();
        assert_eq!(
            (room.old.as_str(), room.text.as_str()),
            ("washing machine", "the dishwasher is broken")
        );
        let roast =
            correction("i meant a turkey", "how long does a chicken take to roast").unwrap();
        assert_eq!(roast.text, "how long does a chicken take to turkey");
    }

    #[test]
    fn corrections_of_values_and_new_questions_leave_the_subject_alone() {
        let prior = "i need to pick a paint colour for the hall";
        for input in [
            "no i meant 500",
            "no, thursday not tuesday",
            "no i meant should i paint it",
            "i meant to pick a colour but forgot",
            "no i meant the hall",
            "no thanks",
        ] {
            assert!(correction(input, prior).is_none(), "{input}");
        }
    }
}
