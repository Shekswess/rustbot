//! Household reference tables from PyBot home.py.

pub(super) const MACHINE_NAMES: &[(&str, &str)] = &[
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
    ("sandwich maker", "sandwich maker"),
    ("hair dryer", "hair dryer"),
    ("hairdryer", "hair dryer"),
    ("toaster", "toaster"),
    ("kettle", "kettle"),
    ("blender", "blender"),
];
pub(super) const MACHINE_TAKES: &[(&str, &str)] = &[
    (
        "fridge",
        "A fridge takes about 4 hours to get cold, and up to 24 hours for a fridge-freezer's freezer side. Put the food in once it is down to temperature, and put a few bottles of water in the gaps - a full fridge holds its temperature far better than an empty one.",
    ),
    (
        "freezer",
        "A freezer takes up to 24 hours to reach -18C from new. Leave it empty and shut until then, and when you move house keep it packed and shut - a full freezer stays frozen for a day.",
    ),
    (
        "oven",
        "An oven takes 10 to 15 minutes to reach 180C (160C fan). The light going out is the thermostat cutting in, not the shelves; a fan oven reaches temperature in about 5 minutes.",
    ),
    (
        "washing machine",
        "A washing machine takes about 2 hours for a normal cotton wash at 40C, and 3 hours or more on an eco cycle. The quick programme is 15 to 30 minutes for a lightly worn load, and a cold wash takes the same time as a hot one.",
    ),
    (
        "dishwasher",
        "A dishwasher takes about 2 hours on a normal cycle. An eco cycle runs longer, 3 hours or more, because it heats the water slowly and uses less of it.",
    ),
    (
        "tumble dryer",
        "A tumble dryer takes 1 to 2 hours for a full load. A spin at the end of the wash cuts it, and so does clearing the lint filter.",
    ),
    (
        "boiler",
        "A boiler takes a few seconds for hot water at the tap and about 20 to 30 minutes to warm the radiators from cold. A combi boiler is the fast one because it heats water as you use it, a hot water tank takes its own half hour to refill and reheat, and radiators that stay cold at the top want bleeding.",
    ),
];
pub(super) const TAKE_WORDS: &[&str] = &[
    "a",
    "about",
    "an",
    "and",
    "boil",
    "brand",
    "chilled",
    "cold",
    "come",
    "comeup",
    "cool",
    "cooled",
    "cycle",
    "do",
    "does",
    "down",
    "dry",
    "finish",
    "for",
    "get",
    "gets",
    "heat",
    "heated",
    "hot",
    "how",
    "in",
    "is",
    "it",
    "its",
    "load",
    "long",
    "my",
    "new",
    "normally",
    "of",
    "on",
    "preheat",
    "radiator",
    "radiators",
    "reach",
    "roughly",
    "run",
    "running",
    "take",
    "takes",
    "taking",
    "temperature",
    "the",
    "time",
    "to",
    "typically",
    "up",
    "usually",
    "warm",
    "warmup",
    "wash",
    "water",
];
pub(super) const WASHING_MACHINE_OFTEN: &str = "There is no yearly service to book: a washing machine has no service interval like a boiler's. What keeps it going is cleaning the filter, leaving the door open to dry, and a hot 60C maintenance wash once a month, with a descaler if your water is hard. Pay someone only when it is noisy, leaking or not draining, and that is a repair rather than a service.";
pub(super) const WASHING_MACHINE_WORTH: &str = "No, a washing machine has no yearly service like a boiler's. What keeps it going is cleaning the filter, leaving the door open to dry, and a hot 60C maintenance wash once a month, with a descaler if your water is hard. Pay someone only when it is noisy, leaking or not draining, and that is a repair rather than a service.";
pub(super) const DISHWASHER_OFTEN: &str = "There is no yearly service to book: a dishwasher looks after itself if you clean the filter once a month, keep the salt topped up in hard water and run the hottest cycle now and then. A paid visit is only worth it if it smells or will not drain, and then it is a repair.";
pub(super) const DISHWASHER_WORTH: &str = "No, a dishwasher does not need a yearly service. Clean the filter once a month, keep the salt topped up if your water is hard, and run the hottest cycle now and then; a paid visit is only worth it if it smells or will not drain, and then it is a repair.";
pub(super) const TUMBLE_DRYER_OFTEN: &str = "There is no yearly service to book: a dryer needs its lint filter emptied after every load, its vent pipe cleared, and the condenser filter rinsed once a month on a heat-pump model. Pay someone only if it takes two cycles to dry, which is usually a blocked vent or a worn belt.";
pub(super) const TUMBLE_DRYER_WORTH: &str = "No, a dryer does not need a yearly service. Empty the lint filter after every load, clear the vent pipe, and rinse the condenser filter on a heat-pump dryer once a month. Pay someone only if it takes two cycles to dry, which is usually a blocked vent or a worn belt.";
pub(super) const OVEN_OFTEN: &str = "There is no yearly service to book: clean the shelves and the door glass, run the pyrolytic setting if it has one, and leave the rest alone unless it stops heating evenly. A paid oven clean is cleaning, not servicing.";
pub(super) const OVEN_WORTH: &str = "No, an oven does not need a yearly service. Clean the shelves and the door glass, run the pyrolytic setting if it has one, and leave the rest alone unless it stops heating evenly; a paid oven clean is cleaning, not servicing.";
pub(super) const FRIDGE_OFTEN: &str = "There is no yearly service to book: dust the coils at the back once a year, keep the door seal clean, and leave a gap round it for the heat to escape. Pay someone only when it stops holding temperature.";
pub(super) const FRIDGE_WORTH: &str = "No, a fridge or freezer does not need a yearly service. Dust the coils at the back once a year, keep the door seal clean, and leave a gap round it for the heat to escape; pay someone only when it stops holding temperature.";
pub(super) const MACHINE_SERVICE: &[(&str, &str, &str)] = &[
    (
        "washing machine",
        WASHING_MACHINE_OFTEN,
        WASHING_MACHINE_WORTH,
    ),
    ("washer", WASHING_MACHINE_OFTEN, WASHING_MACHINE_WORTH),
    ("dishwasher", DISHWASHER_OFTEN, DISHWASHER_WORTH),
    ("tumble dryer", TUMBLE_DRYER_OFTEN, TUMBLE_DRYER_WORTH),
    ("dryer", TUMBLE_DRYER_OFTEN, TUMBLE_DRYER_WORTH),
    ("oven", OVEN_OFTEN, OVEN_WORTH),
    ("fridge freezer", FRIDGE_OFTEN, FRIDGE_WORTH),
    ("fridge", FRIDGE_OFTEN, FRIDGE_WORTH),
    ("freezer", FRIDGE_OFTEN, FRIDGE_WORTH),
];
pub(super) const WATER_USE: &[(&[&str], &str)] = &[
    (
        &["bath", "bathtub", "bath tub"],
        "A full bath is about 80 litres, and a deep one nearer 100, so a short shower is the cheaper wash: five minutes under a normal shower is around 50 litres.",
    ),
    (
        &["shower"],
        "A shower runs at about 10 litres a minute, so five minutes is 50 litres and ten is 100; a power shower can be twice that, and turning the tap off while you soap up is the free half of the saving.",
    ),
    (
        &["toilet", "flush", "cistern"],
        "A modern toilet uses about 6 litres a flush and an older one 9 to 13, so a family of four flushing ten times a day is 420 to 910 litres a week. Put a few drops of food colouring in the cistern: if it reaches the pan without a flush, the valve is leaking and that is a cheap repair that pays for itself.",
    ),
    (
        &["washing machine", "washer dryer", "wash cycle"],
        "A washing machine cycle uses about 45 to 50 litres, and a half load uses nearly the same as a full one, so wait for a full drum; a 30 degree cycle saves the electricity rather than the water.",
    ),
    (
        &["dishwasher"],
        "A dishwasher uses about 10 to 15 litres a wash, which is less than washing up by hand under a running tap, so run it full and on the eco setting rather than rinsing the plates first.",
    ),
    (
        &["bucket", "mop bucket"],
        "A bucket is about 10 litres, a mop bucket 5 to 15 depending on the size, which is the measure to count in when you are sizing a water butt: a 100 litre butt is ten buckets.",
    ),
    (
        &["hose", "sprinkler", "hosepipe"],
        "A garden hose runs at 10 to 15 litres a minute, so twenty minutes with a sprinkler is 200 to 300 litres, more than a family uses indoors in a day: a watering can at the roots is the same water where the plant can reach it.",
    ),
    (
        &["hot tub"],
        "A hot tub holds 1,000 to 1,500 litres. Follow your model's maintenance instructions and adjust cleaning and water replacement for how often it is used. A cover reduces heat loss.",
    ),
    (
        &["water butt", "butt"],
        "A water butt is sold by size: 100 to 250 litres is the usual garden one, and a roof gives about 1 litre for every millimetre of rain on every square metre of it, so 1,000 litres a year per square metre in most of the UK.",
    ),
    (
        &["paddling pool", "paddling pools", "swimming pool"],
        "Measure it and multiply: length times width times depth in metres, times 1,000, gives litres. A 2m by 1m paddling pool 30cm deep holds 600 litres, and a cubic metre is 1,000.",
    ),
];
pub(super) const WATER_FALLBACK: &str = "Measure it and multiply: length times width times depth in metres, times 1,000, gives litres, and a cubic metre is 1,000 litres. A full bath is about 80 litres and five minutes in the shower about 50.";
pub(super) const CITY_DAYS: &[&str] = &[
    "amsterdam",
    "athens",
    "bangkok",
    "barcelona",
    "bath",
    "berlin",
    "boston",
    "brussels",
    "budapest",
    "buenos aires",
    "cape town",
    "chicago",
    "copenhagen",
    "dubai",
    "dublin",
    "dubrovnik",
    "edinburgh",
    "florence",
    "granada",
    "hanoi",
    "havana",
    "helsinki",
    "istanbul",
    "krakow",
    "kyoto",
    "lisbon",
    "ljubljana",
    "london",
    "los angeles",
    "madrid",
    "marrakech",
    "mexico city",
    "milan",
    "munich",
    "naples",
    "new york",
    "oslo",
    "paris",
    "porto",
    "prague",
    "reykjavik",
    "rio de janeiro",
    "rome",
    "san francisco",
    "san sebastian",
    "santorini",
    "seville",
    "singapore",
    "stockholm",
    "sydney",
    "tallinn",
    "tokyo",
    "toronto",
    "valencia",
    "venice",
    "vienna",
    "zurich",
];
pub(super) const HARD_ROLLER: &str = "A sleeve that has gone hard is not worth rescuing: the paint has set through the pile and no amount of white spirit brings the nap back, so bin it and buy another - they are two or three pounds. What saves you the next one is not leaving it to dry: wrap it in cling film between coats, and wash it out the moment you finish.";
pub(super) const HARD_BRUSH: &str = "Soak it before you bin it: water-based paint softens in an hour or two in hot water with a good squeeze of washing-up liquid, worked through the bristles with your fingers and a comb, and gloss wants white spirit or a brush restorer first and the soapy wash after. Once it is soft, comb it straight and dry it bristles down - and if the bristles have set bent, it will paint bent, so keep it for priming and undercoat.";
pub(super) const LOOSE_BRISTLES: &str = "Bristles that come out in the paint are usually a cheap brush rather than a fault you can fix: the ferrule is crimped onto a bundle that was never glued properly, and no amount of washing holds it. Comb it out dry before you start, keep that brush for undercoat and primer where a stray bristle does not show, and buy a decent one for the topcoat - a bristle left in gloss shows for years.";
pub(super) const STORE_ROLLER: &str = "Wrap the sleeve, do not wash it: cling film or a carrier bag pressed round the pile with the air out keeps it soft between coats, and the same goes overnight if you keep the wrapped roller in a dry place away from food. Standing it in the tray with a little paint on it works for an hour or so. A sleeve that is left to dry out is a sleeve you throw away.";
pub(super) const WASH_OIL_ROLLER: &str = "A sleeve that has had gloss on it wants white spirit worked through it, then a wash in warm soapy water: oil paint skins over in a sleeve the same way it does in a brush, and a skinned sleeve sheds fibres on the next wall. Spin it out, stand it on end to drain, and let it dry standing up so the pile does not flatten. A sleeve that has gone hard is not worth saving - they are two or three pounds.";
pub(super) const WASH_ROLLER: &str = "Wash a roller sleeve under warm water with a little washing-up liquid, working it with your fingers until the water runs clear, then spin it out - a roller on its frame spun in a bucket or against the side of the sink - and stand it on end to drain so the pile does not flatten. For a short stop, wrap the sleeve in cling film or stand it in the tray with a little paint on it, and never leave it soaking in water: the glue that holds the pile lets go.";
pub(super) const WASH_OIL_BRUSH: &str = "Gloss and oil-based paint come out with white spirit, not water: work it through the bristles, wipe them on a rag, then wash the brush in warm soapy water and comb it straight. Dry it bristles down so water does not sit in the ferrule, and never leave it standing on its bristles in a jar.";
pub(super) const WASH_BRUSH: &str = "Warm water and washing-up liquid, worked right down to the ferrule until the water runs clear, then comb the bristles straight and dry the brush bristles down - never standing on them in a jar, which sets them splayed. Emulsion comes out easily while it is wet and hardly at all once it has dried.";
pub(super) const STORE_BRUSH: &str = "Wrap and seal it: cling film or a carrier bag pressed round the bristles with the air out keeps a brush soft overnight. Store it in a dry place away from food.";
pub(super) const BETWEEN_COATS: &str = "Wrap it in cling film or a carrier bag with the air squeezed out, or in a damp cloth, and it will still be soft at the end of the day: that is the whole trick between coats, and a brush left out on the tin dries in about an hour. For an overnight stop, seal the wrapping and keep it in a dry place away from food, and for a roller, wrap the sleeve the same way or stand it in the tray with a little paint on it.";
