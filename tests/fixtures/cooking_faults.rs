//! Exact pinned replies from the cooking fault branches.

pub(super) const CASES: &[(&str, &str, Option<&str>)] = &[
    (
        "bake-gummy",
        "My cake is gummy",
        Some(
            "Gummy means it needed longer, not a hotter oven: the outside set while the middle was still wet. Give it another 5 to 10 minutes at the same heat with foil over the top, and test the centre with a skewer, not the edge.",
        ),
    ),
    (
        "bake-gummy",
        "Gummy sourdough",
        Some(
            "A gummy loaf is under-baked rather than under-proved: tap the base for a hollow sound, and if the middle feels heavy put it back for 10 minutes at the same heat. Next time prove until a floured finger springs back slowly.",
        ),
    ),
    (
        "bake-gummy",
        "my loaf stayed gummy in the middle",
        Some(
            "A gummy loaf is under-baked rather than under-proved: tap the base for a hollow sound, and if the middle feels heavy put it back for 10 minutes at the same heat. Next time prove until a floured finger springs back slowly.",
        ),
    ),
    (
        "bake-gummy",
        "my focaccia is gummy",
        Some(
            "A gummy loaf is under-baked rather than under-proved: tap the base for a hollow sound, and if the middle feels heavy put it back for 10 minutes at the same heat. Next time prove until a floured finger springs back slowly.",
        ),
    ),
    (
        "bake-gummy",
        "my bread roll is gummy and heavy",
        Some(
            "A gummy loaf is under-baked rather than under-proved: tap the base for a hollow sound, and if the middle feels heavy put it back for 10 minutes at the same heat. Next time prove until a floured finger springs back slowly.",
        ),
    ),
    (
        "bake-gummy",
        "my breadmaker sponge is gummy",
        Some(
            "Gummy means it needed longer, not a hotter oven: the outside set while the middle was still wet. Give it another 5 to 10 minutes at the same heat with foil over the top, and test the centre with a skewer, not the edge.",
        ),
    ),
    (
        "bake-gummy",
        "my pita is gummy",
        Some(
            "A gummy loaf is under-baked rather than under-proved: tap the base for a hollow sound, and if the middle feels heavy put it back for 10 minutes at the same heat. Next time prove until a floured finger springs back slowly.",
        ),
    ),
    (
        "bake-gummy",
        "one gummy loaf of banana bread",
        Some(
            "A gummy loaf is under-baked rather than under-proved: tap the base for a hollow sound, and if the middle feels heavy put it back for 10 minutes at the same heat. Next time prove until a floured finger springs back slowly.",
        ),
    ),
    (
        "bake-gummy",
        "my cornbread is still gooey in the middle",
        Some(
            "Gummy means it needed longer, not a hotter oven: the outside set while the middle was still wet. Give it another 5 to 10 minutes at the same heat with foil over the top, and test the centre with a skewer, not the edge.",
        ),
    ),
    (
        "bake-gummy",
        "How do I make bread?",
        Some(
            "A gummy loaf is under-baked rather than under-proved: tap the base for a hollow sound, and if the middle feels heavy put it back for 10 minutes at the same heat. Next time prove until a floured finger springs back slowly.",
        ),
    ),
    (
        "soggy-lunch",
        "My sandwich got soggy overnight",
        Some(
            "A sandwich goes soggy because the wet filling soaks into the bread: butter both slices right to the edges first, or spread mayonnaise or soft cheese as a barrier, and keep tomato, cucumber and lettuce in the middle rather than against the bread. Packed in the morning, the wet things go in a separate tub and are added at lunchtime, which is the only thing that really works on a hot day.",
        ),
    ),
    (
        "soggy-lunch",
        "I have soggy salad and a wet wrap",
        Some(
            "A sandwich goes soggy because the wet filling soaks into the bread: butter both slices right to the edges first, or spread mayonnaise or soft cheese as a barrier, and keep tomato, cucumber and lettuce in the middle rather than against the bread. Packed in the morning, the wet things go in a separate tub and are added at lunchtime, which is the only thing that really works on a hot day.",
        ),
    ),
    (
        "soggy-lunch",
        "my lettuce is soggy at lunch",
        Some(
            "A salad goes soggy because the leaves are still wet or the dressing went on early: dry them properly in a tea towel or a salad spinner, dress it at the table, and keep the wet things - tomato, cucumber, dressing - at the bottom of the bowl until you eat. Salt draws the water out too, so season it in the last minute rather than ahead.",
        ),
    ),
    (
        "soggy-lunch",
        "soggy coleslaw with a roast",
        Some(
            "A salad goes soggy because the leaves are still wet or the dressing went on early: dry them properly in a tea towel or a salad spinner, dress it at the table, and keep the wet things - tomato, cucumber, dressing - at the bottom of the bowl until you eat. Salt draws the water out too, so season it in the last minute rather than ahead.",
        ),
    ),
    (
        "soggy-lunch",
        "why is my pasta salad soggy",
        Some(
            "A salad goes soggy because the leaves are still wet or the dressing went on early: dry them properly in a tea towel or a salad spinner, dress it at the table, and keep the wet things - tomato, cucumber, dressing - at the bottom of the bowl until you eat. Salt draws the water out too, so season it in the last minute rather than ahead.",
        ),
    ),
    (
        "soggy-lunch",
        "my packed lunch got wet",
        Some(
            "A packed lunch goes soggy because water gets in from somewhere. Keep the wet things - tomato, cucumber, dressing, a drink - in their own tub and stand the leaky ones upright, put a layer of kitchen paper or a lettuce leaf under the bread, and pack the dry things on top. A leaking drink and a damp fork do more damage than a whole morning in the bag.",
        ),
    ),
    (
        "soggy-lunch",
        "soppy lunchbox",
        Some(
            "A packed lunch goes soggy because water gets in from somewhere. Keep the wet things - tomato, cucumber, dressing, a drink - in their own tub and stand the leaky ones upright, put a layer of kitchen paper or a lettuce leaf under the bread, and pack the dry things on top. A leaking drink and a damp fork do more damage than a whole morning in the bag.",
        ),
    ),
    (
        "soggy-lunch",
        "why does my sub get soggy in my bag",
        Some(
            "A sandwich goes soggy because the wet filling soaks into the bread: butter both slices right to the edges first, or spread mayonnaise or soft cheese as a barrier, and keep tomato, cucumber and lettuce in the middle rather than against the bread. Packed in the morning, the wet things go in a separate tub and are added at lunchtime, which is the only thing that really works on a hot day.",
        ),
    ),
    (
        "soggy-lunch",
        "what can I use to pack my lunch",
        Some(
            "A packed lunch goes soggy because water gets in from somewhere. Keep the wet things - tomato, cucumber, dressing, a drink - in their own tub and stand the leaky ones upright, put a layer of kitchen paper or a lettuce leaf under the bread, and pack the dry things on top. A leaking drink and a damp fork do more damage than a whole morning in the bag.",
        ),
    ),
    (
        "bake-fault-more",
        "My waffles are soggy",
        Some(
            "A soggy waffle is a batter that is too thin or an iron that is not hot enough: make it thicker than pancake batter - it should plop off the ladle rather than pour - let it rest 10 minutes so the flour takes up the liquid, and heat the iron until a drop of water skitters on it. Fill it three-quarters so the steam can escape, and stand the cooked ones on a rack in a low oven rather than stacking them, because a stack steams itself soft.",
        ),
    ),
    (
        "bake-fault-more",
        "my flapjacks never set",
        Some(
            "A flapjack that will not set wants less liquid or more oats: about 25g more oats and 2 tablespoons less syrup, pressed down hard into the tin with the back of a spoon before it bakes. Bake it at 180C (160C fan) for 25 minutes until the edges are golden, then leave it in the tin until it is completely cold - warm flapjacks are always soft and they firm up as they cool.",
        ),
    ),
    (
        "bake-fault-more",
        "my flapjacks are too soft",
        Some(
            "A flapjack that will not set wants less liquid or more oats: about 25g more oats and 2 tablespoons less syrup, pressed down hard into the tin with the back of a spoon before it bakes. Bake it at 180C (160C fan) for 25 minutes until the edges are golden, then leave it in the tin until it is completely cold - warm flapjacks are always soft and they firm up as they cool.",
        ),
    ),
    (
        "bake-fault-more",
        "my flapjack crumbles",
        Some(
            "Flapjacks that crumble were cut warm or short of syrup: cool them completely in the tin, then cut with a sharp knife straight down, and add 2 tablespoons more golden syrup or honey to the next batch. Pack the mix down firmly before it bakes, because a loose tray bakes into crumbs.",
        ),
    ),
    (
        "bake-fault-more",
        "my brownies are dry",
        Some(
            "A dry brownie stayed in the oven too long or went into too big a tin: 20 to 25 minutes at 180C (160C fan) in a 20cm square tin, and take it out while the middle still looks slightly under, because it sets as it cools. Melt the butter and chocolate together gently rather than boiling them, and keep the flour light - it is there to hold the brownie together, not to make a cake.",
        ),
    ),
    (
        "bake-fault-more",
        "my brownie comes out cakey",
        Some(
            "A dry brownie stayed in the oven too long or went into too big a tin: 20 to 25 minutes at 180C (160C fan) in a 20cm square tin, and take it out while the middle still looks slightly under, because it sets as it cools. Melt the butter and chocolate together gently rather than boiling them, and keep the flour light - it is there to hold the brownie together, not to make a cake.",
        ),
    ),
    ("bake-fault-more", "my brownies are chewy", None),
    (
        "bake-fault-more",
        "why do my scones come out dense?",
        Some(
            "Dense scones come from working the dough or a wet mix: use cold butter, rub it in until it looks like breadcrumbs, and bring it together with a knife and one fold rather than kneading. Pat it 2cm thick, cut straight down with a floured cutter and bake at 220C (200C fan) for 12 to 15 minutes, because a slow oven sets them heavy.",
        ),
    ),
    (
        "bake-fault-more",
        "who makes good waffles?",
        Some(
            "A soggy waffle is a batter that is too thin or an iron that is not hot enough: make it thicker than pancake batter - it should plop off the ladle rather than pour - let it rest 10 minutes so the flour takes up the liquid, and heat the iron until a drop of water skitters on it. Fill it three-quarters so the steam can escape, and stand the cooked ones on a rack in a low oven rather than stacking them, because a stack steams itself soft.",
        ),
    ),
    ("bake-fault-more", "the oat bars never firmed up", None),
    (
        "bake-fault-more",
        "one waffle and a brownie, both dry",
        Some(
            "A soggy waffle is a batter that is too thin or an iron that is not hot enough: make it thicker than pancake batter - it should plop off the ladle rather than pour - let it rest 10 minutes so the flour takes up the liquid, and heat the iron until a drop of water skitters on it. Fill it three-quarters so the steam can escape, and stand the cooked ones on a rack in a low oven rather than stacking them, because a stack steams itself soft.",
        ),
    ),
    ("bake-fault-more", "a muffin is dense", None),
    (
        "veg-tough",
        "my jacket potato is too hard",
        Some(
            "A jacket potato is hard in the middle because it was rushed: prick it, microwave it 8 to 10 minutes until a knife goes in, then 15 to 20 minutes at 200C to crisp the skin. In the oven alone it needs an hour and a quarter at 200C, so give it that long or start it early.",
        ),
    ),
    (
        "veg-tough",
        "my roast potatoes are still hard",
        Some(
            "Hard roast potatoes went in too big or came out too early: give them another 10 to 15 minutes at 200C, and next time cut them no bigger than a golf ball and parboil them 8 minutes first, which is what cooks the middle before the outside crisps.",
        ),
    ),
    (
        "veg-tough",
        "my potatoes are hard inside",
        Some(
            "Boiled potatoes go hard in the middle when the pieces are uneven: cut them the same size, start them in cold salted water and simmer 15 to 20 minutes, until a knife slides in. An old potato can be woody at the core, and that will not soften however long it cooks.",
        ),
    ),
    (
        "veg-tough",
        "why are my mushrooms rubbery",
        Some(
            "Rubbery mushrooms have been steamed rather than fried: they hold a lot of water, so give them a wide hot pan with a little oil, one layer and no lid, and let that water cook off before you turn them. A pinch of salt only at the end, or they give up their water early and stew.",
        ),
    ),
    (
        "veg-tough",
        "why are my carrots woody",
        Some(
            "Roots go hard in the middle when the pieces are uneven: cut them no bigger than a thumb, start them in cold salted water and simmer 15 to 20 minutes, or roast them covered in foil first. If the core is woody, cut it out - an old carrot or parsnip will not soften however long it cooks.",
        ),
    ),
    (
        "veg-tough",
        "my roasted sprouts are tough",
        Some(
            "Slice it thinner and give it longer: a brassica goes tough when the stalk is undercooked, so cut a cross in the stalk or slice it thin and steam 5 to 8 minutes, until a knife slides in. If it has turned grey and sulphury it is overcooked instead, so give it two minutes less next time.",
        ),
    ),
    (
        "veg-tough",
        "kale stays tough",
        Some(
            "Strip the leaves off the stalks, because the stalk is what stays tough: steam the leaves 3 to 4 minutes, or rub kale with oil and salt for a minute to soften it for a salad without any heat.",
        ),
    ),
    (
        "veg-tough",
        "my courgettes are still hard",
        Some(
            "Give it longer in a hot oven, 200C, until it collapses and the edges colour: these are mostly water, so they taste raw and squeaky until that water cooks out. Salt aubergine and courgette first and dry them, and spread them out so they roast rather than steam.",
        ),
    ),
    (
        "veg-tough",
        "my celery is stringy",
        Some(
            "Slice it thinner and cook it slower: leeks, onions and celery stay tough when they go into a dry pan over high heat, so give them a knob of butter, a splash of water and a lid for 10 minutes. Only the dark green tops of a leek are tough enough to keep back - put those in a stock instead.",
        ),
    ),
    (
        "veg-tough",
        "a tough green bean",
        Some(
            "Beans go tough when they are old or undercooked: top and tail them, pull the string down the seam of runners, and boil 4 to 5 minutes in salted water. Peas and mangetout need 2 minutes at most, and a minute in a hot pan at the end brings the bite back.",
        ),
    ),
    ("veg-tough", "my dried haricot beans still are hard", None),
    ("veg-tough", "my puy lentils are hard", None),
    (
        "veg-tough",
        "why are boiled leeks still firm?",
        Some(
            "Slice it thinner and cook it slower: leeks, onions and celery stay tough when they go into a dry pan over high heat, so give them a knob of butter, a splash of water and a lid for 10 minutes. Only the dark green tops of a leek are tough enough to keep back - put those in a stock instead.",
        ),
    ),
    (
        "veg-tough",
        "is the roasted cauliflower under cooked",
        Some(
            "Slice it thinner and give it longer: a brassica goes tough when the stalk is undercooked, so cut a cross in the stalk or slice it thin and steam 5 to 8 minutes, until a knife slides in. If it has turned grey and sulphury it is overcooked instead, so give it two minutes less next time.",
        ),
    ),
    (
        "veg-tough",
        "why is the stir fry veg raw",
        Some(
            "Cut it into even pieces, no thicker than a finger, and give it longer: a tough vegetable is nearly always undercooked in the middle. Salted water for boiling, 200C in a single layer for roasting, and a knife should slide in before it comes off the heat.",
        ),
    ),
    ("veg-tough", "the potato is purple", None),
    ("veg-tough", "hard thought", None),
    (
        "veg-tough",
        "my baked sweet potatoes are not cooked",
        Some(
            "Boiled potatoes go hard in the middle when the pieces are uneven: cut them the same size, start them in cold salted water and simmer 15 to 20 minutes, until a knife slides in. An old potato can be woody at the core, and that will not soften however long it cooks.",
        ),
    ),
];
