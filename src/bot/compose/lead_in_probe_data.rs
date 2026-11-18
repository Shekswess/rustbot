//! Source fallback patterns and reply data from PyBot 7d63c7f.

pub(super) const PRIVACY_ASK: &str = r#"(?i)\b(?:keep|keeps|keeping|store|stores|storing|save|saves|saving|record|records|recording|log|logs|logging|share|shares|sharing|sell|sells|selling)\s+(?:on\s+|out\s+|around\s+|off\s+|away\s+)?(?:what\s+i\s+(?:said|say|tell|type|typed|write|wrote)|what\s+we\s+(?:said|talk|discussed)|my\s+(?:messages?|data|details|information|info|chats?|records?|conversations?)|our\s+(?:chats?|conversations?)|this\s+(?:chats?|conversations?)|the\s+(?:chats?|conversations?))\b|\b(?:can|could|will|would|do|does|did)\s+(?:anyone|anybody|someone|somebody|people|other\s+people|people\s+else|anyone\s+else|everyone|staff)\s+(?:else\s+)?(?:see|read|hear|get|access|look\s+at)\s+(?:this|what|my|our|the)\b|\b(?:do|does|did|would|will|can|could)\s+(?:you|u)\s+(?:ever\s+)?(?:tell|show|share|give|send|hand|pass|forward)\s+(?:anyone|anybody|someone|somebody|people|others|a\s+third\s+party)\b|\bwho\s+(?:(?:can|could|else\s+can|else\s+could)\s+(?:see|read|access)|sees|reads|accesses)\s+(?:this(?:\s+(?:chat|conversation|thread))?(?=\s*[?.!]*\s*$)|my\s+(?:messages?|data|chats?)\b|what\s+i\s+(?:say|type|write)\b)|\bis\s+(?:this|it|what\s+i\s+say|what\s+i\s+tell\s+you)(?:\s+(?:chat|conversation|page|app|thing|site))?\s+(?:private|confidential|secure|saved|recorded|stored)\b|\bare\s+(?:my|our|these)\s+(?:messages?|data|details|chats?|records?|words)\s+(?:private|safe|secure|confidential|shared|stored|saved|recorded|logged)\b|\b(?:does|do|did|is|was)\s+(?:this|it|that|everything\s+i\s+say|what\s+i\s+say)\s+(?:get\s+|being\s+|ever\s+get\s+)?(?:shared|stored|saved|recorded|logged|kept|seen|read|passed)\b|\bare\s+(?:you|u)\s+(?:keeping|recording|saving|storing|logging)\s+(?:this|what\s+i|my|our|the\s+(?:chat|conversation))\b|\b(?:do|does|did|will|would|can|could)\s+(?:you|u)\s+(?:ever\s+)?(?:keep|save|log|record|store)\s+(?:a\s+|any\s+)?(?:record|log|copy|note|transcript|file|history)\s+of\b|\b(?:do|does|did|would|will|can|could)\s+(?:you|u)\s+(?:ever\s+)?(?:pass|send|hand|give|forward|share)\s+(?:this|that|it|what\s+i\s+say)\s+(?:on|along|around|off|over)?\s*(?:to|for)\s+(?:anyone|anybody|someone|somebody|people|others|a\s+third\s+party)\b|\bprivacy\s+(?:here|on\s+this|in\s+this|with\s+you|at\s+all)\b|\bwhat\s+(?:are|do)\s+(?:you|u)\s+(?:(?:do|doing)\s+)?(?:with|using)\s+(?:(?:my|our)\s+(?:data|details|information|info|messages|words)|what\s+i\s+(?:tell|say|type|write))\b"#;

pub(super) const PRIVACY_SHAPES: &str = r#"(?i)\b(?:is|are|was)\s+(?:this|the|our)\s+(?:a\s+|an\s+)?(?:private|confidential|secure)\s+(?:chat|conversation|call|line|session|thread)\b|\b(?:is|are|was|will|would)\s+(?:this|our|the)\s+(?:chat|conversation|call|line|session|thread)\s+(?:going\s+to\s+)?(?:be\s+|stay\s+|stays\s+|get\s+|gets\s+)?(?:private|confidential|secure|saved|recorded|stored|logged|kept|seen|read|shared)\b|\bwill\s+(?:this|it)\s+(?:be|stay|get)\s+(?:private|confidential|recorded|saved|stored|logged|kept)\b|\bis\s+(?:this|it)\s+going\s+to\s+(?:be\s+|stay\s+|get\s+)?(?:private|confidential|recorded|saved|stored|logged|kept)\b|\b(?:do|does|will|would|are|can|could)\s+(?:you|u)\s+(?:be\s+)?(?:keep|keeping|save|saving|record|recording|store|storing|log|logging)\s+(?:this|it|what\s+i\s+say)(?=\s*[?.!]*\s*$)|\bwho\s+(?:sees|reads|hears|can\s+see|can\s+read|can\s+access)\s+this(?:\s+(?:chat|conversation|thread))?(?=\s*[?.!]*\s*$)|\b(?:is|are)\s+(?:my|our)\s+(?:data|details|info|information|messages?|words)\s+(?:private|confidential)\b|\b(?:is|are)\s+(?:my|our)\s+(?:data|details|info|information)\s+(?:safe|secure)(?:\s+(?:with|on|in)\s+(?:you|here|this\s+chat)\b)?(?=\s*[?.!]*\s*$)|\b(?:is|are)\s+(?:my|our)\s+(?:data|details|info|information|messages?|words)\s+(?:safe|secure)\s+(?:with\s+(?:you|u)|here)\b|\bwhat\s+(?:happens|happened|will\s+happen|would\s+happen)\s+to\s+(?:what\s+i\s+(?:say|said|tell|type|write|wrote)\b|my\s+(?:messages?|data|words|details|information|info|chats?)\b|our\s+(?:chat|conversation)\b)(?=\s*[?.!]*\s*$)"#;

pub(super) const BETWEEN_US_ASK: &str = r#"(?i)\b(?:is|are|was|were|will|would|does|do|can)\s+(?:this|that|it)\s+(?:(?:chat|conversation|thread|session)\s+)?(?:just\s+|really\s+)?between\s+(?:us|you\s+and\s+me)\b|\b(?:is|are)\s+(?:what|everything)\s+i\s+(?:say|tell|tell\s+you|type|write)\s+(?:just\s+)?between\s+(?:us|you\s+and\s+me)\b|\bare\s+(?:we|you)\s+(?:private|confidential|anonymous)\s+(?:here|on\s+this|in\s+this)\b|\bis\s+(?:this|it)\s+(?:chat\s+)?anonymous\b|\bwhat\s+(?:happens|happened)\s+to\s+(?:what|everything)\s+i\s+(?:say|tell|type|write|send)\b"#;

pub(super) const NOT_WHAT_I_SAID: &str = r#"(?i)\b(?:keep|store|save|record|log|share|sell|pass)\w*\s+(?:me\s+|us\s+)?(?:the\s+|a\s+|an\s+|my\s+|our\s+)?(?:kitchen|cat|dog|plants?|receipts?|leftovers?|food|milk|bread|eggs?|cheese|wine|beer|keys?|car|bike|house|flat|money|savings|cash|password|photos?|pictures?|files?|documents?|sheet|table|bill|bills|tickets?|seats?|room|garage|garden|washing|clothes|coat|bag|phone|number|address|email|weight|diary|journal|notes)\b|\bprivate\s+(?:rented|rent|rental|landlord|landlords|sale|sector|school|schools|hospital|hospitals|treatment|health|healthcare|health\s?care|medical|insurance|pension|parking|road|land|garden|tuition|tutor|lessons?|detective|investigator|eye|clinic|care|nursery|swimming|coach|hire|contract|members?|club|firm|company|jet|plane|island|beach)\b|\bshared\s+(?:ownership|owner|equity|driveway|garden|house|home|flat|room|kitchen|bathroom|car|bike|account|login|costs?|bills?|custody|care|space|office)\b"#;

pub(super) const PRIVACY_NOT_THIS: &str = r#"(?i)\b(?:eat|eaten|eating|cook|cooked|cooking|freezer|fridge|left\s+out|defrost|reheat|mouldy|moldy|smell|shelf|use\s+by|best\s+before|baby|child|kid|toddler|pregnan\w*|drink|water|ibuprofen|paracetamol|medicine|dose|tablet|drive|driving|swim)\b"#;

pub(super) const ONLINE_ASK: &str = r#"\b(?:can|could|do|does|are|is|have)\s+(?:you|u)\s+(?:[a-z]+\s+){0,3}?(?:browse|search|google|look[\s-]+(?:things|stuff|it|that|them)?\s*up|check|access|reach|connect(?:ed)?|go|get)\s*(?:the\s+|a\s+)?(?:internet|web|online|net|google|sites?|websites?|pages?)\b|\b(?:can|could|do|does|are|is|have)\s+(?:you|u)\s+(?:[a-z]+\s+){0,3}?(?:online|connected|internet|web|internet\s+access)\b|\bdo\s+(?:you|u)\s+have\s+(?:[a-z]+\s+){0,2}?(?:access|internet|wifi)\b|\bdo\s+(?:you|u)\s+have\s+(?:[a-z]+\s+){0,2}?connection\s+(?:to|with)\s+(?:the\s+)?(?:internet|web|net|online)\b|\b(?:can|could|would|will)\s+(?:you|u)\s+(?:please\s+)?(?:google|search\b(?=\s*(?:$|for\s+(?:me|us)\b|around\b|(?:it|that|this|things|stuff|something)\b|online\b|the\s+(?:web|internet|net)\b|sites?\b|pages?\b|websites?\b))|look\s+(?:it|that|this|things|stuff)\s*up)\b|\b(?:google|search|look\s+(?:\w+\s+){0,2}?up)\b[^.?!]{0,30}?\b(?:times|timetables?|schedules?|numbers?|phone|telephone|address|postcode|postcodes?|prices?|costs?|fares?|deals?|offers?|reviews?|ratings?|scores?|results?|news|flights?|trains?|buses?|coaches|tickets?|availability|stock|opening\s+hours|weather|forecast|vacanc(?:y|ies)|listings?|status|delays?|traffic|menu|lead\s+times?|deadlines?|exchange\s+rates?)\b[^.?!]{0,30}?\bfor\s+(?:me|us)\b\s*(?:(?:please|then|now|thanks|thank\s+you)\s*)*[.?!]*\s*$"#;

pub(super) const OWN_DEVICE: &str = r#"\b(?:my|our)\s+(?:laptop|phone|pc|computer|router|wifi|broadband|tablet|ipad|tv)\b|\bhelp\s+me\b|\bhow\s+(?:do|can|should)\s+i\b"#;

pub(super) const A_BARE_ONLINE: &str = r#"^\s*(?:so\s+|and\s+|ok(?:ay)?\s+|just\s+)?(?:are|is|r)\s+(?:you|u)\s+(?:really\s+|still\s+|even\s+)?(?:online|connected|there)\s*(?:(?:right\s+|at\s+the\s+)?now\s*|at\s+the\s+moment\s*|today\s*|tonight\s*|then\s*|please\s*|mate\s*|though\s*)*[\s?!.]*$"#;

pub(super) const LEAD_IN: &str = r#"^(?:(?:ok|okay|okey|kk|right|so|ah|oh|ohh|well|anyway|anyways|and|but|now|hmm+|er+|um+|uh+|hey|sorry|quick question|random question|stupid question|silly question|daft question|this is (?:a bit |a |totally )?random|bit of a random one|a random one|unrelated|off topic|just wondering|i was wondering|wondering|can i ask|may i ask|just out of curiosity|out of curiosity|by the way|btw)\b[\s,:.!?-]*){1,4}"#;

pub(super) const BARE_PICK: &str = r#"(?i)^\s*(?:(?:so|and|but|ok|okay|well|then|just|hey|hi|right|anyway|anyways|oh|hmm|yeah|yes|alright|honestly|you\s+know)[\s,]+)*(?:what|which)\s+(?:should|shall|do|would|can|could|might)\s+i\s+(?P<verb>wear|put\s+on|buy|get|do|take|bring|look\s+(?:for|at)|expect|ask|sign|order|prepare)\b(?:\s+(?:now|today|tonight|tomorrow|instead|else|next|then))?\s*\??\s*$"#;

pub(super) const LIVE_NOT: &[&str] = &[
    "difference",
    "climate",
    "how does",
    "why does",
    "why do ",
    "explain",
    "meaning of",
];

pub(super) const LIVE_NOT_HOURS: &[&str] = &[
    "sun",
    "sunrise",
    "sunset",
    "sun set",
    "sun rise",
    "moon",
    "moonrise",
    "tide",
    "tides",
    "high tide",
    "low tide",
    "dawn",
    "dusk",
    "daylight",
    "stars",
    "milky way",
    "eclipse",
];

pub(super) const LIVE_STRONG: &[&str] = &[
    "times",
    "forecast",
    "opening hours",
    "opening times",
    "how much",
    "price",
    "what time",
    "the score",
    "latest on",
    "latest news",
    "news today",
    "any news today",
    "any headlines",
    "any news on",
    "any news about",
    "any news of",
    "any update on",
    "any word on",
    "spill the news",
    "any results",
    "any result",
    "what channel",
    "any traffic",
    "traffic problems",
    "road closed",
    "closed road",
    "train timetable",
    "bus timetable",
    "are the buses",
    "are the trains",
    "buses running",
    "trains running",
    "tube line",
    "the m25 closed",
    "m25 closed",
];

pub(super) const LIVE_VENUE_RE: &str = r#"(?<![a-z])(?:recycling\ centre|leisure\ centre|garden\ centre|supermarket|post\ office|hairdresser|launderette|greengrocer|restaurant|sainsbury|morrisons|waitrose|pharmacy|takeaway|hospital|workshop|iceland|postbox|library|chemist|surgery|dentist|gallery|nursery|station|airport|council|butcher|branch|doctor|chippy|museum|centre|market|barber|office|clinic|school|garage|studio|church|bakery|store|tesco|co\-op|salon|hotel|kiosk|venue|depot|shop|asda|aldi|lidl|coop|bank|dump|cafe|café|pool|hall|park|club|vet|tip|pub|bar|gym|gp)(?:s|es)?(?![a-z])"#;

pub(super) const LIVE_HOURS_ASK: &str = r#"\b(?:when|what time|what hours|how late|what times)\s+(?:does|do|is|are|will|would|can|doesnt|don't|dont|isnt|is it)\b[^.!?]{0,40}?\b(?:open|opens|closed|closes|close|shut|shuts|shutting|closing|re-?opens?|re-?opening|re-?opened)\b"#;

pub(super) const LIVE_YES_NO_OPEN: &str = r#"^\s*(?:so|and|but|ok|okay|also|hi|hello|morning|quick one)?\s*,?\s*(?:is|are|will|does|do|can)\s+[^.!?]{0,40}?\b(?:open|opens|shut|shuts|closed|closing|close)\b(?!\s*(?:up|ness|ing|ly|air|plan|letter|account|evening|day|days|morning|afternoon)\b)(?!\s*to\s+(?!the\s+public\b))"#;

pub(super) const OWN_TIME_ASK: &str = r#"(?<![a-z])what time (?:do|does|did|will|would|should|can|could|shall) (?:i|we|my|our)(?![a-z])"#;

pub(super) const OWN_PLACE: &[&str] = &[
    "shop",
    "store",
    "stores",
    "branch",
    "office",
    "cafe",
    "café",
    "canteen",
    "pub",
    "bar",
    "gym",
    "surgery",
    "pharmacy",
    "chemist",
    "bank",
    "salon",
    "museum",
    "tip",
    "library",
    "restaurant",
    "takeaway",
    "chippy",
    "garage",
    "hotel",
    "school",
    "nursery",
    "clinic",
    "centre",
    "center",
    "pool",
    "market",
    "stall",
    "van",
    "kiosk",
    "studio",
    "workshop",
];

pub(super) const OWN_VERB_OPEN: &str = r#"(?<![a-z])(?:i|we) (?:open|close|shut)(?![a-z])"#;

pub(super) const OWN_PERSON: &[&str] = &[
    "son",
    "daughter",
    "kid",
    "kids",
    "child",
    "children",
    "baby",
    "wife",
    "husband",
    "partner",
    "mum",
    "mom",
    "mother",
    "dad",
    "father",
    "boyfriend",
    "girlfriend",
    "nan",
    "grandma",
    "grandad",
    "grandson",
    "granddaughter",
];

pub(super) const LIVE_SPORT: &[&str] = &[
    "match",
    "game",
    "score",
    "scores",
    "football",
    "rugby",
    "cricket",
    "tennis",
    "boxing",
    "league",
    "cup",
    "final",
    "semi",
    "fixtures",
    "fixture",
    "kick off",
    "kick-off",
    "goal",
    "goals",
    "wicket",
    "innings",
    "test match",
    "playoffs",
    "play offs",
    "f1",
    "formula one",
    "grand prix",
    "the ashes",
    "the table",
];

pub(super) const LIVE_LISTING_ASK: &str = r#"\b(?:what|which)\s+channel\b|\bwhere\s+(?:can|do)\s+i\s+watch\b|\bis\s+it\s+on\s+(?:the\s+)?(?:tv|telly|iplayer|bbc|itv|sky)\b|\bwhat\s+time\s+(?:is\s+)?(?:kick\s*off|does\s+(?:the|it)\s+(?:start|kick\s*off))|\bwhat\s+time\s+(?:is|does|do|will)\b[^?]{0,30}?\b(?:start|starts|kick\s*off|begin|kick-off|coverage|highlights)\b|\bwhen\s+(?:is|are)\s+(?:the|we|they)\s+(?:next\s+)?(?:match|game|fixture)|\bwhen\s+do\s+(?:we|they)\s+play\b"#;

pub(super) const POINTING_WORDS: &[&str] = &[
    "about", "again", "ah", "am", "and", "are", "at", "be", "been", "but", "by", "come", "did",
    "do", "does", "exactly", "explain", "for", "how", "in", "is", "it", "know", "long", "many",
    "mean", "meaning", "means", "meant", "much", "of", "oh", "ok", "okay", "on", "one", "ones",
    "please", "really", "right", "say", "so", "tell", "that", "thats", "then", "these", "this",
    "those", "to", "u", "was", "well", "were", "what", "whats", "when", "whens", "where", "wheres",
    "which", "who", "whom", "whos", "why", "with", "work", "working", "works", "you",
];

pub(super) const POINTERS: &[&str] = &[
    "it", "one", "ones", "that", "thats", "these", "this", "those",
];

pub(super) const MEAN_WORDS: &[&str] = &["explain", "mean", "meaning", "means", "meant"];

pub(super) const POINTING_ASK: &[&str] = &[
    "how", "what", "whats", "when", "whens", "where", "wheres", "which", "who", "whos", "why",
];

pub(super) const POINTING_IDIOMS: &[&str] = &["how come", "how comes", "how so", "what for"];

pub(super) const POINTING_LINES: &[&str] = &[
    "I can't see what you're looking at, so 'that' could be anything. Put the thing in a couple of words and I'll answer it properly.",
    "There's nothing on my side for 'that' to land on - tell me the thing itself and I'll take it from there.",
    "'That' could be a lot of things and I've none of them in front of me - say the thing you mean and I'll come at it properly.",
    "'That' has nothing behind it here. What is the thing you're pointing at?",
];

pub(super) const BARE_PICK_DO: &[&str] = &[
    "Tell me what has gone on, even roughly, and I will help you work out the first step. Often the order of the next two things is the whole problem.",
    "Say a bit about what is in front of you and I will go at it with you - what has happened, and what you would like to be true by Friday.",
];

pub(super) const PRIVACY_LINES: &[&str] = &[
    "Nothing you type is written down or sent anywhere - what you tell me is used here, in this conversation, and nowhere else. No account, no log, and nobody else reading it.",
    "Private to this chat. I keep no file of it and I pass nothing on; what you say stays here while we talk, so I can follow it, and that is all.",
    "Nothing leaves this conversation: no database, no record, nobody else seeing it. I hold what you say only for as long as we are talking.",
];

pub(super) const ONLINE_LINES: &[&str] = &[
    "Nothing live on this end, no: I can't open a page, search, or check anything while we talk. What I have is a lot of everyday things written into me - if it's not in there, say so and I'll tell you straight.",
    "No connection at all. No browsing, no searching, no reading the news - just what I was built with and what you tell me. What did you want looked up?",
];

pub(super) const HOURS_ASK_WORDS: &[&str] = &[
    "opening hours",
    "opening times",
    "opening time",
    "open today",
    "open now",
    "close today",
    "closing time",
    "what time does",
    "what time do",
    "is it open",
    "shut today",
    "open on",
];

pub(super) const HOURS_LINES: &[&str] = &[
    "The opening hours are the first thing a venue changes and the last thing it tells anyone: a refurb, a bank holiday or new owners will do it. Their own page or map listing has the current ones.",
    "No opening hours in front of me. Whoever runs the place - a council tip, a chain, one landlord - announces a change itself, and its own page or phone number has today's.",
];
