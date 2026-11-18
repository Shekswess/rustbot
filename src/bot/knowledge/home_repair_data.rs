//! Source terms and reply pools for repair decisions.

pub(super) const DECIDING_NOW: &str = r#"\b(worth|repair|repairing|repairs|replace|replacing|mend|mending|fix|fixing|scrap|scrapping|buy new|buying new|new one|or new|engineer|engineers)\b"#;
pub(super) const CHOOSING: &str = r#"\bor\s+(?:repair|repair|fix|fixed|mend|keep)\b|\b(?:repair|repairing|fix|fixed|fixing|mend|mending|keep|keeping)\b[^.]{0,40}?\bor\b"#;
pub(super) const NEED_A_NEW: &str =
    r#"\b(?:do|will|would|should|could|might)\s+i\s+need\s+(?:a|an|another)\s+new\b"#;
pub(super) const SELF_JOB_NOW: &str = r#"\b(?:myself|my self|diy|diy job|on my own|my own|can i fix|could i fix|can i repair|could i repair|can i replace|can i change|could i change|do it myself|have a go|give it a go|give it a try)\b"#;
pub(super) const CALLING_SOMEONE: &str = r#"\b(?:someone in|somebody in|a man in|someone out|engineer|plumber|repair ?man|call ?out|call someone|who (?:do|should) i (?:call|ring)|get it looked at|looked at|worth getting someone|second opinion|quote)\b"#;
pub(super) const SHOPPING_NOW: &str = r#"\b(worth (?:buying|getting) (?:a|an|the|one|another|new)|should i (?:buy|get) (?:a|an|the|one|another|new)|which \w+ (?:should i|to) buy|what \w+ should i buy|is it worth a new)\b"#;
pub(super) const FAULT_NOW: &str = r#"\b(?:broken|broke|bust|faulty|fault|repair|repairing|fix|fixing|mend|mending|fixes|fixed|replace|replacing|replacement|scrap|scrapping|leak|leaks|leaking|noisy|noise|banging|stopped|stopping|stopped working|not working|gone wrong|on the blink|playing up|packed in|packed up|conked out|given up the ghost|had it|dead|wont|won't|isnt|isn't|error|smell|smells|engineer|part|parts|motor|drum|pump|element|seal|not heating)\b"#;
pub(super) const OWNED_NOW: &str =
    r#"\b(?:my|our|me and|we've got|we have|weve got|i have|i've got|ive got)\b"#;
pub(super) const WORTH_NOW: &str =
    r#"\b(?:worth|scrap|scrapping|new one|buy a new|buying new|or new|money)\b"#;
pub(super) const KEEPING_NOW: &str = r#"\b(keep|keeps|keeping|kept)\b"#;
pub(super) const YEARS_OLD: &str = r#"\b(\d{1,2})\s*(?:year|yr)s?[ -]?old\b"#;
pub(super) const OTHER_FAULT: &str = r#"\bbroke\b|\bbroken\b|\bbreak\b|\bbreaks\b|\bbreaking\b|\bbreaks\ down\b|\bbroke\ down\b|\bstopped\ working\b|\bpacked\ up\b|\bpacked\ in\b|\bconked\ out\b|\bbreaking\ down\b|\bkeep\ breaking\b|\bkeeps\ breaking\b|\bkeeps\ going\ wrong\b|\bgoes\ wrong\b|\bgoing\ wrong\b|\bdied\b|\bdead\b|\bgiven\ up\b|\bon\ the\ blink\b|\bblinking\b|\bknackered\b|\bacting\ up\b|\bplaying\ up\b|\bon\ its\ way\ out\b|\bgone\ wrong\b|\bwont\ turn\ on\b|\bwon\ t\ turn\ on\b|\bwont\ switch\ on\b|\bwon\ t\ switch\ on\b|\bwont\ start\b|\bwon\ t\ start\b|\bwont\ come\ on\b|\bwon\ t\ come\ on\b|\bisnt\ working\b|\bisn\ t\ working\b|\bnot\ working\b|\bfaulty\b|\bkaput\b|\bstopped\b|\bno\ power\b|\bjust\ stopped\b|\bfix\b|\bfixing\b|\bfixed\b|\bhow\ do\ i\ fix\b|\bhow\ to\ fix\b|\bcan\ i\ fix\b|\bhow\ do\ i\ mend\b|\bmend\b|\bis\ rubbish\b|\bare\ rubbish\b|\bis\ useless\b|\bare\ useless\b|\bis\ crap\b|\bis\ a\ dud\b|\bis\ junk\b|\bis\ a\ piece\ of\ junk\b|\bpiece\ of\ junk\b|\bis\ had\ it\b|\bis\ on\ its\ way\ out\b|\bon\ its\ way\ out\b|\brepair\b|\brepairs\b|\brepairing\b"#;
pub(super) const COLD_STOP: &str = r#"\b(food|foods|still safe|safe to eat|safe to keep|thawed|thawing|went off|gone off|keep|keeping|throw out|threw out|temperature|stopped working|has stopped|not working|not cold|cold enough|seal|smell|door left open|off overnight)\b"#;
pub(super) const NOT_A_MACHINE: &[&str] = &[
    "starter",
    "starters",
    "sourdough",
    "levain",
    "dough",
    "doughs",
    "batter",
    "sponge",
    "yeast",
    "bread",
    "breads",
    "loaf",
    "loaves",
    "roll",
    "rolls",
    "boule",
    "batard",
    "focaccia",
    "brioche",
    "croissant",
    "pastry",
    "cake",
    "cakes",
    "bun",
    "buns",
    "proving",
    "proofing",
    "proved",
    "rise",
    "risen",
    "rising",
    "knead",
    "kneaded",
    "ferment",
    "fermented",
    "fermenting",
    "bubbling",
    "bubbles",
];
pub(super) const DIY_EARLIER: &[&str] = &[
    "radiator",
    "radiators",
    "boiler",
    "boilers",
    "thermostat",
    "bike",
    "bicycle",
    "chain",
    "car",
    "van",
    "motorbike",
    "phone",
    "laptop",
    "watch",
    "tv",
    "glasses",
    "specs",
    "spectacles",
    "sunglasses",
    "optician",
    "frames",
    "lose weight",
    "losing weight",
    "kilo",
    "kilos",
    "diet",
    "stones",
    "pounds",
    "hurt",
    "hurts",
    "hurting",
    "injur",
    "pain",
    "sore",
    "ache",
    "aches",
    "sprain",
    "my back",
    "my knee",
    "my shoulder",
    "my ankle",
    "my wrist",
    "glasses",
    "spectacles",
    "specs",
    "sunglasses",
    "lenses",
    "frame",
];
pub(super) const WHITE_GOODS: &[(&str, &str)] = &[
    ("washing machine", "washing machine"),
    ("washer", "washing machine"),
    ("tumble dryer", "tumble dryer"),
    ("dishwasher", "dishwasher"),
    ("fridge", "fridge"),
    ("freezer", "freezer"),
    ("oven", "oven"),
    ("cooker", "cooker"),
    ("hob", "hob"),
    ("microwave", "microwave"),
    ("boiler", "boiler"),
    ("dryer", "tumble dryer"),
    ("vacuum", "vacuum cleaner"),
    ("hoover", "vacuum cleaner"),
];
pub(super) const SMALL_APPLIANCES: &[(&str, &str)] = &[
    ("sandwich maker", "sandwich maker"),
    ("hair dryer", "hair dryer"),
    ("hairdryer", "hair dryer"),
    ("toaster", "toaster"),
    ("kettle", "kettle"),
    ("blender", "blender"),
];
pub(super) const GARDEN_MACHINES: &[(&str, &str)] = &[
    ("hedge trimmer", "hedge trimmer"),
    ("leaf blower", "leaf blower"),
    ("lawnmower", "mower"),
    ("mower", "mower"),
    ("strimmer", "strimmer"),
    ("chainsaw", "chainsaw"),
];
pub(super) const SMALL: &[&str] = &[
    "{Article} {machine} is not worth repairing: a new one costs less than the call-out, so replace it. Buy the next one from a shop with a two-year guarantee and keep the receipt, because that is the only cover a cheap appliance is worth having.",
    "Replace the {machine} rather than repair it: the call-out alone usually costs more than a new one, and the parts are not made to be swapped. A two-year guarantee from the shop is worth more than any repair here.",
];
pub(super) const GARDEN: &[&str] = &[
    "{Article} {machine} is worth a service if the engine is sound: a blade, oil and a spark plug cost a fraction of a new one, and a service is the whole of the repair most of them ever need. If the engine is seized or the body has rusted through, replace it instead.",
    "Service the {machine} if it starts and runs: a new blade, oil and a plug is a small fraction of the price of a new one. If it will not start at all, or the body has rusted through, put the money towards a new one.",
];
pub(super) const REPAIR: &[&str] = &[
    "{Article} {machine} is worth repairing while the quote is under about half the price of a new one and it is under six or seven years old: a seal, a pump, a belt or a heating element is usually money well spent, while a failed drum bearing or control board on an old one is not. Ask how long the parts are guaranteed before you agree, and take a quote near half the price of new as the signal to replace it: an engineer who wants £80 before touching it is already half a new machine.",
    "Repair the {machine} if it is under about six years old and the quote is less than half a new one; replace it if the repair costs more than that or it is past eight years. The wear and tear repairs (seals, belts, pumps, elements) are worth doing on a good machine, and a control board or a drum on an old one is not. Get the quote in writing with a guarantee on the parts, because that is what separates a repair from throwing money at it.",
    "The rule of thumb is half and six: repair the {machine} while the quote is under half the price of a new one and it is under six years old, and replace it after eight years or when the repair passes that half. A seal, a pump or a heating element on a good machine is money well spent; a control board or a drum bearing on an old one is not. Ask how long the parts are guaranteed before you say yes.",
];
pub(super) const OLDER: &[&str] = &[
    "At {age} the {machine} is at the end of its life, so replace it rather than repair it: a quote for a seal or a pump on a machine that age is money towards a new one, and the next fault is already on its way.",
    "Replace the {machine}: at {age} old the repair is throwing money after the next breakdown. A new one comes with a guarantee, and the running costs of a modern {machine} are lower too.",
];
pub(super) const AGES: &[&str] = &[
    "a year",
    "two years",
    "three years",
    "four years",
    "five years",
    "six years",
    "seven years",
    "eight years",
    "nine years",
    "ten years",
    "eleven years",
    "twelve years",
    "thirteen years",
    "fourteen years",
    "fifteen years",
    "sixteen years",
    "seventeen years",
    "eighteen years",
    "nineteen years",
    "twenty years",
];

pub(super) const DIY: &str = "A tap washer, blocked filter or an external hose can be a small DIY job if the manual covers it. Check the warranty first, turn off the water for plumbing work and unplug the appliance before following its owner instructions. Leave gas work, fixed wiring and repairs that require opening the appliance to a qualified tradesperson.";
pub(super) const DIY_UNKNOWN: &str = "Check the manual for the parts it says an owner can clean or swap, such as a filter, a door seal or an inlet hose. Turn off the water and unplug the machine before doing those jobs. Leave work on the motor, drum, wiring or gas components to a qualified repairer.";
pub(super) const BOILER_DIY: &str = "You can check the thermostat, display and pressure gauge as your boiler manual directs. The manual gives the pressure range and tells you whether an owner reset is appropriate. If the boiler keeps locking out, stop using it and arrange a Gas Safe registered engineer. Leave the casing closed.";
pub(super) const MICROWAVE_DIY: &str = "Leave the microwave casing closed. Internal parts can retain dangerous voltage even after unplugging. Check the owner instructions for external checks and ask the retailer or an appliance repairer about repair or replacement.";
pub(super) const BOILER_DECISION: &str = "A gas boiler is Gas Safe work by law: the repair itself is an engineer's job, not a DIY one, so the question is what to spend on it. Repair it while it is under about eight years old and the quote is less than half a new one; past that, or with a failed heat exchanger, put the money towards a new boiler, which will also be cheaper to run.";

pub(super) const DIY_PARTS: &[(&str, &str)] = &[
    (
        "washing machine",
        "check the manual for owner jobs such as cleaning the filter and replacing an accessible hose or seal. Turn off the water and unplug the machine first. Leave the pump, drum, wiring and any repair that needs the case opened to a repairer",
    ),
    (
        "dishwasher",
        "the filter, spray arms, and any seals or hoses the manual lists for owner replacement are jobs to do with the water off and the plug out. Leave the pump and control board to a repairer",
    ),
    (
        "tumble dryer",
        "unplug it before cleaning the filter or replacing a door seal the manual lists as an owner job. Leave the drive belt behind a panel, heating element and thermal fuse to a repairer",
    ),
    (
        "fridge",
        "check the manual for unplugged owner jobs such as cleaning the drain hole or changing a removable door seal. The thermostat, refrigerant circuit and compressor need a repairer",
    ),
    (
        "freezer",
        "check the manual for unplugged owner jobs such as changing a removable door seal. The thermostat, refrigerant circuit and compressor need a repairer",
    ),
    (
        "oven",
        "let it cool and isolate the power before replacing a user-accessible door seal or shelf runners as the manual directs. Leave the heating element, fan motor and wiring to a qualified repairer. Work on gas components requires a Gas Safe registered engineer",
    ),
    (
        "cooker",
        "let it cool and isolate the power before replacing a user-accessible door seal or shelf runners as the manual directs. Leave the heating element and wiring to a qualified repairer. Work on gas components requires a Gas Safe registered engineer",
    ),
    (
        "hob",
        "let it cool and isolate the power before replacing user-accessible knobs or pan supports as the manual directs. Leave the cooking elements and wiring to a qualified repairer. Work on gas components requires a Gas Safe registered engineer",
    ),
    (
        "vacuum cleaner",
        "unplug it before cleaning the filters or a blocked hose, or replacing a belt the manual lists as user-accessible. Leave the motor and wiring to a repairer",
    ),
];

pub(super) const STOPPED_SMALL: &[&str] = &[
    "Check the socket and plug before paying anyone, and follow the manual for any plug-fuse check. If the plug, lead or socket looks damaged or scorched, leave it unplugged. If the {machine} is still dead, ask the retailer about a remedy or guarantee. A paid call-out can cost more than a replacement.",
    "On {article} {machine}, start with the wall switch and the owner's checks in the manual. Leave the casing closed. If it stays dead, check the receipt for any guarantee. Replacing a cheap appliance may cost less than paying for a repair.",
];
pub(super) const STOPPED: &[&str] = &[
    "Check the wall switch, whether the door shuts properly and any filter the manual says you can check. Most {machine} have a troubleshooting section. If the appliance has failed, ask the retailer about your options. The six-year claim limit in England and Wales, or five years in Scotland, only applies where your statutory rights were breached. A manufacturer's guarantee is separate. Before paying for work, compare a written quote and parts guarantee with the price of a replacement.",
    "Start with the wall switch and the manual's checks for a closed door or blocked filter. The manual will say which checks apply to {machine}. If it stays dead, contact the retailer with your receipt. You may be able to claim for faulty goods for up to six years in England and Wales or five in Scotland. The retailer can ask for proof the fault existed at purchase if you claim after six months. Ask for a written quote and a parts guarantee if you decide to pay for repair.",
];
