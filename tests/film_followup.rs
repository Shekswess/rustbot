use rustbot::Bot;

const FAMILY_FILM: &str = "Can you suggest a film we could watch as a family?";
const SHORT_FILM: &str = "Something light that isn't too long.";
const METHOD_ALTERNATE: &str = "That's the lot I have on that one. Tell me which bit isn't landing and I'll try a different angle.";

#[test]
fn film_qualifier_uses_the_selected_family_card_when_the_contraction_is_familiar() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn(FAMILY_FILM);
    let turn = bot.turn(SHORT_FILM);
    assert_eq!(turn.module, Some("answers"));
    assert!(turn.reply.starts_with("*Before Sunset* runs 80 minutes,"));

    let mut bot = Bot::new(Some("0")).unwrap();
    for text in [
        "How do I compare two savings accounts?",
        "How can I get a better deal?",
        "How do I give a talk at work?",
    ] {
        bot.turn(text);
    }
    let handoff = bot.turn("How do I ask for feedback afterwards?");
    assert_eq!(handoff.module, Some("greeting"));
    assert_eq!(handoff.reply, METHOD_ALTERNATE);
    bot.turn(FAMILY_FILM);
    let turn = bot.turn(SHORT_FILM);
    assert_eq!(turn.module, Some("answers"));
    assert!(turn.reply.starts_with("For a laugh: *Groundhog Day*"));
}

#[test]
fn an_unanswered_appliance_report_can_lend_the_older_room_project_to_a_bare_followup() {
    let mut bot = Bot::new(Some("0")).unwrap();
    for text in [
        "I want to paint the spare room.",
        "How much paint do I need for a room four metres by three metres?",
        "And what about the ceiling?",
        "How do I fix a door hinge that squeaks?",
        "My washing machine is making a knocking sound.",
        "What's the first thing to check?",
        "How do I give a talk at work?",
        "I've never done one before.",
    ] {
        bot.turn(text);
    }
    let slide = "What should I put on the first slide?";
    let turn = bot.turn(slide);
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(
        turn.reply,
        rustbot::knowledge_by_id("decorating", "paint-runs-and-sags")
            .unwrap()
            .reply
    );
    let turn = bot.turn("How do I ask for feedback afterwards?");
    assert_eq!(turn.module, Some("greeting"));
    assert_eq!(turn.reply, METHOD_ALTERNATE);

    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("I want to paint the spare room.");
    bot.turn("How do I give a talk at work?");
    bot.turn("I've never done one before.");
    assert!(!bot.turn(slide).reply.starts_with("Load less:"));
}
