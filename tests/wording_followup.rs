use rustbot::Bot;

const METHOD_AFTER_ANSWER: &str = "That's most of what I know on it — say which part is still unclear and I'll come at it another way.";
const HOUSEWARMING: &str = "What could I bring to a friend's housewarming?";
const WRITE_NOTE: &str = "How do I write a short note to go with it?";

#[test]
fn a_generic_note_method_uses_the_prior_answer_and_explicit_card_wording_has_its_own_reply() {
    let mut bot = Bot::new(Some("0")).unwrap();
    let turn = bot.turn(WRITE_NOTE);
    assert_eq!(turn.module, Some("greeting"));
    assert_eq!(
        turn.reply,
        "Happy to help — tell me what you've got so far and I'll add what I can."
    );

    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn(HOUSEWARMING);
    let turn = bot.turn(WRITE_NOTE);
    assert_eq!(turn.module, Some("greeting"));
    assert_eq!(turn.reply, METHOD_AFTER_ANSWER);
    assert!(
        turn.candidates
            .iter()
            .all(|candidate| candidate.module != "answers")
    );

    let turn = bot.turn("What should I write in the card?");
    assert_eq!(turn.module, Some("answers"));
    assert!(
        turn.reply
            .starts_with("Name one real thing about them and one wish")
    );
}
