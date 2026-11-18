//! Authored predicates and replies from PyBot 7d63c7f.

pub(super) const A_WEDDING_SHORTAGE: &str = r#"\b(?:nothing (?:to wear|that fits|at all to wear|in my wardrobe|i can wear)|no (?:outfit|idea what to wear)|haven'?t got (?:anything|a thing|an outfit|nothing)|don'?t have (?:anything|an outfit|a thing)|need (?:something|an outfit|a suit|a dress)|any ideas|ideas|suggestions|suggest|help)\b"#;

pub(super) const A_NAMED_DRESS_CODE: &str = r#"\b(?:black tie|white tie|morning dress|lounge suit|bow tie|dress code|cocktail|smart casual|colour|color|jumpsuit|hat|shoes?|tie)\b"#;

pub(super) const SOCKS_FORMAL: &str = r#"(?i)\b(?:suit|suits|wedding|weddings|interview|interviews|funeral|funerals|smart|formal|black tie|office|work|dress|trousers|shirt|shirts)\b"#;

pub(super) const SOCKS_CASUAL: &str = r#"(?i)\b(?:jeans|trainer|trainers|sneaker|sneakers|shoe|shoes|plimsoll|plimsolls|gym|shorts|casual|sports?|running|walking|hiking|boots?)\b"#;

pub(super) const SOCKS_COLOUR: &str =
    r#"(?i)\b(?:brown|black|navy|grey|gray|white|tan|blue|beige)\b"#;

pub(super) const SOCKS_BLACK_AND_BROWN: &str = r#"(?i)\bblack\b.*\bbrown\b|\bbrown\b.*\bblack\b"#;

pub(super) const SOCKS_SPORT: &str = r#"(?i)\b(?:run|running|jog|jogging|marathon|half marathon|10k|5k|parkrun|gym|cycling|walking|hiking|sport|sports|football|rugby|netball|hockey)\b"#;

pub(super) const ENOUGH_FOR_A_GARMENT: &str = r#"\b(?:cold|warm|hot|cool|wet|dry|windy|sunny|chilly|freezing|mild|damp) enough for (?:a |an |the |my |your )?(?:coat|thick coat|jacket|jumper|cardigan|fleece|shorts|wellies|wellies|boots|hat|gloves|scarf|t-shirt|tshirt|tee shirt|dress|sandals|sun hat|raincoat)\b"#;

pub(super) const A_SIZE_SAID: &str = r#"\b(?:size|uk)\s*(\d{1,2})\b"#;

pub(super) const LACE_TEACH: &[&str] = &[
    "teach",
    "teaching",
    "learn",
    "learning",
    "child",
    "children",
    "kid",
    "kids",
    "son",
    "daughter",
    "toddler",
    "what age",
    "when can",
    "old enough",
    "should be able",
];

pub(super) const LACE_SKATES: &[&str] = &[
    "skate",
    "skates",
    "ice skates",
    "ice skate",
    "roller skates",
    "hockey skates",
    "skating",
];

pub(super) const LACE_BOW: &[&str] = &[
    "a bow", "bow on", "gift bow", "present", "ribbon", "gift", "wrapping", "wrapped",
];

pub(super) const LACE_UNDONE: &[&str] = &[
    "coming undone",
    "comes undone",
    "come undone",
    "keeps coming",
    "wont stay tied",
    "won't stay tied",
    "never stays tied",
    "untied",
    "trailing",
    "trails",
    "trail",
    "dragging",
    "drag",
    "too long",
    "keep treading on",
];

pub(super) const LACE_TEACHING: &str = "About five or six, and it comes quicker if the lace is the practice rather than the shoe: tie a lace round a table leg, a ribbon round a chair back, or one big shoe on the floor, and teach the bunny-ears version first - two loops, cross, push one through. Say one step at a time and let them repeat it five times before adding the next, and be slow in front of them with your own laces. Fifteen minutes on a Saturday works better than twenty rushed tries on the way to school, and being allowed to be bad at it for a few weeks is part of learning it.";

pub(super) const LACE_BOW_REPLY: &str = "Use a ribbon rather than string, because it holds its shape: make a loop in each hand, cross the right loop over the left, tuck it behind and bring it up through the gap underneath, then pull both loops outward until the bow is the size you want. Tape the ribbon to the box underneath first if the present is heavy, so it cannot slide, and curl the tails over a scissor blade and cut them on the angle.";

pub(super) const LACE_SKATES_REPLY: &str = "Skates want a firmer lace than shoes and nothing left trailing: lace from the bottom, keep the foot snug through the middle, then pull the top two tight at the ankle so the heel cannot lift, double knot it, and tuck the ends inside the top of the boot. Do them up with thin socks or warm feet, because tight laces on cold feet is how toes go numb, and loosen the top two before you take them off.";

pub(super) const LACE_UNDONE_REPLY: &str = "It is the knot rather than your tying: a bow on a smooth lace works loose. Tie the first knot twice - a double knot - for round, waxed or nylon laces, or switch to flat laces with a bit of texture, and check you have about 10cm of tail after the bow because a long end swings and pulls itself free. Running for a bus or a bike ride: tuck the ends and loops under the criss-cross above them and they will stay put all day.";

pub(super) const LACE_KNOT: &str = "Make two loops and tie them like a bow: cross the laces once, take the top one under and through the gap so the base knot is snug, then bend each lace into a loop, cross the two loops over, and push one loop down through the hole beneath them. Pull the loops outward rather than up, and it sits flat. Two things make the rest easy: lace from the bottom up so every row lies flat, and leave about 10cm of lace after the bow - a long tail is what drags and comes undone.";

pub(super) const A_SHOE_SIZE: &[&str] = &[
    "shoe", "shoes", "trainer", "trainers", "boot", "boots", "sneaker", "sneakers",
];

pub(super) const WOMENS_SIZES: &[(u8, u8, u8)] = &[
    (6, 34, 2),
    (8, 36, 4),
    (10, 38, 6),
    (12, 40, 8),
    (14, 42, 10),
    (16, 44, 12),
    (18, 46, 14),
    (20, 48, 16),
];

pub(super) const SOCKS_FOR_SPORT: &str = "For running, walking or the gym, wear a thin synthetic or merino-wool sock with a flat seam across the toe, in the same shoe you train in: cotton holds sweat against the skin and that is what blisters come from. Two thin pairs are warmer and rub less than one thick pair, and the sock should be tight enough not to bunch under the arch.";

pub(super) const SOCKS_BLACK_BROWN: &str = "Black socks with brown shoes is the pairing to avoid: the sock should match the trousers, so with brown shoes wear a dark brown, navy or charcoal sock, or keep the shoes for a lighter suit. Nobody notices a sock that matches the trousers and everybody notices one that fights the shoes.";

pub(super) const SOCKS_SMART: &str = "Dark socks that match the trousers rather than the shoes, long enough to cover the calf so no bare leg shows when you sit down: navy, charcoal or black in wool or a wool blend, plain unless the suit is patterned. Black socks with brown shoes is the one pairing that looks wrong, so match the trousers and let the shoes be their own colour.";

pub(super) const SOCKS_WHITE: &str = "White socks are fine with trainers, shorts and gym kit, and wrong with a suit or smart trousers: with those, wear a dark sock that matches the trousers, long enough that the calf stays covered when you sit down.";

pub(super) const SOCKS_EVERYDAY: &str = "Go by the trousers rather than the shoes: a plain sock a shade darker than the trousers is the safe pairing, which means dark grey or navy with jeans and charcoal or black with smart trousers. Cotton for everyday, wool or a wool blend when the foot will be in a shoe all day, because cotton stays wet against the skin.";

pub(super) const SOCKS_DEFAULT: &str = "Match the trousers rather than the shoes, dark navy, charcoal or black, and long enough that no bare leg shows when you sit down. Cotton for everyday and wool or a wool blend for a shoe you are in all day, and thin wool rather than thick cotton with anything smart, because a thick sock makes a well-fitted shoe tight.";
