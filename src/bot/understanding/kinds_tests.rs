use super::extra::{CarePronoun, CareShape, InterestShape, Shift};
use super::feature::AboutBotKind;
use super::time::Change;
use super::*;
use crate::bot::understanding::line::normalise;

fn read(text: &str) -> (Kind, KindFeatures) {
    super::read(&normalise(text))
}

#[test]
fn courtesy_done_and_failed_steps_have_different_intents() {
    let (kind, thanks) = read("thanks, i cleared the cache");
    assert_eq!((kind, thanks.intent), (Kind::Thanks, Intent::Thanks));
    let (kind, request) = read("thanks, i cleared the cache but it is no better");
    assert_eq!((kind, request.intent), (Kind::Statement, Intent::Request));
    assert_eq!(request.asks, ["recommend"]);
    assert!(read("thanks for any advice").1.asks.is_empty());
    assert_eq!(
        read("Thanks for telling me, I'm here if you want to dig into any of that").0,
        Kind::Thanks
    );

    let line = normalise("oh thanks, i cleared the cache");
    let tail = courtesy_tail(&line.norm).unwrap();
    assert_eq!(tail, ["i", "cleared", "the", "cache"]);
    assert!(courtesy_signoff(tail));
    assert!(courtesy_tail(&normalise("thank you card").norm).is_none());
    assert!(courtesy_trouble(&normalise("my back aches").norm));
}

#[test]
fn preserves_what_a_question_or_agreement_is_about() {
    assert!(read("what does that mean").1.asks_what_was_meant);
    assert_eq!(read("tell me about it").1.intent, Intent::Ack);
    assert_eq!(
        read("what have you been doing all day").1.about_bot_kind,
        Some(AboutBotKind::Doings)
    );
    let state = read("not feeling great").1;
    assert_eq!((state.emotion, state.polarity), (Some("sad"), -1));
}

#[test]
fn distinguishes_confirmation_from_other_uses_of_right() {
    let confirmation = read("im in a union so that should help right").1;
    assert!(confirmation.tag);
    assert_eq!(confirmation.asks, ["yesno"]);
    assert!(!read("what did you do right").1.tag);
    assert_eq!(read("is easter early or late this year").1.asks, ["yesno"]);
}

#[test]
fn welcomes_new_names_without_treating_a_feeling_as_a_name() {
    assert_eq!(read("hi im zaphod").0, Kind::Greeting);
    assert_eq!(read("hi my name is jared").0, Kind::Greeting);
    assert_eq!(read("hi im vegetarian").0, Kind::Statement);
    assert_eq!(read("hiya im stessd").0, Kind::Statement);
}

#[test]
fn separates_the_situation_before_an_ask_from_the_opening_frame() {
    let line = normalise("hey, i need a dress for a wedding, any ideas");
    assert_eq!(
        setup(&line).as_deref(),
        Some("i need a dress for a wedding")
    );
    assert!(setup(&normalise("thanks, what should i cook")).is_none());
    assert!(opening_line(&normalise("can i ask a quick question").norm));
    assert!(talk_frame(
        &normalise("right, can i get this off my chest").norm
    ));
    assert!(!talk_frame(&normalise("i need to talk to my sister").norm));
    assert!(asks_with_nothing_named(
        &normalise("what did you say first").norm
    ));
    assert!(!asks_with_nothing_named(
        &normalise("what is your advice on my cv").norm
    ));
    assert_eq!(
        after_any_link(&normalise("any word on the job").norm),
        ["the", "job"]
    );
    assert!(noun_phrase_request(&normalise(
        "things to do in lisbon with kids"
    )));
}

#[test]
fn reads_new_interests_work_shifts_and_regular_habits() {
    let interest = taking_up(&normalise("i got back into Yoga lately")).unwrap();
    assert_eq!(interest.text, "yoga");
    assert_eq!(interest.words, ["Yoga"]);
    assert_eq!(interest.shape, InterestShape::Started);
    assert!(interest.back);
    assert_eq!(shift_worked("im on lates this week"), Some(Shift::Late));
    assert_eq!(shift_worked("my period is late"), None);
    assert!(is_a_routine(
        &normalise("i walk the dog every day"),
        Kind::Statement,
        &[]
    ));
    for text in [
        "i get headaches most days",
        "i used to walk the dog every day",
    ] {
        assert!(!is_a_routine(&normalise(text), Kind::Statement, &[]));
    }
}

#[test]
fn follows_the_topic_of_a_recent_care_home_visit() {
    let previous = "my mum moved into a home last month";
    let moved = a_care_home(previous).unwrap();
    assert_eq!(
        (moved.pronoun, moved.shape),
        (CarePronoun::She, CareShape::Moved)
    );
    assert_eq!(moved.relation.as_deref(), Some("mum"));
    let carried =
        a_home_carried_forward("what should i bring her", ["not sure when", previous]).unwrap();
    assert_eq!(
        (carried.who.as_str(), carried.shape),
        ("my mum", CareShape::Bring)
    );
    assert_eq!(carried.carried.as_deref(), Some(previous));
    assert!(a_home_carried_forward("what should i bring", [previous]).is_none());
}

#[test]
fn distinguishes_live_answers_settled_facts_and_unknowns() {
    use TimeDependence::{Changing, Settled, Unknown};
    for (text, expected) in [
        ("what is the latest score", Changing(Change::News)),
        ("how much is a single to leeds", Changing(Change::Price)),
        ("what is inflation", Settled),
        ("what is the weather", Changing(Change::Weather)),
        ("when should i get the bus", Unknown),
        ("how many people live in leeds", Settled),
        ("why is my train late today", Changing(Change::Timetable)),
        ("how do weather forecasts get it wrong", Settled),
        ("is the 8.15 running", Changing(Change::Timetable)),
        (
            "are there tickets available",
            Changing(Change::Availability),
        ),
        ("whos the prime minister", Changing(Change::Post)),
        ("opening hours", Changing(Change::Hours)),
        ("what is the event on tonight", Changing(Change::Event)),
        ("the weather looks bad this weekend", Unknown),
        ("how much was 23 - 4", Settled),
    ] {
        let (kind, features) = read(text);
        let result = over_time(&normalise(text), kind, &features);
        assert_eq!(result, expected, "{text}");
    }
}

#[test]
fn carries_a_live_question_only_into_a_short_new_followup() {
    let raw = "and a return";
    let line = normalise(raw);
    let (kind, features) = read(raw);
    let price = TimeDependence::Changing(Change::Price);
    let earlier = Some(("how much is a single to leeds", price));
    assert_eq!(over_time(&line, kind, &features), TimeDependence::Unknown);
    assert_eq!(
        over_time_followup(&line, kind, &features, raw, true, earlier),
        Some(price)
    );
    assert_eq!(
        over_time_followup(&line, kind, &features, raw, false, earlier),
        None
    );
    assert_eq!(
        over_time_followup(&line, kind, &features, raw, true, Some((raw, price))),
        None
    );
}
