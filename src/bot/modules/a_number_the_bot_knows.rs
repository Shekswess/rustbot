//! Route questions about known national services to their published contact details.

use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;

struct Service {
    names: &'static [&'static str],
    digits: Option<&'static str>,
    reply: &'static str,
}

const SERVICES: &[Service] = &[
    // https://www.samaritans.org/how-we-can-help/contact-samaritan/talk-us-phone/
    Service {
        names: &["samaritans", "samaritan"],
        digits: Some("116123"),
        reply: "In the UK and Ireland, Samaritans is free to call at any hour on 116 123.",
    },
    // https://www.nationaldahelpline.org.uk/
    Service {
        names: &[
            "domestic abuse helpline",
            "domestic violence helpline",
            "national domestic abuse helpline",
            "refuge",
            "womens aid",
            "women's aid",
        ],
        digits: Some("08082000247"),
        reply: "In England, Refuge's National Domestic Abuse Helpline is free to call 24 hours a day on 0808 2000 247.",
    },
    // https://www.acas.org.uk/contact
    Service {
        names: &["acas"],
        digits: Some("03001231100"),
        reply: "Call Acas on 0300 123 1100 for advice about work. Calls are charged at your provider's usual national rate.",
    },
    // https://111.nhs.uk/
    Service {
        names: &["nhs 111", "111", "nhs direct"],
        digits: Some("111"),
        reply: "In England, call NHS 111 on 111 for urgent medical advice. The service is free and open any time.",
    },
    // https://988lifeline.org/
    Service {
        names: &["988"],
        digits: Some("988"),
        reply: "In the US, you can call or text 988 for the Suicide & Crisis Lifeline, free and available at any hour.",
    },
    // https://www.gov.uk/guidance/999-and-112-the-uks-national-emergency-numbers
    Service {
        names: &["999", "911", "112", "emergency services"],
        digits: Some("999"),
        reply: "Call 999 or 112 in the UK, 911 in the US or Canada, or your local emergency number elsewhere. Tell them where you are and what happened.",
    },
    Service {
        names: &["childline"],
        digits: None,
        reply: "Use Childline's official site, childline.org.uk, to find its phone and online support options.",
    },
    Service {
        names: &["alcoholics anonymous", "al anon", "alanon"],
        digits: None,
        reply: "Use the official website for Alcoholics Anonymous or Al-Anon to find the contact for your area.",
    },
    Service {
        names: &[
            "mind",
            "shelter",
            "stepchange",
            "age uk",
            "ageuk",
            "victim support",
            "cruse",
            "nspcc",
            "rethink",
        ],
        digits: None,
        reply: "Go to the organisation's official website for its helpline details and current opening hours.",
    },
];

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

pattern!(
    LEAD,
    r"^(?:sorry|hi|hello|hey|ok|okay|oh|so|and|also|er|um)\s*[,!.]\s*"
);
pattern!(
    TAIL,
    r"\s*\b(?:again|please|one more time|now|thanks|thank you)\b\s*[?!.]*$"
);
pattern!(
    AGAIN,
    r"\b(?:again|what was (?:it|that)|remind me|one more time|i missed (?:it|that))\b"
);
pattern!(
    NEXT,
    r"^(?:and|also|what about|how about|whats|what is|what's)\b[\s,]*(?:the\s+|a\s+)?(?:number\s+(?:for|of)\s+)?(?:for\s+|of\s+)?(.{2,40})$"
);
pattern!(BARE, r"^[a-z][a-z' ]{1,30}$");
static ASKS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"\b(?:phone\s+|telephone\s+)?number\s+(?:for|of|to)\s+(.{2,60})$",
        r"\b(?:how do i|how can i|how to|where do i|can i|do i)\s+(?:ring|call|phone|dial|contact|reach)\s+(.{2,60})$",
        r"\b(?:phone\s+)?number\s+(?:for\s+)?(.{2,40}?)\s+again$",
        r"^(.{2,40}?)(?:'s|s)?\s+(?:phone\s+)?number$",
        r"\b([a-z][a-z' ]{1,40}?)\s+(?:phone\s+)?number$",
    ].into_iter()
        .map(|pattern| Regex::new(pattern).expect("valid contact-number wording"))
        .collect()
});

fn strip(text: &str) -> &str {
    text.trim_matches([' ', '?', '!', '.', ',', '\'', '"'])
}

fn asked(line: &str) -> Option<String> {
    let tail = TAIL.replace(strip(line), "");
    let text = LEAD.replace(strip(&tail), "");
    ASKS.iter().find_map(|ask| {
        let found = ask.captures(strip(&text))?;
        let thing = strip(found.get(1)?.as_str());
        let name = ["the ", "a ", "an "]
            .iter()
            .find_map(|prefix| thing.strip_prefix(prefix))
            .unwrap_or(thing);
        (!name.is_empty()).then(|| name.to_owned())
    })
}

fn carried(line: &str) -> Option<String> {
    let text = strip(line);
    if let Some(found) = NEXT.captures(text) {
        return Some(strip(found.get(1)?.as_str()).to_owned());
    }
    BARE.is_match(text).then(|| text.to_owned())
}

fn says_name(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(index, part)| {
        !text[..index].ends_with(|letter: char| letter.is_ascii_alphanumeric())
            && !text[index + part.len()..]
                .starts_with(|letter: char| letter.is_ascii_alphanumeric())
    })
}

fn said_number(reply: &str, digits: &str) -> bool {
    let no_spaces: String = reply
        .chars()
        .filter(|letter| !letter.is_whitespace())
        .collect();
    no_spaces.match_indices(digits).any(|(index, number)| {
        !no_spaces[..index].ends_with(|letter: char| letter.is_ascii_digit())
            && !no_spaces[index + number.len()..]
                .starts_with(|letter: char| letter.is_ascii_digit())
    })
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.raw.replace('’', "'").to_lowercase();
    let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
    let again = AGAIN.is_match(&line);
    let previous = memory.turns.last().and_then(|turn| {
        SERVICES
            .iter()
            .find(|service| service.reply == turn.bot)
            .map(|service| service.reply)
    });
    let named = asked(&line).or_else(|| previous.and_then(|_| carried(&line)));
    if let Some(service) = named.as_deref().and_then(|name| {
        SERVICES
            .iter()
            .find(|service| service.names.iter().any(|alias| says_name(name, alias)))
    }) {
        if !again
            && service.digits.is_some_and(|digits| {
                memory
                    .turns
                    .iter()
                    .rev()
                    .take(3)
                    .any(|turn| said_number(&turn.bot, digits))
            })
        {
            return None;
        }
        return Some(service.reply.to_owned());
    }
    previous.filter(|_| again).map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn published_numbers_match_whole_names_and_repeat_when_requested() {
        let mut memory = Memory::default();
        let reply = respond(&Reading::new("What's the number for NHS 111?"), &memory).unwrap();
        assert!(reply.contains("111") && reply.contains("England"));
        assert!(respond(&Reading::new("What's the number for 1111?"), &memory).is_none());
        assert!(respond(&Reading::new("Refugees phone number"), &memory).is_none());
        memory.turns.push(Exchange {
            user: String::new(),
            bot: reply.clone(),
            module: "a_number_the_bot_knows",
        });
        assert!(respond(&Reading::new("NHS 111 number"), &memory).is_none());
        assert_eq!(
            respond(&Reading::new("Sorry, what was it again?"), &memory),
            Some(reply)
        );
        assert!(
            respond(&Reading::new("And for Childline?"), &memory)
                .unwrap()
                .contains("childline.org.uk")
        );
    }

    #[test]
    fn national_number_wins_before_the_directory_fallback() {
        let mut bot = Bot::new(Some("1")).unwrap();
        let answer = bot.turn("What's the number for the Samaritans?");
        assert!(answer.reply.contains("116 123"), "{:?}", answer.candidates);
    }
}
