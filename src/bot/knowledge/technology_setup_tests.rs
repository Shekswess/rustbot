//! Setup and eligibility checks captured from PyBot 7d63c7f.
use super::*;
use crate::{Bot, Turn, lookup_knowledge};

#[test]
fn technology_setup_matches_the_reference() {
    for (line, expected) in [
        ("Where should I put my router?", PLACE),
        ("Should I move the router?", MOVE),
        ("Is moving the router worth it?", MOVE),
        ("Where should I move the router?", PLACE),
        ("Moving my internet hub upstairs", MOVE),
        ("Should I put my broadband box beside the TV?", PLACE),
        ("Router moves", PLACE),
        ("The router moved into the cupboard", PLACE),
        ("WHERE should I put the router if I'm MOVING?", PLACE),
    ] {
        assert_eq!(
            router_place(&Reading::new(line)).as_deref(),
            Some(expected),
            "{line:?}"
        );
    }

    for (line, expected) in [
        ("My app keeps crashing", true),
        ("My banking app is stuck", true),
        ("I'm stuck with my job application", false),
        ("I'm stuck on an application form", false),
        ("I'm stuck on my visa application", false),
        ("My travel applications are being processed", false),
        ("I'm completing my driving licence application", false),
        ("The application form won't open on my phone", true),
        ("The school application won't load on the website", true),
        ("I can't download the application form", true),
        ("My laptop won't open the job application", true),
        ("I need help with my visa application deadline", false),
        ("My application is frozen", true),
        ("Help with my grant application on an iPad", true),
        ("My job application is for a phone shop", true),
    ] {
        assert_eq!(app_not_paper(&Reading::new(line)), expected, "{line:?}");
    }

    for (line, expected) in [
        ("My laptop keeps falling asleep", true),
        ("I keep falling asleep at my laptop", false),
        ("My computer keeps falling asleep on the desk", true),
        ("I'm falling asleep at my laptop", false),
        ("My phone keeps falling asleep when I look away", true),
        ("My son falls asleep on his Xbox", false),
        ("I keep nodding off at my desk", false),
        ("My eyes keep closing at my screen", false),
        ("My eyes are closing with hay fever", true),
        ("My screen strains my eyes", true),
        ("I can barely keep my eyes open", false),
        ("I keep my eyes open for a bargain", true),
        ("I can barely keep up with the rent", true),
        ("I can hardly stay alert at my computer", false),
        ("My tablet goes to sleep after two minutes", true),
        ("I fell asleep on the train", false),
        ("I keep falling over when I stand", true),
        (
            "My laptop keeps falling asleep and I am nodding off too",
            false,
        ),
        ("I fell asleep and now my laptop keeps falling asleep", true),
        ("Eyes shutting with pollen", true),
        ("We kept dozing at my computer", false),
        ("My laptop: keeps falling asleep", true),
        ("My laptop keyboard wakes", true),
        ("I'm dropping things at my laptop", true),
    ] {
        assert_eq!(not_a_sleepy_body(&Reading::new(line)), expected, "{line:?}");
    }
}

const PLACE: &str = "Up high, out in the open and as close to the middle of the flat as the cable allows - on a shelf or the top of a bookcase, not on the floor, in a cupboard, behind the TV or beside the microwave, a cordless phone or the hot water tank. Radio waves spread out sideways and downwards from the box, so one in a corner mostly covers the corner. The line comes in where it comes in: if that is one end, keep the router there and add a mesh unit at the other end rather than running a long cable.";

const MOVE: &str = "Yes, if it is in a cupboard, behind the TV, on the floor or at the far end of the flat, and it is worth the five minutes. Up high, out in the open and as close to the middle of the flat as the cable allows - on a shelf or the top of a bookcase, not on the floor, in a cupboard, behind the TV or beside the microwave, a cordless phone or the hot water tank. Radio waves spread out sideways and downwards from the box, so one in a corner mostly covers the corner. The line comes in where it comes in: if that is one end, keep the router there and add a mesh unit at the other end rather than running a long cable.";

fn proposal(turn: &Turn) -> Option<&str> {
    turn.candidates
        .iter()
        .find(|candidate| candidate.module == "answers")
        .map(|candidate| candidate.reply.as_str())
}

#[test]
fn wireless_equipment_uses_the_held_subject() {
    let booster = lookup_knowledge("Would a signal booster help?").unwrap();
    assert_eq!(booster.id, "wifi-booster");
    assert_eq!(
        lookup_knowledge("Would a wifi booster help?").unwrap().id,
        "router-buy"
    );
    let is_booster = |turn: &Turn| proposal(turn) == Some(booster.reply.as_ref());
    let mut bot = Bot::new(Some("0")).unwrap();
    assert!(!is_booster(&bot.turn("Would a booster help?")));
    bot.turn("My wifi is weak upstairs.");
    bot.turn("Thanks");
    assert!(is_booster(&bot.turn("Would a booster help?")));
    assert!(!is_booster(&bot.turn("Would a booster jab help?")));
    assert!(!is_booster(
        &bot.turn("New question, would a booster help?")
    ));

    bot.reset().unwrap();
    bot.turn("My router is downstairs.");
    bot.turn("What is the capital of Spain?");
    assert!(!is_booster(&bot.turn("Would a booster help?")));
}

#[test]
fn router_admin_password_distinguishes_reports_and_questions() {
    for (prompt, expected) in [
        ("How do I reset my router password?", WIFI_RESET),
        ("What do I do if I am locked out of my router?", WIFI_RESET),
        ("I can’t log in to my router", WIFI_LOCKED_SHORT),
        ("My router password won't work", WIFI_LOCKED_SHORT),
    ] {
        let card = lookup_knowledge(prompt).unwrap();
        assert_eq!(card.id, "router-admin-password", "{prompt:?}");
        assert_eq!(card.reply, expected, "{prompt:?}");
        let turn = Bot::new(Some("0")).unwrap().turn(prompt);
        assert_eq!(proposal(&turn), Some(expected), "{prompt:?}");
    }
    for prompt in [
        "Which router should I buy?",
        "My hub stopped working and I'm locked out, which router should I buy?",
    ] {
        assert_eq!(lookup_knowledge(prompt).unwrap().id, "router-buy");
    }
}

#[test]
fn a_bare_filter_stands_down_in_other_appliance_threads() {
    let mut bot = Bot::new(Some("0")).unwrap();
    let prompt = "How do I clean the filter?";
    assert_eq!(proposal(&bot.turn(prompt)), Some(VACUUM_FILTER));
    bot.reset().unwrap();
    bot.turn("My dishwasher smells");
    bot.turn("It is five years old");
    bot.turn("It has happened several times");
    bot.turn("Thanks");
    assert_ne!(proposal(&bot.turn(prompt)), Some(VACUUM_FILTER));
    assert_eq!(
        proposal(&bot.turn("How do I clean my Miele filter?")),
        Some(VACUUM_FILTER)
    );
    bot.reset().unwrap();
    bot.turn("What is the capital of Spain?");
    assert_eq!(proposal(&bot.turn(prompt)), Some(VACUUM_FILTER));
}
