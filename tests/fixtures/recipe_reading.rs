//! Pinned recipe method, herb, and measuring guard and reply cases.

pub(super) const CASES: &[(&str, &str, bool, Option<&str>)] = &[
    (
        "recipe-amount",
        "does doubling a recipe double the time",
        true,
        Some(
            "No - the time barely changes, the quantity does. Keep the oven or hob where it was and add 10 to 20 minutes for the extra volume to heat through and reduce, checking early rather than trusting the clock. Use a wider pot or two tins rather than a deeper one, swap two trays on their shelves halfway, and cut the liquid by about a tenth because less of it evaporates. Salt, chilli, stock and herbs do not double exactly: add three-quarters, taste, then top up.",
        ),
    ),
    (
        "recipe-amount",
        "how do i halve a recipe",
        true,
        Some(
            "Halve everything except the egg and the raising agent: beat the egg and weigh half of it, and keep the baking powder or yeast nearer two-thirds, because it does not scale down in a straight line. Use a smaller tin or pan so the depth stays the same, keep the temperature, and start checking 5 minutes earlier - a smaller batch comes up to heat and browns sooner.",
        ),
    ),
    (
        "recipe-amount",
        "how do i make half a recipe",
        true,
        Some(
            "Halve everything except the egg and the raising agent: beat the egg and weigh half of it, and keep the baking powder or yeast nearer two-thirds, because it does not scale down in a straight line. Use a smaller tin or pan so the depth stays the same, keep the temperature, and start checking 5 minutes earlier - a smaller batch comes up to heat and browns sooner.",
        ),
    ),
    (
        "recipe-amount",
        "how do i cook for two",
        true,
        Some(
            "Halve everything except the egg and the raising agent: beat the egg and weigh half of it, and keep the baking powder or yeast nearer two-thirds, because it does not scale down in a straight line. Use a smaller tin or pan so the depth stays the same, keep the temperature, and start checking 5 minutes earlier - a smaller batch comes up to heat and browns sooner.",
        ),
    ),
    (
        "recipe-amount",
        "can i make it in a larger tin",
        true,
        Some(
            "Go by the area of the tin rather than the numbers on the page: a 23cm round holds about a third more than a 20cm, so the same batter fills it shallower and bakes 5 to 10 minutes faster, and scaling the batter up by that third fills it to the same depth in the same time. Keep the oven temperature, fill to two-thirds, and check with a skewer 5 minutes before the recipe's time either way.",
        ),
    ),
    (
        "recipe-amount",
        "should i halve the recipe in a big tin",
        true,
        Some(
            "Halve everything except the egg and the raising agent: beat the egg and weigh half of it, and keep the baking powder or yeast nearer two-thirds, because it does not scale down in a straight line. Use a smaller tin or pan so the depth stays the same, keep the temperature, and start checking 5 minutes earlier - a smaller batch comes up to heat and browns sooner.",
        ),
    ),
    (
        "recipe-amount",
        "will a smaller sandwich tin work",
        true,
        Some(
            "Go by the area of the tin rather than the numbers on the page: a 23cm round holds about a third more than a 20cm, so the same batter fills it shallower and bakes 5 to 10 minutes faster, and scaling the batter up by that third fills it to the same depth in the same time. Keep the oven temperature, fill to two-thirds, and check with a skewer 5 minutes before the recipe's time either way.",
        ),
    ),
    (
        "recipe-amount",
        "I'm using a cake tin, can I double it",
        true,
        Some(
            "No - the time barely changes, the quantity does. Keep the oven or hob where it was and add 10 to 20 minutes for the extra volume to heat through and reduce, checking early rather than trusting the clock. Use a wider pot or two tins rather than a deeper one, swap two trays on their shelves halfway, and cut the liquid by about a tenth because less of it evaporates. Salt, chilli, stock and herbs do not double exactly: add three-quarters, taste, then top up.",
        ),
    ),
    (
        "recipe-amount",
        "can i double this in a deeper pan",
        true,
        Some(
            "No - the time barely changes, the quantity does. Keep the oven or hob where it was and add 10 to 20 minutes for the extra volume to heat through and reduce, checking early rather than trusting the clock. Use a wider pot or two tins rather than a deeper one, swap two trays on their shelves halfway, and cut the liquid by about a tenth because less of it evaporates. Salt, chilli, stock and herbs do not double exactly: add three-quarters, taste, then top up.",
        ),
    ),
    (
        "recipe-amount",
        "can i use a smaller pudding tin",
        true,
        Some(
            "Go by the area of the tin rather than the numbers on the page: a 23cm round holds about a third more than a 20cm, so the same batter fills it shallower and bakes 5 to 10 minutes faster, and scaling the batter up by that third fills it to the same depth in the same time. Keep the oven temperature, fill to two-thirds, and check with a skewer 5 minutes before the recipe's time either way.",
        ),
    ),
    (
        "recipe-amount",
        "other tins the recipe says",
        true,
        Some(
            "Go by the area of the tin rather than the numbers on the page: a 23cm round holds about a third more than a 20cm, so the same batter fills it shallower and bakes 5 to 10 minutes faster, and scaling the batter up by that third fills it to the same depth in the same time. Keep the oven temperature, fill to two-thirds, and check with a skewer 5 minutes before the recipe's time either way.",
        ),
    ),
    (
        "recipe-amount",
        "what if i cook this just for one",
        true,
        Some(
            "Halve everything except the egg and the raising agent: beat the egg and weigh half of it, and keep the baking powder or yeast nearer two-thirds, because it does not scale down in a straight line. Use a smaller tin or pan so the depth stays the same, keep the temperature, and start checking 5 minutes earlier - a smaller batch comes up to heat and browns sooner.",
        ),
    ),
    (
        "recipe-amount",
        "toast for two",
        true,
        Some(
            "Halve everything except the egg and the raising agent: beat the egg and weigh half of it, and keep the baking powder or yeast nearer two-thirds, because it does not scale down in a straight line. Use a smaller tin or pan so the depth stays the same, keep the temperature, and start checking 5 minutes earlier - a smaller batch comes up to heat and browns sooner.",
        ),
    ),
    (
        "recipe-amount",
        "for two hundred",
        true,
        Some(
            "Halve everything except the egg and the raising agent: beat the egg and weigh half of it, and keep the baking powder or yeast nearer two-thirds, because it does not scale down in a straight line. Use a smaller tin or pan so the depth stays the same, keep the temperature, and start checking 5 minutes earlier - a smaller batch comes up to heat and browns sooner.",
        ),
    ),
    (
        "recipe-amount",
        "can i use the wrong size cake tin or halve the recipe",
        true,
        Some(
            "Go by the area of the tin rather than the numbers on the page: a 23cm round holds about a third more than a 20cm, so the same batter fills it shallower and bakes 5 to 10 minutes faster, and scaling the batter up by that third fills it to the same depth in the same time. Keep the oven temperature, fill to two-thirds, and check with a skewer 5 minutes before the recipe's time either way.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "look at the state of my mint plant",
        false,
        Some(
            "Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts. Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "i have a sage bush",
        true,
        Some(
            "Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew. Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "i bought rosemary",
        false,
        Some(
            "Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there. Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "i have a pot of rosemary",
        true,
        Some(
            "Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew. Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "what can i cook with my mint plant",
        true,
        Some(
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served. Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "how do i use my parsley plant",
        true,
        Some(
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served. Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "can i eat the basil in my garden",
        true,
        Some(
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served. Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "we got given a bay tree",
        true,
        Some(
            "Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew. Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "my mint plant has taken over the garden",
        true,
        Some(
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served. Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "mint has taken over my garden",
        true,
        Some(
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served. Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "look at my oregano",
        false,
        Some(
            "Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there. Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "how many thyme sprigs",
        false,
        Some(
            "Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there. Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "lots of mint what can i use it in",
        true,
        Some(
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served. Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "i have a pot of marjoram",
        true,
        Some(
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served. Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "the thyme and mint plant says hi",
        false,
        Some(
            "Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there. Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "does rosemary give more uses than mint",
        true,
        Some(
            "Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew. Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "sprouting chives",
        false,
        Some(
            "Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts. Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "how do i make mint sauce",
        true,
        Some(
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served. Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "can we use dill and rosemary",
        true,
        Some(
            "Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew. Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there.",
        ),
    ),
    (
        "herb-plant-to-cook",
        "my planting pot is missing",
        false,
        Some(
            "Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts. Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure flour without scales",
        true,
        Some(
            "Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure plain flour without scales",
        true,
        Some(
            "Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure self raising flour without scales",
        true,
        Some(
            "Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure self-raising flour without scales",
        true,
        Some(
            "Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure bread flour without scales",
        true,
        Some(
            "Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure spelt without scales",
        true,
        Some(
            "Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure wholemeal without scales",
        true,
        Some(
            "Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure sugar without scales",
        true,
        Some(
            "Spoon white sugar into the cup and level it, and pack brown sugar down hard. A cup of caster or granulated sugar is about 200g, a cup of icing sugar about 120g, and a level tablespoon of caster sugar about 12g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure caster sugar without scales",
        true,
        Some(
            "Spoon white sugar into the cup and level it, and pack brown sugar down hard. A cup of caster or granulated sugar is about 200g, a cup of icing sugar about 120g, and a level tablespoon of caster sugar about 12g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure granulated without scales",
        true,
        Some(
            "Spoon white sugar into the cup and level it, and pack brown sugar down hard. A cup of caster or granulated sugar is about 200g, a cup of icing sugar about 120g, and a level tablespoon of caster sugar about 12g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure brown sugar without scales",
        true,
        Some(
            "Spoon white sugar into the cup and level it, and pack brown sugar down hard. A cup of caster or granulated sugar is about 200g, a cup of icing sugar about 120g, and a level tablespoon of caster sugar about 12g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure icing sugar without scales",
        true,
        Some(
            "Spoon white sugar into the cup and level it, and pack brown sugar down hard. A cup of caster or granulated sugar is about 200g, a cup of icing sugar about 120g, and a level tablespoon of caster sugar about 12g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure muscovado without scales",
        true,
        Some(
            "Spoon white sugar into the cup and level it, and pack brown sugar down hard. A cup of caster or granulated sugar is about 200g, a cup of icing sugar about 120g, and a level tablespoon of caster sugar about 12g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure butter without scales",
        true,
        Some(
            "Butter is easiest by weight: a cup is about 225g, a stick about 113g, a level tablespoon about 14g. If you have a block in its wrapper, the paper is marked in 25g lines - cut on them and there is nothing to wash up. Soften it first, or it will not cream evenly. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure margarine without scales",
        true,
        Some(
            "Butter is easiest by weight: a cup is about 225g, a stick about 113g, a level tablespoon about 14g. If you have a block in its wrapper, the paper is marked in 25g lines - cut on them and there is nothing to wash up. Soften it first, or it will not cream evenly. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure lard without scales",
        true,
        Some(
            "Butter is easiest by weight: a cup is about 225g, a stick about 113g, a level tablespoon about 14g. If you have a block in its wrapper, the paper is marked in 25g lines - cut on them and there is nothing to wash up. Soften it first, or it will not cream evenly. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure shortening without scales",
        true,
        Some(
            "Butter is easiest by weight: a cup is about 225g, a stick about 113g, a level tablespoon about 14g. If you have a block in its wrapper, the paper is marked in 25g lines - cut on them and there is nothing to wash up. Soften it first, or it will not cream evenly. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure oats without scales",
        true,
        Some(
            "A cup of rolled oats is about 90g, and 50g a person is a portion of porridge. Spoon them in and level rather than pressing them down, because oats settle and a packed cup can weigh half again as much. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure porridge oats without scales",
        true,
        None,
    ),
    (
        "measure-ingredients",
        "how do i measure rolled oats without scales",
        true,
        Some(
            "A cup of rolled oats is about 90g, and 50g a person is a portion of porridge. Spoon them in and level rather than pressing them down, because oats settle and a packed cup can weigh half again as much. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure cocoa without scales",
        true,
        Some(
            "A cup of cocoa powder is about 85g, and it is the one ingredient that really does want spooning and levelling: cocoa compacts badly and a scooped cup can be a third heavier. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure cocoa powder without scales",
        true,
        Some(
            "A cup of cocoa powder is about 85g, and it is the one ingredient that really does want spooning and levelling: cocoa compacts badly and a scooped cup can be a third heavier. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure honey without scales",
        true,
        Some(
            "Honey and syrup are measured in a jug at eye level, not a cup, because they do not settle flat - a cup of honey is about 340g. Warm the spoon or the jug in hot water first and it will run out clean. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure syrup without scales",
        true,
        Some(
            "Honey and syrup are measured in a jug at eye level, not a cup, because they do not settle flat - a cup of honey is about 340g. Warm the spoon or the jug in hot water first and it will run out clean. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure golden syrup without scales",
        true,
        Some(
            "Honey and syrup are measured in a jug at eye level, not a cup, because they do not settle flat - a cup of honey is about 340g. Warm the spoon or the jug in hot water first and it will run out clean. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure treacle without scales",
        true,
        Some(
            "Honey and syrup are measured in a jug at eye level, not a cup, because they do not settle flat - a cup of honey is about 340g. Warm the spoon or the jug in hot water first and it will run out clean. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure molasses without scales",
        true,
        Some(
            "Honey and syrup are measured in a jug at eye level, not a cup, because they do not settle flat - a cup of honey is about 340g. Warm the spoon or the jug in hot water first and it will run out clean. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure milk without scales",
        true,
        Some(
            "Liquids go in a jug on the worktop, read at eye level with the jug standing still - a cup of milk or water is 240ml and a cup of oil about 225ml. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure water without scales",
        true,
        Some(
            "Liquids go in a jug on the worktop, read at eye level with the jug standing still - a cup of milk or water is 240ml and a cup of oil about 225ml. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure oil without scales",
        true,
        Some(
            "Liquids go in a jug on the worktop, read at eye level with the jug standing still - a cup of milk or water is 240ml and a cup of oil about 225ml. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure buttermilk without scales",
        true,
        Some(
            "Liquids go in a jug on the worktop, read at eye level with the jug standing still - a cup of milk or water is 240ml and a cup of oil about 225ml. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure flour and sugar",
        true,
        Some(
            "Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "measure water and butter",
        true,
        Some(
            "Butter is easiest by weight: a cup is about 225g, a stick about 113g, a level tablespoon about 14g. If you have a block in its wrapper, the paper is marked in 25g lines - cut on them and there is nothing to wash up. Soften it first, or it will not cream evenly. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    ("measure-ingredients", "how do i measure rice", true, None),
    (
        "measure-ingredients",
        "how do i measure flour for bread but not rice",
        true,
        None,
    ),
    (
        "measure-ingredients",
        "how do i measure oatmeal",
        true,
        Some(
            "Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how to measure ingredients",
        true,
        Some(
            "Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how do i measure coconut",
        true,
        Some(
            "Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "how much oil in a cup",
        true,
        Some(
            "Liquids go in a jug on the worktop, read at eye level with the jug standing still - a cup of milk or water is 240ml and a cup of oil about 225ml. Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "measure-ingredients",
        "what ingredients do i measure first",
        true,
        Some(
            "Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight.",
        ),
    ),
    (
        "read-a-recipe",
        "how do i read a recipe",
        true,
        Some(
            "Read it twice, all the way through, before you touch anything: what it serves, the prep time and the cooking time separately, the oven setting, and whether anything has to chill, soak or rest. Then get every ingredient out and cut up, which is what the French call mise en place - most recipes go wrong in the last five minutes because the onion was not chopped yet. Two phrases to know: 'divided' means the amount is split between stages (half the cheese in the sauce, half on top), and 'until' is a cue rather than a time - until golden, until it comes away from the pan. Times in a recipe are for that oven and that tin; trust your eyes and nose first. Check the tin size too, because a wrong-sized tin changes the depth and with it the time.",
        ),
    ),
    (
        "read-a-recipe",
        "what does divided mean in a recipe",
        true,
        Some(
            "'Divided' means the amount is split between stages: half the cheese in the sauce and half on top, or the butter half in the cake and half in the icing. Weigh or measure the whole lot out and set the second part aside in a bowl before you start, or it is easy to tip it all in at once.",
        ),
    ),
    (
        "read-a-recipe",
        "the method says divide and stir until mixed",
        true,
        Some(
            "'Divided' means the amount is split between stages: half the cheese in the sauce and half on top, or the butter half in the cake and half in the icing. Weigh or measure the whole lot out and set the second part aside in a bowl before you start, or it is easy to tip it all in at once.",
        ),
    ),
    (
        "read-a-recipe",
        "what does until mean in a recipe",
        true,
        Some(
            "'Until' is a cue, not a time: until golden, until it comes away from the pan, until a skewer comes out clean. The recipe gives you the sign to look for because the clock cannot - ovens and tins differ - so trust your eyes and nose first and the timer second.",
        ),
    ),
    (
        "read-a-recipe",
        "what does mise en place mean",
        true,
        Some(
            "Mise en place is everything weighed, chopped and set out in bowls before you turn the heat on - the French for 'in place'. It is the whole trick of the last five minutes of a recipe: most dishes go wrong there because the onion was not chopped yet. Read the ingredients as a shopping-and-chopping list and do all of it first.",
        ),
    ),
    (
        "read-a-recipe",
        "what does prep mean",
        true,
        Some(
            "Mise en place is everything weighed, chopped and set out in bowls before you turn the heat on - the French for 'in place'. It is the whole trick of the last five minutes of a recipe: most dishes go wrong there because the onion was not chopped yet. Read the ingredients as a shopping-and-chopping list and do all of it first.",
        ),
    ),
    (
        "read-a-recipe",
        "how do i read a recipe written in cups",
        true,
        Some(
            "A cup in a recipe is a volume: 240ml, or 8 US fluid ounces, which is a bit over half a pint. Most British recipes weigh in grams instead and a British teacup is not a US cup, so a recipe that says cups is written in American measures - and a cup of flour is about 120g, a cup of sugar about 200g.",
        ),
    ),
    (
        "read-a-recipe",
        "what does serves 4 mean in a recipe",
        true,
        Some(
            "'Serves four' usually means four as a main with something alongside, and the number is a guide rather than a promise - a hungry four will finish a dish written for six. Halve or double the weights if your table is smaller or bigger, and keep the tin size and the time in mind when you do.",
        ),
    ),
    (
        "read-a-recipe",
        "what does cup and makes mean in a recipe",
        true,
        Some(
            "A cup in a recipe is a volume: 240ml, or 8 US fluid ounces, which is a bit over half a pint. Most British recipes weigh in grams instead and a British teacup is not a US cup, so a recipe that says cups is written in American measures - and a cup of flour is about 120g, a cup of sugar about 200g.",
        ),
    ),
    (
        "read-a-recipe",
        "what does how many mean",
        true,
        Some(
            "'Serves four' usually means four as a main with something alongside, and the number is a guide rather than a promise - a hungry four will finish a dish written for six. Halve or double the weights if your table is smaller or bigger, and keep the tin size and the time in mind when you do.",
        ),
    ),
    (
        "read-a-recipe",
        "the ingredient says prepared",
        true,
        Some(
            "Read it twice, all the way through, before you touch anything: what it serves, the prep time and the cooking time separately, the oven setting, and whether anything has to chill, soak or rest. Then get every ingredient out and cut up, which is what the French call mise en place - most recipes go wrong in the last five minutes because the onion was not chopped yet. Two phrases to know: 'divided' means the amount is split between stages (half the cheese in the sauce, half on top), and 'until' is a cue rather than a time - until golden, until it comes away from the pan. Times in a recipe are for that oven and that tin; trust your eyes and nose first. Check the tin size too, because a wrong-sized tin changes the depth and with it the time.",
        ),
    ),
    (
        "read-a-recipe",
        "what does preparation mean",
        true,
        Some(
            "Read it twice, all the way through, before you touch anything: what it serves, the prep time and the cooking time separately, the oven setting, and whether anything has to chill, soak or rest. Then get every ingredient out and cut up, which is what the French call mise en place - most recipes go wrong in the last five minutes because the onion was not chopped yet. Two phrases to know: 'divided' means the amount is split between stages (half the cheese in the sauce, half on top), and 'until' is a cue rather than a time - until golden, until it comes away from the pan. Times in a recipe are for that oven and that tin; trust your eyes and nose first. Check the tin size too, because a wrong-sized tin changes the depth and with it the time.",
        ),
    ),
    (
        "read-a-recipe",
        "what is the prep time on a recipe",
        true,
        Some(
            "Mise en place is everything weighed, chopped and set out in bowls before you turn the heat on - the French for 'in place'. It is the whole trick of the last five minutes of a recipe: most dishes go wrong there because the onion was not chopped yet. Read the ingredients as a shopping-and-chopping list and do all of it first.",
        ),
    ),
    (
        "read-a-recipe",
        "does recipe say 8 ounces",
        true,
        Some(
            "Read it twice, all the way through, before you touch anything: what it serves, the prep time and the cooking time separately, the oven setting, and whether anything has to chill, soak or rest. Then get every ingredient out and cut up, which is what the French call mise en place - most recipes go wrong in the last five minutes because the onion was not chopped yet. Two phrases to know: 'divided' means the amount is split between stages (half the cheese in the sauce, half on top), and 'until' is a cue rather than a time - until golden, until it comes away from the pan. Times in a recipe are for that oven and that tin; trust your eyes and nose first. Check the tin size too, because a wrong-sized tin changes the depth and with it the time.",
        ),
    ),
    (
        "read-a-recipe",
        "how do i follow a guide",
        true,
        Some(
            "Read it twice, all the way through, before you touch anything: what it serves, the prep time and the cooking time separately, the oven setting, and whether anything has to chill, soak or rest. Then get every ingredient out and cut up, which is what the French call mise en place - most recipes go wrong in the last five minutes because the onion was not chopped yet. Two phrases to know: 'divided' means the amount is split between stages (half the cheese in the sauce, half on top), and 'until' is a cue rather than a time - until golden, until it comes away from the pan. Times in a recipe are for that oven and that tin; trust your eyes and nose first. Check the tin size too, because a wrong-sized tin changes the depth and with it the time.",
        ),
    ),
    (
        "read-a-recipe",
        "cup divided until prep makes",
        true,
        Some(
            "'Divided' means the amount is split between stages: half the cheese in the sauce and half on top, or the butter half in the cake and half in the icing. Weigh or measure the whole lot out and set the second part aside in a bowl before you start, or it is easy to tip it all in at once.",
        ),
    ),
];
