//! Pinned callback contents. The upstream responder skips a callback that raises.

pub(super) const CASES: &[(&str, &str, Option<&str>)] = &[
    (
        "cake-size",
        "I need a cake for my son's birthday.",
        Some(
            "A two-layer chocolate or madeira sponge is the easy crowd-pleaser: bake it the day before, wrap it well and fill it with buttercream on the day. One 20cm tin serves 8 to 10 people generously.",
        ),
    ),
    (
        "cake-size",
        "how big a cake should i make",
        Some(
            "Size it by the tin: 20cm serves 8 to 10, 23cm 12 to 14, 25cm 18 to 20, and a 30 x 20cm traybake cut into squares feeds 15 to 18. Bake it the day before, wrap it well and fill it with buttercream on the day. To double a recipe double every weight and split it between two tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "can i double a cake recipe?",
        Some(
            "To double a recipe double every weight and split it between two tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "how do i ice the party cake?",
        Some(
            "Buttercream or icing between the layers and on top is all a party cake needs: spread it warm, chill the cake first, and keep it plain or add a handful of fruit or sweets.",
        ),
    ),
    (
        "cake-size",
        "how many cupcakes for a party?",
        Some(
            "Twelve to a tray and two a head, so one 12-hole tray covers six people: bake them at 180C (160C fan) for 12 to 18 minutes and ice them on the day.",
        ),
    ),
    (
        "cake-size",
        "how many will a 25cm cake serve?",
        Some(
            "A 25cm two-layer sponge feeds 18 to 20. Bake it the day before, wrap it well and fill it with buttercream on the day. To double a recipe double every weight and split it between two tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "how many people from a 30 x 20 cm fairy cake traybake?",
        Some(
            "A 30 x 20cm traybake cut into squares feeds 15 to 18.  Bake it the day before, wrap it well and fill it with buttercream on the day. To double a recipe double every weight and split it between two tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "how much cake do i need for 40 people?",
        Some(
            "Two 25cm tins feed 36 to 40 between them, which covers 40. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "a cake for a dozen guests",
        Some(
            "A 23cm two-layer sponge feeds 12 to 14, so one tin covers 12. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "one cake for 12 and one for 25",
        Some(
            "Two 23cm tins feed 24 to 28 between them, which covers 25. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "A cake for 12 minutes",
        Some(
            "A two-layer chocolate or madeira sponge is the easy crowd-pleaser: bake it the day before, wrap it well and fill it with buttercream on the day. One 20cm tin serves 8 to 10 people generously.",
        ),
    ),
    (
        "cake-size",
        "make 1 cake for 90 people",
        Some(
            "Five 30 x 20cm traybakes feed 75 to 90 between them, which covers 90. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    ("cake-size", "a cake for 900 people", None),
    (
        "cake-size",
        "birthday cake for eighty people",
        Some(
            "Four 25cm tins feed 72 to 80 between them, which covers 80. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "a cake for ੨੯ people",
        Some(
            "Two 30 x 20cm traybakes feed 30 to 36 between them, which covers 29. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "a cake to serve guests for 20",
        Some(
            "A 25cm two-layer sponge feeds 18 to 20, so one tin covers 20. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "cake for 20.5 people",
        Some(
            "A 25cm two-layer sponge feeds 18 to 20, so one tin covers 20. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "how many cupcakes for 40",
        Some(
            "80 cupcakes for 40 people: two a head, baked at 180C for 12 to 18 minutes and iced on the day. That is 4 trays of 24, and a muffin tray takes 12 at a time.",
        ),
    ),
    (
        "cake-size",
        "how many cupcakes for 12",
        Some(
            "24 cupcakes for 12 people: two a head, baked at 180C for 12 to 18 minutes and iced on the day.",
        ),
    ),
    (
        "cake-size",
        "how many cupcakes for 13",
        Some(
            "26 cupcakes for 13 people: two a head, baked at 180C for 12 to 18 minutes and iced on the day. That is 2 trays of 24, and a muffin tray takes 12 at a time.",
        ),
    ),
    (
        "cake-size",
        "a cake for 20cm",
        Some(
            "A 20cm two-layer sponge feeds 8 to 10. Bake it the day before, wrap it well and fill it with buttercream on the day. To double a recipe double every weight and split it between two tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "cake serves three",
        Some(
            "Size it by the tin: 20cm serves 8 to 10, 23cm 12 to 14, 25cm 18 to 20, and a 30 x 20cm traybake cut into squares feeds 15 to 18. Bake it the day before, wrap it well and fill it with buttercream on the day. To double a recipe double every weight and split it between two tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "I can make a party cake for 15 kids",
        Some(
            "A 25cm two-layer sponge feeds 18 to 20, so one tin covers 15. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    (
        "cake-size",
        "cake for 120 people",
        Some(
            "Six 25cm tins feed 108 to 120 between them, which covers 120. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    ("cake-size", "cake for 121 people", None),
    (
        "cake-size",
        "cake for 170 people",
        Some(
            "Four 25cm tins and five 30 x 20cm traybakes feed 147 to 170 between them, which covers 170. Bake it the day before, wrap it well and fill it with buttercream on the day. Double every weight and split it between the tins with the same heat and time.",
        ),
    ),
    ("cake-size", "cake for 200 people", None),
    ("cake-size", "cake for 232 people", None),
    ("cake-size", "cake for 233 people", None),
    (
        "cake-eggs",
        "how many eggs in a sponge cake?",
        Some(
            "Weigh the eggs in their shells and use the same weight of soft butter, caster sugar and self-raising flour: three eggs fills a 20cm two-layer sandwich tin (about 175g of each), and four fills a 23cm one. The eggs are the measure, so there is no fixed count - a bigger bake starts by weighing more eggs.",
        ),
    ),
    (
        "cake-eggs",
        "how many eggs in Yorkshire pudding?",
        Some(
            "For batter, use four eggs to 200g plain flour and 300ml milk, with a pinch of salt, and rest it half an hour: the eggs are what make a Yorkshire rise, so don't cut them down. That fills a 12-hole tin.",
        ),
    ),
    (
        "cake-eggs",
        "how many eggs in toad in the hole?",
        Some(
            "For batter, use four eggs to 200g plain flour and 300ml milk, with a pinch of salt, and rest it half an hour: the eggs are what make a Yorkshire rise, so don't cut them down. That fills a 12-hole tin.",
        ),
    ),
    (
        "cake-eggs",
        "how many eggs in thin pancakes?",
        Some(
            "Two eggs to 150g plain flour and 300ml milk makes about eight thin pancakes, enough for four people. More eggs make them richer and more likely to tear; they don't make more pancakes.",
        ),
    ),
    (
        "cake-eggs",
        "how many eggs in pavlova?",
        Some(
            "One egg white to 50g caster sugar, so a meringue nest for four takes two egg whites: whisk them to stiff peaks, then add the sugar a spoon at a time until it feels smooth between your fingers.",
        ),
    ),
    (
        "cake-eggs",
        "do i have to use eggs in a cake?",
        Some(
            "Not for the mix: the egg binds a cake and helps it rise, and in most sponges 60g of yoghurt, half a mashed banana or a tablespoon of ground flax in three of water does the same job. Egg-free sponges are a little more crumbly and stale sooner, so eat them within a day or two, and don't expect a swap to work in a meringue or a genoise, which are made of egg.",
        ),
    ),
    (
        "cake-eggs",
        "can i skip the eggs in batter?",
        Some(
            "Not for the mix: the egg binds a cake and helps it rise, and in most sponges 60g of yoghurt, half a mashed banana or a tablespoon of ground flax in three of water does the same job. Egg-free sponges are a little more crumbly and stale sooner, so eat them within a day or two, and don't expect a swap to work in a meringue or a genoise, which are made of egg.",
        ),
    ),
    (
        "cake-eggs",
        "can i skip cooking in a cake",
        Some(
            "Not for the mix: the egg binds a cake and helps it rise, and in most sponges 60g of yoghurt, half a mashed banana or a tablespoon of ground flax in three of water does the same job. Egg-free sponges are a little more crumbly and stale sooner, so eat them within a day or two, and don't expect a swap to work in a meringue or a genoise, which are made of egg.",
        ),
    ),
    (
        "two-dishes-one-oven",
        "can i bake a roast and a cake in one oven?",
        Some(
            "A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round. A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests.",
        ),
    ),
    (
        "two-dishes-one-oven",
        "two trays at once in the oven",
        Some(
            "Yes, if they want heat within about 20C of each other: put the hotter dish in first, or give the slower one a head start, and swap the two trays on their shelves halfway through. Leave a finger's gap round each tray, keep the one that needs to brown in the middle, and add 5 to 10 minutes because two trays cut the heat - a fan oven copes better than a gas one. A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round. A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests.",
        ),
    ),
    (
        "two-dishes-one-oven",
        "can i cook cake and a pork joint at once?",
        Some(
            "A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round. A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests.",
        ),
    ),
    (
        "two-dishes-one-oven",
        "can i cook a cake and roast vegetables?",
        Some(
            "A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round. A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests.",
        ),
    ),
    (
        "two-dishes-one-oven",
        "how do I cook beefcakes together",
        Some(
            "Yes, if they want heat within about 20C of each other: put the hotter dish in first, or give the slower one a head start, and swap the two trays on their shelves halfway through. Leave a finger's gap round each tray, keep the one that needs to brown in the middle, and add 5 to 10 minutes because two trays cut the heat - a fan oven copes better than a gas one. A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round. A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests.",
        ),
    ),
    (
        "two-dishes-one-oven",
        "can i cook porked cakes?",
        Some(
            "Yes, if they want heat within about 20C of each other: put the hotter dish in first, or give the slower one a head start, and swap the two trays on their shelves halfway through. Leave a finger's gap round each tray, keep the one that needs to brown in the middle, and add 5 to 10 minutes because two trays cut the heat - a fan oven copes better than a gas one. A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round. A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests.",
        ),
    ),
];
