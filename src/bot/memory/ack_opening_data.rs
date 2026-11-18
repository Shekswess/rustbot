//! Openings for facts from the pinned memory acknowledgement.

pattern!(
    A_DAY_FOR_SOMEONE,
    r####"(?i)(?:\b(?:turns?|is|'s|am|'m|are|'re|re) \d+\b|\b(?:im|i'm|i am) \d+\b|\b\d+ (?:next|this|in|on)\b|\bbirthday\b|\bbday\b)"####
);
pattern!(
    A_TIME,
    r####"(?i)\b(?:tonight|this evening|this morning|this afternoon|tomorrow|the day after tomorrow|next (?:week|month|year|weekend|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|this weekend|at the weekend|on monday|on tuesday|on wednesday|on thursday|on friday|on saturday|on sunday|in (?:january|february|march|april|may|june|july|august|september|october|november|december)|in the (?:morning|afternoon|evening)|later today|for the rest of the (?:day|week))\b"####
);
pattern!(
    A_STANDING_STATE,
    r####"(?i)\b(?:every (?:day|week|night|morning|month)|(?:twice|three times|four times|five times|six times) a (?:day|week)|\b(?:six|seven|five|four) days a week|live with|live(s)? with|living with|work(s)? (?:in|at|for|from|nights|shifts|weekends)|have (?:two|three|four|five) (?:kids|children|jobs))\b"####
);
pattern!(
    A_TASTE,
    r####"(?i)^(?:and |so |but )?(?P<who>i|he|she|they|we|my (?:[\w']+ )?[\w']+)\s+(?:really |just |absolutely |also |still )*(?:loves?|likes?|enjoys?|prefers?|(?:is|are) (?:really |very |well )?(?:into|a (?:big )?fan of))\b"####
);
pattern!(
    A_TASTE_OF_A_PERSON,
    r####"(?i)\b(?:you|me|him|her|them|us)\s*[.!]*\s*$"####
);
pattern!(
    A_CRUSH,
    r####"(?i)\b(?:a (?:girl|boy|man|woman|lady|lad)|someone|somebody|anyone)\b"####
);
pattern!(
    A_PERSON_AS_OBJECT,
    r####"(?i)\b(?:mum|mom|mother|dad|father|son|daughter|kids?|children|boys?|girls?|wife|husband|partner|brother|sister|nana|gran|grandma|grandad|granny|family|parents|baby|friend|mates?|neighbours?)\b"####
);
pattern!(
    A_START,
    r####"(?i)\b(?:starts?|starting|begins?|goes? back|going back|finish(?:es)?|leaves?|left) (?:year|school|college|uni|university|nursery|reception)\b|\b(?:has|sits?|taking|takes) (?:his|her|their|the) (?:gcses?|exams?|a levels?|mocks?)\b"####
);
pattern!(
    A_PAST,
    r####"(?i)\b(?:yesterday|last (?:night|week|weekend|month|year|summer|christmas|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|this (?:morning|afternoon)|earlier|ago|used to|was going to|were going to|had|saw|met|went|got married|moved|spent|asked|rang|kicked|took|cooked|bought|ate|paid|came|left|started|finished|arrived)\b"####
);
pattern!(
    MONEY,
    r####"(?i)\b(?:rent|savings|bill|bills|budget|wage|wages|salary|debt|loan|cost|price|payment|money|credit card|overdraft|mortgage)\b"####
);
pattern!(
    A_MEASURE,
    r####"(?i)\b(?:foot|feet|ft|cm|centimetres?|metres?|stone|kilos?|kilograms?|pounds?|inches|inch)\b"####
);
pattern!(A_MONTHS, r####"(?i)\b\d+ months?\b|\bmonths? old\b"####);
pattern!(
    NOT_AN_OCCASION,
    r####"(?i)\b(?:age|date of birth|dob|birthday|rent|savings)\b"####
);
pattern!(
    A_CORRECTION,
    r####"(?i)(?:^\s*(?:no|nope|sorry)\b|\b(?:i (?:said|meant|told you|didn't say|did not say))\b)"####
);

pub(super) const SHE: &[&str] = &[
    "daughter",
    "sister",
    "mum",
    "mother",
    "mam",
    "nan",
    "nana",
    "gran",
    "granny",
    "aunt",
    "auntie",
    "wife",
    "girlfriend",
    "partner",
    "niece",
    "cousin",
    "girl",
    "granddaughter",
    "stepmum",
    "stepdaughter",
    "mother in law",
    "daughter in law",
    "sister in law",
];

pub(super) const HE: &[&str] = &[
    "son",
    "brother",
    "dad",
    "father",
    "grandad",
    "granddad",
    "uncle",
    "husband",
    "boyfriend",
    "nephew",
    "cousin",
    "boy",
    "grandson",
    "stepdad",
    "stepson",
    "father in law",
    "son in law",
    "brother in law",
];

pub(super) const THEY: &[&str] = &[
    "kids",
    "children",
    "parents",
    "friends",
    "twins",
    "boys",
    "girls",
    "little one",
    "little ones",
    "toddler",
    "baby",
    "child",
];

pub(super) const A_DAY_TAILS: &[&str] = &[
    "{Thing} — what are you planning for it?",
    "{Thing}. What would {subj} like to do?",
    "{Thing} — is there anything {subj} {has} asked for?",
    "{Thing}. What {be} {subj} thinking of doing?",
];

pub(super) const A_LIKED_TAILS: &[&str] = &[
    "{Thing} — what got {them} into it?",
    "{Thing}. Is it a new thing, or {has} {subj} always been keen?",
    "{Thing} — what's the best bit of it for {them}?",
    "{Thing}. How long has that been going on?",
    "{Thing} — is that a recent thing, or a lifelong one?",
];

pub(super) const A_DOING_TAILS: &[&str] = &[
    "{Thing} — how {be} {subj} feeling about it?",
    "{Thing} — how {be} {subj} feeling about it?",
    "{Thing}. How's it going so far?",
    "{Thing}. How's it going so far?",
    "{Thing} — {be} {subj} looking forward to it?",
];

pub(super) const A_STATE_TAILS: &[&str] = &[
    "{Thing} — how's that working out?",
    "{Thing}. How's that treating you?",
    "{Thing} — what's that like?",
];
