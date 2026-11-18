//! Seeded direct reply decisions for the household upkeep table.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub Option<&'static str>,
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "appliance-life",
        "how long does a washing machine last",
        &[],
        None,
        Some(
            "Expect a washing machine to last about eight to ten years - a 60C service wash once a month, a clean filter and a drum that is never overloaded are what keep it at the top of that. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "what is the lifespan of a tumble dryer",
        &[],
        None,
        Some(
            "Expect a tumble dryer to last about ten to thirteen years - emptying the lint filter after every load is the whole of it, because a blocked filter is what kills dryers. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "how long should my sofa be",
        &[],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how long should a mattress last",
        &[],
        None,
        Some(
            "Expect a mattress to last about eight to ten years - airing it and turning or rotating it every few months is what keeps it there. Replace it once it sags or the springs go, or you wake up stiff.",
        ),
    ),
    Case(
        "appliance-life",
        "what's the life of the newest air fryer",
        &[],
        None,
        Some(
            "Expect an air fryer to last about five to eight years - washing the basket by hand rather than in the dishwasher and not overfilling it are most of it, and the non-stick goes before the element does. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "how long will a boiler last",
        &[],
        None,
        Some(
            "Expect a boiler to last about ten to fifteen years - an annual service is the whole of it, and the last third of that range is where repair bills start to beat a new one. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "how long do appliances last",
        &[],
        None,
        Some(
            "Washing machines and dishwashers last about eight to ten years, a fridge freezer, oven or tumble dryer ten to fifteen, and a kettle or a toaster three to five. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "how long does it take to reheat in the microwave",
        &[],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how long does bread last in the freezer",
        &[],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how long should i microwave oats",
        &[],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how long does a freezer stay cold without power",
        &[],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how long before i should replace my washing machine",
        &[],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "do hair dryers last a long time",
        &[],
        None,
        Some(
            "Expect a hair dryer to last about four to six years - cleaning the filter at the back is what stops it overheating. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "how long can i stay in the kitchen",
        &[],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how long will a brand new freezer last without food",
        &[],
        None,
        Some(
            "Expect a freezer to last about ten to fifteen years - a freezer kept full and defrosted when the ice thickens lasts longest. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "how long should a carpet last",
        &[],
        None,
        Some(
            "Expect a carpet to last about ten years in a room people walk through every day - the underlay matters as much as the carpet, and a cheap carpet on a good underlay outlasts a good one laid straight onto the floor. Replace it when the pile is worn through on the walkways - a professional clean brings a dirty carpet back once, not a worn one.",
        ),
    ),
    Case(
        "appliance-life",
        "how good is a fridge freezer",
        &[],
        None,
        Some(
            "Expect a fridge freezer to last about ten to fifteen years - dusting the coils at the back once a year and keeping it out of a hot corner is most of it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "are new toasters good",
        &[],
        None,
        Some(
            "Expect a toaster to last about three to five years - emptying the crumb tray and never poking a knife in it is most of it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "how long will a hot water bottle stay warm",
        &[],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how many years will our oven last",
        &[],
        None,
        Some(
            "Expect an oven to last about ten to fifteen years - the element and the thermostat are the parts that go and both are cheap to swap, so one fault is almost never a reason to replace it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "bedding-finished",
        "when should i replace my mattress",
        &[],
        None,
        Some(
            "Expect a mattress to last about eight to ten years - airing it and turning or rotating it every few months is what keeps it there. Replace it once it sags or the springs go, or you wake up stiff.",
        ),
    ),
    Case(
        "bedding-finished",
        "when do i need a new mattress",
        &[],
        None,
        Some(
            "Expect a mattress to last about eight to ten years - airing it and turning or rotating it every few months is what keeps it there. Replace it once it sags or the springs go, or you wake up stiff.",
        ),
    ),
    Case(
        "bedding-finished",
        "how often should i replace pillows",
        &[],
        None,
        None,
    ),
    Case(
        "bedding-finished",
        "my duvet has collapsed",
        &[],
        None,
        None,
    ),
    Case(
        "bedding-finished",
        "is it time for a new pillow",
        &[],
        None,
        None,
    ),
    Case(
        "bedding-finished",
        "how long do bed linens last",
        &[],
        None,
        None,
    ),
    Case("bedding-finished", "my sofa has sagged", &[], None, None),
    Case(
        "bedding-finished",
        "how long do mattresses stay comfortable",
        &[],
        None,
        Some(
            "Expect a mattress to last about eight to ten years - airing it and turning or rotating it every few months is what keeps it there. Replace it once it sags or the springs go, or you wake up stiff.",
        ),
    ),
    Case(
        "bedding-finished",
        "how long should sheets be",
        &[],
        None,
        None,
    ),
    Case(
        "bedding-finished",
        "when should i replace the whole bed",
        &[],
        None,
        None,
    ),
    Case("bedding-finished", "i need a new duvet", &[], None, None),
    Case(
        "bedding-finished",
        "when is it time to replace pillows",
        &[],
        None,
        None,
    ),
    Case(
        "running-cost",
        "how much does a tumble dryer cost",
        &[],
        None,
        Some(
            "A vented tumble dryer is £250 to £400, a condenser £300 to £500, and a heat pump £350 to £600 - the dearer one pays the difference back in about two years of running costs. To run it, it is about 25p a cycle with a heat pump and more like 60p with a vented one.",
        ),
    ),
    Case(
        "running-cost",
        "does an air fryer cost more to run than an oven",
        &[],
        None,
        Some(
            "An air fryer is the cheaper one, about 15 to 20p an hour, and it is hot in three minutes; an electric oven is about 50p an hour at 180C, and much of that is the warm-up. The heat is nearly all of the cost, so fill whatever you use and skip the preheat.",
        ),
    ),
    Case(
        "running-cost",
        "how much to use the washing machine",
        &[],
        None,
        Some("A washing machine is about 25p a cycle at 40C and nearer 15p at 30C."),
    ),
    Case(
        "running-cost",
        "how much electricity does a fan heater use",
        &[],
        None,
        Some(
            "An electric heater is about 45 to 50p an hour whatever the type, because they all turn electricity into heat the same way.",
        ),
    ),
    Case(
        "running-cost",
        "running cost of a fan oven",
        &[],
        None,
        Some("An electric oven is about 50p an hour at 180C, and much of that is the warm-up."),
    ),
    Case(
        "running-cost",
        "fan vs fan heater",
        &[],
        None,
        Some(
            "A fan is the cheaper one, a penny or two an hour, and it cools you rather than the room; an electric heater is about 45 to 50p an hour whatever the type, because they all turn electricity into heat the same way.",
        ),
    ),
    Case(
        "running-cost",
        "is a freezer cheaper than a fridge",
        &[],
        None,
        Some(
            "A fridge freezer is about 25p a day, which works out near a penny an hour because it only ticks over.",
        ),
    ),
    Case("running-cost", "what does a heater cost", &[], None, None),
    Case(
        "running-cost",
        "running a clothes dryer and an air fryer",
        &[],
        None,
        Some("An air fryer is about 15 to 20p an hour, and it is hot in three minutes."),
    ),
    Case(
        "running-cost",
        "electric oven vs slow cooker vs microwave",
        &[],
        None,
        Some(
            "A slow cooker is the cheaper one, a couple of pence an hour, so around 15p for a whole afternoon; an electric oven is about 50p an hour at 180C, and much of that is the warm-up. The heat is nearly all of the cost, so fill whatever you use and skip the preheat.",
        ),
    ),
    Case(
        "running-cost",
        "what is more expensive an oil radiator or an electric oven",
        &[],
        None,
        Some(
            "An electric oven is the cheaper one, about 50p an hour at 180C, and much of that is the warm-up; gas central heating is about 60p an hour while the boiler is actually firing, so five hours a day is £2 to £3, and a heat pump or electric heating is cheaper per unit of heat only if it is a real heat pump.",
        ),
    ),
    Case("running-cost", "best washing cost washing", &[], None, None),
    Case(
        "running-cost",
        "how much is a dehumidifier",
        &[],
        None,
        Some(
            "A 12 to 20 litre dehumidifier is £150 to £250, and a desiccant one for a cold garage or a flat that never warms up is £200 to £300. To run it, it is about 7p an hour, and it dries a room by running longer rather than hotter.",
        ),
    ),
    Case(
        "running-cost",
        "how much does a dehumidifier cost to run",
        &[],
        None,
        Some(
            "A dehumidifier is about 7p an hour, and it dries a room by running longer rather than hotter.",
        ),
    ),
    Case(
        "running-cost",
        "what does the cooker cost per hour",
        &[],
        None,
        None,
    ),
    Case(
        "running-cost",
        "does running it at night save money",
        &[],
        None,
        None,
    ),
    Case(
        "running-cost",
        "air fryer, microwave, oven",
        &[],
        None,
        Some(
            "A microwave is the cheaper one, pennies for a reheated plate, because it runs for minutes not hours; an air fryer is about 15 to 20p an hour, and it is hot in three minutes. The heat is nearly all of the cost, so fill whatever you use and skip the preheat.",
        ),
    ),
    Case(
        "running-cost",
        "cost of an induction hob vs gas hob",
        &[],
        None,
        Some(
            "A gas hob is the cheaper one, roughly 10 to 15p an hour of use, the cheapest of the hobs; an induction hob is about 15 to 20p an hour, close to gas and about twice as quick to boil.",
        ),
    ),
    Case(
        "running-cost",
        "how expensive is a microwave",
        &[],
        None,
        Some("A microwave is pennies for a reheated plate, because it runs for minutes not hours."),
    ),
    Case(
        "running-cost",
        "a 1000 watt kettle",
        &[],
        None,
        Some("A kettle is about 2 to 3p a boil, so boil only what you are going to drink."),
    ),
    Case(
        "worn-out-kit",
        "how long do running shoes last",
        &[],
        None,
        Some(
            "Replace running shoes every 500 to 800km, which is six to twelve months for most people: the midsole foam goes before the tread looks worn, so the sign is a shoe that feels flat or leaves you stiff the morning after a run. Keep the old pair for the garden.",
        ),
    ),
    Case(
        "worn-out-kit",
        "when should i replace my smoke alarm",
        &[],
        None,
        Some(
            "Replace a smoke alarm every ten years, with a new battery every year: the sensor drifts with age and most modern ones are sealed, so the replace-by date stamped on the back is the one to go by, and it should be tested monthly.",
        ),
    ),
    Case(
        "worn-out-kit",
        "how often do smoke alarm batteries need changing",
        &[],
        None,
        Some(
            "Replace a smoke alarm every ten years, with a new battery every year: the sensor drifts with age and most modern ones are sealed, so the replace-by date stamped on the back is the one to go by, and it should be tested monthly.",
        ),
    ),
    Case(
        "worn-out-kit",
        "when does a toothbrush wear out",
        &[],
        None,
        None,
    ),
    Case(
        "worn-out-kit",
        "how long will a smoke detector last",
        &[],
        None,
        Some(
            "Replace a smoke alarm every ten years, with a new battery every year: the sensor drifts with age and most modern ones are sealed, so the replace-by date stamped on the back is the one to go by, and it should be tested monthly.",
        ),
    ),
    Case(
        "worn-out-kit",
        "my running shoes are worn out",
        &[],
        None,
        Some(
            "Replace running shoes every 500 to 800km, which is six to twelve months for most people: the midsole foam goes before the tread looks worn, so the sign is a shoe that feels flat or leaves you stiff the morning after a run. Keep the old pair for the garden.",
        ),
    ),
    Case("worn-out-kit", "do bike tyres wear out", &[], None, None),
    Case(
        "worn-out-kit",
        "how long does a phone battery last",
        &[],
        None,
        Some(
            "Replace a phone battery every two to three years, or about 500 full charges: the settings show the health: on an iPhone it is Settings, Battery, Battery Health, and anything under 80% is worn out. A phone that dies at 20%, shuts down in the cold or gets hot on charge is the same verdict, and a new battery is £30 to £70 fitted rather than a new handset.",
        ),
    ),
    Case("worn-out-kit", "would washing it help", &[], None, None),
    Case(
        "worn-out-kit",
        "how long does the hair dryer last",
        &[],
        None,
        None,
    ),
    Case(
        "worn-out-kit",
        "replacement timing for wiper blades",
        &[],
        None,
        None,
    ),
    Case(
        "worn-out-kit",
        "how long do trainers usually last",
        &[],
        None,
        Some(
            "Replace running shoes every 500 to 800km, which is six to twelve months for most people: the midsole foam goes before the tread looks worn, so the sign is a shoe that feels flat or leaves you stiff the morning after a run. Keep the old pair for the garden.",
        ),
    ),
    Case(
        "worn-out-kit",
        "how often should i change contacts",
        &[],
        None,
        Some(
            "Replace contact lenses as often as the pack says: daily ones every day, monthlies every month, and never past the date on the blister: the material collects protein and deposits as it is worn and the case is where the biofilm grows, so change the case every three months too.",
        ),
    ),
    Case("worn-out-kit", "how long do sponges last", &[], None, None),
    Case(
        "door-lock-stiff",
        "stiff key",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "my door lock is stiff",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "the latch is sticky",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "it's stiff to turn",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "the key will not turn",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "turning the key in the front",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "front starter is stuck",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "what about the barrel",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "it is stuck in the keyhole",
        &[],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "appliance-life",
        "how long do they usually last",
        &["my washing machine is nine years old"],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how long does it normally last",
        &["we have a dishwasher"],
        None,
        None,
    ),
    Case(
        "appliance-life",
        "how long does it normally last",
        &[],
        Some("we have a dishwasher"),
        Some(
            "Expect a dishwasher to last about eight to ten years - rinsing the filter monthly and using salt if your water is hard keeps it at the top of that. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ),
    ),
    Case(
        "appliance-life",
        "how long should i keep it",
        &[],
        Some("i have a microwave"),
        None,
    ),
    Case(
        "bedding-finished",
        "when should i replace it",
        &[],
        Some("my mattress is sagging"),
        Some(
            "Expect a mattress to last about eight to ten years - airing it and turning or rotating it every few months is what keeps it there. Replace it once it sags or the springs go, or you wake up stiff.",
        ),
    ),
    Case(
        "bedding-finished",
        "how long does a duvet last",
        &["that's the same duvet"],
        None,
        None,
    ),
    Case(
        "door-lock-stiff",
        "it clicks when i turn the key",
        &["my car has a starter issue"],
        None,
        None,
    ),
    Case(
        "door-lock-stiff",
        "it clicks when i turn the key",
        &[
            "the car is an old honda",
            "it's been serviced",
            "thanks",
            "yes",
            "quite",
        ],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "it clicks when i turn the key in the lock",
        &["my car has a starter issue"],
        None,
        Some(
            "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.",
        ),
    ),
    Case(
        "door-lock-stiff",
        "it clicks when i turn the key",
        &["does my bicycle need a lock"],
        None,
        None,
    ),
];
