// Moving away, as read by the pinned fallback composer.
pattern!(
    LEAVING_A_PERSON,
    r###"(?i)\b(?:my|our)\s+(?:son|daughter|boy|girl|kid|kids|children|child|eldest|youngest|stepson|stepdaughter|mum|mom|mother|dad|father|parents|sister|brother|best\s+friend|friend|mate|neighbour|neighbor|grandson|granddaughter|grandchild|nan|gran|aunt|uncle|cousin|husband|wife|partner|boyfriend|girlfriend|niece|nephew)\b|\b(?:he|she|they|the\s+kids|the\s+children|everyone)\b"###
);
pattern!(
    LEFT_BEHIND,
    r###"(?i)\b(?:left|leaving|leaves|moved|moving)\b[^.?!,;]{0,24}\bhome\b|\b(?:left|leaving|leaves|went|going|gone|goes|go|off|moved|moving|heading|headed)\b[^.?!,;]{0,24}\b(?:away|abroad|overseas|out of the house)\b|\b(?:moving|moved|relocat\w+|emigrat\w+)\s+(?:away|abroad|overseas)\b|\b(?:went|going|gone|goes|go|off|left|leaving|leaves|starts?|starting|started)\s+to\s+(?:uni|university|college)\b|\b(?:starts?|starting|started)\s+(?:uni|university|college)\b|\b(?:left|leaves|leaving|went|going|gone|goes|off|off\s+to)\s+(?:for\s+)?(?:uni|university|college)\b|\b(?:moved|moving|moves|relocated|relocating|relocates|emigrated|emigrating|emigrates)\s+to\s+(?!a\b|an\b|the\b|another\b|his\b|her\b|their\b|our\b|my\b|that\b|this\b)[a-z]|\bempty\s+nest\b|\b(?:fly|flying|flies|flew|flown)\s+(?:the|their)\s+nest\b"###
);
pattern!(
    NOT_LEAVING_ANYONE,
    r###"(?i)\bhow\s+(?:do|can|should|could|would|might|shall)\s+i\b|\bhow\s+to\b|\bwhat\s+should\s+i\b|\bany\s+(?:tips|advice|ideas)\b|\bany\s+idea\b|\bwhat\s+do\s+you\s+think\b|\bwhat\s+would\s+you\b|\bmoving\s+in\s+with\b|\b(?:coming|moved|moving)\s+(?:in|back)\b|\bstaying\s+with\s+(?:us|me)\b|\bstay(?:ing|s|ed)?\s+with\b|\b(?:starts?|starting|started)\s+(?:school|nursery|reception|primary|year\s+one)\b"###
);
pattern!(
    A_CHILD,
    r###"(?i)\b(?:my|our)\s+(?:son|daughter|boy|girl|kid|kids|children|child|eldest|youngest|stepson|stepdaughter)\b|\bthe\s+(?:kids|children|eldest|youngest)\b"###
);
pattern!(
    THE_LEAVE_HAS_HAPPENED,
    r###"(?i)\b(?:left|moved|went|gone|flew|flown|relocated|emigrated)\b|\b(?:has|have|had|she's|hes|he's|they've|theys|we've|was|were)\b[^.,;!?]{0,16}\b(?:left|moved|gone|flew|flown|relocated|emigrated)\b"###
);
pattern!(
    A_FEELING_IN_THE_LINE,
    r###"(?i)\b(?:hurts?|hurting|stings?|stung|(?:will|do|would|could)\s+miss|miss(?:es|ed)\b(?!\s+\?)|dreading|sad|upset|gutted|lonely|ach(?:e|es|ing|y)|cry(?:ing)?|broken|struggling|hate|worried|devastated|hard|tough|quiet|empty)\b"###
);
const A_CHILD_LEFT: &[&str] = &[
    "The room being empty is the hard bit. How long has it been quiet?",
    "How far away are they now? That usually decides how often you actually see each other.",
    "Proud and a bit hollow at once, and both are allowed. What do you miss most?",
    "Does the house feel different, or is it the evenings that are strange?",
];
const SOMEONE_GONE: &[&str] = &[
    "That leaves a gap in the week, not just the town. What do you miss most?",
    "How far away are they now, and are you a phone-call pair or a message-one?",
    "Nobody did anything wrong and it still stings — that is how a move lands. Have you been out to see them yet?",
    "The first weeks are the odd ones. Does it feel more like a visit when they come back?",
];
const A_CHILD_LEAVING: &[&str] = &[
    "The house is about to get quieter. Which part of it are you dreading?",
    "Proud and a bit hollow at the same time, I'd guess. Has it been on the cards long?",
    "So there's going to be an empty room in your house. How are you feeling about it?",
    "How far away is it going to be? That usually decides how it feels.",
    "Are you doing something together before they go? Those last meals tend to be how it gets marked.",
];
const A_CHILD_LEAVING_PLAIN: &[&str] = &[
    "The house is about to get quieter. Which part of it are you dreading?",
    "Proud and a bit hollow at the same time, I'd guess. Has it been on the cards long?",
    "So there's going to be an empty room in your house. How are you feeling about it?",
    "How far away is it going to be? That usually decides how it feels.",
    "Are you doing something together before they go? Those last meals tend to be how it gets marked.",
];
const SOMEONE_LEAVING: &[&str] = &[
    "That's a real change to your week. What will you miss most?",
    "How far is it, and are you a phone-call pair or a message-one?",
    "Are you getting one more evening together before they go?",
    "Nobody has done anything wrong and it still hurts — that is how it goes when someone moves away. Will you get to visit?",
    "Somebody going leaves a hole in the week, not just the town. Do you keep in touch much as it is?",
];
const SOMEONE_LEAVING_PLAIN: &[&str] = &[
    "That's a real change to your week. What will you miss most?",
    "How far is it, and are you a phone-call pair or a message-one?",
    "Are you getting one more evening together before they go?",
    "Somebody going leaves a hole in the week, not just the town. Do you keep in touch much as it is?",
];
