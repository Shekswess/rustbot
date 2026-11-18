//! Authored predicates and replies from PyBot 7d63c7f.

pub(super) const NO_BOARD: &str = r#"(?i)\bwithout\s+(?:an?\s+|the\s+|my\s+|any\s+)?(?:ironing\s+)?board\b|\bno\s+(?:ironing\s+)?board\b|\b(?:dont|don't|do not)\s+have\s+an?\s+board\b"#;

pub(super) const NO_IRON: &str = r#"(?i)\bwithout\s+(?:an?\s+|the\s+|my\s+|any\s+)?iron\b|\bno\s+iron\b|\b(?:dont|don't|do not)\s+have\s+an?\s+iron\b|\b(?:havent|haven't|have not)\s+got\s+an?\s+iron\b|\biron\s+(?:is\s+)?(?:broke|broken|packed|in storage)\b"#;

pub(super) const NEEDS_SIXTY: &str = r#"(?i)\b(?:bed ?sheets?|sheets?|bedding|duvet covers?|pillow ?cases?|towels?|flannels?|underwear|pants|knickers|boxers|socks?|napp(?:y|ies)|dish ?cloths?|tea towels?)\b"#;

pub(super) const A_FAULTY_MACHINE: &[&str] = &[
    "smell", "smells", "smelly", "stink", "musty", "damp", "mildew", "mould", "mold", "leak",
    "leaks", "leaking", "leaked", "drain", "drains", "draining", "spin", "spins", "spinning",
    "error", "broken", "broke", "wont", "won't", "stopped", "noise", "noisy", "banging", "rattle",
    "rattling", "flood", "flooded", "stuck", "blocked", "smoking", "tripping", "fuse", "repair",
];

pub(super) const COAT_FAULT: &[&str] = &[
    "leak",
    "leaks",
    "leaking",
    "leaky",
    "wet through",
    "soaked",
    "soaking",
    "water gets in",
    "gets wet",
    "let water",
    "letting water",
    "no longer waterproof",
    "not waterproof",
    "damp inside",
    "lets the rain in",
    "lets water in",
    "keeps me dry",
    "wont keep the rain out",
    "beads",
    "beading",
    "reproof",
    "waterproofing",
    "waterproof my",
    "waterproof it",
    "re waterproof",
    "seams",
];

pub(super) const COAT_BUYING: &[&str] = &[
    "worth",
    "money",
    "buy",
    "buying",
    "price",
    "spend",
    "cheap",
    "expensive",
    "cost",
    "which",
    "best",
    "recommend",
    "choose",
    "new",
];

pub(super) const NO_BOARD_TEXT: &str = "A folded bath towel on a table is the board: lay the shirt flat on it, damp, and press with the iron, working in small sections and keeping the towel under the whole of the front. A clean cotton sheet over the table works too, and the collar and cuffs want pressing on the reverse first so they keep their shape.";

pub(super) const NO_IRON_TEXT: &str = "No iron needed: hang the shirt in the bathroom while you take a hot shower, or spritz it with water, smooth it flat with your hands and hang it somewhere warm. Ten minutes in a tumble dryer with a damp towel drops the creases too, and a damp collar pressed flat with your thumb and left to dry hangs right.";

pub(super) const DRY_FLAT_WORDS: &[&str] = &[
    "wool",
    "woollen",
    "woolen",
    "jumper",
    "jumpers",
    "cardigan",
    "cardigans",
    "knitwear",
    "knitted",
    "knit",
    "merino",
    "cashmere",
    "lambswool",
    "sweater",
    "sweaters",
];

pub(super) const DRY_HANGER_WORDS: &[&str] = &[
    "coat",
    "coats",
    "jacket",
    "jackets",
    "waterproof",
    "waterproofs",
    "gore tex",
    "goretex",
    "raincoat",
    "raincoats",
    "parka",
    "ski jacket",
    "fleece",
    "fleeces",
    "softshell",
    "shell",
];

pub(super) const DRY_FLUFFY_WORDS: &[&str] = &[
    "towel",
    "towels",
    "bedding",
    "duvet",
    "duvets",
    "sheets",
    "sheet",
    "pillowcase",
    "pillowcases",
    "blanket",
    "blankets",
    "bathmat",
    "bath mat",
];

pub(super) const DRY_DOWN_WORDS: &[&str] = &[
    "puffer",
    "puffa",
    "gilet",
    "gilets",
    "down jacket",
    "down coat",
    "down gilet",
    "down sleeping bag",
    "down bag",
    "feather jacket",
];

pub(super) const A_DRYER: &[&str] = &[
    "tumble dryer",
    "tumble drier",
    "washer dryer",
    "washer drier",
    "washer-dryer",
    "dryer",
    "drier",
    "tumbler",
];

pub(super) const LAUNDRY_WORDS: &[&str] = &[
    "washing", "laundry", "clothes", "clothing", "shirt", "shirts", "t shirt", "tshirt", "jeans",
    "towel", "towels", "sheet", "sheets", "bedding", "duvet", "load", "socks", "jumper", "jumpers",
    "trousers", "uniform", "nappy", "nappies", "workwear", "gym kit", "scrubs",
];

pub(super) const BUILDING_WORDS: &[&str] = &[
    "room",
    "rooms",
    "house",
    "home",
    "flat",
    "wall",
    "walls",
    "ceiling",
    "ceilings",
    "floor",
    "floors",
    "building",
    "bedroom",
    "kitchen",
    "bathroom",
    "cellar",
    "basement",
    "hallway",
    "landing",
    "conservatory",
    "garage",
    "soil",
    "garden",
    "gardens",
    "compost",
    "plant",
    "plants",
    "raise bed",
    "raised bed",
    "raised beds",
    "flower bed",
    "flower beds",
    "beds",
    "border",
    "borders",
    "lawn",
    "allotment",
    "greenhouse",
    "seed tray",
    "seed trays",
    "veg patch",
    "vegetable patch",
    "pot",
    "pots",
    "planter",
    "planters",
    "hanging basket",
    "hanging baskets",
    "window box",
    "window boxes",
];

pub(super) const A_JOB_LEFT_UNDONE: &[&str] = &[
    "two goes",
    "two cycles",
    "two washes",
    "two loads",
    "three goes",
    "three cycles",
    "three washes",
    "took two",
    "takes two",
    "went through twice",
    "put it through twice",
    "still wet",
    "still damp",
    "leaves everything damp",
    "leaves the clothes damp",
    "leaves clothes damp",
    "leaves them damp",
    "leave everything damp",
    "come out damp",
    "comes out damp",
    "come out wet",
    "comes out wet",
    "doesnt dry",
    "doesn't dry",
    "isnt drying",
    "isn't drying",
    "not drying",
    "never dries",
    "doesnt finish",
    "doesn't finish",
    "not finishing",
    "doesnt cook",
    "doesn't cook",
    "undercooked",
    "comes out raw",
    "not hot enough",
    "takes ages",
    "taking ages",
    "takes forever",
    "taking forever",
    "taking hours",
    "takes hours",
    "taking so long",
    "takes so long",
    "taking all day",
    "takes all day",
    "long to dry",
    "slow to dry",
    "drying slowly",
    "dries slowly",
    "hours to dry",
    "all night to dry",
];

pub(super) const DRYER_SLOW_WORDS: &[&str] = &[
    "again", "ages", "ages.", "cycle", "cycles", "damp", "damp.", "forever", "go", "goes", "hours",
    "long", "longer", "slow", "slower", "slowly", "slowly.", "twice", "wet", "wet.",
];

pub(super) const WASH_HIGH: &str = "90 is only for illness, nappies and bedding that has been through a bug: it shrinks cotton and fades colour, so nothing you want to keep should go in it. 60 does towels, bedding, underwear, socks and anything greasy or stained, and 30 or 40 covers shirts and jeans worn once.";

pub(super) const WASH_SIXTY: &str = "60 is not too hot for towels, bedding, underwear, socks and anything greasy or stained - it is what those need, along with a full dose of detergent. Keep shirts and jeans worn once at 30 or 40, and save 90 for illness and nappies: heat is what shrinks and fades things.";

pub(super) const WASH_BEDDING: &str = "60C for bedding, towels, underwear and socks: that is the heat that kills dust mites and the bacteria that make a towel smell, along with a full dose of detergent, and cotton bedding takes it well. Keep shirts, jeans and anything worn once at 30 or 40, and save 90 for illness and nappies.";

pub(super) const WASH_NORMAL: &str = "Treat 30C as the everyday setting: it is about 10p a load cheaper than 40C and gets shirts, jeans and normally-worn clothes just as clean, as long as the drum is not crammed and you use the full dose of detergent. Save 40 for heavy soiling, 60 for towels, bedding, underwear, socks and anything greasy or stained, and 90 only for illness or nappies.";

pub(super) const DRY_DOWN: &str = "Tumble a down jacket on low with two or three dryer balls and stop every twenty minutes to pull the clumps apart, until it is completely dry - hanging it wet leaves the down in lumps that never loft again. If it is dirty, wash it first on its own with a down wash and no fabric conditioner.";

pub(super) const DRY_HANGER: &str = "Hang it on a wide hanger somewhere airy with the pockets empty and the hood out, and let it drip dry. Keep a waterproof away from a radiator and a tumble dryer, because the heat softens the coating that keeps the rain out.";

pub(super) const DRY_FLAT: &str = "Dry wool flat, never on a hanger: press the water out in a towel, reshape it on a dry towel or a mesh rack and leave it somewhere airy, away from a radiator. Hanging it wet is what stretches a jumper out of shape.";

pub(super) const DRY_FLUFFY: &str = "Shake it out, then hang it with space on both sides - a towel or a duvet bunched up stays damp in the middle. Finish it in a tumble dryer on low if you have one, and do not fold it away until it is dry to the touch, or it comes out musty.";

pub(super) const DRY_INSIDE: &str = "Hang the washing on a rack in the warmest room with a window cracked open, spaced out so air moves round it, and turn thicker things after a few hours. A dehumidifier dries a full load overnight indoors and stops the damp smell.";

pub(super) const DRYER_SLOW: &str = "Slow drying is nearly always the air: empty the fluff filter, rinse the condenser if it has one, and check the vent hose is not kinked. A drum stuffed full will not dry either, so try half a load. If the air still comes out cold, it is the fuse on the element - an engineer's job, cheap part.";

pub(super) const DRYER_COLD: &str = "Nearly always the air or the setting, not a part: check the dial first, since most machines finish on a cool-down phase that heats nothing, then empty the fluff filter and check the vent hose is not kinked. If the drum turns and the air still comes out cold, it is the thermal fuse on the element - an engineer's job, cheap part.";
