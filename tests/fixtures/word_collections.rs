//! Original PyBot behavior for rows lowered by word collection operations.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub bool,
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "cooking",
        "a-batter-that-sticks",
        "pancake sticks when i flip it",
        true,
        Some(
            "A batter that sticks wants a hotter pan and less fat than you would think: heat a non-stick or well-seasoned pan on a medium heat for a couple of minutes, wipe it with a little oil on kitchen paper, and pour when a drop of batter sizzles at once. Wait until the top bubbles and the edges look dry before you turn it, because lifting one early is what tears it, and thin a thick batter with a splash of milk, or it sits in the middle of the pan and grips.",
        ),
    ),
    Case(
        "cooking",
        "a-batter-that-sticks",
        "batter sticks to my stainless pan",
        true,
        Some(
            "A batter that sticks wants a hotter pan and less fat than you would think: heat a non-stick or well-seasoned pan on a medium heat for a couple of minutes, wipe it with a little oil on kitchen paper, and pour when a drop of batter sizzles at once. Wait until the top bubbles and the edges look dry before you turn it, because lifting one early is what tears it, and thin a thick batter with a splash of milk, or it sits in the middle of the pan and grips.",
        ),
    ),
    Case(
        "cooking",
        "a-batter-that-sticks",
        "the batter sticks in the middle",
        true,
        Some(
            "A batter that sticks wants a hotter pan and less fat than you would think: heat a non-stick or well-seasoned pan on a medium heat for a couple of minutes, wipe it with a little oil on kitchen paper, and pour when a drop of batter sizzles at once. Wait until the top bubbles and the edges look dry before you turn it, because lifting one early is what tears it, and thin a thick batter with a splash of milk, or it sits in the middle of the pan and grips.",
        ),
    ),
    Case(
        "cooking",
        "a-batter-that-sticks",
        "my omelette sticks",
        true,
        Some(
            "A batter that sticks wants a hotter pan and less fat than you would think: heat a non-stick or well-seasoned pan on a medium heat for a couple of minutes, wipe it with a little oil on kitchen paper, and pour when a drop of batter sizzles at once. Wait until the top bubbles and the edges look dry before you turn it, because lifting one early is what tears it, and thin a thick batter with a splash of milk, or it sits in the middle of the pan and grips.",
        ),
    ),
    Case(
        "cooking",
        "cake-stuck-tin",
        "cake stuck to the tin while warm",
        true,
        Some(
            "A cake sticks to the tin when it was greased unevenly, turned out before it had firmed, or baked in a tin that was not lined. Line it with a strip of baking paper across the base and up two sides and it lifts out whole. Run a thin knife round the edge while it is still warm, wait ten minutes, then turn it out onto a rack - and if it does crack, stick it back together with a little jam or buttercream and nobody will know.",
        ),
    ),
    Case(
        "cooking",
        "cake-stuck-tin",
        "how to get a cake out of a bundt tin",
        true,
        Some(
            "A cake sticks to the tin when it was greased unevenly, turned out before it had firmed, or baked in a tin that was not lined. Line it with a strip of baking paper across the base and up two sides and it lifts out whole. Run a thin knife round the edge while it is still warm, wait ten minutes, then turn it out onto a rack - and if it does crack, stick it back together with a little jam or buttercream and nobody will know.",
        ),
    ),
    Case(
        "cooking",
        "cake-stuck-tin",
        "my loaf sticks to the tin",
        true,
        Some(
            "A loaf sticks to the tin when the tin was bare or the dough was wet: oil the tin all over, and line the base and the two long sides with a strip of baking paper that overhangs the top, so you lift the loaf out by the paper. Run a knife round the edges while it is still warm, wait ten minutes, then turn it out and tap the base - and let it cool on its side so the crust does not go soft.",
        ),
    ),
    Case(
        "cooking",
        "cake-stuck-tin",
        "the cake was cold when i tried to take it out",
        true,
        Some(
            "A cake sticks to the tin when it was greased unevenly, turned out before it had firmed, or baked in a tin that was not lined. Line it with a strip of baking paper across the base and up two sides and it lifts out whole. Run a thin knife round the edge while it is still warm, wait ten minutes, then turn it out onto a rack - and if it does crack, stick it back together with a little jam or buttercream and nobody will know.",
        ),
    ),
    Case(
        "money",
        "credit-score",
        "what is a credit score",
        true,
        Some(
            "Your credit score is a number worked out from your credit report: what you owe, how much of each card limit you use, whether payments arrive on time and how long you have held accounts. Experian, Equifax and TransUnion each score it on a different scale, and a lender reads the report itself rather than the score, so the free report from all three is worth more than the number.",
        ),
    ),
    Case(
        "money",
        "credit-score",
        "how can i improve my credit rating",
        true,
        Some(
            "Get your report free from Experian, Equifax or TransUnion, correct anything wrong on it, register on the electoral roll and set up a direct debit for at least the minimum on every card. Keeping card balances under about 30% of the limit and not applying for several things at once does most of the rest.",
        ),
    ),
    Case(
        "money",
        "credit-score",
        "what drops my credit score",
        true,
        Some(
            "A drop is nearly always one of five things: a late or missed payment, a card balance over about 30% of its limit, a new application and the search that comes with it, an account you closed, or a mistake on the file. Then get the report free from Experian, Equifax or TransUnion, dispute anything wrong, and it usually recovers within a few months of on-time payments.",
        ),
    ),
    Case(
        "money",
        "credit-score",
        "what does credit score look for",
        true,
        Some(
            "There is no one good number, because the three agencies score on different scales: Experian runs from 0 to 999 and puts roughly 881 to 960 in its good band, Equifax scores out of 1,000 and TransUnion out of 710. What a lender weighs is the report behind the number - payments on time, card balances under about 30% of the limit and no run of new applications - so the band matters less than those three.",
        ),
    ),
    Case(
        "pets",
        "dog-kit",
        "what do i buy before my new rescue dog comes home",
        true,
        Some(
            "A bed it can stretch out flat in, two bowls, a collar and a lead, a harness for walks where it pulls, poo bags and food of the brand it is already on, plus a coat if it has a thin coat like a greyhound. A rescue usually comes with its own lead and a bag of food, so ask the centre what it has been eating and keep to that for the first fortnight, and budget about £1,500 to £2,000 a year for a dog before any vet bill.",
        ),
    ),
    Case(
        "pets",
        "dog-kit",
        "new puppy supplies",
        true,
        Some(
            "A bed it can stretch out flat in, two bowls, a collar and a lead, a harness for walks where it pulls, poo bags and food of the brand it is already on, plus a coat if it has a thin coat like a greyhound. A rescue usually comes with its own lead and a bag of food, so ask the centre what it has been eating and keep to that for the first fortnight, and budget about £1,500 to £2,000 a year for a dog before any vet bill.",
        ),
    ),
    Case(
        "pets",
        "dog-kit",
        "what dog kit do i need in a small flat",
        true,
        Some(
            "A bed it can stretch out flat in, two bowls, a collar and a lead, a harness for walks where it pulls, poo bags and food of the brand it is already on, plus a coat if it has a thin coat like a greyhound. A rescue usually comes with its own lead and a bag of food, so ask the centre what it has been eating and keep to that for the first fortnight, and budget about £1,500 to £2,000 a year for a dog before any vet bill.",
        ),
    ),
    Case(
        "pets",
        "dog-kit",
        "taking my first dog home",
        true,
        Some(
            "Lucky you. The first things to have in are a bed it can stretch out flat in and two bowls, a collar and a lead with a tag on it, poo bags, a harness for walks where it pulls, and the food it is already on so its stomach stays settled. Register with a vet in the first week, and if it is a rescue ask the centre what it has been eating and how much it has been walked.",
        ),
    ),
    Case(
        "money",
        "energy-tariff",
        "should i get a fixed or variable energy tariff",
        true,
        Some(
            "Worth it only if the fix comes in under what the cap is likely to do over its term: a fix buys certainty, so on a falling cap the certainty is all you are paying for and the variable rate wins on price. Compare the unit rate and the standing charge against today's cap rather than the monthly estimate, and check the exit fee before signing.",
        ),
    ),
    Case(
        "money",
        "energy-tariff",
        "what is a standing charge",
        true,
        Some(
            "The price cap is a limit on the unit rate and standing charge of a standard variable tariff, set quarterly, so it moves with the market; a fixed tariff holds your rate for its term whether prices go up or down. Compare the unit rate and the standing charge rather than the monthly estimate, and check the exit fee before signing a fix.",
        ),
    ),
    Case(
        "money",
        "energy-tariff",
        "how does economy seven work",
        true,
        Some(
            "The price cap is a limit on the unit rate and standing charge of a standard variable tariff, set quarterly, so it moves with the market; a fixed tariff holds your rate for its term whether prices go up or down. Compare the unit rate and the standing charge rather than the monthly estimate, and check the exit fee before signing a fix.",
        ),
    ),
    Case(
        "money",
        "energy-tariff",
        "switch to a cheaper energy tariff",
        true,
        Some(
            "The price cap is a limit on the unit rate and standing charge of a standard variable tariff, set quarterly, so it moves with the market; a fixed tariff holds your rate for its term whether prices go up or down. Compare the unit rate and the standing charge rather than the monthly estimate, and check the exit fee before signing a fix.",
        ),
    ),
    Case(
        "cooking",
        "frozen-pizza",
        "how do i cook a frozen pizza",
        true,
        Some(
            "Frozen pizza wants a fully heated oven, not a cold one: 200C (180C fan, gas mark 6) for 12 to 15 minutes, straight on the shelf or a hot tray, until the cheese bubbles and the base lifts. A cold oven gives you a soggy middle and a pale top, and an air fryer does a small one in 8 to 10 minutes at 180C.",
        ),
    ),
    Case(
        "cooking",
        "frozen-pizza",
        "cooking a pizza from scratch",
        false,
        Some(
            "Frozen pizza wants a fully heated oven, not a cold one: 200C (180C fan, gas mark 6) for 12 to 15 minutes, straight on the shelf or a hot tray, until the cheese bubbles and the base lifts. A cold oven gives you a soggy middle and a pale top, and an air fryer does a small one in 8 to 10 minutes at 180C.",
        ),
    ),
    Case(
        "cooking",
        "frozen-pizza",
        "how hot should a frozen margherita be",
        true,
        Some(
            "Frozen pizza wants a fully heated oven, not a cold one: 200C (180C fan, gas mark 6) for 12 to 15 minutes, straight on the shelf or a hot tray, until the cheese bubbles and the base lifts. A cold oven gives you a soggy middle and a pale top, and an air fryer does a small one in 8 to 10 minutes at 180C.",
        ),
    ),
    Case(
        "cooking",
        "frozen-pizza",
        "freeze a pizza",
        true,
        Some(
            "Frozen pizza wants a fully heated oven, not a cold one: 200C (180C fan, gas mark 6) for 12 to 15 minutes, straight on the shelf or a hot tray, until the cheese bubbles and the base lifts. A cold oven gives you a soggy middle and a pale top, and an air fryer does a small one in 8 to 10 minutes at 180C.",
        ),
    ),
    Case(
        "cleaning",
        "grout-sealing",
        "should i seal my bathroom grout",
        true,
        Some(
            "Worth it on cement grout in a shower or bathroom, less so in a dry hall: a penetrating sealer keeps water and stains out of the joints for a year or two, though it does not make the grout waterproof, and epoxy grout needs none. Clean it and let it dry for a day first, or the sealer locks the dirt in.",
        ),
    ),
    Case(
        "cleaning",
        "grout-sealing",
        "sealing grout next to my patio brickwork",
        true,
        Some(
            "Clean the grout and let it dry for at least a day, then work a penetrating sealer into the joints with a small brush or applicator, not a sponge. Wipe the face of the tiles with a damp cloth after ten minutes so no film dries on them, and keep water off it for 24 hours. Reapply every year or two in a shower.",
        ),
    ),
    Case(
        "cleaning",
        "grout-sealing",
        "the grout around my tiles gets wet",
        true,
        Some(
            "Clean the grout and let it dry for at least a day, then work a penetrating sealer into the joints with a small brush or applicator, not a sponge. Wipe the face of the tiles with a damp cloth after ten minutes so no film dries on them, and keep water off it for 24 hours. Reapply every year or two in a shower.",
        ),
    ),
    Case(
        "cleaning",
        "grout-sealing",
        "seal coloured grout",
        true,
        Some(
            "Clean the grout and let it dry for at least a day, then work a penetrating sealer into the joints with a small brush or applicator, not a sponge. Wipe the face of the tiles with a damp cloth after ten minutes so no film dries on them, and keep water off it for 24 hours. Reapply every year or two in a shower.",
        ),
    ),
    Case(
        "science",
        "gulf-stream-what",
        "what is the gulf stream",
        true,
        Some(
            "The Gulf Stream is a fast warm current that carries water from the Gulf of Mexico up the east coast of America and across the Atlantic. Its heat is why Britain and Ireland are milder than the same latitude in Canada.",
        ),
    ),
    Case(
        "science",
        "gulf-stream-what",
        "how does the jet stream work",
        false,
        Some(
            "The Gulf Stream is a fast warm current that carries water from the Gulf of Mexico up the east coast of America and across the Atlantic. Its heat is why Britain and Ireland are milder than the same latitude in Canada.",
        ),
    ),
    Case(
        "science",
        "gulf-stream-what",
        "what happens if gulf stream stops",
        true,
        Some(
            "The Gulf Stream is a fast warm current that carries water from the Gulf of Mexico up the east coast of America and across the Atlantic. Its heat is why Britain and Ireland are milder than the same latitude in Canada.",
        ),
    ),
    Case(
        "science",
        "gulf-stream-what",
        "how far is the gulf stream from uk",
        true,
        Some(
            "The Gulf Stream is a fast warm current that carries water from the Gulf of Mexico up the east coast of America and across the Atlantic. Its heat is why Britain and Ireland are milder than the same latitude in Canada.",
        ),
    ),
    Case(
        "cooking",
        "herbs-keep",
        "keep fresh parsley",
        true,
        Some(
            "Stand soft herbs in a jar with an inch of water, loosely covered with a bag, or wrap them in damp kitchen paper in the fridge. Basil hates the cold, so keep that one on the windowsill instead.",
        ),
    ),
    Case(
        "cooking",
        "herbs-keep",
        "how to store mint",
        true,
        Some(
            "Stand soft herbs in a jar with an inch of water, loosely covered with a bag, or wrap them in damp kitchen paper in the fridge. Basil hates the cold, so keep that one on the windowsill instead.",
        ),
    ),
    Case(
        "cooking",
        "herbs-keep",
        "fresh basil in the fridge turns black",
        true,
        Some(
            "Stand soft herbs in a jar with an inch of water, loosely covered with a bag, or wrap them in damp kitchen paper in the fridge. Basil hates the cold, so keep that one on the windowsill instead.",
        ),
    ),
    Case(
        "cooking",
        "herbs-keep",
        "dried herbs shelf life",
        true,
        Some(
            "Stand soft herbs in a jar with an inch of water, loosely covered with a bag, or wrap them in damp kitchen paper in the fridge. Basil hates the cold, so keep that one on the windowsill instead.",
        ),
    ),
    Case(
        "cooking",
        "herbs-keep",
        "keep dried rosemary",
        true,
        Some(
            "Stand soft herbs in a jar with an inch of water, loosely covered with a bag, or wrap them in damp kitchen paper in the fridge. Basil hates the cold, so keep that one on the windowsill instead.",
        ),
    ),
    Case(
        "house_odds",
        "hood-grease",
        "clean the grease filter in a cooker hood",
        true,
        Some(
            "The grease goes in the filter: the metal ones lift out, go in the dishwasher or a hot soapy sink with soda crystals for half an hour once a month, and that is what stops the hood getting weak and smelly. The outside wipes down with hot water and washing-up liquid, top to bottom so the grease does not run onto what you have already cleaned, and a paper charcoal filter cannot be washed - it is a new one every three to six months.",
        ),
    ),
    Case(
        "house_odds",
        "hood-grease",
        "what about my extractor hood grease filter",
        true,
        Some(
            "The grease goes in the filter: the metal ones lift out, go in the dishwasher or a hot soapy sink with soda crystals for half an hour once a month, and that is what stops the hood getting weak and smelly. The outside wipes down with hot water and washing-up liquid, top to bottom so the grease does not run onto what you have already cleaned, and a paper charcoal filter cannot be washed - it is a new one every three to six months.",
        ),
    ),
    Case(
        "house_odds",
        "hood-grease",
        "hood has a carbon filter",
        true,
        Some(
            "The grease goes in the filter: the metal ones lift out, go in the dishwasher or a hot soapy sink with soda crystals for half an hour once a month, and that is what stops the hood getting weak and smelly. The outside wipes down with hot water and washing-up liquid, top to bottom so the grease does not run onto what you have already cleaned, and a paper charcoal filter cannot be washed - it is a new one every three to six months.",
        ),
    ),
    Case(
        "house_odds",
        "hood-grease",
        "my range hood has a metal filter",
        true,
        Some(
            "The grease goes in the filter: the metal ones lift out, go in the dishwasher or a hot soapy sink with soda crystals for half an hour once a month, and that is what stops the hood getting weak and smelly. The outside wipes down with hot water and washing-up liquid, top to bottom so the grease does not run onto what you have already cleaned, and a paper charcoal filter cannot be washed - it is a new one every three to six months.",
        ),
    ),
    Case(
        "cars",
        "mot-fail",
        "what happens if i fail mot with dangerous fault",
        true,
        Some(
            "Sorry to hear it, and it is usually fixable. Read the fail sheet: 'dangerous' means don't drive it at all, a 'major' needs fixing before it can pass, and an advisory is a warning for next time. Many garages do the repair and the free partial retest in one go if you leave the car with them.",
        ),
    ),
    Case(
        "cars",
        "mot-fail",
        "can i drive after mot failure with a major fault",
        true,
        Some(
            "Sorry to hear it, and it is usually fixable. Read the fail sheet: 'dangerous' means don't drive it at all, a 'major' needs fixing before it can pass, and an advisory is a warning for next time. Many garages do the repair and the free partial retest in one go if you leave the car with them.",
        ),
    ),
    Case(
        "cars",
        "mot-fail",
        "mot fail retest",
        true,
        Some(
            "Sorry to hear it, and it is usually fixable. Read the fail sheet: 'dangerous' means don't drive it at all, a 'major' needs fixing before it can pass, and an advisory is a warning for next time. Many garages do the repair and the free partial retest in one go if you leave the car with them.",
        ),
    ),
    Case(
        "cars",
        "mot-fail",
        "my car failed its mot on brakes",
        true,
        Some(
            "Sorry to hear it, and it is usually fixable. Read the fail sheet: 'dangerous' means don't drive it at all, a 'major' needs fixing before it can pass, and an advisory is a warning for next time. Many garages do the repair and the free partial retest in one go if you leave the car with them.",
        ),
    ),
    Case(
        "cooking",
        "pancakes",
        "how do i make pancakes",
        true,
        Some(
            "For about 12 thin pancakes, whisk 100g plain flour, 2 eggs and 300ml milk with a pinch of salt, then let the batter rest 20 minutes. Cook them in a hot non-stick pan with a little butter, about a minute a side, until bubbles pop on top and the edges look dry. The rest is what stops them tearing; American ones want a teaspoon of baking powder and a thicker batter. Flat ones are a batter that was too thin, or an old raising agent.",
        ),
    ),
    Case(
        "cooking",
        "pancakes",
        "pancakes without egg",
        true,
        Some(
            "For about 12 thin pancakes, whisk 100g plain flour, 2 eggs and 300ml milk with a pinch of salt, then let the batter rest 20 minutes. Cook them in a hot non-stick pan with a little butter, about a minute a side, until bubbles pop on top and the edges look dry. The rest is what stops them tearing; American ones want a teaspoon of baking powder and a thicker batter. Flat ones are a batter that was too thin, or an old raising agent.",
        ),
    ),
    Case(
        "cooking",
        "pancakes",
        "my pancakes stick to the pan",
        true,
        Some(
            "For about 12 thin pancakes, whisk 100g plain flour, 2 eggs and 300ml milk with a pinch of salt, then let the batter rest 20 minutes. Cook them in a hot non-stick pan with a little butter, about a minute a side, until bubbles pop on top and the edges look dry. The rest is what stops them tearing; American ones want a teaspoon of baking powder and a thicker batter. Flat ones are a batter that was too thin, or an old raising agent.",
        ),
    ),
    Case(
        "cooking",
        "pancakes",
        "pancakes that are fluffy and thick",
        true,
        Some(
            "For a dozen thick American pancakes, whisk 150g plain flour, 2 teaspoons of baking powder, a tablespoon of sugar, 2 eggs and 250ml milk into a thick batter that holds its shape, and let it rest 10 minutes. Cook them in a buttered pan on a medium heat, 2 minutes a side, and turn them once, when bubbles pop and stay open. The baking powder is what lifts them; for thin pancakes leave it out and use 300ml milk.",
        ),
    ),
    Case(
        "pets",
        "puppy-kit",
        "what should i buy for a new puppy",
        true,
        Some(
            "For a puppy: a bed, two bowls, a crate or pen, a collar and lead, an adjustable harness, safe chew toys, poo bags and puppy food of the brand it's already on. Book the first vet check for the first week and ask about vaccinations and worming then.",
        ),
    ),
    Case(
        "pets",
        "puppy-kit",
        "kit for a puppy coming home",
        true,
        Some(
            "For a puppy: a bed, two bowls, a crate or pen, a collar and lead, an adjustable harness, safe chew toys, poo bags and puppy food of the brand it's already on. Book the first vet check for the first week and ask about vaccinations and worming then.",
        ),
    ),
    Case(
        "pets",
        "puppy-kit",
        "new puppy harness and leash",
        true,
        Some(
            "Lucky you. Before a puppy comes home: a bed, two bowls, a crate or pen, a collar and lead, an adjustable harness, safe chew toys, poo bags and puppy food of the brand it is already on. Book the first vet check for its first week and ask about vaccinations and worming then.",
        ),
    ),
    Case(
        "pets",
        "puppy-kit",
        "bring home a dog",
        true,
        Some(
            "Lucky you. Before a puppy comes home: a bed, two bowls, a crate or pen, a collar and lead, an adjustable harness, safe chew toys, poo bags and puppy food of the brand it is already on. Book the first vet check for its first week and ask about vaccinations and worming then.",
        ),
    ),
    Case(
        "pets",
        "rabbit-weather",
        "can my rabbit stay outside in snow",
        true,
        Some(
            "Keep the hutch out of the wind and rain, with a dry, deep bed of hay or straw to burrow into, and a cover or an old duvet over the mesh at night, leaving the front open for air. Two rabbits huddle and keep each other warm, so a single one needs the extra bedding. Give warm water rather than cold, and check the bottle has not frozen.",
        ),
    ),
    Case(
        "pets",
        "rabbit-weather",
        "rabbit hutches in hot weather",
        true,
        Some(
            "Shade, a second bowl of water and a frozen bottle of water wrapped in a towel for her to lean against, with the hutch moved out of the sun for the day. Rabbits cannot sweat and over 25C is already risky, and you must never bath one to cool her down: a damp bottom in summer is how flystrike starts. A rabbit stretched out and drooling, or not eating in the heat, needs the vet that day.",
        ),
    ),
    Case(
        "pets",
        "rabbit-weather",
        "rain and my rabbit gets wet outside",
        true,
        Some(
            "Shade, a second bowl of water and a frozen bottle of water wrapped in a towel for her to lean against, with the hutch moved out of the sun for the day. Rabbits cannot sweat and over 25C is already risky, and you must never bath one to cool her down: a damp bottom in summer is how flystrike starts. A rabbit stretched out and drooling, or not eating in the heat, needs the vet that day.",
        ),
    ),
    Case(
        "pets",
        "rabbit-weather",
        "a rabbit in a cold shed",
        true,
        Some(
            "Keep the hutch out of the wind and rain, with a dry, deep bed of hay or straw to burrow into, and a cover or an old duvet over the mesh at night, leaving the front open for air. Two rabbits huddle and keep each other warm, so a single one needs the extra bedding. Give warm water rather than cold, and check the bottle has not frozen.",
        ),
    ),
    Case(
        "cooking",
        "sweet-potato",
        "how do i roast sweet potatoes",
        true,
        Some(
            "For wedges, cut them into 2cm pieces, toss them in oil, salt and paprika and roast at 200C (180C fan) for 25 to 30 minutes. Whole, they bake in about 45 minutes at 200C, or 8 minutes in the microwave for one on its own. Boiled for mash, chunks take 12 to 15 minutes.",
        ),
    ),
    Case(
        "cooking",
        "sweet-potato",
        "sweet potato in the microwave",
        true,
        Some(
            "One medium sweet potato, pricked all over with a fork, takes 5 to 8 minutes on high: give it 5, turn it, then a minute at a time until a knife slides in with no resistance. Two take 10 to 12 minutes, and chunks for mash take 8. It steams in there rather than crisping, so give the skin 5 minutes in a hot oven or a dry pan afterwards if you want it crisp.",
        ),
    ),
    Case(
        "cooking",
        "sweet-potato",
        "can i boil sweet potatoes",
        true,
        Some(
            "Boil peeled chunks for 12 to 15 minutes, until a knife slides in with no resistance: sweet potato softens faster than white potato, so test it at 10. Drain and steam it dry for a minute, then mash it with butter and a pinch of salt and go easy on the milk, because it is wetter than white potato.",
        ),
    ),
    Case(
        "cooking",
        "sweet-potato",
        "crispy sweet potato chips in the air fryer",
        true,
        Some(
            "In the air fryer, wedges cut into 2cm pieces take 15 to 18 minutes at 190C, shaken once halfway: toss them in a little oil, salt and paprika first and do not crowd the basket, or they steam instead of crisping. Whole ones take 35 to 40 minutes at 180C, and fries cut thin take 12 to 15 minutes.",
        ),
    ),
    Case(
        "home",
        "tap-pressure-low",
        "tap water flow is weak in the kitchen",
        true,
        Some(
            "If it is one tap, it is nearly always scale in the spout: the mesh aerator unscrews by hand, so soak it in vinegar and the flow usually comes back. If it is every tap, check the stopcock under the kitchen sink is fully open, then ask a neighbour - if theirs is weak too it is the mains and you ring the water company. A combi boiler reads its pressure on the gauge under the front panel, 1 to 1.5 bar when cold, and low pressure there means weak hot water everywhere, which you top up with the filling loop. A shower that is only weak when hot has a scaled head or a tired hose.",
        ),
    ),
    Case(
        "home",
        "tap-pressure-low",
        "my shower has low water pressure",
        true,
        Some(
            "Scale in the shower head is the first thing to check: unscrew it, pick the grit out of the holes and soak it in vinegar or citric acid overnight, and clean the small filter where the hose joins the outlet. Check the head is not clicked onto an economy or pause stop too. If the other taps are weak as well it is the supply instead: on a combi it is whatever the mains gives, and on a tank it is the pump or the head of water, so look at the boiler gauge (1 to 1.5 bar cold) and the stopcock before you buy a new head.",
        ),
    ),
    Case(
        "home",
        "tap-pressure-low",
        "does a showerhead reduce water pressure",
        true,
        Some(
            "Scale in the shower head is the first thing to check: unscrew it, pick the grit out of the holes and soak it in vinegar or citric acid overnight, and clean the small filter where the hose joins the outlet. Check the head is not clicked onto an economy or pause stop too. If the other taps are weak as well it is the supply instead: on a combi it is whatever the mains gives, and on a tank it is the pump or the head of water, so look at the boiler gauge (1 to 1.5 bar cold) and the stopcock before you buy a new head.",
        ),
    ),
    Case(
        "home",
        "tap-pressure-low",
        "bathroom faucet slow and kitchen tap fine",
        true,
        Some(
            "If it is one tap, it is nearly always scale in the spout: the mesh aerator unscrews by hand, so soak it in vinegar and the flow usually comes back. If it is every tap, check the stopcock under the kitchen sink is fully open, then ask a neighbour - if theirs is weak too it is the mains and you ring the water company. A combi boiler reads its pressure on the gauge under the front panel, 1 to 1.5 bar when cold, and low pressure there means weak hot water everywhere, which you top up with the filling loop. A shower that is only weak when hot has a scaled head or a tired hose.",
        ),
    ),
    Case(
        "science",
        "weathering-what",
        "what is weathering of rocks",
        true,
        Some(
            "Weathering is rock being broken up where it sits, by rain, frost, wind, roots or weak acid, without being carried away. Erosion is the next step: the loosened pieces being moved somewhere else.",
        ),
    ),
    Case(
        "science",
        "weathering-what",
        "weather are clouds and rain erosion",
        true,
        Some(
            "Weathering is rock being broken up where it sits, by rain, frost, wind, roots or weak acid, without being carried away. Erosion is the next step: the loosened pieces being moved somewhere else.",
        ),
    ),
    Case(
        "science",
        "weathering-what",
        "what is rock weathering and erosion",
        true,
        Some(
            "Weathering is rock being broken up where it sits, by rain, frost, wind, roots or weak acid, without being carried away. Erosion is the next step: the loosened pieces being moved somewhere else.",
        ),
    ),
    Case(
        "science",
        "weathering-what",
        "how will the weather be",
        false,
        Some(
            "Weathering is rock being broken up where it sits, by rain, frost, wind, roots or weak acid, without being carried away. Erosion is the next step: the loosened pieces being moved somewhere else.",
        ),
    ),
    Case(
        "cooking",
        "coffee-jitters",
        "is 4pm too late for coffee",
        true,
        Some(
            "Yes, too late if you want to sleep at a normal hour: half the caffeine is still there five or six hours later, so a 4pm coffee can still be working at midnight. Keep to one or two cups a day, stop by early afternoon, make the afternoon one decaf, and eat something with it. Palpitations, shakiness or panic that come with caffeine are worth mentioning to a GP, especially with chest pain or breathlessness.",
        ),
    ),
    Case(
        "cooking",
        "coffee-jitters",
        "does caffeine keep me awake",
        true,
        Some(
            "It is the caffeine: it peaks about half an hour after the cup and takes five or six hours to leave your system, so a 4pm coffee can still be in you at midnight. Keep to one or two cups, stop by early afternoon, make the afternoon one decaf, and eat something with it. Palpitations, shakiness or panic that come with caffeine are worth mentioning to a GP, especially with chest pain or breathlessness.",
        ),
    ),
    Case(
        "cooking",
        "coffee-jitters",
        "can i have caffeine before bed",
        true,
        Some(
            "Stop by early afternoon, and count back: caffeine's half-life is about five hours, so half a 4pm cup is still in you at 9pm and a quarter of it at 2am. If you are in bed at eleven, a last full cup around 2pm leaves almost nothing by then; after that make it decaf, which has about 2mg a cup against 80.",
        ),
    ),
    Case(
        "technology",
        "dark-mode",
        "turn on dark mode at night",
        true,
        Some(
            "On an iPhone it is Settings > Display & Brightness > Dark, and Android has it under Settings > Display; both can switch themselves on at sunset under Options or Schedule. There is a Dark reader for websites too, in Safari's page menu or Chrome's Settings > Appearance.",
        ),
    ),
    Case(
        "technology",
        "dark-mode",
        "how do i change from dark mode",
        true,
        Some(
            "On an iPhone it is Settings > Display & Brightness > Dark, and Android has it under Settings > Display; both can switch themselves on at sunset under Options or Schedule. There is a Dark reader for websites too, in Safari's page menu or Chrome's Settings > Appearance.",
        ),
    ),
    Case(
        "technology",
        "dark-mode",
        "my screen is dark even in daytime",
        true,
        Some(
            "On an iPhone it is Settings > Display & Brightness > Dark, and Android has it under Settings > Display; both can switch themselves on at sunset under Options or Schedule. There is a Dark reader for websites too, in Safari's page menu or Chrome's Settings > Appearance.",
        ),
    ),
    Case(
        "explains",
        "explain-tv-screen",
        "how do oled screens work",
        true,
        Some(
            "A modern screen is a grid of millions of pixels, each with red, green and blue lights under it, and the picture is just each one being told how bright to be for that frame. A new frame goes up 50 or 60 times a second, faster than your eye follows, which is what makes it look like smooth movement.",
        ),
    ),
    Case(
        "explains",
        "explain-tv-screen",
        "what is the difference between led and oled",
        true,
        Some(
            "A modern screen is a grid of millions of pixels, each with red, green and blue lights under it, and the picture is just each one being told how bright to be for that frame. A new frame goes up 50 or 60 times a second, faster than your eye follows, which is what makes it look like smooth movement.",
        ),
    ),
    Case(
        "explains",
        "explain-tv-screen",
        "does screen brightness hurt",
        true,
        Some(
            "A modern screen is a grid of millions of pixels, each with red, green and blue lights under it, and the picture is just each one being told how bright to be for that frame. A new frame goes up 50 or 60 times a second, faster than your eye follows, which is what makes it look like smooth movement.",
        ),
    ),
    Case(
        "study",
        "fail-exams",
        "i failed my gcse exams",
        true,
        Some(
            "Speak to your school or college on results day, not next month: they can tell you the resit date (usually November for maths and English, the following summer for most others) and whether the university or college will hold your place, which plenty will once they know. If you were a mark or two off a grade boundary, a remark can be worth it, but it can also go down, so ask the exams officer what the risk is. Failing one paper is not failing the year, and a plan made this week beats a summer of dread.",
        ),
    ),
    Case(
        "study",
        "fail-exams",
        "failed a university exam and need a resit",
        true,
        Some(
            "Speak to your school or college on results day, not next month: they can tell you the resit date (usually November for maths and English, the following summer for most others) and whether the university or college will hold your place, which plenty will once they know. If you were a mark or two off a grade boundary, a remark can be worth it, but it can also go down, so ask the exams officer what the risk is. Failing one paper is not failing the year, and a plan made this week beats a summer of dread.",
        ),
    ),
    Case(
        "study",
        "fail-exams",
        "what if i fail a driving test",
        false,
        Some(
            "Speak to your school or college on results day, not next month: they can tell you the resit date (usually November for maths and English, the following summer for most others) and whether the university or college will hold your place, which plenty will once they know. If you were a mark or two off a grade boundary, a remark can be worth it, but it can also go down, so ask the exams officer what the risk is. Failing one paper is not failing the year, and a plan made this week beats a summer of dread.",
        ),
    ),
    Case(
        "sport",
        "football-boots",
        "which football boots suit muddy pitches",
        true,
        Some(
            "The surface picks the boot: moulded studs (FG) on grass, screw-in studs (SG) when it is wet and soft, astro or turf trainers (TF) on 3G and 4G, flat indoor shoes for a hall. Astro boots on wet grass give almost no grip, which is how ankles turn. Try them on with match socks and leave a thumb's width at the toe.",
        ),
    ),
    Case(
        "sport",
        "football-boots",
        "football boots for artificial ground",
        true,
        Some(
            "The surface picks the boot: moulded studs (FG) on grass, screw-in studs (SG) when it is wet and soft, astro or turf trainers (TF) on 3G and 4G, flat indoor shoes for a hall. Astro boots on wet grass give almost no grip, which is how ankles turn. Try them on with match socks and leave a thumb's width at the toe.",
        ),
    ),
    Case(
        "sport",
        "football-boots",
        "studying up on football",
        false,
        Some(
            "The surface picks the boot: moulded studs (FG) on grass, screw-in studs (SG) when it is wet and soft, astro or turf trainers (TF) on 3G and 4G, flat indoor shoes for a hall. Astro boots on wet grass give almost no grip, which is how ankles turn. Try them on with match socks and leave a thumb's width at the toe.",
        ),
    ),
    Case(
        "garden",
        "hydrangea-care",
        "why is my hydrangea not flowering",
        true,
        Some(
            "Water a hydrangea well in dry spells, because wilting in the afternoon sun is thirst and the buds for next year dry up if it is left. The colour is the soil: acid soil gives blue, limy soil gives pink, so water in a sulphur or ericaceous feed for blue, and add garden lime for pink. Prune a mophead in early spring, taking the old flower heads and about a third of last year's stems, and leave a lacecap and a climbing hydrangea alone apart from the dead heads.",
        ),
    ),
    Case(
        "garden",
        "hydrangea-care",
        "when do i prune a hydrangea",
        true,
        Some(
            "Water a hydrangea well in dry spells, because wilting in the afternoon sun is thirst and the buds for next year dry up if it is left. The colour is the soil: acid soil gives blue, limy soil gives pink, so water in a sulphur or ericaceous feed for blue, and add garden lime for pink. Prune a mophead in early spring, taking the old flower heads and about a third of last year's stems, and leave a lacecap and a climbing hydrangea alone apart from the dead heads.",
        ),
    ),
    Case(
        "garden",
        "hydrangea-care",
        "how do i change a pink hydrangea",
        true,
        Some(
            "The colour is the soil rather than the plant: acid soil gives blue, limy soil gives pink, and a pot is whatever compost you fill it with. Water in sulphur or an ericaceous feed through the season for blue, or add garden lime for pink, and give it a year, because it is next season's flowers that change. Water it well in dry spells as well - wilting in the afternoon sun is thirst, and the buds for next year dry up if it is left.",
        ),
    ),
    Case(
        "technology",
        "mobile-signal-home",
        "mobile phone has no signal in my house",
        true,
        Some(
            "Turn on wifi calling, which on most networks is Settings > Phone > Wi-Fi Calling: calls then come over your broadband and the signal stops mattering indoors. Keep a mobile call near a window, and ask your network for a signal box - they are free on most contracts if you can show the problem. A repeater you buy yourself only helps if there is at least one bar outdoors.",
        ),
    ),
    Case(
        "technology",
        "mobile-signal-home",
        "does wifi calling help at home",
        false,
        Some(
            "Turn on wifi calling, which on most networks is Settings > Phone > Wi-Fi Calling: calls then come over your broadband and the signal stops mattering indoors. Keep a mobile call near a window, and ask your network for a signal box - they are free on most contracts if you can show the problem. A repeater you buy yourself only helps if there is at least one bar outdoors.",
        ),
    ),
    Case(
        "technology",
        "mobile-signal-home",
        "can i improve my phone signal indoors",
        true,
        Some(
            "Turn on wifi calling, which on most networks is Settings > Phone > Wi-Fi Calling: calls then come over your broadband and the signal stops mattering indoors. Keep a mobile call near a window, and ask your network for a signal box - they are free on most contracts if you can show the problem. A repeater you buy yourself only helps if there is at least one bar outdoors.",
        ),
    ),
    Case(
        "study",
        "remember-names",
        "how do i remember people's names",
        true,
        Some(
            "Use the name out loud in the first minute - \"Nice to meet you, Sarah\" - and hang it on something physical: alliteration (Tall Tom), a rhyme, or a face feature. Repeating it twice early beats any trick later, and if it has gone, say so straight away: \"Sorry, your name's gone\" costs less than guessing.",
        ),
    ),
    Case(
        "study",
        "remember-names",
        "how do i remember characters in my book",
        true,
        Some(
            "Use the name out loud in the first minute - \"Nice to meet you, Sarah\" - and hang it on something physical: alliteration (Tall Tom), a rhyme, or a face feature. Repeating it twice early beats any trick later, and if it has gone, say so straight away: \"Sorry, your name's gone\" costs less than guessing.",
        ),
    ),
    Case(
        "study",
        "remember-names",
        "can you remember my name",
        true,
        Some(
            "Use the name out loud in the first minute - \"Nice to meet you, Sarah\" - and hang it on something physical: alliteration (Tall Tom), a rhyme, or a face feature. Repeating it twice early beats any trick later, and if it has gone, say so straight away: \"Sorry, your name's gone\" costs less than guessing.",
        ),
    ),
    Case(
        "study",
        "remember-reading",
        "how do i remember what i read",
        true,
        Some(
            "Read a section, then close the book and say its three points out loud before going on - rereading feels like study but the recall is what sticks. Turn each heading into a question first (\"what is a supply curve?\") and answer it from memory, then write one line per page in your own words. Come back to those lines after ten minutes, the next day and a week later, and you keep most of it with far less total time.",
        ),
    ),
    Case(
        "study",
        "remember-reading",
        "getting notes from a textbook",
        true,
        Some(
            "Read a section, then close the book and say its three points out loud before going on - rereading feels like study but the recall is what sticks. Turn each heading into a question first (\"what is a supply curve?\") and answer it from memory, then write one line per page in your own words. Come back to those lines after ten minutes, the next day and a week later, and you keep most of it with far less total time.",
        ),
    ),
    Case(
        "study",
        "remember-reading",
        "i forgot the title of the book i read",
        true,
        Some(
            "Read a section, then close the book and say its three points out loud before going on - rereading feels like study but the recall is what sticks. Turn each heading into a question first (\"what is a supply curve?\") and answer it from memory, then write one line per page in your own words. Come back to those lines after ten minutes, the next day and a week later, and you keep most of it with far less total time.",
        ),
    ),
    Case(
        "shopping",
        "smart-meter-how",
        "how does a smart meter work",
        true,
        None,
    ),
    Case(
        "shopping",
        "smart-meter-how",
        "do smart meters need wifi",
        true,
        None,
    ),
    Case(
        "shopping",
        "smart-meter-how",
        "smart meter display blank",
        true,
        Some(
            "Ring your supplier and say the meter or its display has stopped - it is their equipment, so fixing it is free, and it is usually a firmware update or a new display they send out. Photograph the reading on the meter itself before you ring, and send them a reading weekly until it is sorted, so you are not billed on an estimate in the meantime.",
        ),
    ),
    Case(
        "clothing",
        "spots",
        "which clothes cause spots on my back",
        true,
        Some(
            "Wash with a gentle cleanser twice a day and put a benzoyl peroxide or salicylic acid cream on the spots, not all over the face. Give it six weeks, change your pillowcase weekly, and see a pharmacist if it's leaving marks or scars.",
        ),
    ),
    Case(
        "clothing",
        "spots",
        "how to remove spots on a shirt",
        true,
        Some(
            "Wash with a gentle cleanser twice a day and put a benzoyl peroxide or salicylic acid cream on the spots, not all over the face. Give it six weeks, change your pillowcase weekly, and see a pharmacist if it's leaving marks or scars.",
        ),
    ),
    Case(
        "clothing",
        "spots",
        "skincare for teenage spots",
        true,
        Some(
            "Wash with a gentle cleanser twice a day and put a benzoyl peroxide or salicylic acid cream on the spots, not all over the face. Give it six weeks, change your pillowcase weekly, and see a pharmacist if it's leaving marks or scars.",
        ),
    ),
    Case(
        "clothing",
        "stain-sweat",
        "how do i remove yellow sweat marks from a white shirt",
        true,
        Some(
            "Soak the armpits in a paste of bicarbonate of soda and water with a squeeze of lemon for an hour, or use oxygen bleach on white shirts. Wash at 40C and don't iron over the stain until it's gone.",
        ),
    ),
    Case(
        "clothing",
        "stain-sweat",
        "sweat stains on dark shirts",
        true,
        Some(
            "Soak the armpits in a paste of bicarbonate of soda and water with a squeeze of lemon for an hour, or use oxygen bleach on white shirts. Wash at 40C and don't iron over the stain until it's gone.",
        ),
    ),
    Case(
        "clothing",
        "stain-sweat",
        "prevent deodorant marks",
        true,
        Some(
            "Deodorant marks on a dark top are the antiperspirant sitting on the fibres, not a stain: rub the fabric against itself, or run a dry cloth or a pair of tights over it, and they lift off without washing. Let the deodorant dry properly before you dress and the marks stop appearing.",
        ),
    ),
    Case(
        "history",
        "suffragettes",
        "who were the suffragettes",
        true,
        Some(
            "The campaign for women's votes split into the peaceful suffragists and the militant suffragettes led by Emmeline Pankhurst. In Britain, women over 30 with property got the vote in 1918, and all women over 21 in 1928.",
        ),
    ),
    Case(
        "history",
        "suffragettes",
        "the suffragette movement and suffragists",
        true,
        Some(
            "The campaign for women's votes split into the peaceful suffragists and the militant suffragettes led by Emmeline Pankhurst. In Britain, women over 30 with property got the vote in 1918, and all women over 21 in 1928.",
        ),
    ),
    Case(
        "history",
        "suffragettes",
        "when did women gain the vote",
        true,
        Some(
            "The campaign for women's votes split into the peaceful suffragists and the militant suffragettes led by Emmeline Pankhurst. In Britain, women over 30 with property got the vote in 1918, and all women over 21 in 1928.",
        ),
    ),
    Case(
        "travel",
        "suitcase-protect",
        "my suitcase came back damaged",
        true,
        Some(
            "If the airline did it, report it at the baggage desk before you leave the airport and photograph the damage there: the report number is what a claim needs, and under the Montreal rules the airline pays for repair or replacement. If it broke at home, a luggage repair shop fits new wheels, straps and zip pulls for far less than a new case.",
        ),
    ),
    Case(
        "travel",
        "suitcase-protect",
        "how do i protect a suitcase from being damaged",
        true,
        Some(
            "Do not fill it until it splits: a zip bursts from the inside, so leave a hand's width of give and put the heavy things at the wheel end. A luggage strap takes the strain off the zip, a hard shell or a padded cover protects what is fragile, and anything breakable goes in the middle wrapped in clothes. Photograph it open and closed before check-in, because that is what an airline asks for when you claim for the damage.",
        ),
    ),
    Case(
        "travel",
        "suitcase-protect",
        "pack clothes in a suitcase",
        true,
        Some(
            "Do not fill it until it splits: a zip bursts from the inside, so leave a hand's width of give and put the heavy things at the wheel end. A luggage strap takes the strain off the zip, a hard shell or a padded cover protects what is fragile, and anything breakable goes in the middle wrapped in clothes. Photograph it open and closed before check-in, because that is what an airline asks for when you claim for the damage.",
        ),
    ),
    Case(
        "money",
        "tax-refund-scam",
        "text from hmrc about a tax refund",
        true,
        Some(
            "HMRC does not text, email or ring about a refund, and a real one asks nothing of you: it arrives as a P800 letter or appears in your online tax account and is paid automatically. Do not use the link or ring the number in the message. Forward the text to 7726, send the email to phishing@hmrc.gov.uk, and check your account by typing gov.uk into the browser yourself.",
        ),
    ),
    Case(
        "money",
        "tax-refund-scam",
        "my bank sent a refund email",
        true,
        Some(
            "HMRC does not text, email or ring about a refund, and a real one asks nothing of you: it arrives as a P800 letter or appears in your online tax account and is paid automatically. Do not use the link or ring the number in the message. Forward the text to 7726, send the email to phishing@hmrc.gov.uk, and check your account by typing gov.uk into the browser yourself.",
        ),
    ),
    Case(
        "money",
        "tax-refund-scam",
        "does hmrc send refund texts",
        true,
        Some(
            "HMRC does not text, email or ring about a refund, and a real one asks nothing of you: it arrives as a P800 letter or appears in your online tax account and is paid automatically. Do not use the link or ring the number in the message. Forward the text to 7726, send the email to phishing@hmrc.gov.uk, and check your account by typing gov.uk into the browser yourself.",
        ),
    ),
    Case(
        "study",
        "vocab-test",
        "test me on my french vocab",
        true,
        Some(
            "Learn seven to ten words at a time, out loud, and test yourself instead of reading the list: cover the English side and say the word, then swap. Put each new word in a short sentence you would actually say, and go back over yesterday's words for two minutes before adding new ones — spacing is what makes them stay.",
        ),
    ),
    Case(
        "study",
        "vocab-test",
        "how do i learn vocabulary",
        true,
        Some(
            "Learn seven to ten words at a time, out loud, and test yourself instead of reading the list: cover the English side and say the word, then swap. Put each new word in a short sentence you would actually say, and go back over yesterday's words for two minutes before adding new ones — spacing is what makes them stay.",
        ),
    ),
    Case(
        "study",
        "vocab-test",
        "test my vocabulary for tomorrow",
        true,
        Some(
            "Learn seven to ten words at a time, out loud, and test yourself instead of reading the list: cover the English side and say the word, then swap. Put each new word in a short sentence you would actually say, and go back over yesterday's words for two minutes before adding new ones — spacing is what makes them stay.",
        ),
    ),
    Case(
        "warm_food",
        "warm-bread",
        "how to warm up a crusty baguette",
        true,
        Some(
            "Splash it with a little water and give it 3 to 4 minutes in a 180C (160C fan) oven - 6 to 8 for a whole loaf - foil around it for a soft bread like naan or pitta, straight on the shelf for a crusty one. Twenty seconds in the microwave works if you eat it at once, and a wrap or pitta only wants 20 to 30 seconds a side in a dry hot pan or a toaster, with an air fryer doing the same job in 3 minutes at 160C.",
        ),
    ),
    Case(
        "warm_food",
        "warm-bread",
        "how do i warm a pita",
        true,
        Some(
            "Splash it with a little water and give it 3 to 4 minutes in a 180C (160C fan) oven - 6 to 8 for a whole loaf - foil around it for a soft bread like naan or pitta, straight on the shelf for a crusty one. Twenty seconds in the microwave works if you eat it at once, and a wrap or pitta only wants 20 to 30 seconds a side in a dry hot pan or a toaster, with an air fryer doing the same job in 3 minutes at 160C.",
        ),
    ),
    Case(
        "warm_food",
        "warm-bread",
        "my bread is cold and stale",
        true,
        Some(
            "Splash it with a little water and give it 3 to 4 minutes in a 180C (160C fan) oven - 6 to 8 for a whole loaf - foil around it for a soft bread like naan or pitta, straight on the shelf for a crusty one. Twenty seconds in the microwave works if you eat it at once, and a wrap or pitta only wants 20 to 30 seconds a side in a dry hot pan or a toaster, with an air fryer doing the same job in 3 minutes at 160C.",
        ),
    ),
    Case(
        "technology",
        "watch-glass-scratch",
        "scratches in my watch glass",
        true,
        Some(
            "It depends on the crystal, and the make and model tell you which: acrylic or Hesalite (many older and cheaper watches) polishes out at home with a dab of Polywatch or Brasso on a soft cloth, a mineral glass one wants a jeweller with cerium oxide - about £15 to £30 - and a sapphire crystal only scratches on a diamond or a brick, so a mark that will not lift is usually the anti-reflective coating. A scratch you can feel with a fingernail is a new crystal, £30 to £80 fitted, and worth doing before damp gets in.",
        ),
    ),
    Case(
        "technology",
        "watch-glass-scratch",
        "scratched the glass on my table",
        true,
        Some(
            "It depends on the crystal, and the make and model tell you which: acrylic or Hesalite (many older and cheaper watches) polishes out at home with a dab of Polywatch or Brasso on a soft cloth, a mineral glass one wants a jeweller with cerium oxide - about £15 to £30 - and a sapphire crystal only scratches on a diamond or a brick, so a mark that will not lift is usually the anti-reflective coating. A scratch you can feel with a fingernail is a new crystal, £30 to £80 fitted, and worth doing before damp gets in.",
        ),
    ),
    Case(
        "technology",
        "watch-glass-scratch",
        "can i polish a sapphire watch face",
        true,
        Some(
            "It depends on the crystal, and the make and model tell you which: acrylic or Hesalite (many older and cheaper watches) polishes out at home with a dab of Polywatch or Brasso on a soft cloth, a mineral glass one wants a jeweller with cerium oxide - about £15 to £30 - and a sapphire crystal only scratches on a diamond or a brick, so a mark that will not lift is usually the anti-reflective coating. A scratch you can feel with a fingernail is a new crystal, £30 to £80 fitted, and worth doing before damp gets in.",
        ),
    ),
    Case(
        "cooking",
        "buffet-holding",
        "keep hot food warm on a buffet",
        true,
        Some(
            "Hot food wants to stay above 63C: a slow cooker on low, an oven on its lowest with the door ajar, or a lidded pan on a trivet, served in small batches so what is out gets eaten. Cold food wants to stay under 5C. Anything that has sat between the two for more than two hours is the food that makes people ill, so bin it.",
        ),
    ),
    Case(
        "cooking",
        "buffet-holding",
        "can i make sandwiches the night before",
        true,
        Some(
            "Make sandwiches the night before, wrapped in the fridge, and leave the salad or tomato out until the morning because it turns the bread wet. Egg, cheese and ham fillings keep the best; nothing already filled should sit out for more than two hours.",
        ),
    ),
    Case(
        "cooking",
        "buffet-holding",
        "how to keep buffet food cold",
        true,
        Some(
            "Cold food wants to stay under 5C: keep the bowls in the fridge until people arrive, stand them in a bigger bowl of ice, and put out a little at a time. Rice, cooked meat and dairy are the things to be strict about, and anything out for more than two hours goes in the bin.",
        ),
    ),
    Case(
        "cooking",
        "buffet-holding",
        "how long can leave food out",
        true,
        Some(
            "Hot food wants to stay above 63C: a slow cooker on low, an oven on its lowest with the door ajar, or a lidded pan on a trivet, served in small batches so what is out gets eaten. Cold food wants to stay under 5C. Anything that has sat between the two for more than two hours is the food that makes people ill, so bin it.",
        ),
    ),
    Case(
        "cooking",
        "coffee-bitter",
        "my coffee tastes bitter",
        true,
        Some(
            "Bitter coffee is over-extracted: grind it coarser, let the water come off the boil for half a minute, and take it off the grounds sooner - about 4 minutes in a cafetiere, 25 to 30 seconds for an espresso. Sour is the same dial the other way and wants a finer grind, hotter water and a little longer. Stale beans and a dirty machine make both, and instant tastes bitter when it is made with boiling water - let the kettle rest first.",
        ),
    ),
    Case(
        "cooking",
        "coffee-bitter",
        "sour espresso in the morning",
        true,
        Some(
            "Sour coffee is under-extracted: grind it finer, use water just off the boil rather than boiling, and give it longer on the grounds - about 4 minutes in a cafetiere, 25 to 30 seconds for an espresso. Bitter is the same dial the other way and wants a coarser grind and a shorter brew. A sour note that only shows up with milk is usually the milk.",
        ),
    ),
    Case(
        "cooking",
        "coffee-bitter",
        "how to fix bitter french press coffee",
        true,
        Some(
            "Bitter coffee is over-extracted: grind it coarser, let the water come off the boil for half a minute, and take it off the grounds sooner - about 4 minutes in a cafetiere, 25 to 30 seconds for an espresso. Sour is the same dial the other way and wants a finer grind, hotter water and a little longer. Stale beans and a dirty machine make both, and instant tastes bitter when it is made with boiling water - let the kettle rest first.",
        ),
    ),
    Case(
        "pets",
        "dog-guarding",
        "my dog growls when i take his bowl",
        true,
        Some(
            "Guarding food or a bone is ordinary dog behaviour that gets worse when it is punished, so stop taking things off him and teach a trade instead: hold a better treat out to one side, say \"drop\" as he leaves it, give the bone back, and hands coming near start to mean something good. Feed him where nobody walks past and leave him alone with a bone. If he has actually snapped at a child, ask the vet for a referral to a behaviourist rather than a trainer's quick fix.",
        ),
    ),
    Case(
        "pets",
        "dog-guarding",
        "my puppy guards his toys",
        true,
        Some(
            "Guarding food or a bone is ordinary dog behaviour that gets worse when it is punished, so stop taking things off him and teach a trade instead: hold a better treat out to one side, say \"drop\" as he leaves it, give the bone back, and hands coming near start to mean something good. Feed him where nobody walks past and leave him alone with a bone. If he has actually snapped at a child, ask the vet for a referral to a behaviourist rather than a trainer's quick fix.",
        ),
    ),
    Case(
        "pets",
        "dog-guarding",
        "my cat guards the sofa",
        false,
        Some(
            "Guarding food or a bone is ordinary dog behaviour that gets worse when it is punished, so stop taking things off him and teach a trade instead: hold a better treat out to one side, say \"drop\" as he leaves it, give the bone back, and hands coming near start to mean something good. Feed him where nobody walks past and leave him alone with a bone. If he has actually snapped at a child, ask the vet for a referral to a behaviourist rather than a trainer's quick fix.",
        ),
    ),
    Case(
        "pets",
        "dog-guarding",
        "how do i clean a dog's bowl",
        false,
        Some(
            "Guarding food or a bone is ordinary dog behaviour that gets worse when it is punished, so stop taking things off him and teach a trade instead: hold a better treat out to one side, say \"drop\" as he leaves it, give the bone back, and hands coming near start to mean something good. Feed him where nobody walks past and leave him alone with a bone. If he has actually snapped at a child, ask the vet for a referral to a behaviourist rather than a trainer's quick fix.",
        ),
    ),
    Case(
        "pets",
        "dog-handling-growl",
        "my dog growls when i pick him up",
        true,
        Some(
            "A growl while he is being brushed or handled is him saying it hurts or frightens him, so stop as soon as he does it and never tell him off for it - a dog punished for the warning goes straight to the snap. Run your hands over him first, because mats behind the ears, a tight collar and a sore ear or hip are the usual causes, and a dog who has always been fine and has just started wants a vet check. If there is nothing sore, go in two-second bursts with a second person feeding him chicken, and for nails do one a day and let a groomer or a vet nurse do the rest.",
        ),
    ),
    Case(
        "pets",
        "dog-handling-growl",
        "my dog growls when i touch his paw",
        true,
        Some(
            "A growl while he is being brushed or handled is him saying it hurts or frightens him, so stop as soon as he does it and never tell him off for it - a dog punished for the warning goes straight to the snap. Run your hands over him first, because mats behind the ears, a tight collar and a sore ear or hip are the usual causes, and a dog who has always been fine and has just started wants a vet check. If there is nothing sore, go in two-second bursts with a second person feeding him chicken, and for nails do one a day and let a groomer or a vet nurse do the rest.",
        ),
    ),
    Case(
        "pets",
        "dog-handling-growl",
        "my dog growls near his bowl",
        false,
        Some(
            "A growl while he is being brushed or handled is him saying it hurts or frightens him, so stop as soon as he does it and never tell him off for it - a dog punished for the warning goes straight to the snap. Run your hands over him first, because mats behind the ears, a tight collar and a sore ear or hip are the usual causes, and a dog who has always been fine and has just started wants a vet check. If there is nothing sore, go in two-second bursts with a second person feeding him chicken, and for nails do one a day and let a groomer or a vet nurse do the rest.",
        ),
    ),
    Case(
        "pets",
        "dog-handling-growl",
        "my puppy growled at a car",
        false,
        Some(
            "A growl while he is being brushed or handled is him saying it hurts or frightens him, so stop as soon as he does it and never tell him off for it - a dog punished for the warning goes straight to the snap. Run your hands over him first, because mats behind the ears, a tight collar and a sore ear or hip are the usual causes, and a dog who has always been fine and has just started wants a vet check. If there is nothing sore, go in two-second bursts with a second person feeding him chicken, and for nails do one a day and let a groomer or a vet nurse do the rest.",
        ),
    ),
    Case(
        "pets",
        "pet-medicine",
        "can i give my dog paracetamol",
        false,
        Some(
            "Into the back of the mouth rather than the food if he spits it out: hold the head gently up, drop the tablet past the hump of the tongue, close the mouth and rub the throat or blow on his nose until he swallows, then give water or a treat so it doesn't stick. Pate, cheese or a pill pocket hides most of them, and ask the vet for a liquid or a spot-on if a cat fights the tablets. Squirt a liquid into the cheek pouch a little at a time, never straight down the throat, and never crush or open a capsule unless the vet says so.",
        ),
    ),
    Case(
        "pets",
        "pet-medicine",
        "my kitten took a human pill",
        true,
        Some(
            "Into the back of the mouth rather than the food if he spits it out: hold the head gently up, drop the tablet past the hump of the tongue, close the mouth and rub the throat or blow on his nose until he swallows, then give water or a treat so it doesn't stick. Pate, cheese or a pill pocket hides most of them, and ask the vet for a liquid or a spot-on if a cat fights the tablets. Squirt a liquid into the cheek pouch a little at a time, never straight down the throat, and never crush or open a capsule unless the vet says so.",
        ),
    ),
    Case(
        "pets",
        "pet-medicine",
        "medicine for my hamster",
        true,
        Some(
            "Into the back of the mouth rather than the food if he spits it out: hold the head gently up, drop the tablet past the hump of the tongue, close the mouth and rub the throat or blow on his nose until he swallows, then give water or a treat so it doesn't stick. Pate, cheese or a pill pocket hides most of them, and ask the vet for a liquid or a spot-on if a cat fights the tablets. Squirt a liquid into the cheek pouch a little at a time, never straight down the throat, and never crush or open a capsule unless the vet says so.",
        ),
    ),
    Case(
        "pets",
        "pet-medicine",
        "how do i remove a label from the tablet",
        true,
        Some(
            "Into the back of the mouth rather than the food if he spits it out: hold the head gently up, drop the tablet past the hump of the tongue, close the mouth and rub the throat or blow on his nose until he swallows, then give water or a treat so it doesn't stick. Pate, cheese or a pill pocket hides most of them, and ask the vet for a liquid or a spot-on if a cat fights the tablets. Squirt a liquid into the cheek pouch a little at a time, never straight down the throat, and never crush or open a capsule unless the vet says so.",
        ),
    ),
    Case(
        "pets",
        "rescue-or-baby",
        "should i get a rescue cat or a kitten",
        true,
        Some(
            "A rescue cat comes with a known character, is past the kitten climbing and biting, and settles in a week or two; a kitten is months of work but you shape it from the start. Out at work all day, or a first cat: the adult rescue is the kinder start.",
        ),
    ),
    Case(
        "pets",
        "rescue-or-baby",
        "which is easier a puppy or an adult rescue",
        true,
        Some(
            "A rescue adult usually comes with a known character, is past the chewing, and settles in a week or two; a puppy is a year of work but you shape everything from the start. First dog, or a busy household: the rescue is the kinder start.",
        ),
    ),
    Case(
        "pets",
        "rescue-or-baby",
        "rescue rabbit or a baby rabbit",
        true,
        Some(
            "A rescue adult usually comes with a known character, is past the chewing, and settles in a week or two; a puppy is a year of work but you shape everything from the start. First dog, or a busy household: the rescue is the kinder start.",
        ),
    ),
    Case(
        "pets",
        "rescue-or-baby",
        "should i get a dog with a baby",
        true,
        Some(
            "A rescue adult usually comes with a known character, is past the chewing, and settles in a week or two; a puppy is a year of work but you shape everything from the start. First dog, or a busy household: the rescue is the kinder start.",
        ),
    ),
    Case(
        "pregnancy",
        "preg-labour-length",
        "how long does labour usually last",
        true,
        Some(
            "A first labour usually takes 8 to 18 hours from the first regular contractions to the birth, and a second or later baby is usually quicker. Early labour at home can run much longer than that - a day or more of stop-start contractions is normal - so it is the pattern that matters: contractions getting stronger, longer and closer together, or your waters breaking, is the point to ring the midwife.",
        ),
    ),
    Case(
        "pregnancy",
        "preg-labour-length",
        "how long do contractions last",
        true,
        Some(
            "Each contraction usually lasts 30 to 60 seconds early on, then 60 to 90 seconds once labour is established, with a few minutes between them. They get longer, stronger and closer together as it goes on - that pattern is what tells you it is real labour rather than the practice tightenings. Ring the midwife if one lasts much longer than 90 seconds, or there is no gap between them.",
        ),
    ),
    Case(
        "pregnancy",
        "preg-labour-length",
        "how long will i stay in hospital after giving birth",
        true,
        Some(
            "After a straightforward birth most women are home within a day: a few hours if everything is well and it is not your first, overnight or a day or two with a first baby. A caesarean is two to four days, and any concern about you or the baby adds a day, so plan for the longer end when you arrange a lift home.",
        ),
    ),
    Case(
        "pregnancy",
        "preg-labour-length",
        "when will i go into labour",
        false,
        Some(
            "A first labour usually takes 8 to 18 hours from the first regular contractions to the birth, and a second or later baby is usually quicker. Early labour at home can run much longer than that - a day or more of stop-start contractions is normal - so it is the pattern that matters: contractions getting stronger, longer and closer together, or your waters breaking, is the point to ring the midwife.",
        ),
    ),
    Case(
        "pregnancy",
        "preg-labour-length",
        "how long does the second stage of labour take",
        true,
        Some(
            "The second stage is the pushing: about an hour on average for a first baby, sometimes two, and often 10 or 20 minutes with a second or later one. The urge to push is hard to resist and that is normal - go with it, take the breaks between contractions, and expect the midwife to guide you to slow down at the end so you tear less.",
        ),
    ),
    Case(
        "health",
        "shaky-hands",
        "why are my hands shaking",
        true,
        Some(
            "Most shaky hands are caffeine, a missed meal, anxiety or a fever, and they ease when that does: cut the coffee to one or two cups, eat every few hours and sleep. Shaking that crept on over months, is worse on one side, or comes with a slow walk or stiff arm needs a GP, and so does a tremor with a fast heartbeat or weight loss, because the thyroid gets checked for that.",
        ),
    ),
    Case(
        "health",
        "shaky-hands",
        "i keep shaking all over",
        true,
        Some(
            "Most shaking like this is caffeine, a missed meal, anxiety or a fever, and it eases when that does: cut the coffee to one or two cups, eat every few hours and sleep. Shaking that crept on over months, is worse on one side, or comes with a slow walk or stiff arm needs a GP, and so does a tremor with a fast heartbeat or weight loss, because the thyroid gets checked for that.",
        ),
    ),
    Case(
        "health",
        "shaky-hands",
        "my fingers are still shaking after coffee",
        true,
        Some(
            "Most shaky hands are caffeine, a missed meal, anxiety or a fever, and they ease when that does: cut the coffee to one or two cups, eat every few hours and sleep. Shaking that crept on over months, is worse on one side, or comes with a slow walk or stiff arm needs a GP, and so does a tremor with a fast heartbeat or weight loss, because the thyroid gets checked for that.",
        ),
    ),
    Case(
        "cooking",
        "wake-food",
        "what food to serve after a funeral",
        true,
        Some(
            "Food that needs no carving and no serving: sandwiches cut small, sausage rolls, a quiche, a pot of soup kept hot, cut cake, and plenty of tea. Make it the day before, allow three or four sandwich pieces a head, and ask two people to keep the teapot going and the plates moving, because that is the job nobody thinks to do.",
        ),
    ),
    Case(
        "cooking",
        "wake-food",
        "what do i make for the wake",
        false,
        Some(
            "Food that needs no carving and no serving: sandwiches cut small, sausage rolls, a quiche, a pot of soup kept hot, cut cake, and plenty of tea. Make it the day before, allow three or four sandwich pieces a head, and ask two people to keep the teapot going and the plates moving, because that is the job nobody thinks to do.",
        ),
    ),
    Case(
        "cooking",
        "wake-food",
        "i woke up tired from my sleep",
        false,
        Some(
            "Food that needs no carving and no serving: sandwiches cut small, sausage rolls, a quiche, a pot of soup kept hot, cut cake, and plenty of tea. Make it the day before, allow three or four sandwich pieces a head, and ask two people to keep the teapot going and the plates moving, because that is the job nobody thinks to do.",
        ),
    ),
    Case(
        "cooking",
        "wake-food",
        "what food do i bring to the wake",
        true,
        Some(
            "Food that needs no carving and no serving: sandwiches cut small, sausage rolls, a quiche, a pot of soup kept hot, cut cake, and plenty of tea. Make it the day before, allow three or four sandwich pieces a head, and ask two people to keep the teapot going and the plates moving, because that is the job nobody thinks to do.",
        ),
    ),
];
