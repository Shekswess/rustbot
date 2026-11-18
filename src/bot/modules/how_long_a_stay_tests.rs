//! Direct guest-stay replies from the pinned PyBot 7d63c7f source.

use super::*;
use crate::bot::Exchange;

const TURNS: &[(&str, &str, &str)] = &[
    (
        "My mum is coming to stay next week.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My mum is coming to stay for four days.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My brother is visiting for 3 nights.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My dad is coming over for the weekend.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My sister is staying here for about a couple of weeks.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My friend is stopping with us for a fortnight.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "He is staying with us for only one night.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My sister will be here for a week.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My guest is visiting for the weekend.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My mum is coming to stay for three nights.",
        "Three nights is enough. Check the bed.",
        "answers",
    ),
    (
        "My mum is coming to stay for four days.",
        "We can decide tomorrow. Four days is my recommendation.",
        "answers",
    ),
    (
        "My daughter is coming to stay next week.",
        "A weekend. Leave an evening for you.",
        "answers",
    ),
    (
        "My daughter is coming to stay next week.",
        "Choose one shared outing a day. Keep a week free.",
        "answers",
    ),
    (
        "My daughter is coming to stay next week.",
        "We spoke earlier about what to cook. Weekends can be busy.",
        "answers",
    ),
    (
        "My mum is coming to stay next week.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "chat",
    ),
    (
        "I am going to Rome for a week.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "I am visiting Paris in July.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "I want to repaint the hall.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "I will stay home this weekend.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My brother is coming over, with a few things from work.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My friend came to stay for ten days.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    (
        "My daughter is arriving to stay for just a few nights.",
        "Give them the best bed and plan one shared outing a day, so the flat is mainly for sleeping and everyone gets a reason to leave it. Agree meal times on the first evening and keep one evening for yourself; a week goes better when nobody is guessing.",
        "answers",
    ),
    ("I am at home.", "", "chat"),
    ("I am at home.", "Wait until later.", "greeting"),
];

const REPLIES: &[&str] = &[
    "You have not said how long they are staying, and that is what decides it: a weekend is one outing, a week is three or four. How long is it?",
    "Three or four days is the sweet spot for a first stay: long enough to settle in, short enough that nobody runs out of things to say.",
    "Four days is a comfortable stay: one shared outing a day is enough for that long, and keep one evening back for yourself.",
    "3 nights gives you room for an outing a day. The rest of the card is what matters more: the bed and the meal times.",
    "For for the weekend, plan one outing a day rather than a schedule - and leave one evening free.",
    "A couple of weeks is a comfortable stay: one shared outing a day is enough for that long, and keep one evening back for yourself.",
    "For a fortnight, plan one outing a day rather than a schedule - and leave one evening free.",
    "For one night, plan one outing a day rather than a schedule - and leave one evening free.",
    "How long are they with you for? Tell me and I will say how much to plan - the bed and the meal times are the same either way.",
    "Up to a week is the usual answer - long enough to settle in, and the length where an outing a day and one evening kept back still feel like a visit.",
    "Ten days is a comfortable stay: one shared outing a day is enough for that long, and keep one evening back for yourself.",
    "A few nights is a comfortable stay: one shared outing a day is enough for that long, and keep one evening back for yourself.",
    "A fortnight gives you room for an outing a day. The rest of the card is what matters more: the bed and the meal times.",
    "How long is the stay? A weekend wants one thing planned and a week wants two or three, so the answer changes with it.",
    "A weekend to a week. Past a week, plan one evening apart or the visit starts to feel like a duty for both of you.",
];

type Case = (&'static str, &'static [usize], Option<usize>);
const CASES: &[Case] = &[
    ("How long will it take?", &[0], Some(0)),
    ("How long should she stay?", &[0], Some(1)),
    ("How long will it take?", &[0], Some(0)),
    ("How long will it take?", &[1], Some(2)),
    ("How long should she stay?", &[1], Some(2)),
    ("How long should she stay?", &[1], Some(2)),
    ("How long will it take?", &[2], Some(3)),
    ("How long should she stay?", &[2], Some(3)),
    ("How many days should she stay?", &[2], None),
    ("How long will it take?", &[3], Some(4)),
    ("How long should she stay?", &[3], Some(4)),
    ("How many nights is that?", &[3], None),
    ("How long will it take?", &[4], Some(5)),
    ("How long should she stay?", &[4], Some(5)),
    ("How many weeks?", &[4], None),
    ("How long will it take?", &[5], Some(6)),
    ("How long should she stay?", &[5], Some(6)),
    ("How much time is it?", &[5], None),
    ("How long will it take?", &[6], Some(7)),
    ("How long should she stay?", &[6], Some(7)),
    ("How long does the drive take?", &[6], None),
    ("How long will it take?", &[7], None),
    ("How long should she stay?", &[7], None),
    ("How long before the crumble is ready?", &[7], None),
    ("How long will it take?", &[8], Some(4)),
    ("How long should she stay?", &[8], Some(4)),
    ("How long will 10 do?", &[8], Some(4)),
    ("How long will it take?", &[9], None),
    ("How long should she stay?", &[9], None),
    ("Are you certain?", &[9], None),
    ("How long will it take?", &[10], Some(2)),
    ("How long should she stay?", &[10], Some(2)),
    ("How long is too long?", &[10], Some(2)),
    ("How long will it take?", &[11], None),
    ("How long should she stay?", &[11], None),
    ("How long do you think is right?", &[11], None),
    ("How long will it take?", &[12], Some(0)),
    ("How long should she stay?", &[12], Some(1)),
    ("How long has that taken?", &[12], None),
    ("How long will it take?", &[13], Some(0)),
    ("How long should she stay?", &[13], Some(1)),
    ("How long will it take before Monday?", &[13], None),
    ("How long will it take?", &[14], None),
    ("How long should she stay?", &[14], None),
    ("How long will it take?", &[14], None),
    ("How long will it take?", &[15], None),
    ("How long should she stay?", &[15], None),
    ("How long should she stay?", &[15], None),
    ("How long will it take?", &[16], None),
    ("How long should she stay?", &[16], None),
    ("How many days should she stay?", &[16], None),
    ("How long will it take?", &[17], None),
    ("How long should she stay?", &[17], None),
    ("How many nights is that?", &[17], None),
    ("How long will it take?", &[18], None),
    ("How long should she stay?", &[18], None),
    ("How many weeks?", &[18], None),
    ("How long will it take?", &[19], Some(8)),
    ("How long should she stay?", &[19], Some(9)),
    ("How much time is it?", &[19], None),
    ("How long will it take?", &[20], Some(10)),
    ("How long should she stay?", &[20], Some(10)),
    ("How long does the drive take?", &[20], None),
    ("How long will it take?", &[21], Some(11)),
    ("How long should she stay?", &[21], Some(11)),
    ("How long before the crumble is ready?", &[21], None),
    ("How long will it take?", &[], None),
    ("How long will it take?", &[1], Some(2)),
    ("How long should she stay?", &[], None),
    ("How long should she stay?", &[1], Some(2)),
    ("How many days should she stay?", &[], None),
    ("How many days should she stay?", &[1], None),
    ("How many nights is that?", &[], None),
    ("How many nights is that?", &[1], None),
    ("How many weeks?", &[], None),
    ("How many weeks?", &[1], None),
    ("How much time is it?", &[], None),
    ("How much time is it?", &[1], None),
    ("How long does the drive take?", &[], None),
    ("How long does the drive take?", &[1], None),
    ("How long before the crumble is ready?", &[], None),
    ("How long before the crumble is ready?", &[1], None),
    ("How long will 10 do?", &[], None),
    ("How long will 10 do?", &[1], Some(2)),
    ("Are you certain?", &[], None),
    ("Are you certain?", &[1], None),
    ("How long is too long?", &[], None),
    ("How long is too long?", &[1], Some(2)),
    ("How long do you think is right?", &[], None),
    ("How long do you think is right?", &[1], None),
    ("How long has that taken?", &[], None),
    ("How long has that taken?", &[1], None),
    ("How long will it take before Monday?", &[], None),
    ("How long will it take before Monday?", &[1], None),
    ("How long will it take?", &[1, 7], Some(2)),
    ("How long should she stay?", &[1, 7], Some(2)),
    ("How long will it take?", &[1, 22], Some(2)),
    ("How long should she stay?", &[1, 22], Some(2)),
    ("How long will it take?", &[1, 23], None),
    ("How long should she stay?", &[1, 23], None),
    ("How long will it take?", &[0, 5], Some(12)),
    ("How long should she stay?", &[0, 5], Some(12)),
    ("How long will it take?", &[5, 0], Some(13)),
    ("How long should she stay?", &[5, 0], Some(14)),
    ("How long will it take?", &[1, 14], None),
    ("How long should she stay?", &[1, 14], None),
    ("How long will it take?", &[0, 15], Some(13)),
    ("How long should she stay?", &[0, 15], Some(14)),
];

#[test]
fn guest_stay_reads_only_a_previous_arrival_and_an_unanswered_length() {
    for &(line, prior, reply) in CASES {
        let reading = Reading::new(line);
        let memory = Memory {
            turns: prior
                .iter()
                .map(|&index| {
                    let (user, bot, module) = TURNS[index];
                    Exchange {
                        user: user.to_owned(),
                        bot: bot.to_owned(),
                        module,
                    }
                })
                .collect(),
            ..Memory::default()
        };
        assert_eq!(
            respond(&reading, &memory).as_deref(),
            reply.map(|index| REPLIES[index]),
            "{line} {prior:?}"
        );
    }
}
