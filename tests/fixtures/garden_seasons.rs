//! Direct pinned replies and winter guards with an injected month.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub u8,
    pub bool,
    pub Option<&'static str>,
);

pub(super) const CASES: &[Case] = &[
    Case(
        "sow-month",
        "what can i sow now",
        1,
        true,
        Some(
            "It's January, so here's what goes in. Not much goes in this month: plant garlic and shallots if the ground is not frozen, and start broad beans under cloches from late January. Chit early potatoes on a windowsill and order the seed you want for spring.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow now",
        3,
        true,
        Some(
            "It's March, so here's what goes in. The month the plot opens: potatoes, carrots, onions, lettuce, peas, leeks, radish and cabbages all go in from now. Sow a short row every few weeks rather than one big sowing, so the crop comes over months instead of all at once.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow now",
        6,
        true,
        Some(
            "It's June, so here's what goes in. Last chance for runner and french beans, courgettes and swede; keep salad and radish going. Water in the morning and mulch, because this is the month everything bolts.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow now",
        9,
        true,
        Some(
            "It's September, so here's what goes in. Sow winter salad and land cress, plant overwintering onion sets and garlic, and clear the spent crops into the compost. This is the month that decides next spring.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow now",
        12,
        true,
        Some(
            "It's December, so here's what goes in. Nothing goes in outdoors except garlic in a mild spell. Plan next year's rotation, order your seed, and start chitting early potatoes indoors after Christmas.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow next month",
        1,
        true,
        Some(
            "It's February, so here's what goes in. Broad beans and parsnips from late February, and onion sets and shallots as soon as the ground is workable. Sow tomatoes and chillies indoors under cover, and chit the early potatoes.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow next month",
        3,
        true,
        Some(
            "It's April, so here's what goes in. Potatoes by mid-April, then beetroot and beans, courgettes indoors, and more lettuce and radish every three weeks. Keep fleece to hand: a late frost is the thing that sets April back.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow next month",
        6,
        true,
        Some(
            "It's July, so here's what goes in. Sow for the winter now: spring cabbage, kale, winter salad and more radish and lettuce for autumn. Lift onions and garlic as the tops fall over.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow next month",
        9,
        true,
        Some(
            "It's October, so here's what goes in. Plant garlic and overwintering broad beans for the earliest crop next year, and put in onion sets and shallots. Outdoors there is little else to sow except winter salad under cover, so clear and mulch the beds.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow next month",
        12,
        true,
        Some(
            "It's January, so here's what goes in. Not much goes in this month: plant garlic and shallots if the ground is not frozen, and start broad beans under cloches from late January. Chit early potatoes on a windowsill and order the seed you want for spring.",
        ),
    ),
    Case(
        "sow-month",
        "what can i put in this month",
        1,
        true,
        Some(
            "It's January, so here's what goes in. Not much goes in this month: plant garlic and shallots if the ground is not frozen, and start broad beans under cloches from late January. Chit early potatoes on a windowsill and order the seed you want for spring.",
        ),
    ),
    Case(
        "sow-month",
        "what can i put in this month",
        3,
        true,
        Some(
            "It's March, so here's what goes in. The month the plot opens: potatoes, carrots, onions, lettuce, peas, leeks, radish and cabbages all go in from now. Sow a short row every few weeks rather than one big sowing, so the crop comes over months instead of all at once.",
        ),
    ),
    Case(
        "sow-month",
        "what can i put in this month",
        6,
        true,
        Some(
            "It's June, so here's what goes in. Last chance for runner and french beans, courgettes and swede; keep salad and radish going. Water in the morning and mulch, because this is the month everything bolts.",
        ),
    ),
    Case(
        "sow-month",
        "what can i put in this month",
        9,
        true,
        Some(
            "It's September, so here's what goes in. Sow winter salad and land cress, plant overwintering onion sets and garlic, and clear the spent crops into the compost. This is the month that decides next spring.",
        ),
    ),
    Case(
        "sow-month",
        "what can i put in this month",
        12,
        true,
        Some(
            "It's December, so here's what goes in. Nothing goes in outdoors except garlic in a mild spell. Plan next year's rotation, order your seed, and start chitting early potatoes indoors after Christmas.",
        ),
    ),
    Case(
        "sow-month",
        "what do i sow in june",
        6,
        true,
        Some(
            "Last chance for runner and french beans, courgettes and swede; keep salad and radish going. Water in the morning and mulch, because this is the month everything bolts.",
        ),
    ),
    Case(
        "sow-month",
        "what should i sow in may",
        6,
        true,
        Some(
            "Once the frost has gone: beans, courgettes, sweetcorn, squash and swede go out, and keep sowing lettuce and radish for a steady supply. Harden everything off for a week first.",
        ),
    ),
    Case("sow-month", "may i plant carrots", 6, true, None),
    Case(
        "sow-month",
        "what can i plant for winter",
        6,
        true,
        Some(
            "Autumn is the sowing for next year: plant garlic, overwintering broad beans and onion sets, and sow winter salad and land cress under cover. Clear the spent crops into the compost and mulch the empty beds, because this is the season that decides next spring.",
        ),
    ),
    Case(
        "sow-month",
        "what can i plant in winter",
        6,
        true,
        Some(
            "Almost nothing goes in outdoors: garlic and broad beans in a mild spell, and everything else indoors. Sow tomatoes and chillies under cover from late winter, chit early potatoes on a windowsill after Christmas, and order the seed you want for the year.",
        ),
    ),
    Case(
        "sow-month",
        "what can i plant in spring",
        6,
        true,
        Some(
            "Spring is when the plot fills up: potatoes, carrots, onions, lettuce, peas, leeks and radish from March, beetroot and beans in April, and courgettes, sweetcorn and squash once the frost has gone in May. Sow a short row every few weeks rather than one big sowing.",
        ),
    ),
    Case(
        "sow-month",
        "what is there to grow in autumn",
        6,
        true,
        Some(
            "Autumn is the sowing for next year: plant garlic, overwintering broad beans and onion sets, and sow winter salad and land cress under cover. Clear the spent crops into the compost and mulch the empty beds, because this is the season that decides next spring.",
        ),
    ),
    Case(
        "sow-month",
        "what can i sow in march and november",
        6,
        true,
        Some(
            "The month the plot opens: potatoes, carrots, onions, lettuce, peas, leeks, radish and cabbages all go in from now. Sow a short row every few weeks rather than one big sowing, so the crop comes over months instead of all at once.",
        ),
    ),
    Case("sow-month", "what should i sow", 6, true, None),
    Case(
        "garden-jobs-in-a-season",
        "garden jobs today",
        1,
        true,
        Some(
            "The garden in winter is mostly tidying and waiting: clear the dead stems but leave seed heads and any hollow stalks standing, because insects shelter in them and the birds eat the seeds. Mulch bare soil with compost or bark, but only once the ground is not frozen, and stay off wet or frozen grass because footprints kill the blades under them. Prune apples and pears and any shrub that flowers on this year's wood, clean and sharpen the tools, and wash out the greenhouse with a mild disinfectant before spring sowing. Pots are what the cold kills, so move them against a wall, raise them onto feet so the drainage hole does not sit in water, and give tender plants a layer of bubble wrap. Check the stored produce and the dahlias every couple of weeks for rot, and order seeds in January, because the quiet months are the only time the garden has nothing urgent in it.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "garden jobs today",
        3,
        true,
        Some(
            "Spring goes in this order: rake the lawn and fork the beds once the soil has dried out enough not to clod, mulch the borders with compost before the weeds start, and prune the shrubs that flower on the wood they made last year - roses, buddleia, fuchsia - while they are still dormant. Sow the hardy veg outdoors, harden off anything you raised indoors for a week before it goes out, and feed the lawn once the grass is growing. Then watch the forecast, because a late frost is what takes the blossom and the new leaves, and a sheet of fleece over a flowering fruit tree earns its keep.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "garden jobs today",
        6,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "garden jobs today",
        9,
        true,
        Some(
            "Autumn is clearing and planting: pull the spent crops and annuals into the compost, plant bulbs and bare-root roses, fruit trees and hedging while the soil is still warm, and rake the leaves off the lawn and out of the pond. Cut back what has finished but leave the seed heads standing for the birds, mulch the beds, and give the lawn a late cut with an autumn feed rather than a short one. Then check what is stored - apples, onions, dahlias - for rot, and order next year's seed and bare-root plants, because the good varieties go early.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "garden jobs today",
        12,
        true,
        Some(
            "The garden in winter is mostly tidying and waiting: clear the dead stems but leave seed heads and any hollow stalks standing, because insects shelter in them and the birds eat the seeds. Mulch bare soil with compost or bark, but only once the ground is not frozen, and stay off wet or frozen grass because footprints kill the blades under them. Prune apples and pears and any shrub that flowers on this year's wood, clean and sharpen the tools, and wash out the greenhouse with a mild disinfectant before spring sowing. Pots are what the cold kills, so move them against a wall, raise them onto feet so the drainage hole does not sit in water, and give tender plants a layer of bubble wrap. Check the stored produce and the dahlias every couple of weeks for rot, and order seeds in January, because the quiet months are the only time the garden has nothing urgent in it.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what do i need to do in the garden in winter",
        6,
        true,
        Some(
            "The garden in winter is mostly tidying and waiting: clear the dead stems but leave seed heads and any hollow stalks standing, because insects shelter in them and the birds eat the seeds. Mulch bare soil with compost or bark, but only once the ground is not frozen, and stay off wet or frozen grass because footprints kill the blades under them. Prune apples and pears and any shrub that flowers on this year's wood, clean and sharpen the tools, and wash out the greenhouse with a mild disinfectant before spring sowing. Pots are what the cold kills, so move them against a wall, raise them onto feet so the drainage hole does not sit in water, and give tender plants a layer of bubble wrap. Check the stored produce and the dahlias every couple of weeks for rot, and order seeds in January, because the quiet months are the only time the garden has nothing urgent in it.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "spring jobs in the garden",
        6,
        true,
        Some(
            "Spring goes in this order: rake the lawn and fork the beds once the soil has dried out enough not to clod, mulch the borders with compost before the weeds start, and prune the shrubs that flower on the wood they made last year - roses, buddleia, fuchsia - while they are still dormant. Sow the hardy veg outdoors, harden off anything you raised indoors for a week before it goes out, and feed the lawn once the grass is growing. Then watch the forecast, because a late frost is what takes the blossom and the new leaves, and a sheet of fleece over a flowering fruit tree earns its keep.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what can i do in the garden in summer this week",
        1,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what can i do in the garden in summer this week",
        3,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what can i do in the garden in summer this week",
        6,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what can i do in the garden in summer this week",
        9,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what can i do in the garden in summer this week",
        12,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what jobs should i do in my allotment in august",
        6,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what can i do in the garden",
        6,
        true,
        None,
    ),
    Case(
        "garden-jobs-in-a-season",
        "what should i do in the greenhouse this weekend",
        1,
        true,
        Some(
            "The garden in winter is mostly tidying and waiting: clear the dead stems but leave seed heads and any hollow stalks standing, because insects shelter in them and the birds eat the seeds. Mulch bare soil with compost or bark, but only once the ground is not frozen, and stay off wet or frozen grass because footprints kill the blades under them. Prune apples and pears and any shrub that flowers on this year's wood, clean and sharpen the tools, and wash out the greenhouse with a mild disinfectant before spring sowing. Pots are what the cold kills, so move them against a wall, raise them onto feet so the drainage hole does not sit in water, and give tender plants a layer of bubble wrap. Check the stored produce and the dahlias every couple of weeks for rot, and order seeds in January, because the quiet months are the only time the garden has nothing urgent in it.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what should i do in the greenhouse this weekend",
        3,
        true,
        Some(
            "Spring goes in this order: rake the lawn and fork the beds once the soil has dried out enough not to clod, mulch the borders with compost before the weeds start, and prune the shrubs that flower on the wood they made last year - roses, buddleia, fuchsia - while they are still dormant. Sow the hardy veg outdoors, harden off anything you raised indoors for a week before it goes out, and feed the lawn once the grass is growing. Then watch the forecast, because a late frost is what takes the blossom and the new leaves, and a sheet of fleece over a flowering fruit tree earns its keep.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what should i do in the greenhouse this weekend",
        6,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what should i do in the greenhouse this weekend",
        9,
        true,
        Some(
            "Autumn is clearing and planting: pull the spent crops and annuals into the compost, plant bulbs and bare-root roses, fruit trees and hedging while the soil is still warm, and rake the leaves off the lawn and out of the pond. Cut back what has finished but leave the seed heads standing for the birds, mulch the beds, and give the lawn a late cut with an autumn feed rather than a short one. Then check what is stored - apples, onions, dahlias - for rot, and order next year's seed and bare-root plants, because the good varieties go early.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what should i do in the greenhouse this weekend",
        12,
        true,
        Some(
            "The garden in winter is mostly tidying and waiting: clear the dead stems but leave seed heads and any hollow stalks standing, because insects shelter in them and the birds eat the seeds. Mulch bare soil with compost or bark, but only once the ground is not frozen, and stay off wet or frozen grass because footprints kill the blades under them. Prune apples and pears and any shrub that flowers on this year's wood, clean and sharpen the tools, and wash out the greenhouse with a mild disinfectant before spring sowing. Pots are what the cold kills, so move them against a wall, raise them onto feet so the drainage hole does not sit in water, and give tender plants a layer of bubble wrap. Check the stored produce and the dahlias every couple of weeks for rot, and order seeds in January, because the quiet months are the only time the garden has nothing urgent in it.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what was i doing yesterday",
        6,
        true,
        None,
    ),
    Case(
        "garden-jobs-in-a-season",
        "what jobs should i do about my roses now",
        1,
        true,
        Some(
            "The garden in winter is mostly tidying and waiting: clear the dead stems but leave seed heads and any hollow stalks standing, because insects shelter in them and the birds eat the seeds. Mulch bare soil with compost or bark, but only once the ground is not frozen, and stay off wet or frozen grass because footprints kill the blades under them. Prune apples and pears and any shrub that flowers on this year's wood, clean and sharpen the tools, and wash out the greenhouse with a mild disinfectant before spring sowing. Pots are what the cold kills, so move them against a wall, raise them onto feet so the drainage hole does not sit in water, and give tender plants a layer of bubble wrap. Check the stored produce and the dahlias every couple of weeks for rot, and order seeds in January, because the quiet months are the only time the garden has nothing urgent in it.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what jobs should i do about my roses now",
        3,
        true,
        Some(
            "Spring goes in this order: rake the lawn and fork the beds once the soil has dried out enough not to clod, mulch the borders with compost before the weeds start, and prune the shrubs that flower on the wood they made last year - roses, buddleia, fuchsia - while they are still dormant. Sow the hardy veg outdoors, harden off anything you raised indoors for a week before it goes out, and feed the lawn once the grass is growing. Then watch the forecast, because a late frost is what takes the blossom and the new leaves, and a sheet of fleece over a flowering fruit tree earns its keep.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what jobs should i do about my roses now",
        6,
        true,
        Some(
            "Summer is watering, feeding and picking: soak the beds in the evening rather than sprinkling them every day, mulch while the soil is damp to hold that water in, and water the things you planted this year first, because their roots are still shallow. Deadhead the flowers so they keep coming, feed anything in a pot every week or so, and cut the lawn a little higher rather than scalping it, which browns it in a dry spell. Pick courgettes, beans and salad small, because a plant left to swell stops making more, and get the weeds out before they set seed.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what jobs should i do about my roses now",
        9,
        true,
        Some(
            "Autumn is clearing and planting: pull the spent crops and annuals into the compost, plant bulbs and bare-root roses, fruit trees and hedging while the soil is still warm, and rake the leaves off the lawn and out of the pond. Cut back what has finished but leave the seed heads standing for the birds, mulch the beds, and give the lawn a late cut with an autumn feed rather than a short one. Then check what is stored - apples, onions, dahlias - for rot, and order next year's seed and bare-root plants, because the good varieties go early.",
        ),
    ),
    Case(
        "garden-jobs-in-a-season",
        "what jobs should i do about my roses now",
        12,
        true,
        Some(
            "The garden in winter is mostly tidying and waiting: clear the dead stems but leave seed heads and any hollow stalks standing, because insects shelter in them and the birds eat the seeds. Mulch bare soil with compost or bark, but only once the ground is not frozen, and stay off wet or frozen grass because footprints kill the blades under them. Prune apples and pears and any shrub that flowers on this year's wood, clean and sharpen the tools, and wash out the greenhouse with a mild disinfectant before spring sowing. Pots are what the cold kills, so move them against a wall, raise them onto feet so the drainage hole does not sit in water, and give tender plants a layer of bubble wrap. Check the stored produce and the dahlias every couple of weeks for rot, and order seeds in January, because the quiet months are the only time the garden has nothing urgent in it.",
        ),
    ),
    Case("garden-in-winter", "garden jobs now", 6, false, None),
    Case(
        "garden-in-winter",
        "what do i need to do in the garden in winter",
        6,
        true,
        None,
    ),
    Case(
        "garden-in-winter",
        "what should i do this winter in the greenhouse",
        6,
        true,
        None,
    ),
    Case(
        "garden-in-winter",
        "what jobs should i do about my roses now",
        6,
        false,
        None,
    ),
    Case(
        "garden-in-winter",
        "what do i need to do now",
        6,
        true,
        None,
    ),
    Case(
        "garden-in-winter",
        "what should i do outside now",
        6,
        true,
        None,
    ),
    Case(
        "garden-in-winter",
        "garden jobs right now before the winter",
        6,
        true,
        None,
    ),
    Case(
        "garden-in-winter",
        "autumn garden jobs this month",
        6,
        true,
        None,
    ),
    Case(
        "crop-weeks-named",
        "how many weeks until my chilli is ready",
        6,
        true,
        Some(
            "Chillies are 16 to 20 weeks from seed, and they want warmth all of that time. The others, from seed: salad leaves and radishes 4 to 6 weeks, beetroot 10 to 12, carrots and runner beans 12 to 16, peas 12 to 14, courgettes 10 to 12, sweetcorn 12 to 14, tomatoes 16 to 20 from sowing (12 to 16 from planting out), cabbages and broccoli 16 to 20, parsnips 22 to 24, and leeks or onions 20 to 24. Garlic takes about eight months from an autumn clove, and strawberries fruit the summer after planting. The seed packet is the tie-breaker, because a fortnight either way is normal.",
        ),
    ),
    Case(
        "crop-weeks-named",
        "how long do carrots and parsnips take",
        6,
        true,
        Some(
            "Carrots are 12 to 16 weeks from seed. The others, from seed: salad leaves and radishes 4 to 6 weeks, beetroot 10 to 12, carrots and runner beans 12 to 16, peas 12 to 14, courgettes 10 to 12, sweetcorn 12 to 14, tomatoes 16 to 20 from sowing (12 to 16 from planting out), cabbages and broccoli 16 to 20, parsnips 22 to 24, and leeks or onions 20 to 24. Garlic takes about eight months from an autumn clove, and strawberries fruit the summer after planting. The seed packet is the tie-breaker, because a fortnight either way is normal.",
        ),
    ),
    Case(
        "crop-weeks-named",
        "how long from seed to spring onion",
        6,
        true,
        Some(
            "Spring onions are 8 to 10 weeks from seed. The others, from seed: salad leaves and radishes 4 to 6 weeks, beetroot 10 to 12, carrots and runner beans 12 to 16, peas 12 to 14, courgettes 10 to 12, sweetcorn 12 to 14, tomatoes 16 to 20 from sowing (12 to 16 from planting out), cabbages and broccoli 16 to 20, parsnips 22 to 24, and leeks or onions 20 to 24. Garlic takes about eight months from an autumn clove, and strawberries fruit the summer after planting. The seed packet is the tie-breaker, because a fortnight either way is normal.",
        ),
    ),
    Case(
        "crop-weeks-named",
        "how long from seed to radish",
        6,
        true,
        Some(
            "Radishes are 4 to 6 weeks from seed, the quickest crop there is. The others, from seed: salad leaves and radishes 4 to 6 weeks, beetroot 10 to 12, carrots and runner beans 12 to 16, peas 12 to 14, courgettes 10 to 12, sweetcorn 12 to 14, tomatoes 16 to 20 from sowing (12 to 16 from planting out), cabbages and broccoli 16 to 20, parsnips 22 to 24, and leeks or onions 20 to 24. Garlic takes about eight months from an autumn clove, and strawberries fruit the summer after planting. The seed packet is the tie-breaker, because a fortnight either way is normal.",
        ),
    ),
    Case(
        "crop-weeks-named",
        "what crops harvest quickest",
        6,
        true,
        Some(
            "The others, from seed: salad leaves and radishes 4 to 6 weeks, beetroot 10 to 12, carrots and runner beans 12 to 16, peas 12 to 14, courgettes 10 to 12, sweetcorn 12 to 14, tomatoes 16 to 20 from sowing (12 to 16 from planting out), cabbages and broccoli 16 to 20, parsnips 22 to 24, and leeks or onions 20 to 24. Garlic takes about eight months from an autumn clove, and strawberries fruit the summer after planting. The seed packet is the tie-breaker, because a fortnight either way is normal.",
        ),
    ),
    Case(
        "crop-weeks-named",
        "how long would it take a cabbage and carrots",
        6,
        true,
        Some(
            "Carrots are 12 to 16 weeks from seed. The others, from seed: salad leaves and radishes 4 to 6 weeks, beetroot 10 to 12, carrots and runner beans 12 to 16, peas 12 to 14, courgettes 10 to 12, sweetcorn 12 to 14, tomatoes 16 to 20 from sowing (12 to 16 from planting out), cabbages and broccoli 16 to 20, parsnips 22 to 24, and leeks or onions 20 to 24. Garlic takes about eight months from an autumn clove, and strawberries fruit the summer after planting. The seed packet is the tie-breaker, because a fortnight either way is normal.",
        ),
    ),
    Case(
        "crop-weeks-named",
        "how many weeks until my beetroot grows",
        6,
        true,
        Some(
            "Beetroot is 10 to 12 weeks from seed. The others, from seed: salad leaves and radishes 4 to 6 weeks, beetroot 10 to 12, carrots and runner beans 12 to 16, peas 12 to 14, courgettes 10 to 12, sweetcorn 12 to 14, tomatoes 16 to 20 from sowing (12 to 16 from planting out), cabbages and broccoli 16 to 20, parsnips 22 to 24, and leeks or onions 20 to 24. Garlic takes about eight months from an autumn clove, and strawberries fruit the summer after planting. The seed packet is the tie-breaker, because a fortnight either way is normal.",
        ),
    ),
    Case(
        "crop-weeks-named",
        "how long do peas take to grow",
        6,
        true,
        Some(
            "Peas are 12 to 14 weeks from seed. The others, from seed: salad leaves and radishes 4 to 6 weeks, beetroot 10 to 12, carrots and runner beans 12 to 16, peas 12 to 14, courgettes 10 to 12, sweetcorn 12 to 14, tomatoes 16 to 20 from sowing (12 to 16 from planting out), cabbages and broccoli 16 to 20, parsnips 22 to 24, and leeks or onions 20 to 24. Garlic takes about eight months from an autumn clove, and strawberries fruit the summer after planting. The seed packet is the tie-breaker, because a fortnight either way is normal.",
        ),
    ),
    Case(
        "veg-in-shade",
        "what vegetables can i grow in the shade",
        6,
        true,
        Some(
            "For a shady bed, grow the leaf and root crops: salad leaves, spinach, chard, kale, rocket, parsley, mint, rhubarb, beetroot, radish and leeks, and they crop on three to four hours of sun a day, which is what shade gives. Sow them thinner than you would in the open and keep the slugs off, because slow growth and still damp air are what mildew and slugs like best. Anything that has to ripen wants six hours and more, so keep tomatoes, courgettes, peppers, beans, squash and sweetcorn for the sunniest strip you have.",
        ),
    ),
    Case(
        "veg-in-shade",
        "can i grow tomatoes and potatoes in the shade",
        6,
        true,
        Some(
            "Potatoes will crop in half shade, just smaller and later, so they are the one root that earns a place in a bed with morning sun and four hours of it. What they will not do is fill in deep shade, where the tops grow and the tubers stay marble-sized. The crops that do best out of the sun are the leaves: salad leaves, spinach, chard, kale, rocket, parsley, mint, rhubarb, beetroot, radish and leeks.",
        ),
    ),
    Case(
        "veg-in-shade",
        "how much shade for zucchini",
        6,
        true,
        Some(
            "Courgettes want six to eight hours of direct sun a day, and in shade they grow leaves and set nothing - so give them the sunniest strip you have, against a south wall or in a pot you can move. What does crop on three to four hours is the leaf and root list: salad leaves, spinach, chard, kale, rocket, parsley, mint, rhubarb, beetroot, radish and leeks. Sow those thinner than you would in the open, and keep the slugs off, because slow growth and still damp air are what slugs and mildew both like.",
        ),
    ),
    Case(
        "veg-in-shade",
        "can strawberries grow in part shade",
        6,
        true,
        Some(
            "Strawberries want six to eight hours of direct sun a day, and in shade they grow leaves and set nothing - so give them the sunniest strip you have, against a south wall or in a pot you can move. What does crop on three to four hours is the leaf and root list: salad leaves, spinach, chard, kale, rocket, parsley, mint, rhubarb, beetroot, radish and leeks. Sow those thinner than you would in the open, and keep the slugs off, because slow growth and still damp air are what slugs and mildew both like.",
        ),
    ),
    Case(
        "veg-in-shade",
        "what crop grows in my shady bed",
        6,
        true,
        Some(
            "For a shady bed, grow the leaf and root crops: salad leaves, spinach, chard, kale, rocket, parsley, mint, rhubarb, beetroot, radish and leeks, and they crop on three to four hours of sun a day, which is what shade gives. Sow them thinner than you would in the open and keep the slugs off, because slow growth and still damp air are what mildew and slugs like best. Anything that has to ripen wants six hours and more, so keep tomatoes, courgettes, peppers, beans, squash and sweetcorn for the sunniest strip you have.",
        ),
    ),
    Case(
        "veg-in-shade",
        "do beans and peppers mind shade",
        6,
        true,
        Some(
            "Peppers want six to eight hours of direct sun a day, and in shade they grow leaves and set nothing - so give them the sunniest strip you have, against a south wall or in a pot you can move. What does crop on three to four hours is the leaf and root list: salad leaves, spinach, chard, kale, rocket, parsley, mint, rhubarb, beetroot, radish and leeks. Sow those thinner than you would in the open, and keep the slugs off, because slow growth and still damp air are what slugs and mildew both like.",
        ),
    ),
    Case(
        "veg-in-shade",
        "when does a parsnip grow",
        6,
        true,
        Some(
            "For a shady bed, grow the leaf and root crops: salad leaves, spinach, chard, kale, rocket, parsley, mint, rhubarb, beetroot, radish and leeks, and they crop on three to four hours of sun a day, which is what shade gives. Sow them thinner than you would in the open and keep the slugs off, because slow growth and still damp air are what mildew and slugs like best. Anything that has to ripen wants six hours and more, so keep tomatoes, courgettes, peppers, beans, squash and sweetcorn for the sunniest strip you have.",
        ),
    ),
    Case(
        "veg-in-shade",
        "what about peas and tomatoes in the shade",
        6,
        true,
        Some(
            "Tomatoes want six to eight hours of direct sun a day, and in shade they grow leaves and set nothing - so give them the sunniest strip you have, against a south wall or in a pot you can move. What does crop on three to four hours is the leaf and root list: salad leaves, spinach, chard, kale, rocket, parsley, mint, rhubarb, beetroot, radish and leeks. Sow those thinner than you would in the open, and keep the slugs off, because slow growth and still damp air are what slugs and mildew both like.",
        ),
    ),
];
