//! Appliance aliases and fault branches from PyBot home.py.

pub(super) const DRYER_KINDS: &[&str] = &["heat pump", "condenser", "vented"];
pub(super) const A_DRAIN_FAULT: &[&str] = &[
    "not draining",
    "wont drain",
    "won't drain",
    "will not drain",
    "not draining out",
    "wont drain out",
    "won't drain out",
    "not pumping out",
    "wont pump out",
    "won't pump out",
    "water left in",
    "water sitting in",
    "standing water in",
    "full of water",
    "water in the drum",
];
pub(super) const MACHINE_NAMES: &[(&str, &str)] = &[
    ("washing machine", "washing machine"),
    ("washer", "washing machine"),
    ("tumble dryer", "tumble dryer"),
    ("dishwasher", "dishwasher"),
    ("fridge", "fridge"),
    ("freezer", "freezer"),
    ("oven", "oven"),
    ("cooker", "cooker"),
    ("hob", "hob"),
    ("microwave", "microwave"),
    ("boiler", "boiler"),
    ("dryer", "tumble dryer"),
    ("vacuum", "vacuum cleaner"),
    ("hoover", "vacuum cleaner"),
    ("hoover", "vacuum cleaner"),
    ("vacuum", "vacuum cleaner"),
    ("dyson", "vacuum cleaner"),
    ("henry", "vacuum cleaner"),
    ("machine", "machine"),
    ("machines", "machine"),
    ("appliance", "machine"),
    ("appliances", "machine"),
];
pub(super) const MACHINE_PARTS: &[(&[&str], &str)] = &[
    (
        &["bearing", "bearings", "drum bearing", "drum bearings"],
        "A rumble or grinding that grows louder during spin can come from the drum bearing. Unplug the machine before checking whether the drum turns roughly by hand. A bearing repair can involve removing the drum or replacing a sealed tub. Ask an appliance engineer for a quote before deciding whether to repair or replace it.",
    ),
    (
        &["belt", "belts", "drive belt", "drive belts", "fan belt"],
        "The drive belt runs around the motor and the drum or brush bar. Unplug the machine. Check the manual to see whether the belt is user-replaceable. If access means opening the appliance casing, ask a qualified appliance repairer. Use the model number to get the correct part and check the guarantee before agreeing to work.",
    ),
    (
        &[
            "pump",
            "pumps",
            "drain pump",
            "waste pump",
            "not draining",
            "wont drain",
            "won't drain",
            "will not drain",
            "not draining out",
            "wont drain out",
            "won't drain out",
            "not pumping out",
            "wont pump out",
            "won't pump out",
            "water left in",
            "water sitting in",
            "standing water in",
            "full of water",
            "water in the drum",
        ],
        "The drain pump sends the water out. Switch off and unplug the appliance and let any hot water cool. Use the manual to drain it and check an accessible filter for a blockage. If it still won't drain, get the pump checked before ordering a replacement.",
    ),
    (
        &[
            "brush bar",
            "brushbar",
            "brush roll",
            "brushroll",
            "beater bar",
            "beater bars",
            "roller",
            "rollers",
        ],
        "Unplug the vacuum or remove its battery as the manual directs before checking the brush bar. Clear hair or thread with the method your model's manual describes. If the brush bar turns freely by hand but stops in use, the belt may need checking. Only remove user-serviceable parts as directed in the manual.",
    ),
];
pub(super) const CUTTING_OUT: &[(&[&str], &str)] = &[
    (
        &["kettle", "boils", "boil"],
        "Limescale can make a kettle switch off before it boils. Check the manual for the correct descaling method and product for your model. If it still switches off early, ask the manufacturer or retailer to check it.",
    ),
    (
        &["microwave"],
        "A microwave may stop early when it overheats. Turn it off and let it cool, then check the external vents and use the clearance in the manual. If it keeps stopping or the door does not close properly, stop using it and ask the manufacturer or an engineer.",
    ),
    (
        &["dishwasher"],
        "A dishwasher that stops part way can have a blocked drain or an unlatched door. Switch it off before checking the filter as the manual directs, look for a kinked hose, and check the door latch. Check the manual for any fault symbol.",
    ),
    (
        &["washing machine", "washer", "spin", "drum"],
        "A washer that stops mid-cycle can have an unlatched door or a blocked drain. Check the door clicks shut, then turn off and unplug the machine before checking the filter or hose as the manual directs. Look up the error code for your exact model.",
    ),
    (
        &["toaster", "toast", "pops", "popping", "pop up"],
        "A toaster that pops early can have a full crumb tray or a worn timer. Unplug it and let it cool before emptying the crumb tray. Check the timer setting. If it still pops after a few seconds at every setting, ask the manufacturer about the latch.",
    ),
    (
        &["dryer", "tumble dryer", "tumble"],
        "A dryer can cut out when it overheats. Switch it off and check the lint filter and any user-accessible condenser or vent as the manual directs. A blocked vent is a fire risk. If it keeps cutting out, stop using it and have it checked.",
    ),
    (
        &["oven"],
        "An oven that cuts out may have a thermostat or cooling fan problem. If it heats unevenly or the fan is noisy, note what happens and when. Have an appliance engineer check it if the fault recurs.",
    ),
];
