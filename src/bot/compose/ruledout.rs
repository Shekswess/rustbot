//! Respond to a constraint on the preceding answer or report that its first step failed.

#[path = "ruledout_data.rs"]
mod data;

use crate::bot::{Memory, Reading, memory, understanding::kinds::claim};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    RULED_OUT_TAIL,
    RULED_OUT_PREP,
    RULED_OUT_OPENS_WITH,
    RULED_OUT_SOFTENING,
    RULED_OUT_KEEPS,
    RULED_OUT_NAMED,
    RULED_OUT_KEEP_WHOLE,
    RULED_OUT_SYMPATHY,
    MANNERS,
    RULED_OUT_THEN_ELSE,
    CLAIM_OF_FAILURE,
    PET_NAMED,
    WANTED_ONE,
    TITLES_IN_ANSWER,
);
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static CONDITION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:if|when|once|unless|before|after|until)\b").unwrap());
static REFERRAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:gp|doctor|nurse|pharmacist|chemist|dentist|vet|optician|midwife|health visitor|111|999|911|helpline|a&e|hospital|council|citizens advice|plumber|electrician|engineer|gas safe|mechanic|locksmith|roofer|builder|technician|specialist|repair shop)\b").unwrap()
});
type RejectionPatterns = (Vec<(Regex, bool)>, Vec<(Regex, bool)>);
static REJECTIONS: LazyLock<RejectionPatterns> = LazyLock::new(|| {
    let compile = |(source, determiner): &(&str, bool)| {
        (Regex::new(source).expect("rejection pattern"), *determiner)
    };
    (
        data::RULED_OUT.iter().map(compile).collect(),
        data::RULED_OUT_NOT_THEN_ELSE
            .iter()
            .chain(data::RULED_OUT_NO_THEN_ELSE)
            .map(compile)
            .collect(),
    )
});

fn flatten(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn bare(phrase: &str) -> &str {
    RULED_OUT_KEEPS
        .find(phrase)
        .map_or(phrase, |article| &phrase[article.end()..])
}

fn rejected(clean: &str) -> Option<(String, bool)> {
    let clean = MANNERS.replace(clean, "");
    let other = RULED_OUT_THEN_ELSE.find(&clean);
    let text = other.map_or(clean.trim(), |tail| {
        clean[..tail.start()].trim_matches([' ', ',', ';', ':', '-'])
    });
    if text.split_whitespace().take(8).count() > 7 || RULED_OUT_SOFTENING.is_match(text) {
        return None;
    }
    let (thing, determiner) = REJECTIONS
        .0
        .iter()
        .chain(
            other
                .is_some()
                .then_some(&REJECTIONS.1)
                .into_iter()
                .flatten(),
        )
        .find_map(|(regex, determiner)| {
            Some((regex.captures(text)?.name("thing")?.as_str(), *determiner))
        })?;
    let thing = thing
        .split_whitespace()
        .filter(|word| !data::RULED_OUT_WORDS.contains(word))
        .collect::<Vec<_>>()
        .join(" ");
    let thing = RULED_OUT_TAIL.replace(&thing, "");
    let thing = thing.trim_matches([' ', '.', ',', '!', '?', '\'', '"']);
    if thing.is_empty()
        || RULED_OUT_PREP.is_match(thing)
        || RULED_OUT_OPENS_WITH.is_match(thing)
        || thing.split_whitespace().take(5).count() > 4
        || thing.split_whitespace().any(|word| {
            !word.bytes().all(|c| c.is_ascii_alphabetic())
                || data::RULED_OUT_QUALITY.contains(&word)
                || data::RULED_OUT_NOT_A_THING.contains(&word)
                || text.starts_with("nothing") && claim(word)
        })
    {
        return None;
    }
    Some((
        thing.to_owned(),
        determiner && RULED_OUT_KEEPS.is_match(thing),
    ))
}

fn forms_match(word: &str, present: &str, endings: &[&str]) -> bool {
    word == present
        || word.strip_suffix("es") == Some(present)
        || word.strip_suffix('s') == Some(present)
        || present
            .strip_prefix(word)
            .is_some_and(|suffix| endings.contains(&suffix))
}

fn already_names(thing: &str, answer: &str, words: &[&str]) -> bool {
    let rejected = bare(thing);
    let has_content = rejected.split_whitespace().any(|word| {
        !data::RULED_OUT_PARTICLES
            .split_ascii_whitespace()
            .any(|part| part == word)
    });
    let offers_choices = RULED_OUT_NAMED.find_iter(answer).take(2).count() == 2;
    rejected.split_whitespace().any(|word| {
        if (!has_content
            || !data::RULED_OUT_PARTICLES
                .split_ascii_whitespace()
                .any(|part| part == word))
            && words
                .iter()
                .any(|present| forms_match(word, present, &["s", "es", "ing", "ed"]))
        {
            return true;
        }
        let stem = &word[..word.len().min(7)];
        let is_class =
            |key: &str| forms_match(word, key, &["s"]) || stem.len() >= 6 && key.starts_with(stem);
        let class_word = data::RULED_OUT_CLASS.iter().any(|key| is_class(key));
        data::RULED_OUT_CLASS.iter().any(|key| {
            let members = data::RULED_OUT_INSTANCES
                .iter()
                .find_map(|(name, words)| (name == key).then_some(*words))
                .unwrap_or_default();
            if class_word && !is_class(key)
                || !class_word
                    && !members
                        .iter()
                        .any(|member| forms_match(word, member, &["s"]))
            {
                return false;
            }
            let stem = &key[..key.len().min(7)];
            !class_word && offers_choices
                || words.iter().any(|present| {
                    forms_match(key, present, &["s"])
                        || members.contains(present)
                        || stem.len() >= 6 && present.starts_with(stem)
                })
        })
    })
}

fn supporting_opening(answer: &str) -> Option<&str> {
    if answer.chars().count() <= 210 {
        return Some(answer);
    }
    let first = memory::sentences(answer).next()?;
    let first = first
        .split_once(':')
        .filter(|(head, _)| head.split_whitespace().take(9).count() <= 8)
        .map_or(first, |(_, tail)| tail.trim());
    let Some((end, _)) = first.char_indices().nth(120) else {
        return Some(first);
    };
    ["; ", ", ", " - ", " and "]
        .iter()
        .filter_map(|mark| {
            Some(first[..first[..end].rfind(mark)?].trim_matches([' ', ',', ';', ':', '-']))
        })
        .filter(|cut| cut.split_whitespace().take(4).count() == 4)
        .max_by_key(|cut| cut.chars().count())
}

fn capitalize(line: &str) -> String {
    let mut chars = line.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn failure(clean: &str, answer: &str, memory: &Memory) -> Option<String> {
    if !CLAIM_OF_FAILURE.is_match(clean) {
        return None;
    }
    const OPENERS: &str = "nothing nowt none neither of ok okay so well yeah yep yes and but oh right alright no nope nah still now";
    let words: Vec<_> = WORD
        .find_iter(clean)
        .map(|word| word.as_str())
        .skip_while(|word| OPENERS.split_ascii_whitespace().any(|part| part == *word))
        .collect();
    if !words.iter().any(|word| claim(word))
        || words.iter().any(|word| {
            !claim(word)
                && !data::CLAIM_SMALL
                    .split_ascii_whitespace()
                    .any(|small| small == *word)
        })
    {
        return None;
    }
    let step = memory::later_part(clean, answer)?;
    let conditional = CONDITION.is_match(&step);
    if step.split_whitespace().take(4).count() < 4 || !conditional && !REFERRAL.is_match(&step) {
        return None;
    }
    let pool = if conditional {
        data::CLAIM_LEADS_IF
    } else {
        data::CLAIM_LEADS
    };
    Some(render(pool, memory, clean, |line| {
        line.replace("{step}", &capitalize(&step))
    }))
}

fn render(pool: &[&str], memory: &Memory, clean: &str, fill: impl Fn(&str) -> String) -> String {
    let lines: Vec<_> = pool.iter().map(|line| fill(line)).collect();
    let choices: Vec<_> = lines.iter().map(String::as_str).collect();
    super::choose_recent(&choices, memory, clean).to_owned()
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if memory.turns.is_empty() || reading.raw.split_whitespace().take(13).count() > 12 {
        return None;
    }
    let clean = flatten(&reading.raw.to_lowercase().replace('’', "'"));
    let thing = rejected(&clean);
    if thing.is_none() && !CLAIM_OF_FAILURE.is_match(&clean) {
        return None;
    }
    let last = memory.turns.last()?;
    if matches!(
        last.module,
        "urgent" | "safety" | "loss" | "grief" | "a_count_of_packets"
    ) {
        return None;
    }
    let answer = flatten(&last.bot);
    if answer.chars().take(60).count() < 60 || answer.ends_with('?') {
        return None;
    }
    let lower = answer.to_lowercase();
    if RULED_OUT_KEEP_WHOLE.is_match(&lower)
        || RULED_OUT_SYMPATHY.is_match(&lower)
        || data::RULED_OUT_NO_ANSWER
            .iter()
            .any(|head| lower.starts_with(head))
        || super::lead_in_probe::hedged_recently(memory)
    {
        return None;
    }
    if let Some(step) = failure(&clean, &answer, memory) {
        return Some(step);
    }
    let (thing, determiner) = thing?;
    let before = last.user.to_lowercase();
    if PET_NAMED.is_match(&clean) && PET_NAMED.is_match(&before) && WANTED_ONE.is_match(&before) {
        return None;
    }
    let words: Vec<_> = WORD.find_iter(&lower).map(|word| word.as_str()).collect();
    if already_names(&thing, &answer, &words)
        || bare(&thing)
            .split_whitespace()
            .any(|word| data::KINDS_A_LIST_CAN_SWAP.contains(&word))
            && TITLES_IN_ANSWER.find_iter(&answer).take(2).count() == 2
    {
        return None;
    }
    let opening = capitalize(supporting_opening(&answer)?);
    let row = data::A_ROW_NAMES.iter().find_map(|(phrase, name)| {
        bare(&thing)
            .strip_prefix(phrase)
            .is_some_and(|tail| tail.is_empty() || tail.starts_with(' '))
            .then_some(*name)
    });
    let (pool, thing) = if let Some(row) = row {
        (data::RULED_OUT_REPLIES_A_ROW, row)
    } else if determiner {
        (data::RULED_OUT_REPLIES_THE, thing.as_str())
    } else if thing.ends_with('s') {
        (&data::RULED_OUT_REPLIES_PLAIN[1..], bare(&thing))
    } else {
        (data::RULED_OUT_REPLIES_PLAIN, bare(&thing))
    };
    Some(render(pool, memory, &clean, |line| {
        line.replace("{thing}", thing)
            .replace("{opening}", &opening)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn a_rejected_option_only_keeps_advice_that_never_offered_it() {
        let memory = |reply: &str| Memory {
            turns: vec![Exchange {
                user: "What can I do?".into(),
                bot: reply.into(),
                module: "answers",
            }]
            .into(),
            ..Memory::default()
        };
        let sleep = memory(
            "Keep regular hours, get outside in daylight, and avoid caffeine in the evening.",
        );
        assert!(
            respond(&Reading::new("not tablets"), &sleep)
                .unwrap()
                .contains("Keep regular hours")
        );
        let medicine = memory(
            "Your pharmacist can tell you whether paracetamol is suitable and help check interactions.",
        );
        assert!(respond(&Reading::new("not tablets"), &medicine).is_none());
        assert!(respond(&Reading::new("nothing that needs much water"), &sleep).is_none());
        let urgent =
            memory("Call 999 now. Stay outside and wait for the emergency service to arrive.");
        assert!(respond(&Reading::new("not a diesel"), &urgent).is_none());
    }
}
