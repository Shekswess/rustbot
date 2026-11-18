//! Original PyBot planning phrases and replies.

pub(super) const OWN_COUNT: &str = r#"\bhow many (?:\w+ ){0,3}(?:do|does|did) (?:i|we) (?:have|got)\b|\bhow many (?:\w+ ){0,3}have (?:i|we) got\b"#;
pub(super) const A_DATE_ASK: &str = r#"\b(?:when|what date|which date|what day)\b"#;
pub(super) const A_DO: &str = r#"\b(?:party|parties|invitations?|invites?|guests?|venue|plan|plans|planning|organis\w*|organiz\w*|menu|buffet|food|catering|decorations?|cake|games|themes?|book|booking)\b"#;
pub(super) const HOME_IN_THE_LINE: &str =
    r#"(?i)\b(?:house|houses|home|homes|flat|flats|apartment|apartments)\b"#;
pub(super) const A_MOVE_TO_PLAN: &str = r#"(?i)\b(?:plan|planning|planned|organise|organize|organising|organizing|prep|prepare|preparing|arrange|arranging|checklist|tips?|advice|start|started|starting|before|ahead|get ready|getting ready|ready)\b.{0,24}\b(?:a move|the move|the house move|my move|our move|move out|move in)\b|\b(?:a move|the move|my move|our move)\b.{0,24}\b(?:plan|planning|organise|organize|checklist|tips?|advice|prepare|prep|arrange)\b|\bwhen\s+(?:i|we|you)\s+move\b|\bwhen\s+moving\b|\bbefore\s+(?:i|we|you)\s+move\b|\bwe\s*(?:'|\s)?re\s+moving\b"#;
pub(super) const A_MOVE_ELSEWHERE: &str = r#"(?i)\b(?:abroad|overseas|emigrat\w*|visa|another country|different country|new country|australia|canada|new zealand|america|usa|spain|france|portugal|germany|ireland|dubai|team|teams|job|jobs|career|office|house share|houseshare|chess|piece|checkmate|the goalposts|goalposts|in together|moving in with|move in with)\b"#;
pub(super) const FOR_A_RAINY_DAY: &str = r#"\bfor a rainy day\b"#;
pub(super) const MONEY_WITH_IT: &str = r#"\b(?:save|saves|saving|saved|savings|put aside|aside|fund|funds|money|cash|pounds?|quid|income|salary|afford|afforded|budget|emergency|invest|investing|interest)\b"#;
pub(super) const BARE_RID: &str = r#"\bget rid of\s*(?:$|[.?!])"#;
pub(super) const TOO_MUCH: &str =
    r#"\btoo much (?:stuff|clutter|junk|rubbish|things|mess|gear|crap|tat)\b"#;
pub(super) const A_MILESTONE_PERSON: &str = r#"\b(?:mum|mums|mom|moms|mam|mams|dad|dads|father|mother|nan|nans|nana|nan's|gran|grans|granny|grandma|grandad|grandpa|grandmother|grandfather|aunt|auntie|uncle|sister|brother|wife|husband|partner|mother in law|father in law|she|he|son|sons|daughter|daughters|grandson|granddaughter|grandchild|godson|goddaughter|nephew|niece|kid|kids|child|children)\b"#;
pub(super) const A_MILESTONE_AGE: &str =
    r#"\b(?:is|turns|turning|will be|going to be|hits|becomes)\s+(?:the age of\s+)?(\d{1,2})\b"#;
pub(super) const A_CHILD_AT_A_MILESTONE: &str = r#"\b(?:son|sons|daughter|daughters|grandson|granddaughter|grandchild|godson|goddaughter|nephew|niece|kid|kids|child|children|boy|girl|toddler)\b"#;
pub(super) const A_MILESTONE_SOON: &str = r#"\b(?:next (?:week|month|weekend|saturday|sunday|friday|year)|this (?:week|weekend)|on (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|soon|coming up|in \w+ (?:weeks|days))\b"#;
pub(super) const A_PRESENT_ASKED: &str = r#"\b(?:present|presents|gift|gifts)\b|\bwhat\s+(?:should|shall|can|could|do|would)\s+(?:i|we)\s+(?:get|buy|give|make|send)\b|\b(?:get|buy|give|make|send)\s+(?:her|him|them|my\s+(?:mum|mom|mother|dad|father|gran|grandma|grandad|granny|nan|nana|wife|husband|partner|son|daughter|sister|brother|friend|parents?))\b|\bideas?\s+(?:for|of)\s+(?:a\s+)?(?:present|gift)\b"#;
pub(super) const AGE_AT_PARTY: &str = r#"\b(\d{1,2})(?:st|nd|rd|th)?\b"#;
pub(super) const JOB_NAMED: &[&str] = &[
    "clear out",
    "clearing out",
    "clearout",
    "declutter",
    "clear the clutter",
    "clear clutter",
    "big tidy",
    "tidy up the house",
    "tidy my house",
    "tidy up my house",
    "sort the house out",
    "sort the house",
];
pub(super) const DECLUTTER_ROOM: &[&str] = &[
    "room",
    "rooms",
    "house",
    "home",
    "flat",
    "place",
    "garage",
    "loft",
    "shed",
    "attic",
    "wardrobe",
    "wardrobes",
    "cupboard",
    "cupboards",
    "closet",
    "kitchen",
    "bedroom",
    "bedrooms",
    "cellar",
    "spare room",
    "drawer",
    "drawers",
    "shelves",
    "hall",
    "hallway",
    "landing",
    "under the stairs",
    "cabinets",
    "units",
    "car boot",
    "box room",
    "study",
    "desk",
    "filing",
];
pub(super) const DECLUTTER_STUFF: &[&str] = &[
    "stuff",
    "clutter",
    "junk",
    "rubbish",
    "things",
    "mess",
    "gear",
    "bits",
    "crap",
    "tat",
    "odds and ends",
    "it all",
    "everything",
];
pub(super) const GIFT_ASK: &[&str] = &[
    "what should i get",
    "what do i get",
    "what can i get",
    "what to get",
    "what should i buy",
    "what can i buy",
    "what to buy",
    "should i get",
    "ideas for a present",
    "ideas for a gift",
    "present ideas",
    "gift ideas",
    "ideas for presents",
    "ideas for the present",
    "ideas for her",
    "ideas for him",
    "present for",
    "presents for",
    "gift for",
    "gifts for",
    "buy for",
    "get her for",
    "get him for",
    "get them for",
    "get my mum",
    "get my dad",
    "get my mam",
    "get my mom",
    "get my mother",
    "get my father",
    "get my sister",
    "get my brother",
    "get my nan",
    "get my gran",
    "get my partner",
    "get my wife",
    "get my husband",
    "get my friend",
];
pub(super) const CHILD_AT_PARTY: &[&str] = &[
    "kid",
    "kids",
    "child",
    "children",
    "childs",
    "toddler",
    "toddlers",
    "son",
    "sons",
    "daughter",
    "daughters",
    "grandson",
    "granddaughter",
    "grandchild",
    "grandchildren",
    "godson",
    "goddaughter",
    "school",
    "nursery",
    "soft play",
    "playcentre",
    "reception class",
    "year one",
];
pub(super) const KIDS_PARTY: &str = "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.";
pub(super) const ADULT_PARTY: &str = "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.";
