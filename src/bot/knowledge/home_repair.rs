//! Repair decisions, owner checks and first steps for a stopped appliance.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::{Memory, compose, modules::safety::current_gas};

use super::super::home_guards;
use super::{guards::Context, history_cards::whole_words, home_fittings::GAS_EMERGENCY};

#[path = "home_repair_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));
    };
}
pattern!(DECIDING_NOW);
pattern!(CHOOSING);
pattern!(NEED_A_NEW);
pattern!(SELF_JOB_NOW);
pattern!(CALLING_SOMEONE);
pattern!(SHOPPING_NOW);
pattern!(FAULT_NOW);
pattern!(OWNED_NOW);
pattern!(WORTH_NOW);
pattern!(KEEPING_NOW);
pattern!(YEARS_OLD);
pattern!(OTHER_FAULT);
pattern!(COLD_STOP);

static AGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(\d{1,2})\s*(?:year|yr)s?\b").expect("appliance age"));
static NOT_A_MACHINE: LazyLock<RegexSet> = LazyLock::new(|| whole_words(data::NOT_A_MACHINE));
static GENERIC_MACHINE: LazyLock<RegexSet> =
    LazyLock::new(|| whole_words(&["machine", "machines", "appliance", "appliances"]));
static WHITE_GOODS: LazyLock<Names> = LazyLock::new(|| Names::new(data::WHITE_GOODS));
static SMALL_APPLIANCES: LazyLock<Names> = LazyLock::new(|| Names::new(data::SMALL_APPLIANCES));
static GARDEN_MACHINES: LazyLock<Names> = LazyLock::new(|| Names::new(data::GARDEN_MACHINES));

struct Names {
    aliases: RegexSet,
    rows: &'static [(&'static str, &'static str)],
}

impl Names {
    fn new(rows: &'static [(&'static str, &'static str)]) -> Self {
        let aliases = RegexSet::new(
            rows.iter()
                .map(|(name, _)| format!(r"\b{}s?\b", regex::escape(name))),
        )
        .expect("appliance names");
        Self { aliases, rows }
    }

    fn find(&self, line: &str) -> Option<&'static str> {
        let index = self.aliases.matches(line).iter().next()?;
        Some(self.rows[index].1)
    }
}

fn machine_named(line: &str) -> Option<&'static str> {
    SMALL_APPLIANCES
        .find(line)
        .or_else(|| GARDEN_MACHINES.find(line))
        .or_else(|| WHITE_GOODS.find(line))
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Kind {
    Diy,
    Decision,
    Stopped,
}

impl Kind {
    pub(super) fn reply(self, context: &Context<'_>) -> Option<Cow<'static, str>> {
        match self {
            Self::Diy => diy(context).map(Cow::Borrowed),
            Self::Decision => decision(context),
            Self::Stopped => stopped(context),
        }
    }
}

fn diy(context: &Context<'_>) -> Option<&'static str> {
    for turn in context.earlier.iter().rev().take(2) {
        let said = turn.user.to_lowercase();
        let line = said.split_whitespace().collect::<Vec<_>>().join(" ");
        if data::DIY_EARLIER.iter().any(|term| line.contains(term)) {
            return None;
        }
    }
    Some(data::DIY)
}

fn article(machine: &str) -> &'static str {
    if machine.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    }
}

fn pool(
    context: &Context<'_>,
    choices: &[&str],
    machine: &str,
    age: Option<u8>,
) -> Cow<'static, str> {
    let articles = if article(machine) == "an" {
        ("an", "An")
    } else {
        ("a", "A")
    };
    let age = age.map(|age| {
        data::AGES
            .get(usize::from(age).saturating_sub(1))
            .map_or_else(|| format!("{age} years"), |words| (*words).to_owned())
    });
    let options: Vec<_> = choices
        .iter()
        .map(|choice| {
            let reply = choice
                .replace("{Article}", articles.1)
                .replace("{article}", articles.0)
                .replace("{machine}", machine);
            match &age {
                Some(age) => reply.replace("{age}", age),
                None => reply,
            }
        })
        .collect();
    let options: Vec<_> = options.iter().map(String::as_str).collect();
    let empty;
    let memory = match context.memory {
        Some(memory) => memory,
        None => {
            empty = Memory::default();
            &empty
        }
    };
    Cow::Owned(compose::pick(&options, memory, "").to_owned())
}

fn age_in(line: &str) -> Option<u8> {
    YEARS_OLD
        .captures(line)
        .or_else(|| AGE.captures(line))?
        .get(1)?
        .as_str()
        .parse()
        .ok()
}

fn typed<'a>(context: &'a Context<'_>) -> &'a str {
    if context.reading.raw.is_empty() {
        &context.reading.line.clean
    } else {
        context.reading.raw
    }
}

fn decision(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let line = typed(context).to_lowercase();
    let skill_asked = SELF_JOB_NOW.is_match(&line);
    let need_new = NEED_A_NEW.is_match(&line);
    let fault = FAULT_NOW.is_match(&line);
    let shopping = SHOPPING_NOW.is_match(&line) && !CALLING_SOMEONE.is_match(&line);
    if !(DECIDING_NOW.is_match(&line) || skill_asked || need_new)
        || (shopping && !CHOOSING.is_match(&line) && !fault)
        || !(OWNED_NOW.is_match(&line) || need_new || fault)
    {
        return None;
    }

    let said = context.reading.line.clean.to_lowercase();
    let skill = skill_asked && !WORTH_NOW.is_match(&line);
    if !(fault || FAULT_NOW.is_match(&said) || KEEPING_NOW.is_match(&line) || skill || need_new) {
        return None;
    }
    if current_gas(&line) {
        return Some(Cow::Borrowed(GAS_EMERGENCY));
    }
    if let Some(small) = SMALL_APPLIANCES
        .find(&line)
        .or_else(|| SMALL_APPLIANCES.find(&said))
    {
        return Some(pool(context, data::SMALL, small, None));
    }
    if let Some(garden) = GARDEN_MACHINES
        .find(&line)
        .or_else(|| GARDEN_MACHINES.find(&said))
    {
        return Some(pool(context, data::GARDEN, garden, None));
    }
    if skill {
        return Some(diy_decision(context, &said));
    }

    let machine = WHITE_GOODS.find(&said).unwrap_or("machine");
    if machine == "boiler" {
        return Some(Cow::Borrowed(data::BOILER_DECISION));
    }
    let age = age_in(&line)
        .filter(|age| *age != 0)
        .or_else(|| age_in(&said));
    let reply = if age.is_some_and(|age| age >= 8) {
        pool(context, data::OLDER, machine, age)
    } else {
        pool(context, data::REPAIR, machine, None)
    };
    Some(reply)
}

fn diy_decision(context: &Context<'_>, said: &str) -> Cow<'static, str> {
    let machine = machine_named(said).or_else(|| {
        let previous = context.earlier.last()?.user.to_lowercase();
        machine_named(&previous)
    });
    let Some(machine) = machine else {
        return Cow::Borrowed(data::DIY_UNKNOWN);
    };
    match machine {
        "boiler" => return Cow::Borrowed(data::BOILER_DIY),
        "microwave" => return Cow::Borrowed(data::MICROWAVE_DIY),
        _ => {}
    }
    let article = article(machine);
    let parts = data::DIY_PARTS
        .iter()
        .find_map(|&(name, parts)| (name == machine).then_some(parts));
    let reply = if let Some(parts) = parts {
        format!(
            "For {article} {machine}, {parts}. Check the manual and model number before ordering any part."
        )
    } else {
        format!(
            "On {article} {machine}, stick to the checks and part swaps marked for the owner in the manual. Switch it off and unplug it if applicable before doing any work. Leave the motor and wiring to a qualified repairer."
        )
    };
    Cow::Owned(reply)
}

fn stopped(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let line = format!("{} {}", typed(context), context.reading.line.clean).to_lowercase();
    if NOT_A_MACHINE.is_match(&line)
        || (!OTHER_FAULT.is_match(&line) && !home_guards::thing_has_gone(context.reading))
    {
        return None;
    }
    let small = SMALL_APPLIANCES.find(&line);
    let machine = small.or_else(|| WHITE_GOODS.find(&line));
    if matches!(machine, Some("fridge" | "freezer")) && COLD_STOP.is_match(&line) {
        return None;
    }
    if machine.is_none() && !GENERIC_MACHINE.is_match(&line) {
        return None;
    }

    if let Some(machine) = small {
        return Some(pool(context, data::STOPPED_SMALL, machine, None));
    }
    let label = machine.map_or_else(|| "machines".to_owned(), |machine| format!("{machine}s"));
    Some(pool(context, data::STOPPED, &label, None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Exchange, Reading};

    #[test]
    fn repair_cards_preserve_decision_and_abstention_branches() {
        let answer = |kind: Kind, text: &str| {
            let reading = Reading::new(text);
            kind.reply(&Context::new(&reading, &[]))
        };
        assert!(
            answer(Kind::Decision, "Is my 12 year old washer worth repairing?").is_some_and(
                |reply| reply.contains("twelve years") && reply.contains("washing machine")
            )
        );
        assert!(answer(Kind::Decision, "Is it worth buying a new tumble dryer?").is_none());
        assert!(
            answer(Kind::Decision, "Can I repair my microwave myself?")
                .is_some_and(|reply| reply.contains("dangerous voltage"))
        );
        assert!(
            answer(Kind::Stopped, "My kettle has gone")
                .is_some_and(|reply| reply.contains("kettle"))
        );
        assert!(answer(Kind::Stopped, "My fridge stopped working").is_none());
        assert!(
            answer(
                Kind::Stopped,
                "My starter has stopped bubbling in the fridge"
            )
            .is_none()
        );
        assert!(answer(Kind::Stopped, "What should I do when the internet breaks?").is_none());

        let reading = Reading::new("Can I fix it myself?");
        let previous = [Exchange {
            user: "My bike chain keeps slipping".into(),
            bot: String::new(),
            module: "",
        }];
        assert!(Kind::Diy.reply(&Context::new(&reading, &[])).is_some());
        assert!(
            Kind::Diy
                .reply(&Context::new(&reading, &previous))
                .is_none()
        );
    }
}
