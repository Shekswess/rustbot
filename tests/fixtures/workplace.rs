//! Seeded turns from pinned PyBot, spanning recorded work and duration follow-ups.

pub(super) struct Turn(
    pub &'static str,
    pub Option<&'static str>,
    pub Option<&'static str>,
);
pub(super) const SESSIONS: &[&[Turn]] = &[
    &[
        Turn("im a teacher", None, None),
        Turn(
            "i work in a primary school",
            Some("A teacher in a primary school - how long have you been there?"),
            None,
        ),
        Turn(
            "i have been doing it for 10 years",
            None,
            Some("Ten years as a teacher is more than most people stick at anything."),
        ),
        Turn(
            "i have been teaching for 18 months",
            None,
            Some("Eighteen months as a teacher. The first year is the one that tests everyone."),
        ),
    ],
    &[
        Turn("im a teacher", None, None),
        Turn(
            "i work from home",
            Some(
                "A teacher working from home - how do you keep the day from running into the evening?",
            ),
            None,
        ),
        Turn(
            "ive been doing it for a year already",
            None,
            Some("A year as a teacher - the first one is the hardest."),
        ),
        Turn(
            "i've been doing it for 25 months",
            None,
            Some("Two years as a teacher - that's a long stretch."),
        ),
    ],
    &[
        Turn("i work at Tesco", None, None),
        Turn(
            "i have been doing it for 5 years",
            None,
            Some("Five years as a tesco. You must know it inside out by now."),
        ),
        Turn(
            "I work at an office",
            Some("A tesco at an office - what's the best part of the week there?"),
            None,
        ),
    ],
    &[
        Turn("i work as a nurse", None, None),
        Turn("i work nights", None, None),
        Turn("i am working at a hospital", None, None),
        Turn(
            "i've been nursing for 30 years and counting",
            None,
            Some("Thirty years as a nurse is more than most people stick at anything."),
        ),
    ],
    &[
        Turn("i work as a nurse", None, None),
        Turn(
            "i work from home and i keep getting interrupted",
            Some("A nurse working from home - quiet, or people at the door all day?"),
            None,
        ),
        Turn("i work at the hospital, 12 hour shifts", None, None),
        Turn(
            "i work in a nice big local private care home",
            Some("A nurse in a nice big local private - how long have you been there?"),
            None,
        ),
    ],
    &[
        Turn("im a teaching assistant", None, None),
        Turn(
            "i work in the School Office",
            Some("A teaching assistant in the School Office - what do you do there?"),
            None,
        ),
        Turn(
            "i work at the bank and other people are there",
            Some("A teaching assistant at the bank - does it leave you much time for yourself?"),
            None,
        ),
        Turn(
            "I now work in Leeds",
            Some("A teaching assistant in Leeds - what do you do there?"),
            None,
        ),
    ],
    &[
        Turn("I got a job as a chef", None, None),
        Turn(
            "i work in a restaurant",
            Some("A chef in a restaurant - busy one, I should think."),
            None,
        ),
        Turn(
            "i have been doing it for a few years",
            None,
            Some("Three years as a chef is more than most people stick at anything."),
        ),
        Turn(
            "i've been doing it for a couple of months",
            None,
            Some(
                "Two months as a chef. That is still the part where everyone is finding their feet.",
            ),
        ),
    ],
    &[
        Turn("im a nurse", None, None),
        Turn(
            "i have been doing it for a year",
            None,
            Some("A year as a nurse - the first one is the hardest."),
        ),
        Turn(
            "ive been doing it for 1 month",
            None,
            Some("One months as a nurse - the first year is the hardest stretch of it."),
        ),
        Turn("i have been doing it for 0 years", None, None),
        Turn(
            "i have been doing it for 99 years",
            None,
            Some("Ninety-nine years as a nurse is more than most people stick at anything."),
        ),
        Turn(
            "i have been doing it for ١٢ months",
            None,
            Some("Twelve months as a nurse - past the first year, then."),
        ),
    ],
    &[
        Turn("im a nurse", None, None),
        Turn("ive been doing it for 10 weeks", None, None),
        Turn("i've been doing it for 10 minutes", None, None),
        Turn("i've been doing it for 5 years and i hate it", None, None),
        Turn("i've been waiting for 10 years", None, None),
    ],
    &[
        Turn("I used to be a plumber before 2018", None, None),
        Turn(
            "i work at the hospital",
            Some("A plumber at the hospital - what do you do there?"),
            None,
        ),
        Turn(
            "i've been doing it 3 years",
            None,
            Some("Three years as a plumber - what keeps you at it?"),
        ),
        Turn("i was a driver", None, None),
        Turn(
            "i work from home",
            Some("A driver working from home - what's the best part of it?"),
            None,
        ),
    ],
    &[
        Turn("i want a job as a teacher", None, None),
        Turn("i work in a school", None, None),
        Turn(
            "i've been doing it 5 years",
            None,
            Some("Five years as a school. You must know it inside out by now."),
        ),
    ],
    &[
        Turn("I'm a NURSE", None, None),
        Turn(
            "i work at the hospital",
            Some("A nurse at the hospital - what do you do there?"),
            None,
        ),
        Turn("im a nurse now", None, None),
        Turn(
            "i work in the local hospital",
            Some("A nurse in the local hospital - does it leave you much time for yourself?"),
            None,
        ),
    ],
    &[
        Turn("my sister is a nurse", None, None),
        Turn(
            "she has been nursing for several years",
            None,
            Some("Four years is a long time for your sister to stay in one job."),
        ),
        Turn(
            "she has been doing it for 4 months",
            None,
            Some("Your sister has been a nurse for four months - how is it going?"),
        ),
        Turn("he has been doing it for 20 years", None, None),
    ],
    &[
        Turn("my mum works as a teacher", None, None),
        Turn("my father is a driver", None, None),
        Turn(
            "she has been teaching for 6 months",
            None,
            Some("Your mum has been a teacher for six months - early days, then."),
        ),
        Turn(
            "he has been driving for 24 months",
            None,
            Some(
                "Your father has been a driver for two years. That is a lot of years behind them.",
            ),
        ),
    ],
    &[
        Turn("my sister emma is a nurse", None, None),
        Turn(
            "she has been doing it for 7 years",
            None,
            Some(
                "Your sister has been a nurse for seven years. That is a lot of years behind them.",
            ),
        ),
        Turn(
            "they have been doing it for 18 months",
            None,
            Some("Your sister has been a nurse for eighteen months - that's a long stretch."),
        ),
    ],
    &[
        Turn("my son is a teacher", None, None),
        Turn(
            "he has been teaching for years",
            None,
            Some("Your son has been at it a while, then. That is not nothing."),
        ),
        Turn(
            "he has been doing it for a couple of years",
            None,
            Some("Your son has been a teacher for two years - that's a long stretch."),
        ),
        Turn("he has been sleeping for years", None, None),
    ],
    &[
        Turn("my sister is a nurse", None, None),
        Turn("she is a teacher", None, None),
        Turn("she has been teaching for one year", None, None),
        Turn(
            "she has been nursing for several years",
            None,
            Some(
                "Your sister has been a nurse for four years. That is a lot of years behind them.",
            ),
        ),
    ],
    &[
        Turn("my gf is a teacher", None, None),
        Turn(
            "she has been doing it for years",
            None,
            Some("Your girlfriend has put the years in - no wonder it comes up."),
        ),
        Turn(
            "she has been teaching for 5 years",
            None,
            Some("Five years is a long time for your girlfriend to stay in one job."),
        ),
    ],
    &[
        Turn("i work as an electrician", None, None),
        Turn("i have been in that for ten years", None, None),
        Turn(
            "i've been doing it for 20 yrs",
            None,
            Some("Twenty years as an electrician is more than most people stick at anything."),
        ),
        Turn(
            "been doing it for 10 years",
            None,
            Some("Ten years as an electrician - that's a long stretch."),
        ),
        Turn(
            "   i have been doing it 15 years   ",
            None,
            Some("Fifteen years as an electrician is more than most people stick at anything."),
        ),
    ],
];
