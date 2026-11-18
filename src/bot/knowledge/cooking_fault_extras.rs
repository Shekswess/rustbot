//! Dry, tough and food-illness cards from the pinned cooking registry.

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

const BAKE: &[(&str, &str)] = &[
    ("cupcake", "cupcakes"),
    ("muffin", "muffins"),
    ("brownie", "brownies"),
    ("cookie", "cookies"),
    ("biscuit", "biscuits"),
    ("flapjack", "flapjacks"),
    ("scone", "scones"),
    ("sponge", "sponges"),
    ("loaf", "loaves"),
    ("traybake", "traybakes"),
    ("bread", "bread"),
    ("cake", "cakes"),
    ("bun", "buns"),
    ("tart", "tarts"),
    ("pie", "pies"),
    ("pastry", "pastry"),
];

const BAKED_GOODS: &[&str] = &[
    "cake",
    "cupcake",
    "muffin",
    "bun",
    "scone",
    "brownie",
    "blondie",
    "cookie",
    "biscuit",
    "shortbread",
    "flapjack",
    "sponge",
    "loaf",
    "loaves",
    "bun loaf",
    "traybake",
    "cheesecake",
    "madeira",
    "doughnut",
    "donut",
    "fruit cake",
    "victoria sponge",
    "baking",
    "bread",
    "breadcrumbs",
    "waffle",
];
const BAKED_DISH: &[&str] = &[
    "pasta",
    "lasagne",
    "lasagna",
    "moussaka",
    "cannelloni",
    "macaroni",
    "macaroni cheese",
    "fish pie",
    "cottage pie",
    "shepherds pie",
    "shepherd's pie",
    "quiche",
    "frittata",
    "gratin",
];
const STARCH: &[&str] = &[
    "biryani",
    "paella",
    "pilaf",
    "pilau",
    "fried rice",
    "risotto",
    "couscous",
];
const DIPS: &[&str] = &[
    "hummus",
    "houmous",
    "pesto",
    "guacamole",
    "tapenade",
    "dip",
    "dips",
];
const SAUCES: &[&str] = &[
    "bolognese",
    "bolognaise",
    "bol",
    "ragu",
    "curry",
    "chilli",
    "chili",
    "stew",
    "casserole",
    "soup",
    "sauce",
    "dahl",
    "dal",
    "tagine",
    "goulash",
    "stroganoff",
];

const OVERCOOKED_PASTA: &str = "Overcooked pasta cannot be brought back, but it can be used: drain it, toss it in oil so it stops sticking, and fold it into a bake, a frittata or a soup where soft is fine. Next time salt the water properly, take it off a minute before the packet says, and taste a piece rather than trusting the clock, because a big pan of boiling water comes back to the boil slowly and the first minute is not cooking.";
const DRY_BAKED_DISH: &str = "A bake or a pie dries out when it goes in uncovered or stays in too long: stir a splash of milk or a spoon of passata through the sauce, cover it with foil and give it 10 minutes at 180C to come back. Next time keep a little bechamel back to pour over the top, and cover it for the first 20 minutes.";
const DRY_BAKE: &str = "usually a minute or two too long in the oven, or a little too much flour. Take it out as soon as a skewer comes out with a crumb on it, weigh the flour rather than scooping it with a cup, and don't overbeat - and a dry bake is still good with cream, custard or something wet on it.";
const DRY_RICE: &str = "Dry rice means too little water or too long on the heat. If this is freshly cooking rice, add a splash of hot water and steam it on low with the lid on until tender. Next time use one and a half parts water to one of rice, lid on, 10 minutes, then rest it off the heat.";
const DRY_DIP: &str = "Loosen it a spoonful at a time rather than all at once: a splash of cold water and a squeeze of lemon for hummus, or a little more oil for a pesto, blitzed or stirred through until it is soft again. A dull, dry taste is usually not enough salt or acid rather than not enough oil.";
const DRY_SAUCE: &str = "A sauce that has gone dry wants liquid back, not less time in the pan: a little stock, the water the pasta cooked in, a spoon of tinned tomatoes or a splash of milk, then the lid on and five minutes at a bare simmer. A stew, a curry or a chilli tightens as it stands, so loosen it at the end rather than watering it at the start, and keep the heat low enough that it does not catch.";
const DRY_PASTRY: &str = "Dry pastry wants a sprinkle of cold water and a 20 minute rest in the fridge, then roll it between two sheets of baking paper. Next time keep the fat cold and add water a tablespoon at a time, because pastry dries out fast once the flour is in.";
const DRY_MEAT: &str = "Serve it with something wet — gravy, a spoon of yoghurt, a squeeze of lemon and oil. Next time check doneness as it cooks and rest the meat once it is cooked through. Dry meat or fish is often overcooked.";
const WHY_DRY_MEAT: &str = "Dry meat or fish is often overcooked: it went past the point where the juices set, or it was cooked hard and fast without a rest. Next time check doneness as it cooks and rest the meat once it is cooked through. Serve this one with gravy, yoghurt, lemon or oil.";
const DRY_SAFETY: &str = "Dryness alone does not tell you whether food is safe. Cook poultry, pork and minced meat to 70°C or above for 2 minutes at the centre. Refrigerate leftovers within two hours and use them within 48 hours or freeze them. If reheating, do it once until steaming hot throughout.";

const TOUGH_STEW: &str = "A tough stew is an underdone one rather than an overcooked one: another 30 to 45 minutes at a bare simmer with the lid on, and top the liquid up if it is running low. A simmer that is really a boil tightens the meat instead of softening it, so keep it just trembling and test a piece with a fork - it is ready when it gives way without resistance.";
const TOUGH_MEAT_WHY: &str = "It went tough from cooking too fast or from not resting: the muscle fibres contract and squeeze the juice out, and a cheap cut needs longer at a lower heat to break down instead - 2 hours at 160C with liquid in the pan. This piece is best sliced thinly across the grain and served with something wet.";
const TOUGH_MEAT: &str = "Slice it thinly across the grain and serve it with something wet; that rescues more tough meat than anything else. It went tough from cooking too fast or not resting, so next time go slower: a cheap cut wants 2 hours at 160C with liquid in the pan.";
const TOUGH_STARCH: &str = "Tough here is underdone rather than overcooked: give it 5 more minutes in boiling water and test it with the tip of a knife, and cut the pieces the same size so they finish together. A pan that never came back to the boil is the usual reason it is still hard in the middle when the outside is done.";
const TOUGH_BAKE: &str = "mixed too much or a little too long in the oven, or short of fat. Stir only until the flour disappears, fold rather than beat, and weigh the butter rather than eyeballing it, because a sponge wants as much fat as eggs by weight. One that is already tough is best eaten warm with something wet, or torn up for a trifle.";
const TOUGH_PASTRY: &str = "Tough pastry is over-worked or too wet: keep the fat cold, rub it in with your fingertips only until it looks like fine breadcrumbs, add iced water a spoon at a time and bring it together fast. Shortcrust wants about half its weight in fat to flour, and 20 minutes in the fridge before it is rolled; roll it between two sheets of greaseproof so you never stretch it. Pastry that is already baked cannot be rescued, so serve it with something wet.";
const TOUGH_BREAD: &str = "Tough bread is usually over-kneaded or baked too slowly: knead until the dough springs back and stops tearing, prove it somewhere warm until it has doubled, and bake it hot (220C) for the first 10 minutes so it rises before the crust sets. Bread that has already gone leathery comes back in a hot oven for 5 minutes with a splash of water in the tin.";
const TOUGH_EGG: &str = "Rubbery eggs have been cooked too long, and in a microwave that happens in seconds. Beat them in a microwave-safe bowl, cook at medium power in short bursts and stir between each. For eggs that need thorough cooking, cook until fully set. On the hob use low heat and stir slowly.";
const TOUGH_DEFAULT: &str = "Rubbery is overcooked, or a batter that was beaten too much: pancakes and batters want a lumpy mix and a minute less in the pan. It's hard to rescue on the plate, so serve it with something wet and cook it less next time.";

struct Fruit {
    aliases: &'static [&'static str],
    name: &'static str,
    cause: &'static str,
}

const FRUIT: &[Fruit] = &[
    Fruit {
        aliases: &["apple", "apples"],
        name: "apple",
        cause: "a Bramley collapses to a fluff while a Cox or a Braeburn holds its shape, so use Bramleys for a sauce, a crumble or a compote",
    },
    Fruit {
        aliases: &["pear", "pears"],
        name: "pear",
        cause: "a Conference softens where a Comice is for eating raw, and a hard pear wants 10 more minutes",
    },
    Fruit {
        aliases: &["rhubarb"],
        name: "rhubarb",
        cause: "thin young stalks give way in 5 minutes and thick old ones want 15, so pick the thin ones and add the sugar once they have softened",
    },
    Fruit {
        aliases: &["plum", "plums"],
        name: "plum",
        cause: "a ripe plum collapses in 5 minutes and a firm one holds, so halve them and taste at 5",
    },
    Fruit {
        aliases: &["cherry", "cherries"],
        name: "cherry",
        cause: "cherries want 5 to 8 minutes and go from firm to jam quickly, so take them off while they still hold",
    },
    Fruit {
        aliases: &["apricot", "apricots", "peach", "peaches"],
        name: "apricot",
        cause: "a ripe one collapses in 5 minutes and an underripe one stays firm whatever you do, so halve them and poach them gently",
    },
    Fruit {
        aliases: &["berry", "berries"],
        name: "berry",
        cause: "berries want only 2 or 3 minutes and a hard one was underripe to begin with",
    },
    Fruit {
        aliases: &["quince"],
        name: "quince",
        cause: "a quince stays hard for 40 minutes and then turns pink and soft all at once, so give it the time and taste it",
    },
];

fn prefix(words: &[String], terms: &[&str]) -> bool {
    words
        .iter()
        .any(|word| terms.iter().any(|term| word.starts_with(term)))
}

fn exactly(words: &[String], terms: &[&str]) -> bool {
    words.iter().any(|word| terms.contains(&word.as_str()))
}

fn why(words: &[String], joined: &str) -> bool {
    exactly(words, &["why", "howcome"])
        || [
            "what did i do wrong",
            "what went wrong",
            "what am i doing wrong",
        ]
        .iter()
        .any(|s| joined.contains(s))
}

fn bake_answer(words: &[String], adjective: &str, detail: &str) -> Cow<'static, str> {
    let baked = BAKE
        .iter()
        .find(|&&(one, many)| prefix(words, &[one, many]));
    let start = match baked {
        Some(&(one, many)) if one == many => format!("{adjective} {one} is"),
        Some(&(_, many)) => format!("{adjective} {many} are"),
        None => format!("A {} bake is", adjective.to_lowercase()),
    };
    Cow::Owned(format!("{start} {detail}"))
}

fn dry(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words = &reading.line.words;
    let text = &reading.line.clean;
    let has = |terms| prefix(words, terms);
    let exact = |terms| exactly(words, terms);
    if has(&["biscuit", "biscuits", "cookie", "cookies", "shortbread"]) {
        return None;
    }
    let joined = words.join(" ");
    if exact(&[
        "pasta",
        "spaghetti",
        "penne",
        "macaroni",
        "tagliatelle",
        "fusilli",
        "noodles",
        "noodle",
        "rice noodles",
        "udon",
    ]) && exact(&["overcooked", "over-cooked", "over cooked", "mushy", "soggy"])
    {
        return Some(Cow::Borrowed(OVERCOOKED_PASTA));
    }
    let named = has(BAKED_DISH)
        || BAKED_DISH
            .iter()
            .any(|term| term.contains(' ') && joined.contains(term));
    if named
        && exact(&[
            "bake",
            "baked",
            "baking",
            "oven",
            "overcooked",
            "over-cooked",
            "dry",
        ])
    {
        return Some(Cow::Borrowed(DRY_BAKED_DISH));
    }
    if has(BAKED_GOODS) {
        return Some(bake_answer(words, "Dry", DRY_BAKE));
    }
    let rescue = if why(words, &joined) {
        WHY_DRY_MEAT
    } else {
        DRY_MEAT
    };
    Some(Cow::Borrowed(
        if exact(&["rice", "ricecooker"]) || has(STARCH) {
            DRY_RICE
        } else if exact(DIPS) {
            DRY_DIP
        } else if has(SAUCES) {
            DRY_SAUCE
        } else if has(&["pastry"]) {
            DRY_PASTRY
        } else if text.contains("safe to eat")
            || text.contains("ok to eat")
            || exact(&["safe", "safely", "food-poisoning", "ill"])
        {
            return Some(Cow::Owned(format!("{DRY_SAFETY} {rescue}")));
        } else {
            rescue
        },
    ))
}

fn tough(words: &[String]) -> Cow<'static, str> {
    let has = |terms| prefix(words, terms);
    if has(&["squid"]) {
        return Cow::Borrowed(
            "Squid cooks quickly in a very hot pan or slowly in a stew with tomatoes and wine. Check that it is opaque and fully cooked before serving.",
        );
    }
    let fruit = FRUIT.iter().find(|fruit| has(fruit.aliases));
    let is_fruit = fruit.is_some() || has(&["fruit"]);
    if is_fruit && has(&["stewed", "stew", "poached", "compote", "baked", "puree"]) {
        let (name, cause) = fruit.map_or(("fruit", "a firm one holds its shape where a ripe one collapses, so use the ripe ones for a compote"),
            |fruit| (fruit.name, fruit.cause));
        return Cow::Owned(format!(
            "A stewed {name} that stays hard was the fruit rather than the cooking: {cause}. Give it 10 more minutes with a splash of water and taste as it goes, because it goes from firm to pulp quickly once it starts."
        ));
    }
    Cow::Borrowed(
        if !is_fruit
            && has(&[
                "stew",
                "casserole",
                "braise",
                "braised",
                "braising",
                "goulash",
                "hotpot",
                "tagine",
                "ragu",
                "chilli",
                "chili",
            ])
        {
            TOUGH_STEW
        } else if has(&[
            "steak", "meat", "beef", "pork", "lamb", "chicken", "chop", "turkey", "sausage",
            "burger", "roast", "gammon", "duck", "ribs",
        ]) {
            if why(words, &words.join(" ")) {
                TOUGH_MEAT_WHY
            } else {
                TOUGH_MEAT
            }
        } else if has(&["pasta", "noodle", "rice"]) {
            TOUGH_STARCH
        } else if has(&[
            "cake", "sponge", "muffin", "cupcake", "scone", "brownie", "bun", "traybake", "loaf",
            "loaves",
        ]) {
            return bake_answer(words, "Tough", TOUGH_BAKE);
        } else if has(&[
            "pastry",
            "shortcrust",
            "shortcrust pastry",
            "pie crust",
            "pie",
            "tart",
            "quiche",
            "crumble",
        ]) {
            TOUGH_PASTRY
        } else if has(&[
            "bread", "loaf", "roll", "baguette", "pitta", "naan", "bun", "bagel",
        ]) {
            TOUGH_BREAD
        } else if has(&["egg", "omelette", "scrambled"]) {
            TOUGH_EGG
        } else {
            TOUGH_DEFAULT
        },
    )
}

static PREVENTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:avoid|avoiding|prevent|preventing|not\s+get|don'?t\s+get|dont\s+get|",
        r"never\s+get|stop\s+(?:me\s+)?getting|protect\s+(?:myself|yourself|against))\b",
        r"|\b(?:without|so\s+(?:i|we|they|he|she)\s+don'?t|so\s+(?:i|we|they|he|she)\s+dont)\s+",
        r"(?:\w+\s+){0,3}?(?:poison\w*|getting\s+ill|get\s+ill|becoming\s+ill|going\s+off)\b",
        r"|\b(?:ill|sick)\s+(?:from|off)\s+food\b|contamination",
    ))
    .expect("pinned illness prevention")
});

const AWAY_FROM_HOME: &[&str] = &[
    "abroad",
    "holiday",
    "holidays",
    "travelling",
    "traveling",
    "trip",
    "trips",
    "camping",
    "mexico",
    "india",
    "egypt",
    "thailand",
    "turkey",
    "morocco",
    "peru",
    "kenya",
    "bali",
    "spain",
    "france",
    "greece",
    "italy",
    "portugal",
    "america",
    "usa",
    "canada",
    "australia",
    "airport",
    "plane",
    "flight",
    "hotel",
    "resort",
    "street",
    "beach",
    "restaurant",
    "takeaway",
];

fn eat_when_ill(reading: &Reading<'_>) -> Option<&'static str> {
    if PREVENTION.is_match(&reading.line.clean) {
        let ingredients = super::super::pantry_names::ingredients(reading, &reading.line.words);
        if !ingredients.is_empty() || exactly(&reading.line.words, AWAY_FROM_HOME) {
            return None;
        }
        Some(
            "Wash your hands before preparing food and after handling raw meat. Keep raw meat and fish separate from food you will eat without cooking. Cook food thoroughly. Refrigerate cooled leftovers within two hours and use them within 48 hours or freeze them. Cool rice within one hour and use refrigerated rice within 24 hours. Reheat leftovers once until steaming hot throughout. Keep the fridge at 5°C or below and follow use-by dates.",
        )
    } else {
        Some(
            "Eat when you feel able. Small portions of plain food can be easier to manage. Take frequent small sips of water. Contact NHS 111 if you keep vomiting and cannot keep fluids down, or have signs of dehydration such as passing much less urine than usual.",
        )
    }
}

pub(super) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "fix-dry" => dry(reading),
        "fix-rubbery" => Some(tough(&reading.line.words)),
        "eat-when-ill" => eat_when_ill(reading).map(Cow::Borrowed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_and_rubbery_readers_follow_source_precedence_and_safe_doneness() {
        assert!(dry(&Reading::new("My shortbread is dry")).is_none());
        let pasta = dry(&Reading::new("My pasta is mushy and overcooked")).unwrap();
        assert_eq!(pasta, OVERCOOKED_PASTA);
        let cake = dry(&Reading::new("My sponge is dry")).unwrap();
        assert!(cake.starts_with("Dry sponges are"));
        let poultry = dry(&Reading::new("Is this dry turkey safe to eat?")).unwrap();
        assert!(poultry.contains("70°C or above for 2 minutes"));
        let stewed_fruit = tough(&Reading::new("My stewed apples are tough").line.words);
        assert!(stewed_fruit.starts_with("A stewed apple"));
        assert!(eat_when_ill(&Reading::new("How can I prevent food poisoning abroad?")).is_none());
        assert!(
            eat_when_ill(&Reading::new("What can I eat now I am ill?"))
                .unwrap()
                .contains("NHS 111")
        );
    }
}
