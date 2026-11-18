//! Direct crop and pest replies and wood-splitting guards from pinned PyBot.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub bool,
    pub Option<&'static str>,
);

pub(super) const CASES: &[Case] = &[
    Case(
        "root-split",
        "why are my carrots forked",
        &[],
        true,
        Some(
            "A carrot forks when the taproot meets a stone, a hard pan or fresh manure, or when the seedlings were left crowded: the root splits in two to get round it. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "my beetroot has split",
        &[],
        true,
        Some(
            "Beetroot splits when it takes a drink after a dry spell: the root swells faster than the skin keeps up, and too much feed does the same. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "my turnips have run to seed",
        &[],
        true,
        Some(
            "A turnip runs to seed when a hot dry spell pushes it up in its first year, and once the stem goes up the root stops swelling and turns woody in the ground. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "my peas are woody",
        &[],
        true,
        Some(
            "A root crop goes woody when it is left in the ground too long, and in hot dry soil it happens sooner: the root keeps growing and the middle turns hard and stringy. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "my celeriac split badly",
        &[],
        true,
        Some(
            "Celeriac splits when it takes a drink after a dry spell: the root swells faster than the skin keeps up, and too much feed does the same. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "my swede has got tough and pithy",
        &[],
        true,
        Some(
            "A swede goes woody when it is left in the ground too long, and in hot dry soil it happens sooner: the root keeps growing and the middle turns hard and stringy. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "why is a radish woody",
        &[],
        true,
        Some(
            "A radish goes woody when it is left in the ground too long, and in hot dry soil it happens sooner: the root keeps growing and the middle turns hard and stringy. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "what do i do with a root going to seed",
        &[],
        true,
        Some(
            "A root crop runs to seed when a hot dry spell pushes it up in its first year, and once the stem goes up the root stops swelling and turns woody in the ground. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "why did my parsnips fork and bolt",
        &[],
        true,
        Some(
            "A parsnip runs to seed when a hot dry spell pushes it up in its first year, and once the stem goes up the root stops swelling and turns woody in the ground. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "why did my carrots split up",
        &[],
        true,
        Some(
            "A carrot splits when it takes a drink after a dry spell: the root swells faster than the skin keeps up, and too much feed does the same. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "why are my radishes so hard",
        &[],
        true,
        Some(
            "A radish splits when it takes a drink after a dry spell: the root swells faster than the skin keeps up, and too much feed does the same. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "root-split",
        "why did the beetroot go pithy",
        &[],
        true,
        Some(
            "Beetroot goes woody when it is left in the ground too long, and in hot dry soil it happens sooner: the root keeps growing and the middle turns hard and stringy. Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first.",
        ),
    ),
    Case(
        "plant-pests",
        "how do i get rid of vine weevils",
        &[],
        true,
        Some(
            "Vine weevil is a root grub with a beetle above ground, and the beetles notch the leaf edges at night. The grubs are what kills the plant: tip a pot out and look for fat white C-shaped ones in the compost, then bin that compost and repot in fresh. A nematode watered on in late summer or early autumn is the cleanest control, and going out with a torch after dark deals with the beetles.",
        ),
    ),
    Case(
        "plant-pests",
        "there are whitefly on my plants",
        &[],
        true,
        Some(
            "Whitefly sits under the leaves and flies up in a cloud when you touch the plant. Yellow sticky traps catch the adults, and a spray of soapy water - a teaspoon of washing-up liquid in a litre - on the undersides of the leaves kills the young ones; repeat every few days, because it does not touch the eggs. Badly hit leaves come off. Check new plants before they go in the greenhouse, which is where whitefly gets in.",
        ),
    ),
    Case(
        "plant-pests",
        "aphids are eating my runner beans",
        &[],
        true,
        Some(
            "Greenfly and blackfly are both aphids and get the same treatment: squash them by hand or spray the undersides of the leaves and the soft growing tips with soapy water - a teaspoon of washing-up liquid in a litre. Broad beans are the usual blackfly crop, and the tips are the first place to look: pinch the top few inches out once the flowers set. Ladybirds and their larvae will clear a lot for you, so leave a marigold or a poached egg plant nearby.",
        ),
    ),
    Case(
        "plant-pests",
        "cabbage white caterpillars",
        &[],
        true,
        Some(
            "Caterpillars are easiest picked off by hand in the evening, when they come out to feed, and dropped in a bucket of water - check the undersides of the leaves for the yellow eggs and squash those too. A net over the crop from May to September is the real fix, because the cabbage white butterfly cannot get in to lay: keep it clear of the leaves and peg the edges down. Inspect every few days, since a small caterpillar becomes a big one fast.",
        ),
    ),
    Case(
        "plant-pests",
        "which cabbage worms get under the net",
        &[],
        true,
        Some(
            "Caterpillars are easiest picked off by hand in the evening, when they come out to feed, and dropped in a bucket of water - check the undersides of the leaves for the yellow eggs and squash those too. A net over the crop from May to September is the real fix, because the cabbage white butterfly cannot get in to lay: keep it clear of the leaves and peg the edges down. Inspect every few days, since a small caterpillar becomes a big one fast.",
        ),
    ),
    Case(
        "plant-pests",
        "my plant has sticky leaves from scale insects",
        &[],
        true,
        Some(
            "Scale insects sit on stems and the undersides of leaves as little brown or white shells you can flick off with a fingernail, and they leave sticky honeydew that turns the leaves black. Wipe them off with a cotton bud dipped in rubbing alcohol, or spray the plant with soapy water - a teaspoon of washing-up liquid in a litre - a few times over a month, because the shells protect the ones underneath. Check the plant that arrived last, which is usually where they came in.",
        ),
    ),
    Case(
        "plant-pests",
        "can i stop mealybugs spreading",
        &[],
        true,
        Some(
            "Mealybug is the white cotton-wool fluff in the leaf joints and along the stems, and it hides in the same places as scale. Paint it off with a cotton bud dipped in rubbing alcohol, or spray the plant with soapy water - a teaspoon of washing-up liquid in a litre - every week for a month, and take off the worst leaves. It thrives in warm, still air, so give the plant more room and check the plants either side of it.",
        ),
    ),
    Case(
        "plant-pests",
        "i saw white fluff on my plant",
        &[],
        true,
        Some(
            "Mealybug is the white cotton-wool fluff in the leaf joints and along the stems, and it hides in the same places as scale. Paint it off with a cotton bud dipped in rubbing alcohol, or spray the plant with soapy water - a teaspoon of washing-up liquid in a litre - every week for a month, and take off the worst leaves. It thrives in warm, still air, so give the plant more room and check the plants either side of it.",
        ),
    ),
    Case(
        "plant-pests",
        "aphids and caterpillars on my kale",
        &[],
        true,
        Some(
            "Caterpillars are easiest picked off by hand in the evening, when they come out to feed, and dropped in a bucket of water - check the undersides of the leaves for the yellow eggs and squash those too. A net over the crop from May to September is the real fix, because the cabbage white butterfly cannot get in to lay: keep it clear of the leaves and peg the edges down. Inspect every few days, since a small caterpillar becomes a big one fast.",
        ),
    ),
    Case(
        "plant-pests",
        "what are the bugs on my herbs",
        &[],
        true,
        None,
    ),
    Case("plant-pests", "how do i look after it", &[], true, None),
    Case("split-a-log", "how do i split a log", &[], true, None),
    Case(
        "split-a-log",
        "how do i split a wooden round",
        &[],
        true,
        None,
    ),
    Case(
        "split-a-log",
        "how to split logs for the wood burner",
        &[],
        true,
        None,
    ),
    Case(
        "split-a-log",
        "how can i separate the hob ring",
        &[],
        false,
        None,
    ),
    Case("split-a-log", "how to chop rounds", &[], true, None),
    Case(
        "split-a-log",
        "should i split firewood with a wedge",
        &[],
        true,
        None,
    ),
    Case(
        "split-a-log",
        "how do i clean a ring on the hob",
        &[],
        false,
        None,
    ),
    Case(
        "split-a-log",
        "where does the gold ring go",
        &[],
        false,
        None,
    ),
    Case("split-a-log", "split the timbers", &[], false, None),
    Case(
        "split-a-log",
        "is it safe to split the rounds",
        &[],
        true,
        None,
    ),
    Case(
        "split-a-log",
        "a counter ring fits this worktop",
        &[],
        false,
        None,
    ),
    Case(
        "split-a-log",
        "what are the circles on a tree trunk",
        &[],
        true,
        None,
    ),
    Case("split-a-log", "it is a wedding ring", &[], false, None),
    Case(
        "split-a-log",
        "rounds or circles on the chopping block",
        &[],
        true,
        None,
    ),
    Case(
        "plant-pests",
        "will it kill them",
        &["my cabbage has caterpillars"],
        true,
        Some(
            "Caterpillars are easiest picked off by hand in the evening, when they come out to feed, and dropped in a bucket of water - check the undersides of the leaves for the yellow eggs and squash those too. A net over the crop from May to September is the real fix, because the cabbage white butterfly cannot get in to lay: keep it clear of the leaves and peg the edges down. Inspect every few days, since a small caterpillar becomes a big one fast.",
        ),
    ),
    Case(
        "plant-pests",
        "will it kill them",
        &["my plant has aphids", "i see whitefly now"],
        true,
        Some(
            "Whitefly sits under the leaves and flies up in a cloud when you touch the plant. Yellow sticky traps catch the adults, and a spray of soapy water - a teaspoon of washing-up liquid in a litre - on the undersides of the leaves kills the young ones; repeat every few days, because it does not touch the eggs. Badly hit leaves come off. Check new plants before they go in the greenhouse, which is where whitefly gets in.",
        ),
    ),
    Case(
        "plant-pests",
        "will it kill them",
        &[
            "there are weevils on the vine",
            "good morning",
            "can you help",
            "what next",
        ],
        true,
        None,
    ),
    Case(
        "plant-pests",
        "will it kill them",
        &["there are mealybugs on it", "okay", "nice"],
        true,
        Some(
            "Mealybug is the white cotton-wool fluff in the leaf joints and along the stems, and it hides in the same places as scale. Paint it off with a cotton bud dipped in rubbing alcohol, or spray the plant with soapy water - a teaspoon of washing-up liquid in a litre - every week for a month, and take off the worst leaves. It thrives in warm, still air, so give the plant more room and check the plants either side of it.",
        ),
    ),
    Case(
        "plant-pests",
        "will it kill them",
        &[
            "there are whitefly on it",
            "i see no clue",
            "there is a notch now",
        ],
        true,
        Some(
            "Vine weevil is a root grub with a beetle above ground, and the beetles notch the leaf edges at night. The grubs are what kills the plant: tip a pot out and look for fat white C-shaped ones in the compost, then bin that compost and repot in fresh. A nematode watered on in late summer or early autumn is the cleanest control, and going out with a torch after dark deals with the beetles.",
        ),
    ),
    Case(
        "plant-pests",
        "what about the aphids",
        &["there are whitefly on the plant"],
        true,
        Some(
            "Greenfly and blackfly are both aphids and get the same treatment: squash them by hand or spray the undersides of the leaves and the soft growing tips with soapy water - a teaspoon of washing-up liquid in a litre. Broad beans are the usual blackfly crop, and the tips are the first place to look: pinch the top few inches out once the flowers set. Ladybirds and their larvae will clear a lot for you, so leave a marigold or a poached egg plant nearby.",
        ),
    ),
];
