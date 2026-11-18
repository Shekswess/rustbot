//! Source-authored cooking fault families and reply text.

pub(super) const BREAD_WORDS: &[&str] = &[
    "bread",
    "loaf",
    "loaves",
    "sourdough",
    "bun",
    "buns",
    "roll",
    "rolls",
    "baguette",
    "focaccia",
    "pitta",
    "pita",
    "brioche",
    "ciabatta",
];
pub(super) const SOGGY_BREAD: &[&str] = &[
    "sandwich",
    "sandwiches",
    "wrap",
    "wraps",
    "roll",
    "rolls",
    "baguette",
    "sub",
    "subs",
    "ciabatta",
    "pitta",
    "pita",
    "tortilla",
    "toastie",
    "butty",
    "butties",
    "bread",
    "loaf",
    "bun",
    "buns",
    "brioche",
    "focaccia",
    "sourdough",
];
pub(super) const SOGGY_LEAF: &[&str] = &[
    "salad",
    "salads",
    "lettuce",
    "leaves",
    "coleslaw",
    "slaw",
    "pasta salad",
    "greens",
    "spinach",
    "rocket",
    "cucumber",
    "tomato",
    "tomatoes",
];
pub(super) const GUMMY_BREAD: &str = "A gummy loaf is under-baked rather than under-proved: tap the base for a hollow sound, and if the middle feels heavy put it back for 10 minutes at the same heat. Next time prove until a floured finger springs back slowly.";
pub(super) const GUMMY_CAKE: &str = "Gummy means it needed longer, not a hotter oven: the outside set while the middle was still wet. Give it another 5 to 10 minutes at the same heat with foil over the top, and test the centre with a skewer, not the edge.";
pub(super) const SOGGY_SANDWICH: &str = "A sandwich goes soggy because the wet filling soaks into the bread: butter both slices right to the edges first, or spread mayonnaise or soft cheese as a barrier, and keep tomato, cucumber and lettuce in the middle rather than against the bread. Packed in the morning, the wet things go in a separate tub and are added at lunchtime, which is the only thing that really works on a hot day.";
pub(super) const SOGGY_SALAD: &str = "A salad goes soggy because the leaves are still wet or the dressing went on early: dry them properly in a tea towel or a salad spinner, dress it at the table, and keep the wet things - tomato, cucumber, dressing - at the bottom of the bowl until you eat. Salt draws the water out too, so season it in the last minute rather than ahead.";
pub(super) const SOGGY_BOX: &str = "A packed lunch goes soggy because water gets in from somewhere. Keep the wet things - tomato, cucumber, dressing, a drink - in their own tub and stand the leaky ones upright, put a layer of kitchen paper or a lettuce leaf under the bread, and pack the dry things on top. A leaking drink and a damp fork do more damage than a whole morning in the bag.";
pub(super) const WAFFLE: &str = "A soggy waffle is a batter that is too thin or an iron that is not hot enough: make it thicker than pancake batter - it should plop off the ladle rather than pour - let it rest 10 minutes so the flour takes up the liquid, and heat the iron until a drop of water skitters on it. Fill it three-quarters so the steam can escape, and stand the cooked ones on a rack in a low oven rather than stacking them, because a stack steams itself soft.";
pub(super) const FLAPJACK_SET: &str = "A flapjack that will not set wants less liquid or more oats: about 25g more oats and 2 tablespoons less syrup, pressed down hard into the tin with the back of a spoon before it bakes. Bake it at 180C (160C fan) for 25 minutes until the edges are golden, then leave it in the tin until it is completely cold - warm flapjacks are always soft and they firm up as they cool.";
pub(super) const FLAPJACK_CRUMBLE: &str = "Flapjacks that crumble were cut warm or short of syrup: cool them completely in the tin, then cut with a sharp knife straight down, and add 2 tablespoons more golden syrup or honey to the next batch. Pack the mix down firmly before it bakes, because a loose tray bakes into crumbs.";
pub(super) const DRY_BROWNIE: &str = "A dry brownie stayed in the oven too long or went into too big a tin: 20 to 25 minutes at 180C (160C fan) in a 20cm square tin, and take it out while the middle still looks slightly under, because it sets as it cools. Melt the butter and chocolate together gently rather than boiling them, and keep the flour light - it is there to hold the brownie together, not to make a cake.";
pub(super) const DENSE_SCONE: &str = "Dense scones come from working the dough or a wet mix: use cold butter, rub it in until it looks like breadcrumbs, and bring it together with a knife and one fold rather than kneading. Pat it 2cm thick, cut straight down with a floured cutter and bake at 220C (200C fan) for 12 to 15 minutes, because a slow oven sets them heavy.";
pub(super) const VEG_FAULTS: &[&str] = &[
    "tough",
    "tougher",
    "hard",
    "harder",
    "still hard",
    "too hard",
    "undercooked",
    "under cooked",
    "under-done",
    "not cooked",
    "not soft",
    "woody",
    "stringy",
    "squeaky",
    "chewy",
    "raw",
    "firm",
    "rubbery",
];
pub(super) const JACKET: &str = "A jacket potato is hard in the middle because it was rushed: prick it, microwave it 8 to 10 minutes until a knife goes in, then 15 to 20 minutes at 200C to crisp the skin. In the oven alone it needs an hour and a quarter at 200C, so give it that long or start it early.";
pub(super) const POTATO_ROAST: &str = "Hard roast potatoes went in too big or came out too early: give them another 10 to 15 minutes at 200C, and next time cut them no bigger than a golf ball and parboil them 8 minutes first, which is what cooks the middle before the outside crisps.";
pub(super) const POTATO_BOILED: &str = "Boiled potatoes go hard in the middle when the pieces are uneven: cut them the same size, start them in cold salted water and simmer 15 to 20 minutes, until a knife slides in. An old potato can be woody at the core, and that will not soften however long it cooks.";
pub(super) const MUSHROOM: &str = "Rubbery mushrooms have been steamed rather than fried: they hold a lot of water, so give them a wide hot pan with a little oil, one layer and no lid, and let that water cook off before you turn them. A pinch of salt only at the end, or they give up their water early and stew.";
pub(super) const VEG_FAMILIES: &[(&[&str], &str)] = &[
    (
        &[
            "green beans",
            "runner beans",
            "broad beans",
            "french beans",
            "beans",
            "mangetout",
            "sugar snap",
            "sugar snaps",
            "peas",
        ],
        "Beans go tough when they are old or undercooked: top and tail them, pull the string down the seam of runners, and boil 4 to 5 minutes in salted water. Peas and mangetout need 2 minutes at most, and a minute in a hot pan at the end brings the bite back.",
    ),
    (
        &[
            "cabbage",
            "sprouts",
            "sprout",
            "broccoli",
            "cauliflower",
            "romanesco",
            "kalettes",
            "spring greens",
            "leafy greens",
            "collard greens",
            "mixed greens",
            "collard",
            "pak choi",
            "bok choy",
            "broccolini",
            "savoy",
            "spring cabbage",
        ],
        "Slice it thinner and give it longer: a brassica goes tough when the stalk is undercooked, so cut a cross in the stalk or slice it thin and steam 5 to 8 minutes, until a knife slides in. If it has turned grey and sulphury it is overcooked instead, so give it two minutes less next time.",
    ),
    (
        &[
            "carrot",
            "carrots",
            "parsnip",
            "parsnips",
            "swede",
            "turnip",
            "turnips",
            "beetroot",
            "celeriac",
            "sweet potato",
            "sweet potatoes",
            "jerusalem artichoke",
        ],
        "Roots go hard in the middle when the pieces are uneven: cut them no bigger than a thumb, start them in cold salted water and simmer 15 to 20 minutes, or roast them covered in foil first. If the core is woody, cut it out - an old carrot or parsnip will not soften however long it cooks.",
    ),
    (
        &["kale", "cavolo nero", "chard", "spinach"],
        "Strip the leaves off the stalks, because the stalk is what stays tough: steam the leaves 3 to 4 minutes, or rub kale with oil and salt for a minute to soften it for a salad without any heat.",
    ),
    (
        &[
            "aubergine",
            "aubergines",
            "courgette",
            "courgettes",
            "zucchini",
            "pepper",
            "peppers",
            "squash",
            "pumpkin",
            "fennel",
            "okra",
            "tomato",
            "tomatoes",
        ],
        "Give it longer in a hot oven, 200C, until it collapses and the edges colour: these are mostly water, so they taste raw and squeaky until that water cooks out. Salt aubergine and courgette first and dry them, and spread them out so they roast rather than steam.",
    ),
    (
        &[
            "leek",
            "leeks",
            "celery",
            "onion",
            "onions",
            "spring onion",
            "spring onions",
            "shallot",
            "shallots",
        ],
        "Slice it thinner and cook it slower: leeks, onions and celery stay tough when they go into a dry pan over high heat, so give them a knob of butter, a splash of water and a lid for 10 minutes. Only the dark green tops of a leek are tough enough to keep back - put those in a stock instead.",
    ),
    (
        &[
            "vegetables",
            "veg",
            "veggies",
            "root veg",
            "roast veg",
            "stir fry veg",
            "mixed veg",
            "ratatouille",
        ],
        "Cut it into even pieces, no thicker than a finger, and give it longer: a tough vegetable is nearly always undercooked in the middle. Salted water for boiling, 200C in a single layer for roasting, and a knife should slide in before it comes off the heat.",
    ),
];
