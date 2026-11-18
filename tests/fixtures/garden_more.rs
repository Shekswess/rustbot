// Pinned PyBot garden callbacks read in isolation at 7d63c7f.
pub(super) struct Case(
    pub(super) &'static str,
    pub(super) &'static str,
    pub(super) Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "raised-bed",
        "how much soil for a raised bed",
        Some(
            "Work it out as a volume: length times width times depth, all in metres, and add a tenth for settling. Bulk loam-based compost from a builders' merchant is far cheaper than bags, and two parts topsoil to one part compost fills it better than pure compost, which sinks.",
        ),
    ),
    Case(
        "raised-bed",
        "bed dries out and needs more compost",
        Some(
            "A raised bed dries out much faster than the ground, because the roots are above it on all four sides: water when the top 2cm is dry, a good soak twice a week in summer rather than a daily sprinkle, and a mulch of compost or grass clippings holds it. Thirty centimetres of soil holds far longer than fifteen.",
        ),
    ),
    Case(
        "raised-bed",
        "lining the bottom of raised beds",
        Some(
            "Line the bottom with cardboard to smother the weeds, then fill it free: logs, twigs, leaves and any garden waste you have, then two parts topsoil to one part compost on top. Skip rubble, gravel and plastic, and keep it no wider than about 1.2m so you can reach the middle from both sides without standing on the soil.",
        ),
    ),
    Case(
        "raised-bed",
        "can a raised bed go on grass",
        Some(
            "Straight onto the grass with no digging: mow it short, lay cardboard or four layers of newspaper over the whole footprint, soak it, and fill on top - the grass dies underneath and the worms pull it in. Keep it no wider than about 1.2m so you can reach the middle from both sides.",
        ),
    ),
    Case(
        "raised-bed",
        "putting a raised bed on slabs",
        Some(
            "On slabs or concrete it needs no cardboard and no digging: stand the frame on the paving, leave a gap at the bottom edge so water runs off, and be ready to water more, because a bed on slabs dries out faster than one on soil. Twenty centimetres of soil is enough for salad, herbs and dwarf beans.",
        ),
    ),
    Case(
        "raised-bed",
        "what to grow in a raised bed",
        Some(
            "In twenty centimetres of soil: salad, herbs, radishes, dwarf beans, onions and carrots. Potatoes, tomatoes, courgettes and anything tall want forty or more, because their roots and their water need the depth. Keep it no wider than about 1.2m so you can reach the middle from both sides without standing on the soil.",
        ),
    ),
    Case(
        "raised-bed",
        "what kind of soil in a raised bed",
        Some(
            "Fill it with a mix of topsoil and compost rather than pure compost, which dries out and sinks: two parts topsoil to one part compost, and a handful of grit if it holds water. Twenty centimetres is enough for salad, herbs and dwarf beans, thirty for carrots, onions and beetroot.",
        ),
    ),
    Case(
        "raised-bed",
        "what timber for a raised bed",
        Some(
            "Untreated timber is fine and cheap and lasts about five years; tanalised timber 8 to 10 years; old bricks or paving slabs cost nothing and last longer, and creosoted railway sleepers stay away from anything you eat. Keep it no wider than about 1.2m so you can reach the middle.",
        ),
    ),
    Case(
        "raised-bed",
        "how deep should a raised bed be",
        Some(
            "Twenty centimetres deep does salad, herbs and dwarf beans; thirty for carrots, onions and beetroot; forty or more for potatoes and tomatoes. Untreated timber is fine and cheap, line the bottom with cardboard to smother the weeds, and fill it with a mix of topsoil and compost rather than pure compost, which dries out and sinks. Keep it no wider than about 1.2m so you can reach the middle from both sides without standing on the soil.",
        ),
    ),
    Case(
        "harvest-bean",
        "when to pick broad beans",
        Some(
            "Broad beans are ready from June, when the pods bulge and the beans inside are the size of a thumbnail: pick from the bottom of the plant upwards, and pinch out the tops once the flowers have set to stop blackfly. Runner and french beans follow from July.",
        ),
    ),
    Case(
        "harvest-bean",
        "when to pick runner beans",
        Some(
            "Runner and french beans are ready from July: pick them at 15cm while they are slim and snap without a string. Broad beans are a month earlier, when the pods bulge. Pick every couple of days or the plants stop setting.",
        ),
    ),
    Case(
        "harvest-bean",
        "are french beans ready",
        Some(
            "Runner and french beans are ready from July: pick them at 15cm while they are slim and snap without a string. Broad beans are a month earlier, when the pods bulge. Pick every couple of days or the plants stop setting.",
        ),
    ),
    Case(
        "harvest-courgette",
        "when can i harvest courgettes",
        Some(
            "Pick courgettes from July when they are 10 to 15cm long, and go over the plants every day in warm weather: one left to swell to marrow size stops the plant producing. Cut them with a short stalk and eat them within a few days.",
        ),
    ),
    Case(
        "harvest-courgette",
        "when do i pick my squash",
        Some(
            "Pumpkins and squash are ready in September or October, when the skin has hardened so a fingernail will not mark it and the stalk has gone corky: cut with a long handle, cure them in the sun for a week, and they keep for months somewhere dry.",
        ),
    ),
    Case(
        "harvest-courgette",
        "how can i tell when pumpkins are ready",
        Some(
            "Pumpkins and squash are ready in September or October, when the skin has hardened so a fingernail will not mark it and the stalk has gone corky: cut with a long handle, cure them in the sun for a week, and they keep for months somewhere dry.",
        ),
    ),
    Case(
        "harvest-cabbage",
        "when can i harvest broccoli",
        Some(
            "Cut broccoli while the head is still tight and green, before any yellow shows: about ten to twelve weeks after planting out for a summer kind. Cut the central head first with 10cm of stem, then the side shoots over the weeks after.",
        ),
    ),
    Case(
        "harvest-cabbage",
        "when do i pick cauliflower",
        Some(
            "Cut a cauliflower when the curd is still tight and white, ten to sixteen weeks after planting out: bend the outer leaves over the head to keep it white, and cut it before the curd loosens or looks grainy.",
        ),
    ),
    Case(
        "harvest-cabbage",
        "can i pick the sprouts now",
        Some(
            "Pick sprouts from the bottom of the stalk upwards from October, after a frost or two, twisting each one off when it is tight and walnut-sized. The stalk goes on producing upwards for weeks, and the tops are eaten as spring greens.",
        ),
    ),
    Case(
        "harvest-cabbage",
        "when to cut a sprout",
        Some(
            "Pick sprouts from the bottom of the stalk upwards from October, after a frost or two, twisting each one off when it is tight and walnut-sized. The stalk goes on producing upwards for weeks, and the tops are eaten as spring greens.",
        ),
    ),
    Case(
        "harvest-cabbage",
        "how to pick kale",
        Some(
            "Pick kale leaf by leaf from the top of the plant, never more than a third at once, and it goes on cropping all winter; the young leaves at the crown are the tender ones.",
        ),
    ),
    Case(
        "harvest-cabbage",
        "when do i cut cabbage",
        Some(
            "Cut cabbages when the heart is firm: spring and summer kinds about three months after planting out, winter ones from October. Cut rather than pull, so the stump can throw a second small head.",
        ),
    ),
    Case(
        "harvest-cabbage",
        "spring is the key time",
        Some(
            "Cut cabbages when the heart is firm: spring and summer kinds about three months after planting out, winter ones from October. Cut rather than pull, so the stump can throw a second small head. Pick sprouts from the bottom of the stalk upwards from October, after a frost or two, twisting each one off when it is tight and walnut-sized. The stalk goes on producing upwards for weeks, and the tops are eaten as spring greens. Pick kale leaf by leaf from the top of the plant, never more than a third at once, and it goes on cropping all winter; the young leaves at the crown are the tender ones.",
        ),
    ),
    Case(
        "lawn-care",
        "how often should i aerate my lawn",
        Some(
            "Aerate once a year, in early autumn while the grass is still growing and the ground is soft: push a garden fork in about 10cm deep at 15cm intervals and rock it back, which is enough for most lawns, or hire a hollow-tine machine for compacted clay or a lawn that people walk on all summer. Brush sharp sand or a top dressing into the holes afterwards, leave the plugs on the grass to break down, and water it if the weather stays dry for a week. A lawn that is bad enough to want aerating twice a year wants the thatch scarified out first, a fortnight before.",
        ),
    ),
    Case(
        "lawn-care",
        "how do i scarify a lawn with dead grass",
        Some(
            "Scarify in early autumn, or in spring on a lawn that has gone spongy: set a spring-tine rake or a machine to just touch the surface and pull the dead thatch out in two passes, one way across the lawn and one up and down, taking off no more than a centimetre at a time. Clear what comes up, water it if the weather stays dry, and feed it a fortnight later rather than the same day. Once a year is plenty, and a lawn with less than a centimetre of thatch does not need it at all.",
        ),
    ),
    Case(
        "lawn-care",
        "when can i reseed grass",
        Some(
            "Reseed in early autumn, or in spring once the soil has warmed up: fork the bare patch over to loosen it, rake it level, scatter the seed at the rate on the box, and press it in with a board or the back of a rake so it is in contact with the soil. Keep it damp for a fortnight - a light sprinkle morning and evening beats one soak - and keep off it until the new grass is about 5cm tall.",
        ),
    ),
    Case(
        "lawn-care",
        "my lawn is dull and pale",
        Some(
            "Deeper green comes from feeding and watering rather than from seed or a spray: a spring lawn feed with nitrogen in it, and a deep soak once a week in dry weather instead of a daily sprinkle. Mow little and often in the growing season, not shorter than about 4cm, and leave the clippings off unless it is long. Scarify and aerate in autumn, and reseed the bare patches.",
        ),
    ),
    Case(
        "lawn-care",
        "my lawn died in patches",
        Some(
            "Yellow or brown patches in dry weather are usually thirst - water deeply once a week in a dry spell rather than a sprinkle daily - and thin pale grass in spring wants a feed rather than a reseed. Mow little and often in the growing season, not shorter than about 4cm, and leave the clippings off unless it is long. Scarify and aerate in autumn, and reseed the bare patches.",
        ),
    ),
    Case(
        "lawn-care",
        "my lawn is dead",
        Some(
            "A brown lawn is usually dormant rather than dead, so check before you dig it up: pull a tuft - if the roots hold and there is green at the base it will come back after a week of deep watering, and if the tuft lifts like a mat it has gone. Dead patches want the old grass raked out and reseeded in early autumn, when the ground is warm and the rain does the watering. Scarify and aerate in autumn, and reseed the bare patches.",
        ),
    ),
    Case(
        "lawn-care",
        "my grass is yellow",
        Some(
            "Yellow or brown patches in dry weather are usually thirst - water deeply once a week in a dry spell rather than a sprinkle daily - and thin pale grass in spring wants a feed rather than a reseed. Mow little and often in the growing season, not shorter than about 4cm, and leave the clippings off unless it is long. Scarify and aerate in autumn, and reseed the bare patches.",
        ),
    ),
    Case(
        "lawn-care",
        "how do i mow the lawn",
        Some(
            "Mow little and often in the growing season, not shorter than about 4cm, and leave the clippings off unless it is long. Scarify and aerate in autumn, reseed bare patches, and water deeply once a week in dry spells rather than a sprinkle daily.",
        ),
    ),
    Case(
        "sow-fruit-prune",
        "when to prune cherry trees",
        Some(
            "Prune plums, cherries and damsons in June, not in winter: silver leaf gets in through a winter cut and kills a branch a year later. Cut back to a growing shoot and leave the pruning cuts open to the air rather than painting them. Prune apples and pears in January or February while they are dormant: dead, diseased and crossing branches out first, then the oldest third of the stems, and the middle kept open.",
        ),
    ),
    Case(
        "sow-fruit-prune",
        "how do i prune a plum",
        Some(
            "Prune plums, cherries and damsons in June, not in winter: silver leaf gets in through a winter cut and kills a branch a year later. Cut back to a growing shoot and leave the pruning cuts open to the air rather than painting them. Prune apples and pears in January or February while they are dormant: dead, diseased and crossing branches out first, then the oldest third of the stems, and the middle kept open.",
        ),
    ),
    Case(
        "sow-fruit-prune",
        "prune a pear tree",
        Some(
            "Prune apples and pears in January or February while they are dormant: dead, diseased and crossing branches out first, then the oldest third of the stems, and the middle kept open. Prune plums, cherries and damsons in June, not in winter: silver leaf gets in through a winter cut and kills a branch a year later. Cut back to a growing shoot and leave the pruning cuts open to the air rather than painting them.",
        ),
    ),
    Case(
        "prune-shrub",
        "how do i prune buddleia",
        Some(
            "Cut a buddleia hard in March, back to about 30cm: it flowers on the shoots that grow from there, so the harder you cut the bigger the spikes. Do it every year, and take out any dead stems at the base at the same time. A neglected one can lose two thirds of its height at once and still come back.",
        ),
    ),
    Case(
        "prune-shrub",
        "when to prune a wisteria",
        Some(
            "Wisteria is pruned twice: in July or August shorten the long whippy shoots to five or six leaves, then in January cut those same shoots back to two or three buds. That is what turns growth into flowers. Leave any shoot you are training out longer, and do not feed it with nitrogen, which gives you leaves instead.",
        ),
    ),
    Case(
        "prune-shrub",
        "how hard can i prune a hydrangea",
        Some(
            "Leave the old flowers on over winter for frost protection, then in March cut each stem back to a fat pair of buds about 30cm up and take out the oldest stems at the base. Do not cut a mophead to the ground: it flowers on last year's wood. A paniculata kind is the exception and flowers best cut hard in spring.",
        ),
    ),
    Case(
        "prune-shrub",
        "when to prune raspberries",
        Some(
            "Summer-fruiting raspberries: cut the canes that fruited this year down to the ground in February and tie in the new ones. Autumn-fruiting ones are easier, because every cane is cut to the ground in February and the fruit comes on the new growth the same year. Take out the weak and crowded canes either way.",
        ),
    ),
    Case(
        "prune-shrub",
        "how to trim bay",
        Some(
            "Trim an evergreen shrub in mid to late spring, once the frosts are over: bay, holly, laurel and box all take a tidy-up then, and bay takes a light trim in August too. Do not cut back into bare old wood, because most evergreens will not shoot again from it - a hard cut is better spread over two years.",
        ),
    ),
    Case(
        "prune-shrub",
        "when to prune forsythia",
        Some(
            "Prune it straight after it has flowered, not in winter: it made this year's flower buds last summer, so a spring cut takes the flowers with it. Take the flowered shoots back by about a third as the blooms fade, and leave the rest of the growth alone until next year.",
        ),
    ),
    Case(
        "prune-shrub",
        "cutting back a hebe",
        Some(
            "Prune it in late winter or early spring, around March, while it is dormant: it flowers on the shoots that grow this year, so cutting it back then gives the strongest growth and the most flower. Take out dead and crossing stems first, then up to a third of the oldest stems at the base.",
        ),
    ),
    Case(
        "prune-shrub",
        "when to prune a shrub",
        Some(
            "Shrubs want one of two dates. If it flowers before midsummer, cut it just after flowering, because the buds were made last year. If it flowers later, cut it in March while it is dormant, because the flowers come on the new growth. Take out dead and crossing stems first, then a third of the oldest at the base.",
        ),
    ),
    Case(
        "herb-pruning",
        "how do i keep parsley over winter",
        Some(
            "Most soft herbs die down and come again rather than dying: chives and mint go to nothing and return from the roots in spring, parsley is a biennial so its second year is thin and woody, and thyme, sage, oregano and rosemary stay green through it. Before the first frost, move the pots against a wall and off the ground, cut the old stems down to about 5cm and mulch around the crowns. Then leave them alone: no feed, and water only when the compost has dried right out, because cold and wet kills more herbs than frost does.",
        ),
    ),
    Case(
        "herb-pruning",
        "my mint is spreading everywhere",
        Some(
            "Mint spreads by runners just under the surface, so pulling it up makes it worse. Sink it in a pot with the rim 2cm above the soil, or a bottomless bucket, or grow it in a container away from the bed. In a border already taken over, fork out the runners in spring and again in autumn for two years, and cut the stems before they flower so it cannot seed.",
        ),
    ),
    Case(
        "herb-pruning",
        "my basil has gone to seed",
        Some(
            "A herb goes woody when it has been allowed to flower: the stems harden and it stops making leaf. Pinch the flower buds off as soon as they show, keep picking the tips, and cut a third of the oldest stems back to a low leaf pair in spring. Basil is the one that will not come back from old wood for long, so take cuttings in late summer or sow a fresh pot in June rather than fight it.",
        ),
    ),
    Case(
        "herb-pruning",
        "when should i prune rosemary",
        Some(
            "Take the top third back in spring, once the worst of the frost has gone and before the plant starts into growth: that is the one hard cut rosemary, thyme, sage and oregano need, and it is what stops them going leggy and bare in the middle. Never cut into the old bare wood, because a stem cut back to it usually does not come again, and a light trim over the tips after flowering holds the shape in between.",
        ),
    ),
    Case(
        "herb-pruning",
        "when to pinch basil",
        Some(
            "Pick basil, mint, parsley, coriander and chives from the top all through the growing season, from June into September: pinch or snip the top pair of leaves every week or two and the plant bushes out instead of running up into one tall stem. There is no yearly hard cut for these - stop at a leaf pair, because a soft stem cut back to bare wood usually does not come again - and take a few stems at a time rather than stripping the plant in one go.",
        ),
    ),
    Case(
        "herb-pruning",
        "how do i harvest coriander",
        Some(
            "Pinch the top pair of leaves off each stem, just above the next pair down: every stem you tip sends out two sideshoots, so the plant bushes out instead of running up into one tall stem. Cut no lower than a leaf pair on basil, mint and coriander, because a stem cut back to bare wood usually does not come again. Pick from the top, a few stems at a time, and never strip a plant in one go. Rosemary, thyme and sage are the woody ones and are not pinched like basil: take the top third back in spring, once the worst of the frost has gone, and never cut into the old bare wood.",
        ),
    ),
    Case(
        "herb-pruning",
        "how can i look after herbs",
        Some(
            "Soft herbs want sun, drainage and picking: water when the top of the soil is dry, feed weakly, and take the tips off often so the plant keeps growing leaf instead of stem. Basil and coriander are annuals, so a pot that lasts a season is normal; mint, thyme, oregano, sage and rosemary are perennials and want cutting back in spring.",
        ),
    ),
    Case("herb-pruning", "how do i cut back a tree", None),
    Case(
        "seedlings-leggy",
        "my seedlings have gone leggy",
        Some(
            "Seedlings go leggy for one reason: not enough light, which is worse indoors on a windowsill than in a greenhouse, and worse still too warm. Move them somewhere brighter and cooler, turn the tray each day so they do not lean, and prick them out deeper than they were - a tomato or a brassica will make roots along the buried stem. Next sowing, thin them out sooner and sow a little later in the year.",
        ),
    ),
    Case(
        "seedlings-leggy",
        "why are my seedlings falling over",
        Some(
            "Seedlings go leggy for one reason: not enough light, which is worse indoors on a windowsill than in a greenhouse, and worse still too warm. Move them somewhere brighter and cooler, turn the tray each day so they do not lean, and prick them out deeper than they were - a tomato or a brassica will make roots along the buried stem. Next sowing, thin them out sooner and sow a little later in the year.",
        ),
    ),
    Case("seedlings-leggy", "my seedlings failed to germinate", None),
    Case(
        "cactus-soft",
        "my cactus is squishy",
        Some(
            "A cactus or succulent that has gone soft is rotting from too much water: the tissue goes squishy and translucent from the base up, and it will not firm up again. Stop watering, take it out of the pot and cut away every soft part with a clean knife until the flesh is firm and clean, leave the cut to callous for a week, then set it on dry gritty compost. Water only when the compost is dry right through, and never leave the pot standing in water.",
        ),
    ),
    Case(
        "cactus-soft",
        "my cacti have collapsed",
        Some(
            "A cactus or succulent that has gone soft is rotting from too much water: the tissue goes squishy and translucent from the base up, and it will not firm up again. Stop watering, take it out of the pot and cut away every soft part with a clean knife until the flesh is firm and clean, leave the cut to callous for a week, then set it on dry gritty compost. Water only when the compost is dry right through, and never leave the pot standing in water.",
        ),
    ),
    Case("cactus-soft", "my cactus is prickly", None),
    Case(
        "dig-over-bed",
        "when to dig over a vegetable bed",
        Some(
            "Autumn, from October into December, once the beds are emptying: dig it over then and the frost does the work of breaking the clods down over winter, so the soil is workable in spring. On heavy clay that is the one time it is worth doing, because digging wet clay turns it into brick and digging dry clay leaves the lumps like stones. If you have to do it in spring, fork it over rather than digging.",
        ),
    ),
    Case(
        "dig-over-bed",
        "how to dig over the bed",
        Some(
            "One spade deep is enough for most beds: turn the soil over, break up the clods and fork a couple of buckets of compost or well-rotted manure into each square metre as you go. Pick out every white root of bindweed and nettle as it comes up, or it will be back within weeks. Autumn is the best time, because the frost breaks the clods down for you, and leave it alone if the soil is wet enough to ball in your hand - digging clay when it is wet turns it into brick.",
        ),
    ),
    Case(
        "dig-over-bed",
        "what month should i dig over my veggie patch",
        Some(
            "Autumn, from October into December, once the beds are emptying: dig it over then and the frost does the work of breaking the clods down over winter, so the soil is workable in spring. On heavy clay that is the one time it is worth doing, because digging wet clay turns it into brick and digging dry clay leaves the lumps like stones. If you have to do it in spring, fork it over rather than digging.",
        ),
    ),
    Case("watering-garden", "how do i dry flowers", None),
    Case(
        "watering-garden",
        "why does my flower bed dry out so fast",
        Some(
            "A raised bed, a pot and a sandy border all lose water faster than a clay border does: the compost drains freely, sun and wind take it off the surface, and roots that have filled the pot drink what is left. Water until it runs out of the bottom and check a spade's depth down before deciding it is dry, then spread 5cm of compost or bark over the surface - that mulch is what stops the next lot evaporating, and a light sprinkle every day never reaches the roots at all.",
        ),
    ),
    Case(
        "watering-garden",
        "how often should i water my flower beds",
        Some(
            "Water deeply and less often, in the evening or early morning: a good soak twice a week in a dry spell beats a light sprinkle every day, because it reaches the roots instead of wetting the surface. Aim at the soil, not the leaves. Established shrubs, borders and perennials need nothing until the soil is dry a spade's depth down, while anything newly planted and anything in a pot is the exception and wants water every day or two in heat.",
        ),
    ),
    Case(
        "rose-care",
        "how often to water a rose",
        Some(
            "A rose in the ground wants a full watering can per plant each week in dry weather, poured at the base rather than over the leaves, and a rose in a pot wants that much every other day in summer, because a pot dries out far faster than the soil. Water in the morning, and give it a proper soak: a light sprinkle every evening only wets the top inch, and the roots stay shallow and thirsty.",
        ),
    ),
    Case(
        "rose-care",
        "what do i feed a rose",
        Some(
            "Feed a rose with a rose feed, not a general one, from the first leaves in spring until the end of July: a handful of granules round the base in March and again in June, watered in, or a liquid feed every fortnight while it flowers. Stop by August so the new growth hardens before winter, and never feed a dry plant - water it first, or the feed burns the roots.",
        ),
    ),
    Case(
        "rose-care",
        "my roses are dying",
        Some(
            "Cut roses die in a day or two for a simple reason: trim 2cm off each stem at an angle, strip any leaf that would sit in the water, change the water every other day and keep them out of sun. A rose in the ground that looks dead is usually thirsty or hungry instead: a full can a week at the base, a rose feed while it flowers, and cut the brown stems back to green wood - one dry fortnight drops every leaf before the plant is dead.",
        ),
    ),
    Case(
        "rose-care",
        "rose leaves on my rose bush are yellow",
        Some(
            "A rose in the ground is nearly always thirsty, hungry or short of air: water it at the base, a full can a week in dry spells, more in a pot, and feed it with a rose feed while it flowers. Cut out the yellowing and spotted leaves and bin them rather than compost them, because black spot and rust live on the fallen leaves and splash back up in the rain. If the stems themselves are brown and dry, prune those back to green wood and water: a rose left dry for a fortnight drops its leaves and looks dead before it is.",
        ),
    ),
];
