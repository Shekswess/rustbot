//! Direct doubt replies from PyBot 7d63c7f.

use super::*;
use crate::bot::Exchange;

type Case = (
    &'static str,
    &'static [(&'static str, &'static str, &'static str)],
    Option<&'static str>,
);

const CASES: &[Case] = &[
    (
        "That makes no sense.",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "That doesnt make sense at all.",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "That made no sense to me.",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        None,
    ),
    (
        "Youre not making sense.",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "You're confusing me.",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        Some("That is on me. Which bit was the muddled one?"),
    ),
    (
        "You’re confusing me.",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "You are confused me.",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        Some("That is on me. Which bit was the muddled one?"),
    ),
    (
        "U R confusing me.",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "That can't be right.",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        Some("Then I may have it wrong. Say what you were expecting and we'll compare the two."),
    ),
    (
        "That cannot be right!",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        Some("Then I may have it wrong. Say what you were expecting and we'll compare the two."),
    ),
    (
        "Are you sure?",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."),
    ),
    (
        "Are you certain?",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."),
    ),
    (
        "You sure though?",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        Some("Then I may have it wrong. Say what you were expecting and we'll compare the two."),
    ),
    (
        "That’s nonsense.",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "This is rubbish.",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        Some(
            "It could be out of date, that is a real risk with these things. Point at the part and I will go through it.",
        ),
    ),
    (
        "Thats wrong, mate.",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "No way is that right.",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        None,
    ),
    (
        "I don't follow",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        None,
    ),
    (
        "I dont get it",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("That is on me. Which bit was the muddled one?"),
    ),
    (
        "I don’t get this tbh",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "Are you sure it is two weeks?",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "That makes no sense the council called again.",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "It doesnt make sense to buy new.",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        None,
    ),
    (
        "That makes no sense, the council said they would ring me.",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "Sorry, that makes no sense tbh.",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "What doesnt make sense?",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "Are you sure, that makes no sense.",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("That is on me. Which bit was the muddled one?"),
    ),
    (
        "I am sure that what you put doesnt make sense at all.",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        None,
    ),
    (
        "You sure too?",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "That cannot be right, at all.",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "That can’t be right. Is it?",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        None,
    ),
    (
        "It makes no sense, tbh mate!",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "Sorry but I think this makes no sense.",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "I cannot follow the story, that makes no sense.",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        Some("That is on me. Which bit was the muddled one?"),
    ),
    (
        "THAT MAKES NO SENSE!!",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("That is on me. Which bit was the muddled one?"),
    ),
    (
        "That makes no sense.",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "\tThat makes   no sense\t",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "the bank was closed",
        &[],
        Some(
            "I'm not sure which bank you mean - a word or two about it and I'll answer the right one.",
        ),
    ),
    (
        "the seal is broken",
        &[],
        Some("Which seal do you mean? Tell me a bit more and I'll take it from there."),
    ),
    ("the trunk is open", &[], None),
    ("the spring is broken", &[], None),
    ("is a bat a mammal", &[], None),
    ("the spring was beautiful", &[], None),
    ("the bat broke", &[], None),
    ("light is gone", &[], None),
    ("shall i have a light", &[], None),
    ("that makes no sense", &[], None),
    ("are you sure", &[], None),
    ("this sounds wrong", &[], None),
    ("the harbour is nice", &[], None),
    ("the crane is broken", &[], None),
    ("the crane is standing in the field", &[], None),
    ("i want to open a bank account", &[], None),
    (
        "I dont follow",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "I dont follow",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        None,
    ),
    (
        "I dont follow",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        None,
    ),
    (
        "I dont follow",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "I dont follow",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "I dont follow",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "I dont follow",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "I dont follow",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "That makes no sense",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "That makes no sense",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "That makes no sense",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "That makes no sense",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "That makes no sense",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "That makes no sense",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "That makes no sense",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "That makes no sense",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "Are you sure?",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "Are you sure?",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        Some("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."),
    ),
    (
        "Are you sure?",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        Some("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."),
    ),
    (
        "Are you sure?",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."),
    ),
    (
        "Are you sure?",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."),
    ),
    (
        "Are you sure?",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        Some("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."),
    ),
    (
        "Are you sure?",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "Are you sure?",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        Some("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."),
    ),
    (
        "I don't get it",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "I don't get it",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        None,
    ),
    (
        "I don't get it",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        None,
    ),
    (
        "I don't get it",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "I don't get it",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "I don't get it",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "I don't get it",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "I don't get it",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        Some("Fair enough. Say which part and I will put it another way."),
    ),
    (
        "You're confusing me",
        &[("What can I do?", "A reply.", "greeting")],
        None,
    ),
    (
        "You're confusing me",
        &[(
            "How do I boil an egg?",
            "Start the eggs in already boiling water, then simmer and cool them in cold water.",
            "answers",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "You're confusing me",
        &[(
            "Which one?",
            "The second one is the one to start with, then check the instruction on the box.",
            "chat",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "You're confusing me",
        &[(
            "What about the timer?",
            "Noted, the timer is the next part of it. We can see what it does afterwards.",
            "greeting",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "You're confusing me",
        &[(
            "What about the place?",
            "It is not something I can get from here. The seller is the one who can say.",
            "greeting",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "You're confusing me",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
    (
        "You're confusing me",
        &[("Ready?", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "chat")],
        None,
    ),
    (
        "You're confusing me",
        &[(
            "What to do now?",
            "Leave the building and call the local gas emergency service once you're outside.",
            "safety",
        )],
        Some("Let me try that again, plainer. Which bit lost you?"),
    ),
];

#[test]
fn doubt_preserves_the_previous_answer_fence_and_seeded_wording() {
    let mut misses = Vec::new();
    for &(raw, earlier, expected) in CASES {
        let memory = Memory {
            turns: earlier
                .iter()
                .map(|&(user, bot, module)| Exchange {
                    user: user.to_owned(),
                    bot: bot.to_owned(),
                    module,
                })
                .collect(),
            ..Memory::default()
        };
        let reading = Reading::new(raw);
        let actual = respond(&reading, &memory);
        if actual.as_deref() != expected {
            misses.push((
                raw,
                earlier,
                expected,
                actual,
                reading.context_fields.subject_unclear,
            ));
        }
    }
    assert!(misses.is_empty(), "{misses:#?}");
}
