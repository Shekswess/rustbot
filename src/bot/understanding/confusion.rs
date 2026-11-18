//! Read what the speaker cannot follow, from bot/understanding/confusion.py.
use regex::Regex;
use std::sync::LazyLock;
static CONFUSION_SAID_OF_A_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?P<thing>(?:the|this|that|these|those|my|our|your|his|her|their|its|a|an)\s+(?:[a-z'-]+\s+){0,8}?[a-z'-]+)\s+(?:is|are|was|were|looks?|seems?|reads?|sounds?)\s+(?:(?:so|really|very|totally|just|completely|properly|bloody|dead|too|all)\s+)?(?P<cue>confusing|baffling|impossible|incomprehensible|unreadable|a nightmare|double dutch|gobbledygook)\b"#).unwrap()
});
static CONFUSION_POINTS_AT_NOTHING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:this|that|it|all this|all of this|everything|the whole thing)\s+(?:is|are|was)\s+(?:(?:so|really|very|totally|just|completely|properly)\s+)?(?P<cue>confusing|baffling|impossible|incomprehensible|a nightmare|double dutch)\b"#).unwrap()
});
static CONFUSION_CANNOT_FOLLOW: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:i|we)\s+(?:just\s+|still\s+|really\s+|honestly\s+|literally\s+|even\s+)?(?:can not|cannot|cant|could not|couldnt|do not|dont|did not|didnt)\s+(?P<cue>follow|understand|make\s+sense\s+of|figure\s+out|work\s+out|get)\s+(?P<thing>(?:the|this|that|these|those|my|our|your|his|her|their|its|a|an)\s+(?:[a-z'-]+\s+){0,8}?[a-z'-]+)"#).unwrap()
});
static CONFUSION_BY_A_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:i am|im|i feel|i felt|i was|i get|i got|we are|we got|were|feeling)\s+(?:(?:so|really|totally|completely|properly|just|a bit)\s+)?(?P<cue>baffled|lost|stumped|puzzled|confused|flummoxed)\s+(?:by|with|on|at)\s+(?P<thing>(?:the|this|that|these|those|my|our|your|his|her|their|its|a|an)\s+(?:[a-z'-]+\s+){0,8}?[a-z'-]+)"#).unwrap()
});
static CONFUSION_HARD_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?P<thing>(?:the|this|that|these|those|my|our|your|his|her|their|its|a|an)\s+(?:[a-z'-]+\s+){0,8}?[a-z'-]+)\s+(?:is|are|was|were)\s+(?:(?:so|really|very|too|just|almost)\s+)?(?:impossible|hard|difficult|tough|not\s+easy|awkward)\s+to\s+(?P<cue>read|follow|understand|fill\s+in|make\s+sense\s+of|work\s+out|get\s+through)\b"#).unwrap()
});
static CONFUSION_HEAD_NOR_TAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:i|we)\s+(?:(?:just|still|really)\s+)?(?:can not|cannot|cant|could not|couldnt|do not|dont|did not|didnt)\s+(?P<cue>make|tell)\s+(?:head|heads)\s+(?:nor|or)\s+tail\s+of\s+(?P<thing>(?:the|this|that|these|those|my|our|your|his|her|their|its|a|an)\s+(?:[a-z'-]+\s+){0,8}?[a-z'-]+)"#).unwrap()
});
static CONFUSION_MAKES_NO_SENSE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?P<thing>(?:the|this|that|these|those|my|our|your|his|her|their|its|a|an)\s+(?:[a-z'-]+\s+){0,8}?[a-z'-]+)\s+(?P<cue>makes|made|does not make|doesnt make|did not make|didnt make)\s+(?:any\s+|no\s+|much\s+)?sense\b"#).unwrap()
});
static CONFUSION_HEAD_SPINNING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\bmy\s+head\s+(?:is|was|'s)\s+(?:spinning|swimming|in a spin|all over the place)\b\s+(?:with|from|over)\s+(?P<thing>(?:(?:all|both)\s+(?:of\s+)?)?(?:the|this|that|these|those|my|our|your|his|her|their|its|a|an)\s+(?:[a-z'-]+\s+){0,8}?[a-z'-]+|all\s+(?:this|that|it)|everything|it)\b"#).unwrap()
});
static CONFUSION_HELPER_NOT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:can\s+not|cannot|do\s+not|dont|does\s+not|doesnt|did\s+not|didnt|could\s+not|couldnt|would\s+not|wouldnt|will\s+not|wont|shall\s+not|should\s+not|shouldnt|is\s+not|isnt|are\s+not|arent|was\s+not|wasnt|were\s+not|werent|have\s+not|havent|has\s+not|hasnt|had\s+not|hadnt|must\s+not)\s*$"#).unwrap()
});
static DENY_BEFORE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:not|no|never|neither|nor|hardly|barely|cannot|cant|dont|doesnt|didnt|isnt|arent|wasnt|werent|wont|wouldnt|shouldnt|couldnt|aint|nothing)\b(?:\s+(?:really|very|so|too|much|quite|pretty|true|honestly|actually|totally|completely|absolutely|seriously|still|even|at|all|bit|little|kind|sort|feeling|feel|felt|a|an|the|in|on|for|under|any|more|most|about|having|had|havent|been|being|be|am|is|are|was|were|got|get|getting)){0,4}\s*$"#).unwrap()
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Shape {
    NothingNamed,
    HardTo,
    SaidOfAThing,
    CannotFollow,
    ByAThing,
    HeadSpinning,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Confusion {
    pub shape: Shape,
    pub thing: Option<String>,
}

fn cut(word: &str) -> bool {
    matches!(
        word,
        "of" | "for"
            | "with"
            | "and"
            | "in"
            | "at"
            | "to"
            | "from"
            | "about"
            | "by"
            | "on"
            | "that"
            | "which"
            | "but"
            | "so"
            | "because"
            | "when"
            | "if"
            | "as"
            | "while"
    )
}
fn trailing(word: &str) -> bool {
    matches!(
        word,
        "the"
            | "a"
            | "an"
            | "it"
            | "its"
            | "is"
            | "are"
            | "was"
            | "were"
            | "this"
            | "these"
            | "those"
            | "my"
            | "our"
            | "me"
            | "i"
            | "we"
            | "you"
    )
}

fn trim_thing(thing: &str) -> Option<String> {
    let mut words: Vec<_> = thing
        .split_whitespace()
        .take_while(|word| !cut(word))
        .collect();
    while words.last().is_some_and(|word| trailing(word)) {
        words.pop();
    }
    (words.len() >= 2).then(|| words.join(" "))
}

pub(crate) fn read(norm_text: &str) -> Option<Confusion> {
    for clause in norm_text
        .split('|')
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if CONFUSION_POINTS_AT_NOTHING.is_match(clause) {
            return Some(Confusion {
                shape: Shape::NothingNamed,
                thing: None,
            });
        }
        for (pattern, shape) in [
            (&CONFUSION_HARD_TO, Shape::HardTo),
            (&CONFUSION_SAID_OF_A_THING, Shape::SaidOfAThing),
            (&CONFUSION_MAKES_NO_SENSE, Shape::SaidOfAThing),
            (&CONFUSION_CANNOT_FOLLOW, Shape::CannotFollow),
            (&CONFUSION_HEAD_NOR_TAIL, Shape::CannotFollow),
            (&CONFUSION_BY_A_THING, Shape::ByAThing),
            (&CONFUSION_HEAD_SPINNING, Shape::HeadSpinning),
        ] {
            if let Some(m) = pattern.captures(clause) {
                let cue = m.name("cue").or_else(|| m.get(0))?;
                if DENY_BEFORE.is_match(&CONFUSION_HELPER_NOT.replace(&clause[..cue.start()], "")) {
                    continue;
                }
                return Some(Confusion {
                    shape,
                    thing: m.name("thing").and_then(|s| trim_thing(s.as_str())),
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clauses_and_denials() {
        assert_eq!(
            read("not the problem | i can not follow the recipe for soup").unwrap(),
            Confusion {
                shape: Shape::CannotFollow,
                thing: Some("the recipe".into())
            }
        );
        assert_eq!(
            read("the leaflet is impossible to read").unwrap().shape,
            Shape::HardTo
        );
        assert_eq!(read("this is so confusing").unwrap().thing, None);
        assert_eq!(read("the instructions are not confusing"), None);
        assert_eq!(read("my son cant follow the manual"), None);
    }
}
