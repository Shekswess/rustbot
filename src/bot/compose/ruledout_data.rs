//! Patterns and reply pools from bot/compose/ruledout.py at swarm 24cbc0439e07f.

pub(super) const RULED_OUT_TAIL: &str = "\\s+(?:related|based|ish|esque)$";
pub(super) const RULED_OUT_PREP: &str = "\\s+(?:of|to|on|for|by|from|about)\\s+";
pub(super) const RULED_OUT_OPENS_WITH: &str = "^(?:with|without|in|on|at|for|about|of|to)\\b";
pub(super) const RULED_OUT_SOFTENING: &str =
    "^(?:and\\s+|but\\s+)?nothing\\s+(?:too|very|that|which|of)\\b";
pub(super) const RULED_OUT_KEEPS: &str = "^(?:a|an|any|the|my|another)\\s+";
pub(super) const RULED_OUT_NAMED: &str =
    "\\*[^*]{2,}\\*|\\\"[^\\\"]{2,}\\\"|\\u{201c}[^\\u{201d}]{2,}\\u{201d}";
pub(super) const RULED_OUT_KEEP_WHOLE: &str =
    "\\b(?:999|911|112|111|ambulance|a&e|casualty|emergency|urgent care|999 line)\\b";
pub(super) const RULED_OUT_SYMPATHY: &str = "\\b(?:sorry|sad news|passed away|grief|grieving|bereave\\w*|condolence\\w*|miss (?:him|her|them))\\b";
pub(super) const MANNERS: &str = "(?:[\\s,!.?]*\\b(?:pls+|plz|please|pleas|plse|thx|thanx|thanks|thankyou|thank\\s+you|ta|ty|cheers|cheerio|mate|m8|fella|buddy|bud|pal|x|xx|xxx)\\b)+[\\s,!.?]*$";
pub(super) const RULED_OUT_THEN_ELSE: &str = "[\\s,;:-]+(?:then\\s+|and\\s+)?(?:so\\s+)?(?:what|anything|something)\\s+else(?:\\s+[a-z']+){0,4}\\s*[.?!]*$";
pub(super) const CLAIM_OF_FAILURE: &str = "^\\W*(?:(?:ok|okay|so|well|yeah|yep|yes|and|but|oh|right|alright|no|nope|nah|still|now)\\W+)*(?:nothing|nowt|none\\s+of\\s+(?:it|that|them|these|those)|neither\\s+of\\s+(?:them|those|it))(?:\\s+\\w+){0,5}\\W*$";
pub(super) const PET_NAMED: &str = "\\b(?:dog|dogs|puppy|puppies|pup|pups|cat|cats|kitten|kittens|rabbit|rabbits|bunny|bunnies|hamster|hamsters|gerbil|gerbils|guinea pig|guinea pigs|parrot|parrots|budgie|budgies|bird|birds|horse|horses|pony|ponies|fish|goldfish|tortoise|tortoises|snake|snakes|lizard|lizards|ferret|ferrets|pet|pets)\\b";
pub(super) const WANTED_ONE: &str = "\\b(?:want|wants|wanted|would like|'d like|after|thinking (?:of|about)|considering|planning to get|should (?:i|we) get|can (?:i|we) get|shall we get|get (?:a|an)|getting (?:a|an)|looking at getting)\\b";
pub(super) const TITLES_IN_ANSWER: &str = "\\*[^*]{2,60}\\*";
pub(super) const RULED_OUT_WORDS: &[&str] = &[
    "thing",
    "things",
    "anything",
    "something",
    "that",
    "this",
    "which",
    "what",
    "any",
    "more",
    "much",
    "needs",
    "need",
    "uses",
    "use",
    "takes",
    "take",
    "gets",
    "get",
    "has",
    "have",
    "requires",
    "require",
    "wants",
    "want",
    "please",
    "thanks",
    "really",
    "very",
];
pub(super) const RULED_OUT_QUALITY: &[&str] = &[
    "bad",
    "good",
    "sad",
    "happy",
    "awful",
    "terrible",
    "horrible",
    "weird",
    "wrong",
    "fine",
    "ok",
    "okay",
    "sure",
    "nice",
    "lovely",
    "great",
    "better",
    "worse",
    "best",
    "worst",
    "big",
    "small",
    "little",
    "long",
    "short",
    "new",
    "old",
    "easy",
    "hard",
    "difficult",
    "expensive",
    "cheap",
    "different",
    "same",
    "busy",
    "tired",
    "stressed",
    "ill",
    "sick",
];
pub(super) const RULED_OUT_NOT_A_THING: &[&str] = &[
    "case",
    "point",
    "problem",
    "issue",
    "fact",
    "joke",
    "idea",
    "whole",
    "kind",
    "sort",
    "type",
    "way",
    "matter",
    "question",
    "answer",
    "reason",
    "truth",
    "deal",
    "end",
    "yet",
    "either",
    "though",
    "well",
    "thanks",
    "is",
    "are",
    "was",
    "were",
    "be",
    "been",
    "going",
    "happening",
    "happened",
    "here",
    "there",
    "now",
    "ever",
    "again",
    "else",
    "right",
    "left",
    "much",
    "more",
    "less",
    "least",
    "still",
    "even",
    "already",
    "quite",
    "rather",
    "able",
    "possible",
    "worth",
    "enough",
    "true",
    "false",
    "sure",
    "wrong",
    "fine",
];
pub(super) const RULED_OUT_REPLIES_PLAIN: &[&str] = &[
    "There's no {thing} in that answer, so it still stands. {opening}",
    "That one never mentions {thing} at all, so nothing changes. {opening}",
    "Nothing there was about {thing}, so the answer holds. {opening}",
];
pub(super) const RULED_OUT_REPLIES_THE: &[&str] = &[
    "None of that was {thing} - the answer still stands. {opening}",
    "That one never mentions {thing} at all, so nothing changes. {opening}",
    "Nothing there was {thing}, so the answer holds. {opening}",
];
pub(super) const RULED_OUT_REPLIES_A_ROW: &[&str] = &[
    "Fair enough - no {thing} needed. {opening}",
    "Quite right, no {thing} wanted. {opening}",
    "Fair enough: one calm sentence, no {thing}. {opening}",
];
pub(super) const RULED_OUT_CLASS: &[&str] = &[
    "tablets",
    "pills",
    "painkillers",
    "painkiller",
    "medication",
    "medicine",
    "medicines",
    "drugs",
    "antibiotics",
    "supplements",
    "vitamins",
    "inhalers",
    "creams",
    "series",
    "episodes",
    "thriller",
    "thrillers",
    "horror",
    "comedies",
    "documentaries",
    "meat",
    "dairy",
    "gluten",
    "carbs",
    "carbohydrates",
    "sugar",
    "chemicals",
    "screens",
    "toys",
];
pub(super) const RULED_OUT_PARTICLES: &str = "out off up down away back over through around about along together home here there it self selves";
pub(super) const RULED_OUT_NO_ANSWER: &[&str] = &[
    "sorry",
    "that sounds",
    "sounds like",
    "i'm listening",
    "i am listening",
    "got it",
    "thanks for telling",
    "no worries",
    "of course",
    "noted",
    "fair enough",
    "that's rough",
    "thats rough",
];
pub(super) const CLAIM_SMALL: &str = "at all so far yet again now still really just about on with me my it this that them those these any more much either the a an of in for to day days week weeks moment while times time one bit lot lots out up down been has have had is was are were am do does did";
pub(super) const CLAIM_LEADS: &[&str] = &[
    "Then it is the next part that matters: {step}",
    "Then take it a step further: {step}",
    "Then the next step: {step}",
    "Next: {step}",
];
pub(super) const CLAIM_LEADS_IF: &[&str] = &[
    "The next thing to try: {step}",
    "Next: {step}",
    "Then the next check: {step}",
];
pub(super) const KINDS_A_LIST_CAN_SWAP: &[&str] = &[
    "horror",
    "scary",
    "frightening",
    "creepy",
    "comedy",
    "comedies",
    "funny",
    "romance",
    "romantic",
    "sad",
    "documentary",
    "gory",
];
pub(super) const RULED_OUT: &[(&str, bool)] = &[
    (
        "^(?:and\\s+|but\\s+)?not\\s+(?P<thing>(?:a|an|any|the)\\s+[a-z][a-z' ]{0,24})$",
        true,
    ),
    (
        "^(?:and\\s+|but\\s+)?not\\s+(?P<thing>[a-z][a-z' ]{0,20}s)$",
        false,
    ),
    (
        "^(?:and\\s+|but\\s+)?not\\s+(?P<thing>[a-z][a-z' ]{0,20}\\s+related)$",
        false,
    ),
    (
        "^(?:and\\s+|but\\s+)?not\\s+(?:too|much)\\s+(?P<thing>[a-z][a-z' ]{1,20})$",
        false,
    ),
    (
        "^(?:and\\s+|but\\s+)?nothing\\s+(?P<thing>[a-z][a-z' ]{1,24})$",
        false,
    ),
    (
        "^(?:and\\s+|but\\s+)?(?:i|we)\\s+(?:really\\s+|just\\s+|actually\\s+|personally\\s+)?(?:dont|don't|do\\s+not|didnt|did\\s+not|never)\\s+want\\s+(?P<thing>trouble|hassle|aggro|drama|upset|bother)$",
        false,
    ),
    (
        "^(?:and\\s+|but\\s+)?(?:i|we)\\s+(?:really\\s+|just\\s+|actually\\s+|personally\\s+)?(?:dont|don't|do\\s+not|didnt|did\\s+not|never)\\s+want\\s+to\\s+(?P<thing>(?:fall\\s+out|falling\\s+out|argue|fight|quarrel|kick\\s+up\\s+a\\s+fuss|make\\s+a\\s+fuss|get\\s+into\\s+it))$",
        false,
    ),
    (
        "^(?:and\\s+|but\\s+)?(?:i|we)\\s+(?:really\\s+|just\\s+|actually\\s+|personally\\s+)?(?:dont|don't|do\\s+not|didnt|did\\s+not|never)\\s+(?:like|want|have|own|use|need|rate|fancy)\\s+(?P<thing>(?:a|an|any|the|my|another)\\s+[a-z][a-z' ]{0,24})$",
        true,
    ),
    (
        "^(?:and\\s+|but\\s+|no,?\\s+|nah,?\\s+)?(?:it|that|this)(?:'s|s)?\\s+(?:is\\s+not|isnt|isn't|not)\\s+(?P<thing>(?:a|an|any|the|my|another)\\s+[a-z][a-z' ]{0,24})$",
        true,
    ),
    (
        "^(?:and\\s+|but\\s+|no,?\\s+|nah,?\\s+)?(?:it|that|this)(?:'s|s)?\\s+(?:is\\s+not|isnt|isn't|not)\\s+(?P<thing>[a-z][a-z' ]{1,20}s)$",
        false,
    ),
];
pub(super) const RULED_OUT_NOT_THEN_ELSE: &[(&str, bool)] = &[
    (
        "^(?:and\\s+|but\\s+)?not\\s+(?P<thing>(?:a|an|any|the)\\s+[a-z][a-z' ]{0,24})$",
        true,
    ),
    (
        "^(?:and\\s+|but\\s+)?not\\s+(?P<thing>[a-z][a-z' ]{1,24})$",
        false,
    ),
];
pub(super) const RULED_OUT_NO_THEN_ELSE: &[(&str, bool)] = &[
    (
        "^(?:no|nah|nope|no,|nah,|nope,)\\s+(?P<thing>(?:a|an|any|the)\\s+[a-z][a-z' ]{0,24})$",
        true,
    ),
    (
        "^(?:no|nah|nope|no,|nah,|nope,)\\s+(?P<thing>[a-z][a-z' ]{1,24})$",
        false,
    ),
];
pub(super) const A_ROW_NAMES: &[(&str, &str)] = &[
    ("fall out", "falling out"),
    ("falling out", "falling out"),
    ("argue", "argument"),
    ("quarrel", "argument"),
    ("get into it", "argument"),
    ("fight", "fight"),
    ("kick up a fuss", "fuss"),
    ("make a fuss", "fuss"),
    ("row", "row"),
    ("trouble", "trouble"),
    ("hassle", "hassle"),
    ("bother", "bother"),
    ("aggro", "aggro"),
    ("drama", "drama"),
    ("upset", "upset"),
];
pub(super) const RULED_OUT_INSTANCES: &[(&str, &[&str])] = &[
    (
        "tablets",
        &[
            "paracetamol",
            "ibuprofen",
            "aspirin",
            "codeine",
            "antihistamine",
            "melatonin",
            "sertraline",
            "tablet",
            "pill",
            "painkiller",
            "dose",
        ],
    ),
    (
        "pills",
        &[
            "paracetamol",
            "ibuprofen",
            "aspirin",
            "codeine",
            "antihistamine",
            "melatonin",
            "pill",
            "tablet",
            "painkiller",
        ],
    ),
    (
        "painkillers",
        &[
            "paracetamol",
            "ibuprofen",
            "aspirin",
            "codeine",
            "painkiller",
        ],
    ),
    (
        "painkiller",
        &[
            "paracetamol",
            "ibuprofen",
            "aspirin",
            "codeine",
            "painkiller",
        ],
    ),
    (
        "medication",
        &[
            "paracetamol",
            "ibuprofen",
            "tablet",
            "pill",
            "dose",
            "prescription",
        ],
    ),
    (
        "medicine",
        &["paracetamol", "ibuprofen", "tablet", "pill", "dose"],
    ),
    (
        "medicines",
        &["paracetamol", "ibuprofen", "tablet", "pill", "dose"],
    ),
    (
        "drugs",
        &[
            "paracetamol",
            "ibuprofen",
            "antibiotic",
            "painkiller",
            "tablet",
        ],
    ),
    ("antibiotics", &["antibiotic", "penicillin", "amoxicillin"]),
    (
        "supplements",
        &["supplement", "vitamin", "iron", "magnesium", "omega"],
    ),
    (
        "vitamins",
        &["vitamin", "supplement", "iron", "magnesium", "omega"],
    ),
    ("creams", &["cream", "ointment", "lotion", "gel"]),
    (
        "series",
        &[
            "series", "episode", "episodes", "season", "seasons", "film", "films", "movie",
            "movies",
        ],
    ),
    (
        "episodes",
        &[
            "episode", "series", "season", "film", "films", "movie", "movies",
        ],
    ),
    (
        "thriller",
        &[
            "thriller",
            "murder",
            "mystery",
            "crime",
            "detective",
            "suspense",
            "serial",
            "killer",
        ],
    ),
    (
        "thrillers",
        &[
            "thriller",
            "murder",
            "mystery",
            "crime",
            "detective",
            "suspense",
            "serial",
            "killer",
        ],
    ),
    (
        "horror",
        &["horror", "scary", "ghost", "haunted", "frightening"],
    ),
    (
        "meat",
        &[
            "meat", "chicken", "beef", "pork", "lamb", "bacon", "sausage", "mince", "steak", "ham",
            "turkey",
        ],
    ),
    (
        "dairy",
        &[
            "dairy", "milk", "cheese", "butter", "cream", "yoghurt", "yogurt",
        ],
    ),
    (
        "gluten",
        &[
            "gluten", "bread", "pasta", "flour", "wheat", "couscous", "barley",
        ],
    ),
    (
        "carbs",
        &["rice", "pasta", "bread", "potato", "potatoes", "flour"],
    ),
    (
        "carbohydrates",
        &["rice", "pasta", "bread", "potato", "potatoes", "flour"],
    ),
    ("sugar", &["sugar", "honey", "syrup", "sweetener", "jam"]),
    (
        "chemicals",
        &[
            "vinegar",
            "bleach",
            "ammonia",
            "caustic",
            "solvent",
            "spray",
            "bicarbonate",
        ],
    ),
    (
        "screens",
        &[
            "screen",
            "phone",
            "television",
            "tv",
            "laptop",
            "monitor",
            "tablet",
        ],
    ),
    ("toys", &["toy", "lego", "blocks", "play"]),
];
