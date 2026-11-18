//! Source appliance and mower branch order.

pub(super) const MACHINE_BURNING: &[&str] = &[
    "smoking",
    "smokes",
    "smoke",
    "sparking",
    "sparks",
    "sparked",
    "spark",
    "smells of burning",
    "smells like burning",
    "smells like its burning",
    "smell of burning",
    "burning smell",
    "burnt plastic",
    "smells hot",
    "getting very hot",
    "overheating",
    "caught fire",
    "smouldering",
    "smoldering",
];
pub(super) const MACHINE_LEAKING: &[&str] = &[
    "leaking",
    "leaks",
    "leaked",
    "dripping",
    "drips",
    "water everywhere",
    "puddle",
    "puddles",
    "water coming out",
];
pub(super) const DEAD_ALREADY_ANSWERED: &[&str] = &[
    "kettle",
    "toaster",
    "sandwich toaster",
    "coffee machine",
    "espresso",
    "bean to cup",
    "air fryer",
    "airfryer",
    "air-fryer",
    "microwave",
];
pub(super) const BURNING_ALREADY_ANSWERED: &[&str] = &[
    "air fryer",
    "airfryer",
    "air-fryer",
    "toaster",
    "sandwich toaster",
    "kettle",
    "microwave",
];
pub(super) const MOWER_CORDLESS: &[&str] = &[
    "cordless",
    "battery mower",
    "battery lawnmower",
    "battery powered",
    "battery-powered",
    "no cable",
    "electric mower",
    "robot mower",
    "robot lawnmower",
    "charging",
];
pub(super) const MOWER_BATTERY_BUY: &[&str] = &[
    "which battery",
    "what battery",
    "what sort of battery",
    "what size battery",
    "which mower battery",
    "best battery",
    "battery do i need",
    "battery should i buy",
    "replace the battery",
    "new battery",
];
pub(super) const MOWER_BATTERY: &[&str] = &[
    "battery",
    "batteries",
    "battery is flat",
    "battery flat",
    "battery dead",
    "dead battery",
    "flat battery",
    "jump start",
    "jumping it",
    "charger",
    "starter",
    "starter motor",
    "turn over",
    "turns over",
    "turning over",
    "wont turn over",
    "won t turn over",
];
pub(super) const MOWER_PETROL: &[&str] = &[
    "wont start",
    "won t start",
    "not starting",
    "not start",
    "stalls",
    "stalling",
    "cutting out",
    "smoking",
    "smoke",
    "coughing",
    "spluttering",
    "firing",
];
pub(super) const MOWER_BLADE: &[&str] = &[
    "blade",
    "blades",
    "blunt",
    "sharpen",
    "sharpening",
    "sharp",
    "cutting",
    "leaves grass",
    "leaves the grass",
    "torn grass",
    "uneven",
    "scruffy",
];
pub(super) const NOISE_KINDS: &[(&str, &[&str])] = &[
    ("grind", &["grind", "scrap", "roar", "rumbl"]),
    (
        "squeak",
        &[
            "squeak", "squeal", "screech", "shriek", "scream", "whine", "whistl",
        ],
    ),
    ("knock", &["knock", "bang", "thump", "hammer"]),
    ("rattle", &["rattl", "vibrat", "shak", "clunk", "clank"]),
    ("click", &["click", "tick"]),
    ("gurgle", &["gurgl", "glug"]),
    (
        "loud",
        &[
            "noise",
            "noises",
            "noisy",
            "loud",
            "racket",
            "racketing",
            "din",
            "clatter",
            "clattering",
            "thunderous",
            "deafening",
            "row",
            "drone",
            "droning",
            "palaver",
        ],
    ),
];
pub(super) struct NoiseRow {
    pub(super) names: &'static [&'static str],
    pub(super) normal: &'static str,
    pub(super) first: &'static [(&'static str, &'static [&'static str])],
    pub(super) kinds: &'static [(&'static str, &'static str)],
    pub(super) urgent: &'static str,
}
pub(super) const NOISE_ROWS: &[NoiseRow] = &[
    NoiseRow {
        names: &["dishwasher", "dish washer"],
        normal: "A hum from the dishwasher is the wash pump and the whoosh at the start is it draining, both ordinary.",
        first: &[],
        kinds: &[
            (
                "grind",
                "Grinding can come from the filter or debris in the sump. Switch off and unplug the dishwasher before checking the filter. Follow the manual and be alert for broken glass.",
            ),
            (
                "rattle",
                "A rattle is normally cutlery in the basket or a spray arm fouling the rack.",
            ),
            (
                "knock",
                "A bang can come from a pan hitting a spray arm. With the dishwasher switched off, check that the arms turn freely and that the filter is seated. If the noise happens while draining, get the pump checked before reaching into the sump.",
            ),
        ],
        urgent: "If water has reached the electrics, keep clear and switch off only from a dry, safe point.",
    },
    NoiseRow {
        names: &["tumble dryer", "dryer"],
        normal: "A steady rumble from the tumble dryer is the drum turning, which is ordinary.",
        first: &[("shake", &["shak", "vibrat"])],
        kinds: &[
            (
                "squeak",
                "A squeak can come from a belt or a drum roller. Switch off the dryer before checking the filter. Get recurring squeaks checked.",
            ),
            (
                "rattle",
                "A rattle can come from loose coins or a bra wire. Switch off the dryer before checking the drum.",
            ),
            (
                "shake",
                "A dryer that shakes is nearly always the load or the feet: undo the tangle, put a couple of towels in with a heavy item, and turn the front feet out until it sits flat and cannot rock. If it still walks, an overloaded drum or the machine touching a cupboard is the next thing to check - and a dryer has no transport bolts to undo.",
            ),
            (
                "grind",
                "Grinding means stop using it and arrange for it to be checked.",
            ),
        ],
        urgent: "Lint round the heater is a fire risk, so clear the filter after every load.",
    },
    NoiseRow {
        names: &["oven fan", "oven"],
        normal: "The fan running on after the oven is switched off is it cooling itself, which is ordinary.",
        first: &[],
        kinds: &[
            (
                "rattle",
                "A rattle is usually a bent blade or baked-on grease on the fan.",
            ),
            (
                "squeak",
                "A squeal is the fan motor, which is an engineer's job.",
            ),
            (
                "loud",
                "A noisy fan is usually baked-on grease throwing it off balance, which a good clean fixes.",
            ),
        ],
        urgent: "If the oven also heats unevenly, the fan is the thing to have looked at.",
    },
    NoiseRow {
        names: &["kettle"],
        normal: "A rumble as it comes to the boil is ordinary.",
        first: &[],
        kinds: &[
            (
                "rattle",
                "Limescale can cause crackling. Descale only as the kettle's manual directs.",
            ),
            (
                "loud",
                "A noisy kettle can be caused by limescale. Descale only as the kettle's manual directs.",
            ),
            (
                "click",
                "Clicking off before it boils is a scaled or worn switch, which is not worth repairing.",
            ),
        ],
        urgent: "Follow the manual for safe descaling, and never fill above the maximum mark.",
    },
    NoiseRow {
        names: &["microwave"],
        normal: "A hum is the microwave's transformer and fan, and a clunk is the turntable, all ordinary.",
        first: &[],
        kinds: &[
            (
                "rattle",
                "A rattle is usually the turntable roller or the plate not seated.",
            ),
            (
                "squeak",
                "A squeak from the turntable motor is harmless until the plate stops turning.",
            ),
        ],
        urgent: "A loud buzz with no heat, a burning smell, or sparking at the door mesh means stop using it.",
    },
    NoiseRow {
        names: &["vacuum", "hoover"],
        normal: "A high whine from a vacuum is a blocked hose or a full bag.",
        first: &[],
        kinds: &[
            (
                "rattle",
                "A rattle can come from something caught in the brush bar. Switch off and unplug the vacuum or remove the battery before checking it as the manual directs.",
            ),
            (
                "squeak",
                "A squeal can come from hair wound round the brush bar. Switch off and unplug the vacuum or remove the battery before cleaning it as the manual directs.",
            ),
        ],
        urgent: "A burning smell means switch off: the motor is overheating.",
    },
    NoiseRow {
        names: &[
            "fan",
            "fans",
            "ceiling fan",
            "desk fan",
            "pedestal fan",
            "floor fan",
        ],
        normal: "A fan hums because that is the motor and the moving air, so a steady low hum is the fan working: on a hot night it is louder than you expect because the room is silent and it is running all night.",
        first: &[],
        kinds: &[
            (
                "rattle",
                "A rattle can come from the guard or a dusty blade. Unplug the fan and follow the manual for cleaning or tightening user-accessible parts.",
            ),
            (
                "squeak",
                "A squeal or screech can come from a bearing. Stop using the fan and follow the manufacturer's instructions for service or replacement.",
            ),
            (
                "grind",
                "Grinding means stop using it: the bearing has gone.",
            ),
            (
                "click",
                "A click can come from the guard or a loose blade. Unplug the fan and check the manual before adjusting any parts.",
            ),
        ],
        urgent: "Unplug it before you touch the blades. Stop using a fan whose plug or cable gets hot.",
    },
];
pub(super) const LEAK_CHECKS: &[(&[&str], &str)] = &[
    (
        &["kettle"],
        "Stop using the leaking kettle. Switch it off and unplug it if safe. A damaged seal or split body can let water reach electrical parts. Ask the retailer about replacement.",
    ),
    (
        &["coffee machine", "espresso", "bean to cup"],
        "Switch off and unplug the coffee machine and let it cool. Check the removable tank and seals in the manual. If it still leaks, ask the manufacturer or an appliance repairer.",
    ),
    (
        &["blender", "food processor", "juicer", "soup maker"],
        "Unplug the appliance before checking the jug or bowl's seal. Follow the manual for any parts near the blades. If the jug is split, get the right replacement for your model.",
    ),
    (
        &[
            "slow cooker",
            "pressure cooker",
            "instant pot",
            "multi cooker",
        ],
        "Switch off and unplug the cooker if safe and let it cool. Follow the model instructions for pressure release and for inspecting the fill level, lid seal and bowl.",
    ),
];
pub(super) const MACHINE_CHECKS: &[(&[&str], &str, &str)] = &[
    (
        &["air fryer", "airfryer", "air-fryer"],
        "Check the basket and drawer for old grease only once the air fryer is off, unplugged and cool. If it sparked or is damaged, have it checked before using it again.",
        "Unplug it, check the fuse in the plug and try another socket; if the display stays dark the element or the thermal cut-out has gone and it is not a repair worth doing. If it lights up but never gets hot, the element itself has failed.",
    ),
    (
        &[
            "slow cooker",
            "pressure cooker",
            "instant pot",
            "multi cooker",
        ],
        "Stop using it. A slow cooker or pressure cooker with a burning smell may have a fault. Ask the retailer or manufacturer to check it once it has cooled.",
        "Check the socket and the fuse first, then that the pot or the inner bowl is sitting right down on the base - a slow cooker that will not heat is usually one of those three and not the machine.",
    ),
    (
        &["microwave"],
        "Stop using the microwave, keep the casing closed and have it checked if it sparks or has a burning smell.",
        "If the light and the clock work but nothing heats, the magnetron or its fuse has gone and it is an engineer's job. If it is completely dead, check the fuse in the plug first - a microwave on the same ring as a kettle blows it more often than you would think.",
    ),
    (
        &["kettle"],
        "Stop using the kettle if it sparks or smells of burning. Have it checked or replaced.",
        "If it will not switch on at all, check the fuse in the plug and try another socket first, because the contacts in the base are a common fault. If it stays dead it is a new kettle rather than a repair.",
    ),
    (
        &[
            "toaster",
            "sandwich toaster",
            "grill",
            "griddle",
            "waffle maker",
            "panini",
        ],
        "Stop using it if it sparks or smells of burning. Unplug it if safe and ask the manufacturer or retailer to check it.",
        "If it will not pop up, the spring or the timer has gone and it is a new toaster. If it will not heat at all, check the fuse in the plug first, then the timer dial.",
    ),
    (
        &[
            "bread maker",
            "bread machine",
            "stand mixer",
            "blender",
            "food processor",
            "soup maker",
            "yogurt maker",
            "coffee machine",
            "rice cooker",
            "deep fat fryer",
            "fryer",
            "juicer",
        ],
        "A kitchen machine that smells of burning may be overheating or have a fault. Stop using it and ask the manufacturer or retailer to check it.",
        "Check the fuse in the plug, then the switch, then the lid or the bowl interlock - a machine that will not start very often thinks the lid is open. If it hums and does not turn, the motor or the belt has gone.",
    ),
];
