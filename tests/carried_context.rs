use rustbot::{Bot, knowledge_by_id};

#[test]
fn an_unresolved_washer_note_can_leave_an_earlier_project_as_the_knowledge_subject() {
    let washer = "My washing machine is making a knocking sound.";
    let check = "What's the first thing to check?";
    let paint = knowledge_by_id("home", "which-paint").unwrap();
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("Why do onions make you cry?");
    bot.turn("I want to paint the spare room.");
    assert!(bot.turn(washer).reply.ends_with(" — noted."));
    let turn = bot.turn(check);
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(turn.reply, paint.reply);

    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn(washer);
    assert_ne!(bot.turn(check).reply, paint.reply);
}

#[test]
fn a_backpack_capacity_question_can_use_the_previous_packing_key() {
    let ask = "Would a small backpack be enough?";
    let school = knowledge_by_id("study", "school-bag").unwrap();
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("What should I pack for a weekend away?");
    let turn = bot.turn(ask);
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(turn.reply, school.reply);

    let mut bot = Bot::new(Some("0")).unwrap();
    assert_ne!(bot.turn(ask).reply, school.reply);
}

#[test]
fn leftover_fit_repeats_the_matching_earlier_sentence_only_when_it_was_said() {
    let vegetables = "What can I make with leftover roast vegetables?";
    let ask = "Would they work in a soup?";
    let repeat = "Same answer, and it covers this: The rule that works for most people is one full rest day a week plus an easy day between hard ones: never two hard sessions back to back.";
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("What should I do on rest days?");
    bot.turn(vegetables);
    let turn = bot.turn(ask);
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(turn.reply, repeat);

    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn(vegetables);
    assert_ne!(bot.turn(ask).reply, repeat);

    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("What should I do on rest days?");
    bot.turn(vegetables);
    assert_ne!(bot.turn("Could they work in a soup?").reply, repeat);

    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("What should I do on rest days?");
    bot.turn("Different question.");
    bot.turn(vegetables);
    assert_ne!(bot.turn(ask).reply, repeat);
}
