//! Authored replies from the pinned meals.py and dishes.py.
pub(super) const CARRY: &[&str] = &[
    "Things that travel well: a frittata or a quiche cut in wedges, cold sausages or sausage rolls, cheese and oatcakes, cherry tomatoes, and fruit that survives a bag. Pack anything wet in its own box and take a bag for the rubbish.",
    "Cold and easy to carry: a frittata cut in wedges, sausage rolls, cheese with oatcakes or bread, tomatoes and a bag of grapes. Freeze a bottle of water the night before - it keeps the food cold and then it is your drink.",
];

pub(super) const COUNTERS: &[(&[&str], &str)] = &[
    (
        &[
            "pizza place",
            "pizza shop",
            "pizza parlour",
            "pizza parlor",
            "pizza",
            "pizzas",
            "dominos",
            "domino's",
        ],
        "One pizza each and a garlic bread to share: a 12-inch is one hungry person and a 9-inch a light one, so a 12-inch between two plus a side is the cheaper way to feed a pair. A plain margherita and one with a topping beats two fully loaded ones, which are the ones left half eaten.",
    ),
    (
        &["bakery", "bakers", "baker", "patisserie", "bakery shop"],
        "Buy the loaf that is still warm and one pastry to eat before you get home: a sourdough or a tin loaf keeps two days on the board and a week sliced in the freezer, and a cheese twist or a croissant is what a bakery is for. Ask which loaf is today's rather than yesterday's - it is the one question they are always happy to answer.",
    ),
    (
        &[
            "sandwich shop",
            "deli",
            "deli counter",
            "subway",
            "baguette shop",
            "sandwich bar",
        ],
        "The filling that is made fresh, in a baguette or a sub rather than sliced bread, and ask for the salad on the side if it has to travel back to a desk: wet salad in the sandwich is what makes it collapse. A hot filling - meatballs, a toastie, a chicken escalope - is the one worth ordering out, because it is the one that will not be as good made at home.",
    ),
];

pub(super) const COUNTER_ASK: &[&str] = &[
    "best",
    "better",
    "cheap",
    "cheapest",
    "decent",
    "good",
    "great",
    "healthy",
    "nice",
    "safe",
    "solid",
    "vegan",
    "vegetarian",
    "worth",
];

pub(super) const COUNTER_DEFINITION: &str =
    "(?i)\\bwhat(?:'s| is| are|s)?\\s+(?:a|an|the)\\s+([a-z']+)";

pub(super) const PUDDINGS: &[(&[&str], &str)] = &[
    (
        &[
            "no oven",
            "without an oven",
            "no baking",
            "no bake",
            "hob only",
            "just a hob",
            "fridge",
        ],
        "No oven, so a cold one: an Eton mess (broken meringue, whipped cream, strawberries or raspberries), a lemon posset (300ml cream, 75g sugar and the juice of a lemon, simmered 3 minutes and set overnight), or a chocolate fridge cake (200g chocolate and 100g butter melted over broken biscuits, chilled 2 hours).",
    ),
    (
        &["chocolate", "choccy", "chocolatey"],
        "Chocolate mousse: melt 200g of chocolate, beat in 3 egg yolks, fold in 300ml of whipped cream and chill 2 hours. A fridge cake is quicker - broken biscuits, melted chocolate and butter, a handful of raisins, 2 hours in the fridge.",
    ),
    (
        &[
            "cold",
            "ice cream",
            "icecream",
            "frozen",
            "summer",
            "hot day",
        ],
        "Something cold: an Eton mess, a lemon posset, or affogato - a scoop of vanilla ice cream with a shot of hot espresso over it.",
    ),
    (
        &[
            "quick",
            "quickly",
            "fast",
            "easy",
            "last minute",
            "ten minutes",
            "5 minutes",
        ],
        "Ten minutes: an Eton mess, bananas with ice cream and butterscotch sauce, or yoghurt with honey and toasted nuts. A good shop-bought tart with cream is a proper pudding too, and no one minds.",
    ),
];

pub(super) const DESSERT_WORDS: &[&str] = &[
    "pudding", "puddings", "pud", "dessert", "desserts", "afters", "sweet", "sweets", "puddingy",
];

pub(super) const CROWD_WORDS: &[&str] = &[
    "crowd", "lots", "many", "everyone", "party", "dozen", "guests", "guest", "people", "six",
    "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
    "sixteen", "eighteen", "twenty", "thirty", "forty", "fifty", "hundred",
];

pub(super) const REWARDED: &str = "(?i)\\b(?:reward|rewards|rewarding|rewarded|bribe|bribes|bribing|bribed|good behaviou?r|star chart|sticker|stickers|incentive|incentives|pocket money|allowance)\\b";

pub(super) const ATE_ALREADY: &str =
    "(?i)\\b(?:ate|eaten|drank|drunk|had|has had|have had|having)\\b";

pub(super) const PUDDING_ASK: &str = "(?i)\\b(?:what|which|who|any|anything|ideas?|suggest\\w*|recommend\\w*|make|makes|making|made|recipe|serve|serving|shall|could|should|want|need|fancy|instead|tonight|pudding|dessert)\\b";

pub(super) const CROWD_PUDDING: &str = "For a crowd, one big dish rather than plates: an apple and blackberry crumble (fruit in a dish, a crumble of 100g each of flour and butter with 50g of sugar, 40 minutes at 180C/160C fan, with custard), or a baked rice pudding, or a bread and butter pudding - all made in the morning, and all soft enough for anyone who finds chewing hard. An Eton mess takes five minutes if you buy the meringues.";

pub(super) const FRUIT_PUDDING: &str = "An apple and blackberry crumble: the fruit in a dish with a spoon of sugar, a crumble of 100g each of flour and butter rubbed together with 50g of sugar, 40 minutes at 180C (160C fan), and custard. It scales to any number, it is better made in the morning, and it is the pudding people are pleased to see.";
