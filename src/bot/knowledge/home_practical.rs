//! Household, craft and hand-tool advice from the pinned home domain.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::{
    Reading,
    understanding::{body::Part, topics::output_position},
};

use super::{fun, guards::Context};

#[path = "home_practical_data.rs"]
mod data;

use data::*;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(WORD, r"[a-z']+");
pattern!(CAR_KEYS, CAR_KEY_LOST);
pattern!(LOST_AGAIN, KEYS_AGAIN);
pattern!(KEPT_IN_ONE_PLACE, KEYS_KEPT_SOMEWHERE);
pattern!(STOOL, LOOSE_STOOL);
pattern!(CHAIR, FURNITURE_WORDS);
pattern!(DEFINES, DEFINITION_ASK);
pattern!(TAKES_UP, GETTING_INTO);
pattern!(CAST_ON, r"\bcast(?:ing)?\s+on\b");
pattern!(LASER, r"\blaser\b");
pattern!(NEEDLES, NEEDLE_SIZE);
pattern!(PURL_STITCH, PURL);
pattern!(
    TERM_END,
    r"^(?:\s+(?:mean|stand for))?(?:\s+(?:in (?:an? )?(?:knitting|crochet)(?: patterns?| charts?)?|when knitting))?\s*[?!.]*\s*$"
);

type ReplyTable = &'static [(&'static str, &'static str)];

fn word_table(table: ReplyTable) -> RegexSet {
    RegexSet::new(
        table
            .iter()
            .map(|(word, _)| format!(r"\b{}s?\b", regex::escape(word))),
    )
    .expect("authored craft terms")
}

static ABBREVIATIONS: LazyLock<RegexSet> = LazyLock::new(|| word_table(KNITTING_MEANINGS));
static CRAFTS: LazyLock<RegexSet> = LazyLock::new(|| word_table(CRAFT_KINDS));

// The source's order breaks ties when more than one project is mentioned.
const PROJECTS: &[(&str, &[&str])] = &[
    (SOCK, SOCK_REPLIES),
    (JOIN_WOOL, JOIN_WOOL_REPLIES),
    (IN_THE_ROUND, ROUND_REPLIES),
    (SEWING_MACHINE, SEWING_MACHINE_REPLIES),
    (THREAD_NEEDLE, THREAD_NEEDLE_REPLIES),
    (POM_POM, POM_POM_REPLIES),
    (PATCH, PATCH_REPLIES),
];

static PROJECT_CUES: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(PROJECTS.iter().map(|(pattern, _)| pattern)).expect("authored project cues")
});

pub(super) fn colour_pairing(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = typed_text(reading).to_lowercase();
    let mut colours = WORD.find_iter(&said).filter_map(|matched| {
        COLOUR_PARTNERS
            .iter()
            .find(|(word, _)| *word == matched.as_str())
    });
    let Some(&(first, partner)) = colours.next() else {
        let default = concat!(
            "Pick the thing you cannot change - the sofa, the floor, the tiles - and pull one colour ",
            "out of it for the walls in a paler version of itself, then one deeper tone for a rug or ",
            "cushions and a wood or metal to warm it. Three colours, one of them a neutral, and a ",
            "sample pot painted on a patch of wall you can see in the morning and at night: that is ",
            "the whole trick."
        );
        return Some(Cow::Borrowed(default));
    };
    let Some(&(second, _)) = colours.find(|(word, _)| *word != first) else {
        return Some(Cow::Owned(format!(
            "{partner} Sample it before you buy with a patch on the wall or a cushion taken home, because {first} changes with the light, and keep the rest of the room quiet so the one colour has somewhere to sit."
        )));
    };
    let reply = if COLOUR_NEUTRAL.contains(&first) && COLOUR_NEUTRAL.contains(&second) {
        let first_label = first[..1].to_ascii_uppercase() + &first[1..];
        if COLOUR_WARM.contains(&first) != COLOUR_WARM.contains(&second) {
            format!(
                "{first_label} and {second} work together, and that is the point: one is warm and one is cool, so the room has depth. Anchor them with a wood, a metal and texture - a rug, a throw - because two neutrals alone look unfinished, and put one deeper tone in the cushions."
            )
        } else {
            format!(
                "{first_label} and {second} are both quiet, so nothing in the room is doing the work: add a third colour you actually like - green, mustard or rust - plus a wood and a texture, or it will look like a show home."
            )
        }
    } else {
        format!(
            "They go together if one stays big and the other stays small: let {first} be the walls or the sofa, and keep {second} to cushions, a rug or a picture. Two strong colours at the same size fight, and a cream or white between them settles it."
        )
    };
    Some(Cow::Owned(reply))
}

pub(super) fn lost_keys(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = clean_text(reading);
    let again = LOST_AGAIN.is_match(said) && !KEPT_IN_ONE_PLACE.is_match(said);
    let reply = match (CAR_KEYS.is_match(said), again) {
        (true, true) => CAR_KEYS_AGAIN_REPLY,
        (true, false) => LOST_CAR_KEYS_REPLY,
        (false, true) => KEYS_AGAIN_REPLY,
        (false, false) => LOST_HOUSE_KEYS_REPLY,
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn chair_not_stool(reading: &Reading<'_>) -> bool {
    let said = clean_text(reading);
    not_stomach(reading) && (!STOOL.is_match(said) || CHAIR.is_match(said))
}

pub(super) fn not_stomach(reading: &Reading<'_>) -> bool {
    reading.body != Some(Part::Stomach) && output_position(&reading.line.norm).is_none()
}

pub(super) fn thing_part_not_body(reading: &Reading<'_>) -> bool {
    not_stomach(reading) && !super::body::person_part(reading)
}

pub(super) fn start_craft_applies(reading: &Reading<'_>) -> bool {
    let said = typed_text(reading);
    !DEFINES.is_match(said) && !TAKES_UP.is_match(said)
}

fn clean_text<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    }
}

fn typed_text<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.raw.is_empty() {
        &reading.line.clean
    } else {
        reading.raw
    }
}

fn craft_text(reading: &Reading<'_>) -> String {
    format!("{} {}", reading.raw, reading.line.clean).to_lowercase()
}

fn named(text: &str, regexes: &RegexSet, table: ReplyTable) -> Option<&'static str> {
    let found = regexes.matches(text).into_iter().next()?;
    Some(table[found].1)
}

fn craft_kind(text: &str) -> &'static str {
    named(text, &CRAFTS, CRAFT_KINDS).unwrap_or("knitting")
}

/// The authored term and definition in a direct craft question.
pub(super) fn named_pattern_term(reading: &Reading<'_>) -> Option<(&'static str, &'static str)> {
    let said = typed_text(reading).to_lowercase();
    let head = DEFINES.find(&said).filter(|found| found.start() == 0)?;
    let term = said[head.end()..].trim_start_matches(crate::text::is_space);
    let term = ["the ", "a ", "an "]
        .iter()
        .find_map(|article| term.strip_prefix(*article))
        .unwrap_or(term);
    KNITTING_MEANINGS.iter().copied().find(|(name, _)| {
        term.strip_prefix(*name)
            .is_some_and(|tail| TERM_END.is_match(tail.strip_prefix('s').unwrap_or(tail)))
    })
}

fn pattern_reply(context: &Context<'_>) -> &'static str {
    if let Some((_, meaning)) = named_pattern_term(context.reading) {
        return meaning;
    }
    let text = craft_text(context.reading);
    if let Some(meaning) = named(&text, &ABBREVIATIONS, KNITTING_MEANINGS) {
        return meaning;
    }
    let pool = match craft_kind(&text) {
        "knitting" if CAST_ON.is_match(&text) => CAST_ON_REPLIES,
        "crochet" => CROCHET_PATTERN_REPLIES,
        "sewing" => SEWING_PATTERN_REPLIES,
        _ => KNIT_PATTERN_REPLIES,
    };
    fun::pick(context, pool, "")
}

fn first_project(context: &Context<'_>) -> &'static str {
    let text = craft_text(context.reading);
    let pool = if let Some(index) = PROJECT_CUES.matches(&text).into_iter().next() {
        PROJECTS[index].1
    } else {
        match craft_kind(&text) {
            "sewing" => HAND_SEWING_REPLIES,
            _ if NEEDLES.is_match(&text) => NEEDLE_SIZE_REPLIES,
            "crochet" => CROCHET_START_REPLIES,
            _ if PURL_STITCH.is_match(&text) => PURL_REPLIES,
            _ => KNIT_START_REPLIES,
        }
    };
    fun::pick(context, pool, "")
}

#[derive(Clone, Copy)]
pub(super) enum Craft {
    Pattern,
    FirstProject,
    Tape,
    Level,
}

impl Craft {
    pub(super) fn answer(self, context: &Context<'_>) -> Option<Cow<'static, str>> {
        let reply = match self {
            Self::Pattern => pattern_reply(context),
            Self::FirstProject => first_project(context),
            Self::Tape => fun::pick(context, TAPE_REPLIES, ""),
            Self::Level => {
                let pool = if LASER.is_match(&craft_text(context.reading)) {
                    LASER_LEVEL_REPLIES
                } else {
                    BUBBLE_LEVEL_REPLIES
                };
                fun::pick(context, pool, "")
            }
        };
        Some(Cow::Borrowed(reply))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Exchange, Memory};

    #[test]
    fn colour_pairing_uses_the_first_two_different_colours_in_order() {
        for (line, first) in [
            ("what colour goes with navy navy", "Cream, camel and brass"),
            (
                "will cream and brown go together",
                "Cream and brown work together",
            ),
            (
                "will beige and grey go together",
                "Beige and grey are both quiet",
            ),
            ("blue and blue with orange and silver", "They go together"),
            (
                "what goes well with my room",
                "Pick the thing you cannot change",
            ),
        ] {
            let reply = colour_pairing(&Reading::new(line)).unwrap();
            assert!(reply.starts_with(first), "{line}: {reply}");
        }
        let reply = colour_pairing(&Reading::new("pink blue green")).unwrap();
        assert!(reply.contains("let pink be the walls or the sofa, and keep blue to cushions"));
    }

    #[test]
    fn key_routines_and_furniture_do_not_overwrite_the_typed_problem() {
        for (line, expected) in [
            ("i cant find my keys", LOST_HOUSE_KEYS_REPLY),
            ("i cant find my keyless fob", LOST_CAR_KEYS_REPLY),
            ("my keys go missing every single day", KEYS_AGAIN_REPLY),
            ("i keep losing my car keys", CAR_KEYS_AGAIN_REPLY),
            (
                "i keep my keys in the bowl but they are gone",
                LOST_HOUSE_KEYS_REPLY,
            ),
        ] {
            assert_eq!(lost_keys(&Reading::new(line)).as_deref(), Some(expected));
        }
        for (line, expected) in [
            ("my desk wobbles", true),
            ("how do i fix a stool with a loose leg", true),
            ("loose stools for 3 days", false),
            ("my stools are loose", false),
        ] {
            assert_eq!(chair_not_stool(&Reading::new(line)), expected, "{line}");
        }
        assert!(thing_part_not_body(&Reading::new(
            "the leg of my table aches"
        )));
        assert!(!thing_part_not_body(&Reading::new(
            "my leg aches in the car"
        )));
        assert!(!not_stomach(&Reading::new(
            "my bowels havent moved in 4 days"
        )));
    }

    #[test]
    fn craft_recognises_definitions_and_uses_the_source_seed_and_seen_replies() {
        for line in [
            "what does aran mean",
            "I want to start knitting",
            "how do I get into crochet",
        ] {
            assert!(!start_craft_applies(&Reading::new(line)), "{line}");
        }
        let input = "how do i knit socks with a sewing machine";
        let reading = Reading::new(input);
        assert!(start_craft_applies(&reading));
        let context = Context::new(&reading, &[]);
        assert_eq!(
            Craft::FirstProject.answer(&context).as_deref(),
            Some(SOCK_REPLIES[1])
        );

        let input = "how do i read a tape measure";
        let reading = Reading::new(input);
        let mut memory = Memory::default();
        let mut context = Context::new(&reading, &memory.turns);
        context.memory = Some(&memory);
        assert_eq!(
            Craft::Tape.answer(&context).as_deref(),
            Some(TAPE_REPLIES[1])
        );
        memory.turns.extend([Exchange {
            user: input.to_owned(),
            bot: TAPE_REPLIES[1].to_owned(),
            module: "answers",
        }]);
        let mut context = Context::new(&reading, &memory.turns);
        context.memory = Some(&memory);
        assert_eq!(
            Craft::Tape.answer(&context).as_deref(),
            Some(TAPE_REPLIES[0])
        );
    }

    #[test]
    fn a_craft_definition_names_the_term_as_its_subject() {
        for (line, expected) in [
            ("what is ssk in knitting", Some("ssk")),
            ("what does K2TOG mean in a knitting chart?", Some("k2tog")),
            ("what does yarn over mean in crochet", Some("yarn over")),
            ("what is a knitting pattern that uses ssk", None),
            ("what is ssk on an invoice", None),
            ("how do i use ssk in knitting", None),
        ] {
            assert_eq!(
                named_pattern_term(&Reading::new(line)).map(|(name, _)| name),
                expected,
                "{line}"
            );
        }
    }
}
