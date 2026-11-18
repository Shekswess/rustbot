//! Direct dish-fault callbacks on the pinned reference, including explicit abstention.

pub(super) const CASES: &[(&str, &str, Option<&str>)] = &[
    (
        "soup-thick",
        "my soup is too thick",
        Some(
            "Too thick: add hot stock, milk or just water a splash at a time, stirring, until it pours the way you want it, then taste it again because thinning takes the salt with it. A spoon of butter or cream stirred in at the end does the same with more flavour, and a stew that has cooked down only needs the lid on for ten minutes.",
        ),
    ),
    (
        "soup-thick",
        "how do i thicken soup",
        Some(
            "Simmer it uncovered for 10 minutes to reduce it, or blend a ladleful and stir it back in for body without flour. A handful of red lentils thickens a soup or stew in 15 minutes; for a stew, take the lid off for the last half hour.",
        ),
    ),
    (
        "soup-thick",
        "this soup is so thick",
        Some(
            "Too thick: add hot stock, milk or just water a splash at a time, stirring, until it pours the way you want it, then taste it again because thinning takes the salt with it. A spoon of butter or cream stirred in at the end does the same with more flavour, and a stew that has cooked down only needs the lid on for ten minutes.",
        ),
    ),
    (
        "soup-thick",
        "how do i thin soup",
        Some(
            "Simmer it uncovered for 10 minutes to reduce it, or blend a ladleful and stir it back in for body without flour. A handful of red lentils thickens a soup or stew in 15 minutes; for a stew, take the lid off for the last half hour.",
        ),
    ),
    (
        "soup-thick",
        "my soup is really thick and creamy",
        Some(
            "Too thick: add hot stock, milk or just water a splash at a time, stirring, until it pours the way you want it, then taste it again because thinning takes the salt with it. A spoon of butter or cream stirred in at the end does the same with more flavour, and a stew that has cooked down only needs the lid on for ten minutes.",
        ),
    ),
    (
        "soup-thick",
        "how do i fix a thick soup",
        Some(
            "Simmer it uncovered for 10 minutes to reduce it, or blend a ladleful and stir it back in for body without flour. A handful of red lentils thickens a soup or stew in 15 minutes; for a stew, take the lid off for the last half hour.",
        ),
    ),
    (
        "soup-thick",
        "how to make my soup thicker",
        Some(
            "Simmer it uncovered for 10 minutes to reduce it, or blend a ladleful and stir it back in for body without flour. A handful of red lentils thickens a soup or stew in 15 minutes; for a stew, take the lid off for the last half hour.",
        ),
    ),
    (
        "soup-thick",
        "my soup was thick but i want to thicken it",
        Some(
            "Simmer it uncovered for 10 minutes to reduce it, or blend a ladleful and stir it back in for body without flour. A handful of red lentils thickens a soup or stew in 15 minutes; for a stew, take the lid off for the last half hour.",
        ),
    ),
    (
        "fix-salty",
        "my bread is too salty",
        Some(
            "Baking is the one place salt cannot be cooked out or bulked out: leave it and cut the salt from whatever goes with it, or add a little more sugar and fat to push it into the background. Next time weigh the salt with the measuring spoons rather than shaking it in, and remember salted butter is already about a quarter salt.",
        ),
    ),
    (
        "fix-salty",
        "my soup is too salty",
        Some(
            "Don't add water: bulk it out with more of what's already in it — potatoes, rice, tinned tomatoes, cream or unsalted stock — and finish with a squeeze of lemon. In a soup or stew, a peeled potato simmered in it for 15 minutes takes up some of the salt too.",
        ),
    ),
    (
        "fix-salty",
        "my roast is too salty",
        Some(
            "Don't add water: bulk it out with more of what's already in it — potatoes, rice, tinned tomatoes, cream or unsalted stock — and finish with a squeeze of lemon. In a soup or stew, a peeled potato simmered in it for 15 minutes takes up some of the salt too.",
        ),
    ),
    (
        "fix-salty",
        "my brownies are salty",
        Some(
            "Baking is the one place salt cannot be cooked out or bulked out: leave it and cut the salt from whatever goes with it, or add a little more sugar and fat to push it into the background. Next time weigh the salt with the measuring spoons rather than shaking it in, and remember salted butter is already about a quarter salt.",
        ),
    ),
    (
        "fix-salty",
        "how to rescue a very salty sauce",
        Some(
            "Don't add water: bulk it out with more of what's already in it — potatoes, rice, tinned tomatoes, cream or unsalted stock — and finish with a squeeze of lemon. In a soup or stew, a peeled potato simmered in it for 15 minutes takes up some of the salt too.",
        ),
    ),
    (
        "fix-salty",
        "i put too much salt in the cake",
        Some(
            "Baking is the one place salt cannot be cooked out or bulked out: leave it and cut the salt from whatever goes with it, or add a little more sugar and fat to push it into the background. Next time weigh the salt with the measuring spoons rather than shaking it in, and remember salted butter is already about a quarter salt.",
        ),
    ),
    (
        "fix-salty",
        "the biscuits are salty",
        Some(
            "Baking is the one place salt cannot be cooked out or bulked out: leave it and cut the salt from whatever goes with it, or add a little more sugar and fat to push it into the background. Next time weigh the salt with the measuring spoons rather than shaking it in, and remember salted butter is already about a quarter salt.",
        ),
    ),
    (
        "fix-salty",
        "what to do with food that is salty",
        Some(
            "Don't add water: bulk it out with more of what's already in it — potatoes, rice, tinned tomatoes, cream or unsalted stock — and finish with a squeeze of lemon. In a soup or stew, a peeled potato simmered in it for 15 minutes takes up some of the salt too.",
        ),
    ),
    (
        "fix-lumpy",
        "my mashed potatoes are lumpy",
        Some(
            "Mash them while they're hot with plenty of butter and warm milk, and beat with a wooden spoon rather than a fork. Lumps come from cold potatoes or too little fat; if they're stubborn, push the mash through a sieve or a potato ricer.",
        ),
    ),
    (
        "fix-lumpy",
        "my mash is lumpy",
        Some(
            "Mash them while they're hot with plenty of butter and warm milk, and beat with a wooden spoon rather than a fork. Lumps come from cold potatoes or too little fat; if they're stubborn, push the mash through a sieve or a potato ricer.",
        ),
    ),
    (
        "fix-lumpy",
        "why is my custard lumpy",
        Some(
            "Take it off the heat and whisk hard, then push it through a sieve. Custard goes lumpy when the heat is too high or the eggs go in too fast, so next time pour the hot milk into the eggs a splash at a time and cook it slowly, stirring.",
        ),
    ),
    (
        "fix-lumpy",
        "why are my oats lumpy",
        Some(
            "Beat in a splash more milk or water and whisk it hard; porridge lumps come from dry oats hitting hot liquid too fast. Next time stir the oats into cold liquid and bring it up slowly, stirring all the time.",
        ),
    ),
    (
        "fix-lumpy",
        "lumpy porridge",
        Some(
            "Beat in a splash more milk or water and whisk it hard; porridge lumps come from dry oats hitting hot liquid too fast. Next time stir the oats into cold liquid and bring it up slowly, stirring all the time.",
        ),
    ),
    (
        "fix-lumpy",
        "how to fix lumpy gravy",
        Some(
            "Whisk it hard with the pan off the heat and push it through a sieve, because lumps break up far better off the ring. Next time stir the flour into the fat before any liquid goes in, then add the liquid a splash at a time.",
        ),
    ),
    (
        "fix-lumpy",
        "my bechamel is lumpy",
        Some(
            "Whisk it hard with the pan off the heat and push it through a sieve, because lumps break up far better off the ring. Next time stir the flour into the fat before any liquid goes in, then add the liquid a splash at a time.",
        ),
    ),
    (
        "fix-lumpy",
        "the sauce has lumps",
        Some(
            "Whisk it hard with the pan off the heat and push it through a sieve, because lumps break up far better off the ring. Next time stir the flour into the fat before any liquid goes in, then add the liquid a splash at a time.",
        ),
    ),
    (
        "curry-split",
        "my curry split",
        Some(
            "A sauce splits when it gets too hot: take it off the heat and whisk in a spoonful of cold cream or butter to bring it back. Next time, lower the heat and add dairy at the end.",
        ),
    ),
    (
        "curry-split",
        "my creme anglaise is curdled",
        Some(
            "A curdled custard is the eggs getting hot too fast, so the protein in them sets into grains: take it off the heat at once, whisk in a spoonful of cold cream or milk to stop it cooking, then push it through a sieve and it comes back smooth. Next time pour the hot milk into the beaten eggs a splash at a time, keep the pan over a low heat, stir all the time, and take it off at 82C, because a custard that comes to the boil will always curdle.",
        ),
    ),
    (
        "curry-split",
        "my custard is grainy",
        Some(
            "A curdled custard is the eggs getting hot too fast, so the protein in them sets into grains: take it off the heat at once, whisk in a spoonful of cold cream or milk to stop it cooking, then push it through a sieve and it comes back smooth. Next time pour the hot milk into the beaten eggs a splash at a time, keep the pan over a low heat, stir all the time, and take it off at 82C, because a custard that comes to the boil will always curdle.",
        ),
    ),
    (
        "curry-split",
        "my mayo split",
        Some(
            "Mayonnaise splits when the oil goes in faster than the yolk can take it up, so start again with a clean bowl: an egg yolk, a teaspoon of water and a pinch of salt, then the oil a drop at a time until it thickens and only then in a thin stream. To rescue a split one, put a fresh yolk in a clean bowl and whisk the split mayonnaise into it a spoon at a time - one yolk will bring back up to about a mugful.",
        ),
    ),
    (
        "curry-split",
        "my aioli broke",
        Some(
            "Mayonnaise splits when the oil goes in faster than the yolk can take it up, so start again with a clean bowl: an egg yolk, a teaspoon of water and a pinch of salt, then the oil a drop at a time until it thickens and only then in a thin stream. To rescue a split one, put a fresh yolk in a clean bowl and whisk the split mayonnaise into it a spoon at a time - one yolk will bring back up to about a mugful.",
        ),
    ),
    (
        "curry-split",
        "my hollandaise split",
        Some(
            "Hollandaise splits when the bowl gets too warm or the butter goes in too hot: take the bowl off the water at once, add a teaspoon of cold water and whisk hard. If it will not come back, start a fresh yolk in a clean bowl with a spoon of water and whisk the split sauce into it a little at a time, off the heat. Next time keep the water barely trembling and let the melted butter cool for a minute first.",
        ),
    ),
    (
        "curry-split",
        "my hollandaise has butter in it",
        Some(
            "Hollandaise splits when the bowl gets too warm or the butter goes in too hot: take the bowl off the water at once, add a teaspoon of cold water and whisk hard. If it will not come back, start a fresh yolk in a clean bowl with a spoon of water and whisk the split sauce into it a little at a time, off the heat. Next time keep the water barely trembling and let the melted butter cool for a minute first.",
        ),
    ),
    (
        "curry-split",
        "why is bearnaise split",
        Some(
            "Hollandaise splits when the bowl gets too warm or the butter goes in too hot: take the bowl off the water at once, add a teaspoon of cold water and whisk hard. If it will not come back, start a fresh yolk in a clean bowl with a spoon of water and whisk the split sauce into it a little at a time, off the heat. Next time keep the water barely trembling and let the melted butter cool for a minute first.",
        ),
    ),
    (
        "curry-split",
        "my split custard and mayo",
        Some(
            "A curdled custard is the eggs getting hot too fast, so the protein in them sets into grains: take it off the heat at once, whisk in a spoonful of cold cream or milk to stop it cooking, then push it through a sieve and it comes back smooth. Next time pour the hot milk into the beaten eggs a splash at a time, keep the pan over a low heat, stir all the time, and take it off at 82C, because a custard that comes to the boil will always curdle.",
        ),
    ),
    (
        "chocolate-seize",
        "what do i do when my chocolate has seized",
        Some(
            "Chocolate seizes when even a drop of water gets in, and the fix is more water, not less: stir in a teaspoon of boiling water at a time, off the heat, until it goes smooth again. A tablespoon of warm cream does the same and leaves you with a ganache.",
        ),
    ),
    (
        "chocolate-seize",
        "my chocolate melted and went grainy",
        Some(
            "Chocolate seizes when even a drop of water gets in, and the fix is more water, not less: stir in a teaspoon of boiling water at a time, off the heat, until it goes smooth again. A tablespoon of warm cream does the same and leaves you with a ganache.",
        ),
    ),
    (
        "chocolate-seize",
        "how do i melt white chocolate",
        Some(
            "Melt chocolate in a bowl over barely simmering water, with the bowl not touching the water, or in 20-second bursts in the microwave, stirring between them. Keep every spoon and bowl dry, because a drop of water makes it seize, and take it off while most of it is melted - the rest will go as you stir.",
        ),
    ),
    (
        "chocolate-seize",
        "white streaks on my dark chocolate",
        Some(
            "White streaks on chocolate are bloom, and it is harmless: either cocoa butter or sugar has come to the surface because the chocolate got warm and then cool again. It still tastes the same and it is fine to eat or to melt. To stop it happening, store it cool and steady, around 15 to 18C, and never in the fridge.",
        ),
    ),
    (
        "chocolate-seize",
        "is dark chocolate healthy",
        Some(
            "As a treat, chocolate is fine: dark with 70% cocoa or more has the least sugar, so it is the one to pick if you are watching it, and two squares is a normal portion. White chocolate is the sweetest of the three because it is cocoa butter, milk and sugar with no cocoa solids at all.",
        ),
    ),
    (
        "chocolate-seize",
        "is white chocolate real chocolate",
        Some(
            "White chocolate is cocoa butter, milk powder and sugar, with no cocoa solids at all, which is why it tastes of vanilla and milk rather than of chocolate. It is still real chocolate by the labelling rules - at least 20% cocoa butter in Europe - just a different thing from dark, and it is the sweetest of the three.",
        ),
    ),
    (
        "chocolate-seize",
        "what is chocolate made of",
        Some(
            "What most people mean by chocolate is dark: cocoa solids and cocoa butter with sugar, and the higher the percentage the less sugar there is. Milk chocolate adds milk powder and more sugar, and white chocolate is cocoa butter, milk and sugar with no cocoa solids at all.",
        ),
    ),
    (
        "chocolate-seize",
        "how do i temper dark chocolate",
        Some(
            "To temper chocolate, melt two thirds of it in a bowl over barely simmering water to 45C (or 40C for white), take it off the heat and stir in the rest, chopped, until it drops to 27C, then warm it gently back to 31C for dark or 29C for milk and white. That sets it glossy and hard at room temperature. A thermometer earns its keep; without one, a clean spoon dipped in should set firm in 5 minutes.",
        ),
    ),
    (
        "chocolate-seize",
        "how do i dip strawberries in chocolate",
        Some(
            "Melt it slowly, then dip: dry the fruit first so the chocolate grips, dip about two thirds of the way in, let the extra drip back into the bowl, and set them on baking paper in the fridge for 10 minutes to set hard. If the chocolate feels thin, stir in a handful more chopped chocolate off the heat.",
        ),
    ),
    (
        "chocolate-seize",
        "how do i melt chocolate for dipping",
        Some(
            "Melt chocolate in a bowl over barely simmering water, with the bowl not touching the water, or in 20-second bursts in the microwave, stirring between them. Keep every spoon and bowl dry, because a drop of water makes it seize, and take it off while most of it is melted - the rest will go as you stir.",
        ),
    ),
    (
        "chocolate-seize",
        "how do i make chocolate smooth",
        Some(
            "Melt chocolate in a bowl over barely simmering water, with the bowl not touching the water, or in 20-second bursts in the microwave, stirring between them. Keep every spoon and bowl dry, because a drop of water makes it seize, and take it off while most of it is melted - the rest will go as you stir.",
        ),
    ),
    (
        "chocolate-seize",
        "what are gray streaks on chocolate",
        Some(
            "White streaks on chocolate are bloom, and it is harmless: either cocoa butter or sugar has come to the surface because the chocolate got warm and then cool again. It still tastes the same and it is fine to eat or to melt. To stop it happening, store it cool and steady, around 15 to 18C, and never in the fridge.",
        ),
    ),
    (
        "chocolate-seize",
        "is white chocolate healthy",
        Some(
            "As a treat, chocolate is fine: dark with 70% cocoa or more has the least sugar, so it is the one to pick if you are watching it, and two squares is a normal portion. White chocolate is the sweetest of the three because it is cocoa butter, milk and sugar with no cocoa solids at all.",
        ),
    ),
    (
        "chocolate-seize",
        "which chocolate do i melt",
        Some(
            "Melt chocolate in a bowl over barely simmering water, with the bowl not touching the water, or in 20-second bursts in the microwave, stirring between them. Keep every spoon and bowl dry, because a drop of water makes it seize, and take it off while most of it is melted - the rest will go as you stir.",
        ),
    ),
    (
        "chocolate-seize",
        "do sugar calories matter in white chocolate",
        Some(
            "As a treat, chocolate is fine: dark with 70% cocoa or more has the least sugar, so it is the one to pick if you are watching it, and two squares is a normal portion. White chocolate is the sweetest of the three because it is cocoa butter, milk and sugar with no cocoa solids at all.",
        ),
    ),
    (
        "fix-watery",
        "my oat milk is watery",
        Some(
            "A carton of oat, almond or soy milk that pours thin has separated: shake it hard for half a minute before you open it and it comes back together, and a cheap barista one holds up better in coffee. Homemade oat milk is thin by nature - blend 1 part oats to 4 of water, strain it lightly, and add a pinch of salt; it thickens as it stands in the fridge.",
        ),
    ),
    (
        "fix-watery",
        "my oat milk porridge is watery",
        Some(
            "Porridge that has gone too loose wants a minute more on the heat with the lid off and a stir, and less milk next time: 40g of oats to 300ml of milk or water is enough for one, and it thickens as it stands. Do not add more oats once it is cooking, because they will not soften in time.",
        ),
    ),
    (
        "fix-watery",
        "my mashed potato is watery",
        Some(
            "Mash goes watery when the potatoes come out of wet water or the milk goes in cold: drain them, put the pan back on a low heat for a minute to steam the rest off, then mash with warm milk and butter. If it has already gone loose, warm it through in the pan and stir rather than adding anything - it firms up as the extra water steams away.",
        ),
    ),
    (
        "fix-watery",
        "my hummus is watery",
        Some(
            "A hummus goes watery from water or ice going in with the chickpeas, or from too much lemon: drain the tin well, add a spoon more tahini or a few more chickpeas and blitz again, and serve it chilled, because it thickens as it sits. Loosen it later with oil, not water.",
        ),
    ),
    (
        "fix-watery",
        "my tapenade is watery",
        Some(
            "Tapenade goes watery from olives that came out of the brine wet, or from too much oil: drain them, dry them on a tea towel, and pound or pulse them with the capers and anchovy before the oil goes in. A thin tapenade wants more olives and a few capers, not less oil, and it firms up in the fridge.",
        ),
    ),
    (
        "fix-watery",
        "my pesto is watery",
        Some(
            "Pesto goes watery from basil that was still wet, or from too much oil in one go: dry the leaves in a tea towel, pound or pulse them with the nuts and cheese first and add the oil last, a little at a time. A thin pesto in the jar wants more grated cheese and nuts rather than less oil.",
        ),
    ),
    (
        "fix-watery",
        "my dip is watery",
        Some(
            "A dip goes watery from water in the base: drain the tin or squeeze out the grated vegetable, then thicken it with a spoon of the same solid - chickpeas, yogurt, cream cheese - and taste it again, because thinning it dilutes the salt and the acid too.",
        ),
    ),
    (
        "fix-watery",
        "my guacamole is watery",
        Some(
            "Guacamole goes watery from over-ripe avocado or too much lime: mash it with a fork rather than a blender, drain any liquid off and stir in a little more avocado, and leave the stone in if you are keeping it - it slows the browning, not the water.",
        ),
    ),
    (
        "fix-watery",
        "my lasagna is watery",
        Some(
            "A bake goes watery when the sauce was thin before it was layered, or the vegetables still had water in them. Next time drain and squeeze the veg and let the sauce come to a simmer with a spoon of cornflour slaked in cold water before it goes in. For this one, give it 10 to 15 minutes uncovered at the same heat, then rest it 10 minutes before you cut it, and the sauce will firm up.",
        ),
    ),
    (
        "fix-watery",
        "my dumpling mix is watery",
        Some(
            "A dumpling mix that is too wet wants plain flour a spoonful at a time until it stops sticking to the bowl, then shape them with wet hands and drop them onto a stew that is simmering, put the lid on and give them 20 minutes without lifting it. Too wet and they spread out into the gravy instead of sitting on top.",
        ),
    ),
    (
        "fix-watery",
        "my fritter mix is watery",
        Some(
            "A mix that will not hold together is too wet to fry: squeeze grated onion or veg in a clean tea towel, stir in a spoonful of plain or gram flour, and chill it for 30 minutes so the starch takes up the water. Make sure the oil is properly hot before the first one goes in - a cool pan is what turns a soft mix into a pancake.",
        ),
    ),
    (
        "fix-watery",
        "my bread dough is watery",
        Some(
            "Add water a spoonful at a time and knead a minute after each one: a stiff dough reads drier than it is, because the flour takes a few minutes to take up what is already in there, and you only feel the difference after kneading. A bread dough should be tacky rather than sticky - it clings to your hand but comes away clean - and it is ready when a piece stretches thin enough to see light through without tearing. If you overshoot, a spoonful of flour brings it back.",
        ),
    ),
    (
        "fix-watery",
        "my pizza dough is too sticky",
        Some(
            "Add flour a spoonful at a time and knead it in before you judge, because a dough is wettest in the first few minutes: it should cling to your hand and come away clean. Wetter is better for an open crumb, so stop as soon as it stops running and keep a little of the water back next time, since flour alone can be 10% different in how much it drinks.",
        ),
    ),
    (
        "fix-watery",
        "my batter is watery",
        Some(
            "A batter that is too wet wants flour, not heat: whisk in a spoonful at a time until it coats the back of the spoon, then let it rest 20 minutes so the flour takes up the liquid. Too thin and it will not hold the bubbles, so pancakes come out pale and flat rather than risen, and a cake will not hold its rise either.",
        ),
    ),
    (
        "fix-watery",
        "my porridge is watery",
        Some(
            "Porridge that has gone too loose wants a minute more on the heat with the lid off and a stir, and less milk next time: 40g of oats to 300ml of milk or water is enough for one, and it thickens as it stands. Do not add more oats once it is cooking, because they will not soften in time.",
        ),
    ),
    (
        "fix-watery",
        "my risotto is watery",
        Some(
            "Risotto that will not thicken wants more stirring, not more heat: keep it moving and add the stock a ladle at a time, waiting for each one to be taken up, then beat in cold butter and grated cheese off the heat at the end - that is what makes it creamy and loose at the same time.",
        ),
    ),
    (
        "fix-watery",
        "my couscous is watery",
        Some(
            "Couscous goes watery from too much water: pour it into a wide dish, fork it through to break up the clumps and leave it uncovered for 5 minutes so the steam escapes. Next time use the same volume of couscous and boiling water, cover it for 5 minutes, then fork it with a knob of butter rather than stirring it.",
        ),
    ),
    (
        "fix-watery",
        "my quiche is watery",
        Some(
            "A quiche or a pie goes watery when the filling is too wet or the egg is undercooked: give it 5 to 10 minutes more in a 180C oven and it will set, and next time cook the vegetables or the fruit down first and let them cool, then blind-bake the case before the filling goes in.",
        ),
    ),
    (
        "fix-watery",
        "my shortcrust dough is watery",
        Some(
            "Add water a spoonful at a time and knead a minute after each one: a stiff dough reads drier than it is, because the flour takes a few minutes to take up what is already in there, and you only feel the difference after kneading. A bread dough should be tacky rather than sticky - it clings to your hand but comes away clean - and it is ready when a piece stretches thin enough to see light through without tearing. If you overshoot, a spoonful of flour brings it back.",
        ),
    ),
    (
        "fix-watery",
        "my rice is watery",
        Some(
            "Rice goes watery from too much water or the lid coming off: tip it into a sieve, then back in the pan with the lid on and 5 minutes on the lowest heat to steam dry. Next time one and a half parts water to one of rice, lid on, and leave it alone.",
        ),
    ),
    (
        "fix-watery",
        "my custard is watery",
        Some(
            "Custard that is thin has not had enough heat: stir a teaspoon of cornflour into a splash of cold milk, whisk it in and give it 2 more minutes on a low heat until it coats the back of the spoon. It also thickens as it cools, so take it off before it looks set rather than after.",
        ),
    ),
    (
        "fix-watery",
        "my icing is watery",
        Some(
            "Icing that is too thin wants icing sugar, a spoon at a time, beaten in until it holds a soft peak: a teaspoon of water or milk goes a long way, so add that in drops. A glaze is meant to be thin - it should coat the back of the spoon and run off in ribbons, which is a different job from icing a cake.",
        ),
    ),
    (
        "fix-watery",
        "my smoothie is watery",
        Some(
            "A thin smoothie or shake wants less liquid and something solid: half a banana, a spoon of oats or yoghurt, or a handful of frozen fruit, blended again - the fruit and the ice are what give it body. Pour the milk or juice in last, a splash at a time, instead of all at the start.",
        ),
    ),
    (
        "fix-watery",
        "my yoghurt is watery",
        Some(
            "A pot of yoghurt that has gone watery has separated, and that is harmless: stir it back together, or spoon the whey off the top, or strain it through muslin or a clean tea towel in a sieve for an hour for a thicker pot. A yoghurt dip goes watery because the vegetables in it were wet: salt the grated cucumber or carrot for 10 minutes, squeeze it out in a tea towel, and fold it into thick yoghurt at the end - a spoon of cream cheese or labneh brings back one that has already split.",
        ),
    ),
    (
        "fix-watery",
        "my tzatziki is watery",
        Some(
            "A yoghurt dip goes watery because the vegetables in it were wet: salt the grated cucumber or carrot for 10 minutes, squeeze it out in a tea towel, and fold it into thick yoghurt at the end - a spoon of cream cheese or labneh brings back one that has already split. A pot of yoghurt that has gone watery has separated, and that is harmless: stir it back together, or spoon the whey off the top, or strain it through muslin or a clean tea towel in a sieve for an hour for a thicker pot.",
        ),
    ),
    (
        "fix-watery",
        "my dal is watery",
        Some(
            "A thin dal wants the lid off for 10 more minutes, stirring now and then: it thickens as the lentils break up, and mashing a ladleful against the side of the pan and stirring it back in thickens it at once. Add water a splash at a time at the end, because dal sets as it cools.",
        ),
    ),
    (
        "fix-watery",
        "my coleslaw is watery",
        Some(
            "A watery salad is nearly always salt pulling the water out of the vegetables: salt the shredded cabbage or the cucumber for 10 minutes, squeeze it in a clean tea towel, and dress it just before it goes to the table. Mayo loosened with a splash of vinegar coats better than a thick spoonful.",
        ),
    ),
    (
        "fix-watery",
        "my soup is watery",
        Some(
            "Take the lid off and let it simmer for 10 to 15 minutes: most of the water goes off in the steam. To thicken it properly, stir in a spoon of tomato puree, a handful of red lentils, or cornflour slaked in cold water, and give it 5 more minutes.",
        ),
    ),
    (
        "fix-watery",
        "my avocado dip is watery",
        Some(
            "A dip goes watery from water in the base: drain the tin or squeeze out the grated vegetable, then thicken it with a spoon of the same solid - chickpeas, yogurt, cream cheese - and taste it again, because thinning it dilutes the salt and the acid too.",
        ),
    ),
    (
        "fix-watery",
        "what can i do with watery almond milk and soup",
        Some(
            "Take the lid off and let it simmer for 10 to 15 minutes: most of the water goes off in the steam. To thicken it properly, stir in a spoon of tomato puree, a handful of red lentils, or cornflour slaked in cold water, and give it 5 more minutes.",
        ),
    ),
    (
        "fix-watery",
        "watery raita and yoghurt",
        Some(
            "A yoghurt dip goes watery because the vegetables in it were wet: salt the grated cucumber or carrot for 10 minutes, squeeze it out in a tea towel, and fold it into thick yoghurt at the end - a spoon of cream cheese or labneh brings back one that has already split. A pot of yoghurt that has gone watery has separated, and that is harmless: stir it back together, or spoon the whey off the top, or strain it through muslin or a clean tea towel in a sieve for an hour for a thicker pot.",
        ),
    ),
    (
        "fix-watery",
        "my bread and pastry dough is wet",
        Some(
            "Add flour a spoonful at a time and knead it in before you judge, because a dough is wettest in the first few minutes: it should cling to your hand and come away clean. Wetter is better for an open crumb, so stop as soon as it stops running and keep a little of the water back next time, since flour alone can be 10% different in how much it drinks.",
        ),
    ),
    (
        "fix-watery",
        "how to fix watery sauce",
        Some(
            "Take the lid off and let it simmer for 10 to 15 minutes: most of the water goes off in the steam. To thicken it properly, stir in a spoon of tomato puree, a handful of red lentils, or cornflour slaked in cold water, and give it 5 more minutes.",
        ),
    ),
    ("fix-watery", "my mixture is too wet", None),
    (
        "fix-watery",
        "the liquid is too wet",
        Some(
            "Take the lid off and let it simmer for 10 to 15 minutes: most of the water goes off in the steam. To thicken it properly, stir in a spoon of tomato puree, a handful of red lentils, or cornflour slaked in cold water, and give it 5 more minutes.",
        ),
    ),
    (
        "fix-watery",
        "my stew is a bit wet",
        Some(
            "Take the lid off and let it simmer for 10 to 15 minutes: most of the water goes off in the steam. To thicken it properly, stir in a spoon of tomato puree, a handful of red lentils, or cornflour slaked in cold water, and give it 5 more minutes.",
        ),
    ),
    ("fix-watery", "the unknown thing went wet", None),
];
