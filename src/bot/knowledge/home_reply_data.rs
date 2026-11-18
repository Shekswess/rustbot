//! Authored household replies and response variants from the pinned PyBot source.

pub(super) const WASHER_DRAWER: &str = "The drawer is the front-left panel: pull it out and you have three compartments. The main wash (usually marked II) takes the powder, the liquid or the detergent. The small one marked I is the pre-wash and only matters if you choose that programme. The one with the flower symbol is the softener, filled to the MAX line and never over it. A tablet or a pod goes straight into the empty drum instead, before the washing, never in the drawer. If the machine has no drawer, the detergent goes in the little box inside the door.";
pub(super) const DISHWASHER_COMPARTMENTS: &str = "Dishwasher salt goes in the screw-cap well in the floor of the drum, under the bottom basket: fill it to the top using a funnel, because the water softener uses it up over a few weeks and the light comes on when it is low. Rinse aid goes in the smaller compartment in the door, the one with the dial - set that to 4 or 5. The tablet or powder goes in the dispenser behind the door flap, never in the cutlery basket.";
pub(super) const PLUG_ANSWER: &str = "A drill bit the same diameter as the plug: 5mm for a yellow plug, 6mm for a red one, 7mm for a brown and 10mm for a blue. Drill a few millimetres deeper than the plug is long, push it in flush with the wall - a tap with the screwdriver handle finishes it - and take the screw size from the packet, which is the one number it always prints. In plasterboard with nothing behind it no plug holds: find a stud, or use a plasterboard fixing rated well above what you are hanging.";
pub(super) const SHELF_ANSWER: &str = "For a shelf into studs or masonry, 4 x 40mm or 5 x 50mm screws are plenty, and the brackets matter more than the screws do: fix each bracket to a stud where you can, keep the shelf within the brackets' span, and put the heavy things over the brackets rather than at the ends. Into plasterboard with nothing behind it, no screw holds a loaded shelf - use plasterboard fixings rated well above the weight, or a batten screwed into the studs.";
pub(super) const NUT_ANSWER: &str = "A nut and a bolt match on the thread, and the number in the name is the thread's diameter in millimetres: an M8 nut goes on an M8 bolt, which takes a 13mm spanner, and M6 takes 10mm, M10 takes 17mm and M12 takes 19mm. Take the bolt with you if you only need the nut, and a washer or a nyloc nut is what stops the joint shaking loose.";
pub(super) const THE_TAP_WASHER: &str = "Good - then you have what you need. Turn the water off at the isolating valve under the sink or the small screw on the pipe, take the handle off and unscrew the head, then pick the old washer out with a small screwdriver. Match it at a hardware shop: half an inch fits most taps, three quarters the older big bath taps. A smear of silicone grease on the new one and it goes back the same way, and the spanner is for the big nut under the handle.";
pub(super) const WASHER_TWO_THINGS: &str = "Two things in a house are called a washer. The ring inside a tap costs pennies: turn the water off at the isolating valve, take the tap apart and take the old ring to a hardware shop to match - half an inch fits most taps, three quarters the older big bath taps. If it is the machine, the fix depends on what it is doing: water on the floor is usually the door seal or a hose, clothes that come out wet is the spin, and no heat is the element. All three are cheap parts, so a machine with a sound drum is worth repairing rather than replacing.";
pub(super) const TAP_TALK: &str =
    r#"(?i)\b(?:tap|taps|faucet|dripping|drip|drips|spout|cistern)\b"#;
pub(super) const UPSTAIRS_ITEMS: &[(&[&str], &str)] = &[
    (
        &["mattress"],
        "A mattress never folds down the middle, so it goes on its edge: bend it more than about thirty degrees and the springs and the foam take a set and it never lies flat again",
    ),
    (
        &["sofa", "couch", "settee", "armchair", "suite"],
        "The legs come off, the cushions come off, and a two-piece sofa splits at the back, which is usually enough on its own",
    ),
    (
        &["table", "desk"],
        "The legs come off and the top goes up on its edge, which turns a wide thing into a tall one",
    ),
    (
        &["bed frame", "bedframe", "divan", "bed"],
        "A bed frame comes apart into sides and ends, and a divan splits into two halves",
    ),
    (
        &[
            "wardrobe",
            "chest of drawers",
            "sideboard",
            "bookcase",
            "bookshelf",
            "shelving",
            "dresser",
            "cabinet",
            "furniture",
        ],
        "Doors and drawers come out and the back panel comes off, because the carcass is a good deal narrower than anything with the doors on",
    ),
];
pub(super) const WASHER_SOUND_PATTERNS: &[(&str, &str)] = &[
    (
        r#"\b(?:grind|scrap|roar|rumbl)"#,
        "Grinding or scraping means stop using it and find the metal: a bra wire, a coin or a key gets past the drum and grinds against the outer tub. Pull the plug out, turn the drum by hand and listen, feel round the inside through the door, and empty the filter and the coin trap behind the kick plate. If it grinds only on the spin, with the drum turning freely by hand, that is the drum bearing rather than something loose, and on a machine past six or seven years it is usually the end of the machine rather than a repair.",
    ),
    (
        r#"\b(?:squeak|squeal|screech|shriek|whine|whistl)"#,
        "A squeal or a screech is nearly always the drive belt, which is a cheap part and the usual cause: it slips on the spin, loudest with a heavy load. Stop using it if there is a burning smell with the noise, because that is the belt rubbing rather than slipping.",
    ),
    (
        r#"\b(?:rattl|clunk)"#,
        "A rattle is something loose in the drum rather than a fault in the motor: empty out the pockets that went through the wash, feel round the inside of the drum for a bra wire or a coin, and check the filter and the coin trap behind the kick plate.",
    ),
];
pub(super) const CONSUMABLE_WORDS: &[&str] = &[
    "salt",
    "rinse aid",
    "rinse-aid",
    "drying",
    "tablet",
    "tablets",
    "powder",
    "pod",
    "pods",
    "capsule",
    "capsules",
    "detergent",
    "liquid",
    "softener",
    "conditioner",
];
