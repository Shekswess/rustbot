//! Direct pinned source outputs for counts and ordered historical discoveries.

pub(super) const CASES: &[(&str, &str, Option<&str>)] = &[
    (
        "steps-in-a-distance",
        "how many steps in a mile",
        Some(
            "About 2,000 steps to a mile, so 10,000 steps is roughly five miles, a 5k is about 6,500 steps and a marathon is around 55,000. Stride length moves that by around a tenth either way - a tall walker covers a mile in fewer - so use it for a pedometer target rather than a measurement.",
        ),
    ),
    (
        "steps-in-a-distance",
        "how many steps in one kilometre",
        Some(
            "About 2,000 steps to a mile, so 10,000 steps is roughly five miles, a 5k is about 6,500 steps and a marathon is around 55,000. Stride length moves that by around a tenth either way - a tall walker covers a mile in fewer - so use it for a pedometer target rather than a measurement.",
        ),
    ),
    (
        "steps-in-a-distance",
        "walked 12000 steps today",
        Some(
            "That's about 6 miles - 12,000 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "12,300 steps today",
        Some(
            "That's about 6.2 miles - 12,300 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "10,700 steps so far",
        Some(
            "That's about 5.3 miles - 10,700 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "999 steps today",
        Some(
            "About 2,000 steps to a mile, so 10,000 steps is roughly five miles, a 5k is about 6,500 steps and a marathon is around 55,000. Stride length moves that by around a tenth either way - a tall walker covers a mile in fewer - so use it for a pedometer target rather than a measurement.",
        ),
    ),
    (
        "steps-in-a-distance",
        "1,000 steps today",
        Some(
            "That's about 0.5 miles - 1,000 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "100,000 steps yesterday",
        Some(
            "About 2,000 steps to a mile, so 10,000 steps is roughly five miles, a 5k is about 6,500 steps and a marathon is around 55,000. Stride length moves that by around a tenth either way - a tall walker covers a mile in fewer - so use it for a pedometer target rather than a measurement.",
        ),
    ),
    (
        "steps-in-a-distance",
        "100000 steps today",
        Some(
            "About 2,000 steps to a mile, so 10,000 steps is roughly five miles, a 5k is about 6,500 steps and a marathon is around 55,000. Stride length moves that by around a tenth either way - a tall walker covers a mile in fewer - so use it for a pedometer target rather than a measurement.",
        ),
    ),
    (
        "steps-in-a-distance",
        "101,000 steps today",
        Some(
            "About 2,000 steps to a mile, so 10,000 steps is roughly five miles, a 5k is about 6,500 steps and a marathon is around 55,000. Stride length moves that by around a tenth either way - a tall walker covers a mile in fewer - so use it for a pedometer target rather than a measurement.",
        ),
    ),
    (
        "steps-in-a-distance",
        "1,099 steps this morning",
        Some(
            "That's about 0.5 miles - 1,099 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "2,100 steps just now",
        Some(
            "That's about 1.1 miles - 2,100 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "2,300 steps before work",
        Some(
            "That's about 1.1 miles - 2,300 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "10,900 steps already",
        Some(
            "That's about 5.5 miles - 10,900 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "how far is 10000 steps",
        Some(
            "About 2,000 steps to a mile, so 10,000 steps is roughly five miles, a 5k is about 6,500 steps and a marathon is around 55,000. Stride length moves that by around a tenth either way - a tall walker covers a mile in fewer - so use it for a pedometer target rather than a measurement.",
        ),
    ),
    (
        "steps-in-a-distance",
        "walked 10,000 steps then 20,000 steps today",
        Some(
            "That's about 5 miles - 10,000 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "walked ١٢٠٠٠ steps today",
        Some(
            "That's about 6 miles - 12,000 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-in-a-distance",
        "１２３００ steps today",
        Some(
            "That's about 6.2 miles - 12,300 steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk.",
        ),
    ),
    (
        "steps-up-a-building",
        "how many steps in the leaning tower of pisa",
        Some(
            "The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. St Paul's dome is 528 steps. A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "how many stairs in empire state building",
        Some(
            "The Empire State Building is 1,860 steps to the 102nd-floor observatory. The Leaning Tower of Pisa is 296 steps to the top. St Paul's dome is 528 steps. A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "steps in st paul's cathedral",
        Some(
            "St Paul's dome is 528 steps. The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "stairs up to a studio flat",
        Some(
            "A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. St Paul's dome is 528 steps. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "what is the number of steps in a flight of stairs",
        Some(
            "A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. St Paul's dome is 528 steps. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "steps in one storey",
        Some(
            "A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. St Paul's dome is 528 steps. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "how many risers",
        Some(
            "A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. St Paul's dome is 528 steps. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "steps in the palace",
        Some(
            "The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. St Paul's dome is 528 steps. A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "dome with 4 floors",
        Some(
            "St Paul's dome is 528 steps. The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-up-a-building",
        "empire leaning tower",
        Some(
            "The Leaning Tower of Pisa is 296 steps to the top. The Empire State Building is 1,860 steps to the 102nd-floor observatory. St Paul's dome is 528 steps. A single flight between house floors is 12 to 13 steps, and a whole staircase up to an upstairs flat is usually 14 to 16. A cathedral or a stadium prints its own count at the entrance.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps in an hour",
        Some(
            "About 6,000 to 8,000 steps in an hour of walking: an ordinary pace is about 5km/h, which is 1,250 steps a kilometre, so an hour gets you five of them. A brisk walk nearer 6.5km/h is the top of that range, and a slow wander round shops is well under it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps in an hour of fast walking",
        Some(
            "About 8,000 steps in an hour of brisk walking - 6.5km/h or so, which is 1,250 steps a kilometre over six and a half of them. An ordinary-paced hour is about 6,000, and a slow wander round the shops is well under that.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps in an hour of hilly walking",
        Some(
            "About 8,000 steps in an hour of brisk walking - 6.5km/h or so, which is 1,250 steps a kilometre over six and a half of them. An ordinary-paced hour is about 6,000, and a slow wander round the shops is well under that.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking half an hour",
        Some(
            "About 3,000 steps in half an hour of walking at an ordinary pace, or 4,000 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 90 minutes",
        Some(
            "About 9,000 steps in 90 minutes of walking at an ordinary pace, or 11,000 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 2 hours",
        Some(
            "About 12,000 steps in 2 hours of walking at an ordinary pace, or 15,000 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 1.5 hours",
        Some(
            "About 9,000 steps in 90 minutes of walking at an ordinary pace, or 11,000 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 0.01 hours",
        Some(
            "About 62 steps in 1 minutes of walking at an ordinary pace, or 75 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 0 hours",
        Some(
            "About 6,000 to 8,000 steps in an hour of walking: an ordinary pace is about 5km/h, which is 1,250 steps a kilometre, so an hour gets you five of them. A brisk walk nearer 6.5km/h is the top of that range, and a slow wander round shops is well under it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 60min",
        Some(
            "About 6,000 steps in an hour of walking at an ordinary pace, or 8,000 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 20 mins",
        Some(
            "About 2,000 steps in 20 minutes of walking at an ordinary pace, or 2,500 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 1 minute",
        Some(
            "About 104 steps in 1 minutes of walking at an ordinary pace, or 125 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 30 minutes",
        Some(
            "About 3,000 steps in half an hour of walking at an ordinary pace, or 4,000 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking 3hrs",
        Some(
            "About 19,000 steps in 3 hours of walking at an ordinary pace, or 22,000 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking ٠٥ minutes",
        Some(
            "About 500 steps in 5 minutes of walking at an ordinary pace, or 600 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-in-a-time",
        "how many steps walking １２ minutes",
        Some(
            "About 1,250 steps in 12 minutes of walking at an ordinary pace, or 1,500 if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it.",
        ),
    ),
    (
        "steps-calories",
        "how many calories does 10000 steps burn",
        Some(
            "About 400 to 500 calories in 10,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories for 10,000 steps",
        Some(
            "About 400 to 500 calories in 10,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories for 100,000 steps",
        Some(
            "About 400 to 500 calories in 10,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories for 123000 steps",
        Some(
            "About 400 to 500 calories in 10,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories for 1,234 steps",
        Some(
            "About 50 to 60 calories in 1,250 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories in 2.5 miles",
        Some(
            "About 200 to 250 calories in 5,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories in 2.5 km",
        Some(
            "About 120 to 160 calories in 3,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories in 1 km and 2 miles",
        Some(
            "About 160 to 200 calories in 4,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories in a mile and 2 miles",
        Some(
            "About 80 to 100 calories in 2,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories over 4 kilometers",
        Some(
            "About 200 to 250 calories in 5,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories in a kilometre",
        Some(
            "About 50 to 60 calories in 1,250 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "how many calories walking an hour",
        Some(
            "About 250 to 310 calories in 6,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories burned in a single hour",
        Some(
            "About 250 to 310 calories in 6,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories burned in one hour",
        Some(
            "About 250 to 310 calories in 6,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories burned in half an hour",
        Some(
            "About 250 to 310 calories in 6,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories burned in 2.5 hours",
        Some(
            "About 620 to 780 calories in 16,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories in 35 minutes",
        Some(
            "About 150 to 180 calories in 3,500 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories in 0 minutes",
        Some(
            "About 400 to 500 calories in 10,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "how many calories in zero miles",
        Some(
            "About 80 to 100 calories in 2,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories walking a long time",
        Some(
            "About 400 to 500 calories in 10,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "steps-calories",
        "calories for ١٢٠٠٠ steps",
        Some(
            "About 480 to 600 calories in 12,000 steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories.",
        ),
    ),
    (
        "who-discovered",
        "who discovered penicillin",
        Some(
            "Alexander Fleming found penicillin in 1928, when a mould that had drifted onto a petri dish killed the bacteria round it.",
        ),
    ),
    (
        "who-discovered",
        "who discovered gravity",
        Some(
            "Isaac Newton worked out the law of gravity in 1687, and the same rule still covers a falling apple and an orbiting moon.",
        ),
    ),
    (
        "who-discovered",
        "who discovered evolution",
        Some(
            "Charles Darwin set out evolution by natural selection in 1859, in On the Origin of Species, with Alfred Russel Wallace reaching the same idea at the same time.",
        ),
    ),
    (
        "who-discovered",
        "who discovered natural selection",
        Some("Charles Darwin set out natural selection in 1859, in On the Origin of Species."),
    ),
    (
        "who-discovered",
        "who discovered x-rays",
        Some(
            "Wilhelm Rontgen discovered X-rays in 1895, and won the first Nobel Prize in Physics for them.",
        ),
    ),
    (
        "who-discovered",
        "who discovered x rays",
        Some(
            "Wilhelm Rontgen discovered X-rays in 1895, and won the first Nobel Prize in Physics for them.",
        ),
    ),
    (
        "who-discovered",
        "who discovered radioactivity",
        Some(
            "Henri Becquerel found radioactivity in 1896; Marie and Pierre Curie then did the chemistry that explained it.",
        ),
    ),
    (
        "who-discovered",
        "who discovered dna",
        Some(
            "The structure of DNA was worked out by James Watson and Francis Crick in 1953, using Rosalind Franklin's X-ray photographs.",
        ),
    ),
    (
        "who-discovered",
        "who discovered the electron",
        Some("J. J. Thomson discovered the electron in 1897."),
    ),
    (
        "who-discovered",
        "who discovered oxygen",
        Some(
            "Joseph Priestley isolated oxygen in 1774, and Carl Wilhelm Scheele had found it two years earlier without publishing first.",
        ),
    ),
    (
        "who-discovered",
        "who discovered the vaccine",
        Some("Edward Jenner made the first vaccine in 1796, using cowpox against smallpox."),
    ),
    (
        "who-discovered",
        "who discovered vaccines",
        Some("Edward Jenner made the first vaccine in 1796, using cowpox against smallpox."),
    ),
    (
        "who-discovered",
        "who discovered oxygen and gravity",
        Some(
            "Isaac Newton worked out the law of gravity in 1687, and the same rule still covers a falling apple and an orbiting moon.",
        ),
    ),
    (
        "who-discovered",
        "who discovered gravity and oxygen",
        Some(
            "Isaac Newton worked out the law of gravity in 1687, and the same rule still covers a falling apple and an orbiting moon.",
        ),
    ),
    (
        "who-discovered",
        "who discovered dna and natural selection",
        Some("Charles Darwin set out natural selection in 1859, in On the Origin of Species."),
    ),
    (
        "who-discovered",
        "who discovered earth's magnetic poles",
        None,
    ),
    (
        "who-discovered",
        "who discovered gravity waves",
        Some(
            "Isaac Newton worked out the law of gravity in 1687, and the same rule still covers a falling apple and an orbiting moon.",
        ),
    ),
    ("who-discovered", "who discovered napoleon", None),
    (
        "who-discovered",
        "who discovered how to make penicillins",
        None,
    ),
    ("who-discovered", "who discovered penicillin's power", None),
];
