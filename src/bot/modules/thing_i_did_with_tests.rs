use crate::bot::{Bot, Turn};

fn has(turn: &Turn, module: &str) -> Option<String> {
    turn.candidates
        .iter()
        .find(|candidate| candidate.module == module)
        .map(|candidate| candidate.reply.clone())
}

fn chat(history: &[&str], ask: &str) -> Turn {
    let mut bot = Bot::new(Some("0")).unwrap();
    for line in history {
        bot.turn(line);
    }
    bot.turn(ask)
}

#[test]
fn prefers_the_line_that_names_the_doing_and_uses_repeat_wording() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("I ordered a coat in a medium last Tuesday");
    bot.turn("The coat arrived scratched");
    let first = bot.turn("which coat was it i ordered");
    assert_eq!(first.module, Some("thing_i_did_with"));
    assert_eq!(
        first.reply,
        "You said: \"I ordered a coat in a medium last Tuesday\""
    );
    let repeated = bot.turn("what size coat did i order");
    assert_eq!(repeated.module, Some("thing_i_did_with"));
    assert_eq!(
        repeated.reply,
        "Same line again: \"I ordered a coat in a medium last Tuesday\""
    );
}

#[test]
fn lets_the_store_answer_the_recorded_model_or_choice() {
    let phone = chat(&["my phone is a pixel"], "what phone have i got");
    assert_eq!(phone.module, Some("memory_store"));
    assert_eq!(phone.reply, "Your phone is a Pixel.");
    assert_eq!(has(&phone, "thing_i_did_with"), None);

    let choice = chat(
        &[
            "i chose the red helmet",
            "which helmet was that i chose",
            "which helmet is it i chose!",
        ],
        "what model helmet did i choose.",
    );
    assert_eq!(choice.module, Some("memory_store"));
    assert_eq!(choice.reply, "You settled on the red helmet.");
    assert_eq!(has(&choice, "thing_i_did_with"), None);
}

#[test]
fn leaves_a_specialist_card_beside_the_omitted_detail_question() {
    let phone = chat(
        &[
            "my phone is a pixel",
            "what phone have i got",
            "which phone did i buy",
        ],
        "what brand phone did i buy",
    );
    assert_eq!(phone.module, Some("answers"));
    assert_eq!(
        has(&phone, "memory_store").as_deref(),
        Some("You haven't said - what brand phone did you buy?")
    );
    assert_eq!(has(&phone, "thing_i_did_with"), None);

    let laptop = chat(&["my laptop is a Dell"], "what color laptop did i buy");
    assert_eq!(
        has(&laptop, "memory_store").as_deref(),
        Some("You haven't said - what colour laptop did you buy?")
    );
    assert_eq!(laptop.module, Some("said_fact_back"));
    assert_eq!(has(&laptop, "thing_i_did_with"), None);
}

#[test]
fn requires_a_line_from_the_user_and_a_kept_fact_for_a_missing_part() {
    let no_coat = chat(&["What coat should I order?"], "which coat did i order");
    assert_eq!(has(&no_coat, "thing_i_did_with"), None);

    let untyped = chat(&["i bought a phone in May"], "what brand phone did i buy");
    assert_eq!(untyped.module, Some("thing_i_did_with"));
    assert_eq!(has(&untyped, "memory_store"), None);
}

#[test]
fn frames_a_possession_fact_already_given() {
    let repeated = chat(
        &[
            "The car in the driveway is my Fiesta",
            "what car did i buy",
            "which car did i say",
        ],
        "what car did i drive",
    );
    assert_eq!(repeated.module, Some("memory_store"));
    assert_eq!(repeated.reply, "Same as before: You drive a Fiesta.");
}
