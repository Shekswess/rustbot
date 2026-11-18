//! Pinned nature replies, exclusions, and local civil timestamps.

pub(super) type Case = (
    &'static str,
    &'static str,
    &'static [(&'static str, &'static str)],
    bool,
    Option<&'static str>,
);
pub(super) const CARDS: &[Case] = &[
    (
        "leaves-autumn",
        "why do the trees lose leaves",
        &[],
        true,
        Some(
            "Shorter days tell trees to stop making chlorophyll, the green stuff, and as it fades the yellow and orange pigments underneath show through; some trees also make red pigments. The tree then seals off each leaf and lets it drop, so it loses less water over winter.",
        ),
    ),
    (
        "leaves-autumn",
        "what turns leaves brown in autumn",
        &[],
        true,
        Some(
            "Shorter days tell trees to stop making chlorophyll, the green stuff, and as it fades the yellow and orange pigments underneath show through; some trees also make red pigments. The tree then seals off each leaf and lets it drop, so it loses less water over winter.",
        ),
    ),
    (
        "leaves-autumn",
        "what turns leaves brown in autumn",
        &[("my pear tree grows quickly", "")],
        true,
        Some(
            "Shorter days tell trees to stop making chlorophyll, the green stuff, and as it fades the yellow and orange pigments underneath show through; some trees also make red pigments. The tree then seals off each leaf and lets it drop, so it loses less water over winter.",
        ),
    ),
    (
        "leaves-autumn",
        "why are the leaves turning",
        &[("i planted tomatoes", "")],
        true,
        None,
    ),
    (
        "leaves-autumn",
        "why are the leaves turning",
        &[
            ("i planted tomatoes", ""),
            ("what about oak trees", ""),
            ("tell me about autumn", ""),
        ],
        true,
        None,
    ),
    (
        "leaves-autumn",
        "why are the leaves turning",
        &[
            ("i planted tomatoes", ""),
            ("what about oak trees", ""),
            ("tell me about autumn", ""),
            ("and oak trees?", ""),
        ],
        true,
        Some(
            "Shorter days tell trees to stop making chlorophyll, the green stuff, and as it fades the yellow and orange pigments underneath show through; some trees also make red pigments. The tree then seals off each leaf and lets it drop, so it loses less water over winter.",
        ),
    ),
    (
        "leaves-autumn",
        "why are the leaves turning",
        &[("My BASIL? very nice.", "")],
        true,
        None,
    ),
    (
        "leaves-autumn",
        "why are the leaves turning",
        &[("i have cuttings", "")],
        true,
        None,
    ),
    (
        "leaves-autumn",
        "why are the leaves turning",
        &[("do trees drop leaves", "my tomatoes are yellow")],
        true,
        Some(
            "Shorter days tell trees to stop making chlorophyll, the green stuff, and as it fades the yellow and orange pigments underneath show through; some trees also make red pigments. The tree then seals off each leaf and lets it drop, so it loses less water over winter.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in january",
        &[],
        true,
        Some(
            "In the UK, about the middle of january gives roughly 8 hours 10 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in february",
        &[],
        true,
        Some(
            "In the UK, about the middle of february gives roughly 9 hours 55 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in march",
        &[],
        true,
        Some(
            "In the UK, about the middle of march gives roughly 11 hours 55 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in april",
        &[],
        true,
        Some(
            "In the UK, about the middle of april gives roughly 14 hours 0 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in may",
        &[],
        true,
        Some(
            "In the UK, about the middle of may gives roughly 15 hours 55 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in june",
        &[],
        true,
        Some(
            "The same tilt the other way: your half leans towards the Sun in summer, so it rises earlier, climbs higher and sets later, and the day is long. In the UK that is about 16 hours 45 minutes of daylight around 21 June, against 7 hours 45 minutes in December.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in july",
        &[],
        true,
        Some(
            "The same tilt the other way: your half leans towards the Sun in summer, so it rises earlier, climbs higher and sets later, and the day is long. In the UK that is about 16 hours 45 minutes of daylight around 21 June, against 7 hours 45 minutes in December.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in august",
        &[],
        true,
        Some(
            "The same tilt the other way: your half leans towards the Sun in summer, so it rises earlier, climbs higher and sets later, and the day is long. In the UK that is about 16 hours 45 minutes of daylight around 21 June, against 7 hours 45 minutes in December.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in september",
        &[],
        true,
        Some(
            "In the UK, about the middle of september gives roughly 12 hours 30 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in october",
        &[],
        true,
        Some(
            "In the UK, about the middle of october gives roughly 10 hours 30 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in november",
        &[],
        true,
        Some(
            "In the UK, about the middle of november gives roughly 8 hours 40 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight in december",
        &[],
        true,
        Some(
            "Daylight hours swing with the Earth's tilt: your half leans away from the Sun in winter, so it rises later, climbs less high and sets earlier, and the day is short. In the UK that is about 7 hours 45 minutes of daylight around 21 December, against 16 hours 45 minutes in June.",
        ),
    ),
    (
        "winter-daylight",
        "how many hours of daylight may we get",
        &[],
        true,
        Some(
            "Daylight hours swing with the Earth's tilt: your half leans away from the Sun in winter, so it rises later, climbs less high and sets earlier, and the day is short. In the UK that is about 7 hours 45 minutes of daylight around 21 December, against 16 hours 45 minutes in June.",
        ),
    ),
    (
        "winter-daylight",
        "when does it get dark in may?",
        &[],
        true,
        Some(
            "Daylight hours swing with the Earth's tilt: your half leans away from the Sun in winter, so it rises later, climbs less high and sets earlier, and the day is short. In the UK that is about 7 hours 45 minutes of daylight around 21 December, against 16 hours 45 minutes in June.",
        ),
    ),
    (
        "winter-daylight",
        "light on the 1st of may",
        &[],
        true,
        Some(
            "In the UK, about the middle of may gives roughly 15 hours 55 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "how long is the day during may",
        &[],
        true,
        Some(
            "In the UK, about the middle of may gives roughly 15 hours 55 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "why are the days longer in summer than winter",
        &[],
        true,
        Some(
            "The same tilt the other way: your half leans towards the Sun in summer, so it rises earlier, climbs higher and sets later, and the day is long. In the UK that is about 16 hours 45 minutes of daylight around 21 June, against 7 hours 45 minutes in December.",
        ),
    ),
    (
        "winter-daylight",
        "why are the days shorter in winter than summer",
        &[],
        true,
        Some(
            "Daylight hours swing with the Earth's tilt: your half leans away from the Sun in winter, so it rises later, climbs less high and sets earlier, and the day is short. In the UK that is about 7 hours 45 minutes of daylight around 21 December, against 16 hours 45 minutes in June.",
        ),
    ),
    (
        "winter-daylight",
        "how dark do the nights get during june in scotland",
        &[],
        true,
        Some(
            "The same tilt the other way: your half leans towards the Sun in summer, so it rises earlier, climbs higher and sets later, and the day is long. In the UK that is about 16 hours 45 minutes of daylight around 21 June, against 7 hours 45 minutes in December. In the far north of Scotland around midsummer the sun only dips just below the horizon, so it never gets properly dark - the simmer dim - and Shetland's longest day is over 18 hours of light.",
        ),
    ),
    (
        "winter-daylight",
        "june solstice in shetland",
        &[],
        true,
        Some(
            "The same tilt the other way: your half leans towards the Sun in summer, so it rises earlier, climbs higher and sets later, and the day is long. In the UK that is about 16 hours 45 minutes of daylight around 21 June, against 7 hours 45 minutes in December. In the far north of Scotland around midsummer the sun only dips just below the horizon, so it never gets properly dark - the simmer dim - and Shetland's longest day is over 18 hours of light.",
        ),
    ),
    (
        "winter-daylight",
        "december solstice in shetland",
        &[],
        true,
        Some(
            "Daylight hours swing with the Earth's tilt: your half leans away from the Sun in winter, so it rises later, climbs less high and sets earlier, and the day is short. In the UK that is about 7 hours 45 minutes of daylight around 21 December, against 16 hours 45 minutes in June.",
        ),
    ),
    (
        "winter-daylight",
        "what is winter daylight",
        &[],
        true,
        Some(
            "Daylight hours swing with the Earth's tilt: your half leans away from the Sun in winter, so it rises later, climbs less high and sets earlier, and the day is short. In the UK that is about 7 hours 45 minutes of daylight around 21 December, against 16 hours 45 minutes in June.",
        ),
    ),
    (
        "winter-daylight",
        "early dark at 4pm in summer",
        &[],
        true,
        Some(
            "Daylight hours swing with the Earth's tilt: your half leans away from the Sun in winter, so it rises later, climbs less high and sets earlier, and the day is short. In the UK that is about 7 hours 45 minutes of daylight around 21 December, against 16 hours 45 minutes in June.",
        ),
    ),
    (
        "winter-daylight",
        "daylight in october or november",
        &[],
        true,
        Some(
            "In the UK, about the middle of october gives roughly 10 hours 30 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "may",
        &[],
        true,
        Some(
            "In the UK, about the middle of may gives roughly 15 hours 55 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "winter-daylight",
        "october and january",
        &[],
        true,
        Some(
            "In the UK, about the middle of january gives roughly 8 hours 10 minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "what if it rains",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "what if it rains all week",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky. If it is set in for the week, pick one proper indoor day rather than a wet walk every day, and keep the evenings for the things that need no weather at all.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "what if the weather is bad",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky. If it is set in for the week, pick one proper indoor day rather than a wet walk every day, and keep the evenings for the things that need no weather at all.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "what if the weather turns",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky. If it is set in for the week, pick one proper indoor day rather than a wet walk every day, and keep the evenings for the things that need no weather at all.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "what if it rains a lot",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky. If it is set in for the week, pick one proper indoor day rather than a wet walk every day, and keep the evenings for the things that need no weather at all.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "if it rains every day what should we do",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky. If it is set in for the week, pick one proper indoor day rather than a wet walk every day, and keep the evenings for the things that need no weather at all.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "what if it pours",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky. If it is set in for the week, pick one proper indoor day rather than a wet walk every day, and keep the evenings for the things that need no weather at all.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "if it rains during the evening",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky.",
        ),
    ),
    (
        "wet-day-on-a-plan",
        "what if the weather gets worse",
        &[],
        true,
        Some(
            "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a aubergine a fruit",
        &[],
        true,
        Some(
            "Botanically an aubergine is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a aubergine a vegetable",
        &[],
        true,
        Some(
            "Botanically an aubergine is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a aubergine a berry",
        &[],
        true,
        Some(
            "Yes - an aubergine is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a avocado a fruit",
        &[],
        true,
        Some(
            "Botanically an avocado is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a avocado a vegetable",
        &[],
        true,
        Some(
            "Botanically an avocado is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a avocado a berry",
        &[],
        true,
        Some(
            "Yes - an avocado is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a banana a fruit",
        &[],
        true,
        Some(
            "Botanically a banana is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a banana a vegetable",
        &[],
        true,
        Some(
            "Botanically a banana is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a banana a berry",
        &[],
        true,
        Some(
            "Yes - a banana is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a blackberry a fruit",
        &[],
        true,
        Some(
            "No - a blackberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a blackberry a vegetable",
        &[],
        true,
        Some(
            "No - a blackberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a blackberry a berry",
        &[],
        true,
        Some(
            "No - a blackberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a blueberry a fruit",
        &[],
        true,
        Some(
            "Yes - a blueberry is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a blueberry a vegetable",
        &[],
        true,
        Some(
            "Yes - a blueberry is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a blueberry a berry",
        &[],
        true,
        Some(
            "Yes - a blueberry is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a broccoli a fruit",
        &[],
        true,
        Some(
            "No - a broccoli is a flower, not a fruit. The fruit is the part that holds the seeds, so a broccoli is a vegetable in the kitchen and a flower in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a broccoli a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen a broccoli is a vegetable, but botanically it is a flower, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a broccoli a berry",
        &[],
        true,
        Some(
            "No - a broccoli is a flower, not a fruit. The fruit is the part that holds the seeds, so a broccoli is a vegetable in the kitchen and a flower in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a carrot a fruit",
        &[],
        true,
        Some(
            "No - a carrot is a root, not a fruit. The fruit is the part that holds the seeds, so a carrot is a vegetable in the kitchen and a root in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a carrot a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen a carrot is a vegetable, but botanically it is a root, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a carrot a berry",
        &[],
        true,
        Some(
            "No - a carrot is a root, not a fruit. The fruit is the part that holds the seeds, so a carrot is a vegetable in the kitchen and a root in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cauliflower a fruit",
        &[],
        true,
        Some(
            "No - a cauliflower is a flower, not a fruit. The fruit is the part that holds the seeds, so a cauliflower is a vegetable in the kitchen and a flower in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cauliflower a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen a cauliflower is a vegetable, but botanically it is a flower, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cauliflower a berry",
        &[],
        true,
        Some(
            "No - a cauliflower is a flower, not a fruit. The fruit is the part that holds the seeds, so a cauliflower is a vegetable in the kitchen and a flower in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a celery a fruit",
        &[],
        true,
        Some(
            "No - a celery is a stem, not a fruit. The fruit is the part that holds the seeds, so a celery is a vegetable in the kitchen and a stem in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a celery a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen a celery is a vegetable, but botanically it is a stem, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a celery a berry",
        &[],
        true,
        Some(
            "No - a celery is a stem, not a fruit. The fruit is the part that holds the seeds, so a celery is a vegetable in the kitchen and a stem in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cherry a fruit",
        &[],
        true,
        Some(
            "Botanically a cherry is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cherry a vegetable",
        &[],
        true,
        Some(
            "Botanically a cherry is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cherry a berry",
        &[],
        true,
        Some(
            "No - a cherry is a drupe: one seed inside a hard stone, not a berry. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a chili a fruit",
        &[],
        true,
        Some(
            "Botanically a chili is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a chili a vegetable",
        &[],
        true,
        Some(
            "Botanically a chili is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a chili a berry",
        &[],
        true,
        Some(
            "Yes - a chili is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a chilli a fruit",
        &[],
        true,
        Some(
            "Botanically a chilli is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a chilli a vegetable",
        &[],
        true,
        Some(
            "Botanically a chilli is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a chilli a berry",
        &[],
        true,
        Some(
            "Yes - a chilli is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a courgette a fruit",
        &[],
        true,
        Some(
            "Botanically a courgette is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a courgette a vegetable",
        &[],
        true,
        Some(
            "Botanically a courgette is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a courgette a berry",
        &[],
        true,
        Some(
            "Yes - a courgette is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cucumber a fruit",
        &[],
        true,
        Some(
            "Botanically a cucumber is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cucumber a vegetable",
        &[],
        true,
        Some(
            "Botanically a cucumber is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a cucumber a berry",
        &[],
        true,
        Some(
            "Yes - a cucumber is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a eggplant a fruit",
        &[],
        true,
        Some(
            "Botanically an eggplant is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a eggplant a vegetable",
        &[],
        true,
        Some(
            "Botanically an eggplant is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a eggplant a berry",
        &[],
        true,
        Some(
            "Yes - an eggplant is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a garlic a fruit",
        &[],
        true,
        Some(
            "No - a garlic is a bulb, not a fruit. The fruit is the part that holds the seeds, so a garlic is a vegetable in the kitchen and a bulb in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a garlic a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen a garlic is a vegetable, but botanically it is a bulb, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a garlic a berry",
        &[],
        true,
        Some(
            "No - a garlic is a bulb, not a fruit. The fruit is the part that holds the seeds, so a garlic is a vegetable in the kitchen and a bulb in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a grape a fruit",
        &[],
        true,
        Some(
            "Botanically a grape is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a grape a vegetable",
        &[],
        true,
        Some(
            "Botanically a grape is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a grape a berry",
        &[],
        true,
        Some(
            "Yes - a grape is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a kiwi a fruit",
        &[],
        true,
        Some(
            "Botanically a kiwi is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a kiwi a vegetable",
        &[],
        true,
        Some(
            "Botanically a kiwi is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a kiwi a berry",
        &[],
        true,
        Some(
            "Yes - a kiwi is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lemon a fruit",
        &[],
        true,
        Some(
            "Botanically a lemon is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lemon a vegetable",
        &[],
        true,
        Some(
            "Botanically a lemon is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lemon a berry",
        &[],
        true,
        Some(
            "Yes - a lemon is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lettuce a fruit",
        &[],
        true,
        Some(
            "No - a lettuce is leaves, not a fruit. The fruit is the part that holds the seeds, so a lettuce is a vegetable in the kitchen and leaves in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lettuce a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen a lettuce is a vegetable, but botanically it is leaves, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lettuce a berry",
        &[],
        true,
        Some(
            "No - a lettuce is leaves, not a fruit. The fruit is the part that holds the seeds, so a lettuce is a vegetable in the kitchen and leaves in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lime a fruit",
        &[],
        true,
        Some(
            "Botanically a lime is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lime a vegetable",
        &[],
        true,
        Some(
            "Botanically a lime is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a lime a berry",
        &[],
        true,
        Some(
            "Yes - a lime is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a onion a fruit",
        &[],
        true,
        Some(
            "No - an onion is a bulb, not a fruit. The fruit is the part that holds the seeds, so an onion is a vegetable in the kitchen and a bulb in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a onion a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen an onion is a vegetable, but botanically it is a bulb, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a onion a berry",
        &[],
        true,
        Some(
            "No - an onion is a bulb, not a fruit. The fruit is the part that holds the seeds, so an onion is a vegetable in the kitchen and a bulb in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a pepper a fruit",
        &[],
        true,
        Some(
            "Botanically a pepper is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a pepper a vegetable",
        &[],
        true,
        Some(
            "Botanically a pepper is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a pepper a berry",
        &[],
        true,
        Some(
            "Yes - a pepper is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a potato a fruit",
        &[],
        true,
        Some(
            "No - a potato is a tuber, not a fruit. The fruit is the part that holds the seeds, so a potato is a vegetable in the kitchen and a tuber in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a potato a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen a potato is a vegetable, but botanically it is a tuber, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a potato a berry",
        &[],
        true,
        Some(
            "No - a potato is a tuber, not a fruit. The fruit is the part that holds the seeds, so a potato is a vegetable in the kitchen and a tuber in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a pumpkin a fruit",
        &[],
        true,
        Some(
            "Botanically a pumpkin is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a pumpkin a vegetable",
        &[],
        true,
        Some(
            "Botanically a pumpkin is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a pumpkin a berry",
        &[],
        true,
        Some(
            "Yes - a pumpkin is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a raspberry a fruit",
        &[],
        true,
        Some(
            "No - a raspberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a raspberry a vegetable",
        &[],
        true,
        Some(
            "No - a raspberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a raspberry a berry",
        &[],
        true,
        Some(
            "No - a raspberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a rhubarb a fruit",
        &[],
        true,
        Some(
            "No - a rhubarb is a stem, not a fruit. The fruit is the part that holds the seeds, so a rhubarb is a vegetable in the kitchen and a stem in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a rhubarb a vegetable",
        &[],
        true,
        Some(
            "Yes - in the kitchen a rhubarb is a vegetable, but botanically it is a stem, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a rhubarb a berry",
        &[],
        true,
        Some(
            "No - a rhubarb is a stem, not a fruit. The fruit is the part that holds the seeds, so a rhubarb is a vegetable in the kitchen and a stem in the ground.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a squash a fruit",
        &[],
        true,
        Some(
            "Botanically a squash is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a squash a vegetable",
        &[],
        true,
        Some(
            "Botanically a squash is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a squash a berry",
        &[],
        true,
        Some(
            "Yes - a squash is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a strawberry a fruit",
        &[],
        true,
        Some(
            "No - a strawberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a strawberry a vegetable",
        &[],
        true,
        Some(
            "No - a strawberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a strawberry a berry",
        &[],
        true,
        Some(
            "No - a strawberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a tomato a fruit",
        &[],
        true,
        Some(
            "Botanically a tomato is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a tomato a vegetable",
        &[],
        true,
        Some(
            "Botanically a tomato is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a tomato a berry",
        &[],
        true,
        Some(
            "Yes - a tomato is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a zucchini a fruit",
        &[],
        true,
        Some(
            "Botanically a zucchini is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a zucchini a vegetable",
        &[],
        true,
        Some(
            "Botanically a zucchini is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a zucchini a berry",
        &[],
        true,
        Some(
            "Yes - a zucchini is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "are strawberries berries",
        &[],
        true,
        Some(
            "No - a strawberry is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "are tomatoes fruits",
        &[],
        true,
        Some(
            "Botanically a tomato is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "are potatoes vegetables",
        &[],
        true,
        Some(
            "Yes - in the kitchen a potato is a vegetable, but botanically it is a tuber, not a fruit: the fruit is the part that holds the seeds.",
        ),
    ),
    (
        "fruit-or-berry",
        "are cherries berries",
        &[],
        true,
        Some(
            "No - a cherry is a drupe: one seed inside a hard stone, not a berry. A banana or a tomato really is a berry, which is where the rule surprises people.",
        ),
    ),
    (
        "fruit-or-berry",
        "are squashes vegetables",
        &[],
        true,
        Some(
            "Botanically a squash is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "are kiwis berries",
        &[],
        true,
        Some(
            "Yes - a kiwi is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "are courgettes or aubergines fruit",
        &[],
        true,
        Some(
            "Botanically an aubergine is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "how do pears grow",
        &[],
        true,
        Some(
            "Botanically a fruit is the part that grows from the flower and holds the seeds, so tomatoes, cucumbers and peppers are fruits. A berry is a fruit from one ovary with seeds in the flesh, which makes bananas berries but strawberries not.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a physalis a fruit",
        &[],
        true,
        Some(
            "Botanically a fruit is the part that grows from the flower and holds the seeds, so tomatoes, cucumbers and peppers are fruits. A berry is a fruit from one ovary with seeds in the flesh, which makes bananas berries but strawberries not.",
        ),
    ),
    (
        "fruit-or-berry",
        "will my tomatoes still fruit this year",
        &[],
        true,
        None,
    ),
    (
        "fruit-or-berry",
        "why wont my peppers fruit",
        &[],
        false,
        Some(
            "Botanically a pepper is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "pepper gives no berries",
        &[],
        false,
        Some(
            "Yes - a pepper is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside.",
        ),
    ),
    (
        "fruit-or-berry",
        "my aubergine flowers fell off",
        &[],
        false,
        Some(
            "Botanically an aubergine is a fruit that grows from the flower and holds its seeds. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one.",
        ),
    ),
    (
        "fruit-or-berry",
        "i have no cucurbit in the garden",
        &[],
        true,
        Some(
            "Botanically a fruit is the part that grows from the flower and holds the seeds, so tomatoes, cucumbers and peppers are fruits. A berry is a fruit from one ovary with seeds in the flesh, which makes bananas berries but strawberries not.",
        ),
    ),
    (
        "fruit-or-berry",
        "is a fruit something to harvest",
        &[],
        true,
        None,
    ),
];
pub(super) const MOON: &[(&str, &str)] = &[
    (
        "2026-11-01T00:00:00",
        "The next full moon is on Tuesday 24 November, in 23 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-02T00:00:00",
        "The next full moon is on Tuesday 24 November, in 22 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-03T00:00:00",
        "The next full moon is on Tuesday 24 November, in 21 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-04T00:00:00",
        "The next full moon is on Tuesday 24 November, in 20 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-05T00:00:00",
        "The next full moon is on Tuesday 24 November, in 19 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-06T00:00:00",
        "The next full moon is on Tuesday 24 November, in 18 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-07T00:00:00",
        "The next full moon is on Tuesday 24 November, in 17 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-08T00:00:00",
        "The next full moon is on Tuesday 24 November, in 16 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-09T00:00:00",
        "The next full moon is on Tuesday 24 November, in 15 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-10T00:00:00",
        "The next full moon is on Tuesday 24 November, in 14 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-11T00:00:00",
        "The next full moon is on Tuesday 24 November, in 13 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-12T00:00:00",
        "The next full moon is on Tuesday 24 November, in 12 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-13T00:00:00",
        "The next full moon is on Tuesday 24 November, in 11 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-14T00:00:00",
        "The next full moon is on Tuesday 24 November, in 10 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-15T00:00:00",
        "The next full moon is on Tuesday 24 November, in 9 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-16T00:00:00",
        "The next full moon is on Tuesday 24 November, in 8 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-17T00:00:00",
        "The next full moon is on Tuesday 24 November, in 7 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-18T00:00:00",
        "The next full moon is on Tuesday 24 November, in 6 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-19T00:00:00",
        "The next full moon is on Tuesday 24 November, in 5 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-20T00:00:00",
        "The next full moon is on Tuesday 24 November, in 4 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-21T00:00:00",
        "The next full moon is on Tuesday 24 November, in 3 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-22T00:00:00",
        "The next full moon is on Tuesday 24 November, in 2 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-23T00:00:00",
        "The next full moon is tomorrow, Tuesday 24 November. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-24T00:00:00",
        "Tonight is a full moon. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-25T00:00:00",
        "The next full moon is on Thursday 24 December, in 29 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-26T00:00:00",
        "The next full moon is on Thursday 24 December, in 28 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-27T00:00:00",
        "The next full moon is on Thursday 24 December, in 27 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-28T00:00:00",
        "The next full moon is on Thursday 24 December, in 26 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-29T00:00:00",
        "The next full moon is on Thursday 24 December, in 25 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-11-30T00:00:00",
        "The next full moon is on Thursday 24 December, in 24 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-01T00:00:00",
        "The next full moon is on Thursday 24 December, in 23 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-02T00:00:00",
        "The next full moon is on Thursday 24 December, in 22 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-03T00:00:00",
        "The next full moon is on Thursday 24 December, in 21 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-04T00:00:00",
        "The next full moon is on Thursday 24 December, in 20 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-05T00:00:00",
        "The next full moon is on Thursday 24 December, in 19 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-06T00:00:00",
        "The next full moon is on Thursday 24 December, in 18 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-07T00:00:00",
        "The next full moon is on Thursday 24 December, in 17 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-08T00:00:00",
        "The next full moon is on Thursday 24 December, in 16 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-09T00:00:00",
        "The next full moon is on Thursday 24 December, in 15 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2026-12-10T00:00:00",
        "The next full moon is on Thursday 24 December, in 14 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "1999-06-15T11:02:00",
        "The next full moon is on Wednesday 28 July, in 43 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2000-01-13T00:00:00",
        "The next full moon is on Sunday 20 February, in 38 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2000-01-21T00:00:00",
        "The next full moon is on Sunday 20 February, in 30 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2000-01-21T23:59:00",
        "The next full moon is on Sunday 20 February, in 30 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2033-05-10T21:36:10",
        "The next full moon is on Saturday 14 May, in 4 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
    (
        "2040-03-12T12:24:00",
        "The next full moon is on Wednesday 28 March, in 16 days. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer.",
    ),
];
