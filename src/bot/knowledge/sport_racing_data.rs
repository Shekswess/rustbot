//! Race and event data from PyBot 7d63c7f, bot/knowledge/sport.py.

pub(super) const CLOCK_HALF: &[&str] = &[
    "half one",
    "half two",
    "half three",
    "half four",
    "half five",
    "half six",
    "half seven",
    "half eight",
    "half nine",
    "half ten",
    "half eleven",
    "half twelve",
    "half 1",
    "half 2",
    "half 3",
    "half 4",
    "half 5",
    "half 6",
    "half 7",
    "half 8",
    "half 9",
    "half 10",
    "half 11",
    "half 12",
];

pub(super) const COMPARED: &[&str] = &[
    "longer",
    "longer than",
    "longest",
    "further",
    "furthest",
    "which is longer",
    "how much longer",
    "difference",
    "compared",
    "compare",
    "versus",
    "vs",
    "twice",
];

pub(super) const RECOVERY_LOOKBACK: usize = 6;

pub(super) const RECOVERY_BY_DISTANCE: &[(&[&str], &str)] = &[
    (
        &["a marathon", "full marathon", "26.2", "42.2", "42k"],
        "Three to four weeks before anything hard: easy running or walking the first week, short easy runs the second, and a tempo or a long run only around a month out. The fitness comes back faster than the legs do, which is what catches people who feel fine on day five.",
    ),
    (
        &[
            "a half marathon",
            "half marathon",
            "half-marathon",
            "13.1",
            "21k",
        ],
        "A week to ten days: three or four short easy runs in the first week, nothing hard for ten days, and a normal week after a fortnight. Most people feel fine after three days, and going hard then is how the second week goes wrong.",
    ),
    (
        &["10k", "10 k", "ten k", "6.2", "six miles", "six mile"],
        "Three or four easy days and then normal training: a 10k at effort takes more out of the legs than the distance suggests, and the second day after is usually the sorest.",
    ),
    (
        &["5k", "5 k", "five k", "three miles", "parkrun"],
        "A day or two of easy running: a hard 5k costs your legs about 48 hours, and an easy run the day after does more good than a full day off.",
    ),
];

pub(super) const EVENT_START_GENERIC: &str = "I can't see the schedule from here, so the organiser's page or your confirmation email is the one to trust. Ask about the gate or start pen closing time as well, because that is usually earlier than the start, and plan to be there half an hour before it.";

pub(super) const A_SHORTER_RACE: &str = r#"(?i)\b(?:5\s?k|5km|five k|10\s?k|10km|ten k|parkrun)\b"#;

pub(super) const THE_FULL_RACE: &str = r#"(?i)\b(?:full|marathon)\b(?<!half marathon)"#;

pub(super) const RECOVERY_GENERAL: &str = "Scale it to what you did: a day or two easy after a hard 5k, three or four days after a 10k, a week to ten days after a half marathon, and three to four weeks before anything hard after a marathon. A gym session is about 48 hours for the same muscle group. Soreness shifts faster with a short easy effort than with total rest, and if you are still stiff on the stairs after a week, you went back too soon.";

pub(super) const SCHEDULES: &[(&[&str], &str)] = &[
    (
        &[
            "marathon",
            "marathons",
            "half marathon",
            "10k",
            "5k",
            "parkrun",
            "race",
            "races",
            "ultra",
            "triathlon",
            "cross country",
            "fun run",
        ],
        "Mass races start early and in waves: most city marathons and half marathons go off between 8 and 10 in the morning, a 10k usually between 9 and 10, and a parkrun at 9 on a Saturday. Your race email has your own wave and the time the pens close, which is the one people miss - and the earlier wave is often the one with the friendlier cut-offs.",
    ),
    (
        &[
            "match", "matches", "kick off", "kick-off", "kickoff", "game", "football", "rugby",
            "cricket", "netball", "hockey", "fixture", "derby",
        ],
        "Most Saturday football kicks off at 3pm and Sunday games at 2pm or 4.30pm, but a televised game moves by hours, so the club's or the ticket page's fixture list is the only reliable one. Turn up an hour before for a big ground, because the queues at the turnstiles are what make people late.",
    ),
    (
        &[
            "concert",
            "concerts",
            "gig",
            "gigs",
            "festival",
            "glastonbury",
            "gig ",
        ],
        "Doors for a gig usually open an hour before the support act, and the headline act comes on about two hours after doors; most venues have a curfew at half past ten or eleven, so that is roughly when it ends. A big venue also has a bag check, so add twenty minutes. The venue's own page or your ticket email gives the door time, and check the last train home before the encore.",
    ),
    (
        &["fireworks", "firework", "bonfire", "display"],
        "A firework display goes up after dark, so most are between 7 and 9 in the evening in November and later in midsummer, and the organiser's page gives the exact time because it depends on sunset. Wrap up warm and stand upwind, and get there early if you are driving, since car parks empty slowly.",
    ),
];
