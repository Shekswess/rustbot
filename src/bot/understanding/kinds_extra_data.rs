// Patterns and vocabulary from PyBot’s dialogue shape readers.

pattern!(
    FRAME_NOT_A_SETUP,
    r#"^(?: (?:and |but |oh |well |so |ok |okay |right |no )?(?:never mind|nevermind|i mean|i meant|you know what i mean|i dunno|dunno|not sure|no idea|honestly|sorry|one more thing|one thing|another thing|quick one|back to \w+(?: \w+){0,2}|of course|fair enough|me too|same here|no worries|anyway|just wondering|just asking|quick question|random question|random one|hi|hey|hello|hiya|good question)\b(?: .{0,12})?)+$"#
);
pattern!(
    A_FRAME_IN_FRONT,
    r#"(?i)^(?:(?:hi|hiya|hey|hello|morning|evening|ok|okay|right|well|also|i mean|i meant|honestly|sorry|cheers|thanks|thank you|no worries|quick one|quick question|random question|random one|one more thing|another thing|just wondering|just asking|lol|haha|lmao|anyway|bit of a random one|bit random|random one|and|but|so)(?:[\s,.-]+|$))+"#
);
pattern!(
    INTEREST_HEAD,
    r#"(?i)^\W*(?:i\s*m|im|i am|i\s*ve|ive|i have|we\s*re|were|we are|i\s+got|we\s+got)\s+(?:really\s+|so\s+|very\s+much\s+|very\s+|well\s+|quite\s+|now\s+|also\s+|totally\s+|still\s+)?(?:(?:getting|got|started)\s+(?:back\s+)?|back\s+)?(?:into|in to)\s+(?P<thing>[a-z' -]+?)\s*[.!]*$"#
);
pattern!(
    INTEREST_TAIL,
    r#"(?i)\s+(?:at the moment|right now|now|these days|lately|recently|a lot|lots|again|too|as well|also|on\s+(?:mon|tues|wednes|thurs|fri|satur|sun)days?|every\s+(?:day|week|weekend|morning|evening|night|mon|tues|wednes|thurs|fri|satur|sun)days?|at\s+the\s+weekend|after\s+work|in\s+the\s+evenings?)$"#
);
pattern!(
    STARTED_OUT,
    r#"(?i)\b(?:got|getting|started)\s+(?:back\s+)?into\b"#
);
pattern!(BACK_INTO, r#"(?i)\bback\s+into\b"#);
pattern!(
    ROUTINE_SHAPE,
    r#"(?i)\b(?:every (?:day|week|month|morning|evening|afternoon|night|weekend|other day)|every (?:mon|tues|wednes|thurs|fri|satur|sun)day|each (?:day|week|morning|evening)|most (?:days|weeks|mornings|evenings)|(?:once|twice|three times|four times|a few times) a (?:day|week|month)|on (?:a|an) (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|at the weekend)\b"#
);
pattern!(
    ROUTINE_IS_A_FAULT,
    r#"(?i)\b(?:pain|pains|\w*ache|\w*aches|aching|hurts?|hurting|sore|sick|ill|unwell|worse|worst|worried|anxious|panic|cry|crying|cried|shout|shouting|shouted|argue|arguing|fight|fighting|bleed|bleeding|blood|swollen|rash|fever|cough|coughing|vomit|vomiting|dizzy|numb|sneeze|sneezing|stopped|stopping|wont stop|won't stop|leak|leaking|broken|broke|craving|crave)\b"#
);
pattern!(
    ROUTINE_IN_THE_PAST,
    r#"(?i)\b(?:used to|didn'?t use to|would (?:always |often )?\w+|back then|years ago|in those days)\b"#
);
pattern!(
    ROUTINE_SUBJECT,
    r#"(?i)^\s*(?:and\s+|so\s+|well\s+|oh\s+)?(?:i|i(?:'m|ve|d)|we|we(?:'re|ve))\b"#
);
pattern!(
    SHIFT_EARLY,
    r#"(?i)\b(?:(?:i|we|im|i'm|we're|were)\s+)?(?:am|are|'m|'re|was|were)?\s*(?:on|back on|working|work|doing|do)\s+(?:the\s+)?(?:earlies|early shifts?)\b|\bon (?:the )?earlies\b|\bearlies (?:all|this|next|every|for)\b|\bearly shifts?\b"#
);
pattern!(
    SHIFT_LATE,
    r#"(?i)\b(?:(?:i|we|im|i'm|we're|were)\s+)?(?:am|are|'m|'re|was|were)?\s*(?:on|back on|working|work|doing|do)\s+(?:the\s+)?(?:lates|late shifts?)\b|\bon (?:the )?lates\b|\blates (?:all|this|next|every|for)\b|\blate shifts?\b"#
);
pattern!(
    SHIFT_NIGHT,
    r#"(?i)\b(?:night shifts?|on nights|work(?:ing)? nights|nights this week)\b"#
);
pattern!(
    CARE_HOME_PLACE,
    r#"(?i)\b(?:in|into|at)\s+(?:a|the)\s+(?:care\s+|nursing\s+|residential\s+)?home\b"#
);
pattern!(
    CARE_HOME_NOT,
    r#"(?i)\b(?:work|works|working|worked|stay|staying)\s+from\s+home\b|\bhome\s+(?:town|office|made|birth|baked)\b"#
);
pattern!(
    CARE_HOME_MOVED,
    r#"(?i)\b(?:moved|moves|moving|went|goes|gone|going|put|puts|putting|decided|decision|agreed|arranged|sorted)\b"#
);
pattern!(
    CARE_HOME_VISIT,
    r#"(?i)\b(?:visit|visits|visiting|visited|see|seeing|saw|bring|brings|bringing|brought|take her|take him|take them)\b"#
);
pattern!(
    CARE_HOME_GUILT,
    r#"(?i)\b(?:guilt|guilty|selfish|should have|shouldve|my fault|blame|couldnt cope|could not cope|no choice|heartbroken|awful)\b"#
);
pattern!(
    CARE_HOME_BRING_ASKS,
    r#"(?i)\b(?:what|anything|something|whats|should i|shall i|can i|could i|do i|did i|is there anything)\b"#
);
pattern!(
    CARE_HOME_BRING_WORD,
    r#"(?i)\b(?:bring|brings|bringing|take|takes|taking)\b"#
);
pattern!(
    CARE_HOME_BRING_WHO,
    r#"(?i)\b(?:her|him|them|she|he|they)\b"#
);
pattern!(
    OPENING_FRAME,
    r#"(?x)
    \b(?:can|could|may|might)\s+i\s+ask\s+(?:you\s+)?(?:something|somethin|a\s+question|one\s+thing|advice|for\s+advice|your\s+advice)\b
  | \b(?:i|ive)?\s*(?:have|got|has)\s+(?:a\s+|one\s+|quick\s+)?question\b
  | \bquick\s+question\b
  | \b(?:i|we)\s+(?:need|want|would\s+like|could\s+use)\s+(?:(?:some|a\s+bit\s+of)\s+)?(?:your\s+)?advice\b
  | \b(?:can|could|would|will)\s+you\s+help\s+me\s+(?:with\s+something|with\s+a\s+thing|with\s+one\s+thing|out\s+with\s+something)\b
  | \bmind\s+if\s+i\s+ask\s+(?:you\s+)?(?:something|a\s+question)\b
  | \bi\s+(?:want|need)\s+to\s+ask\s+you\s+(?:something|a\s+question)\b
"#
);
pattern!(
    TALK_OPENING,
    r#"\A(?:(?:i\s+|we\s+)?(?:just\s+|really\s+|honestly\s+|properly\s+)?(?:am\s+|are\s+)?(?:(?:want|wanna|need|have|got|gotta|planning|going|gonna)\s+)?(?:to\s+)?(?:vent|rant|moan|whinge|grumble|offload|unload)(?:\s+(?:a\s+bit|a\s+minute|a\s+sec|a\s+moment|one\s+thing|for\s+a\s+minute|for\s+a\s+bit|about\s+something|about\s+it|about\s+this|to\s+you|to\s+someone|on\s+you|here|first))*|(?:can|could|may)\s+(?:i|we)\s+(?:just\s+)?(?:have\s+a\s+)?(?:vent|rant|moan|whinge|grumble|offload|unload)(?:\s+(?:a\s+bit|a\s+minute|a\s+sec|a\s+moment|one\s+thing|for\s+a\s+minute|for\s+a\s+bit|about\s+something|about\s+it|about\s+this|to\s+you|to\s+someone|on\s+you|here|first))*|(?:i\s+|we\s+)?(?:just\s+)?(?:am\s+|are\s+)?(?:going\s+to\s+|gonna\s+)?(?:(?:want|wanna|need|have|got|gotta)\s+(?:to\s+)?)?(?:have\s+)?a\s+(?:proper\s+|good\s+|quick\s+|little\s+)?(?:vent|rant|moan|whinge|grumble)(?:\s+(?:a\s+bit|a\s+minute|a\s+sec|a\s+moment|one\s+thing|for\s+a\s+minute|for\s+a\s+bit|about\s+something|about\s+it|about\s+this|to\s+you|to\s+someone|on\s+you|here|first))*|(?:i\s+)?(?:just\s+)?(?:(?:want|wanna|need|have|got|gotta)\s+(?:to\s+)?|(?:can|could|may)\s+i\s+|let\s+me\s+)(?:get\s+)?(?:something|somethin|this|it)\s+off\s+my\s+chest|(?:i\s+|we\s+)?(?:just\s+|really\s+)?(?:am\s+|are\s+)?(?:(?:want|wanna|need|have|got|gotta|going)\s+(?:to\s+)?|(?:can|could|may)\s+(?:i|we)\s+)talk(?:\s+(?:a\s+bit|a\s+minute|a\s+sec|a\s+moment|one\s+thing|for\s+a\s+minute|for\s+a\s+bit|about\s+something|about\s+it|about\s+this|to\s+you|to\s+someone|on\s+you|here|first))*|(?:i\s+)?(?:need|want|would\s+like)\s+(?:someone|somebody|anyone|anybody)\s+to\s+talk\s+to|(?:can|could|may)\s+i\s+(?:have\s+a\s+word)|(?:i\s+)?(?:need|want)\s+a\s+word|(?:(?:can|could|may)\s+(?:i|we)|do\s+you\s+mind\s+if\s+i|would\s+you\s+mind\s+if\s+i|is\s+it\s+ok\s+if\s+i|is\s+it\s+okay\s+if\s+i)\s+(?:run|bounce|try|talk\s+through)\s+(?:something|somethin|this|that|it|an?\s+idea|one\s+thing|a\s+couple\s+of\s+things|a\s+few\s+things|some\s+ideas?)\s+(?:by|past|off)\s+you(?:\s+(?:a\s+bit|a\s+minute|a\s+sec|a\s+moment|one\s+thing|for\s+a\s+minute|for\s+a\s+bit|about\s+something|about\s+it|about\s+this|to\s+you|to\s+someone|on\s+you|here|first))*|(?:(?:can|could|may)\s+(?:i|we)|do\s+you\s+mind\s+if\s+i|would\s+you\s+mind\s+if\s+i|is\s+it\s+ok\s+if\s+i|is\s+it\s+okay\s+if\s+i)\s+(?:pick\s+your\s+brains?|bend\s+your\s+ear)(?:\s+(?:a\s+bit|a\s+minute|a\s+sec|a\s+moment|one\s+thing|for\s+a\s+minute|for\s+a\s+bit|about\s+something|about\s+it|about\s+this|to\s+you|to\s+someone|on\s+you|here|first))*|(?:(?:can|could|may)\s+(?:i|we)|do\s+you\s+mind\s+if\s+i|would\s+you\s+mind\s+if\s+i|is\s+it\s+ok\s+if\s+i|is\s+it\s+okay\s+if\s+i)\s+(?:tell\s+you\s+what\s+happened|share\s+(?:something|this|it)\s+with\s+you|have\s+a\s+second\s+opinion|get\s+a\s+second\s+opinion)(?:\s+(?:a\s+bit|a\s+minute|a\s+sec|a\s+moment|one\s+thing|for\s+a\s+minute|for\s+a\s+bit|about\s+something|about\s+it|about\s+this|to\s+you|to\s+someone|on\s+you|here|first))*)\z"#
);
pub(super) const TAIL_ASKS: &[&str] = &[
    "advice",
    "help",
    "idea",
    "ideas",
    "pointers",
    "recommendation",
    "recommendations",
    "suggestion",
    "suggestions",
    "thought",
    "thoughts",
    "tip",
    "tips",
];
pub(super) const TAIL_ASK_FILLER: &[&str] =
    &["a", "any", "few", "more", "please", "quick", "some", "your"];
pub(super) const SETUP_NOTHING: &[&str] = &[
    "ah",
    "alright",
    "and",
    "anyway",
    "but",
    "cheers",
    "evening",
    "goodnight",
    "haha",
    "hello",
    "hey",
    "hi",
    "hmm",
    "lmao",
    "lol",
    "morning",
    "no",
    "oh",
    "ok",
    "okay",
    "right",
    "so",
    "ta",
    "thank",
    "thanks",
    "um",
    "well",
    "yeah",
    "yes",
    "you",
];
pub(super) const SETUP_CONNECTIVES: &[&str] = &[
    "also", "although", "and", "as", "at", "because", "both", "but", "by", "cause", "cos",
    "either", "for", "from", "here", "if", "in", "it", "its", "me", "neither", "nor", "of", "on",
    "or", "plus", "same", "since", "so", "that", "then", "there", "these", "this", "those",
    "though", "to", "too", "when", "while", "with",
];
pub(super) const INTEREST_NOT: &[&str] = &[
    "a",
    "an",
    "any",
    "argument",
    "arguments",
    "bed",
    "debt",
    "debts",
    "fight",
    "fights",
    "her",
    "him",
    "his",
    "hospital",
    "it",
    "me",
    "my",
    "not",
    "our",
    "prison",
    "row",
    "rows",
    "some",
    "that",
    "the",
    "their",
    "them",
    "these",
    "this",
    "those",
    "trouble",
    "us",
    "you",
    "your",
];
pub(super) const CARE_HOME_KIN: &[&str] = &[
    "brother-in-law",
    "daughter-in-law",
    "father-in-law",
    "grandaddy",
    "grannie",
    "mam",
    "mammy",
    "mate",
    "mates",
    "mother-in-law",
    "nan",
    "nana",
    "sister-in-law",
    "son-in-law",
];
pub(super) const OPENING_WORDS: &[&str] = &[
    "a",
    "advice",
    "ask",
    "bit",
    "can",
    "could",
    "for",
    "got",
    "has",
    "have",
    "help",
    "i",
    "if",
    "is",
    "it",
    "ive",
    "like",
    "may",
    "me",
    "might",
    "mind",
    "need",
    "of",
    "one",
    "out",
    "question",
    "quick",
    "some",
    "somethin",
    "something",
    "thing",
    "to",
    "use",
    "want",
    "we",
    "will",
    "with",
    "would",
    "you",
    "your",
];
pub(super) const TALK_HEAD: &[&str] = &[
    "alright", "honestly", "listen", "look", "now", "ok", "okay", "right", "so", "then", "well",
];
pub(super) const TALK_TAIL: &[&str] = &[
    "to be honest",
    "if that is ok",
    "if thats ok",
    "now",
    "please",
    "then",
    "thanks",
    "right",
    "actually",
    "quickly",
    "quick",
    "mate",
];
