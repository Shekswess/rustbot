//! Authored prompts and constants from the pinned cakes and dishes modules.

pub(super) const CAKE_CLOCK: &[&str] = &[
    "minute", "minutes", "min", "mins", "hour", "hours", "cm", "g", "kg", "degrees", "second",
    "seconds", "percent",
];
pub(super) const CAKE_SIZE_WORDS: &[&str] = &[
    "how much cake",
    "size",
    "sized",
    "big",
    "bigger",
    "serve",
    "serving",
    "people",
    "guests",
    "crowd",
    "portions",
    "slices",
    "serves",
    "serve",
    "feeds",
    "feed",
    "heads",
];
pub(super) const CAKE_DOUBLE_WORDS: &[&str] = &[
    "double",
    "doubling",
    "halve",
    "halving",
    "half the",
    "scale up",
    "scaling up",
    "bigger batch",
    "twice as much",
    "two batches",
];
pub(super) const CAKE_DECOR_WORDS: &[&str] = &[
    "ice",
    "icing",
    "iced",
    "decorate",
    "decorating",
    "decorated",
    "buttercream",
    "frost",
    "frosting",
    "fill it",
    "filling",
    "pipe",
    "piping",
    "writing on",
    "writing happy birthday",
];
pub(super) const CUPCAKE_WORDS: &[&str] = &[
    "cupcake",
    "cupcakes",
    "cup cake",
    "cup cakes",
    "fairy cake",
    "fairy cakes",
];
pub(super) const CAKE_NUMBERS: &[&str] = &["", "one", "two", "three", "four", "five", "six"];
pub(super) const CAKE_DAY_BEFORE: &str =
    "Bake it the day before, wrap it well and fill it with buttercream on the day.";
pub(super) const CAKE_SIZE: &str = "Size it by the tin: 20cm serves 8 to 10, 23cm 12 to 14, 25cm 18 to 20, and a 30 x 20cm traybake cut into squares feeds 15 to 18.";
pub(super) const CAKE_DOUBLE: &str = "To double a recipe double every weight and split it between two tins with the same heat and time.";
pub(super) const CAKE_DECOR: &str = "Buttercream or icing between the layers and on top is all a party cake needs: spread it warm, chill the cake first, and keep it plain or add a handful of fruit or sweets.";
pub(super) const CAKE_BASE: &str = "A two-layer chocolate or madeira sponge is the easy crowd-pleaser: bake it the day before, wrap it well and fill it with buttercream on the day. One 20cm tin serves 8 to 10 people generously.";
pub(super) const CUPCAKE_KEEP: &str = "Twelve to a tray and two a head, so one 12-hole tray covers six people: bake them at 180C (160C fan) for 12 to 18 minutes and ice them on the day.";
pub(super) const EGG_BATTER: &str = "For batter, use four eggs to 200g plain flour and 300ml milk, with a pinch of salt, and rest it half an hour: the eggs are what make a Yorkshire rise, so don't cut them down. That fills a 12-hole tin.";
pub(super) const EGG_PANCAKE: &str = "Two eggs to 150g plain flour and 300ml milk makes about eight thin pancakes, enough for four people. More eggs make them richer and more likely to tear; they don't make more pancakes.";
pub(super) const EGG_MERINGUE: &str = "One egg white to 50g caster sugar, so a meringue nest for four takes two egg whites: whisk them to stiff peaks, then add the sugar a spoon at a time until it feels smooth between your fingers.";
pub(super) const EGG_SPONGE: &str = "Weigh the eggs in their shells and use the same weight of soft butter, caster sugar and self-raising flour: three eggs fills a 20cm two-layer sandwich tin (about 175g of each), and four fills a 23cm one. The eggs are the measure, so there is no fixed count - a bigger bake starts by weighing more eggs.";
pub(super) const EGG_SKIP: &str = "Not for the mix: the egg binds a cake and helps it rise, and in most sponges 60g of yoghurt, half a mashed banana or a tablespoon of ground flax in three of water does the same job. Egg-free sponges are a little more crumbly and stale sooner, so eat them within a day or two, and don't expect a swap to work in a meringue or a genoise, which are made of egg.";
pub(super) const TWO_OVEN: &str = "Yes, if they want heat within about 20C of each other: put the hotter dish in first, or give the slower one a head start, and swap the two trays on their shelves halfway through. Leave a finger's gap round each tray, keep the one that needs to brown in the middle, and add 5 to 10 minutes because two trays cut the heat - a fan oven copes better than a gas one. A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round. A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests.";
pub(super) const CAKE_AND_ROAST: &str = "A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round. A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests.";
pub(super) const TWO_OVEN_BEEF: &[&str] = &["beef", "lamb", "ham", "pork", "joint", "meat"];
pub(super) const CAKE_COUNTS: &[(&str, u16)] = &[
    ("a dozen", 12),
    ("dozen", 12),
    ("ten", 10),
    ("twelve", 12),
    ("fifteen", 15),
    ("sixteen", 16),
    ("eighteen", 18),
    ("twenty", 20),
    ("twenty five", 25),
    ("thirty", 30),
    ("thirty five", 35),
    ("forty", 40),
    ("forty five", 45),
    ("fifty", 50),
    ("sixty", 60),
    ("seventy", 70),
    ("eighty", 80),
    ("ninety", 90),
    ("a hundred", 100),
    ("hundred", 100),
];
