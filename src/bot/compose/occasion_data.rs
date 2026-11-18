//! Occasion markers and greetings from the pinned composer.

pattern!(
    NOT_AN_OCCASION,
    r####"(?i)\byou\b|\byour\b|\bwould(?:'ve| have|ve)?\s+been\b|\b(?:what|which|when|whens|when's|how|where)\b|\b(?:present|presents|gift|gifts|idea|ideas|nothing|need|needs)\b|\b(?:was|were)\b"####
);
pattern!(
    LET_DOWN,
    r####"(?i)\b(?:forgot|forgets|forgetting|forgotten|fell\s+apart|cancell?ed|ruined|messed|mucked|hate|wrong|disaster|horrible|nobody|no\s?body|no\s?one|none\s+of\s+them|annoyed|annoying|upset|arguing|argued|row|fight|fighting|didn't|didnt|did not|won't|wont|worried|worry|scared|terrified|dreading|stressed|anxious)\b"####
);
pattern!(
    SOMEONE_LEFT,
    r####"(?i)\b(?:he|she|they)\s*(?:'s|'d|s|d)?\s+(?:has\s+|had\s+|just\s+|then\s+|already\s+)?(?:left|leaves|leaving|walked\s+out|walking\s+out|moved\s+out|dumped|split\s+up)\b"####
);
pattern!(
    BAD,
    r####"(?i)\b(?:lost|miscarried|miscarriage|died|death|stillborn|failed|failing|not going|didn't|didnt|did not|haven't|havent|hasn't|hasnt|not yet|never|gave up|turned down|rejected|but|except|however|fight|fights|trouble|accident|argument|row|debt|hospital|intensive care|poorly|poorly|not well)\b"####
);
pattern!(
    ORDINAL_DAY,
    r####"(?i)\b(?:my|our)\s+(?:(?P<whose>[\w'-]+)\s+)?(?P<day>\d+(?:st|nd|rd|th))\b(?:\s*,?\s*(?:is|was|are)?\s*(?:in|on|next|this|tomorrow|tonight|soon|coming|at\s+the\s+end|early|late|this\s+weekend|the\s+weekend)\b|[\s.!]*$)"####
);
pattern!(
    RELATION_BEFORE_DAY,
    r####"(?i)\b(?:my|our)\s+(?:(?P<whose>[\w'-]+(?:\s+[\w'-]+)?)\s+)?(?:\d+(?:st|nd|rd|th)\s+)?(?:birthday|anniversary)\b"####
);
pattern!(
    BIRTHDAY,
    r####"(?i)\b(?:its|it's|it is)\s+(?:\d+(?:st|nd|rd|th)\s+)?birthday\b|\b(?:my|our)\s+(?:[\w'-]+\s+)?(?:\d+(?:st|nd|rd|th)\s+)?birthday\b"####
);
pattern!(
    ANNIVERSARY,
    r####"(?i)\b(?:its|it's|it is|my|our)\s+(?:\d+(?:st|nd|rd|th)\s+)?(?:\w+\s+)?anniversary\b"####
);
pattern!(
    WEDDING,
    r####"(?i)\b(?:im|i'm|i am|we're|we are|we re)\s+(?:getting\s+)?married\b|\b(?:my|our)\s+wedding\b"####
);
pattern!(
    WISH,
    r####"(?i)^(?:a\s+|very\s+|so\s+|aw+\s+|oh\s+|many\s+)*(?:happy\s+(?:birthday|bday|b-day)|many\s+happy\s+returns)\s*[\s!.,]*$"####
);
pattern!(
    APART,
    r####"(?i)\b(?:away|apart|elsewhere|abroad|miles|far\s+away|over\s+there|back\s+home|visiting|visit|stay(?:ing)?\s+with|lives?\s+(?:in|near|abroad|away)|living\s+(?:in|near|abroad|away)|uni|university|college|the\s+other\s+side|different\s+(?:town|city|country)|in\s+(?:london|manchester|birmingham|leeds|glasgow|edinburgh|cardiff|belfast|dublin|paris|spain|france|italy|america|australia|canada|germany|portugal|greece|scotland|wales|ireland|england))\b"####
);
pattern!(
    PLACES_THEM,
    r####"(?i)\b(?:seeing\s+them|see\s+them|off\s+to\s+see|go\s+and\s+see|going\s+to\s+see|visit\s+them|visiting\s+them|go\s+over|going\s+over|drive\s+(?:up|down)|travell?ing\s+(?:to|up|down))\b"####
);
pattern!(
    HAPPENING_NOW,
    r####"\b(?:right\s+now|just\s+now|as\s+we\s+speak|this\s+minute|right\s+this\s+minute|happening\s+now|is\s+having\s+it|are\s+having\s+it|still\s+going\s+on|in\s+the\s+next\s+hour|on\s+my\s+way\s+to\s+the\s+hospital)\b"####
);
pattern!(
    DATE_GIVEN,
    r####"(?i)\b(?:in|on|next|this|at)\s+(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|sept|oct|nov|dec|week|weekend|month|year|spring|summer|autumn|winter|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\b"####
);

pub(super) const SHE: &[&str] = &[
    "aunt",
    "auntie",
    "daughter",
    "daughterinlaw",
    "fiancee",
    "girlfriend",
    "gran",
    "grandma",
    "grandmother",
    "grandmum",
    "granny",
    "missus",
    "mom",
    "mother",
    "motherinlaw",
    "mrs",
    "mum",
    "mummy",
    "nan",
    "nana",
    "niece",
    "sister",
    "sisterinlaw",
    "stepdaughter",
    "stepmother",
    "stepmum",
    "wife",
];

pub(super) const HE: &[&str] = &[
    "boyfriend",
    "brother",
    "brotherinlaw",
    "dad",
    "daddy",
    "father",
    "fatherinlaw",
    "fiance",
    "grandad",
    "granddad",
    "grandfather",
    "grandpa",
    "husband",
    "mr",
    "nephew",
    "son",
    "soninlaw",
    "stepdad",
    "stepfather",
    "stepson",
    "uncle",
];

pub(super) const FAMILY: &[&str] = &[
    "aunt",
    "baby",
    "boss",
    "boy",
    "boyfriend",
    "brother",
    "brotherinlaw",
    "carer",
    "child",
    "client",
    "colleague",
    "cousin",
    "coworker",
    "dad",
    "daughter",
    "ex",
    "father",
    "fatherinlaw",
    "fiance",
    "fiancee",
    "flatmate",
    "friend",
    "girl",
    "girlfriend",
    "gran",
    "grandad",
    "grandchild",
    "grandma",
    "grandpa",
    "grandparent",
    "granny",
    "housemate",
    "husband",
    "kid",
    "lad",
    "landlord",
    "mate",
    "mom",
    "mother",
    "motherinlaw",
    "mum",
    "nan",
    "nana",
    "neighbor",
    "neighbour",
    "nephew",
    "niece",
    "parent",
    "parents",
    "partner",
    "sibling",
    "sister",
    "sisterinlaw",
    "son",
    "stepdad",
    "stepdaughter",
    "stepmum",
    "stepson",
    "student",
    "teacher",
    "teenager",
    "twins",
    "uncle",
    "wife",
];

pub(super) const SURE: &[&str] = &[
    "believe", "believed", "certain", "expect", "guess", "imagine", "knew", "know", "reckon",
    "say", "suppose", "sure", "think", "thought",
];

pub(super) const DOUBTED: &[&str] = &[
    "doubt",
    "doubted",
    "doubting",
    "doubts",
    "uncertain",
    "unsure",
];

pub(super) const BELIEVE_IT: &[&str] = &["believe", "believed"];

pub(super) const DOUBT: &[&str] = &[
    "believe", "believed", "imagine", "imagined", "think", "thought",
];

pub(super) const CANNOT: &[&str] = &["always", "cannot", "cant", "couldnt", "never"];

pub(super) const BIRTHDAY_REPLIES: &[&str] = &[
    "Happy birthday — anything planned for it?",
    "A birthday, nice. What are you doing for it?",
    "Happy birthday when it comes. Anything planned?",
];

pub(super) const ANNIVERSARY_REPLIES: &[&str] = &[
    "Happy anniversary — anything planned for it?",
    "An anniversary, lovely. What are you two doing for it?",
    "Happy anniversary when it comes. Are you doing anything for it?",
];

pub(super) const WISH_REPLIES: &[&str] = &[
    "Happy birthday to whoever it's for — is it yours today?",
    "Aw, happy birthday — though I don't have one to celebrate. Whose is it?",
    "Happy birthday! Is it yours, or someone you're off to see?",
];

pub(super) const WEDDING_REPLIES: &[&str] = &[
    "A wedding to look forward to — how are the plans going?",
    "Lovely. How is the planning going?",
    "Congratulations! How far off is it, and how are the plans going?",
];

pub(super) const BIRTHDAY_FOR_SOMEONE_ELSE: &[&str] = &[
    "Happy birthday to them — are you doing something for it?",
    "A birthday to get something for. Any idea what yet?",
    "Lovely. Are you seeing them for it?",
];

pub(super) const ANNIVERSARY_FOR_SOMEONE_ELSE: &[&str] = &[
    "Happy anniversary to them — are you doing anything for it?",
    "Lovely. Are you part of the plans for it?",
    "Nice. Is there something you are giving them for it?",
];
