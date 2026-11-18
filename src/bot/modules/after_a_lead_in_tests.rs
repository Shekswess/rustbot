//! Direct lead-in replies and receipt checks from PyBot 7d63c7f.

use super::*;
use crate::bot::Exchange;

#[test]
fn probing_without_history_restores_the_transcript_and_its_index() {
    let exchange = |reply: &str| Exchange {
        user: "fixture".into(),
        bot: reply.into(),
        module: "answers",
    };
    let mut memory = Memory::default();
    memory.turns.record(exchange("Remember this."));
    {
        let empty = WithoutTurns::new(&mut memory);
        assert_eq!(empty.memory.turns.first_reply("Remember this."), None);
        empty.memory.turns.record(exchange("Only in the probe."));
    }
    assert_eq!(memory.turns.first_reply("REMEMBER THIS."), Some(0));
    assert_eq!(memory.turns.first_reply("Only in the probe."), None);
}
use crate::bot::Bot;

type Case = (
    &'static str,
    &'static [(&'static str, &'static str, &'static str)],
    Option<&'static str>,
);

const CASES: &[Case] = &[
    ("And is B&Q open on Sunday?", &[], None),
    ("And is the museum free?", &[], None),
    ("And is the library open Sunday?", &[], None),
    ("Okay, so, and is B&Q open on Sunday?", &[], None),
    ("And can I bring a bike on the train?", &[], None),
    ("So, when is the museum open?", &[], None),
    ("Right, what time does the supermarket close?", &[], None),
    ("And can I make an oatcake?", &[], None),
    ("But where can I get a parking permit?", &[], None),
    ("Also, what time is my flight?", &[], None),
    ("Plus, are the term dates out?", &[], None),
    ("And is the next train delayed?", &[], None),
    ("Anyway, do you know the ticket price?", &[], None),
    ("But, are they open now?", &[], None),
    ("And is that okay to do?", &[], None),
    ("And do you know my name?", &[], None),
    ("And what about it?", &[], None),
    ("And is it?", &[], None),
    ("I’m still here.", &[], None),
    ("And my mum is in hospital.", &[], None),
    ("And it was lovely.", &[], None),
    ("And does this count?", &[], None),
    ("And can you smell gas?", &[], None),
    ("And, is my local shop open tonight?", &[], None),
    ("What time does the museum open?", &[], None),
    ("And is the bank open?", &[], None),
    ("Oh, and how much does parking cost?", &[], None),
    ("And show me a cheap phone.", &[], None),
    ("But can a rabbit see colours?", &[], None),
    ("Oh and when does that start?", &[], None),
    ("And when is the match on?", &[], None),
    ("And when is the next bank holiday?", &[], None),
    ("Do they still do that?", &[], None),
    ("And I’m riding my bike tomorrow.", &[], None),
    ("And how does the fridge work?", &[], None),
    ("Right then - is it open now?", &[], None),
    ("And  is B&Q\topen on Sunday?", &[], None),
    ("and is the library open Sundays.", &[], None),
    (
        "And is B&Q open on Sunday?",
        &[(
            "I want to paint the hall.",
            "Start with the walls, then cut in around the skirting boards.",
            "answers",
        )],
        None,
    ),
    (
        "And is B&Q open on Sunday?",
        &[(
            "The boiler is making a noise.",
            "Tell me what sort of noise it makes and we can work from there.",
            "chat",
        )],
        None,
    ),
    (
        "And is B&Q open on Sunday?",
        &[(
            "Have you got plans?",
            "There is time for a walk if you want to fit something in.",
            "greeting",
        )],
        None,
    ),
    (
        "And is the supermarket open?",
        &[(
            "I want to paint the hall.",
            "Start with the walls, then cut in around the skirting boards.",
            "answers",
        )],
        None,
    ),
    (
        "And is the supermarket open?",
        &[(
            "The boiler is making a noise.",
            "Tell me what sort of noise it makes and we can work from there.",
            "chat",
        )],
        None,
    ),
    (
        "And is the supermarket open?",
        &[(
            "Have you got plans?",
            "There is time for a walk if you want to fit something in.",
            "greeting",
        )],
        None,
    ),
    (
        "And when does it open?",
        &[(
            "I want to paint the hall.",
            "Start with the walls, then cut in around the skirting boards.",
            "answers",
        )],
        None,
    ),
    (
        "And when does it open?",
        &[(
            "The boiler is making a noise.",
            "Tell me what sort of noise it makes and we can work from there.",
            "chat",
        )],
        None,
    ),
    (
        "And when does it open?",
        &[(
            "Have you got plans?",
            "There is time for a walk if you want to fit something in.",
            "greeting",
        )],
        None,
    ),
    (
        "And what about my coat?",
        &[(
            "I want to paint the hall.",
            "Start with the walls, then cut in around the skirting boards.",
            "answers",
        )],
        None,
    ),
    (
        "And what about my coat?",
        &[(
            "The boiler is making a noise.",
            "Tell me what sort of noise it makes and we can work from there.",
            "chat",
        )],
        None,
    ),
    (
        "And what about my coat?",
        &[(
            "Have you got plans?",
            "There is time for a walk if you want to fit something in.",
            "greeting",
        )],
        None,
    ),
    (
        "And what about it?",
        &[(
            "I want to paint the hall.",
            "Start with the walls, then cut in around the skirting boards.",
            "answers",
        )],
        None,
    ),
    (
        "And what about it?",
        &[(
            "The boiler is making a noise.",
            "Tell me what sort of noise it makes and we can work from there.",
            "chat",
        )],
        None,
    ),
    (
        "And what about it?",
        &[(
            "Have you got plans?",
            "There is time for a walk if you want to fit something in.",
            "greeting",
        )],
        Some("'That' has nothing behind it here. What is the thing you're pointing at?"),
    ),
    (
        "And can you smell gas?",
        &[(
            "I want to paint the hall.",
            "Start with the walls, then cut in around the skirting boards.",
            "answers",
        )],
        None,
    ),
    (
        "And can you smell gas?",
        &[(
            "The boiler is making a noise.",
            "Tell me what sort of noise it makes and we can work from there.",
            "chat",
        )],
        None,
    ),
    (
        "And can you smell gas?",
        &[(
            "Have you got plans?",
            "There is time for a walk if you want to fit something in.",
            "greeting",
        )],
        None,
    ),
    (
        "And how long does the painting take?",
        &[(
            "I want to paint the hall.",
            "Start with the walls, then cut in around the skirting boards.",
            "answers",
        )],
        None,
    ),
    (
        "And how long does the painting take?",
        &[(
            "The boiler is making a noise.",
            "Tell me what sort of noise it makes and we can work from there.",
            "chat",
        )],
        None,
    ),
    (
        "And how long does the painting take?",
        &[(
            "Have you got plans?",
            "There is time for a walk if you want to fit something in.",
            "greeting",
        )],
        None,
    ),
    (
        "and is b&q open on sunday",
        &[],
        Some(
            "The opening hours are the first thing a venue changes and the last thing it tells anyone: a refurb, a bank holiday or new owners will do it. Their own page or map listing has the current ones.",
        ),
    ),
    (
        "and is b&q open on sunday",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        Some(
            "The opening hours are the first thing a venue changes and the last thing it tells anyone: a refurb, a bank holiday or new owners will do it. Their own page or map listing has the current ones.",
        ),
    ),
    ("and is the library open sunday", &[], None),
    (
        "and is the library open sunday",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        None,
    ),
    ("and is the co op open on sunday", &[], None),
    (
        "and is the co op open on sunday",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        None,
    ),
    ("and is the council office open on sunday", &[], None),
    (
        "and is the council office open on sunday",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        None,
    ),
    ("so can i take a bike on the train", &[], None),
    ("and what time is the flight", &[], None),
    ("and is there a pharmacy open today", &[], None),
    (
        "and is there a pharmacy open today",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        None,
    ),
    ("so how much does a stamp cost", &[], None),
    ("and is it raining tomorrow", &[], None),
    ("and are there any trains to brighton tomorrow", &[], None),
    ("well what time does tescos close", &[], None),
    ("and is the museum open today", &[], None),
    (
        "and is the museum open today",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        None,
    ),
    ("and what should i do", &[], None),
    ("and how do i make a boat", &[], None),
    ("and how can i start", &[], None),
    ("well what is the next bus to oxford", &[], None),
    ("and where are my keys", &[], None),
    ("and is the library open today?", &[], None),
    (
        "and is the library open today?",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        None,
    ),
    ("and is b&q open on sunday?", &[], None),
    (
        "and is b&q open on sunday?",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        None,
    ),
    ("and when is the museum open", &[], None),
    (
        "and when is the museum open",
        &[(
            "The boiler makes a noise.",
            "Humming and banging are two different things.",
            "answers",
        )],
        None,
    ),
    ("and is this private", &[], None),
    ("and are you connected to the internet", &[], None),
    ("and can you speak spanish", &[], None),
    ("and can you smell gas", &[], None),
    ("and can a rabbit see colours", &[], None),
];

#[test]
fn lead_in_replays_the_authored_fallback_probes_without_retaining_any_turn() {
    let mut misses = Vec::new();
    for &(raw, earlier, expected) in CASES {
        let mut bot = Bot::new(Some("0")).unwrap();
        let mut control = Bot::new(Some("0")).unwrap();
        let memory = &mut bot.memory;
        memory.turns = earlier
            .iter()
            .map(|&(user, bot, module)| Exchange {
                user: user.to_owned(),
                bot: bot.to_owned(),
                module,
            })
            .collect();
        let reading = Reading::understand(
            raw,
            memory.turns.iter().map(|turn| turn.user.as_str()),
            None,
            &memory.prior_readings,
        );
        let actual = respond(&reading, memory);
        if actual.as_deref() != expected {
            misses.push((raw, earlier, expected, actual));
        }
        for (seen, &(user, bot, module)) in memory.turns.iter().zip(earlier) {
            assert_eq!(
                (seen.user.as_str(), seen.bot.as_str(), seen.module),
                (user, bot, module)
            );
        }
        assert_eq!(memory.turns.len(), earlier.len(), "{raw}");
        assert_eq!(
            compose::draw(&mut memory.rng.get_mut().0, 19),
            compose::draw(&mut control.memory.rng.get_mut().0, 19),
            "{raw}"
        );
    }
    assert!(misses.is_empty(), "{misses:#?}");
}

#[test]
fn lead_in_identifies_receipts_and_echoes() {
    let cases = [
        ("", "", true, true),
        ("Got it – thanks for telling me.", "ok thanks", true, false),
        ("The bins — noted. Is it a lot?", "the bins", true, false),
        ("Noted: .", "noted", true, true),
        ("Your bike, good to know.", "your bike", true, false),
        (
            "Is B&Q open on Sunday. Not long now, then.",
            "And is B&Q open on Sunday?",
            false,
            true,
        ),
        (
            "The opening hours are set by the shop.",
            "And is the shop open?",
            false,
            false,
        ),
        ("It’s a plan. More soon.", "my plan", false, false),
        ("The floor awaits", "open", false, false),
        ("Ask away, I’m listening", "open", true, false),
        (
            "What should we do. No clue.",
            "what should we do",
            false,
            false,
        ),
    ];
    for (reply, ask, floor, echoed) in cases {
        assert_eq!(floor_or_nod(reply), floor, "{reply}");
        assert_eq!(echo(reply, ask), echoed, "{reply} to {ask}");
    }
}
