//! Exact baking and cooking guide replies from the pinned PyBot callbacks.

pub(super) const CASES: &[(&str, &str, Option<&str>)] = &[
    (
        "loaf-tin",
        "What size loaf tin do I need?",
        Some(
            "A 2lb loaf tin is 19 x 11cm for a 900g loaf and a 1lb tin is 15 x 9.5cm for a 450g one; a 20cm round cake tin takes the 4-egg sponge and an 18cm one the 3-egg. Fill a bread tin two-thirds full and a cake tin three-quarters at most. Line bread with baking parchment; for a cake, grease the tin and line only the base, because loose paper at the sides makes the sponge climb rather than rise.",
        ),
    ),
    (
        "loaf-tin",
        "how do i line a loaf tin",
        Some(
            "Line bread with baking parchment; for a cake, grease the tin and line only the base, because loose paper at the sides makes the sponge climb rather than rise. A 2lb loaf tin is 19 x 11cm for a 900g loaf and a 1lb tin is 15 x 9.5cm for a 450g one; a 20cm round cake tin takes the 4-egg sponge and an 18cm one the 3-egg. Fill a bread tin two-thirds full and a cake tin three-quarters at most.",
        ),
    ),
    (
        "loaf-tin",
        "how do i grease a tin",
        Some(
            "Rub the inside of the tin all over with butter or a little oil, then dust it with flour and tip the extra out; a circle of baking paper on the base does the same job. A 2lb loaf tin is 19 x 11cm for a 900g loaf and a 1lb tin is 15 x 9.5cm for a 450g one; a 20cm round cake tin takes the 4-egg sponge and an 18cm one the 3-egg.",
        ),
    ),
    (
        "loaf-tin",
        "Grease the tin and use baking paper?",
        Some(
            "Line bread with baking parchment; for a cake, grease the tin and line only the base, because loose paper at the sides makes the sponge climb rather than rise. A 2lb loaf tin is 19 x 11cm for a 900g loaf and a 1lb tin is 15 x 9.5cm for a 450g one; a 20cm round cake tin takes the 4-egg sponge and an 18cm one the 3-egg. Fill a bread tin two-thirds full and a cake tin three-quarters at most.",
        ),
    ),
    (
        "loaf-tin",
        "Which size tin for a Christmas cake?",
        Some(
            "A fruit cake or a Christmas cake wants a deep tin, not a sponge tin: a 20cm round one for a 2kg mix, lined with a double thickness of parchment and a collar of paper round the outside, because it bakes for two and a half hours or more. A 2lb loaf tin is 19 x 11cm for a 900g loaf and a 1lb tin is 15 x 9.5cm for a 450g one; a 20cm round cake tin takes the 4-egg sponge and an 18cm one the 3-egg.",
        ),
    ),
    (
        "loaf-tin",
        "What size tin for a wedding cake with lining?",
        Some(
            "A fruit cake or a Christmas cake wants a deep tin, not a sponge tin: a 20cm round one for a 2kg mix, lined with a double thickness of parchment and a collar of paper round the outside, because it bakes for two and a half hours or more. A 2lb loaf tin is 19 x 11cm for a 900g loaf and a 1lb tin is 15 x 9.5cm for a 450g one; a 20cm round cake tin takes the 4-egg sponge and an 18cm one the 3-egg.",
        ),
    ),
    (
        "cake-fault",
        "why did my cake dome",
        Some(
            "A dome is the outside setting before the middle has caught up: bake 10C lower, 170C (150C fan) on the middle shelf, use the tin the recipe names rather than a smaller one, and stop beating the moment the flour is in. Trim the dome flat with a bread knife before you fill it, and nobody sees it.",
        ),
    ),
    (
        "cake-fault",
        "why does the cake raise in the middle",
        Some(
            "An even rise is a level batter and a steady oven: sift the raising agent through the flour rather than adding it last, spread the batter level with the back of a spoon, and use a tin that the mix fills by two thirds. Turn the tin once, after 15 minutes, if your oven browns the back corner first.",
        ),
    ),
    (
        "cake-fault",
        "why did my sponge split on top",
        Some(
            "A split top is the crust setting while the middle is still rising: lower the oven 10C, put the tin on the middle shelf, and keep the door shut for the first 20 minutes. Cool it in the tin for 10 minutes, then on a rack, because a cake cooled in a hot tin goes clammy and splits.",
        ),
    ),
    (
        "cake-fault",
        "my cake tastes of egg",
        Some(
            "An eggy taste is too many eggs for the flour or a bake that stopped a few minutes short: add a quarter teaspoon of salt and a teaspoon of vanilla or the zest of a lemon, and give it 5 more minutes, checking with a skewer. The same mix tastes eggier cold, so try it once it has cooled rather than from the tin.",
        ),
    ),
    (
        "cake-fault",
        "my cake has a soggy bottom",
        Some(
            "A soggy bottom is underbaked underneath: use a metal tin rather than a ceramic dish, bake on the middle shelf and not above a tray of water, and give it 5 to 10 minutes longer with foil over the top once it is brown. A cold filling or a wet fruit layer under a thick batter does the same thing.",
        ),
    ),
    (
        "cake-fault",
        "how do i get cakes to rise evenly",
        Some(
            "An even rise is a level batter and a steady oven: sift the raising agent through the flour rather than adding it last, spread the batter level with the back of a spoon, and use a tin that the mix fills by two thirds. Turn the tin once, after 15 minutes, if your oven browns the back corner first.",
        ),
    ),
    (
        "cake-fault",
        "the cake is cracked and domed",
        Some(
            "A dome is the outside setting before the middle has caught up: bake 10C lower, 170C (150C fan) on the middle shelf, use the tin the recipe names rather than a smaller one, and stop beating the moment the flour is in. Trim the dome flat with a bread knife before you fill it, and nobody sees it.",
        ),
    ),
    (
        "cake-fault",
        "My cake's got a WET BOTTOM.",
        Some(
            "A soggy bottom is underbaked underneath: use a metal tin rather than a ceramic dish, bake on the middle shelf and not above a tray of water, and give it 5 to 10 minutes longer with foil over the top once it is brown. A cold filling or a wet fruit layer under a thick batter does the same thing.",
        ),
    ),
    (
        "pastry-fault",
        "why does my pastry shrink",
        Some(
            "Pastry shrinks when it is stretched into the tin: press it gently into the corners rather than pulling the sides up, chill the lined case for 20 minutes before it goes in, and leave the overhang on until after baking, because the spare pastry holds the sides up while they set.",
        ),
    ),
    (
        "pastry-fault",
        "how do i fix a soggy pastry base",
        Some(
            "A soggy pastry base wants more heat underneath it and less water in the filling: blind bake it 5 minutes longer than the recipe says, brush the warm base with beaten egg white before the filling goes in, and stand the tin on a hot baking sheet so the heat reaches the pastry rather than stopping at the filling.",
        ),
    ),
    (
        "pastry-fault",
        "My tart case slid down and has a soggy bottom",
        Some(
            "Pastry shrinks when it is stretched into the tin: press it gently into the corners rather than pulling the sides up, chill the lined case for 20 minutes before it goes in, and leave the overhang on until after baking, because the spare pastry holds the sides up while they set.",
        ),
    ),
    (
        "pastry-fault",
        "The pastry looks limp",
        Some(
            "Pastry shrinks when it is stretched into the tin: press it gently into the corners rather than pulling the sides up, chill the lined case for 20 minutes before it goes in, and leave the overhang on until after baking, because the spare pastry holds the sides up while they set.",
        ),
    ),
    (
        "pastry-fault",
        "how do i avoid shrinking when blind baking",
        Some(
            "Pastry shrinks when it is stretched into the tin: press it gently into the corners rather than pulling the sides up, chill the lined case for 20 minutes before it goes in, and leave the overhang on until after baking, because the spare pastry holds the sides up while they set.",
        ),
    ),
    (
        "bread-rise",
        "how long does bread dough take to rise",
        Some(
            "Leave it somewhere around 25C for 1 to 2 hours, until the dough has doubled. Yeast eats the sugars in the flour and gives off gas, and the stretchy gluten holds it in bubbles, so warmth and time are the whole job. If it hasn't moved at all, the yeast is dead, so check the date and use water that's warm rather than hot.",
        ),
    ),
    (
        "bread-rise",
        "why won't my bread rise",
        Some(
            "Give it another hour somewhere around 25C, until it has doubled, and check the water was warm rather than hot and that the yeast is in date. If it still hasn't moved at all, the yeast is dead, so start again with fresh yeast.",
        ),
    ),
    (
        "bread-rise",
        "can i prove bread overnight",
        Some(
            "A night in the fridge is a slow prove on purpose, not a fault: shape it cold, then leave it an hour at room temperature before it goes in the oven. If it hasn't risen at all by the morning, the yeast is dead, so check the date.",
        ),
    ),
    (
        "bread-rise",
        "can i leave dough in the cold",
        Some(
            "A night in the fridge is a slow prove on purpose, not a fault: shape it cold, then leave it an hour at room temperature before it goes in the oven. If it hasn't risen at all by the morning, the yeast is dead, so check the date.",
        ),
    ),
    (
        "bread-rise",
        "do i have to prove my dough twice",
        Some(
            "Yes, twice: the first is done when the dough has doubled, and the second is the shaped loaf, about an hour, until it springs back slowly when you poke it. Skipping the second gives you a tight, close crumb.",
        ),
    ),
    (
        "bread-rise",
        "can i skip the second rise",
        Some(
            "Yes, twice: the first is done when the dough has doubled, and the second is the shaped loaf, about an hour, until it springs back slowly when you poke it. Skipping the second gives you a tight, close crumb.",
        ),
    ),
    (
        "bread-rise",
        "why is my sourdough flat",
        Some(
            "A sourdough loaf rises on the starter, not on warmth alone: a starter that doubles in 4 to 6 hours after a feed is strong enough to bake with, and one that only bubbles is not, so feed it twice a day for three days first. Then give the bulk 4 to 6 hours at room temperature with three or four sets of stretch and fold in the first two hours, and shape it with real tension. A flat loaf usually wants a stiffer dough and a longer, colder second prove.",
        ),
    ),
    ("bread-rise", "does a no yeast loaf need to rise", None),
    (
        "bread-rise",
        "why is my starter not rising after a night in the fridge",
        Some(
            "A sourdough loaf rises on the starter, not on warmth alone: a starter that doubles in 4 to 6 hours after a feed is strong enough to bake with, and one that only bubbles is not, so feed it twice a day for three days first. Then give the bulk 4 to 6 hours at room temperature with three or four sets of stretch and fold in the first two hours, and shape it with real tension. A flat loaf usually wants a stiffer dough and a longer, colder second prove.",
        ),
    ),
    (
        "bread-rise",
        "my bread hasnt risen",
        Some(
            "Give it another hour somewhere around 25C, until it has doubled, and check the water was warm rather than hot and that the yeast is in date. If it still hasn't moved at all, the yeast is dead, so start again with fresh yeast.",
        ),
    ),
    ("bread-rise", "yeast-free dough won't rise", None),
    (
        "flatbreads",
        "how do i make flatbread",
        Some(
            "Self-raising flour, a pinch of salt and enough yoghurt or water to make a soft dough: about 250g of flour to 150g of yoghurt, kneaded for a minute and rested for 10. Roll them thin, then cook each one in a dry, very hot frying pan for a minute a side until it blisters and puffs. Keep them under a tea towel while you do the rest, and eat them the same day; they freeze well between sheets of paper.",
        ),
    ),
    (
        "flatbreads",
        "how do i make naan",
        Some(
            "Self-raising flour, a pinch of salt and enough yoghurt or water to make a soft dough: about 250g of flour to 150g of yoghurt, kneaded for a minute and rested for 10. Roll them thin, then cook each one in a dry, very hot frying pan for a minute a side until it blisters and puffs. Keep them under a tea towel while you do the rest, and eat them the same day; they freeze well between sheets of paper.",
        ),
    ),
    (
        "flatbreads",
        "how do i make a tortilla wrap",
        Some(
            "Plain flour, salt, a little oil or lard and warm water, nothing else: about 250g of flour to 150ml of warm water and a tablespoon of oil. Knead for two minutes, rest the dough under a bowl for 20, then roll each one out as thin as you can. Cook in a dry, very hot pan for about 30 seconds a side - they want speckles, not browning, or they go crisp instead of foldable. Press the bubble down with a spatula as it puffs and stack them under a tea towel.",
        ),
    ),
    (
        "flatbreads",
        "chapati in a pan",
        Some(
            "Wholemeal flour and water, no raising agent and no yoghurt: about 200g of wholemeal or atta to 120ml of warm water, a pinch of salt and a smear of oil. Knead for five minutes until smooth, rest 20 minutes, roll thin, then cook in a dry pan for a minute a side until it speckles. Hold it over a flame with tongs for a couple of seconds at the end and it puffs up.",
        ),
    ),
    (
        "flatbreads",
        "what is the best way to make pitta",
        Some(
            "Pitta needs yeast, because the pocket is steam trapped inside the dough: 250g of strong white flour, 150ml of warm water, 5g of instant yeast, 5g of salt and a teaspoon of oil. Knead, prove an hour until doubled, then roll out about 1cm thick - too thin and there is no steam to open it. Bake on a tray as hot as the oven goes, 250C, for about five minutes, and they puff and hollow as they cool.",
        ),
    ),
    ("flatbreads", "making a spanish tortilla", None),
    ("flatbreads", "Spanish flour tortillas", None),
    ("flatbreads", "can i make potato omelette tortillas", None),
    (
        "flatbreads",
        "a pitas recipe",
        Some(
            "Pitta needs yeast, because the pocket is steam trapped inside the dough: 250g of strong white flour, 150ml of warm water, 5g of instant yeast, 5g of salt and a teaspoon of oil. Knead, prove an hour until doubled, then roll out about 1cm thick - too thin and there is no steam to open it. Bake on a tray as hot as the oven goes, 250C, for about five minutes, and they puff and hollow as they cool.",
        ),
    ),
    (
        "flatbreads",
        "how do i make a roti",
        Some(
            "Wholemeal flour and water, no raising agent and no yoghurt: about 200g of wholemeal or atta to 120ml of warm water, a pinch of salt and a smear of oil. Knead for five minutes until smooth, rest 20 minutes, roll thin, then cook in a dry pan for a minute a side until it speckles. Hold it over a flame with tongs for a couple of seconds at the end and it puffs up.",
        ),
    ),
    (
        "flatbreads",
        "quick flour tortilla and chapati",
        Some(
            "Plain flour, salt, a little oil or lard and warm water, nothing else: about 250g of flour to 150ml of warm water and a tablespoon of oil. Knead for two minutes, rest the dough under a bowl for 20, then roll each one out as thin as you can. Cook in a dry, very hot pan for about 30 seconds a side - they want speckles, not browning, or they go crisp instead of foldable. Press the bubble down with a spatula as it puffs and stack them under a tea towel.",
        ),
    ),
    (
        "pasta-cook",
        "how should i cook pasta",
        Some(
            "Use a big pan of well-salted water, about 10g of salt per litre, and cook the pasta one minute less than the packet says. Keep a mug of the cooking water: a splash of it makes any sauce cling to the pasta.",
        ),
    ),
    (
        "pasta-cook",
        "my pasta is sticking together",
        Some(
            "Use a big pan of well-salted water, about 10g of salt per litre, and cook the pasta one minute less than the packet says. Keep a mug of the cooking water: a splash of it makes any sauce cling to the pasta. Clumping is the pasta sitting still in water that is not deep or hot enough, so stir it for the first minute and again every couple of minutes, and leave the oil out: it stops the sauce clinging.",
        ),
    ),
    (
        "pasta-cook",
        "how do i stop pasta clumping",
        Some(
            "Use a big pan of well-salted water, about 10g of salt per litre, and cook the pasta one minute less than the packet says. Keep a mug of the cooking water: a splash of it makes any sauce cling to the pasta. Clumping is the pasta sitting still in water that is not deep or hot enough, so stir it for the first minute and again every couple of minutes, and leave the oil out: it stops the sauce clinging.",
        ),
    ),
    (
        "pasta-cook",
        "spaghetti stuck together",
        Some(
            "Use a big pan of well-salted water, about 10g of salt per litre, and cook the pasta one minute less than the packet says. Keep a mug of the cooking water: a splash of it makes any sauce cling to the pasta. Clumping is the pasta sitting still in water that is not deep or hot enough, so stir it for the first minute and again every couple of minutes, and leave the oil out: it stops the sauce clinging.",
        ),
    ),
    (
        "pasta-cook",
        "what is clump pasta",
        Some(
            "Use a big pan of well-salted water, about 10g of salt per litre, and cook the pasta one minute less than the packet says. Keep a mug of the cooking water: a splash of it makes any sauce cling to the pasta. Clumping is the pasta sitting still in water that is not deep or hot enough, so stir it for the first minute and again every couple of minutes, and leave the oil out: it stops the sauce clinging.",
        ),
    ),
    (
        "pasta-cook",
        "how do i cook dried pasta",
        Some(
            "Use a big pan of well-salted water, about 10g of salt per litre, and cook the pasta one minute less than the packet says. Keep a mug of the cooking water: a splash of it makes any sauce cling to the pasta.",
        ),
    ),
    (
        "pasta-cook",
        "my spaghetti is lumpy",
        Some(
            "Use a big pan of well-salted water, about 10g of salt per litre, and cook the pasta one minute less than the packet says. Keep a mug of the cooking water: a splash of it makes any sauce cling to the pasta.",
        ),
    ),
    (
        "soup-flavour",
        "how do i make a vegetable soup",
        Some(
            "Soften an onion and a carrot in butter with a pinch of salt, add whatever vegetables or pulses you have and enough stock to cover them, and simmer 20 minutes. Blend it smooth or leave it chunky, taste it, and finish with salt, pepper and a squeeze of lemon.",
        ),
    ),
    (
        "soup-flavour",
        "how do i give soup more flavour",
        Some(
            "Add salt first, then a squeeze of lemon or a splash of vinegar: most dull soup is short of both, not of spice. After that something savoury - a stock cube, a spoon of miso, a parmesan rind or a knob of butter - and roast the vegetables next time rather than boiling them.",
        ),
    ),
    (
        "soup-flavour",
        "how to improve my boring soup",
        Some(
            "Add salt first, then a squeeze of lemon or a splash of vinegar: most dull soup is short of both, not of spice. After that something savoury - a stock cube, a spoon of miso, a parmesan rind or a knob of butter - and roast the vegetables next time rather than boiling them.",
        ),
    ),
    (
        "soup-flavour",
        "the soup tastes flavorless",
        Some(
            "Soften an onion and a carrot in butter with a pinch of salt, add whatever vegetables or pulses you have and enough stock to cover them, and simmer 20 minutes. Blend it smooth or leave it chunky, taste it, and finish with salt, pepper and a squeeze of lemon.",
        ),
    ),
    (
        "soup-flavour",
        "how do i make my veggie soup more interesting",
        Some(
            "Add salt first, then a squeeze of lemon or a splash of vinegar: most dull soup is short of both, not of spice. After that something savoury - a stock cube, a spoon of miso, a parmesan rind or a knob of butter - and roast the vegetables next time rather than boiling them.",
        ),
    ),
    (
        "soup-flavour",
        "how do i make soup better",
        Some(
            "Add salt first, then a squeeze of lemon or a splash of vinegar: most dull soup is short of both, not of spice. After that something savoury - a stock cube, a spoon of miso, a parmesan rind or a knob of butter - and roast the vegetables next time rather than boiling them.",
        ),
    ),
    (
        "soup-flavour",
        "how can a person make an uninteresting soup",
        Some(
            "Soften an onion and a carrot in butter with a pinch of salt, add whatever vegetables or pulses you have and enough stock to cover them, and simmer 20 minutes. Blend it smooth or leave it chunky, taste it, and finish with salt, pepper and a squeeze of lemon.",
        ),
    ),
];
