//! Direct PyBot cases for extracting exactly one figure from an earlier answer.

pub(super) struct Case(
    pub &'static str,
    pub &'static [(&'static str, &'static str)],
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "how long",
        &[(
            "answers",
            "For a simple loaf, the first rise often takes 1 to 2 hours at room temperature before you shape it. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how long is that",
        &[(
            "answers",
            "Some first rises take about two hours depending on the dough. Keep an eye on the size. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("About two hours."),
    ),
    Case(
        "how many weeks",
        &[(
            "answers",
            "Allow eight to ten weeks to sort out the job and then speak to a tradesperson. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("Eight to ten weeks."),
    ),
    Case(
        "how many days",
        &[(
            "answers",
            "Allow eight to ten weeks to sort out the job and then speak to a tradesperson. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how many hours",
        &[(
            "answers",
            "Allow 2 hrs for the journey, including the wait at the station when the other train has left. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how long",
        &[(
            "answers",
            "Put something light over the bowl and wait for a while until it has doubled in size. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how many weeks again",
        &[(
            "answers",
            "Six weeks is a good pace to try something small. Give it six weeks before deciding. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("Six weeks."),
    ),
    Case(
        "how many weeks again",
        &[(
            "answers",
            "Six weeks is a good pace. Give it seven weeks before deciding what to do about it. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how many minutes",
        &[(
            "answers",
            "5 to 6 minute eggs stay a little soft in the middle, so open one to see if you like it. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how long",
        &[(
            "answers",
            "5 to 6 minutes is the length of the boil, and the texture is the thing you can test. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("5 to 6 minutes."),
    ),
    Case(
        "how long",
        &[(
            "answers",
            "6 minutes for a runny yolk, 7 for jammy, 9 to 10 for hard boiled. Put them in cold water. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how many weeks",
        &[(
            "answers",
            "20 minutes at a time with a cloth round it for the first two days, then start over. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how long",
        &[(
            "answers",
            "Keep the sting cool first. It often gets better in about three days when it is left on its own. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("About three days."),
    ),
    Case(
        "how long",
        &[(
            "answers",
            "When it is ready, cool it on a rack for half an hour before you go to bed. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how long",
        &[(
            "answers",
            "I can't see live delays here, and there aren't published figures for this service. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how many days",
        &[(
            "answers",
            "Most things improve in ٢ to ٣ days with a break from long walks and regular meals. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("٢ to ٣ days."),
    ),
    Case(
        "how many weeks not days",
        &[(
            "answers",
            "Allow 2 weeks at the start, with the rehearsal in a separate room every evening. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how long do you reckon",
        &[(
            "answers",
            "About three weeks is enough to get to the place you wanted if nothing changes. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("About three weeks."),
    ),
    Case(
        "how long should i rest it",
        &[(
            "answers",
            "About three weeks is enough to get to the place you wanted if nothing changes. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how much",
        &[(
            "answers",
            "A new panel costs £50 to £120 at a local shop. There are two good alternatives. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("£50 to £120."),
    ),
    Case(
        "how much a month",
        &[(
            "answers",
            "For £4,000: £100 a month is 40 months, £250 a month is 16 months, and you choose the number. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how much",
        &[(
            "answers",
            "Track anything that leaves your account. Pick 10 to 20 per cent of what you take home. Raise by 10 pounds whenever a bill goes. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("10 to 20 per cent."),
    ),
    Case(
        "how much",
        &[(
            "answers",
            "The main repair costs 200 to 300 pounds; wait for somebody to come and inspect it. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("200 to 300 pounds."),
    ),
    Case(
        "how much",
        &[(
            "answers",
            "You could spend £50. If you also want delivery, that adds £10, but it depends on where you are. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("£50."),
    ),
    Case(
        "how much",
        &[(
            "answers",
            "The part costs about £59.50 on average when a shop can do the whole thing. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how much water",
        &[(
            "answers",
            "A reasonable amount is 200 ml after exercise, and then take more as needed. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how many litres",
        &[(
            "answers",
            "A reasonable amount is 200 ml after exercise, and then take more as needed. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how far",
        &[(
            "answers",
            "It is about 8 miles from the station, though you may decide to cycle the whole thing. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how far",
        &[(
            "answers",
            "That is about 8 kilometres along the lane. It takes a while to do it with a pushchair. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how many",
        &[(
            "answers",
            "There were four or five people at the start, and another 6 people came after lunch. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how often",
        &[(
            "answers",
            "Go back and see whether it needs more than 3 years of repair history the next day. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "and how many grams",
        &[(
            "answers",
            "Use about 20 g of coffee so the espresso comes out steadily, with a clear taste. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "and how many grams",
        &[(
            "answers",
            "Use about 20 grams of coffee so the espresso comes out steadily, with a clear taste. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "grams?",
        &[(
            "answers",
            "Use about 20 grams of coffee so the espresso comes out steadily, with a clear taste. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how long?",
        &[(
            "answers",
            "The little jobs take 7 MINUTES but only five hours go into the long job in total. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("Only five hours."),
    ),
    Case(
        "how many weeks",
        &[(
            "answers",
            "It should be 4 weeks of quiet practice, 6 for the second group, and 8 weeks for the concert. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how much a day",
        &[(
            "answers",
            "One to two hours in the evening are enough if this is the first time you use the library. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "and how much longer",
        &[(
            "answers",
            "It takes about 75 minutes to fill the direct train once a day. The trip is coming back in the same evening. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        None,
    ),
    Case(
        "how long",
        &[(
            "answers",
            "The answer on the screen was more than 5-6 days even on the fastest schedule you can arrange. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("5 to 6 days."),
    ),
    Case(
        "how long",
        &[(
            "answers",
            "It usually takes 10 or 11 minutes to finish cooking on its own when the heat stays up. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("10 to 11 minutes."),
    ),
    Case(
        "how many days",
        &[(
            "answers",
            "One day is enough when you only want to see the town and you are not staying longer. Other details might change the plan as the situation develops, so keep the conditions you already learned in view.",
        )],
        Some("One day."),
    ),
    Case(
        "how long",
        &[
            (
                "answers",
                "It takes about three weeks to get back into the swing of things in a lot of places.",
            ),
            ("greeting", "Okay. What else would you like to know?"),
        ],
        None,
    ),
    Case(
        "how long",
        &[
            (
                "answers",
                "Usually a few weeks if you do it slowly, though a move with a lorry takes much longer.",
            ),
            (
                "answers",
                "It takes a while for the colour to show up below the surface of the new paint.",
            ),
        ],
        None,
    ),
    Case(
        "how long",
        &[
            (
                "answers",
                "Usually eight weeks is enough for the next test at the end of the planning phase.",
            ),
            (
                "answers",
                "In the first six days, expect one or two days at a time around work and family.",
            ),
        ],
        None,
    ),
    Case(
        "how long",
        &[
            (
                "answers",
                "It takes about three weeks for the first stage to show results.",
            ),
            (
                "answers",
                "Six weeks for the job. Is a short report what you're looking for?",
            ),
        ],
        None,
    ),
    Case(
        "how long",
        &[
            (
                "chat",
                "It takes about three weeks for the first stage to show results.",
            ),
            ("greeting", "Okay. What else would you like to know?"),
        ],
        None,
    ),
    Case(
        "how much",
        &[
            (
                "answers",
                "This work costs about £400 which is what the shop will put on the quote.",
            ),
            (
                "memory_store",
                "It is going to take some time to hear back from the new manager.",
            ),
        ],
        None,
    ),
    Case(
        "how long",
        &[(
            "answers",
            "First time, it takes 6–8 weeks if you work around the school timetable.",
        )],
        None,
    ),
    Case(
        "how long",
        &[
            ("answers", "Usually it takes 4 hours."),
            ("greeting", "Go on."),
        ],
        None,
    ),
];
