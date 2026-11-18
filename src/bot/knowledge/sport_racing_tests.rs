//! Race and event checks captured from PyBot 7d63c7f.
use super::*;

#[test]
fn race_guards_match_the_reference() {
    for (line, expected) in [
        (
            "How far is a half marathon?",
            [true, false, true, false, true],
        ),
        (
            "How far is a half-marathon?",
            [true, false, true, false, true],
        ),
        (
            "How long is a full marathon?",
            [true, true, false, false, true],
        ),
        (
            "At half 5 my shift starts",
            [false, false, true, false, false],
        ),
        ("I start at half five", [false, false, true, false, false]),
        ("I start at half 12", [false, false, true, false, false]),
        (
            "I start at half thirteen",
            [true, false, true, false, false],
        ),
        (
            "I signed up for the half marathon and start at half five",
            [true, false, true, false, true],
        ),
        (
            "I start at half 5 and I'm running a half-marathon",
            [true, false, true, false, true],
        ),
        (
            "What is a good time for a half marathon?",
            [true, false, true, false, true],
        ),
        ("Can I do a half now?", [true, false, true, false, false]),
        (
            "Is a marathon longer than a half marathon?",
            [true, true, true, false, true],
        ),
        (
            "What's the difference between a marathon and a half?",
            [true, true, true, false, true],
        ),
        (
            "Is a half marathon longer than a 10k?",
            [true, false, false, true, true],
        ),
        (
            "Is a half marathon longer than a 5km?",
            [true, false, false, true, true],
        ),
        (
            "Is the half longer than parkrun?",
            [true, false, false, true, true],
        ),
        (
            "How far is a marathon compared to a 10k?",
            [true, true, false, false, true],
        ),
        (
            "Half, full and 5k, what's longer?",
            [true, true, false, false, true],
        ),
        (
            "Is 13.1 miles farther than a 10km?",
            [true, false, false, true, true],
        ),
        (
            "Which is longer, 21.1 km or a marathon?",
            [true, true, true, false, true],
        ),
        ("How long is 21.1km?", [true, true, false, false, false]),
        (
            "How long is a marathon divided in half?",
            [true, false, true, false, true],
        ),
        (
            "Is a marathon twice as far as a half?",
            [true, true, true, false, true],
        ),
        (
            "Should I rest a leg of lamb?",
            [true, true, false, false, false],
        ),
        (
            "Should I rest between gym sessions?",
            [true, true, false, false, false],
        ),
        (
            "How long should I rest after 5k?",
            [true, true, false, true, true],
        ),
        (
            "How long should I rest after the 10k?",
            [true, true, false, true, true],
        ),
        (
            "How long should I rest after six miles?",
            [true, true, false, false, true],
        ),
        (
            "How long should I rest after an ultra marathon?",
            [true, true, false, false, true],
        ),
        (
            "How long should I rest after the marathon?",
            [true, true, false, false, false],
        ),
        (
            "How long should I rest after a marathon?",
            [true, true, false, false, true],
        ),
        (
            "How long before I can run after a half marathon?",
            [true, false, true, false, true],
        ),
        (
            "What is the halfway point of a marathon?",
            [true, true, false, false, true],
        ),
        (
            "A half marathon or a 5 k?",
            [true, false, false, true, true],
        ),
        (
            "Do I enter a half at half past seven?",
            [true, false, true, false, false],
        ),
        ("Half eleven as usual", [false, false, true, false, false]),
    ] {
        let reading = Reading::new(line);
        let actual = [
            half_race(&reading),
            full_distance(&reading),
            compare_full(&reading),
            compare_shorter(&reading),
            named_distance(&reading),
        ];
        assert_eq!(actual, expected, "{line:?}");
    }
}

type Case = (
    Reply,
    &'static [&'static str],
    &'static str,
    Option<&'static str>,
);

#[test]
fn race_recovery_and_event_replies_match_the_reference() {
    let cases: &[Case] = &[
        (
            Reply::Recovery,
            &[],
            "How long should I rest after a marathon?",
            Some(AFTER_MARATHON),
        ),
        (
            Reply::Recovery,
            &[],
            "How long should I recover after the 10k?",
            Some(AFTER_TEN_K),
        ),
        (
            Reply::Recovery,
            &[],
            "What is recovery after half marathon?",
            Some(AFTER_HALF),
        ),
        (
            Reply::Recovery,
            &[],
            "How long do I rest after parkrun?",
            Some(AFTER_FIVE_K),
        ),
        (
            Reply::Recovery,
            &[],
            "How long do I rest after three miles?",
            Some(AFTER_FIVE_K),
        ),
        (
            Reply::Recovery,
            &[],
            "How much rest after an ultra marathon?",
            Some(AFTER_MARATHON),
        ),
        (
            Reply::Recovery,
            &[],
            "How long to rest after 42.2?",
            Some(AFTER_MARATHON),
        ),
        (
            Reply::Recovery,
            &[],
            "How long to rest after 21k?",
            Some(AFTER_HALF),
        ),
        (
            Reply::Recovery,
            &[],
            "How long to rest after 6.2?",
            Some(AFTER_TEN_K),
        ),
        (
            Reply::Recovery,
            &[],
            "How long to rest after a run?",
            Some(RECOVERY_GUIDE),
        ),
        (
            Reply::Recovery,
            &[],
            "Recovery after a 10k and a half marathon?",
            Some(AFTER_HALF),
        ),
        (
            Reply::Recovery,
            &[],
            "Recovery from a 5k or 10k?",
            Some(AFTER_TEN_K),
        ),
        (
            Reply::Recovery,
            &[],
            "Recover from a marathon and a half marathon?",
            Some(AFTER_MARATHON),
        ),
        (
            Reply::Recovery,
            &["I'm doing a half marathon."],
            "How long do I recover?",
            Some(AFTER_HALF),
        ),
        (
            Reply::Recovery,
            &["I entered a 5k.", "After that I ran a 10k."],
            "How long should I rest?",
            Some(AFTER_TEN_K),
        ),
        (
            Reply::Recovery,
            &["I entered a 5k.", "After that I ran a 10k."],
            "How long should I rest after a half marathon?",
            Some(AFTER_HALF),
        ),
        (
            Reply::Recovery,
            &["I ran a marathon.", "A", "B", "C", "D", "E"],
            "How long to recover?",
            Some(AFTER_MARATHON),
        ),
        (
            Reply::Recovery,
            &["I ran a marathon.", "A", "B", "C", "D", "E", "F"],
            "How long to recover?",
            Some(RECOVERY_GUIDE),
        ),
        (
            Reply::Recovery,
            &["I walked a few kilometres.", "No races yet."],
            "How long to recover?",
            Some(RECOVERY_GUIDE),
        ),
        (
            Reply::Recovery,
            &["I did a 5k.", "I did a half marathon and a 10k."],
            "How long to recover?",
            Some(AFTER_HALF),
        ),
        (
            Reply::Start,
            &[],
            "What time do most marathons start?",
            Some(RACE_START),
        ),
        (
            Reply::Start,
            &[],
            "What time does a half marathon start?",
            Some(RACE_START),
        ),
        (
            Reply::Start,
            &[],
            "What time does a 10k start?",
            Some(RACE_START),
        ),
        (
            Reply::Start,
            &[],
            "When does the triathlon start?",
            Some(RACE_START),
        ),
        (
            Reply::Start,
            &[],
            "When does parkrun start?",
            Some(RACE_START),
        ),
        (
            Reply::Start,
            &[],
            "What time does the football match start?",
            Some(MATCH_START),
        ),
        (Reply::Start, &[], "When is kick-off?", Some(MATCH_START)),
        (
            Reply::Start,
            &[],
            "What time do doors open for the concert?",
            Some(GIG_START),
        ),
        (
            Reply::Start,
            &[],
            "When does the gig start?",
            Some(GIG_START),
        ),
        (
            Reply::Start,
            &[],
            "What time do the fireworks start?",
            Some(FIREWORKS_START),
        ),
        (
            Reply::Start,
            &[],
            "What time does the display start?",
            Some(FIREWORKS_START),
        ),
        (
            Reply::Start,
            &[],
            "When is the bonfire?",
            Some(FIREWORKS_START),
        ),
        (
            Reply::Start,
            &[],
            "What time does a marathon match start?",
            Some(MIXED_SCHEDULE),
        ),
        (
            Reply::Start,
            &[],
            "What time do doors open for a concert with fireworks?",
            Some(MIXED_SCHEDULE),
        ),
        (
            Reply::Start,
            &[],
            "When does a football match and a football game start?",
            Some(MATCH_START),
        ),
        (Reply::Start, &[], "What time does the shop open?", None),
        (Reply::Start, &[], "What time does the show start?", None),
        (Reply::Start, &[], "When should I start?", None),
        (
            Reply::Start,
            &[],
            "When does the race begin at the football festival?",
            Some(MIXED_SCHEDULE),
        ),
        (
            Reply::Start,
            &[],
            "What time does the match start on a race day?",
            Some(MIXED_SCHEDULE),
        ),
        (
            Reply::Start,
            &[],
            "What time can I start to trace it?",
            Some(RACE_START),
        ),
    ];
    for &(form, before, line, expected) in cases {
        let prior: Vec<_> = before
            .iter()
            .map(|user| Exchange {
                user: (*user).into(),
                bot: "All right.".into(),
                module: "greeting",
            })
            .collect();
        let reading = Reading::new(line);
        assert_eq!(
            reply(form, &reading, &prior).as_deref(),
            expected,
            "{form:?} for {line:?} after {before:?}"
        );
    }
}

const AFTER_MARATHON: &str = "Three to four weeks before anything hard: easy running or walking the first week, short easy runs the second, and a tempo or a long run only around a month out. The fitness comes back faster than the legs do, which is what catches people who feel fine on day five.";

const AFTER_TEN_K: &str = "Three or four easy days and then normal training: a 10k at effort takes more out of the legs than the distance suggests, and the second day after is usually the sorest.";

const AFTER_HALF: &str = "A week to ten days: three or four short easy runs in the first week, nothing hard for ten days, and a normal week after a fortnight. Most people feel fine after three days, and going hard then is how the second week goes wrong.";

const AFTER_FIVE_K: &str = "A day or two of easy running: a hard 5k costs your legs about 48 hours, and an easy run the day after does more good than a full day off.";

const RECOVERY_GUIDE: &str = "Scale it to what you did: a day or two easy after a hard 5k, three or four days after a 10k, a week to ten days after a half marathon, and three to four weeks before anything hard after a marathon. A gym session is about 48 hours for the same muscle group. Soreness shifts faster with a short easy effort than with total rest, and if you are still stiff on the stairs after a week, you went back too soon.";

const RACE_START: &str = "Mass races start early and in waves: most city marathons and half marathons go off between 8 and 10 in the morning, a 10k usually between 9 and 10, and a parkrun at 9 on a Saturday. Your race email has your own wave and the time the pens close, which is the one people miss - and the earlier wave is often the one with the friendlier cut-offs.";

const MATCH_START: &str = "Most Saturday football kicks off at 3pm and Sunday games at 2pm or 4.30pm, but a televised game moves by hours, so the club's or the ticket page's fixture list is the only reliable one. Turn up an hour before for a big ground, because the queues at the turnstiles are what make people late.";

const GIG_START: &str = "Doors for a gig usually open an hour before the support act, and the headline act comes on about two hours after doors; most venues have a curfew at half past ten or eleven, so that is roughly when it ends. A big venue also has a bag check, so add twenty minutes. The venue's own page or your ticket email gives the door time, and check the last train home before the encore.";

const FIREWORKS_START: &str = "A firework display goes up after dark, so most are between 7 and 9 in the evening in November and later in midsummer, and the organiser's page gives the exact time because it depends on sunset. Wrap up warm and stand upwind, and get there early if you are driving, since car parks empty slowly.";

const MIXED_SCHEDULE: &str = "I can't see the schedule from here, so the organiser's page or your confirmation email is the one to trust. Ask about the gate or start pen closing time as well, because that is usually earlier than the start, and plan to be there half an hour before it.";
