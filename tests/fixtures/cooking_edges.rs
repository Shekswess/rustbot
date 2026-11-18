// Pinned PyBot card callbacks read in isolation at 7d63c7f.
pub(super) struct Case(
    pub(super) &'static str,
    pub(super) &'static str,
    pub(super) &'static [(&'static str, &'static str)],
    pub(super) Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "non-stick-pan",
        "how do i clean a non-stick pan",
        &[],
        Some(
            "Warm water, washing-up liquid and a soft sponge, then dry it with a cloth: no scouring pad, no wire wool and no metal tools, because a scratch in the coating is what ruins it. For burnt-on food, simmer a little water in the pan for five minutes and lift it with a wooden spatula rather than scraping. If the coating is flaking or peeling, replace the pan - that is not something to cook through.",
        ),
    ),
    Case(
        "non-stick-pan",
        "is a scratched non-stick pan still safe",
        &[],
        Some(
            "Small scratches are fine to keep cooking on. Once it is scratched through to the metal, or the coating is flaking or peeling into the food, replace the pan: you do not want flakes of coating in a meal, and a worn pan stops being non-stick anyway. Until then, keep metal tools, scouring pads and an empty pan on high heat away from it, because those are what scratch it.",
        ),
    ),
    Case(
        "non-stick-pan",
        "should i replace a peeling pan",
        &[],
        Some(
            "Small scratches are fine to keep cooking on. Once it is scratched through to the metal, or the coating is flaking or peeling into the food, replace the pan: you do not want flakes of coating in a meal, and a worn pan stops being non-stick anyway. Until then, keep metal tools, scouring pads and an empty pan on high heat away from it, because those are what scratch it.",
        ),
    ),
    Case(
        "non-stick-pan",
        "how do i clean a pan with peeling coating",
        &[],
        Some(
            "Warm water, washing-up liquid and a soft sponge, then dry it with a cloth: no scouring pad, no wire wool and no metal tools, because a scratch in the coating is what ruins it. For burnt-on food, simmer a little water in the pan for five minutes and lift it with a wooden spatula rather than scraping. If the coating is flaking or peeling, replace the pan - that is not something to cook through.",
        ),
    ),
    Case(
        "learn-to-cook",
        "how do i learn to cook",
        &[],
        Some(
            "Learn four things and the rest follows: pasta with a sauce, an omelette, a tray of roasted vegetables, and a one-pot curry or stew. Cook the same one twice in a week rather than a new one each night, and taste as you go - food that tastes flat usually wants salt, a squeeze of lemon, or more time in the pan. A cheap thermometer takes the fear out of chicken and pork.",
        ),
    ),
    Case(
        "learn-to-cook",
        "how do i cook for my family",
        &[],
        Some(
            "Learn four things and the rest follows: pasta with a sauce, an omelette, a tray of roasted vegetables, and a one-pot curry or stew. Cook the same one twice in a week rather than a new one each night, and taste as you go - food that tastes flat usually wants salt, a squeeze of lemon, or more time in the pan. A cheap thermometer takes the fear out of chicken and pork.",
        ),
    ),
    Case(
        "learn-to-cook",
        "where do i start cooking",
        &[],
        Some(
            "Learn four things and the rest follows: pasta with a sauce, an omelette, a tray of roasted vegetables, and a one-pot curry or stew. Cook the same one twice in a week rather than a new one each night, and taste as you go - food that tastes flat usually wants salt, a squeeze of lemon, or more time in the pan. A cheap thermometer takes the fear out of chicken and pork.",
        ),
    ),
    Case("learn-to-cook", "how do i cook potatoes", &[], None),
    Case("learn-to-cook", "how do i cook pasta", &[], None),
    Case("learn-to-cook", "how do i cook chicken breasts", &[], None),
    Case("learn-to-cook", "how do i cook chard", &[], None),
    Case(
        "dough-tight",
        "my dough is too stiff",
        &[],
        Some(
            "Add water a spoonful at a time and knead a minute after each one: a stiff dough reads drier than it is, because the flour takes a few minutes to take up what is already in there, and you only feel the difference after kneading. A bread dough should be tacky rather than sticky - it clings to your hand but comes away clean - and it is ready when a piece stretches thin enough to see light through without tearing. If you overshoot, a spoonful of flour brings it back.",
        ),
    ),
    Case(
        "dough-tight",
        "this dough is sticky",
        &[],
        Some(
            "Add flour a spoonful at a time and knead it in before you judge, because a dough is wettest in the first few minutes: it should cling to your hand and come away clean. Wetter is better for an open crumb, so stop as soon as it stops running and keep a little of the water back next time, since flour alone can be 10% different in how much it drinks.",
        ),
    ),
    Case(
        "dough-tight",
        "my bread dough isnt wet enough",
        &[],
        Some(
            "Add water a spoonful at a time and knead a minute after each one: a stiff dough reads drier than it is, because the flour takes a few minutes to take up what is already in there, and you only feel the difference after kneading. A bread dough should be tacky rather than sticky - it clings to your hand but comes away clean - and it is ready when a piece stretches thin enough to see light through without tearing. If you overshoot, a spoonful of flour brings it back.",
        ),
    ),
    Case(
        "dough-tight",
        "its not dry enough",
        &[],
        Some(
            "Add flour a spoonful at a time and knead it in before you judge, because a dough is wettest in the first few minutes: it should cling to your hand and come away clean. Wetter is better for an open crumb, so stop as soon as it stops running and keep a little of the water back next time, since flour alone can be 10% different in how much it drinks.",
        ),
    ),
    Case(
        "dough-tight",
        "the dough is dry and turns sticky",
        &[],
        Some(
            "Add flour a spoonful at a time and knead it in before you judge, because a dough is wettest in the first few minutes: it should cling to your hand and come away clean. Wetter is better for an open crumb, so stop as soon as it stops running and keep a little of the water back next time, since flour alone can be 10% different in how much it drinks.",
        ),
    ),
    Case(
        "cookie-spread",
        "my cookies spread too far",
        &[],
        Some(
            "Cookies spread when the dough is warm or the butter is too soft: chill the dough 30 minutes and put it on a cool tray, and leave a good 5cm between them. Too much sugar or a greased tin makes them spread further, and a fan oven needs 20C less than the recipe.",
        ),
    ),
    Case(
        "cookie-spread",
        "cookies didnt spread",
        &[],
        Some(
            "Cookies that stay in a ball have usually had too much flour or butter that was too cold: weigh the flour rather than scooping the cup, use soft butter, and press each ball down to about 1cm with the back of a spoon before it goes in. A tablespoon of milk or an extra yolk loosens a dry dough, and the oven wants to be properly hot, 180C (160C fan), so they set before they can spread.",
        ),
    ),
    Case(
        "cookie-spread",
        "the biscuits stayed round",
        &[],
        Some(
            "Cookies that stay in a ball have usually had too much flour or butter that was too cold: weigh the flour rather than scooping the cup, use soft butter, and press each ball down to about 1cm with the back of a spoon before it goes in. A tablespoon of milk or an extra yolk loosens a dry dough, and the oven wants to be properly hot, 180C (160C fan), so they set before they can spread.",
        ),
    ),
    Case(
        "cookie-spread",
        "wont spread on the tray",
        &[],
        Some(
            "Cookies that stay in a ball have usually had too much flour or butter that was too cold: weigh the flour rather than scooping the cup, use soft butter, and press each ball down to about 1cm with the back of a spoon before it goes in. A tablespoon of milk or an extra yolk loosens a dry dough, and the oven wants to be properly hot, 180C (160C fan), so they set before they can spread.",
        ),
    ),
    Case(
        "cookie-spread",
        "why do my cookies spread",
        &[],
        Some(
            "Cookies spread when the dough is warm or the butter is too soft: chill the dough 30 minutes and put it on a cool tray, and leave a good 5cm between them. Too much sugar or a greased tin makes them spread further, and a fan oven needs 20C less than the recipe.",
        ),
    ),
    Case(
        "sugar-grainy",
        "grainy icing",
        &[],
        Some(
            "Grainy icing is undissolved sugar: sift the icing sugar first, then add the liquid a teaspoon at a time, and if it is already made, beat in a spoonful of warm milk or a few drops of lemon juice and it goes smooth. Cold liquid and coarse sugar are what leave the grain, so warm the milk slightly next time and give it a full minute of beating.",
        ),
    ),
    Case(
        "sugar-grainy",
        "fudge turned grainy",
        &[],
        Some(
            "Grainy fudge has sugar crystals in it, which happens when it is stirred while it boils or a crust of sugar from the pan gets back in. Warm it very gently with two tablespoons of milk and beat it hard until it is smooth again, or press it out flat and call it tablet. Next time stir only until the sugar dissolves, then wipe the pan sides down with a wet pastry brush and leave it to 116C.",
        ),
    ),
    Case(
        "sugar-grainy",
        "what about creme anglaise",
        &[],
        Some(
            "Grainy custard has curdled: the eggs got too hot too fast. Take it off the heat at once, whisk in a splash of cold milk to stop it cooking, then strain it through a sieve into a jug and it comes back smooth. Next time cook it over a low heat, stirring all the time, and take it off at about 82C or as soon as it coats the back of the spoon.",
        ),
    ),
    Case(
        "sugar-grainy",
        "caramel is grainy",
        &[],
        Some(
            "Grainy caramel or toffee has crystallised: add a tablespoon of water and warm it very gently without stirring until the crystals melt, then let it cool slowly. It crystallises when the sugar is stirred once it is boiling, or when one crystal from the side of the pan gets back in, so stir only until the sugar has dissolved and keep the sides of the pan wiped clean.",
        ),
    ),
    Case(
        "sugar-grainy",
        "the sugar set grainy",
        &[],
        Some(
            "Sugar that sets grainy has crystallised, from stirring once it boils or a crystal from the pan getting back in. Warm it gently with a tablespoon of water until it melts back, and next time stir only until the sugar dissolves, wipe the pan sides with a wet pastry brush, and add a spoonful of golden syrup or a squeeze of lemon at the start.",
        ),
    ),
    Case(
        "pancake-batch",
        "pancakes for my family",
        &[],
        Some(
            "For four people allow 200g plain flour, 3 eggs and 600ml milk, which makes about 16 thin pancakes; a family with small children is happy on 150g. Cook them a minute a side in a hot pan until the bubbles pop and the edges look dry.",
        ),
    ),
    Case(
        "pancake-batch",
        "how many pancakes for ten people",
        &[],
        Some(
            "For 10 people allow 500g plain flour, 7 or 8 eggs and 1.5 litres of milk, which makes about 40 thin pancakes. Cook them a minute a side in a hot non-stick pan with a little butter, until the bubbles pop and the edges look dry; for thick American ones add a teaspoon of baking powder per 200g of flour.",
        ),
    ),
    Case(
        "pancake-batch",
        "make pancakes for 2 guests",
        &[],
        Some(
            "For 2 people allow 100g plain flour, 1 or 2 eggs and 300ml of milk, which makes about 8 thin pancakes. Cook them a minute a side in a hot non-stick pan with a little butter, until the bubbles pop and the edges look dry; for thick American ones add a teaspoon of baking powder per 200g of flour.",
        ),
    ),
    Case(
        "pancake-batch",
        "pancakes for 6",
        &[],
        Some(
            "For 6 people allow 300g plain flour, 4 or 5 eggs and 900ml of milk, which makes about 24 thin pancakes. Cook them a minute a side in a hot non-stick pan with a little butter, until the bubbles pop and the edges look dry; for thick American ones add a teaspoon of baking powder per 200g of flour.",
        ),
    ),
    Case(
        "pancake-batch",
        "pancakes for 7",
        &[],
        Some(
            "For 7 people allow 350g plain flour, 5 eggs and 1.1 litres of milk, which makes about 28 thin pancakes. Cook them a minute a side in a hot non-stick pan with a little butter, until the bubbles pop and the edges look dry; for thick American ones add a teaspoon of baking powder per 200g of flour.",
        ),
    ),
    Case(
        "pancake-batch",
        "pancakes for 20",
        &[],
        Some(
            "For 20 people allow 1kg plain flour, 15 eggs and 3 litres of milk, which makes about 80 thin pancakes. Cook them a minute a side in a hot non-stick pan with a little butter, until the bubbles pop and the edges look dry; for thick American ones add a teaspoon of baking powder per 200g of flour.",
        ),
    ),
    Case(
        "pancake-batch",
        "pancakes for 30",
        &[],
        Some(
            "For 30 people allow 1 and a half kg plain flour, 22 or 23 eggs and 4.5 litres of milk, which makes about 120 thin pancakes. Cook them a minute a side in a hot non-stick pan with a little butter, until the bubbles pop and the edges look dry; for thick American ones add a teaspoon of baking powder per 200g of flour.",
        ),
    ),
    Case(
        "pancake-batch",
        "pancakes for 40",
        &[],
        Some(
            "For 40 people allow 2kg plain flour, 30 eggs and 6 litres of milk, which makes about 160 thin pancakes. Cook them a minute a side in a hot non-stick pan with a little butter, until the bubbles pop and the edges look dry; for thick American ones add a teaspoon of baking powder per 200g of flour.",
        ),
    ),
    Case(
        "pancake-batch",
        "pancakes for 50",
        &[],
        Some(
            "For four people allow 200g plain flour, 3 eggs and 600ml milk, which makes about 16 thin pancakes; a family with small children is happy on 150g. Cook them a minute a side in a hot pan until the bubbles pop and the edges look dry.",
        ),
    ),
    Case(
        "kitchen-without-an-appliance",
        "what can i cook with no oven",
        &[],
        Some(
            "With a hob you can still eat well: pasta with a sauce made in the pan, rice with stir-fried veg and an egg on top, a one-pan curry of chickpeas or lentils with tomatoes, quesadillas or toasted sandwiches pressed in a dry pan, or potatoes boiled and then crushed into a pan with butter and greens. Eggs, tinned fish and beans all cook in a pan in minutes.",
        ),
    ),
    Case(
        "kitchen-without-an-appliance",
        "my oven is broken what can i cook",
        &[],
        Some(
            "With a hob you can still eat well: pasta with a sauce made in the pan, rice with stir-fried veg and an egg on top, a one-pan curry of chickpeas or lentils with tomatoes, quesadillas or toasted sandwiches pressed in a dry pan, or potatoes boiled and then crushed into a pan with butter and greens. Eggs, tinned fish and beans all cook in a pan in minutes.",
        ),
    ),
    Case(
        "kitchen-without-an-appliance",
        "should i replace my broken oven",
        &[],
        None,
    ),
    Case(
        "kitchen-without-an-appliance",
        "oven not working",
        &[],
        None,
    ),
    Case(
        "kitchen-without-an-appliance",
        "no hob and no oven just a microwave",
        &[],
        Some(
            "With only a microwave you can still eat properly: jacket potatoes (10 minutes, turned once), rice or pasta cooked in a big bowl of water, porridge, scrambled eggs, steamed frozen veg, tinned soup or beans, and a poached egg in a mug of water. Give a jacket potato a rub of oil and salt first and it comes out closer to oven-baked.",
        ),
    ),
    Case(
        "kitchen-without-an-appliance",
        "no oven and i want biscuits",
        &[],
        Some(
            "You cannot bake without an oven, but a fridge cake needs none: melt 100g butter, 200g chocolate and 2 tablespoons of golden syrup in a pan, stir in 200g crushed biscuits and a handful of raisins, press it into a tin and chill it for 2 hours. A cheesecake that sets in the fridge, tiramisu and lemon posset are the same trick: they set cold, not in the oven.",
        ),
    ),
    Case(
        "kitchen-without-an-appliance",
        "my oven broke what should i make",
        &[],
        Some(
            "With a hob you can still eat well: pasta with a sauce made in the pan, rice with stir-fried veg and an egg on top, a one-pan curry of chickpeas or lentils with tomatoes, quesadillas or toasted sandwiches pressed in a dry pan, or potatoes boiled and then crushed into a pan with butter and greens. Eggs, tinned fish and beans all cook in a pan in minutes.",
        ),
    ),
    Case(
        "kitchen-without-an-appliance",
        "only an air fryer",
        &[],
        None,
    ),
    Case(
        "kitchen-without-an-appliance",
        "ive got no hob what can i cook",
        &[],
        Some(
            "With an oven and no hob, cook in one tray: chicken thighs or sausages with chopped potato, onion and pepper, oil, salt and herbs, 40 minutes at 200C (180C fan), turned once. Jacket potatoes take an hour at the same heat, baked pasta or a tin of tomatoes with butter and breadcrumbs bakes in 25 minutes, and roasted veg with a drained tin of chickpeas makes a meal on its own.",
        ),
    ),
    Case(
        "kitchen-without-an-appliance",
        "what can i cook with no oven",
        &[(
            "my oven is broken",
            "You can still eat well without an oven",
        )],
        None,
    ),
    Case(
        "kitchen-without-an-appliance",
        "can i cook without an oven",
        &[(
            "my oven is broken",
            "You can still eat well without an oven",
        )],
        None,
    ),
    Case(
        "tea-brew",
        "how to brew a cup of tea",
        &[],
        Some(
            "Fresh water boiled once, one teabag per mug, three to four minutes and then the bag out: that is the whole of a good cup, and the milk goes in last. Green tea wants cooler water, about 80C, for two to three minutes, and black tea wants it just off the boil, three to five. Bitter tea has steeped too long in water that was too hot, and a mug that tastes of the last one is a bag left sitting in it.",
        ),
    ),
    Case("tea-brew", "what can i have for tea", &[], None),
    Case(
        "tea-brew",
        "iced tea at home",
        &[],
        Some(
            "Cold brew it: eight teabags to a litre of cold water in the fridge overnight, bags out in the morning, then lemon and sugar to taste, which has none of the bitterness of a hot brew cooled down. In a hurry, brew double strength in a mug of boiling water for four minutes, take the bags out, and pour it over a jug of ice.",
        ),
    ),
    Case(
        "tea-brew",
        "how many tea bags for a pot",
        &[],
        Some(
            "Warm the pot first with a splash of boiling water swirled round and tipped away, then one teabag or one teaspoon of leaves per cup and one extra for the pot, water just off the boil, three to four minutes. Take the bags or leaves out before it goes to the table, and a cosy keeps the second cup hot without stewing it.",
        ),
    ),
    Case(
        "tea-brew",
        "why is my tea cloudy",
        &[],
        Some(
            "Cloudy tea is the minerals in hard water binding with the tannin, and it is harmless to drink: filter the water, or use bottled, to get it clear, or add a small pinch of bicarbonate of soda to the pot, which keeps the tannin dissolved. It is not a dirty kettle and it is not a sign the tea has gone off.",
        ),
    ),
    Case(
        "tea-brew",
        "how to make a cup of green tea",
        &[],
        Some(
            "Green tea wants water at about 80C rather than boiling, for two to three minutes: boil the kettle and let it stand for a minute or two, or add a splash of cold water first. Water straight off the boil makes it bitter, and green tea goes bitter quickly, so pull the bag out at three minutes and drink it soon after.",
        ),
    ),
    Case(
        "tea-brew",
        "can i use the teabag twice",
        &[],
        Some(
            "A second cup from the same teabag is weak and mostly colour, but it is harmless: if you want it stronger, use a fresh bag and pull it out sooner instead. Reuse one within the same hour at most, and not once it has been sitting in milk.",
        ),
    ),
    Case(
        "tea-brew",
        "why does my tea taste bitter",
        &[],
        Some(
            "Green tea wants water at about 80C for 2 to 3 minutes; black tea wants it just off the boil for 3 to 5. Bitter tea is nearly always tea that steeped too long in water that was too hot, and sour or stewed tea is a pot left standing on a warmer or in the leaves: pour it off them and make the next one weaker and shorter.",
        ),
    ),
    Case(
        "tea-brew",
        "make iced green tea in a pot",
        &[],
        Some(
            "Cold brew it: eight teabags to a litre of cold water in the fridge overnight, bags out in the morning, then lemon and sugar to taste, which has none of the bitterness of a hot brew cooled down. In a hurry, brew double strength in a mug of boiling water for four minutes, take the bags out, and pour it over a jug of ice.",
        ),
    ),
    Case(
        "tea-brew",
        "my jasmine tea is cloudy",
        &[],
        Some(
            "Cloudy tea is the minerals in hard water binding with the tannin, and it is harmless to drink: filter the water, or use bottled, to get it clear, or add a small pinch of bicarbonate of soda to the pot, which keeps the tannin dissolved. It is not a dirty kettle and it is not a sign the tea has gone off.",
        ),
    ),
];
