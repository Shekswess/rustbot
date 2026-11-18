// Reading and response pools for a mobility limit on a trip.

pattern!(
    LIMIT_VERB,
    r#"(?i)\b(?:i|we|my \w+|mum|mom|mother|dad|father|nan|nana|gran|granny|grandma|grandad|grandpa|partner|husband|wife|oh|other half|daughter|son|sister|brother|aunt|uncle|cousin|friend|mil|fil|she|he|they|her|his|their)\s+(?:can'?t|cannot|can not|isnt able to|isn'?t able to|not able to|dont|doesn'?t|do not|does not|struggles? (?:to|with)|struggling (?:to|with)|finds? it hard to|find it hard to|has trouble|have trouble|bad (?:on|at|with))\s+(?:really |even |anymore |any more )?(?:walk(?:ing)? (?:far|much|long distances|very far|any distance)|manage (?:the )?stairs|do (?:the )?stairs|climb (?:the )?stairs|stairs|stand (?:for long|up for long|for very long|about for long|for more than \w+ minutes?)|get (?:about|around) (?:much|easily|well)|(?:walk|move) (?:as )?(?:well|easily) as (?:she|he|they) used to)\b"#
);
pattern!(
    AID,
    r#"(?i)\b(?:uses?|using|needs?|has|got|with|on|in)\s+(?:a |an |his |her |their |my )?(?:wheelchair|mobility scooter|scooter|walking stick|stick|walking frame|zimmer|rollator|walking aid|walker|crutches)\b"#
);
pattern!(
    IN_A_CHAIR,
    r#"(?i)\b(?:is in|are in|in|into) a wheelchair\b|\bwheelchair[- ]?(?:user|bound)\b|\bon crutches\b|cant walk at all|can'?t walk at all|non[- ]?weight ?bearing\b"#
);
pattern!(
    FLAT,
    r#"(?i)\bsomewhere flat\b|\bflat(?:ter)? (?:ground|walk|walking|paths?|route|town|city|seafront|promenade|terrain|streets?)\b|\bneed(?:s)? (?:it|somewhere|the ground) flat\b|\bkeep (?:it|to) (?:the )?flat\b"#
);
pattern!(
    NO_WALKS,
    r#"(?i)\b(?:no|nothing|not|never|nowt)\s+(?:(?:that\s+)?(?:needs?|involves?|involving|means|with)\s+)?(?:too\s+|very\s+|much\s+|any\s+|more\s+)*(?:long\s+|proper\s+|big\s+|far\s+|steep\s+|many\s+)?(?:walks?|walking|hikes?|hiking|hills?|stairs|steps|climbing)\b"#
);
pattern!(
    RULE_LEAD,
    r#"(?i)^(?:(?:and|so|but|then|also|oh|plus|ok|okay|well|just|actually|honestly|now|right)\b[^a-z]*)*"#
);
pattern!(
    RULE_TAIL,
    r#"(?i)^[\s,.;:!-]*(?:(?:please|thanks|ta|then|either|though|too|really|honestly|sadly|unfortunately|today|tomorrow|tonight|now|though|any ?more|at all|this time|for (?:me|us)|on (?:this|the) trip|i think|i suppose)\b[^a-z]*)*$"#
);
pattern!(STAIRS_SAID, r#"(?i)\b(?:stairs|steps|climbing)\b"#);
pattern!(
    REFUSES_AID,
    r#"(?i)\b(?:wont|won'?t|will not|would not|refuses?|refused|refusing|hates?|hated|doesn'?t want|dont want|does not want|do not want)\b[^.]{0,25}\b(?:stick|frame|zimmer|rollator|walker|wheelchair|scooter|aid|rail|handle|crutch(?:es)?)\b"#
);
pattern!(
    DISLIKES_THE_WALK,
    r#"(?i)\b(?:hates?|hated|dislikes?|can'?t stand|cannot stand|cant stand|doesn'?t like|don'?t like|do not like|isn'?t keen on|not keen on|would rather not|prefers? not to)\b[^.]{0,24}\b(?:walk(?:ing)?|hikes?|hiking|rambl(?:e|ing)|hills?|steps?|stairs)\b|\brather sit\b|\bsit in a pub\b"#
);
pattern!(
    PUSHCHAIR,
    r#"(?i)\b(?:pushchair|pram|buggy|stroller|car ?seat|carrycot|cot|highchair)\b"#
);
pattern!(
    FLAT_IS_A_HOME,
    r#"(?i)\b(?:to live|to rent|to buy|to move to|a flat|flat to|ground floor flat|bungalow)\b"#
);
pattern!(
    ILLNESS,
    r#"(?i)\b(?:pain|painful|hurts?|hurting|swollen|swelling|fell|fallen|fall|falls|dizzy|numb|weak|unwell|ill|poorly|blood|bleed|bleeding|bruise|broken|fracture|sore|stiff|doctor|gp|nurse|hospital|a&e|ambulance|111|999|since (?:her|his|the)|getting worse|arthritis|dementia|alzheimers|parkinsons|ms|stroke|surgery|operation|replacement|physio|physiotherapy|chemo|cancer|diagnosis|tablets?|medication|prescribed)\b"#
);
pattern!(
    PET,
    r#"(?i)\b(?:dog|dogs|puppy|pup|cat|cats|kitten|pet|pets|rabbit|hamster|guinea|horse|pony|ferret|parrot)\b"#
);
pattern!(
    ASKS,
    r#"(?i)^\s*(?:what|where|when|why|how|which|who|can|could|do|does|did|is|are|was|were|will|would|should|shall|any|anyone|anybody)\b"#
);
pattern!(
    A_TRIP,
    r#"(?i)\b(?:trip|trips|holiday|holidays|city break|weekend|days? away|nights? away|day out|days out|flight|flights|flying|abroad|visit(?:ing)?|sightsee\w*|tour(?:ing)?|museums?|galleries|castles?|cathedral|hotels?|somewhere to stay|what should we see|where should we go|things to do|days in|nights in|weeks in|a week|a few days|\d+ (?:days|nights)|taking \w+(?: \w+)? (?:to|off)|off to|going to|booked|booking|somewhere to stay|seaside|beach|coast|countryside|shopping|old town|town cent(?:re|er))\b"#
);
pub(super) const WALK_REPLIES: &[&str] = &[
    "Then plan for benches. One big thing a morning, a sit-down after it, and a taxi or a bus for the long hops - two sights close together beats four across town.",
    "Good to know. Keep the days short and stop before the legs give out rather than after, and check the step-free way into the one place you most want to see.",
    "That shapes the trip more than the sights do: one thing a day, short walks between, and no shame in a taxi for the last stretch.",
    "Then build the day around a sit-down every hour or so, and pick your two must-sees rather than a list. Most museums have a step-free entrance if you ask before you go.",
    "Fair enough - short hops, a stop for a coffee in the middle, and somewhere to sit at the end. That is a better trip than a marathon of churches anyway.",
];
pub(super) const STAIRS_REPLIES: &[&str] = &[
    "Ask about lifts and step-free entrances before you book anything: most big places have them, and it is usually not the front door.",
    "Then ring ahead for the ones you care about - a lift to the good floors and a loo you can get to are the two questions worth asking.",
    "Old towns are the hard part, so keep to the flatter streets and take the bus between the bits at the top of the hill.",
];
pub(super) const CHAIR_REPLIES: &[&str] = &[
    "Then ask about the step-free entrance for each place before you go - most big museums have one. Cobbles and the oldest streets are the thing to plan around.",
    "Ring ahead for the ones that matter: step-free entry, a lift to the good floors, and an accessible loo. Most places will tell you straight if you ask.",
    "Plan around the ground rather than the sights: smooth pavements, a bus or a taxi between stops, and one thing a day is plenty.",
];
