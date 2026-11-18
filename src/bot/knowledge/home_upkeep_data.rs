//! Bounds and authored replies from the pinned appliance and household cards.

pub(super) const FIRST_PERSON: &str = r#"(?i)\b(?:i|im|i'm|my|me|we|us|our)\b|\bhow\s+long\s+to\b"#;
pub(super) const A_LIFE: &str = r#"(?i)\b(?:last|lasts|lasting|lasted|life|lifespan|life\s*span|expectancy|expect|years?|old)\b"#;
pub(super) const WHEN_TO_REPLACE: &str = r#"(?i)\bwhen\s+to\s+replac|\bwhen\s+(?:should|shall|do|does|would)\s+(?:i\s+|we\s+|you\s+)?replac|\bshould\s+(?:i|we|you)\s+replac|\btime\s+to\s+replac|\btime\s+(?:for|i)\s+(?:a\s+)?(?:new|replac)|\bneed\s+a\s+new\b"#;
pub(super) const HOLDS: &str = r#"(?i)\b(?:stay|stays|staying|hold|holds|holding)\b"#;
pub(super) const HOLDS_A_THING: &str = r#"(?i)\b(?:cold|hot|warm|frozen|chilled|cool|ice|heat|power|off|outage|without|temperature)\b"#;
pub(super) const THE_PRICE_OF: &str = r#"(?i)\b(?:how much (?:does|do|is|are|would|will)|what (?:does|do|is|are) (?:a|an|the|one)|price of|prices of|cost of (?:a|an|the)\b|cost to buy|how much for|what do (?:a|an|the)\b)"#;
pub(super) const A_COMPARISON_SAID: &str = r#"(?:^|[^a-z])(?:cheaper|cheap|cheapest|expensive|more\ than|less\ than|save|saving|savings|compare|comparison|rather|worth\ running|worth\ leaving|better\ than|worse\ than)(?:$|[^a-z])"#;
pub(super) const WHAT_IT_IS_FOR_SAID: &str = r#"(?:^|[^a-z])(?:run|runs|running|ran|to\ use|using|use\ it|cycle|cycles|wash|load|loads|boil|boils|boiling|hour|hours|minute|minutes|day|days|week|weeks|month|months|night|overnight|per|a\ go|each\ go|electricity|energy|kwh|unit|units|standby|left\ on|leave\ it\ on|keep\ it\ on|on\ all\ day|all\ day|bill|bills|quarter|year)(?:$|[^a-z])"#;
pub(super) const IN_A_MACHINE: &[&str] = &[
    "washing machine",
    "washer",
    "tumble dryer",
    "dishwasher",
    "fridge",
    "freezer",
    "oven",
    "cooker",
    "hob",
    "microwave",
    "boiler",
    "dryer",
    "vacuum",
    "hoover",
    "sandwich maker",
    "hair dryer",
    "hairdryer",
    "toaster",
    "kettle",
    "blender",
];
pub(super) const COOKING: &[&str] = &[
    "an electric oven",
    "an air fryer",
    "a microwave",
    "a slow cooker",
];
pub(super) const NAMES_A_LOCK: &[&str] = &[
    "door", "doors", "lock", "locks", "barrel", "deadlock", "yale", "latch", "keyhole", "front",
    "back",
];
pub(super) const ELSEWHERE_KEY: &[&str] = &[
    "car",
    "cars",
    "van",
    "bike",
    "bicycle",
    "motorbike",
    "scooter",
    "laptop",
    "computer",
    "suitcase",
    "padlock",
    "garage",
    "safe",
    "keyboard",
];
pub(super) const LOCK_STIFF: &str = "Puff graphite powder, or a PTFE dry lubricant, into the keyhole and work the key in and out a dozen times: that is the fix for a stiff barrel, and unlike oil or grease it does not collect grit. If it went hard to turn all at once, try the spare key first, because a worn or slightly bent key is the usual cause and a copy cut from the spare solves it. WD-40 frees a seized lock, but finish with graphite. If the key snaps off inside, or the barrel spins, that is a locksmith, and keep the door on the latch until they come.";
pub(super) const RUNNING_TAIL: &str =
    "The heat is nearly all of the cost, so fill whatever you use and skip the preheat.";
pub(super) const GENERAL_LIFE: &str = "Washing machines and dishwashers last about eight to ten years, a fridge freezer, oven or tumble dryer ten to fifteen, and a kettle or a toaster three to five. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.";
pub(super) const WEAR: &[(&str, &str)] = &[
    (
        "running trainers",
        "Replace running shoes every 500 to 800km, which is six to twelve months for most people: the midsole foam goes before the tread looks worn, so the sign is a shoe that feels flat or leaves you stiff the morning after a run. Keep the old pair for the garden.",
    ),
    (
        "climbing harness",
        "Replace a climbing harness every five years of regular use, and ten years from new even if it has hardly been used: the webbing and the stitching age from sweat, sun and abrasion rather than from wear you can see, so the date on the label is the rule; retire it at once after any load that was not a normal fall, and after a big fall.",
    ),
    (
        "smoke detectors",
        "Replace a smoke alarm every ten years, with a new battery every year: the sensor drifts with age and most modern ones are sealed, so the replace-by date stamped on the back is the one to go by, and it should be tested monthly.",
    ),
    (
        "cycling helmet",
        "Replace a bike helmet every three to five years, and at once after any knock: the foam is what absorbs a hit and it does not spring back, so a helmet that has been crashed or dropped hard is finished even when the shell looks perfect, and a chalky or cracked shell is the other sign.",
    ),
    (
        "smoke detector",
        "Replace a smoke alarm every ten years, with a new battery every year: the sensor drifts with age and most modern ones are sealed, so the replace-by date stamped on the back is the one to go by, and it should be tested monthly.",
    ),
    (
        "contact lenses",
        "Replace contact lenses as often as the pack says: daily ones every day, monthlies every month, and never past the date on the blister: the material collects protein and deposits as it is worn and the case is where the biofilm grows, so change the case every three months too.",
    ),
    (
        "monthly lenses",
        "Replace contact lenses as often as the pack says: daily ones every day, monthlies every month, and never past the date on the blister: the material collects protein and deposits as it is worn and the case is where the biofilm grows, so change the case every three months too.",
    ),
    (
        "running shoes",
        "Replace running shoes every 500 to 800km, which is six to twelve months for most people: the midsole foam goes before the tread looks worn, so the sign is a shoe that feels flat or leaves you stiff the morning after a run. Keep the old pair for the garden.",
    ),
    (
        "cycling shoes",
        "Replace cycling shoes and cleats every 1,000 to 2,000km for the cleats, and a new pair of shoes when the sole or the ratchet has gone: the cleat wears on the side you put your foot down first, so it starts clipping in loose or rocking side to side, and a shoe whose sole flexes over the pedal is spreading the load onto your foot instead.",
    ),
    (
        "walking boots",
        "Replace walking boots every 800 to 1,000 miles, or sooner if the sole has gone thin under the ball of the foot: a boot is finished when the midsole has compressed and it no longer holds your foot over rough ground, which is why a leather upper in fine condition can still be a boot to throw away; a resole buys another few hundred miles.",
    ),
    (
        "phone battery",
        "Replace a phone battery every two to three years, or about 500 full charges: the settings show the health: on an iPhone it is Settings, Battery, Battery Health, and anything under 80% is worn out. A phone that dies at 20%, shuts down in the cold or gets hot on charge is the same verdict, and a new battery is £30 to £70 fitted rather than a new handset.",
    ),
    (
        "running shoe",
        "Replace running shoes every 500 to 800km, which is six to twelve months for most people: the midsole foam goes before the tread looks worn, so the sign is a shoe that feels flat or leaves you stiff the morning after a run. Keep the old pair for the garden.",
    ),
    (
        "cycle helmet",
        "Replace a bike helmet every three to five years, and at once after any knock: the foam is what absorbs a hit and it does not spring back, so a helmet that has been crashed or dropped hard is finished even when the shell looks perfect, and a chalky or cracked shell is the other sign.",
    ),
    (
        "smoke alarms",
        "Replace a smoke alarm every ten years, with a new battery every year: the sensor drifts with age and most modern ones are sealed, so the replace-by date stamped on the back is the one to go by, and it should be tested monthly.",
    ),
    (
        "contact lens",
        "Replace contact lenses as often as the pack says: daily ones every day, monthlies every month, and never past the date on the blister: the material collects protein and deposits as it is worn and the case is where the biofilm grows, so change the case every three months too.",
    ),
    (
        "daily lenses",
        "Replace contact lenses as often as the pack says: daily ones every day, monthlies every month, and never past the date on the blister: the material collects protein and deposits as it is worn and the case is where the biofilm grows, so change the case every three months too.",
    ),
    (
        "walking boot",
        "Replace walking boots every 800 to 1,000 miles, or sooner if the sole has gone thin under the ball of the foot: a boot is finished when the midsole has compressed and it no longer holds your foot over rough ground, which is why a leather upper in fine condition can still be a boot to throw away; a resole buys another few hundred miles.",
    ),
    (
        "hiking boots",
        "Replace walking boots every 800 to 1,000 miles, or sooner if the sole has gone thin under the ball of the foot: a boot is finished when the midsole has compressed and it no longer holds your foot over rough ground, which is why a leather upper in fine condition can still be a boot to throw away; a resole buys another few hundred miles.",
    ),
    (
        "bike helmet",
        "Replace a bike helmet every three to five years, and at once after any knock: the foam is what absorbs a hit and it does not spring back, so a helmet that has been crashed or dropped hard is finished even when the shell looks perfect, and a chalky or cracked shell is the other sign.",
    ),
    (
        "smoke alarm",
        "Replace a smoke alarm every ten years, with a new battery every year: the sensor drifts with age and most modern ones are sealed, so the replace-by date stamped on the back is the one to go by, and it should be tested monthly.",
    ),
    (
        "cycle shoes",
        "Replace cycling shoes and cleats every 1,000 to 2,000km for the cleats, and a new pair of shoes when the sole or the ratchet has gone: the cleat wears on the side you put your foot down first, so it starts clipping in loose or rocking side to side, and a shoe whose sole flexes over the pedal is spreading the load onto your foot instead.",
    ),
    (
        "hiking boot",
        "Replace walking boots every 800 to 1,000 miles, or sooner if the sole has gone thin under the ball of the foot: a boot is finished when the midsole has compressed and it no longer holds your foot over rough ground, which is why a leather upper in fine condition can still be a boot to throw away; a resole buys another few hundred miles.",
    ),
    (
        "harnesses",
        "Replace a climbing harness every five years of regular use, and ten years from new even if it has hardly been used: the webbing and the stitching age from sweat, sun and abrasion rather than from wear you can see, so the date on the label is the rule; retire it at once after any load that was not a normal fall, and after a big fall.",
    ),
    (
        "batteries",
        "Replace a phone battery every two to three years, or about 500 full charges: the settings show the health: on an iPhone it is Settings, Battery, Battery Health, and anything under 80% is worn out. A phone that dies at 20%, shuts down in the cold or gets hot on charge is the same verdict, and a new battery is £30 to £70 fitted rather than a new handset.",
    ),
    (
        "trainers",
        "Replace running shoes every 500 to 800km, which is six to twelve months for most people: the midsole foam goes before the tread looks worn, so the sign is a shoe that feels flat or leaves you stiff the morning after a run. Keep the old pair for the garden.",
    ),
    (
        "contacts",
        "Replace contact lenses as often as the pack says: daily ones every day, monthlies every month, and never past the date on the blister: the material collects protein and deposits as it is worn and the case is where the biofilm grows, so change the case every three months too.",
    ),
    (
        "clipless",
        "Replace cycling shoes and cleats every 1,000 to 2,000km for the cleats, and a new pair of shoes when the sole or the ratchet has gone: the cleat wears on the side you put your foot down first, so it starts clipping in loose or rocking side to side, and a shoe whose sole flexes over the pedal is spreading the load onto your foot instead.",
    ),
    (
        "runners",
        "Replace running shoes every 500 to 800km, which is six to twelve months for most people: the midsole foam goes before the tread looks worn, so the sign is a shoe that feels flat or leaves you stiff the morning after a run. Keep the old pair for the garden.",
    ),
    (
        "helmets",
        "Replace a bike helmet every three to five years, and at once after any knock: the foam is what absorbs a hit and it does not spring back, so a helmet that has been crashed or dropped hard is finished even when the shell looks perfect, and a chalky or cracked shell is the other sign.",
    ),
    (
        "harness",
        "Replace a climbing harness every five years of regular use, and ten years from new even if it has hardly been used: the webbing and the stitching age from sweat, sun and abrasion rather than from wear you can see, so the date on the label is the rule; retire it at once after any load that was not a normal fall, and after a big fall.",
    ),
    (
        "battery",
        "Replace a phone battery every two to three years, or about 500 full charges: the settings show the health: on an iPhone it is Settings, Battery, Battery Health, and anything under 80% is worn out. A phone that dies at 20%, shuts down in the cold or gets hot on charge is the same verdict, and a new battery is £30 to £70 fitted rather than a new handset.",
    ),
    (
        "helmet",
        "Replace a bike helmet every three to five years, and at once after any knock: the foam is what absorbs a hit and it does not spring back, so a helmet that has been crashed or dropped hard is finished even when the shell looks perfect, and a chalky or cracked shell is the other sign.",
    ),
    (
        "cleats",
        "Replace cycling shoes and cleats every 1,000 to 2,000km for the cleats, and a new pair of shoes when the sole or the ratchet has gone: the cleat wears on the side you put your foot down first, so it starts clipping in loose or rocking side to side, and a shoe whose sole flexes over the pedal is spreading the load onto your foot instead.",
    ),
    (
        "cleat",
        "Replace cycling shoes and cleats every 1,000 to 2,000km for the cleats, and a new pair of shoes when the sole or the ratchet has gone: the cleat wears on the side you put your foot down first, so it starts clipping in loose or rocking side to side, and a shoe whose sole flexes over the pedal is spreading the load onto your foot instead.",
    ),
    (
        "boots",
        "Replace walking boots every 800 to 1,000 miles, or sooner if the sole has gone thin under the ball of the foot: a boot is finished when the midsole has compressed and it no longer holds your foot over rough ground, which is why a leather upper in fine condition can still be a boot to throw away; a resole buys another few hundred miles.",
    ),
];
pub(super) struct Life(pub &'static [&'static str], pub &'static [&'static str]);
pub(super) const LIFESPAN: &[Life] = &[
    Life(
        &["washing machine", "washer"],
        &[
            "A washing machine lasts about eight to ten years if it is looked after: a 60C service wash once a month, a clean filter and a drum that is never overloaded are what keep it at the top of that. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a washing machine to last about eight to ten years - a 60C service wash once a month, a clean filter and a drum that is never overloaded are what keep it at the top of that. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["dishwasher"],
        &[
            "A dishwasher lasts about eight to ten years if it is looked after: rinsing the filter monthly and using salt if your water is hard keeps it at the top of that. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a dishwasher to last about eight to ten years - rinsing the filter monthly and using salt if your water is hard keeps it at the top of that. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["tumble dryer", "tumbler", "dryer"],
        &[
            "A tumble dryer lasts about ten to thirteen years if it is looked after: emptying the lint filter after every load is the whole of it, because a blocked filter is what kills dryers. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a tumble dryer to last about ten to thirteen years - emptying the lint filter after every load is the whole of it, because a blocked filter is what kills dryers. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["fridge freezer"],
        &[
            "A fridge freezer lasts about ten to fifteen years if it is looked after: dusting the coils at the back once a year and keeping it out of a hot corner is most of it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a fridge freezer to last about ten to fifteen years - dusting the coils at the back once a year and keeping it out of a hot corner is most of it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["fridge"],
        &[
            "A fridge lasts about ten to fifteen years if it is looked after: dusting the coils at the back once a year and keeping it out of a hot corner is most of it, and a door seal that grips a sheet of paper is what keeps the cold in. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a fridge to last about ten to fifteen years - dusting the coils at the back once a year and keeping it out of a hot corner is most of it, and a door seal that grips a sheet of paper is what keeps the cold in. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["freezer"],
        &[
            "A freezer lasts about ten to fifteen years if it is looked after: a freezer kept full and defrosted when the ice thickens lasts longest. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a freezer to last about ten to fifteen years - a freezer kept full and defrosted when the ice thickens lasts longest. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["range cooker"],
        &[
            "A range cooker lasts about fifteen to twenty years if it is looked after: the element and the thermostat are the parts that go and both are cheap to swap, so one fault is almost never a reason to replace it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a range cooker to last about fifteen to twenty years - the element and the thermostat are the parts that go and both are cheap to swap, so one fault is almost never a reason to replace it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["cooker", "stove"],
        &[
            "A cooker lasts about ten to fifteen years if it is looked after: the element and the thermostat are the parts that go and both are cheap to swap, so one fault is almost never a reason to replace it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a cooker to last about ten to fifteen years - the element and the thermostat are the parts that go and both are cheap to swap, so one fault is almost never a reason to replace it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["oven"],
        &[
            "An oven lasts about ten to fifteen years if it is looked after: the element and the thermostat are the parts that go and both are cheap to swap, so one fault is almost never a reason to replace it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect an oven to last about ten to fifteen years - the element and the thermostat are the parts that go and both are cheap to swap, so one fault is almost never a reason to replace it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["slow cooker"],
        &[
            "A slow cooker lasts about five to ten years if it is looked after: the stoneware and the seal are what go first, so wash it by hand rather than in the dishwasher and do not plunge it hot into cold water. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a slow cooker to last about five to ten years - the stoneware and the seal are what go first, so wash it by hand rather than in the dishwasher and do not plunge it hot into cold water. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &[
            "heat pump",
            "heat pumps",
            "air source heat pump",
            "ground source heat pump",
        ],
        &[
            "A heat pump lasts about fifteen to twenty years if it is looked after: an annual service and a clean filter are most of it, and the compressor and the outdoor unit are what go first. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a heat pump to last about fifteen to twenty years - an annual service and a clean filter are most of it, and the compressor and the outdoor unit are what go first. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["boiler"],
        &[
            "A boiler lasts about ten to fifteen years if it is looked after: an annual service is the whole of it, and the last third of that range is where repair bills start to beat a new one. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a boiler to last about ten to fifteen years - an annual service is the whole of it, and the last third of that range is where repair bills start to beat a new one. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["microwave"],
        &[
            "A microwave lasts about seven to ten years if it is looked after: never running it empty and keeping the door seal clean is most of it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a microwave to last about seven to ten years - never running it empty and keeping the door seal clean is most of it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["vacuum cleaner", "hoover", "vacuum"],
        &[
            "A vacuum cleaner lasts about six to eight years if it is looked after: emptying the bin and washing the filters is most of it; the motor is what fails in the end. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a vacuum cleaner to last about six to eight years - emptying the bin and washing the filters is most of it; the motor is what fails in the end. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["kettle"],
        &[
            "A kettle lasts about three to five years if it is looked after: descaling it now and then is most of it, because the element is what scales up and dies. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a kettle to last about three to five years - descaling it now and then is most of it, because the element is what scales up and dies. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["toaster"],
        &[
            "A toaster lasts about three to five years if it is looked after: emptying the crumb tray and never poking a knife in it is most of it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a toaster to last about three to five years - emptying the crumb tray and never poking a knife in it is most of it. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["air fryer", "airfryer"],
        &[
            "An air fryer lasts about five to eight years if it is looked after: washing the basket by hand rather than in the dishwasher and not overfilling it are most of it, and the non-stick goes before the element does. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect an air fryer to last about five to eight years - washing the basket by hand rather than in the dishwasher and not overfilling it are most of it, and the non-stick goes before the element does. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["hob"],
        &[
            "A hob lasts about ten to fifteen years if it is looked after: a ceramic or induction hob usually dies at the touch controls, so keep spills off them. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a hob to last about ten to fifteen years - a ceramic or induction hob usually dies at the touch controls, so keep spills off them. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["hair dryer", "hairdryer"],
        &[
            "A hair dryer lasts about four to six years if it is looked after: cleaning the filter at the back is what stops it overheating. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a hair dryer to last about four to six years - cleaning the filter at the back is what stops it overheating. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["blender"],
        &[
            "A blender lasts about three to five years if it is looked after: not running it past a minute at a time is most of it; the motor is what goes. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a blender to last about three to five years - not running it past a minute at a time is most of it; the motor is what goes. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["sandwich maker", "toastie maker"],
        &[
            "A sandwich maker lasts about five to eight years if it is looked after: wiping the plates and brushing the crumbs out after each use is most of it, and the hinge and the non-stick coating are what go first. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
            "Expect a sandwich maker to last about five to eight years - wiping the plates and brushing the crumbs out after each use is most of it, and the hinge and the non-stick coating are what go first. Repair it while a fix is well under half the price of a new one, and replace it once it needs two repairs in a year.",
        ],
    ),
    Life(
        &["mattress"],
        &[
            "A mattress lasts about eight to ten years if it is looked after: airing it and turning or rotating it every few months is what keeps it there. Replace it once it sags or the springs go, or you wake up stiff.",
            "Expect a mattress to last about eight to ten years - airing it and turning or rotating it every few months is what keeps it there. Replace it once it sags or the springs go, or you wake up stiff.",
        ],
    ),
    Life(
        &["carpet", "carpets"],
        &[
            "A carpet lasts about ten years in a room people walk through every day if it is looked after: the underlay matters as much as the carpet, and a cheap carpet on a good underlay outlasts a good one laid straight onto the floor. Replace it when the pile is worn through on the walkways - a professional clean brings a dirty carpet back once, not a worn one.",
            "Expect a carpet to last about ten years in a room people walk through every day - the underlay matters as much as the carpet, and a cheap carpet on a good underlay outlasts a good one laid straight onto the floor. Replace it when the pile is worn through on the walkways - a professional clean brings a dirty carpet back once, not a worn one.",
        ],
    ),
    Life(
        &["sofa", "sofas", "couch", "settee", "armchair", "suite"],
        &[
            "A sofa lasts about seven to fifteen years if it is looked after: the frame decides it - a hardwood frame outlasts chipboard - and the cushions and covers can be replaced long before the frame goes. Re-cover it or have the cushions refilled before you replace it: that is a fraction of the price of a new one.",
            "Expect a sofa to last about seven to fifteen years - the frame decides it - a hardwood frame outlasts chipboard - and the cushions and covers can be replaced long before the frame goes. Re-cover it or have the cushions refilled before you replace it: that is a fraction of the price of a new one.",
        ],
    ),
    Life(
        &["frying pan", "saucepan", "pans", "wok", "cookware"],
        &[
            "A pan lasts about five to ten years if it is non-stick, and a lifetime if it is stainless steel or cast iron if it is looked after: the coating is what goes first, and metal utensils and the dishwasher hurry it along. Replace a non-stick one once the coating is scratched or flaking, and keep a stainless or cast iron one for good - it cleans back to itself.",
            "Expect a pan to last about five to ten years if it is non-stick, and a lifetime if it is stainless steel or cast iron - the coating is what goes first, and metal utensils and the dishwasher hurry it along. Replace a non-stick one once the coating is scratched or flaking, and keep a stainless or cast iron one for good - it cleans back to itself.",
        ],
    ),
    Life(
        &["hot water bottle", "hot water bottles"],
        &[
            "A hot water bottle lasts about two to three years if it is looked after: the rubber perishes from the inside whether or not it is used, so go by the date on the neck rather than by how it looks. Replace it at the first crack, bulge or stiff patch, and never fill one that has sat in a cupboard for years.",
            "Expect a hot water bottle to last about two to three years - the rubber perishes from the inside whether or not it is used, so go by the date on the neck rather than by how it looks. Replace it at the first crack, bulge or stiff patch, and never fill one that has sat in a cupboard for years.",
        ],
    ),
    Life(
        &["bed frame", "bedframe", "bedstead"],
        &[
            "A bed frame lasts about ten to fifteen years if it is looked after: a wooden frame that is glued as well as screwed, and joints tightened once a year, is what stops it creaking and wobbling before then. Tighten the joints once a year and put a felt pad under a leg that wobbles rather than waiting for the frame to work itself loose.",
            "Expect a bed frame to last about ten to fifteen years - a wooden frame that is glued as well as screwed, and joints tightened once a year, is what stops it creaking and wobbling before then. Tighten the joints once a year and put a felt pad under a leg that wobbles rather than waiting for the frame to work itself loose.",
        ],
    ),
];
pub(super) struct Running(
    pub &'static [&'static str],
    pub f64,
    pub &'static str,
    pub &'static str,
    pub Option<&'static str>,
);
pub(super) const RUNNING: &[Running] = &[
    Running(
        &["fan", "desk fan", "tower fan", "standing fan"],
        2.0,
        "a fan",
        "a penny or two an hour, and it cools you rather than the room",
        Some(
            "A pedestal or tower fan is £25 to £80, and a good one moves the same air as a cheap one for half the noise. A portable air conditioner, which actually cools, is £250 to £500.",
        ),
    ),
    Running(
        &["air fryer", "airfryer"],
        18.0,
        "an air fryer",
        "about 15 to 20p an hour, and it is hot in three minutes",
        Some(
            "An air fryer is £40 to £120: a 4 to 5 litre one is £70 to £90 and is the size most people use, and a two-drawer one is £120 to £200.",
        ),
    ),
    Running(
        &["oven", "fan oven", "electric oven", "double oven"],
        50.0,
        "an electric oven",
        "about 50p an hour at 180C, and much of that is the warm-up",
        Some(
            "A built-in electric oven is £300 to £600, and a range cooker £700 to £2,000; fitting is usually £60 to £120 on top.",
        ),
    ),
    Running(
        &["gas oven", "gas cooker"],
        22.0,
        "a gas oven",
        "about 20 to 25p an hour at 180C, roughly half what an electric oven of the same size costs",
        None,
    ),
    Running(
        &["gas hob", "gas ring", "gas burner"],
        12.0,
        "a gas hob",
        "roughly 10 to 15p an hour of use, the cheapest of the hobs",
        None,
    ),
    Running(
        &[
            "electric hob",
            "electric ring",
            "ceramic hob",
            "halogen hob",
        ],
        30.0,
        "an electric hob",
        "about 25 to 35p an hour, and more if the pan is smaller than the ring",
        None,
    ),
    Running(
        &["induction hob", "induction cooker"],
        16.0,
        "an induction hob",
        "about 15 to 20p an hour, close to gas and about twice as quick to boil",
        None,
    ),
    Running(
        &["microwave", "microwave oven"],
        5.0,
        "a microwave",
        "pennies for a reheated plate, because it runs for minutes not hours",
        Some(
            "A freestanding microwave is £60 to £150, and £150 to £250 buys a combi or a flatbed one; a built-in model is £250 upwards plus fitting.",
        ),
    ),
    Running(
        &["slow cooker", "crockpot"],
        2.0,
        "a slow cooker",
        "a couple of pence an hour, so around 15p for a whole afternoon",
        Some(
            "A slow cooker is £30 to £70, and a 3.5 to 6 litre one with a sear setting covers most households.",
        ),
    ),
    Running(
        &["dishwasher"],
        22.0,
        "a dishwasher",
        "about 20 to 25p a cycle on eco, water and all",
        Some(
            "A freestanding dishwasher is £300 to £500, and £450 to £700 for a quieter one with a cutlery tray; integrated models cost about the same plus a door panel.",
        ),
    ),
    Running(
        &[
            "washing up by hand",
            "wash by hand",
            "washing by hand",
            "washing up",
            "by hand",
        ],
        8.0,
        "washing up by hand",
        "roughly 5 to 10p of gas to heat a bowl, and 20p or more if the hot water is electric",
        None,
    ),
    Running(
        &[
            "tumble dryer",
            "tumbler",
            "heat pump dryer",
            "heat pump drier",
            "heat pump tumble dryer",
            "vented dryer",
            "condenser dryer",
        ],
        30.0,
        "a tumble dryer",
        "about 25p a cycle with a heat pump and more like 60p with a vented one",
        Some(
            "A vented tumble dryer is £250 to £400, a condenser £300 to £500, and a heat pump £350 to £600 - the dearer one pays the difference back in about two years of running costs.",
        ),
    ),
    Running(
        &["dehumidifier"],
        7.0,
        "a dehumidifier",
        "about 7p an hour, and it dries a room by running longer rather than hotter",
        Some(
            "A 12 to 20 litre dehumidifier is £150 to £250, and a desiccant one for a cold garage or a flat that never warms up is £200 to £300.",
        ),
    ),
    Running(
        &[
            "electric heater",
            "fan heater",
            "convector heater",
            "oil heater",
            "oil filled radiator",
            "space heater",
        ],
        48.0,
        "an electric heater",
        "about 45 to 50p an hour whatever the type, because they all turn electricity into heat the same way",
        None,
    ),
    Running(
        &["kettle"],
        3.0,
        "a kettle",
        "about 2 to 3p a boil, so boil only what you are going to drink",
        Some(
            "A kettle is £25 to £60; a 3kW one boils about twice as fast as a 1.7kW one, which matters more than the shape of the spout.",
        ),
    ),
    Running(
        &["washing machine"],
        25.0,
        "a washing machine",
        "about 25p a cycle at 40C and nearer 15p at 30C",
        Some(
            "A new basic washing machine is about £300 to £400, and £450 to £600 buys a quieter one with a bigger drum and a longer guarantee. Refurbished from a dealer is £180 to £250, and a private sale £80 to £150 with no guarantee.",
        ),
    ),
    Running(
        &["bath", "baths", "bathtub", "hot bath"],
        30.0,
        "a hot bath",
        "about 25 to 30p of gas for 80 litres, and nearer 60 to 80p if the water comes from an electric immersion",
        None,
    ),
    Running(
        &["shower", "showers", "showering"],
        15.0,
        "a shower",
        "roughly 10 to 15p for five minutes on a gas boiler, and 15 to 20p on an electric shower, which heats its own water as it goes",
        None,
    ),
    Running(
        &[
            "central heating",
            "the heating",
            "heating on",
            "gas heating",
            "radiator",
            "radiators",
            "boiler",
            "combi",
        ],
        60.0,
        "gas central heating",
        "about 60p an hour while the boiler is actually firing, so five hours a day is £2 to £3, and a heat pump or electric heating is cheaper per unit of heat only if it is a real heat pump",
        None,
    ),
    Running(
        &["electric blanket", "heated blanket"],
        2.0,
        "an electric blanket",
        "a penny or two for an evening, and far cheaper than heating the room",
        Some(
            "An electric blanket is £40 to £80 for a single and £60 to £120 for a double, and one with separate controls and a timer is worth the extra.",
        ),
    ),
    Running(
        &["vacuum cleaner", "hoover", "vacuum"],
        30.0,
        "a vacuum cleaner",
        "about 25 to 50p an hour, though you rarely run one for an hour",
        Some(
            "A corded vacuum is £100 to £250, and a cordless one £150 to £400 with the battery being most of the price - check what a replacement battery costs before you buy.",
        ),
    ),
    Running(
        &["air conditioning", "air con", "aircon", "air conditioner"],
        40.0,
        "a portable air conditioner",
        "30 to 50p an hour, and a fitted one more than that",
        None,
    ),
    Running(
        &["fridge freezer", "fridge", "freezer"],
        1.0,
        "a fridge freezer",
        "about 25p a day, which works out near a penny an hour because it only ticks over",
        Some(
            "A fridge freezer is £400 to £700 for a frost-free 50/50 model, and £700 to £1,200 for an American-style one; measure the gap and the door swing first.",
        ),
    ),
];
