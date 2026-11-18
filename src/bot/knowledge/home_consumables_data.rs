//! Source search terms for household supplies and chemical hazards.

pub(super) const CHEMICAL_CHILD: &[&str] = &[
    "toddler",
    "toddlers",
    "baby",
    "babies",
    "newborn",
    "child",
    "children",
    "kid",
    "kids",
    "grandchild",
    "grandchildren",
];
pub(super) const CHEMICAL_PET: &[&str] = &[
    "cat", "cats", "kitten", "kittens", "dog", "dogs", "puppy", "puppies", "pet", "pets",
    "hamster", "rabbit", "bird", "birds", "parrot", "fish",
];
pub(super) const CHEMICAL_EXPECTING: &[&str] = &["pregnant", "pregnancy", "expecting"];
pub(super) const CHEMICALS: &[(&[&str], &str)] = &[
    (
        &[
            "bait",
            "rat bait",
            "mouse bait",
            "bait box",
            "bait station",
            "rat poison",
            "mouse poison",
            "rodenticide",
        ],
        "Keep rat and mouse bait in its original packaging in a locked place. Use only a bait station specified on the label, keep it out of reach of children and animals, and never leave loose bait where they can reach it.",
    ),
    (
        &["slug pellet", "slug pellets", "slug bait"],
        "Keep slug pellets out of reach of children and pets. Use them only as the label directs, including any instructions for keeping children and pets away from treated areas.",
    ),
    (
        &[
            "ant powder",
            "ant killer",
            "ant bait",
            "insect powder",
            "pesticide",
        ],
        "Follow the insecticide label for where to apply it, required ventilation and when children and pets can return. Store the original package securely.",
    ),
    (
        &["weedkiller", "weed killer"],
        "Follow the weedkiller label for mixing, application and when children and pets can return to the treated area. Keep it in its labelled container.",
    ),
    (
        &[
            "trap",
            "traps",
            "mouse trap",
            "mouse traps",
            "rat trap",
            "rat traps",
        ],
        "Put traps where children and pets cannot reach them. Use an enclosed trap according to the product instructions and check it regularly.",
    ),
    (
        &["flea bomb", "flea bombs", "fogger", "bug bomb"],
        "Follow the fogger label for who and what must leave the area, how long to stay out and when to ventilate. Keep children and pets away until the conditions on the label are met.",
    ),
    (
        &[
            "insect spray",
            "fly spray",
            "ant spray",
            "wasp spray",
            "flea spray",
            "bug spray",
            "mosquito spray",
            "room spray",
            "fly killer",
            "insect killer",
            "aerosol",
            "sprayed",
            "spraying",
            "spray",
            "sprays",
        ],
        "Follow the spray label for ventilation and re-entry. Keep children and pets away while using the product. Ask a vet before using pest-control products around animals if you are unsure.",
    ),
    (
        &[
            "air freshener",
            "air fresheners",
            "diffuser",
            "reed diffuser",
            "plug in air freshener",
            "chemical",
            "chemicals",
        ],
        "Keep diffusers and their refills out of reach of children and pets. Store oils and refills in their original, closed containers.",
    ),
    (
        &["antifreeze"],
        "Keep antifreeze in a closed, labelled container away from children and pets. Keep them away from spills and clean spills as the product label directs.",
    ),
    (
        &[
            "bleach",
            "oven cleaner",
            "drain cleaner",
            "white spirit",
            "paint thinner",
        ],
        "Keep cleaners in their labelled containers, closed and out of reach of children and pets. Follow the label for gloves and ventilation. Never mix cleaning products.",
    ),
];
pub(super) const CHEMICAL_FALLBACK: &str = "Keep cleaners in their labelled containers, closed and out of reach of children and pets. Follow the label for gloves and ventilation. Never mix cleaning products.";
pub(super) const SUPPLIES: &[(&[&str], &str)] = &[
    (
        &[
            "washing powder",
            "soap powder",
            "laundry powder",
            "laundry detergent",
            "laundry liquid",
            "washing liquid",
            "washing capsules",
            "laundry capsules",
            "washing detergent",
        ],
        "Use laundry detergent made for your machine and follow its dosing instructions. If you have run out, wait to run the load until you have the right detergent.",
    ),
    (
        &[
            "kitchen roll",
            "kitchen towel",
            "kitchen towels",
            "paper towel",
            "paper towels",
            "kitchen paper",
        ],
        "A tea towel and a pile of old cloths do the same job, and newspaper is better than either for glass and windows. Buy a four-pack and keep one roll back; a roll lasts about a week in a busy kitchen, and the blue shop towels are the ones worth the extra for grease.",
    ),
    (
        &["tin foil", "foil", "aluminium foil", "silver foil"],
        "Baking paper on a tray, and a lid or an upturned plate over a dish, cover most of what foil does; for the grill, oil the rack and slide a tray underneath. A 30cm roll lasts a couple of months, and a wide catering roll is cheaper by the metre if you use it for roasting.",
    ),
    (
        &[
            "cling film",
            "clingfilm",
            "plastic wrap",
            "saran wrap",
            "shrink wrap",
        ],
        "A clean plate or lid over a bowl covers most things in the fridge. Use a food-safe freezer bag or a freezer-safe container in the freezer. Reusable food wraps are another option if suitable for the food.",
    ),
    (
        &[
            "dishwasher tablets",
            "dishwasher tablet",
            "dishwasher powder",
            "dishwasher salt",
            "rinse aid",
        ],
        "Use only dishwasher detergent in the machine. Check the manual for the right salt and rinse aid for your model and water hardness. If you have no dishwasher detergent, wash by hand with washing-up liquid.",
    ),
    (
        &[
            "toilet roll",
            "toilet rolls",
            "loo roll",
            "toilet paper",
            "bog roll",
        ],
        "Use tissues or kitchen roll temporarily and put them in the bin after use. Flush only pee, poo and toilet paper.",
    ),
    (
        &[
            "bin bags",
            "bin bag",
            "bin liners",
            "bin liner",
            "rubbish bags",
            "black bags",
        ],
        "A carrier bag, or the bag out of a cereal box, lines a kitchen bin for a day: double it if the rubbish is heavy and keep food waste in its own bag so it cannot split. A roll of 20 lasts about two months, and the next size up holds a week of kitchen rubbish.",
    ),
    (
        &[
            "washing up liquid",
            "washing-up liquid",
            "dish soap",
            "dishwashing liquid",
            "fairy liquid",
        ],
        "Use a detergent intended for washing dishes. If you have run out, scrape the dishes and wash them when you have washing-up liquid. Dishwasher tablets are for the machine.",
    ),
    (
        &[
            "bleach",
            "toilet cleaner",
            "loo cleaner",
            "disinfectant",
            "harpic",
            "domestos",
        ],
        "Use a product suitable for the job and follow the label. Washing-up liquid and water can clean many surfaces. If you need to disinfect, use an appropriate disinfectant as directed. Never mix bleach with vinegar, limescale remover or other cleaners.",
    ),
    (
        &["shampoo", "shampoos", "conditioner"],
        "Rinse your hair with water if you need to wash it before getting more shampoo. When you restock, choose a shampoo your scalp tolerates.",
    ),
    (
        &["toothpaste", "tooth paste"],
        "Keep brushing while you get more toothpaste. Resume brushing twice a day with fluoride toothpaste as soon as you have it.",
    ),
    (
        &["soap", "hand soap", "bar soap", "handwash", "hand wash"],
        "Use bar or liquid hand soap and water. If you have run out of hand soap, body wash can work until you get more.",
    ),
];
