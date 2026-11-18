//! Conditioned measure readings from PyBot 7d63c7f.
use super::*;

struct Case {
    name: &'static str,
    prior: &'static [(&'static str, &'static str, &'static str)],
    ask: &'static str,
    alone: Option<&'static str>,
    expected: Option<&'static str>,
}
const CASES: &[Case] = &[
    Case {
        name: "cake",
        ask: "how long will it take",
        alone: Some(
            "A 20cm sponge bakes at 180C (160C fan) for 25 to 30 minutes; a loaf tin needs 45 to 55. It's done when a skewer in the middle comes out clean, and it's worth checking 5 minutes early the first time. About an hour and a half start to finish, counting mixing and cooling.",
        ),
        expected: Some(
            "A 20cm sponge bakes at 180C (160C fan) for 25 to 30 minutes; a loaf tin needs 45 to 55. It's done when a skewer in the middle comes out clean, and it's worth checking 5 minutes early the first time. About an hour and a half start to finish, counting mixing and cooling.",
        ),
        prior: &[(
            "im making a birthday cake",
            "answers",
            "A two-layer chocolate or madeira sponge is the easy crowd-pleaser: bake it the day before, wrap it well and fill it with buttercream on the day. One 20cm tin serves 8 to 10 people generously.",
        )],
    },
    Case {
        name: "drive",
        ask: "how long does that take",
        alone: Some(
            "Most people need around 45 hours of lessons plus 20 hours of private practice before they pass, so two lessons a week gets you there in about six months at 40 just as at 18. Lessons run 25 to 40 pounds an hour, and a block with one instructor beats single lessons with three. Do the theory test early, because the pass lasts two years, and ask the price of a test-day car hire before you book the test.",
        ),
        expected: Some(
            "Most people need around 45 hours of lessons plus 20 hours of private practice before they pass, so two lessons a week gets you there in about six months at 40 just as at 18. Lessons run 25 to 40 pounds an hour, and a block with one instructor beats single lessons with three. Do the theory test early, because the pass lasts two years, and ask the price of a test-day car hire before you book the test.",
        ),
        prior: &[(
            "im learning to drive",
            "chat",
            "Worth doing. What would you like help with first?",
        )],
    },
    Case {
        name: "hob",
        ask: "how long on the hob",
        alone: Some(
            "A hob lasts about ten to fifteen years if it is looked after: a ceramic or induction hob usually dies at the touch controls, so keep spills off them. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
        expected: Some(
            "Dried pasta is 8 to 12 minutes, depending on the shape: 8 to 9 for thin spaghetti. Use a big pan with about 10g of salt per litre, keep a mug of the water for the sauce, and taste a piece a minute before the packet says: shapes differ, and so do pans.",
        ),
        prior: &[(
            "im cooking pasta",
            "answers",
            "Use a big pan of well-salted water, about 10g of salt per litre, and cook the pasta one minute less than the packet says. Keep a mug of the cooking water: a splash of it makes any sauce cling to the pasta.",
        )],
    },
    Case {
        name: "bad-place",
        ask: "how long in the oven",
        alone: None,
        expected: None,
        prior: &[(
            "im learning to drive",
            "chat",
            "Worth doing. What would you like help with first?",
        )],
    },
    Case {
        name: "rice",
        ask: "how long in the microwave",
        alone: None,
        expected: None,
        prior: &[(
            "how do i reheat rice",
            "answers",
            "Cool cooked rice fast and box it: it keeps 24 hours in the fridge, no longer, and you reheat it once until it's piping hot all the way through. Rice left out at room temperature is the one leftover worth being strict about.",
        )],
    },
    Case {
        name: "spanish",
        ask: "how long until im good at it",
        alone: Some(
            "About 600 hours of real study to hold a proper conversation in Spanish - around 2 years at an hour a day, or three times that at twenty minutes a day, which is why a little every day beats a long weekend. A holiday and small talk come far sooner: about 100 hours, which a twice-weekly evening class reaches in a year.",
        ),
        expected: None,
        prior: &[(
            "im learning spanish",
            "answers",
            "Split your time in three: a few minutes going over yesterday's phrases out loud, most of it on new phrases said aloud and written down once, and a few minutes listening to something short at normal speed. Learn whole phrases you will use rather than grammar tables, and speak from the first week — even to yourself in the kitchen. One lesson or one conversation partner a week is what makes the rest work.",
        )],
    },
    Case {
        name: "boiler",
        ask: "how long will it take",
        alone: Some("Ten minutes."),
        expected: None,
        prior: &[(
            "my boiler is broken",
            "answers",
            "Check the pressure gauge (1 to 1.5 bar), the thermostat and the boiler's reset button, and make sure the programmer is on the hour you think it is; a pressure top-up or a reset fixes a lot of no-heat mornings. The checks take ten minutes. In a cold snap, the white condensate pipe outside can freeze and stop the boiler, so pour warm water along it. If it locks out twice, or you smell gas, stop and call a Gas Safe engineer: gas is not a DIY job.",
        )],
    },
    Case {
        name: "washer",
        ask: "how long will it take",
        alone: Some("Six years."),
        expected: Some(
            "Most jobs are done in one visit: an hour or two on site for a repair, and the wait is usually the part, so if something has to be ordered expect a few days between the first call and the fix. Ask when you book whether the price covers the first hour and whether the part is included, and get the call-out fee before anyone comes out. A boiler swap is the long one - usually a full day, plus the gas check afterwards.",
        ),
        prior: &[(
            "my washing machine has stopped",
            "answers",
            "Start with the plug fuse, the wall switch and the door or any filter: those account for most washing machines that stop dead, and they cost nothing to check. Then ring the retailer with your receipt, because your claim is against the shop rather than the maker - up to six years in England and Wales, five in Scotland, for a repair, a replacement or a refund - and any guarantee you were given sits on top of that right. Before agreeing to work, ask for the quote in writing and for how long the parts are guaranteed.",
        )],
    },
    Case {
        name: "dog",
        ask: "how much would that cost",
        alone: Some(
            "A puppy is about 800 to 3,000 pounds from a breeder depending on the breed, and 150 to 350 from a rescue, and the first year costs more than the puppy does: jabs and a microchip 60 to 100, neutering 150 to 300, insurance 20 to 40 a month, plus food, worming, a crate and a bed. Budget around 1,500 in the first year and keep a vet fund separately. Walk away from anyone who will not show you the mother.",
        ),
        expected: Some(
            "A puppy is about 800 to 3,000 pounds from a breeder depending on the breed, and 150 to 350 from a rescue, and the first year costs more than the puppy does: jabs and a microchip 60 to 100, neutering 150 to 300, insurance 20 to 40 a month, plus food, worming, a crate and a bed. Budget around 1,500 in the first year and keep a vet fund separately. Walk away from anyone who will not show you the mother.",
        ),
        prior: &[
            (
                "im thinking of getting a dog",
                "answers",
                "Want one only if the person who wants it is also the one who will walk it at seven on a wet Tuesday for the next twelve years. A dog costs £1,500 to £2,000 a year before any vet bill and needs someone home at lunchtime or a walker; an adult rescue is far easier than a puppy. If that sounds fine rather than like a sacrifice, get one.",
            ),
            (
                "it would be for the kids",
                "greeting",
                "I'm listening — say more if it helps.",
            ),
        ],
    },
    Case {
        name: "speech",
        ask: "how long should it be",
        alone: None,
        expected: None,
        prior: &[(
            "im worried about the speech",
            "chat",
            "Worry with no end date on it is the tiring kind. Say the worst case out loud and it usually gets a bit smaller.",
        )],
    },
    Case {
        name: "cat",
        ask: "the cost",
        alone: Some(
            "A kitten is about 50 to 150 pounds from a rescue or a friend's litter, and 300 to 800 for a pedigree; the first year costs more than the kitten does: jabs and a microchip 60 to 100, neutering 60 to 120 for a male and more for a female, insurance 8 to 20 a month, plus litter, food and a carrier. Budget around 800 in the first year and keep a vet fund separately.",
        ),
        expected: Some(
            "A kitten is about 50 to 150 pounds from a rescue or a friend's litter, and 300 to 800 for a pedigree; the first year costs more than the kitten does: jabs and a microchip 60 to 100, neutering 60 to 120 for a male and more for a female, insurance 8 to 20 a month, plus litter, food and a carrier. Budget around 800 in the first year and keep a vet fund separately.",
        ),
        prior: &[(
            "were adopting a rescue cat",
            "greeting",
            "Brilliant. Really pleased for you.",
        )],
    },
    Case {
        name: "cat-feed",
        ask: "and the food",
        alone: None,
        expected: Some(
            "Two measured meals a day for an adult cat, adding up to about 3 or 4 pouches of 85g for a 4kg cat, or 40-50g of dry, and less for a cat that sleeps all day: a complete cat food, wet or wet with a little dry. Cats are obligate carnivores, so no dog food and no vegetarian diet, and a cat should never go more than a day without eating. Free-fed dry food is what makes most indoor cats fat: check by feeling the ribs without pressing.",
        ),
        prior: &[(
            "were adopting a rescue cat",
            "greeting",
            "Brilliant. Really pleased for you.",
        )],
    },
    Case {
        name: "new",
        ask: "how much would that cost",
        alone: Some(
            "You have 10 working days from the fail to get the repair done and take the car back for a partial retest - free if it goes back by the end of the next working day - and after that the test starts again from scratch. Sorry to hear it, and it is usually fixable. Read the fail sheet: 'dangerous' means don't drive it at all, a 'major' needs fixing before it can pass, and an advisory is a warning for next time. Many garages do the repair and the free partial retest in one go if you leave the car with them.",
        ),
        expected: None,
        prior: &[
            (
                "my car failed its mot",
                "answers",
                "Sorry to hear it, and it is usually fixable. Read the fail sheet: 'dangerous' means don't drive it at all, a 'major' needs fixing before it can pass, and an advisory is a warning for next time. Many garages do the repair and the free partial retest in one go if you leave the car with them.",
            ),
            (
                "it needs a new spring",
                "answers",
                "On an MOT a fault in the brakes, the steering, the suspension, the structure or a tyre is normally marked 'dangerous', and a dangerous fail means the car must not be driven on the road at all - not even to the garage. Book recovery or ask the garage to collect it, and take it back the next working day, when the partial retest is free. If the sheet says 'major' instead, you can drive it to the garage or to a pre-booked test.",
            ),
        ],
    },
    Case {
        name: "wrong",
        ask: "how much does a passport cost",
        alone: Some(
            "An adult passport is about £95 and a child's about £62, done on gov.uk, and about £12 more each if you post the paper form instead. Do it on gov.uk itself: the websites that charge £20 to fill the same form in for you are charging for a service the site gives away, and a Post Office check-and-send is about £16 on top for checking the photo. The fee goes up each April, and it usually takes about three weeks, longer in the summer.",
        ),
        expected: None,
        prior: &[(
            "how do i fix a dripping tap",
            "answers",
            "A dripping tap is nearly always a worn washer or cartridge: turn the water off at the isolating valve under the sink, take the handle off, and take the old part to a hardware shop to match it. It's a 20-minute job and saves a lot of water.",
        )],
    },
    Case {
        name: "trip",
        ask: "how long do i need",
        alone: Some(
            "Three full days in Rome, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        expected: None,
        prior: &[(
            "im going to rome",
            "answers",
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        )],
    },
    Case {
        name: "runs",
        ask: "how often should i go",
        alone: Some(
            "Three sessions a week is the number to aim for, with a day off between them, and it is the same for running, the gym or swimming. The health guideline is 150 minutes of something that makes you breathe harder each week, which is three 50-minute sessions or five half hours. Two or three weeks in, that gets easier and you can add a session.",
        ),
        expected: None,
        prior: &[(
            "i want to start running",
            "answers",
            "Start with run-walk: one minute of easy jogging, two minutes of walking, repeated for 20 minutes, three times a week with a rest day between. Each week add a minute to the jog and take one off the walk, and in about eight weeks you will run 20 minutes without stopping. Go slowly enough to speak in short sentences - that is the pace that builds the habit.",
        )],
    },
    Case {
        name: "swim",
        ask: "how long until i can do it",
        alone: Some("Eight weeks."),
        expected: Some(
            "Same answer, and it covers this — Three runs a week gets most people to 5k in about eight weeks. If you cannot run five minutes yet, start with run-walk: one minute jogging, two walking, seven times. Once you can run 20 minutes without a break, make one run a week the long one and add five minutes to it until it is 30. Keep the other two short and slow enough to talk in sentences, take a rest day between runs, and keep the last week light.",
        ),
        prior: &[(
            "im training for a 5k",
            "answers",
            "Three runs a week gets most people to 5k in about eight weeks. If you cannot run five minutes yet, start with run-walk: one minute jogging, two walking, seven times. Once you can run 20 minutes without a break, make one run a week the long one and add five minutes to it until it is 30. Keep the other two short and slow enough to talk in sentences, take a rest day between runs, and keep the last week light.",
        )],
    },
    Case {
        name: "nature",
        ask: "and what about the food",
        alone: None,
        expected: None,
        prior: &[(
            "i saw a bat last night",
            "greeting",
            "Sounds alright. Anything good in it?",
        )],
    },
    Case {
        name: "named",
        ask: "how long to clean my oven",
        alone: Some(
            "Make a paste of bicarbonate of soda and water, coat the inside (not the elements), leave it overnight and wipe it out with a damp cloth and a little vinegar. It takes some scrubbing but doesn't fill the kitchen with fumes like caustic oven cleaner. The racks soak in hot water with a dishwasher tablet while the paste works.",
        ),
        expected: None,
        prior: &[(
            "im making a birthday cake",
            "answers",
            "A two-layer chocolate or madeira sponge is the easy crowd-pleaser: bake it the day before, wrap it well and fill it with buttercream on the day. One 20cm tin serves 8 to 10 people generously.",
        )],
    },
    Case {
        name: "empty",
        ask: "how long will it take",
        alone: None,
        expected: None,
        prior: &[],
    },
];

#[test]
fn cake_duration_has_the_distinct_measured_answer_candidate() {
    let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
    bot.turn("im making a birthday cake");
    let turn = bot.turn("how long will it take");
    assert_eq!(turn.module, Some("answers"));
    assert!(turn.candidates.iter().any(|candidate| {
        candidate.module == "answers"
            && candidate.reply == "A 20cm sponge bakes at 180C (160C fan) for 25 to 30 minutes; a loaf tin needs 45 to 55. It's done when a skewer in the middle comes out clean, and it's worth checking 5 minutes early the first time. About an hour and a half start to finish, counting mixing and cooling."
    }));
}

fn check_reference(case: &Case) {
    let memory = Memory {
        turns: case
            .prior
            .iter()
            .map(|&(user, module, bot)| Exchange {
                user: user.into(),
                module,
                bot: bot.into(),
            })
            .collect(),
        ..Memory::default()
    };
    let result = respond(&Reading::new(case.ask), &memory, case.alone);
    assert_eq!(
        result.as_ref().map(|answer| answer.reply.as_str()),
        case.expected,
        "{}",
        case.name
    );
}

#[test]
fn measured_followup_offers_match_the_reference() {
    for case in CASES
        .iter()
        .chain(EXTRA)
        .filter(|case| case.name != "cycle")
    {
        check_reference(case);
    }
}

#[test]
#[ignore = "PyBot maps the widened cycling question to the menstrual cycle card"]
fn cycle_disambiguation_matches_the_reference() {
    check_reference(EXTRA.iter().find(|case| case.name == "cycle").unwrap());
}

const EXTRA: &[Case] = &[
    Case {
        name: "guitar",
        ask: "how long until im good at it",
        alone: Some(
            "With 15–20 minutes a day, four chords and a simple song come together in about a month, and something recognisable with a few more chords in three to six months.",
        ),
        expected: Some(
            "With 15–20 minutes a day, four chords and a simple song come together in about a month, and something recognisable with a few more chords in three to six months.",
        ),
        prior: &[(
            "i want to learn guitar",
            "answers",
            "Start with four chords that fit a hundred songs — G, C, D and E minor — and the strumming hand before the fretting one. Twenty minutes a day gets you a simple song in two or three months, and six lessons at the start mean nobody has to unlearn your hand position.",
        )],
    },
    Case {
        name: "cycle",
        ask: "how long until im good at it",
        alone: Some("No set time on that one - twenty minutes at a time beats an afternoon."),
        expected: Some(
            "The usual cycle is 21 to 35 days from the first day of one period to the first day of the next, and the bleeding itself is usually two to seven days - up to eight is still normal. What is worth a GP appointment: cycles closer than 21 days or longer than 35, bleeding that lasts more than eight days, bleeding between periods or after sex, soaking a pad or tampon an hour for several hours, or three months with no period at all. Write the dates down for three cycles first, because that is what the GP will ask for, and the pill can change the pattern while you are on it.",
        ),
        prior: &[(
            "im learning to cycle",
            "answers",
            "Take the pedals off and drop the saddle until both feet reach the ground, then let them push along and coast with their feet up - a child who can balance for five seconds can ride, and an adult learning does the same on a bike with the saddle low. Hold the back of the saddle, not the handlebars, when they first pedal, and let go without saying so. Twenty minutes at a time beats an afternoon, and a flat, empty car park beats a narrow pavement.",
        )],
    },
    Case {
        name: "laptop",
        ask: "how long will that take",
        alone: Some(
            "A laptop is good for about four to six years and a tablet for three to five, and the clock starts when the software updates stop rather than when it feels slow: a machine with no security fixes is the reason to change, not the speed. The battery is the exception, since £40 to £80 fits a new one to a laptop and £50 to £120 to a phone, and that alone buys another two or three years. If the case and the screen are still sound, a battery is nearly always cheaper than a replacement.",
        ),
        expected: None,
        prior: &[(
            "my laptop stopped charging",
            "answers",
            "Turn on the setting rather than watching the number: an iPhone has Settings > Battery > Charging Optimisation, and most Android phones have Protect Battery or a charge limit in Settings > Battery. Between 20% and 80% is the range a lithium battery lasts longest in, and the makers' own optimised charging gets you most of that without any effort.",
        )],
    },
    Case {
        name: "fridge",
        ask: "how long will it take",
        alone: Some("It's a stage rather than a set stretch: six years is where it starts."),
        expected: Some(
            "Most jobs are done in one visit: an hour or two on site for a repair, and the wait is usually the part, so if something has to be ordered expect a few days between the first call and the fix. Ask when you book whether the price covers the first hour and whether the part is included, and get the call-out fee before anyone comes out. A boiler swap is the long one - usually a full day, plus the gas check afterwards.",
        ),
        prior: &[(
            "my fridge has stopped",
            "answers",
            "A dead appliance is usually the plug, the fuse or a door switch: swap the fuse, try another socket, shut the door firmly and look the fault code up in the manual. If it stays dead, take the model number off the sticker and ask the maker or a local repairer for a fixed-price quote, because under six years old a repair is usually the cheaper way.",
        )],
    },
    Case {
        name: "rice-hob",
        ask: "how long on the hob",
        alone: Some(
            "A hob lasts about ten to fifteen years if it is looked after: a ceramic or induction hob usually dies at the touch controls, so keep spills off them. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
        expected: None,
        prior: &[(
            "im cooking rice",
            "answers",
            "One mug of rice to one and a half mugs of water, lid on, 10 to 12 minutes on the lowest heat, then 10 minutes off the heat with the lid still on before you fork it through. Rinse it until the water runs clear first. Don't stir it and don't drain it - that is what makes it gluey.",
        )],
    },
    Case {
        name: "rice-reheat",
        ask: "how long in the microwave",
        alone: None,
        expected: Some(
            "White long grain takes 10 to 12 minutes on a low simmer with the lid on, then 10 more off the heat. Use 1 part rice to about 1.5 parts water by volume, or the fingertip test: water one knuckle above the rice. Brown and wholegrain are the ones that take longer. Too much water is what turns rice to porridge.",
        ),
        prior: &[(
            "im reheating rice",
            "answers",
            "Cool cooked rice fast and box it: it keeps 24 hours in the fridge, no longer, and you reheat it once until it's piping hot all the way through. Rice left out at room temperature is the one leftover worth being strict about.",
        )],
    },
    Case {
        name: "roast",
        ask: "what temperature",
        alone: Some(
            "As I said: About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes. Check the thigh reads 74C on a thermometer and the juices run clear, then rest it for 15 minutes before carving.",
        ),
        expected: None,
        prior: &[(
            "how do i roast a chicken",
            "answers",
            "About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes. Check the thigh reads 74C on a thermometer and the juices run clear, then rest it for 15 minutes before carving.",
        )],
    },
    Case {
        name: "deposit",
        ask: "how long will it take",
        alone: Some(
            "Open a Lifetime ISA and put a monthly standing order into it: that adds 25% on up to £4,000 a year. Keep your credit file clean and find out what you could borrow, about four and a half times your income, so the deposit is all that is left to save: 10% of a £200,000 flat is £20,000, or £333 a month for five years.",
        ),
        expected: None,
        prior: &[(
            "im saving for a deposit",
            "answers",
            "Aim for 10% of the price: £20,000 on a £200,000 home, and it gets you noticeably better rates than 5%. A Lifetime ISA adds 25% on up to £4,000 a year towards a first home, and a broker can tell you what you'd actually be offered before you start looking.",
        )],
    },
    Case {
        name: "wedding",
        ask: "how long should it be",
        alone: Some("Three minutes."),
        expected: None,
        prior: &[
            (
                "my sister is getting married in june",
                "greeting",
                "That's lovely news — and something to look forward to. How are the plans going?",
            ),
            (
                "im worried about the speech",
                "answers",
                "Three minutes, about 450 words, is the length to aim for: a line thanking the people who got everyone there, one story with a real detail in it that shows what the person is like, then what they mean to you and a toast. Open with a fact everyone in the room recognises, then turn it: \"My sister has spent her life fixing animals, so it's a relief she's finally found one she can train.\" Keep that opening 30 seconds short, and put the sincere bit in the middle where people aren't braced for it.",
            ),
        ],
    },
    Case {
        name: "school",
        ask: "how many hours a week",
        alone: Some(
            "Budget two to three hours of reading and homework for every hour in class: a three-hour evening class is usually 9 to 12 hours a week including the class, and a part-time degree is 16 to 20 hours a week because it is half of a full-time course. The college has to publish those guided-learning hours on the course page, so ask for them before you enrol and pick the two evenings you would protect. Reading is what gets dropped first, so put it in the diary as an appointment rather than a hope.",
        ),
        expected: None,
        prior: &[(
            "im starting a school course",
            "greeting",
            "A fresh start, then. What is the first thing you will do?",
        )],
    },
    Case {
        name: "repair-repeat",
        ask: "how long will it take",
        alone: Some("About an hour for that one, start to finish."),
        expected: None,
        prior: &[
            (
                "my washing machine has stopped",
                "answers",
                "Start with the plug fuse, the wall switch and the door or any filter: those account for most washing machines that stop dead, and they cost nothing to check. Then ring the retailer with your receipt, because your claim is against the shop rather than the maker - up to six years in England and Wales, five in Scotland, for a repair, a replacement or a refund - and any guarantee you were given sits on top of that right. Before agreeing to work, ask for the quote in writing and for how long the parts are guaranteed.",
            ),
            (
                "how long will it take",
                "answers",
                "Most jobs are done in one visit: an hour or two on site for a repair, and the wait is usually the part, so if something has to be ordered expect a few days between the first call and the fix. Ask when you book whether the price covers the first hour and whether the part is included, and get the call-out fee before anyone comes out. A boiler swap is the long one - usually a full day, plus the gas check afterwards.",
            ),
        ],
    },
];
