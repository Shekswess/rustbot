//! The pinned source labels these UK item ranges as 2025 prices.
pub(super) const A_FIGURE: &str = "(?:£|\\$|€)\\s?\\d[\\d,]*(?:\\.\\d+)?|\\b\\d[\\d,]*(?:\\.\\d+)?\\s*(?:k\\b|grand\\b|quid|pounds?|bucks)|\\b\\d[\\d,]*(?:\\.\\d+)?\\s+(?:for\\b|a\\b|an\\b|per\\b|each\\b|every\\b|too\\b|to\\s+spend\\b|in\\s+total\\b|all\\s+in\\b)";
pub(super) const A_JUDGEMENT: &str = "\\b(?:too much|too expensive|too dear|overpriced|over the odds|about right|fair price|a fair price|reasonable price|rip off|ripped off|a lot of money|worth it)\\b";
pub(super) const MONEY_WORD: &str =
    "\\b(?:price|prices|priced|cost|costs|charge|charges|fee|fees|pay|pays|paid|quid)\\b";
pub(super) const JUDGING: &str = "(?i)(?:\\u00a3\\s*\\d|\\b\\d+(?:\\.\\d+)?\\s*(?:pounds?|quid|pence|gbp|p\\b))[^.?!]*?\\b(?:a lot|lot for|too much|expensive|cheap|cheaper|cheapest|dear|dearer|overpriced|rip.?off|worth it|worth the|worth buying|reasonable|a fair price|fair price|fair|good price|bad price|good buy|bad buy|good value|poor value|a bargain|a steal|ok|okay|alright|all right|acceptable|about right|sensible|good deal|bad deal|worth the money)\\b";
pub(super) const PRICE: &str = "\\b(?:how much (?:is|are|does|do|would)|price|prices|cost|costs|cheaper|cheapest|expensive|going rate|these days|nowadays|average)\\b";
pub(super) const NOT_A_PRICE: &str = "\\b(?:weigh|weighs|grams?|g\\b|ml\\b|millilitres?|litres?|cups?|tablespoons?|teaspoons?|recipe|recipes|in a cake|for a cake|per person|serves?|servings?|calories|units?|safe|limit|how many|should i drink|drink a week|a day|make|making|grow|keep|keeps|last|lasts|freeze|store|storing|goes in|do i need|is in|rise|rises|rising|dough|yeast|knead|prove|proofing)\\b";
pub(super) const NOT_AN_ITEM: &str = "\\b(?:bill|bills|meter|tariff|standing charge|contract|insurance|premium|tax|rent|mortgage|salary|wage|wages|pension|fine|postage|delivery|shipping|vets?|prescription|train ticket|bus ticket|plane ticket|flight ticket|concert ticket|rail fare|flight|hotel|rental|deposit|lesson|course|subscription|mobile|broadband|energy|gas|electric|electricity|water bill|council|send|sending|sent|parcel|parcels|letter|letters|post office|recorded|tracked|sandwich|sandwiches|soup|curry|pie|nuggets|wings|thighs|breasts|breast|fillet|drumsticks|kebab|kiev|kievs|goujons?|burgers?|strips|wrapping?|for dinner|for tea|ireland|irish|dublin|america|australia|canada|spain|france|germany)\\b";
pub(super) const ITEMS: &[(&[&str], &str)] = &[
    (
        &[
            "pint of beer",
            "pint of lager",
            "pint of bitter",
            "pint of ale",
            "pint of cider",
            "pint of guinness",
            "pint of stout",
            "a pint",
            "beer",
            "lager",
            "bitter",
            "cider",
            "ale",
            "stout",
        ],
        "A pint is about £4 to £5.50 in most pubs and £6 to £7.50 in London and the big cities, so £5.50 is a fair average to budget. A 500ml bottle or can from a supermarket is £1.50 to £2.50, and a four-pack £5 to £7.",
    ),
    (
        &[
            "bottle of wine",
            "wine",
            "red wine",
            "white wine",
            "rose",
            "prosecco",
            "champagne",
        ],
        "A drinkable supermarket bottle is £6 to £8 and a step up is £10 to £15; a pub or restaurant charges £18 to £26 for the same sort of bottle. A small 187ml bottle is £2.50 to £3.50.",
    ),
    (
        &["pint of milk", "milk", "bottle of milk"],
        "About 70p a pint, so a two-pint bottle is £1.25 to £1.45 and a four-pint one £1.70 to £2. The filtered and the organic sorts cost half as much again, and a corner shop is dearer than a supermarket.",
    ),
    (
        &["loaf", "loaf of bread", "bread", "sliced bread"],
        "£1.20 to £1.60 for a supermarket own-brand white or wholemeal loaf, £2.50 to £4 for a sourdough or a bakery loaf, and about £1 for the value line.",
    ),
    (
        &["eggs", "half a dozen eggs", "dozen eggs", "box of eggs"],
        "£1.50 to £2.20 for six free-range eggs and £2.50 to £3.50 for twelve, with large and organic at the top of that.",
    ),
    (
        &[
            "lentils",
            "lentil",
            "red lentils",
            "bag of lentils",
            "dried lentils",
            "split peas",
            "dried beans",
            "chickpeas",
            "pulses",
        ],
        "Dry red lentils are £1 to £1.50 for 500g and £2 to £2.50 a kilo, and chickpeas and dried beans are much the same. That is why a lentil curry is one of the cheapest dinners there is: a kilo is about ten portions.",
    ),
    (
        &[
            "potatoes",
            "potato",
            "kilo of potatoes",
            "bag of potatoes",
            "new potatoes",
            "spuds",
        ],
        "About 60p to £1 a kilo for white potatoes, so a 2.5kg bag is £1.50 to £2.50. New potatoes and the small salad ones are £1.50 to £2.50 a kilo, and a baking potato about 25p to 40p each.",
    ),
    (
        &[
            "whole chicken",
            "whole chickens",
            "chicken",
            "a roasting chicken",
        ],
        "£4 to £6 for a whole chicken of about 1.5kg to 2kg, and £3.50 to £5 for a smaller one. That is four to six portions of meat plus a pot of stock, which is why it costs less per meal than the same weight of breasts.",
    ),
    (
        &[
            "stamp",
            "stamps",
            "first class stamp",
            "second class stamp",
            "book of stamps",
            "book of first class stamps",
        ],
        "About £1.70 for a first class stamp and 87p for a second class one in 2025, and a book of eight first class is about £13.50. A large letter costs more - about £2.70 first class.",
    ),
    (
        &[
            "coffee",
            "flat white",
            "latte",
            "cappuccino",
            "cup of coffee",
            "takeaway coffee",
        ],
        "£3 to £3.80 for a flat white or a latte in a chain and £3.50 to £4.50 in an independent cafe, with oat or soya milk often 30 to 50p extra.",
    ),
    (
        &["tea", "cup of tea", "pot of tea"],
        "£2.50 to £3.50 for a pot of tea in a cafe, and about 3p a cup made at home from a £2.50 box of 80 bags.",
    ),
    (
        &[
            "cigarettes",
            "fags",
            "packet of cigarettes",
            "pack of cigarettes",
            "twenty cigarettes",
        ],
        "About £14 to £16 for 20 cigarettes in 2025, and £5 to £6 for a 30g pouch of rolling tobacco.",
    ),
    (
        &[
            "punnet",
            "punnet of strawberries",
            "strawberries",
            "raspberries",
            "blueberries",
            "berries",
            "punnet of berries",
        ],
        "A 227g punnet of strawberries is £2 to £2.50 in summer and £3 to £4 out of season, and a punnet of raspberries or blueberries £2 to £3. British berries in June and July are about half the price of the imported ones in January.",
    ),
    (
        &[
            "fish and chips",
            "fish supper",
            "chippy",
            "chip shop",
            "cod and chips",
            "haddock and chips",
            "sausage supper",
        ],
        "£9 to £12 for fish and chips from a chip shop and £12 to £16 in London and the big cities, with a sit-down restaurant a few pounds more than that. A small or pensioner's portion is £6 to £8, and mushy peas about £1.50.",
    ),
    (
        &[
            "takeaway",
            "takeaway curry",
            "chinese takeaway",
            "indian takeaway",
            "pizza delivery",
            "delivered pizza",
            "a pizza",
            "kebab",
        ],
        "£9 to £13 for one takeaway main - a curry, a Chinese dish, a large pizza - and £15 to £25 for a delivery for two once the sides and the fee are on it. Collecting it yourself saves the £3 to £5 delivery fee.",
    ),
    (
        &[
            "trainers",
            "a pair of trainers",
            "running shoes",
            "sneakers",
            "school shoes",
        ],
        "£45 to £85 for a decent pair of everyday trainers, and last season's colour of the same shoe is often half price. Proper running shoes start about £70, and school shoes come in cheaper.",
    ),
    (
        &[
            "gym membership",
            "gym",
            "leisure centre",
            "swimming pool",
            "pool membership",
        ],
        "A gym is £25 to £45 a month, with a joining fee of about £20 on top. Pay as you go is about £8 a visit, which is worth working out against a month of a membership you do not use.",
    ),
    (
        &[
            "cinema ticket",
            "cinema",
            "the pictures",
            "film ticket",
            "imax ticket",
        ],
        "£8 to £15 for an adult cinema ticket, and up to £20 in London or for IMAX and a reclining seat. Most chains have a cheaper showing on a weekday before five.",
    ),
    (
        &[
            "netflix",
            "streaming service",
            "netflix subscription",
            "disney plus",
            "prime video",
            "spotify",
        ],
        "£5 to £16 a month for a streaming service, depending on the tier: the ad-supported ones are at the bottom of that and the ad-free 4K ones at the top. Paying for a year upfront, where it is offered, works out about two months free.",
    ),
    (
        &["bread and milk", "weekly shop", "food shop", "groceries"],
        "The average household spends about £70 to £80 a week on food shopping, from £40 for one person cooking from scratch to £150 for a family that buys ready meals.",
    ),
];
