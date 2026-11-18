//! Remember one group age list and recall it in its original order.

use crate::bot::{Memory, Reading};
use fancy_regex::Regex;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "groupages_data.rs"]
mod data;
#[path = "groupages_store.rs"]
mod store;
use data::*;

pattern!(AGES, r"\d{1,3}");
pattern!(WORDS, r"[a-z']+");
static GROUP: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(&format!(r"\b(?:{})\b", GROUP_WORDS.join("|"))).expect("known family group words")
});

#[derive(Default)]
pub(crate) struct Facts {
    latest: Option<AgeList>,
    stored: store::Stored,
}

struct AgeList {
    group: &'static str,
    ages: Vec<u16>,
}

pub(crate) fn decimal(text: &str) -> Option<u16> {
    text.chars()
        .try_fold(0u16, |number, next| {
            let value = u32::from(next);
            let zero = DECIMAL_ZEROES
                .partition_point(|zero| *zero <= value)
                .checked_sub(1)?;
            let digit = value - DECIMAL_ZEROES[zero];
            if digit >= 10 {
                return None;
            }
            number.checked_mul(10)?.checked_add(digit as u16)
        })
        .filter(|_| !text.is_empty())
}

fn group(text: &str) -> Option<&'static str> {
    let canonical = match text {
        "child" => "children",
        "kid" => "kids",
        "grandchild" | "grandkids" => "grandchildren",
        "stepchild" => "stepchildren",
        "stepkid" => "stepkids",
        other => other,
    };
    GROUP_WORDS
        .iter()
        .copied()
        .find(|group| *group == canonical)
}

fn the_list_given(text: &str) -> Option<AgeList> {
    [&GIVES_A_LIST, &GIVES_A_LIST_BARE, &THEYRE_A_LIST]
        .into_iter()
        .find_map(|pattern| {
            let found = pattern.captures(text).ok().flatten()?;
            let group = group(&found.name("word")?.as_str().to_lowercase())?;
            let ages: Vec<_> = AGES
                .find_iter(found.name("ages")?.as_str())
                .flatten()
                .filter_map(|age| decimal(age.as_str()))
                .collect();
            (ages.len() >= 2).then_some(AgeList { group, ages })
        })
}

impl Facts {
    pub(crate) fn observe(&mut self, raw: &str) {
        if let Some(given) = the_list_given(raw) {
            self.latest = Some(given);
        }
        self.stored.observe(raw);
    }
}

fn belongs_to_group(group: &str, word: &str) -> bool {
    group.strip_suffix('s') == Some(word)
        || SINGULARS
            .iter()
            .any(|(label, words)| *label == group && words.contains(&word))
}

fn answer(raw: &str, facts: &Facts) -> Option<String> {
    let stored = facts.latest.as_ref()?;
    let low = raw.to_lowercase();
    let named: Vec<_> = GROUP
        .find_iter(&low)
        .flatten()
        .filter_map(|word| group(word.as_str()))
        .collect();
    if !named.is_empty() && !named.contains(&stored.group) {
        return None;
    }
    if let Some(which) = ASKS_THE_END_ONE.captures(raw).ok().flatten() {
        let tail = raw[which.get(0)?.end()..].to_lowercase();
        let next = WORDS.find(&tail).ok().flatten().map(|word| word.as_str());
        if next.is_some_and(|word| {
            !matches!(word, "one" | "ones" | "of") && !belongs_to_group(stored.group, word)
        }) {
            return None;
        }
        let youngest = matches!(
            which.name("which")?.as_str().to_lowercase().as_str(),
            "youngest" | "younger" | "little" | "little one"
        );
        let (which, age) = if youngest {
            ("youngest", stored.ages.iter().min()?)
        } else {
            ("oldest", stored.ages.iter().max()?)
        };
        if named.is_empty() {
            return Some(format!("Your {which} is {age}."));
        }
        let word = stored.group.strip_suffix('s').unwrap_or(stored.group);
        return Some(format!("Your {which} {word} is {age}."));
    }
    if !ASKS_THE_LIST.is_match(raw).unwrap_or(false) {
        return None;
    }
    let (last, previous) = stored.ages.split_last()?;
    let previous = previous
        .iter()
        .map(u16::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!("Your {} are {previous} and {last}.", stored.group))
}

pub(crate) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    answer(reading.raw, &memory.group_ages)
}

pub(crate) fn store_reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let answer = store::reply(reading, memory, &memory.group_ages.stored)?;
    Some(super::frame_reply(&answer, reading.raw, memory).unwrap_or(answer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_age_list_replaces_the_previous_group_and_keeps_the_reported_order() {
        let mut state = Facts::default();
        assert_eq!(answer("how old are they", &state), None);
        state.observe("my parents are 66 and 70");
        assert_eq!(
            answer("how old are they", &state).as_deref(),
            Some("Your parents are 66 and 70.")
        );
        state.observe("my two kids theyre 7 and 4");
        state.observe("my kids 3 and 8 times");
        assert_eq!(
            answer("what are their ages", &state).as_deref(),
            Some("Your kids are 7 and 4.")
        );
        assert_eq!(answer("how old are my parents", &state), None);
        assert_eq!(
            answer("how old is my youngest kid", &state).as_deref(),
            Some("Your youngest is 4.")
        );
        assert_eq!(answer("how old is my oldest cat", &state), None);
        state.observe("actually my kids are 8 and 5");
        assert_eq!(
            answer("how old is my oldest one", &state).as_deref(),
            Some("Your oldest is 8.")
        );
    }

    #[test]
    fn the_source_patterns_require_two_numeric_ages_and_normalize_grandkids() {
        let mut state = Facts::default();
        for line in [
            "my parents are 66 and 70.",
            "my son is 7",
            "my parents are sixty and seventy",
        ] {
            state.observe(line);
            assert_eq!(answer("how old are they", &state), None, "{line}");
        }
        state.observe("our grandkids are ٣ and 9 and 6");
        assert_eq!(
            answer("how old are my grandchildren", &state).as_deref(),
            Some("Your grandchildren are 3, 9 and 6.")
        );
    }

    #[test]
    fn per_relation_store_keeps_older_lists_and_frames_a_repeat() {
        let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
        bot.turn("my parents are 66 and 70");
        bot.turn("my daughters are 6 and 9");
        let parents = bot.turn("how old are my parents");
        assert_eq!(parents.module, Some("memory_store"));
        assert_eq!(parents.reply, "Your parents are 66 and 70.");
        let daughters = bot.turn("how old are my daughters");
        assert_eq!(daughters.module, Some("group_ages"));
        assert_eq!(daughters.reply, "Your daughters are 6 and 9.");
        let repeated = bot.turn("what are their ages");
        assert_eq!(repeated.module, Some("memory_store"));
        assert_eq!(
            repeated.reply,
            "Same as before: Your daughters are 6 and 9."
        );
    }

    #[test]
    fn store_age_lists_have_their_own_statement_syntax_and_corrections() {
        let mut memory = Memory::default();
        let parents = Reading::new("how old are my parents");
        memory.group_ages.observe("my parents are 66 and 70.");
        assert_eq!(reply(&parents, &memory), None);
        assert_eq!(
            store_reply(&parents, &memory).as_deref(),
            Some("Your parents are 66 and 70.")
        );
        memory.group_ages.observe("my parents are 67 and 71.");
        assert_eq!(
            store_reply(
                &Reading::new("how old is the earth in our family?"),
                &memory
            ),
            None
        );
        assert_eq!(
            store_reply(&parents, &memory).as_deref(),
            Some("Your parents are 67 and 71.")
        );
        memory.group_ages.observe("my parents are 70");
        assert_eq!(store_reply(&parents, &memory), None);
    }
}
