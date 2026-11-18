//! Scoped follow-ups to a selected safety reply about someone else or domestic abuse.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Exchange, Memory, Reading};

#[path = "carers_turn_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(data::$name).expect(stringify!($name))
        });)+
    };
}
patterns!(
    ROUTE_NAMED,
    THEY_ARE_EASING,
    EASING_UNDONE,
    THEM_ASKING,
    EASING_NOT_ONE,
    MOVE_ON,
    THEY_SAY_ITS_FINE,
    ABUSE_ANSWER,
    ABUSER_ANSWER,
    THE_APOLOGY,
    THE_KINDNESS,
    THE_DRINK,
    THEIR_OWN_FAULT,
    THREAT_AGAIN,
    NODDING_ALONG,
    THE_TROUBLE_IS_OVER,
    NO_WAY_OUT,
    SWALLOWED_NOT_RELIEF,
    TOO_MUCH_TO_CARRY,
    THE_ANSWER_ASKED_A_QUESTION,
    ANSWERED,
    ABOUT_THE_THREAD,
    CHANGE_THE_SUBJECT,
    ABOUT_THE_INCIDENT,
    PACKING,
    THE_FEAR_IT_REPEATS,
);

const MONTHS: &str = r"(?:\d+|a|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a\s+few|several)\s+(?:days?|weeks?|months?|years?)";
static RECOVERING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"\b(?:sober|clean|dry|straight|drugs?[- ]?free|off\s+(?:it|the\s+drink|the\s+booze)|self[- ]harm\s+free)\b[\w\s,'-]{{0,14}}?\b{MONTHS}|\b{MONTHS}\s+(?:sober|clean|dry)\b|\b(?:hasn'?t|has\s+not|haven'?t|have\s+not|hadn'?t)\s+(?:touched|used|drunk|drank|self[- ]harmed|cut|relapsed)\b[\w\s,'-]{{0,14}}?\b(?:in|for)\s+{MONTHS}"
    ))
    .expect("reported recovery over time")
});
static RELATIVE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:my|our)\s+(?P<who>mum|mam|mom|mother|dad|father|son|daughter|baby|child|kid|",
        r"brother|sister|step(?:son|daughter|dad|mum)|partner|husband|wife|boyfriend|girlfriend|",
        r"grandma|grandad|grandpa|grandmother|grandfather|nan|nana|gran|granny|aunt|uncle|",
        r"cousin|niece|nephew|neighbor|neighbour|friend|flatmate|housemate|patient)\b"
    ))
    .expect("person looked after")
});
static ANOTHER_PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:he|she|they)(?:'s|s)?\s|\b(?:his|her|their)\s+(?:head|breathing|face|body|neck|lips|eyes|colour|pulse)\b")
        .expect("reported other person")
});
static OTHER_TOPIC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:tax return|for (?:a|my|our|the) (?:flight|trip|holiday|train|course)|leave (?:my|the|our) (?:job|work|course|school))\b")
        .expect("unrelated follow-up subjects")
});
static PLAN: LazyLock<Regex> = LazyLock::new(|| {
    let end = r"(?:\s+(?:now|next|though|then))?\s*(?:[.?!,]|$)";
    Regex::new(&format!(
        concat!(
            r"\b(?:should|shall|do|can|could|will) (?:i|we) (?:leave|go|stay|move|get out|walk out)\b{end}",
            r"|\bwhere (?:do|can|should) i (?:go|stay|sleep tonight)\b{end}",
            r"|\b(?:do i need to (?:leave|go|get out)|what (?:should|do|can|could) i do|what now)\b{end}",
            r"|\bshould i (?:go back|stay with (?:him|her|them))\b",
            r"|\b(?:how|when) (?:do|can|should|would) i (?:get|collect|pick up|take|move) (?:my|the|our)\s+",
            r"(?:stuff|things|belongings|clothes|papers|paperwork|documents|passport)\b",
            r"|\bcan i take (?:the|my) (?:kids|children)\b",
            r"|\bwhat (?:do|should|can|could) i (?:tell|say to) (?:the|my|our)\s+",
            r"(?:kids|children|son|daughter|family|mum|mom|mother|parents|boss)\b{end}",
            r"|\bhow long (?:will|does|is) (?:this|it|all this) (?:take|going to take|going to go on)\b"
        ),
        end = end,
    ))
    .expect("abuse safety-planning questions")
});

const CRISIS: &[&str] = &[
    "crisis",
    "116 123",
    "988",
    "samaritans",
    "helpline",
    "stay with you",
];
const END_SUBJECT: &[&str] = &[
    "Of course. What would you rather talk about?",
    "We can leave that there. What's on your mind instead?",
];

fn cleaned(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn abuse_reply(reply: &str) -> bool {
    let line = reply.to_lowercase();
    ABUSE_ANSWER.is_match(&line) && !ABUSER_ANSWER.is_match(&line)
}

fn safety_turn(turn: &Exchange) -> bool {
    matches!(turn.module, "safety" | "safety_cards")
}

fn anchor(memory: &Memory) -> Option<(&Exchange, bool)> {
    let mut turns = memory.turns.iter().rev();
    let last = turns.next()?;
    if safety_turn(last) {
        return Some((last, abuse_reply(&last.bot)));
    }
    if last.module != "carers_turn" || CHANGE_THE_SUBJECT.is_match(&cleaned(&last.user)) {
        return None;
    }
    let above = turns.find(|turn| {
        turn.module != "carers_turn" || CHANGE_THE_SUBJECT.is_match(&cleaned(&turn.user))
    })?;
    (safety_turn(above) && abuse_reply(&above.bot)).then_some((above, true))
}

fn pick(pool: &[&str], clean: &str, memory: &Memory) -> String {
    let previous = memory.turns.last().map(|turn| turn.bot.trim());
    let fresh: Vec<_> = pool
        .iter()
        .copied()
        .filter(|reply| Some(*reply) != previous)
        .collect();
    let pool = if fresh.is_empty() { pool } else { &fresh };
    let salt = clean.chars().map(|letter| letter as usize).sum::<usize>();
    pool[salt % pool.len()].to_owned()
}

// https://www.nhs.uk/live-well/getting-help-for-domestic-violence/
fn abuse_followup(clean: &str, memory: &Memory) -> Option<String> {
    if OTHER_TOPIC.is_match(clean) {
        return None;
    }
    let pool: &[&str] = if PACKING.is_match(clean) {
        &[
            "If you need to leave, prioritize safety. Take your phone, keys, medicines and essential ID only if they are within reach. A domestic abuse service can help you plan what to take or retrieve later.",
        ]
    } else if PLAN.is_match(clean) {
        &[
            "If you are in immediate danger, call 999 in the UK. If you can talk safely, ask a domestic abuse service or someone you trust to help you plan where to go and how to get there.",
            "You can talk through a plan with a domestic abuse service before deciding what to do. If it is safe, tell a trusted person what is happening and where you could go. In immediate danger, call 999 in the UK.",
        ]
    } else if THEIR_OWN_FAULT.is_match(clean) {
        &[
            "Their decision to hurt you is their responsibility. You deserve to be safe. If you can, involve someone you trust or a domestic abuse service.",
            "You are entitled to safety, whatever was said before they hurt you. A domestic abuse service can help you work out your options when you can speak safely.",
        ]
    } else if NO_WAY_OUT.is_match(clean) {
        &[
            "A domestic abuse service can help you look for a safe place or refuge and talk through your options. If your device is monitored, use a safer phone if you can. Call 999 in the UK if you are in immediate danger.",
        ]
    } else if THREAT_AGAIN.is_match(clean) || THE_FEAR_IT_REPEATS.is_match(clean) {
        &[
            "Take the threat seriously. If you are in danger now, call 999 in the UK. If you can do so safely, get near people you trust and contact a domestic abuse service to plan what to do next.",
        ]
    } else if THE_DRINK.is_match(clean) {
        &[
            "Drinking can affect behaviour. Their decision to harm you is their responsibility. If you can, speak with someone you trust or a domestic abuse service about your safety.",
        ]
    } else if THE_KINDNESS.is_match(clean) || THE_APOLOGY.is_match(clean) {
        &[
            "An apology can be hard to weigh alongside what happened. Take your safety concerns seriously. If you can speak safely, talk with someone you trust or a domestic abuse service about your options.",
            "Kind words can feel reassuring after being hurt. Keep any safety plan you have and talk it through with someone you trust or a domestic abuse service if you can.",
        ]
    } else if TOO_MUCH_TO_CARRY.is_match(clean) {
        &[
            "Start with one person you trust if you can contact them safely. You can tell them what happened and ask them to stay with you or help you reach a domestic abuse service.",
        ]
    } else if ABOUT_THE_THREAD.is_match(clean) || ABOUT_THE_INCIDENT.is_match(clean) {
        &[
            "You deserve support with what happened. If it is safe to talk, a domestic abuse service or someone you trust can help you decide what feels safest now.",
        ]
    } else {
        return None;
    };
    Some(pick(pool, clean, memory))
}

fn easing(clean: &str) -> bool {
    THEY_ARE_EASING.is_match(clean)
        && !EASING_NOT_ONE.is_match(clean)
        && !MOVE_ON.is_match(clean)
        && !THEM_ASKING.is_match(clean)
}

// https://www.nhs.uk/symptoms/what-to-do-if-someone-has-a-seizure-fit/
// https://www.nhs.uk/conditions/poisoning/
fn carer_followup(
    clean: &str,
    above: &Exchange,
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<String> {
    let prior = cleaned(&above.user);
    let relative = RELATIVE
        .captures(&prior)
        .and_then(|found| found.name("who"));
    if relative.is_none() && !ANOTHER_PERSON.is_match(&prior) {
        return None;
    }
    let who = relative.map_or_else(
        || "the person you're with".to_owned(),
        |relation| format!("your {}", relation.as_str()),
    );
    let last = above.bot.to_lowercase();
    if !clean.contains('?') && THEY_ARE_EASING.is_match(clean) && EASING_UNDONE.is_match(clean) {
        return Some(format!(
            "Call 999 now in the UK, or the local emergency number, for severe confusion, an abnormal colour or breathing changes after coming round. Stay with {who} and tell the call handler what happened and when."
        ));
    }
    let improving = easing(clean);
    if improving && (SWALLOWED_NOT_RELIEF.is_match(&prior) || last.contains("possible poisoning")) {
        let past_reading = Reading::new(&above.user);
        if super::poison::explicit_dose(&past_reading)
            || super::poison::is_child_dose(&past_reading)
        {
            return Some(super::poison::emergency(reading, memory));
        }
        return Some("Get emergency advice for possible poisoning now. Call 999 in the UK if they may have swallowed something harmful. Tell the call handler what it was and when, keep the packaging and stay with them.".to_owned());
    }
    if improving && THE_ANSWER_ASKED_A_QUESTION.is_match(&above.bot) && !ANSWERED.is_match(clean) {
        return Some(format!(
            "It is encouraging that {who} is more responsive. Keep the earlier question open and watch for confusion, unusual colour or trouble breathing. Call 999 in the UK if those symptoms appear."
        ));
    }
    if CRISIS.iter().any(|word| last.contains(word)) {
        if THEY_SAY_ITS_FINE.is_match(clean) {
            return Some(format!(
                "It helps that {who} is talking to you. Ask directly how safe they feel and whether they can reach their crisis team or someone they trust. If there is immediate danger, call 999 in the UK."
            ));
        }
        if RECOVERING.is_match(clean) {
            return Some(format!(
                "That is a milestone for {who}. Ask what kind of support they would like to keep going, and keep a way to reach their support team if things become difficult again."
            ));
        }
        if improving {
            return Some(format!(
                "I’m glad things have eased for {who}. Keep their support plan and help them get professional advice if they feel unsafe again. In immediate danger, call 999 in the UK."
            ));
        }
        return None;
    }
    if improving && ROUTE_NAMED.is_match(&last) {
        return Some(format!(
            "It is encouraging that {who} is responding again. Follow the advice you were already given and let the clinician know how they are now. If they become hard to wake, confused or breathe abnormally, call 999 in the UK."
        ));
    }
    None
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let (above, abuse) = anchor(memory)?;
    let clean = cleaned(reading.raw);
    if clean.is_empty() || NODDING_ALONG.is_match(&clean) || THE_TROUBLE_IS_OVER.is_match(&clean) {
        return None;
    }
    if CHANGE_THE_SUBJECT.is_match(&clean) {
        return Some(pick(END_SUBJECT, &clean, memory));
    }
    if abuse {
        abuse_followup(&clean, memory)
    } else {
        carer_followup(&clean, above, reading, memory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    fn safety(user: &str, bot: &str) -> Exchange {
        Exchange {
            user: user.into(),
            bot: bot.into(),
            module: "safety",
        }
    }

    #[test]
    fn carers_turn_reads_only_the_latest_safety_thread() {
        let mut memory = Memory::default();
        let apology = Reading::new("He says he's sorry");
        assert!(respond(&apology, &memory).is_none());
        memory.turns.push(safety(
            "my boyfriend hit me",
            "Contact a domestic abuse helpline when safe.",
        ));
        let reply = respond(&apology, &memory).unwrap();
        assert!(reply.contains("safety") || reply.contains("safety plan"));
        memory.turns.push(Exchange {
            user: apology.raw.to_owned(),
            bot: reply,
            module: "carers_turn",
        });
        assert!(
            respond(&Reading::new("Should I leave?"), &memory)
                .unwrap()
                .contains("domestic abuse")
        );
        assert!(respond(&Reading::new("What do I do about my tax return?"), &memory).is_none());
        memory.turns.push(safety(
            "my dad fell down the stairs",
            "Call NHS 111 for urgent advice.",
        ));
        assert!(respond(&apology, &memory).is_none());
        assert!(
            respond(&Reading::new("He's awake now"), &memory)
                .unwrap()
                .contains("your dad")
        );
        assert!(
            respond(&Reading::new("He's come round but he's confused"), &memory)
                .unwrap()
                .contains("Call 999 now")
        );
    }

    #[test]
    fn carers_turn_keeps_the_safety_plan_over_unrelated_cards() {
        let mut control = Bot::new(Some("0")).unwrap();
        assert_ne!(
            control.turn("My boyfriend hit me up about dinner").module,
            Some("safety")
        );
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(bot.turn("My boyfriend hit me").module, Some("safety"));
        let apology = bot.turn("He says he's sorry");
        assert_eq!(apology.module, Some("carers_turn"), "{apology:?}");
        let plan = bot.turn("Should I leave?");
        assert_eq!(plan.module, Some("carers_turn"), "{plan:?}");
        let other = bot.turn("Can we talk about something else?");
        assert_eq!(other.module, Some("carers_turn"), "{other:?}");
        assert_ne!(
            bot.turn("Should I leave my job?").module,
            Some("carers_turn")
        );
    }
}
