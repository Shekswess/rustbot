use super::*;
use crate::bot::Bot;

#[test]
fn uses_the_first_matching_doneness_key_from_the_current_subject() {
    let earlier = ["my oven runs a bit cool", "a slow roast shoulder of lamb"];
    assert_eq!(
        fill("  HOW   will i know when it's done  ", true, &earlier).as_deref(),
        Some("lamb how will i know when it's done")
    );
    assert_eq!(
        fill("how will i know when it's done", false, &earlier),
        None
    );
    assert_eq!(fill("can we cook it faster", true, &earlier), None);
    assert_eq!(fill("is it ready yet", true, &["we are out all day"]), None);
    assert_eq!(
        fill(
            "how do i know its cooked",
            true,
            &["my car needs a service", "im roasting a chicken"]
        ),
        None
    );
}

#[test]
fn updates_the_lexical_view_of_the_active_read() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("a slow roast shoulder of lamb");
    bot.turn("my oven runs a bit cool");

    let raw = "how will i know when it's done";
    let mut reading = Reading::new(raw);
    let kind = reading.kind;
    let norm = reading.line.norm.clone();
    observe(&mut reading, &mut bot.memory);

    assert_eq!(reading.raw, raw);
    assert_eq!(reading.kind, kind);
    assert_eq!(reading.line.norm, norm);
    assert_eq!(reading.clean, "lamb how will i know when it's done");
    assert_eq!(reading.line.norm_text, reading.clean);
    assert_eq!(
        reading.line.words,
        ["lamb", "how", "will", "i", "know", "when", "it", "done"]
    );
}

#[test]
fn reads_the_session_topic_and_preserves_a_new_domain() {
    let cases = [
        (
            &["a slow roast shoulder of lamb", "my oven runs a bit cool"][..],
            "how will i know when it's done",
            Some((
                "answers",
                "Lamb is done by temperature in the thickest part: 55C is pink, 60C is medium, 70C is well done, and it is juiciest if you take it out 5C under and rest it 10 minutes. A leg roasts 25 minutes per 500g at 180C (160C fan) plus 25 minutes on top, and a chop takes 3 minutes a side for pink.",
            )),
        ),
        (
            &[
                "how long should i roast lamb",
                "now my car keeps overheating",
            ][..],
            "how do i know its cooked",
            None,
        ),
        (&["we are out all day"][..], "is it ready yet", None),
    ];

    for (history, raw, expected) in cases {
        let mut bot = Bot::new(Some("0")).unwrap();
        for text in history {
            bot.turn(text);
        }
        let turn = bot.turn(raw);
        match expected {
            Some((module, reply)) => {
                assert_eq!(turn.module, Some(module));
                assert_eq!(turn.reply, reply);
            }
            None => assert!(
                turn.candidates.iter().all(|one| one.module != "answers"),
                "{turn:?}"
            ),
        }
    }
}

#[test]
fn does_not_fill_a_question_during_an_active_emergency() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("a slow roast shoulder of lamb");
    bot.turn("my oven runs a bit cool");
    let ask = "how will i know when it's done";
    assert!(fill(ask, true, &bot.memory.doneness_topics.texts()).is_some());

    assert_eq!(
        bot.turn("i smell gas in the kitchen").module,
        Some("safety")
    );
    let mut reading = Reading::new(ask);
    observe(&mut reading, &mut bot.memory);
    assert_eq!(reading.clean, ask);
    assert!(reading.lexical_words().is_none());
}
