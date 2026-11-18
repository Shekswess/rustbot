// Rule and wording data from the pinned source.

pattern!(
    ASKING,
    r#"(?i)^\W*(?:(?:do|did|does|have|has|are|is|was|were|can|could|would|should|will)\s+(?:i|you|we|they|he|she|it|there|that|this|these|those|any|some|my|your|our|their|his|her|its)\b|how|what|which|who|when|where|why)\b"#
);
pattern!(
    GROUP_OF_US,
    r#"(?i)\b(?:(?:there|it)\s*(?:'s|s|'re|re|are|is|'ll|ll|will|would)|we\s*(?:'re|re|are|'ll|ll|will))\s+(?:(?:going to be|gonna be|will be|would be|should be|be)\s+)?(?:(?:just|only|now|still|currently|actually)\s+){0,2}(?:the\s+)?(?P<n>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+of us\b(?=\s*(?:$|[,.!?;:]|\b(?:here|now|at home|in the house|in the flat|in the home|in total|altogether|total|and|but|so)\b))|\b(?:we\s*(?:'re|re|are)|there\s*(?:'s|s|'re|re|are|is))\s+(?:(?:just|only|now|still|currently|actually)\s+){0,2}(?P<n2>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+(?:people|persons?)\b(?=\s*(?:$|[,.!?;:]|\b(?:here|now|at home|in the house|in the flat|in the home|in total|altogether|total|and|but|so)\b))|^\W*(?:(?:and|so|well|ok|okay|oh|hmm)\b[,.]?\s+)*(?:(?:just|only)\s+)?(?:the\s+)?(?P<n3>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+of us\b(?=\s*(?:$|[,.!?;:]|\b(?:here|now|at home|in the house|in the flat|in the home|in total|altogether|total|and|but|so)\b))"#
);
pattern!(
    GROUP_CORRECTED,
    r#"(?i)^\W*(?:no|nope|sorry|actually|wait|i mean|make it|make that|hold on|hang on|correction)\b[^.!?]{0,12}?(?:the\s+)?(?P<n>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+of us\b"#
);
pattern!(
    GROUP_OF_FOUR,
    r#"(?i)\b(?:family|household)\s+of\s+(?P<n>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\b(?!\s+[a-z])"#
);
pattern!(
    GROUP_ASK,
    r#"(?i)\bhow many of us(?:\s+(?:(?:are|is|were|was|there|here|now|altogether|total|in all|at home|it|that|again|then|did i say|did we say|do you remember|remind me|in the house|in the flat|in the home|in my (?:house|flat|home|household)|in our (?:house|flat|home|household)|live here|living here|live in the (?:house|flat|home)|living in the (?:house|flat|home))|(?:is it|was it|were they|is that|was that|again|then|did i say|did we say|i say)))*\s*[?.!]*$|\bhow many people(?:\s+(?:(?:are|is|were|was|there|here|now|altogether|total|in all|at home|it|that|again|then|did i say|did we say|do you remember|remind me|in the house|in the flat|in the home|in my (?:house|flat|home|household)|in our (?:house|flat|home|household)|live here|living here|live in the (?:house|flat|home)|living in the (?:house|flat|home))|(?:is it|was it|were they|is that|was that|again|then|did i say|did we say|i say)))*\s*[?.!]*$|\bhow (?:big|large) (?:is|are) (?:my|our|the) (?:family|household)\b"#
);
pattern!(
    GROUP_ASK_YOU,
    r#"(?i)\bhow many of (?:you|u)(?:\s+(?:(?:are|is|were|was|there|here|now|altogether|total|in all|at home|it|that|again|then|did i say|did we say|do you remember|remind me|in the house|in the flat|in the home|in my (?:house|flat|home|household)|in our (?:house|flat|home|household)|live here|living here|live in the (?:house|flat|home)|living in the (?:house|flat|home))|(?:is it|was it|were they|is that|was that|again|then|did i say|did we say|i say)))*\s*[?.!]*$|\bhow many of (?:you|u) (?:live|living|are living)(?:\s+(?:there|here|at home|now|altogether))?\s*[?.!]*$"#
);
pattern!(
    BARE_GROUP_ASK,
    r#"(?i)\bhow many (?:are|is|were|was) (?:coming|going|expected|there)(?:\s+(?:then|now|again|altogether|in total))?\s*[?.!]*$"#
);
pattern!(
    BARE_COUNT_ASK,
    r#"(?i)\bhow many\s+(?:was|is|were|are)\s+(?:it|that|there)(?:\s+(?:again|then|now|altogether|in total))?\s*[?.!]*$|\bhow many again\b\s*[?.!]*$|\A\W*how many\W*[?.!]*$"#
);
pattern!(
    PARTY,
    r#"(?i)\b(?:for|having|expecting|feeding|cooking for|catering for|a table for|table for|booking for|booked for|party for|dinner for|lunch for|tea for|supper for|seats? for|invited|inviting)\s+(?:the\s+|just\s+|only\s+)?(?P<n>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+(?:people|persons?|guests?|adults?|heads?|folk|friends|kids|children)\b|\b(?P<n2>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+(?:people|persons?|guests?|adults?|heads?|friends)\s+(?:coming|over|round|for|at|on)\b|\b(?P<n4>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+of us\s+(?:(?:coming|over|round)\b|for\s+(?!(?:a|an|the|good|now|once|ages|keeps|ever)\b)[a-z])|\b(?P<n7>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+of us\s+(?:on|this|next)\s+(?:the\s+)?(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|tues|tue|weds|wed|thurs|thur|thu|fri|sat|sun|weekend|tonight|tomorrow|today)\b|\b(?:a table for|table for|booking for|booked for|reservation for)\s+(?P<n3>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\b(?!(?:\s*(?:minutes?|hours?|days?|weeks?|months?|years?|courses?)\b))|\b(?:cooking for|catering for|feeding|having over|having round)\s+(?P<n6>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\b(?!(?:\s*(?:minutes?|hours?|days?|weeks?|months?|years?|courses?|of them)\b))|\b(?:roast|dinner|supper|tea|lunch|breakfast|meal|barbecue|bbq|barbeque|party|spread|buffet|feast|brunch|carvery)\b[^.?!]{0,24}?\bfor\s+(?:the\s+|just\s+|only\s+)?(?P<n5>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2}|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|a hundred|one hundred|\d{1,3})\b(?!(?:\s*(?:minutes?|hours?|days?|weeks?|months?|years?|courses?|more|less|pounds?|quid|euros?|dollars?|kg|kilos?|grams?|litres?|pints?|miles?|metres?|o'clock)\b))"#
);
pattern!(
    PARTY_ASK,
    r#"(?i)\bhow many (?:people|persons?|guests?|heads?|adults?)\b\s*(?:$|[,.!?;:]|\b(?:again|then|though|is it|is that|are coming|coming|was it|was that|you think|do you think|do i need|do i cook|am i cooking|we having|did i say|did we say)\b)|\bhow many (?:are|is) (?:coming|it for|we having|we cooking for|we expecting)\b|\bhow many did i say\b|\bhow many (?:did|have) (?:i|we) (?:book|booked|reserve|reserved|ask for|asked for)\b|\bhow many is (?:the|that|this|my|our) (?:table|booking|reservation) for\b|\bhow many (?:am|are|was|were) (?:i|we) (?:cooking|baking|making|catering|feeding|preparing|planning|doing)\b[^.?!]{0,18}?\bfor\b|\bhow many (?:do|did|will) (?:i|we) (?:cook|bake|make|cater|feed|prepare|plan) for\b"#
);
pattern!(
    CHANGED,
    r#"(?i)\b(?:actually|instead|now|sorry|hang on|one more|another|extra|make that|no)\b"#
);
pattern!(
    CHANGED_BARE,
    r#"(?i)\b(?:make it|make that|put it at|change it to|change to|actually|sorry|no|instead|lets say|let's say|now)\b[\s,]*(?:it\s+)?(?:to\s+|at\s+)?(?P<n>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\b(?:\s*,?\s*(?:people|persons?|guests?|heads?|adults?|of us|now|instead|please|thanks|after all))*\s*[.!]*$"#
);
pattern!(
    NOW_THE_NUMBER,
    r#"(?i)\b(?:we|there)\s*(?:'s|'re|re|are|is|s)?\s+(?:now\s+)?(?P<n>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+(?:of us\s+)?now\b"#
);
pattern!(
    ONE_MORE,
    r#"(?i)^\W*(?:and\s+|oh\s+|plus\s+)?(?:one|1)\s+more(?:\s+(?:is\s+coming|is\s+coming\s+now|coming|now|person|guest|guest\s+is\s+coming|of\s+us|of\s+them))?\s*[.!]*$"#
);
pattern!(
    ONE_LESS,
    r#"(?i)^\W*(?:and\s+|oh\s+)?(?:one|1)(?:\s+of\s+(?:them|the\s+guests?|the\s+group))?\s+(?:less|cant\s+come|cannot\s+come|can't\s+come|isnt\s+coming|isn't\s+coming|is\s+not\s+coming|has\s+dropped\s+out|dropped\s+out|cant\s+make\s+it|can't\s+make\s+it)(?:\s+(?:now|after\s+all|anymore|any\s+more))?\s*[.!]*$"#
);
pattern!(
    ABOUT_THE_HOUSE,
    r#"(?i)\b(?:at home|in the house|in the flat|in the home|live|lives|living|living here|household|family|house)\b"#
);
pattern!(
    POINTS_AT_A_NUMBER,
    r#"(?i)\b(?:again|was it|was that|is it for|did i say|did we say|do you remember|remind me)\b"#
);
pattern!(
    BARE_PEOPLE,
    r#"(?i)\b(?:(?:about|around|roughly|maybe|probably|just|only|some|like)\s+)?(?P<n>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2}|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|a hundred|one hundred|\d{1,3})\s+(?:people|persons?|guests?|adults?|heads?|folk)\b"#
);
pattern!(
    A_MEAL_IN_THE_CHAT,
    r#"(?i)\b(?:roast|dinner|supper|tea|lunch|breakfast|meal|barbecue|bbq|barbeque|party|spread|buffet|feast|brunch|carvery|cake|cakes|bake|baking|cooking|cook|menu|scones?|birthday|buffet|canapes|finger food)\b"#
);
pattern!(
    A_TABLE_BOOKED,
    r#"(?i)\b(?:book(?:ed|ing|ings)?|reserv(?:ed|ing|ation|ations))\b[^.]{0,24}\btables?\b|\btables?\b[^.]{0,12}\bfor\s+(?:\d+|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b|\b(?:book(?:ed|ing)?|reserved)\b\s+(?:for\s+)?(?:\d+|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b"#
);
pattern!(
    A_TABLE_CUE,
    r#"(?i)\b(?:people|persons?|guests?|adults?|heads?|folk|friends|kids|children)\b[^.]{0,16}\b(?:coming|over|round)\b|\b(?:for|having|expecting|feeding|catering for|cooking for|invited|inviting)\s+(?:the\s+|just\s+|only\s+)?(?:a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\b"#
);
pattern!(
    NOT_THE_COUNT,
    r#"(?i)\b(?:should|could|would|might|may|can|will)\b"#
);
pub(super) const LOOK_BACK_FOR_A_MEAL: usize = 6;
pub(super) const NUMBER_MAP: &[(&str, u16)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("ten", 10),
    ("a couple of", 2),
    ("a pair of", 2),
    ("couple of", 2),
    ("eleven", 11),
    ("twelve", 12),
    ("thirteen", 13),
    ("fourteen", 14),
    ("fifteen", 15),
    ("sixteen", 16),
    ("seventeen", 17),
    ("eighteen", 18),
    ("nineteen", 19),
    ("twenty", 20),
    ("thirty", 30),
    ("forty", 40),
    ("fifty", 50),
    ("sixty", 60),
    ("seventy", 70),
    ("eighty", 80),
    ("ninety", 90),
    ("a hundred", 100),
    ("one hundred", 100),
    ("a dozen", 12),
    ("dozen", 12),
];
pub(super) const PET_NUMBERS: usize = 13;
