// Rule and wording data from the pinned source.

pattern!(
    WHO_LIVES,
    r#"(?i)\bwho (?:else )?(?:lives|live|living|is living) (?:with (?:you|u|us|me)|there|here|at home)\b|\bwho (?:do|does) (?:you|u) live with\b|\bwho am i living with\b|\bwho (?:do|did) i live with\b|\banyone (?:else )?(?:live|living|lives) (?:with (?:you|u|us|me)|there|here|at home)\b|\banybody (?:else )?(?:live|living|lives) (?:with (?:you|u|us|me)|there|here)\b|\bdoes anyone (?:else )?live with (?:you|u)\b|\bwho(?:'s| is| s) (?:at home|there with you|living there)\b|\bwho else is (?:at home|there)\b|\bhow many of (?:you|us) (?:are|is) there\b"#
);
pattern!(
    JUST_ME_AND,
    r#"(?i)^(?:well |ok |okay |so |and )*(?:its |it's |it is )?(?:just |only )?(?:me|us) (?:and|&) (?:my |our )?(?P<who>[a-z]+(?: [a-z]+)?)(?: and (?:my |our )?(?P<more>[a-z]+(?: [a-z]+)?))?(?: at home| living here| here)?$"#
);
pattern!(
    JUST_ME,
    r#"(?i)^(?:well |ok |okay |so |and )*(?:its |it's |it is )?(?:just |only )?(?:me|the two of us|the three of us|us)(?: at home| here| on my own)?$"#
);
pattern!(
    ON_MY_OWN,
    r#"(?i)\bi (?:live|am living|'m living|m living) (?:on my own|by myself|alone)\b|\bi (?:live|'m|am) (?:on my own|by myself)\b|\bliving alone\b"#
);
pattern!(
    SHOULD_I_MORE,
    r#"(?i)\bshould i\s+(?:\w+\s+){0,2}(?:more|more often|again|every day|every week)\b"#
);
pattern!(
    YES_AND_NO,
    r#"(?i)(?:yes|yeah|yep|yup|she does|he does|they do|she lives alone|he lives alone|no|nope|not really|she doesnt|she doesn't|he doesnt|he doesn't|they dont|they don't|she isnt|she isn't|he isnt|he isn't|they arent|they aren't)\b"#
);
pattern!(
    THEIR_HOUSE_ASK,
    r#"(?i)\b(?:does|do|is|are)\s+(?P<who>she|he|they|your\s+[a-z]+)(?:\s+still)?\s+(?:live|living|be\s+living)\b[^?]*?\b(?:alone|on\s+(?:her|his|their)\s+own|by\s+(?:her|him|their)self)\b|\b(?:is|are)\s+(?P<who2>she|he|they|your\s+[a-z]+)(?:\s+still)?\s+(?:on\s+(?:her|his|their)\s+own|alone|by\s+(?:her|him|their)self)\b"#
);
pattern!(
    THEIR_ALONE_SAID,
    r#"(?i)\bmy (?P<who>mum|mom|mam|dad|father|mother|gran|granny|nan|nana|grandma|grandad|grandpa|sister|brother|aunt|uncle|cousin|friend|neighbour|neighbor|wife|husband|partner)\s+(?:lives|living|is living|'s living|s living)\s+(?:on (?:her|his|their) own|alone|by (?:her|him|their)self)\b"#
);
pattern!(
    THEIR_HOUSE_WHO,
    r#"(?i)\bwho (?:does|do) (?P<who>she|he|they|your\s+[a-z]+)(?:\s+actually)?\s+live with\b|\bwho (?:is|'s|s) (?P<who2>she|he|they|your\s+[a-z]+) living with\b|\b(?:does|do) (?:anyone|anybody|someone|somebody)(?:\s+else)?\s+live with (?P<who3>her|him|them)\b"#
);
pub(super) const WITH_SOMEONE: &[&str] = &[
    "As far as you've told me, it's you and {who}.",
    "You and {who} — that's who you've mentioned.",
    "Just you and {who}, the way you told me.",
];
pub(super) const ALONE: &[&str] = &[
    "On your own, you told me.",
    "Just you, then — I'm glad of the company.",
    "You live on your own, as far as I know.",
];
pub(super) const NOT_SAID: &[&str] = &[
    "You haven't said who lives with you — who's there?",
    "I don't know that yet. Who do you live with?",
];
pub(super) const THEIR_HOUSE: &[&str] = &[
    "Just you and {who}, then — noted.",
    "You and {who}, then. What's an average evening like there?",
    "{Who} and you, then — that's the household.",
];
pub(super) const THEIRS_NOT_SAID: &[&str] = &[
    "I can't tell from here — but if {who} is living alone, a standing call or visit beats a one-off grand gesture.",
    "You'd know better than me. If {who} is living alone and it's getting harder, the council's free needs assessment is the first thing to ask about.",
    "I don't know that one. If you're worried about {who} living alone, it's the small regular contact that helps most.",
];
pub(super) const THEIRS_NOT_SAID_WHO: &[&str] = &[
    "I don't know — you haven't told me who lives with {who}.",
    "Not something you've said. If it's company for {who} you're wondering about, a weekly class or club is the usual way in.",
];
pub(super) const SHOULD_MORE: &[&str] = &[
    "Little and often beats long and rare: a short call on the same day each week is easier to keep than a long one whenever there is time, and it is the one that gets counted on.",
    "Yes — the regular short one does more than the long catch-up. Put it in the diary for the same day each week and let it be dull; dull is what makes it last.",
];
pub(super) const THEN_ALONE: &[&str] = &[
    "Then a standing time to ring {who} is worth more than a long catch-up now and then — the same day each week, ten minutes, and it becomes a fixture rather than a good intention.",
    "That is the hard part, the ordinary week on their own. A short call at the same time each week does more than a long one whenever it can be fitted in.",
];
pub(super) const THEN_NOT_ALONE: &[&str] = &[
    "That is good — then it is the ordinary days that want the visits rather than the occasions, and a standing one is the easiest kind to keep.",
    "Good. Then what helps most is turning up on a dull Tuesday rather than a birthday — the ordinary days are the ones that get long.",
];
pub(super) const THEIRS_WITH: &[&str] = &[
    "You said so yourself — someone lives with {who}.",
    "You told me — {who} has company.",
];
pub(super) const THEIRS_ALONE: &[&str] = &[
    "You told me — {who} lives alone.",
    "You said so yourself: {who} lives alone.",
];
pattern!(
    THE_ANSWER_ASKED,
    r#"(?i)\b(?:does|do|is|are) (?P<who>she|he|they|your\s+[a-z]+)(?:\s+still)?\s+(?:live|living|alone|on (?:her|his|their) own)\b"#
);
