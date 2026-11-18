// Food allowances and portion patterns from the pinned bot/knowledge/units.py.
pattern!(
    CONTAINER,
    Regex,
    r#"\b(?:bags?|packs?|packets?|tins?|cans?|jars?|boxes?|cartons?)\b"#
);
pattern!(
    PEOPLE_WORD,
    Lookaround,
    r#"\b(?:per\s+(?:person|head)|an?\s+person|an?\s+head(?!\s+of\b)|people|persons?|of\s+us|of\s+you|guests|adults?|between\s+us|party|table\s+for|we(?:'re|\s+are)\s+\d{1,3}\b|there\s+are\s+\d{1,3}\b|expecting\s+\d{1,3}\b|each\s+(?:person|head|guest|adult|of\s+us|one)|family\s+of)\b|\beach\b"#
);
pattern!(
    PEOPLE_COUNT,
    Lookaround,
    r#"\b(?:for|between)\s+(\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\s+(?:people|persons?|guests|adults|of\s+us|of\s+you|heads)\b|\b(\d{1,2})\s+(?:people|persons?|guests|adults|heads)\b|\bfamily\s+of\s+(\d|one|two|three|four|five|six|seven|eight|nine|ten)\b|\bfor\s+(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\b(?!\s*(?:minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|slices?|meals?|dinners?|lunches?|kg|kilos?|g|grams?|litres?|liters?|ml|pints?|cans?|tins?|packs?|batches?|portions?|servings?|kms?|kilometres?|kilometers?|k|miles?|metres?|meters?|m|ft|feet|foot|yards?|steps?|laps?|lengths?|rounds?|times?|degrees?))|\b(?:we(?:'re|\s+are)|there(?:'s|\s+are))\s+(\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b(?!\s*(?:minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|slices?|meals?|dinners?|lunches?|kg|kilos?|g|grams?|litres?|liters?|ml|pints?|cans?|tins?|packs?|batches?|portions?|servings?|kms?|kilometres?|kilometers?|k|miles?|metres?|meters?|m|ft|feet|foot|yards?|steps?|laps?|lengths?|rounds?|times?|degrees?))|\b(\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\s+of\s+us\b|\b(?:party|table|group|booking|buffet)\s+(?:of|for)\s+(\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b|\b(\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a dozen)\s+coming\b|\b(\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\s+(?:of\s+us\s+)?(?:for|at)\s+(?:dinner|lunch|supper|christmas|the\s+weekend)\b"#
);
pattern!(
    PORTION_AVOID,
    Regex,
    r#"\b(?:calories|kcal|carbs?|protein|sugar|fat|fibre|fiber|cost|costs|price|prices|worth|budget|alcohol|units|feed|feeds|plant|planting|grow|growing|seeds?|leftover|spend|spending|money|pay|paid|shop|shopping|dressing|sauces?|mayo|mayonnaise|stock|dips?|cream\s+sauces?|salad\s+cream|chutney|left\s+over|weigh|weighs|weight|heavy|boards?|cheeseboards?|platters?|powders?|spices?|seasoning|ground|flakes|granules|extracts?|essence|herbs?)\b"#
);
pattern!(
    BAKING_PORTION,
    Regex,
    r#"\b(?:cake|cakes|sponge|cupcakes?|batter|meringues?|pavlova|yorkshire|pancakes?|crepes?|pastry|dough|loaf|loaves|bread\s+and\s+butter)\b"#
);
pattern!(
    PORTION_COUNT,
    Regex,
    r#"\bhow many (?:portions|servings|slices|pieces|bits|helpings)\b"#
);
pattern!(PORTION_WORD, Regex, r#"\bportions?\b|\bservings?\b"#);
pattern!(
    TABLE_GROUPS,
    Regex,
    r#"(?i)\b(\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a dozen)\s+(?:adults?|grown[\s-]?ups?|pensioners?|teenagers?|kids?|children|child|toddlers?|babies|baby|guests?|people|persons?|heads|of\s+us)\b"#
);
pattern!(
    CHILLI_THE_VEGETABLE,
    Regex,
    r#"\bhow (?:much|many) (?:red |green |small |big |hot )?chilli(?:es)?\b"#
);
pattern!(
    A_DISH_PART,
    Regex,
    r#"\s+(?:powder|powders|sauce|sauces|paste|pastes|oil|flakes|seeds|seasoning|spice|spices|mix|puree|ketchup|granules|extract|essence|leaf|leaves|plant|pepper|peppers)\b"#
);
pattern!(
    A_TIME_IS_THE_ASK,
    Regex,
    r#"\bhow long\b|\bhow soon\b|\bwhat time\b|\bwhen\b|\bhow many (?:hours?|minutes?|mins?|days?)\b"#
);
pattern!(
    FEEDS_ASK,
    Regex,
    r#"\benough\b|\bfeed(?:s|ing)?\b|\bserves?\b|\bgo(?:es)?\s+round\b|\bhow\s+far\b|\bdo\s+for\b|\bhow\s+many\s+(?:people|persons?|of\s+us|adults?|guests?)\b"#
);
pattern!(
    COUNT_OF_BIRDS,
    Regex,
    r#"\b(how many|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+(?:whole\s+)?(chickens|ducks)\b"#
);
const PER_PERSON: &[Allowance] = &[
    Allowance {
        keys: &["potato", "potatoes", "new potatoes", "mash"],
        amount: 250.0,
        unit: Unit::Mass,
        noun: "potatoes",
    },
    Allowance {
        keys: &[
            "chicken breast",
            "chicken breasts",
            "chicken thigh",
            "chicken thighs",
            "chicken",
        ],
        amount: 200.0,
        unit: Unit::Mass,
        noun: "chicken",
    },
    Allowance {
        keys: &["mince", "beef mince", "lamb mince"],
        amount: 150.0,
        unit: Unit::Mass,
        noun: "mince",
    },
    Allowance {
        keys: &["steak", "steaks"],
        amount: 200.0,
        unit: Unit::Mass,
        noun: "steak",
    },
    Allowance {
        keys: &["salmon", "salmon fillet", "salmon fillets"],
        amount: 150.0,
        unit: Unit::Mass,
        noun: "salmon",
    },
    Allowance {
        keys: &["fish", "white fish", "cod", "haddock"],
        amount: 150.0,
        unit: Unit::Mass,
        noun: "fish",
    },
    Allowance {
        keys: &["meat", "joint", "roast beef", "beef", "lamb", "pork"],
        amount: 250.0,
        unit: Unit::Mass,
        noun: "raw meat",
    },
    Allowance {
        keys: &["turkey", "turkeys"],
        amount: 500.0,
        unit: Unit::Mass,
        noun: "turkey",
    },
    Allowance {
        keys: &["duck", "ducks"],
        amount: 500.0,
        unit: Unit::Mass,
        noun: "duck",
    },
    Allowance {
        keys: &["goose", "geese"],
        amount: 500.0,
        unit: Unit::Mass,
        noun: "goose",
    },
    Allowance {
        keys: &["ham", "gammon", "gammon joint"],
        amount: 250.0,
        unit: Unit::Mass,
        noun: "ham",
    },
    Allowance {
        keys: &["prawns", "prawn", "king prawns"],
        amount: 150.0,
        unit: Unit::Mass,
        noun: "prawns",
    },
    Allowance {
        keys: &["cheese", "cheddar"],
        amount: 50.0,
        unit: Unit::Mass,
        noun: "cheese",
    },
    Allowance {
        keys: &["sausage", "sausages"],
        amount: 2.0,
        unit: Unit::Count,
        noun: "sausages",
    },
    Allowance {
        keys: &["egg", "eggs"],
        amount: 2.0,
        unit: Unit::Count,
        noun: "eggs",
    },
    Allowance {
        keys: &["salad", "salad leaves", "lettuce", "rocket", "spinach"],
        amount: 50.0,
        unit: Unit::Mass,
        noun: "salad leaves",
    },
    Allowance {
        keys: &["couscous", "bulgur", "bulghur"],
        amount: 60.0,
        unit: Unit::Mass,
        noun: "couscous",
    },
    Allowance {
        keys: &["quinoa"],
        amount: 60.0,
        unit: Unit::Mass,
        noun: "quinoa",
    },
    Allowance {
        keys: &["lentils", "red lentils", "green lentils", "dal", "dhal"],
        amount: 75.0,
        unit: Unit::Mass,
        noun: "dried lentils",
    },
    Allowance {
        keys: &["prawns", "shrimp"],
        amount: 150.0,
        unit: Unit::Mass,
        noun: "prawns",
    },
    Allowance {
        keys: &[
            "wine",
            "red wine",
            "white wine",
            "rose",
            "prosecco",
            "fizz",
            "champagne",
        ],
        amount: 0.5,
        unit: Unit::Bottle,
        noun: "wine",
    },
    Allowance {
        keys: &["chips", "oven chips", "fries", "french fries"],
        amount: 150.0,
        unit: Unit::Mass,
        noun: "chips",
    },
    Allowance {
        keys: &["carrot", "carrots"],
        amount: 1.0,
        unit: Unit::Count,
        noun: "carrot",
    },
    Allowance {
        keys: &[
            "sandwich",
            "sandwiches",
            "finger sandwiches",
            "rounds of sandwiches",
        ],
        amount: 3.0,
        unit: Unit::Count,
        noun: "sandwiches",
    },
    Allowance {
        keys: &["sausage roll", "sausage rolls"],
        amount: 3.0,
        unit: Unit::Count,
        noun: "sausage rolls",
    },
    Allowance {
        keys: &["quiche", "quiches"],
        amount: 150.0,
        unit: Unit::Mass,
        noun: "quiche",
    },
    Allowance {
        keys: &["food", "dish", "casserole", "pasta bake", "curry"],
        amount: 300.0,
        unit: Unit::Mass,
        noun: "the dish",
    },
    Allowance {
        keys: &["cake"],
        amount: 60.0,
        unit: Unit::Mass,
        noun: "cake",
    },
    Allowance {
        keys: &["stuffing"],
        amount: 50.0,
        unit: Unit::Mass,
        noun: "stuffing",
    },
    Allowance {
        keys: &["gravy"],
        amount: 150.0,
        unit: Unit::Volume,
        noun: "gravy",
    },
    Allowance {
        keys: &["pigs in blankets", "pig in blankets"],
        amount: 3.0,
        unit: Unit::Count,
        noun: "pigs in blankets",
    },
    Allowance {
        keys: &["scone", "scones"],
        amount: 2.0,
        unit: Unit::Count,
        noun: "scones",
    },
    Allowance {
        keys: &["hummus", "houmous"],
        amount: 50.0,
        unit: Unit::Mass,
        noun: "hummus",
    },
    Allowance {
        keys: &["porridge oats", "porridge", "oats"],
        amount: 50.0,
        unit: Unit::Mass,
        noun: "porridge oats",
    },
    Allowance {
        keys: &["soup"],
        amount: 300.0,
        unit: Unit::Volume,
        noun: "soup",
    },
    Allowance {
        keys: &["meatballs", "meatball"],
        amount: 4.0,
        unit: Unit::Count,
        noun: "meatballs",
    },
    Allowance {
        keys: &["slices of bread", "slice of bread", "bread"],
        amount: 2.0,
        unit: Unit::Count,
        noun: "slices of bread",
    },
    Allowance {
        keys: &["sprouts", "brussels sprouts", "brussel sprouts"],
        amount: 5.0,
        unit: Unit::Count,
        noun: "sprouts",
    },
    Allowance {
        keys: &["yorkshire puddings", "yorkshire pudding", "yorkshires"],
        amount: 2.0,
        unit: Unit::Count,
        noun: "yorkshire puddings",
    },
    Allowance {
        keys: &["broccoli"],
        amount: 80.0,
        unit: Unit::Mass,
        noun: "broccoli",
    },
    Allowance {
        keys: &["baked beans", "beans"],
        amount: 150.0,
        unit: Unit::Mass,
        noun: "baked beans",
    },
    Allowance {
        keys: &["custard"],
        amount: 100.0,
        unit: Unit::Volume,
        noun: "custard",
    },
    Allowance {
        keys: &["coleslaw"],
        amount: 75.0,
        unit: Unit::Mass,
        noun: "coleslaw",
    },
    Allowance {
        keys: &["pizza", "pizzas"],
        amount: 3.0,
        unit: Unit::Count,
        noun: "slices of pizza",
    },
    Allowance {
        keys: &["burger", "burgers"],
        amount: 2.0,
        unit: Unit::Count,
        noun: "burgers",
    },
    Allowance {
        keys: &["mince pie", "mince pies"],
        amount: 2.0,
        unit: Unit::Count,
        noun: "mince pies",
    },
];
const PER_PERSON_WHEN_COUNTED: &[Allowance] = &[
    Allowance {
        keys: &["rice", "basmati", "long grain rice"],
        amount: 75.0,
        unit: Unit::Mass,
        noun: "dry rice",
    },
    Allowance {
        keys: &[
            "pasta",
            "spaghetti",
            "penne",
            "fusilli",
            "macaroni",
            "tagliatelle",
        ],
        amount: 100.0,
        unit: Unit::Mass,
        noun: "dry pasta",
    },
];
const BIRDS: &[&str] = &[
    "turkey", "turkeys", "chicken", "chickens", "duck", "ducks", "goose", "geese",
];

const MADE_NOT_MEASURED: &[&str] = &["cake", "the dish"];
const BIRD_GRAMS: f64 = 500.0;
const PEOPLE: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve",
];
const TABLE_DISHES: &[Dish] = &[
    Dish {
        keys: &[
            "chilli con carne",
            "chili con carne",
            "chilli",
            "chili",
            "chilli beef",
            "chilli mince",
        ],
        main: &["mince"],
        side: &["rice"],
        tomatoes: true,
        spice: true,
    },
    Dish {
        keys: &["spaghetti bolognese", "bolognaise", "bolognese", "spag bol"],
        main: &["mince"],
        side: &["spaghetti", "pasta"],
        tomatoes: true,
        spice: false,
    },
    Dish {
        keys: &["lasagne", "lasagna"],
        main: &["mince"],
        side: &[],
        tomatoes: true,
        spice: false,
    },
    Dish {
        keys: &["shepherds pie", "shepherd's pie", "cottage pie"],
        main: &["mince"],
        side: &["potato"],
        tomatoes: false,
        spice: false,
    },
    Dish {
        keys: &["tarka dal", "dhal", "dal"],
        main: &["lentils"],
        side: &["rice"],
        tomatoes: false,
        spice: true,
    },
    Dish {
        keys: &[
            "curry",
            "curries",
            "tikka masala",
            "korma",
            "bhuna",
            "jalfrezi",
        ],
        main: &["chicken"],
        side: &["rice"],
        tomatoes: false,
        spice: true,
    },
    Dish {
        keys: &["casserole", "stew"],
        main: &["meat"],
        side: &["potato"],
        tomatoes: false,
        spice: false,
    },
];

const WHOLE_SERVES: &[&str] = &[
    "chicken",
    "chickens",
    "duck",
    "ducks",
    "leg of lamb",
    "lamb leg",
    "leg of mutton",
    "shoulder of lamb",
    "lamb shoulder",
    "gammon joint",
    "ham joint",
    "gammon",
    "whole ham",
    "whole salmon",
    "side of salmon",
];
