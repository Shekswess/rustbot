//! Apply a stated limit to a recent indexed answer.

use crate::bot::{Memory, Reading, knowledge::lookup, memory::history};
use crate::text::is_space;
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $regex:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($regex).expect(stringify!($name)));
    };
}

pattern!(
    LEAD,
    r"(?i)^(?:well|ok|okay|so|oh|ah|and|but|honestly|actually|yeah|sorry|anyway|course|of\s+course)\b[\s,]*"
);
pattern!(
    TAIL,
    r"(?i)(?:\s+(?:tbh|tho|though|anyway|actually|really|literally|honestly|i\s+suppose|i\s+guess|still|also|too|by\s+the\s+way|btw|atm|at\s+the\s+moment|lol|ha|haha|so\s+yeah|mate|dude|sadly|unfortunately))+$"
);
pattern!(
    ASKS,
    r"\?|^\s*(?:what|how|when|where|why|who|can|could|should|would|do|does|did|is|are|was|were|will|shall|any)\b"
);
pattern!(
    INSTRUCTION,
    r"(?i)^(?:sow|plant|prune|feed|mulch|pot|repot|thin|stake|tie|harvest|practise|practice|learn|watch|read|listen|repeat|spend|save|budget|compare|download|install|print|wear|cut|freeze|store|label|turn|put|take|use|keep|start|add|set|get|open|close|ask|book|check|try|make|give|water|clean|wash|report|write|stand|hold|mix|place|leave|move|cover|wrap|soak|strip|sand|fill|fit|call|text|email|ring|go|walk|run|sit|lie|rest|eat|drink|buy|pick|choose|measure|weigh|boil|fry|bake|simmer|stir|season|remove|replace|tighten|loosen|unplug|switch|plug|drain|bleed)\b"
);
pattern!(
    JOINER,
    r"(?i)^(?:so|and|then|but|just|really|also|first|afterwards?)\s+"
);
pattern!(
    LEAD_INS,
    r"(?i)^(?:same\s+as\s+before|the\s+first\s+thing\s+to\s+do|first\s+thing|so|right|okay|ok|then)\s*:\s*"
);
pattern!(CLAUSE, r"[,;]\s");
pattern!(
    WIRING,
    r"\b(?:consumer unit|lighting circuit|live wires?|loop wires?|mains wiring|electrical wiring)\b"
);

const DANGLING: &[&str] = &[
    "and", "or", "with", "a", "an", "the", "of", "to", "for", "in", "on", "at", "plus", "then",
    "but", "so", "your", "my", "its",
];
const GARDEN_WORDS: &[&str] = &[
    "plant",
    "sow",
    "sowing",
    "grow",
    "growing",
    "pot",
    "pots",
    "garden",
    "soil",
    "seed",
    "seeds",
    "compost",
    "bed",
    "patch",
    "allotment",
    "water",
    "sun",
    "shade",
    "herbs",
    "tomato",
    "tomatoes",
    "flower",
    "flowers",
    "bulb",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Subject {
    RentalRepair,
    Garden,
    Other,
}

struct Limit {
    subject: Subject,
    shapes: RegexSet,
    words: &'static [&'static str],
    replies: [&'static str; 3],
}

static LIMITS: LazyLock<[Limit; 5]> = LazyLock::new(|| {
    let limit = |subject, shapes: &[&str], words, replies| Limit {
        subject,
        shapes: RegexSet::new(shapes).expect("source limit shapes"),
        words,
        replies,
    };
    [
        limit(
            Subject::RentalRepair,
            &[
                r"^(?:we|i)\s*(?:just\s+)?rent(?:ing)?(?:\s+(?:a|an|the|our|this)\s+(?:flat|house|place|home|bungalow|room|council\s+flat))?$",
                r"^(?:we|i)\s*(?:are|re)\s+(?:renting|in\s+a\s+rented|renters?)$",
                r"^(?:its|it\s+is)\s+(?:a\s+)?(?:rented|rental|council|housing\s+association)(?:\s+(?:flat|house|place|home))?$",
                r"^(?:we|i)\s+(?:are|re)?\s*(?:just\s+)?tenants?$",
                r"^(?:we|i)\s+(?:dont|do\s+not)\s+own\s+(?:it|the\s+flat|the\s+house|our\s+home)$",
            ],
            &[
                "repair", "fix", "damp", "boiler", "radiator", "tap", "leak", "leaking",
                "dripping", "drip", "wall", "ceiling", "paint", "heating", "plumb", "roof",
                "floor", "fuse", "washer", "mould", "mold", "window", "door", "drains", "toilet",
                "shower", "bath", "landlord", "carpet", "electric", "socket", "gutter", "chimney",
            ],
            [
                "Then it's your landlord's to fix, not yours - but do the free bit first: {move}",
                "Renting puts that on them: report it in writing, and meanwhile {move_lower}",
                "Right, so it's their job. What you can do now: {move_lower}",
            ],
        ),
        limit(
            Subject::Other,
            &[
                r"^(?:we|i)\s*(?:have|ve)\s+(?:got\s+)?no\s+car$",
                r"^(?:there\s+is\s+)?no\s+car(?:\s+here)?$",
                r"^(?:we|i)\s*(?:dont|do\s+not|can\s*not|can't|cant)\s*drive$",
                r"^(?:we|i)\s*(?:dont|do\s+not)\s+have\s+(?:a\s+)?car$",
                r"^(?:its|it\s+is)\s+(?:a\s+)?no\s+car\s+(?:household|family)$",
            ],
            &[
                "drive",
                "driving",
                "car",
                "travel",
                "journey",
                "get there",
                "getting there",
                "route",
                "station",
                "bus",
                "train",
                "coach",
                "parking",
                "park",
                "trip",
                "visit",
                "ferry",
                "flight",
                "fly",
                "airport",
                "motorway",
            ],
            [
                "Then go by train or bus instead of driving - and the rest of it still holds: {move_lower}",
                "No car changes the how, not the plan: {move_lower}",
                "Then book the train rather than drive. Everything else stands: {move_lower}",
            ],
        ),
        limit(
            Subject::Garden,
            &[
                r"^(?:we|i)\s*(?:only|just)\s+(?:have|ve\s+got|got)\s+(?:a\s+|one\s+)?(?:small\s+)?(?:balcony|windowsill|window\s+box|yard|patio|terrace)$",
                r"^(?:its|it\s+is)\s+(?:just\s+)?(?:a\s+)?(?:balcony|windowsill|window\s+box)$",
            ],
            GARDEN_WORDS,
            [
                "Then keep it all in pots - that's the part that matters: {move_lower}",
                "A balcony is enough for that: {move_lower}",
                "Then it's pots on the balcony rather than a bed: {move_lower}",
            ],
        ),
        limit(
            Subject::Garden,
            &[
                r"^(?:we|i)\s*(?:dont|do\s+not)\s+have\s+(?:a\s+|any\s+)?(?:garden|outside\s+space|yard|plot|allotment)$",
                r"^(?:no|without\s+a)\s+(?:garden|outside\s+space|allotment)$",
            ],
            GARDEN_WORDS,
            [
                "Then keep it all in pots or a window box - that's the part that matters: {move_lower}",
                "No garden is fine for that: {move_lower}",
                "Then it's containers rather than a bed: {move_lower}",
            ],
        ),
        limit(
            Subject::Other,
            &[
                r"^(?:hes|he\s+is|shes|she\s+is|its|it\s+is)\s+(?:a\s+)?(?:rescue|rescued)(?:\s+(?:and\s+)?(?:quite\s+|very\s+|really\s+)?(?:nervous|anxious|shy|scared|terrified|timid))?$",
                r"^(?:hes|he\s+is|shes|she\s+is|its|it\s+is)\s+(?:quite\s+|very\s+|really\s+)?(?:nervous|anxious|scared|terrified|timid|shy)$",
                r"^(?:hes|shes|its)\s+(?:a\s+)?rescue$",
            ],
            &[
                "dog",
                "dogs",
                "puppy",
                "cat",
                "cats",
                "kitten",
                "lead",
                "leash",
                "walk",
                "walks",
                "train",
                "training",
                "scratch",
                "scratching",
                "bark",
                "barking",
                "sofa",
                "litter",
                "collar",
                "harness",
                "garden",
                "vet",
                "animal",
                "rescue",
            ],
            [
                "Then go slower than that and keep every go short: {move_lower}",
                "A nervous rescue needs that done in small doses: {move_lower}",
                "Then take it at their pace - the first step is the same: {move_lower}",
            ],
        ),
    ]
});

fn words(line: &str) -> impl DoubleEndedIterator<Item = &str> {
    line.split(is_space).filter(|word| !word.is_empty())
}

fn scope(line: &str) -> Option<&'static Limit> {
    let normalized = line.replace('’', "'");
    let mut clean = words(&normalized).collect::<Vec<_>>().join(" ");
    clean = clean.trim_matches([' ', '.', '!', ',']).to_owned();
    for _ in 0..3 {
        let lead = LEAD.replace(&clean, "");
        let tail = TAIL.replace(&lead, "");
        if tail == clean {
            break;
        }
        clean = tail.into_owned();
    }
    let lowered = clean.to_lowercase();
    let clean = lowered.trim_matches([' ', '.', ',', '!']);
    if clean.is_empty() || ASKS.is_match(clean) {
        return None;
    }
    LIMITS.iter().find(|limit| limit.shapes.is_match(clean))
}

fn trim_clause(clause: &str) -> &str {
    clause.trim_matches([' ', '.', ';', ':', ',', '-'])
}

fn first_instruction(answer: &str) -> Option<String> {
    let normalized = answer.replace('—', "- ");
    let flat = words(&normalized).collect::<Vec<_>>().join(" ");
    let body = LEAD_INS.replace(&flat, "");
    let end = body.char_indices().find_map(|(offset, ch)| {
        (matches!(ch, '.' | '!' | '?') && body[offset + 1..].starts_with(' ')).then_some(offset + 1)
    });
    let mut first = trim_clause(&body[..end.unwrap_or(body.len())]);
    if let Some((_, tail)) = first.split_once(':')
        && words(tail).take(4).count() == 4
    {
        first = tail.trim();
    }
    let clauses = CLAUSE
        .split(first)
        .map(str::trim)
        .filter(|part| !part.is_empty());
    let clauses = clauses.map(|part| JOINER.replace(part, "").into_owned());
    let selected: Vec<_> = clauses
        .skip_while(|clause| !INSTRUCTION.is_match(clause))
        .collect();
    let get_clauses = |limit| {
        let mut count = 0;
        selected
            .iter()
            .enumerate()
            .take_while(move |(index, part)| {
                count += words(part).count();
                *index == 0 || count <= limit
            })
            .count()
    };
    let mut count = get_clauses(45);
    if count > 1
        && selected[..count]
            .iter()
            .map(|part| words(part).count())
            .sum::<usize>()
            > 34
    {
        count = get_clauses(34);
    }
    let statement = selected.get(..count)?.join(", ");
    let mut cut = trim_clause(&statement);
    while let Some(last) = words(cut).next_back() {
        if !DANGLING.contains(&trim_clause(last).to_lowercase().as_str()) {
            break;
        }
        cut = cut.strip_suffix(last)?.trim_end();
    }
    let mut cut = trim_clause(cut);
    if cut.matches('(').count() > cut.matches(')').count() {
        cut = trim_clause(cut.split_once('(')?.0);
    }
    let mut chars = cut.chars();
    Some(format!(
        "{}{}.",
        chars.next()?.to_uppercase(),
        chars.as_str()
    ))
}

pub(in crate::bot) fn observe(_: &Reading<'_>, _: &mut Memory) {}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let anchor = memory.last_answer_anchor?;
    if memory.turns.len().checked_sub(anchor.turn + 1)? > 2 || history::in_a_safety_thread(memory) {
        return None;
    }
    let limit = scope(reading.raw)?;
    let reply = memory.entries.latest()?;
    let low = reply.to_lowercase();
    if limit.subject == Subject::RentalRepair && low.contains("landlord")
        || limit.subject == Subject::Garden && WIRING.is_match(&low)
    {
        return None;
    }
    let keyed = anchor
        .card
        .and_then(|(domain, id)| lookup::by_id(domain, id))
        .is_some_and(|card| {
            card.keys()
                .iter()
                .flat_map(|group| group.iter())
                .flat_map(|phrase| words(phrase))
                .any(|word| {
                    let lower = word.to_lowercase();
                    let stripped = lower.trim_end_matches('s');
                    limit.words.contains(&if stripped.is_empty() {
                        lower.as_str()
                    } else {
                        stripped
                    })
                })
        });
    if !keyed
        && limit
            .words
            .iter()
            .filter(|word| low.contains(**word))
            .take(2)
            .count()
            < 2
    {
        return None;
    }
    let instruction = first_instruction(reply)?;
    let mut chars = instruction.chars();
    let lower_instruction = format!("{}{}", chars.next()?.to_lowercase(), chars.as_str());
    let frame = limit.replies[memory.turns.len() % limit.replies.len()];
    Some(
        frame
            .replace("{move}", &instruction)
            .replace("{move_lower}", &lower_instruction),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn a_rented_flats_leaking_tap_keeps_the_whole_first_instruction() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let prompt = "we rent the flat";
        assert!(respond(&Reading::new(prompt), &bot.memory).is_none());
        bot.turn("the kitchen tap is dripping");
        let turn = bot.turn(prompt);
        assert_eq!(turn.module, Some("a_limit_over_the_answer"));
        assert_eq!(
            turn.reply,
            "Renting puts that on them: report it in writing, and meanwhile turn the water off at the isolating valve under the sink, take the handle off, take the old part to a hardware shop to match it."
        );
        assert!(scope("Can I report it to the landlord?").is_none());
    }

    #[test]
    fn a_balcony_then_no_garden_each_narrow_the_same_tomato_answer() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("how do i grow tomatoes");
        for (prompt, expected) in [
            (
                "we only have a balcony",
                "A balcony is enough for that: sow indoors in March or April and plant out in early June, once the nights stay above 10C.",
            ),
            (
                "no garden",
                "Then it's containers rather than a bed: sow indoors in March or April and plant out in early June, once the nights stay above 10C.",
            ),
        ] {
            let turn = bot.turn(prompt);
            assert_eq!(turn.module, Some("a_limit_over_the_answer"), "{prompt}");
            assert_eq!(turn.reply, expected, "{prompt}");
        }
    }

    #[test]
    fn gardening_limits_do_not_restate_a_wiring_answer() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let wrong_subject = bot.turn("how do i plant bulbs");
        assert!(wrong_subject.reply.contains("lighting circuit"));
        let prompt = "i just have a windowsill";
        assert!(scope(prompt).is_some());
        let next = bot.turn(prompt);
        assert!(!next.reply.contains("lighting circuit"));
        assert!(
            !next
                .candidates
                .iter()
                .any(|row| row.module == "a_limit_over_the_answer")
        );
    }
}
