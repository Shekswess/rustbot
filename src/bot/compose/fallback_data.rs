//! Reply pools and matching tables from PyBot at 7d63c7fbebb44c7e466e5c8b5c1f341ab767929e.

pub(super) const GREETING: &[&str] = &[
    "Hey! What's on your mind?",
    "Hi there — what can I help with?",
    "Hello! Good to see you. What are you thinking about?",
    "Hey! I'm here. What would you like to talk about?",
    "Hi! What's up?",
];

pub(super) const THANKS: &[&str] = &[
    "Any time!",
    "You're welcome — glad that helped.",
    "No problem at all. Shout if you need anything else.",
    "Happy to help.",
];

pub(super) const EMPTY: &[&str] = &[
    "I'm here — what's on your mind?",
    "Go ahead, I'm listening.",
    "Take your time. What would you like to talk about?",
];

pub(super) const UNKNOWN: &[&str] = &[
    "I'm not sure about that one, sorry — I'd rather not guess. A quick search online will get you a solid answer, and I can help you make sense of what you find.",
    "That's beyond what I know well, sorry. Searching for it will turn up better sources than I can offer; tell me what you find and I'll help you weigh it up.",
    "I don't know enough to answer that honestly. A search should sort you out quickly — and I'm happy to help with the rest of it.",
];

pub(super) const UNKNOWN_DECIDE: &[&str] = &[
    "That's your call, not mine — I don't know enough about your side of it. Tell me what you're weighing and I'll help you pick.",
    "Can't call that one for you, sorry. Say what you'd lose either way and I'll lay it out with you.",
    "Not mine to decide, sorry. Tell me what matters most here and I'll help you think it through.",
];

pub(super) const NEUTRAL: &[&str] = &[
    "Got it. Is there anything you'd like a hand with?",
    "Thanks for telling me — I'm here if you want to dig into any of that.",
    "I'm listening — say more if it helps.",
];

pub(super) const COLD_OPEN: &[&str] = &[
    "Go on - what's been happening?",
    "What do you need? I'll take it from there.",
    "What would be most useful to sort out?",
];

pub(super) const ACK: &[&str] = &[
    "Great — shout if you need anything else.",
    "Ha, fair enough. I'm here if you want a hand with anything.",
    "Right — anything else on your mind?",
];

pub(super) const HAND: &[&str] = &[
    "Of course - what are you stuck on?",
    "Happy to. Tell me what's going on and we'll take it from there.",
    "Yes, say what it is and I'll help if I can.",
];

pub(super) const HOW_ARE_YOU: &[&str] = &[
    "Good, thanks for asking — and good to have you here. What's on your mind?",
    "Can't complain. Better question: how are you doing?",
    "Fine and busy, the two usual things. How's your day been?",
    "I'm well, thanks. What are you up to today?",
];

pub(super) const SORRY_FOR_ASKING: &[&str] = &[
    "No need to apologise - asking is what I'm here for.",
    "You're not bothering me at all. What's on your mind?",
    "No bother - ask away.",
    "Nothing to be sorry about. Go on.",
];

pub(super) const SORRY_ON_ITS_OWN: &[&str] = &[
    "Nothing to be sorry about.",
    "No need to apologise - no harm done.",
    "That's alright. Nothing to say sorry for.",
    "No worries at all.",
];

pub(super) const NEVER_MIND: &[&str] = &[
    "Fine - we can leave that one.",
    "No problem. Say no more.",
    "Alright - forgotten. What else is on your mind?",
    "Dropped. What else would you like to do?",
];

pub(super) const THANKS_THEN_SAID: &[&str] = &[
    "Glad to hear it — and thanks for coming back to say so.",
    "Good. Thanks for letting me know.",
    "Pleased for you. Thanks for telling me how it went.",
    "That's good to hear. Thanks for saying so.",
];

pub(super) const TIPS_LINES: &[&str] = &[
    "Happy to help — tell me what you've got so far and I'll add what I can.",
    "Sure. What have you tried so far? That tells me where to start.",
];

pub(super) const BARE_PICK_DO: &[&str] = &[
    "Tell me what has gone on, even roughly, and I will help you work out the first step. Often the order of the next two things is the whole problem.",
    "Say a bit about what is in front of you and I will go at it with you - what has happened, and what you would like to be true by Friday.",
];

pub(super) const RETURNED_TO_IT: &[&str] = &[
    "Go on - I'm listening.",
    "Right, go on.",
    "Go on then, I'm with you.",
    "Carry on - I'm all ears.",
];

pub(super) const RETURNED_TO_IT_FIRST: &[&str] = &[
    "There's nothing earlier in this chat - what's on your mind?",
    "Nothing to catch up on yet - what would you like to talk about?",
];

pub(super) const RETURNED_TO_A_THING: &[&str] = &[
    "Go on - what about {thing}?",
    "Right, go on. What about {thing}?",
];

pub(super) const FLAT: &[&str] = &[
    "Ugh. What's the day doing to you?",
    "One of those days? What's up?",
    "Flat one, then. What's going on?",
    "Same as ever, then. Anything you'd like to change?",
    "Fair enough. What would make it better?",
];

pub(super) const REACTION_RELIEF: &[&str] = &[
    "That sounds like a relief. What was it in the end?",
    "Phew is right. What was going on?",
    "Thank god for that. What happened?",
];

pub(super) const REACTION_OUCH: &[&str] = &[
    "Ouch. What happened?",
    "Yikes. That sounds rough - what happened?",
    "Ouch is right. Go on.",
    "That's rough. What happened?",
];

pub(super) const REACTION_MISHAP: &[&str] = &[
    "Oops. What did you do?",
    "Whoops. What happened?",
    "Oh dear. What was it?",
];

pub(super) const REACTION_DISBELIEF: &[&str] = &[
    "Go on - what happened?",
    "Really? Tell me what happened.",
    "No kidding. What's going on?",
    "Straight up? Tell me more.",
];

pub(super) const REACTION_REPEAT: &[&str] = &[
    "I'll put it another way - what bit was unclear?",
    "Tell me which part and I'll say it differently.",
    "Say what you're after and I'll come at it from another side.",
];

pub(super) const THANKS_NO_BETTER: &[&str] = &[
    "Sorry it hasn't settled. What's it doing now?",
    "Sorry to hear that — thanks for coming back with it.",
    "That's a nuisance. What's the worst of it now?",
    "Sorry it's no better yet. Say what's changed and I'll go again.",
    "Right, still not right. What's it doing now?",
];

pub(super) const NOTHING_BEFORE_IT: &[&str] = &[
    "There's nothing before this line for me to go on - say what it was about and I'll pick it up.",
    "I haven't said anything yet, so give me the thing itself: what are we going back to?",
    "Nothing on my side yet - tell me what it was and I'll take it from there.",
    "That's the first thing you've said to me, so I'll need it in full: what was it about?",
];

pub(super) const ABOUT_THE_THING: &[&str] = &[
    "Go on — what about {thing}?",
    "Ask away about {thing}.",
    "{Thing} — right. What do you want to know?",
    "What about {thing}? I'm listening.",
];

pub(super) const ASK_AWAY: &[&str] = &[
    "Ask away.",
    "Sure — what is it you want to know?",
    "Go on, I'm listening.",
    "Yes, ask away — I'm here.",
];

pub(super) const GO_ON: &[&str] = &[
    "Go on — what is it?",
    "Sure, go ahead.",
    "I'm listening — tell me.",
    "Yes, what else?",
    "Go ahead, I'm here.",
];

pub(super) const GOODBYE: &[&str] = &[
    "Go well, and see you when you're back.",
    "Right, take it easy. Look after yourself.",
    "No worries at all. Mind yourself.",
    "Bye for now — mind how you go.",
    "All the best — talk soon.",
];

pub(super) const NIGHT_GOODBYE_POOL: &[&str] = &[
    "Night then. Sleep well.",
    "Goodnight - sleep well.",
    "Night night. Sleep well.",
    "Right - goodnight, and sleep well.",
    "Night. Sleep well, and I'll be here tomorrow.",
];

pub(super) const REASON_POOL: &[&str] = &[
    "Go on then — that comes before me. Take care.",
    "Fair enough, that matters more. Look after yourself.",
    "Of course. Go and see to it, and mind yourself.",
    "That first, definitely. Come back when you have a minute.",
];

pub(super) const TALK_AT_A_TIME: &[&str] = &[
    "I can't hold a time or remind you - no diary and no clock on this end. The chat doesn't mind when you come back, so start now or whenever suits: what's it about?",
    "Nothing here books a slot or rings you, so a time is the one thing I can't keep. What I can do is talk it through now and pick it up again when you come back. What did you want to say?",
    "There's no diary on this end, so I can't promise to be there at a set time - but nothing here closes either, so the thread keeps its place. What's on your mind?",
    "I can't put anything in a diary or set a reminder, and I would rather say that than nod along. What I can do is start now and keep the thread for whenever you come back. What's up?",
    "No clock here, so any hour is all the same to me, and the conversation stays where we leave it. What did you want to talk about?",
    "I can't save a slot or bring it up again, sorry - but the chat is here at any hour of the day and keeps its place. What's going on?",
];

pub(super) const ACKS: &[&str] = &[
    "cool", "exactly", "fair", "good", "great", "ha", "haha", "hmm", "indeed", "k", "lol", "mm",
    "nah", "nice", "no", "nope", "ok", "okay", "right", "same", "sure", "ta", "thanks", "thx",
    "true", "yeah", "yep", "yes",
];

pub(super) const FILLERS: &[&str] = &[
    "actually",
    "anyway",
    "basically",
    "even",
    "exactly",
    "honestly",
    "just",
    "literally",
    "maybe",
    "perhaps",
    "please",
    "really",
    "still",
];

pub(super) const HOW_ARE_YOU_ASKS: &[&str] = &[
    "how are you",
    "how are things",
    "how is things",
    "how's things",
    "how's your day",
    "how is your day",
    "your day going",
    "howz your day",
    "whats your day",
    "hows your day",
    "how's it going",
    "how is it going",
    "how have you been",
    "you doing",
    "you ok",
    "you okay",
    "you all right",
    "how do you do",
    "you up to",
    "up to much",
    "been up to",
];

pub(super) const HOW_ARE_YOU_GOING_ON: &[&str] = &["much going on", "going on"];

pub(super) const HOW_ARE_YOU_TAGS: &[&str] = &[
    "again",
    "all",
    "anyway",
    "at",
    "been",
    "both",
    "coping",
    "days",
    "doing",
    "everybody",
    "everyone",
    "far",
    "feeling",
    "folks",
    "friend",
    "go",
    "going",
    "gone",
    "guys",
    "hmm",
    "holding",
    "honestly",
    "keeping",
    "lately",
    "like",
    "lol",
    "looking",
    "love",
    "madam",
    "mate",
    "moment",
    "now",
    "recently",
    "sir",
    "so",
    "team",
    "thanks",
    "the",
    "then",
    "these",
    "though",
    "today",
    "tonight",
    "too",
    "up",
];

pub(super) const MANNERS_ONLY: &[&str] = &[
    "a",
    "about",
    "again",
    "also",
    "am",
    "an",
    "and",
    "another",
    "apologies",
    "apologise",
    "apologize",
    "apology",
    "ask",
    "asked",
    "asking",
    "at",
    "be",
    "been",
    "being",
    "bit",
    "bother",
    "bothered",
    "bothering",
    "but",
    "can",
    "could",
    "daft",
    "did",
    "do",
    "does",
    "dump",
    "dumping",
    "early",
    "essay",
    "for",
    "go",
    "goes",
    "going",
    "had",
    "hassle",
    "have",
    "honestly",
    "i",
    "im",
    "interrupt",
    "interrupting",
    "is",
    "it",
    "just",
    "keep",
    "keeping",
    "know",
    "knowing",
    "late",
    "lot",
    "me",
    "moan",
    "moaned",
    "moaning",
    "moans",
    "more",
    "my",
    "nag",
    "nagging",
    "nuisance",
    "of",
    "ok",
    "okay",
    "on",
    "one",
    "pain",
    "pester",
    "pestering",
    "please",
    "question",
    "questions",
    "quick",
    "quickly",
    "ramble",
    "rambling",
    "really",
    "silly",
    "slow",
    "so",
    "sorry",
    "stupid",
    "tbh",
    "thank",
    "thanks",
    "that",
    "the",
    "thing",
    "this",
    "time",
    "to",
    "trouble",
    "troubled",
    "troubling",
    "very",
    "was",
    "well",
    "with",
    "would",
    "you",
    "your",
];

pub(super) const THANK_WORDS: &[&str] = &[
    "appreciate",
    "appreciated",
    "cheers",
    "ta",
    "thank",
    "thanks",
    "thankyou",
];

pub(super) const THANKS_ONLY: &[&str] = &[
    "a",
    "about",
    "ace",
    "advice",
    "again",
    "all",
    "also",
    "am",
    "amazing",
    "an",
    "and",
    "another",
    "apologies",
    "apologise",
    "apologize",
    "apology",
    "ask",
    "asked",
    "asking",
    "at",
    "be",
    "been",
    "being",
    "bit",
    "bother",
    "bothered",
    "bothering",
    "brill",
    "brilliant",
    "but",
    "can",
    "could",
    "daft",
    "did",
    "do",
    "does",
    "dump",
    "dumping",
    "early",
    "essay",
    "everything",
    "excellent",
    "for",
    "go",
    "goes",
    "going",
    "good",
    "great",
    "had",
    "hassle",
    "have",
    "help",
    "helpful",
    "honestly",
    "i",
    "im",
    "interrupt",
    "interrupting",
    "is",
    "it",
    "just",
    "keep",
    "keeping",
    "kind",
    "kindly",
    "know",
    "knowing",
    "late",
    "legend",
    "lifesaver",
    "lot",
    "lovely",
    "many",
    "me",
    "moan",
    "moaned",
    "moaning",
    "moans",
    "more",
    "much",
    "my",
    "nag",
    "nagging",
    "nice",
    "nuisance",
    "of",
    "ok",
    "okay",
    "on",
    "one",
    "pain",
    "perfect",
    "pester",
    "pestering",
    "please",
    "question",
    "questions",
    "quick",
    "quickly",
    "ramble",
    "rambling",
    "really",
    "right",
    "silly",
    "slow",
    "so",
    "sorry",
    "sorted",
    "star",
    "stupid",
    "tbh",
    "thank",
    "thanks",
    "that",
    "that's",
    "thats",
    "the",
    "then",
    "thing",
    "this",
    "time",
    "to",
    "too",
    "trouble",
    "troubled",
    "troubling",
    "useful",
    "very",
    "was",
    "well",
    "will",
    "with",
    "wonderful",
    "would",
    "you",
    "your",
];

pub(super) const IDENTITY: &[&str] = &[
    "are you a real person",
    "are you real",
    "are you human",
    "are you a human",
    "are you a bot",
    "are you a robot",
    "are you an ai",
    "are you ai",
    "who are you",
    "what is a chatbot",
    "are you a chatbot",
    "introduce yourself",
];

pub(super) const MEMORY: &[&str] = &[
    "do you remember",
    "can you remember",
    "do you know what we talked",
    "what did we talk about",
    "remember what we",
    "remember me",
    "remember us",
];

pub(super) const HELP: &[&str] = &[
    "what can you help",
    "what can you do",
    "how can you help",
    "what do you do",
    "what kind of things can you help",
    "what are you for",
    "how do you help",
    "what can i ask you",
    "what are you good at",
];

pub(super) const HELP_TAIL: &[&str] = &[
    "about", "and", "anyway", "around", "at", "for", "here", "honestly", "it", "me", "now",
    "please", "really", "so", "thanks", "then", "there", "though", "to", "us", "with",
];

pub(super) const QUESTION_WORDS: &[&str] = &[
    "are", "can", "could", "did", "do", "does", "has", "have", "how", "is", "should", "what",
    "when", "where", "which", "who", "why", "will", "would",
];

pub(super) const A_POINTING_FILLER: &[&str] = &[
    "again", "ah", "already", "and", "any", "anyroad", "anyway", "as", "at", "back", "before",
    "but", "carry", "earlier", "er", "for", "from", "haha", "here", "hmm", "i", "i'm", "im", "in",
    "it", "just", "like", "me", "my", "now", "of", "off", "oh", "ok", "okay", "on", "right",
    "said", "saying", "so", "sorry", "that", "the", "then", "there", "this", "to", "um", "up",
    "us", "was", "we", "well", "were", "with", "yeah", "yes", "you",
];

pub(super) const A_THING_MARKER: &[&str] = &["about", "regarding"];

pub(super) const PRODUCT_WORDS: &[&str] = &[
    "account",
    "android",
    "app",
    "apps",
    "bicycle",
    "bike",
    "boiler",
    "broadband",
    "browser",
    "camera",
    "car",
    "chain",
    "chains",
    "clock",
    "computer",
    "cooker",
    "cycle",
    "derailleur",
    "dishwasher",
    "door",
    "drill",
    "dryer",
    "dryers",
    "email",
    "excel",
    "facebook",
    "fan",
    "freezer",
    "fridge",
    "gate",
    "gear",
    "gears",
    "gmail",
    "gutter",
    "headphones",
    "hob",
    "hoover",
    "instagram",
    "ios",
    "ipad",
    "iphone",
    "kettle",
    "laptop",
    "lawnmower",
    "mac",
    "machine",
    "microwave",
    "motorbike",
    "mower",
    "outlook",
    "oven",
    "password",
    "pc",
    "phone",
    "powerpoint",
    "printer",
    "pump",
    "radiator",
    "roof",
    "router",
    "saw",
    "scooter",
    "settings",
    "shed",
    "shower",
    "speaker",
    "speakers",
    "stereo",
    "tablet",
    "tap",
    "television",
    "tiktok",
    "toaster",
    "tv",
    "vacuum",
    "van",
    "washer",
    "watch",
    "whatsapp",
    "wifi",
    "window",
    "windows",
    "word",
];

pub(super) const POINTS_AT_NOTHING: &[&str] = &[
    "anything",
    "everything",
    "her",
    "him",
    "it",
    "it all",
    "me",
    "nothing",
    "something",
    "stuff",
    "that",
    "the lot",
    "them",
    "there",
    "they",
    "thing",
    "things",
    "this",
    "us",
    "you",
];

pub(super) const HANDOFF_NOT_SUBJECT: &[&str] = &[
    "and", "because", "but", "how", "if", "so", "that", "what", "when", "where", "whether",
    "which", "who", "whom", "why",
];

pub(super) const GREETINGS: &[&str] = &[
    "hi",
    "hello",
    "hey",
    "hiya",
    "morning",
    "good morning",
    "good afternoon",
    "good evening",
    "evening",
    "yo",
    "howdy",
    "good day",
    "greetings",
    "salutations",
];

pub(super) const GREETING_TAIL: &[&str] = &[
    "again",
    "all",
    "alright",
    "also",
    "are",
    "back",
    "day",
    "doing",
    "everybody",
    "everyone",
    "fine",
    "folks",
    "friends",
    "going",
    "good",
    "guys",
    "here",
    "hope",
    "hopes",
    "hoping",
    "how",
    "is",
    "it",
    "just",
    "life",
    "mate",
    "ok",
    "okay",
    "question",
    "quick",
    "so",
    "sup",
    "thank",
    "thanks",
    "then",
    "there",
    "things",
    "today",
    "too",
    "up",
    "well",
    "you",
    "your",
];

pub(super) const FAREWELLS: &[&str] = &[
    "bye",
    "goodbye",
    "see you",
    "see ya",
    "later",
    "night",
    "good night",
    "talk later",
    "take care",
    "ttyl",
    "gtg",
    "cya",
    "best wishes",
    "best regards",
    "kind regards",
    "warm regards",
    "ttfn",
    "cheerio",
    "laters",
    "adios",
    "night night",
    "nighty night",
    "ta ta",
    "see you later",
    "see you soon",
    "talk to you later",
    "speak later",
    "speak to you later",
    "catch you later",
    "bye for now",
];

pub(super) const FAREWELL_TAIL: &[&str] = &[
    "all",
    "everybody",
    "everyone",
    "folks",
    "for",
    "good",
    "guys",
    "hun",
    "later",
    "love",
    "mates",
    "night",
    "now",
    "ok",
    "okay",
    "right",
    "so",
    "soon",
    "then",
    "to",
    "tomorrow",
    "tonight",
    "well",
    "xx",
    "xxx",
];

pub(super) const THANKS_PHRASES: &[&str] = &[
    "thanks",
    "thank you",
    "thx",
    "cheers",
    "ty",
    "ta",
    "much appreciated",
    "appreciate",
    "appreciated",
    "appreciate it",
];

pub(super) const THANKS_ABOUT: &[&str] = &[
    "it", "that", "them", "these", "they", "this", "those", "which",
];

pub(super) const NEEDS_ANSWER: &[&str] = &[
    "ache", "aches", "aching", "agony", "arent", "awful", "bled", "bleeding", "breaks", "broke",
    "broken", "burning", "cannot", "cant", "dead", "didnt", "die", "died", "doesnt", "dont",
    "hurt", "hurting", "hurts", "isnt", "itches", "keeps", "kept", "killed", "killing", "leak",
    "leaking", "leaks", "never", "noise", "noisy", "not", "pain", "painful", "problem", "problems",
    "rash", "rubbish", "scared", "sick", "slow", "smells", "sore", "stings", "stopped", "stops",
    "stuck", "swollen", "terrible", "unwell", "wasnt", "weren", "wont", "worried", "worrying",
    "worse", "worst", "wrong",
];

pub(super) const POLITE: &[&str] = &["mate", "please", "pls", "plz", "ta"];

pub(super) const ADDRESSED: &[&str] = &[
    "babes", "buddy", "chuck", "dear", "duck", "flower", "hen", "hun", "love", "lovey", "mate",
    "pal", "pet", "shug", "sunshine",
];

pub(super) const GREETING_ASK_WORDS: &[&str] = &[
    "all",
    "alright",
    "am",
    "anyway",
    "are",
    "be",
    "been",
    "day",
    "days",
    "do",
    "doin",
    "doing",
    "evening",
    "everything",
    "fine",
    "friend",
    "getting",
    "going",
    "good",
    "has",
    "have",
    "having",
    "how",
    "is",
    "it",
    "keeping",
    "lately",
    "life",
    "love",
    "mate",
    "morning",
    "much",
    "night",
    "now",
    "ok",
    "okay",
    "r",
    "say",
    "saying",
    "so",
    "the",
    "then",
    "things",
    "today",
    "tonight",
    "too",
    "u",
    "was",
    "week",
    "weekend",
    "well",
    "were",
    "what",
    "whats",
    "ya",
    "you",
    "your",
    "yours",
    "yourself",
    "yourselves",
];

pub(super) const GREETING_ASK_SUBJECT: &[&str] = &[
    "day",
    "days",
    "evening",
    "everything",
    "life",
    "morning",
    "night",
    "things",
    "u",
    "week",
    "weekend",
    "ya",
    "you",
    "your",
    "yours",
    "yourself",
    "yourselves",
];

pub(super) const GREETING_ASK_STATE: &[&str] = &["alright", "fine", "good", "ok", "okay", "well"];

pub(super) const GREETING_ASK_ABOUT_SOMETHING: &[&str] =
    &["about", "at", "for", "if", "in", "on", "to", "when", "with"];

pub(super) const GREETING_ASK_SECOND_PERSON: &[&str] =
    &["u", "ya", "you", "your", "yours", "yourself", "yourselves"];

pub(super) const GREETING_WHAT_SAYING: &[&str] = &[
    "all", "are", "is", "mate", "now", "r", "say", "sayin", "saying", "then", "there", "u", "what",
    "ya", "you",
];

pub(super) const HELLO_TAIL: &[&str] = &[
    "all",
    "everybody",
    "everyone",
    "folks",
    "friends",
    "guys",
    "mate",
    "there",
    "to",
    "you",
];

pub(super) const SUBJECT_SAID: &[&str] = &[
    "everybody",
    "everyone",
    "folk",
    "he",
    "i",
    "it",
    "nobody",
    "people",
    "she",
    "somebody",
    "they",
    "u",
    "we",
    "ya",
    "you",
    "yous",
];

pub(super) const LEAVING_ALLOWED: &[&str] = &[
    "a",
    "about",
    "alright",
    "an",
    "and",
    "anyway",
    "anyways",
    "bed",
    "bit",
    "but",
    "day",
    "early",
    "folks",
    "for",
    "gonna",
    "hey",
    "hi",
    "home",
    "honey",
    "hun",
    "i",
    "i'd",
    "i'll",
    "i'm",
    "i've",
    "id",
    "ill",
    "im",
    "it",
    "it's",
    "just",
    "kip",
    "later",
    "let",
    "listen",
    "little",
    "look",
    "love",
    "luv",
    "mate",
    "me",
    "mhm",
    "minute",
    "mm",
    "morning",
    "my",
    "nap",
    "night",
    "nite",
    "now",
    "off",
    "oh",
    "ok",
    "okay",
    "one",
    "right",
    "sec",
    "second",
    "shop",
    "should",
    "sleep",
    "so",
    "soon",
    "sorry",
    "sweetheart",
    "ta",
    "thank",
    "thanks",
    "that",
    "that's",
    "the",
    "then",
    "think",
    "to",
    "today",
    "tomorrow",
    "tonight",
    "us",
    "we",
    "we'd",
    "we'll",
    "we're",
    "well",
    "while",
    "work",
    "x",
    "xx",
    "xxx",
    "yeah",
    "yes",
    "you",
];

pub(super) const SIGN_OFF_WORDS: &[&str] = &[
    "adios",
    "alright",
    "and",
    "anyway",
    "anyways",
    "bye",
    "byebye",
    "cheerio",
    "cheers",
    "goodbye",
    "goodnight",
    "later",
    "laters",
    "laterz",
    "night",
    "nighty",
    "nite",
    "nites",
    "now",
    "ok",
    "okay",
    "right",
    "so",
    "ta",
    "tara",
    "thank",
    "thanks",
    "then",
    "ttfn",
    "ttyl",
    "well",
    "you",
];

pub(super) const SIGN_OFF_VOCATIVE: &[&str] = &[
    "all",
    "bella",
    "chick",
    "duck",
    "everybody",
    "everyone",
    "folks",
    "guys",
    "honey",
    "hun",
    "love",
    "luv",
    "mate",
    "pal",
    "pet",
    "sweetheart",
    "too",
    "x",
    "xx",
    "xxx",
];

pub(super) const LEAVE_SHORTHAND: &[&str] = &[
    "cya",
    "g2g",
    "gtg",
    "l8r",
    "l8tr",
    "n8",
    "talklater",
    "ttyl",
    "xx",
    "xxx",
];

pub(super) const LEAVING_THANKS: &[&str] = &[
    "a",
    "advice",
    "again",
    "all",
    "and",
    "appreciate",
    "chat",
    "everything",
    "for",
    "help",
    "helped",
    "helps",
    "i",
    "it",
    "listened",
    "listening",
    "lot",
    "mate",
    "means",
    "much",
    "really",
    "so",
    "talk",
    "talking",
    "thank",
    "thanks",
    "that",
    "the",
    "useful",
    "was",
    "you",
];

pub(super) const ABOUT_IDENTITY: &[&str] = &[
    "I'm a chatbot, not a person: a program built to help with everyday questions and have a chat. What can I do for you?",
    "I'm a chatbot — here to help and chat. Ask me anything, or just talk.",
];

pub(super) const ABOUT_MEMORY: &[&str] = &[
    "I remember what we've said in this chat, but nothing from before it — so not yesterday, sorry. What was it about?",
    "Only this conversation: I keep what you tell me here, like your name and plans, but nothing carries over to a new chat. What were we talking about?",
    "What we say in this chat stays here for as long as we are talking, so ask me again later in it and I still have it. A new conversation starts empty, every time.",
];

pub(super) const ABOUT_PRIVACY: &[&str] = &[
    "Nothing you type is written down or sent anywhere - what you tell me is used here, in this conversation, and nowhere else. No account, no log, and nobody else reading it.",
    "Private to this chat. I keep no file of it and I pass nothing on; what you say stays here while we talk, so I can follow it, and that is all.",
    "Nothing leaves this conversation: no database, no record, nobody else seeing it. I hold what you say only for as long as we are talking.",
];

pub(super) const ABOUT_TRUST: &[&str] = &[
    "Trust me the way you would a well-read friend: good on the everyday, worth a check on the things that cost money or matter to your health. I say so when I am unsure, and anything I give you a number for is worth a second look. Which is it?",
    "You can, with a limit: I am a program working from what is written into me, not someone who checked today's prices or read your test results. Everyday questions, yes; a dose, a deadline or a legal thing, check it. What are you weighing up?",
];

pub(super) const ABOUT_SECRET: &[&str] = &[
    "Yes - nothing you type is written down or sent anywhere, so it stays between us. What is it? The one thing I can't do is act on it: if somebody needs help in the real world, tell a person who can give it.",
    "Kept: no record, no log, nothing passed on. Go on. Anything that needs doing rather than saying, though, take to someone with hands - me knowing about it is not the same as it being dealt with.",
    "Nothing you tell me leaves this chat, so a secret is safe here. What's on your mind? If it is the kind of thing somebody should know about, I'd rather you told them as well.",
];

pub(super) const ABOUT_JUDGING: &[&str] = &[
    "Not judging, no. Say it plainly and we'll look at it together.",
    "No judgement here - whatever it is, half of it usually makes sense once it is said out loud. Tell me what happened.",
    "Nothing you tell me gets graded. Tell me what's going on and we'll take it from there.",
];

pub(super) const ABOUT_HELP: &[&str] = &[
    "Everyday things: cooking, travel, money, tech, health questions, plans, and a chat when you want one. What's on your mind?",
    "Cooking, travel, money, work, gadgets, kids, feelings — everyday questions and plans. Ask away.",
];

pub(super) const ABOUT_PURPOSE: &[&str] = &[
    "To be useful with the ordinary things — a question, a plan, what to cook, what to say to someone. That's the whole of it, and I'd rather do it well than sound clever.",
    "No grand reason. Someone built a thing that helps with everyday questions without a search engine, and this is it. What did you want a hand with?",
];

pub(super) const ABOUT_MODEL: &[&str] = &[
    "No model, nothing like one. I'm a program of written rules rather than a trained network: what I say comes from the everyday things people put in the program and from what you have told me here.",
    "There's no model under this, and nothing learned. I'm hand-written rules for ordinary questions - no network, no training data, and nothing fetched while we talk.",
    "None that would mean anything to you: no neural network, no weights, no training. Just a lot of carefully written rules about everyday things, run on the words you send.",
];

pub(super) const ABOUT_WORKINGS: &[&str] = &[
    "No network under this at all: a lot of written rules and a store of everyday things, and I work it out from the words you send. Ask me something ordinary - what to cook, what to say to someone - and you will see what it can do.",
    "I am not a trained model: everything I know was written into the program before we met, and what I do is match your line against it and reason from there. No learning as we talk, and nothing fetched while we do.",
];

pub(super) const ABOUT_ONLINE: &[&str] = &[
    "Nothing live on this end, no: I can't open a page, search, or check anything while we talk. What I have is a lot of everyday things written into me - if it's not in there, say so and I'll tell you straight.",
    "No connection at all. No browsing, no searching, no reading the news - just what I was built with and what you tell me. What did you want looked up?",
];

pub(super) const ABOUT_LIMITS: &[&str] = &[
    "Plenty. Nothing live: no news, no prices, no weather, no clock, and no page I can open. I can't remember a chat from another day either. Everyday things I'm built for: what to cook, what to say, how to get something done.",
    "I can't see anything happen, and I can't look anything up - what I know was written into me before we met, plus whatever you tell me here. Where that runs out I'd rather say so than guess.",
    "Live things, new things, and sums too big to do in my head. Ask me about ordinary days and I'm on firmer ground.",
];

pub(super) const HELLO_SAID_SHORT: &[(&str, &str)] = &[
    ("top of the morning", "good morning"),
    ("g morning", "good morning"),
    ("gmorning", "good morning"),
    ("g'morning", "good morning"),
    ("gm", "good morning"),
    ("gday", "good day"),
    ("g'day", "good day"),
    ("mornin'", "morning"),
    ("mornin", "morning"),
    ("evenin'", "evening"),
    ("evenin", "evening"),
];

pub(super) const SAID_OUT: &[(&str, &str)] = &[
    ("&", "and"),
    ("2day", "today"),
    ("2mora", "tomorrow"),
    ("2moro", "tomorrow"),
    ("2morra", "tomorrow"),
    ("2morro", "tomorrow"),
    ("2morrow", "tomorrow"),
    ("2moz", "tomorrow"),
    ("2mz", "tomorrow"),
    ("2nit", "tonight"),
    ("2nite", "tonight"),
    ("abt", "about"),
    ("abt.", "about"),
    ("abwt", "about"),
    ("agen", "again"),
    ("aight", "alright"),
    ("alrite", "alright"),
    ("altho", "although"),
    ("any1", "anyone"),
    ("anyfin", "anything"),
    ("anyfink", "anything"),
    ("appt", "appointment"),
    ("appts", "appointments"),
    ("awright", "alright"),
    ("awrite", "alright"),
    ("aye", "yes"),
    ("b'day", "birthday"),
    ("b-day", "birthday"),
    ("b/c", "because"),
    ("b4", "before"),
    ("bairn", "child"),
    ("bairns", "children"),
    ("bc", "because"),
    ("bcoz", "because"),
    ("bday", "birthday"),
    ("bdays", "birthdays"),
    ("becos", "because"),
    ("becoz", "because"),
    ("bf", "boyfriend"),
    ("bfs", "boyfriends"),
    ("biccies", "biscuits"),
    ("biccy", "biscuit"),
    ("bikkie", "biscuit"),
    ("bikkies", "biscuits"),
    ("bort", "bought"),
    ("botherd", "bothered"),
    ("bova", "bothered"),
    ("bovad", "bothered"),
    ("bovered", "bothered"),
    ("brekkie", "breakfast"),
    ("brekky", "breakfast"),
    ("brort", "brought"),
    ("bubba", "baby"),
    ("buhbye", "bye"),
    ("butties", "sandwiches"),
    ("butty", "sandwich"),
    ("bv", "have"),
    ("bwt", "about"),
    ("caus", "because"),
    ("cba", "can not be bothered"),
    ("cbf", "can not be bothered"),
    ("cheeers", "thanks"),
    ("cheers", "thanks"),
    ("cheerz", "thanks"),
    ("chek", "check"),
    ("chekcing", "checking"),
    ("choccies", "chocolates"),
    ("choccy", "chocolate"),
    ("cn", "can"),
    ("coud", "could"),
    ("coulda", "could have"),
    ("coz", "because"),
    ("crabbit", "grumpy"),
    ("crimbo", "christmas"),
    ("cud", "could"),
    ("culd", "could"),
    ("cupple", "couple"),
    ("cupples", "couples"),
    ("cuz", "because"),
    ("dat", "that"),
    ("decs", "decorations"),
    ("decs.", "decorations"),
    ("def", "definitely"),
    ("defo", "definitely"),
    ("dem", "them"),
    ("dere", "there"),
    ("dey", "they"),
    ("dis", "this"),
    ("dm", "message"),
    ("drs", "doctors"),
    ("dunno", "i do not know"),
    ("duz", "does"),
    ("elp", "help"),
    ("elped", "helped"),
    ("elping", "helping"),
    ("elps", "helps"),
    ("eny", "any"),
    ("enything", "anything"),
    ("every1", "everyone"),
    ("eveything", "everything"),
    ("evry1", "everyone"),
    ("evrythin", "everything"),
    ("fanks", "thanks"),
    ("fone", "phone"),
    ("fones", "phones"),
    ("forgetin", "forgetting"),
    ("forgettin", "forgetting"),
    ("gd", "good"),
    ("gewd", "good"),
    ("gf", "girlfriend"),
    ("gfs", "girlfriends"),
    ("gimme", "give me"),
    ("giv", "give"),
    ("givin", "giving"),
    ("gna", "going to"),
    ("goina", "going to"),
    ("gona", "going to"),
    ("gonig", "going"),
    ("gonna", "going to"),
    ("gota", "got to"),
    ("gotta", "got to"),
    ("gr8", "great"),
    ("gubbed", "shattered"),
    ("gud", "good"),
    ("guna", "going to"),
    ("gunna", "going to"),
    ("h8", "hate"),
    ("h8d", "hated"),
    ("h8ing", "hating"),
    ("h8s", "hates"),
    ("hackit", "rough"),
    ("hafta", "have to"),
    ("haha", "haha"),
    ("hai", "hi"),
    ("hasta", "have to"),
    ("hav", "have"),
    ("haz", "has"),
    ("hehe", "hehe"),
    ("heyoo", "hey"),
    ("hihi", "hi"),
    ("hol", "holiday"),
    ("hols", "holidays"),
    ("hosp", "hospital"),
    ("hrs", "hours"),
    ("hu", "who"),
    ("hubby", "husband"),
    ("hve", "have"),
    ("hw", "how"),
    ("hwo", "how"),
    ("ideea", "idea"),
    ("ideeas", "ideas"),
    ("idk", "i do not know"),
    ("jst", "just"),
    ("kay", "ok"),
    ("kinda", "kind of"),
    ("kk", "ok"),
    ("kno", "know"),
    ("knos", "knows"),
    ("knw", "know"),
    ("knwo", "know"),
    ("l8", "late"),
    ("l8ly", "lately"),
    ("l8r", "later"),
    ("lemme", "let me"),
    ("lic", "licence"),
    ("lik", "like"),
    ("lmao", "lol"),
    ("lol", "lol"),
    ("luv", "love"),
    ("luvs", "loves"),
    ("lv", "love"),
    ("lyk", "like"),
    ("m8", "mate"),
    ("m8s", "mates"),
    ("mayb", "maybe"),
    ("me self", "myself"),
    ("meds", "medicines"),
    ("ment", "meant"),
    ("meself", "myself"),
    ("mesen", "myself"),
    ("mins", "minutes"),
    ("missus", "wife"),
    ("mnth", "month"),
    ("mnths", "months"),
    ("morngin", "morning"),
    ("mornging", "morning"),
    ("mornign", "morning"),
    ("mornin", "morning"),
    ("msg", "message"),
    ("msgs", "messages"),
    ("mth", "month"),
    ("mths", "months"),
    ("musta", "must have"),
    ("n8", "night"),
    ("nd", "and"),
    ("ne1", "anyone"),
    ("nethin", "anything"),
    ("nething", "anything"),
    ("nighto", "night"),
    ("nite", "night"),
    ("nope", "no"),
    ("normaly", "normally"),
    ("nowt", "nothing"),
    ("nt", "not"),
    ("nuffin", "nothing"),
    ("nuffink", "nothing"),
    ("nuthin", "nothing"),
    ("nxt", "next"),
    ("okey", "ok"),
    ("okey dokey", "ok"),
    ("okie", "ok"),
    ("outta", "out of"),
    ("ovr", "over"),
    ("owta", "out of"),
    ("owtside", "outside"),
    ("peace out", "bye"),
    ("phne", "phone"),
    ("pic", "picture"),
    ("pics", "pictures"),
    ("pleas", "please"),
    ("plis", "please"),
    ("pls", "please"),
    ("plss", "please"),
    ("plz", "please"),
    ("ppls", "people's"),
    ("presant", "present"),
    ("presnt", "present"),
    ("probs", "probably"),
    ("prolly", "probably"),
    ("propa", "proper"),
    ("quik", "quick"),
    ("qwik", "quick"),
    ("recive", "receive"),
    ("reckn", "reckon"),
    ("recknd", "reckoned"),
    ("reckning", "reckoning"),
    ("reckns", "reckons"),
    ("rite", "right"),
    ("rlly", "really"),
    ("rly", "really"),
    ("rofl", "lol"),
    ("ropey", "unwell"),
    ("ropy", "unwell"),
    ("sarnie", "sandwich"),
    ("sarnies", "sandwiches"),
    ("secs", "seconds"),
    ("seeya", "see you"),
    ("shd", "should"),
    ("shld", "should"),
    ("shood", "should"),
    ("shoppin", "shopping"),
    ("shoulda", "should have"),
    ("shouldve", "should have"),
    ("shud", "should"),
    ("shuld", "should"),
    ("skl", "school"),
    ("slee", "sleep"),
    ("some1", "someone"),
    ("somet", "something"),
    ("sommat", "something"),
    ("sorta", "sort of"),
    ("speek", "speak"),
    ("spk", "speak"),
    ("spose", "suppose"),
    ("srry", "sorry"),
    ("srsly", "seriously"),
    ("sry", "sorry"),
    ("stomakaike", "stomach ache"),
    ("stuf", "stuff"),
    ("styff", "stuff"),
    ("sum1", "someone"),
    ("sumfink", "something"),
    ("sumin", "something"),
    ("sumink", "something"),
    ("summat", "something"),
    ("summin", "something"),
    ("summink", "something"),
    ("sumthing", "something"),
    ("syas", "says"),
    ("ta", "thanks"),
    ("tahnks", "thanks"),
    ("tday", "today"),
    ("thanku", "thanks"),
    ("thankx", "thanks"),
    ("thankyou", "thanks"),
    ("thankz", "thanks"),
    ("thanx", "thanks"),
    ("thanxs", "thanks"),
    ("thks", "thanks"),
    ("thnaks", "thanks"),
    ("thnks", "thanks"),
    ("thnkx", "thanks"),
    ("thnx", "thanks"),
    ("thnxs", "thanks"),
    ("tho'", "though"),
    ("thort", "thought"),
    ("thru", "through"),
    ("thruout", "throughout"),
    ("thx", "thanks"),
    ("thxs", "thanks"),
    ("tks", "thanks"),
    ("tkx", "thanks"),
    ("tmoro", "tomorrow"),
    ("tmorro", "tomorrow"),
    ("tmoz", "tomorrow"),
    ("tmr", "tomorrow"),
    ("tmrow", "tomorrow"),
    ("tmrw", "tomorrow"),
    ("tmw", "tomorrow"),
    ("tn", "tonight"),
    ("tnight", "tonight"),
    ("tnite", "tonight"),
    ("tnks", "thanks"),
    ("tnx", "thanks"),
    ("todai", "today"),
    ("tommorow", "tomorrow"),
    ("tomo", "tomorrow"),
    ("tomoro", "tomorrow"),
    ("tomoz", "tomorrow"),
    ("tonit", "tonight"),
    ("tonite", "tonight"),
    ("tonyt", "tonight"),
    ("toodles", "bye"),
    ("tryna", "trying to"),
    ("trynna", "trying to"),
    ("txt", "text"),
    ("txts", "texts"),
    ("ty", "thanks"),
    ("tysm", "thanks"),
    ("tyty", "thanks"),
    ("urt", "hurt"),
    ("urted", "hurt"),
    ("urting", "hurting"),
    ("urts", "hurts"),
    ("usualy", "usually"),
    ("w/", "with"),
    ("w/o", "without"),
    ("w8", "wait"),
    ("wana", "want to"),
    ("wanna", "want to"),
    ("wanne", "want to"),
    ("wannna", "want to"),
    ("wanta", "want to"),
    ("wat", "what"),
    ("wdym", "what do you mean"),
    ("wdyt", "what do you think"),
    ("weerd", "weird"),
    ("wen", "when"),
    ("wens", "when"),
    ("werk", "work"),
    ("werked", "worked"),
    ("werking", "working"),
    ("werks", "works"),
    ("whatre", "what are"),
    ("wher", "where"),
    ("wht", "what"),
    ("whut", "what"),
    ("wid", "with"),
    ("widout", "without"),
    ("wiff", "with"),
    ("wiv", "with"),
    ("wk", "week"),
    ("wknd", "weekend"),
    ("wknds", "weekends"),
    ("wks", "weeks"),
    ("wnt", "want"),
    ("wot", "what"),
    ("woulda", "would have"),
    ("woz", "was"),
    ("wt", "what"),
    ("wu2", "what you up to"),
    ("wud", "would"),
    ("wuld", "would"),
    ("wut", "what"),
    ("wuu", "what you up to"),
    ("wuu2", "what you up to"),
    ("wuz", "was"),
    ("wym", "what do you mean"),
    ("x-mas", "christmas"),
    ("xmas", "christmas"),
    ("ya", "you"),
    ("yah", "yes"),
    ("yday", "yesterday"),
    ("yea", "yes"),
    ("yeh", "yes"),
    ("yep", "yes"),
    ("yestday", "yesterday"),
    ("yo", "hi"),
    ("yoo", "hi"),
    ("ystday", "yesterday"),
];

pub(super) const WORD_FIXES: &[(&str, &str)] = &[
    ("cn", "can"),
    ("cud", "could"),
    ("hows", "how"),
    ("wat", "what"),
    ("wats", "what"),
    ("what", "what"),
    ("whats", "what"),
    ("whens", "when"),
    ("wheres", "where"),
    ("whos", "who"),
    ("whys", "why"),
    ("wot", "what"),
];

use regex::Regex;
use std::sync::LazyLock;

macro_rules! regex {
    ($name:ident, $value:expr) => {
        pub(super) static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

regex!(
    HAND_ASK,
    r##"^(?:so |ok |okay |right |hi |hiya |hello |hey |and |please )*(?:(?:can|could|will|would) (?:you|u) (?:please )?(?:help(?: (?:me|us))?(?: out)?|give (?:me|us) a hand|lend (?:me|us) a hand|do (?:me|us) a favour|advise (?:me|us)|be a hand)|(?:i )?(?:could do with|need|want|would like) (?:(?:your|some|a bit of) )?(?:help|a hand|advice|a bit of advice)|(?:can|could) (?:i|we) (?:get|have|ask for) (?:(?:your|some|a little|a bit of) )?(?:help|a hand|advice)|help(?: me)?(?: out)?|give me a hand)(?: (?:with|about) (?:something|anything|a thing|stuff|it))?(?: (?:here|please|now|today|tonight|asap|urgently|quick(?:ly)?|straight away))?(?: (?:right )?now)?(?: please)?$"##
);
regex!(
    GREETING_OPEN,
    r##"(?i)^\s*(?:(?:nice|good|great|pleased|lovely|glad|happy|grand)\s+to\s+(?:meet|see|hear\s+from)\s+(?:you|u|ya)\b|(?:good|great|lovely|nice)\s+to\s+have\s+you\b|how\s+(?:are|r)\s+(?:you|u|ya)\b|how(?:'s|s|\s+is)\s+(?:it\s+going|things|tricks|you\s+doing|your\s+day)\b|how\s+do\s+you\s+do\b|long\s+time\s+no\s+see\b)"##
);
regex!(
    YOUR_DAY_ASK,
    r##"\b(?:are|were|have|had|did|do|does)\s+(?:you|u)\s+(?:been\s+)?(?:having|had|have)\s+(?:a\s+|an\s+)?(?:good|nice|great|lovely|ok|okay|quiet|crap|bad)?\s*(?:day|days|weekend|evening|night|morning|afternoon)\b"##
);
regex!(
    YOUR_TIME_ASK,
    r##"\b(?:what|whats)\s+(?:have|has|ve)\s+(?:you|u)\s+been\s+(?:up\s+to|doing|at)\b|\b(?:you|u)\s+been\s+(?:up\s+to|doing)\b"##
);
regex!(
    THANKS_OPENS,
    r##"^(?:(?:ok|okay|ah|oh|right|great|brilliant|nice|lovely|perfect)\b[\s,!.]*)?(?:thanks|thank\s+you|thankyou|thank-you|cheers|ta)\b[\s,!.]*(?P<said>.*)$"##
);
regex!(
    THANKS_WENT_BADLY,
    r##"\b(?:worse|worst|no\s+better|not\s+better|no\s+change|still|came\s+back|back\s+again|hurts?|hurting|sorer?|failed|nothing|didnt|didn't|doesnt|doesn't|wont|won't|can'?t|cant|hasnt|hasn't|(?:is|are|was|were|has|have|had|its|it's|im|i'm|they're|theyre)\s+back|(?:come|comes|coming|came)\s+back|flares?|flared|flaring|\breturned|returning|back\s+to\s+square\s+one)\b"##
);
regex!(
    THANKS_GOOD_AGAIN,
    r##"\bback\s+to\s+(?:normal|how\s+it\s+was|where\s+it\s+was|itself|its\s+old\s+self|being\s+normal)\b|\bstill\s+(?:keeping\s+|doing\s+|going\s+)?(?:good|well|fine|great|ok|okay|improving|getting\s+better|on\s+the\s+mend|strong)\b|\bstill\s+(?:a\s+)?bit\s+(?:sore|stiff|tired)\s+but\b"##
);
regex!(
    THANKS_TAKES_A_QUESTION,
    r##"\b(?:can|could|would|should|will|shall|any\s+idea|what|how|why|when|where|who|is\s+it|do\s+i|should\s+i)\b"##
);
regex!(
    ACKNOWLEDGES_ALONE,
    r##"^(?:(?:oh|ok|okay|right|well|ah|aye|yes|yeah|great|good|nice|cool|brilliant|lovely|perfect|interesting|thats|that's|that|it's|its|thanks|thank\s+you|cheers|ta|ill|i'll|i|will|do)[\s,]+)*(?:(?:good|great|useful|handy|nice|lovely|brilliant|perfect|helpful|interesting|reassuring|worth)\s+)?(?:to\s+know|knowing|to\s+hear|to\s+see)\b(?:[\s,]+(?:that|it|this|then|so|now|thanks|thank\s+you|cheers|ta|mate|for\s+that))*[\s.!]*$"##
);
regex!(
    SORRY_ASKING,
    r##"\b(?:sorry|apolog(?:y|ies)|apologise|apologize)\b[^.?!]{0,30}?\b(?:bother\w*|pester\w*|nagging|ask\w*|questions?|pain|nuisance|hassle|trouble\w*|keep\s+(?:asking|going|coming|picking|on)|one\s+more|another\s+one|essay|ramble\w*|going\s+on|dumping|moan\w*|interrupt\w*|taking\s+up)\b"##
);
regex!(
    FLAT_LINE,
    r##"^\W*(?:(?:yeah|yep|yes|ok|okay|and|but|oh|ah|well|honestly|tbh|just|so|still|really|very|it|its|is|was|been|a|bit|pretty|kind of|sort of)\s+)*(?:ugh+|urgh+|blah+|bleh+|bleurgh+|blech+|sigh|fr|ngl|pfff+|same old(?: same old)?|same as (?:ever|always)|could be worse|cant complain|up and down|so so|bit of a rubbish|bit rubbish)\W*$"##
);
regex!(
    REACTION_LINE,
    r##"^\W*(?:(?:oh|ah|ooh|ooh|aw|aww|aah|my|good|god|jesus|christ|jeez|well|just|so|and|but|ok|okay|bloody|hell|damn|fucking)\s+)*(?:(?:ouch(?:ie)?s?|ow+|yikes|eek|oof|yowch|sucks|brutal|phew|finally|oops|whoops|dear|dammit|damn|bugger|crap|shoot|huh|eh|pardon|what|seriously|serious|joking|kidding|real|way|chance|surely|really|honestly|wtf|the|hell|heck|fuck|get|out|come|on|wow+|woah|whoa|omg|what on earth|why on earth|how on earth|what in the world|why in the world|devil|blazes|dickens|damn|fucking|going|goes|happen|happens|happened|happening|you|youre|are|joke|for|blimey|crikey|thank|good|god|heavens|christ|my|word|that|this|it|is|so|just|awful|terrible|horrible|rough|much|sick|the|no))(?:[\s,]+(?:and\s+)?(?:ouch(?:ie)?s?|ow+|yikes|eek|oof|yowch|sucks|brutal|phew|finally|oops|whoops|dear|dammit|damn|bugger|crap|shoot|huh|eh|pardon|what|seriously|serious|joking|kidding|real|way|chance|surely|really|honestly|wtf|the|hell|heck|fuck|get|out|come|on|wow+|woah|whoa|omg|what on earth|why on earth|how on earth|what in the world|why in the world|devil|blazes|dickens|damn|fucking|going|goes|happen|happens|happened|happening|you|youre|are|joke|for|blimey|crikey|thank|good|god|heavens|christ|my|word|that|this|it|is|so|just|awful|terrible|horrible|rough|much|sick|the|no)){0,6}\W*$"##
);
regex!(
    POINTS_BACK,
    r##"(?:as|like)\s+i\s+(?:was|were)\s+saying|back\s+to\s+what\s+i\s+was\s+saying|as\s+i\s+said|where\s+(?:was|were)\s+(?:i|we)|(?:got|get|getting)\s+cut\s+off|cut\s+off\s+there|(?:i\s+)?got\s+interrupted|was\s+interrupted|carry\s+on|back\s+again|any\s?way|any\s?road"##
);
regex!(
    LATER_QUESTION,
    r##"(?i)(?:^|[,.!?;\u2013\u2014-]\s*|\b(?:and|but|so|then|also|ok|okay|cool|right|btw|anyway|quick|thing)\s+)(?:what|whats|wats|wat|wot|why|whys|how|hows|when|whens|where|wheres|who|whos|which|can (?:i|you|we|it) \w+|could (?:i|you|we) \w+|should i \w+|would (?:i|you) \w+|do (?:i|you|we) \w+|does (?:it|that|this) \w+|is (?:it|that|this|there) \w+|are (?:there|you|we) \w+)\b"##
);
regex!(
    ASK_SHAPES,
    r##"\s(who|what|where|when|why|how|which)\s+(should|do|does|can|could|would|will|might|is|are|am)\s|\s(should|shall|do|does|can|could|would|will|did)\s+(i|we|you)\s|\sany (idea|ideas|advice|tips|suggestions|thoughts)\s|\swhat (do|should|can|would) (you|i)\s|\sis it worth\s|\sworth (it|doing|paying|getting|fixing|buying)\s"##
);
regex!(
    INDIRECT,
    r##"\s(?:about|over|of|on|at|to)\s+(?:who|what|where|when|why|how|whether)\s"##
);
regex!(
    METHOD_OPENING,
    r##"^(?:so |ok |okay |and |but )?(?:how|where) (?:do|does|should|shall|can|could|would|might) i\b"##
);
regex!(
    HOW_TO_TELL,
    r##"\b(?:know|tell|check|find\s+out) (?:if|whether|the|a|an|my|your|his|her|its|it's|that|this|which|apart|the\s+difference)\b"##
);
regex!(
    TIPS_ASKS,
    r##"\b(any|some|got any|need some|need any|want some)\s+(tips|tip|advice|pointers|suggestions|ideas|tricks|trick)\b"##
);
regex!(NOT_A_TIPS_ASK, r##"\btricks?\s+or\s+treat\b"##);
regex!(
    BETWEEN_US_ASK,
    r##"(?i)\b(?:is|are|was|were|will|would|does|do|can)\s+(?:this|that|it)\s+(?:(?:chat|conversation|thread|session)\s+)?(?:just\s+|really\s+)?between\s+(?:us|you\s+and\s+me)\b|\b(?:is|are)\s+(?:what|everything)\s+i\s+(?:say|tell|tell\s+you|type|write)\s+(?:just\s+)?between\s+(?:us|you\s+and\s+me)\b|\bare\s+(?:we|you)\s+(?:private|confidential|anonymous)\s+(?:here|on\s+this|in\s+this)\b|\bis\s+(?:this|it)\s+(?:chat\s+)?anonymous\b|\bwhat\s+(?:happens|happened)\s+to\s+(?:what|everything)\s+i\s+(?:say|tell|type|write|send)\b"##
);
regex!(
    NOT_WHAT_I_SAID,
    r##"(?i)\b(?:keep|store|save|record|log|share|sell|pass)\w*\s+(?:me\s+|us\s+)?(?:the\s+|a\s+|an\s+|my\s+|our\s+)?(?:kitchen|cat|dog|plants?|receipts?|leftovers?|food|milk|bread|eggs?|cheese|wine|beer|keys?|car|bike|house|flat|money|savings|cash|password|photos?|pictures?|files?|documents?|sheet|table|bill|bills|tickets?|seats?|room|garage|garden|washing|clothes|coat|bag|phone|number|address|email|weight|diary|journal|notes)\b|\bprivate\s+(?:rented|rent|rental|landlord|landlords|sale|sector|school|schools|hospital|hospitals|treatment|health|healthcare|health\s?care|medical|insurance|pension|parking|road|land|garden|tuition|tutor|lessons?|detective|investigator|eye|clinic|care|nursery|swimming|coach|hire|contract|members?|club|firm|company|jet|plane|island|beach)\b|\bshared\s+(?:ownership|owner|equity|driveway|garden|house|home|flat|room|kitchen|bathroom|car|bike|account|login|costs?|bills?|custody|care|space|office)\b"##
);
regex!(
    TRUST_ASK,
    r##"(?i)\b(?:can|could|should|shall|may)\s+(?:i|we)\s+(?:really\s+|honestly\s+|actually\s+)?(?:trust|believe|rely\s+on)\s+(?:you|u)\b|\bcan\s+(?:i|we)\s+(?:trust|believe)\s+what\s+(?:you|u)\s+(?:say|tell\s+me)\b|\bare\s+(?:you|u)\s+(?:trustworthy|reliable|honest|truthful)\b|\bwill\s+(?:you|u)\s+(?:tell|give)\s+me\s+the\s+truth\b|\bare\s+(?:you|u)\s+telling\s+(?:me\s+)?the\s+truth\b|\bare\s+(?:you|u)\s+lying\b|\bdo\s+(?:you|u)\s+(?:ever\s+)?(?:lie|guess|make\s+(?:things|stuff|that|it|this|them)\s+up|make\s+up\s+(?:things|stuff|answers|facts))\b|\bare\s+(?:you|u)\s+(?:making|inventing)\s+(?:that|this|it|things|stuff)\s+up\b"##
);
regex!(
    SECRET_ASK,
    r##"(?i)\bcan\s+(?:you|u)\s+keep\s+(?:a\s+)?secret\b|\bare\s+(?:you|u)\s+good\s+at\s+keeping\s+(?:a\s+)?secret\b|\bcan\s+i\s+tell\s+(?:you|u)\s+(?:a\s+)?secret\b|\bkeep\s+(?:this|it)\s+(?:between\s+(?:us|you\s+and\s+me)|to\s+yourself)\b|\bthis\s+stays\s+between\s+(?:us|you\s+and\s+me)\b"##
);
regex!(
    JUDGING_ASK,
    r##"(?i)\b(?:will|would|are|do|did|might)\s+(?:you|u)\s+(?:going\s+to\s+)?(?:judge|judging|judgement|judgment)(?:\s+me\b|\s+us\b|\s+my\b|\s+what\s+i\b|\s+someone\b|[\s?!.]*$)|\b(?:don'?t|do\s+not|please\s+don'?t|promise\s+(?:me\s+)?(?:you\s+won'?t|not\s+to)|you\s+won'?t)\s+judge\b|\bwill\s+(?:you|u)\s+(?:think|say)\s+i'?m\s+(?:weird|stupid|mad|dumb|thick|odd|crazy|silly|pathetic|a\s+failure)\b"##
);
regex!(
    WORKINGS_ASK,
    r##"\bhow\s+(?:do|does)\s+(?:you|u)\s+(?:actually\s+|really\s+)?work\s*(?:then|though|really|anyway|now)?\s*$|\bhow\s+(?:are|were)\s+(?:you|u)\s+(?:built|made|programmed|put together|written)\b|\bwhat(?:s| is)\s+(?:inside|under)\s+(?:you|your hood|the hood)\b|\bdo\s+(?:you|u)\s+use\s+(?:ai|an ai|chatgpt|gpt|a model|an llm|llm|a neural network)\b|\bhow\s+do\s+(?:you|u)\s+answer\s+(?:questions|these|things|that)\s*$"##
);
regex!(
    MODEL_ASK,
    r##"\b(?:what|which)\s+(?:\w+\s+){0,2}?(?:model|version|engine|llm|ai)\s+(?:are|is|do|did)\s+(?:you|u)\b|\bwhat\s+(?:are|is|were)\s+(?:you|u)\s+(?:running|based|built|trained)\s+on\b|\bare\s+(?:you|u)\s+(?:a\s+|an\s+)?(?:llm|large\s+language\s+model|neural\s+net\w*|transformer|deep\s+learning\s+model)\b"##
);
regex!(
    HELP_ELSE,
    r##"\b(?:what|which)\s+(?:else|other|more)\b[^.?]{0,25}?\b(?:can|could|do|does)\s+you\s+(?:do|help|offer|manage|cover)\b([^.?]*)|\b(?:anything|something)\s+else\b[^.?]{0,20}?\byou\s+(?:can|could)\s+(?:do|help|offer)\b([^.?]*)"##
);
regex!(
    DEFINITION,
    r##"^\s*(?:what|what's|whats|wats)\s+(?:is|are|was|were|does|do)\b|^\s*what(?:'s|s)?\s+(?:a|an|the)\b|^\s*(?:explain|tell me about|define)\b|\bwhat does .* mean\b"##
);
regex!(
    DECIDE_OPENING,
    r##"(?:^|[,.!?;]\s*|\b(?:so|but|then|and|ok|okay)\s+)(?:should (?:i|we)|shall (?:i|we)|is it worth|is it better|would it be better|which (?:one )?should i|(?:do |)(?:you|u|ya) (?:think|reckon) i should|what would you do|do i (?:say|tell|ask|mention|reply|text|call|ring|accept|send|bring)\b|would you)\b"##
);
regex!(
    A_SORRY_ON_ITS_OWN,
    r##"(?i)^(?:(?:oh|ok|okay|right|so|well|hey|hi|and|but|no)\s+)*(?:(?:that\s*s|thats|that\s+is|it\s*s|its)\s+)?(?:(?:it\s+)?(?:was|is)\s+)?(?:my\s+)?(?:(?:i\s*m|im|i\s+am)\s+)?(?:so\s+|very\s+|really\s+|dead\s+|truly\s+|awfully\s+|terribly\s+)?(?:sorry|apologies|apologise|apologize|bad|fault|mistake)\b(?:\s+(?:about|for)\s+(?:that|this|it|everything))?(?:\s+(?:again|honestly|mate|pal|then|anyway|sorry|soz|eh))*[\s.!,]*$"##
);
regex!(
    A_DISMISSAL,
    r##"(?i)^(?:(?:oh|ok|okay|right|so|well|and|but|no|nah|hey)\s+)*(?:never\s*mind|nevermind|forget\s+it|forget\s+about\s+(?:it|that|this|everything|all\s+of\s+it|the\s+(?:whole\s+)?(?:thing|lot))|forget\s+(?:i|what\s+i)\s+said|ignore\s+(?:that|it|me|this)|scratch\s+that|as\s+you\s+were|nvm|no\s+worries|dont\s+worry(?:\s+about\s+it)?|do\s+not\s+worry(?:\s+about\s+it)?|leave\s+it|drop\s+it)\b(?:\s+(?:about\s+(?:that|it)|that|this|it|eh|huh|then|anyway|now|thanks|ta|mate|pal|honestly|though|tho))*(?:\s*[,;]\s*[^.!?]{0,40})?[\s.!]*$"##
);
regex!(
    A_RECEIPT_OF_NOTHING,
    r##"(?i)^\s*(?:got it[.!]?\s*(?:is there anything you'?d like a hand with\??)?|thanks for telling me\s*[\u2014\u2013-]?\s*i'?m here if you(?:'d| would)? ?(?:want|like) to dig into any of that)\s*[.!]?\s*$"##
);
regex!(
    REFERS_BACK,
    r##"\b(?:you|u|yous)\s+(?:said|told me|mentioned|suggested|advised|recommended|agreed|promised|told me to)\b|^(?:(?:o+|oh|and|so|ok|okay|right|now|but|well|anyway|erm|um)\s+)*(?:back|going back|return|coming back)\s+to\s+(?:what|that|it|this|where|our|the)\b|^(?:(?:o+|oh|and|so|ok|okay|right|now|but|well|anyway|erm|um)\s+)*(?:what|which)\s+(?:was|were)\s+we\b|\bwhat we (?:were|was) (?:saying|talking about|on about)\b|\b(?:the )?same (?:as )?(?:before|last time)\b|\bthat thing i (?:told|mentioned|said to|asked) you\b|\bas (?:we|you) (?:said|were saying|agreed|discussed)\b|\bas i (?:said|mentioned|told you)\b"##
);
regex!(
    HANDOFF_ABOUT,
    r##"(?i)^(?:(?:and|oh|ooh|ok|okay|so|right|ah|er|um|umm|hmm|sorry|also|just|plus|hey|ta|anyway|anyways|first|then|actually|also)\b[\s,]+)*(?:(?:one|two|three|a couple|a few|couple)(?: more| other| final| last) (?:things?|questions?|qs?|asks?|quer(?:y|ies)|bits?|points?|ones?)|another (?:things?|questions?|qs?|asks?|ones?|points?|quer(?:y|ies))|(?:one |the |a )?(?:last|final|next|other|second) (?:things?|questions?|qs?|asks?|ones?|bits?)|a? ?quick (?:ones?|questions?|qs?|asks?|things?|quer(?:y|ies))|(?:a |one |an )?(?:different|separate|unrelated|side) (?:questions?|ones?|asks?|things?|quer(?:y|ies))|one thing more|before i forget|while (?:im|i am) (?:here|at it))(?:\s+(?:actually|too|as well|then|now|first|ok|okay|sorry|thanks|tho|though|i think|i suppose|i reckon))?[\s,:-]*(?:about|on|to do with|regarding|around)\s+(?P<thing>(?:(?:the|my|our|your|his|her|their|its|a|an|this|that)\s+)?[\w'-]+(?:\s+[\w'-]+){0,3})[\s.?!,]*$"##
);
regex!(
    HANDOFF,
    r##"(?i)^(?:(?:and|oh|ooh|ok|okay|so|right|ah|er|um|umm|hmm|sorry|also|just|plus|hey|ta|anyway|anyways|first|then|actually|also)\b[\s,]+)*(?:(?:one|two|three|a couple|a few|couple)(?: more| other| final| last) (?:things?|questions?|qs?|asks?|quer(?:y|ies)|bits?|points?|ones?)|another (?:things?|questions?|qs?|asks?|ones?|points?|quer(?:y|ies))|(?:one |the |a )?(?:last|final|next|other|second) (?:things?|questions?|qs?|asks?|ones?|bits?)|a? ?quick (?:ones?|questions?|qs?|asks?|things?|quer(?:y|ies))|(?:a |one |an )?(?:different|separate|unrelated|side) (?:questions?|ones?|asks?|things?|quer(?:y|ies))|one thing more|before i forget|while (?:im|i am) (?:here|at it))(?:\s+(?:actually|too|as well|then|now|first|ok|okay|sorry|thanks|tho|though|i think|i suppose|i reckon))?[\s.?!,]*$"##
);
regex!(
    HANDOFF_QUESTION,
    r##"(?i)\b(?:question|questions|q|qs|ask|asks|query|queries)\b"##
);
regex!(
    LEAVE,
    r##"\b(?:(?:got|gotta|have|has|need|needs|better|best|must|ought|should)\s+(?:to\s+)?(?:go|get\s+off|get\s+going|be\s+off|be\s+going|head\s+off|shoot|shoot\s+off|run|dash|fly)|(?:i'?m|im|i\s+am|we'?re|we\s+are|i'?ll|ill)\s+(?:gonna|going\s+to)\s+(?:go|be\s+off|head\s+off|shoot(?:\s+off)?|get\s+off)\b|(?:i'?m|im|i\s+am|we'?re|we\s+are)\s+(?:just\s+)?(?:gonna\s+|going\s+to\s+)?(?:turn(?:ing)?\s+in|going\s+up\s+to\s+bed|signing\s+off|up\s+to\s+bed)\b|(?:i'?m|im|i\s+am)\s+(?:gonna\s+|going\s+to\s+|just\s+)?(?:get|getting)\s+(?:some\s+)?(?:kip|kips|shut\s?eye|shuteye|sleep)\b|(?:i'?m|im|i\s+am|we'?re|we\s+are)\s+(?:just\s+|probably\s+)?(?:having|going\s+to\s+have|off\s+for|going\s+for)\s+an\s+early\s+(?:night|one)\b|(?:(?:an\s+)?early\s+night)\b|(?:time\s+for\s+an\s+early\s+night)\b|(?:^just\s+having\s+an\s+early\s+(?:night|one))\b|(?:i'?m|im|i\s+am|we'?re|we\s+are)\s+(?:just\s+)?(?:going\s+to\s+|gonna\s+)?(?:get|getting)\s+my\s+head\s+down\b|(?:i'?m|im|i\s+am)\s+(?:just\s+)?(?:heading|going)\s+up\s+to\s+(?:my\s+)?bed\b|(?:i'?m|im|i\s+am)\s+(?:just\s+)?(?:going\s+to\s+|gonna\s+)?try\s+(?:and\s+|to\s+)(?:sleep|get\s+some\s+sleep)\b|(?:time\s+to\s+turn\s+in)\b|(?:bed\s*o'?\s*clock)\b|(?:(?:it'?s|its)\s+)?sleep\s*time\b|(?:i'?m|im|i\s+am|we'?re|we\s+are|i'?ll|ill)\s+(?:be\s+)?(?:off|going|head(?:ing)?\s+off|shoot(?:ing)?\s+off|out\s+of\s+here|get\s+(?:off|going))\b|(?:i'?m|im|i\s+am|we'?re|we\s+are)\s+(?:away|out)\b|(?:off\s+to\s+(?:bed|sleep|kip|nap|work|school|the\s+shop))\b|(?:going\s+for\s+a\s+(?:kip|nap|lie\s+down))\b|(?:hitting\s+the\s+hay)\b|(?:(?:bed|bed\s*time)\s+for\s+(?:me|us))\b|(?:bed\s*time)\b|(?:call(?:ing)?\s+it\s+a\s+night)\b|(?:^going\s+up\s+to\s+(?:my\s+)?bed)\b|(?:^just\s+going\s+up\s+to\s+(?:my\s+)?bed)\b|(?:\bgood\s?night\b|\bgood\s?nite\b|\bnite\b|\bnight\b)|(?:get(?:ting)?\s+out\s+of\s+your\s+hair)\b|(?:time\s+(?:for\s+me\s+)?to\s+(?:go|head\s+off|be\s+off|shoot(?:\s+off)?))\b|(?:(?:that'?s|thats)\s+me\s+(?:away|off|out|done))\b|(?:let\s+you\s+go|leave\s+you\s+to\s+it|leave\s+you\s+be|let\s+you\s+get\s+on|let\s+you\s+get\s+back|stop\s+bothering\s+you)\b|(?:talk|speak|chat)(?:ing)?\s+(?:to\s+you\s+)?(?:later|soon|tomorrow)\b|(?:talk|speak|chat)(?:ing)?\s+(?:to\s+you\s+)?(?:\b(?:(?:at|around|about)\s+(?:about\s+|around\s+|half\s+)?\d{1,2}(?::\d{2})?\s*(?:am|pm|o'?clock)?|(?:at|around|about)\s+(?:about\s+|around\s+|half\s+)?(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b|at\s+(?:lunch|lunchtime|teatime|breakfast|break)\b|after\s+(?:work|school|college|uni|university|lunch|tea|dinner|the\s+school\s+run)\b|on\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|weds|thurs|fri|sat|sun)\b|this\s+(?:evening|afternoon|weekend)\b|next\s+week\b|at\s+the\s+weekend\b|in\s+the\s+(?:morning|afternoon|evening)\b|in\s+(?:a\s+)?(?:bit|while|minute|sec|second|mo|moment)\b))|(?:catch|see)\s+(?:you|ya)\b|(?:time\s+(?:for\s+me\s+)?to\s+go)\b|(?:that'?s|thats)\s+(?:me\s+)?done\b|(?:that'?s|thats)\s+it\s+from\s+me\b|(?:i'?m|im|i\s+am)\s+done\s+for\s+(?:the\s+)?(?:day|today|now)\b|(?:time\s+(?:i\s+was\s+off|(?:for\s+)?(?:my\s+)?bed))\b|(?:need|needs)\s+my\s+bed\b|(?:my\s+bed'?s?\s+calling)\b)"##
);
regex!(
    A_SIGN_OFF,
    r##"\b(?:(?:catch|see)\s+(?:you|ya|u)\b|(?:talk|speak|chat)(?:ing)?\s+(?:to\s+)?(?:you|ya|u)\b|(?:c\s*u|c\s*ya)\b)"##
);
regex!(
    A_LEAVE_TIME,
    r##"\b(?:(?:at|around|about)\s+(?:about\s+|around\s+|half\s+)?\d{1,2}(?::\d{2})?\s*(?:am|pm|o'?clock)?|(?:at|around|about)\s+(?:about\s+|around\s+|half\s+)?(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b|at\s+(?:lunch|lunchtime|teatime|breakfast|break)\b|after\s+(?:work|school|college|uni|university|lunch|tea|dinner|the\s+school\s+run)\b|on\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|weds|thurs|fri|sat|sun)\b|this\s+(?:evening|afternoon|weekend)\b|next\s+week\b|at\s+the\s+weekend\b|in\s+the\s+(?:morning|afternoon|evening)\b|in\s+(?:a\s+)?(?:bit|while|minute|sec|second|mo|moment)\b)"##
);
regex!(
    NITE,
    r##"\b(?:good\s?night|good\s?nite|nite|night|bed|bedtime|kip|hay|sleep|sleeping|turn(?:ing)?\s+in|head\s+down|shut\s?eye|\blie\s+down)\b"##
);
regex!(
    NOT_A_BEDTIME,
    r##"\b(?:shift|shifts|bus|buses|duty|class|classes|out|club|feeds?|mare|terrors?|nurse|nursing|porter|stock)\b"##
);
regex!(
    KEEPS,
    r##"\b(?:dont|don't|doesnt|doesn't|didnt|didn't|not|wont|won't|never)\b[^,.;]{0,24}\b(?:go|going|leave|off)\b"##
);
regex!(
    AWAY_OUT,
    r##"\b(?:i'?m|im|i\s+am|we'?re|we\s+are)\s+(?:off|away|out)\b"##
);
regex!(
    AWAY_PLAN,
    r##"\b(?:next\s+(?:week|month|year|weekend)|this\s+(?:weekend|evening|afternoon)|tonight|tomorrow|on\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|at\s+the\s+weekend|for\s+the\s+weekend|in\s+(?:june|july|august|september|october|november|december|january|february|march|april|may))\b"##
);
regex!(
    BED_LEAVE,
    r##"\b(?:early\s+(?:night|one)|head\s+down|heading\s+up\s+to\s+bed|try\s+(?:and\s+|to\s+)sleep)\b"##
);
regex!(
    NEXT_DAY,
    r##"\b(?:tomorrow|tomorra|tmrw|tmoz|next\s+(?:week|month|year|weekend)|on\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday))\b"##
);
regex!(
    ASKS_OPENER,
    r##"^(?:can|could|should|shall|do|does|did|is|are|was|were|will|would|may|might|must|what|whats|when|where|why|how|hows|who|whos|which|whose)\b"##
);
regex!(
    A_COMPANY_ASK,
    r##"^(?:hi|hiya|hey|hello|so|ok|okay|and|but|just|right)?[,!. ]*(?:(?:can|could|will|would|do)\s+(?:you|u)\s+(?:please\s+)?)?(?:(?:keep|stay)\s+me\s+company|stay\s+with\s+me)(?:(?:\s+(?:please|thanks|thank you|ta))?|\s+(?:(?:(?:(?:this |next |on |by )(?:(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend))|(?:(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend))(?:\s+(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed)))?|(?:this |in the |in |during the |tomorrow )(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed))|(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed))|later(?:\s+on)?|in a (?:bit|minute|while|mo)|in the week|during the week|sometime|some time|any time|at some point|at some stage|whenever|next week|this week|next month|this month|at\s+\d{1,2}(?:[.:]\d{2})?\s*(?:am|pm|a\.m\.|p\.m\.|o'?clock)?|at\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|noon|midday|midnight|teatime|lunchtime|half\s+\w+|quarter\s+\w+)|in\s+(?:a|one|two|three|half\s+an?|an?|\d{1,3})\s+(?:hour|hours|hr|hrs|minute|minutes|min|mins)|when you(?:'re| are) free|when you have time|when you get a chance|when you can)(?:\s+(?:(?:this |next |on |by )(?:(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend))|(?:(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend))(?:\s+(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed)))?|(?:this |in the |in |during the |tomorrow )(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed))|(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed))|later(?:\s+on)?|in a (?:bit|minute|while|mo)|in the week|during the week|sometime|some time|any time|at some point|at some stage|whenever|next week|this week|next month|this month|at\s+\d{1,2}(?:[.:]\d{2})?\s*(?:am|pm|a\.m\.|p\.m\.|o'?clock)?|at\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|noon|midday|midnight|teatime|lunchtime|half\s+\w+|quarter\s+\w+)|in\s+(?:a|one|two|three|half\s+an?|an?|\d{1,3})\s+(?:hour|hours|hr|hrs|minute|minutes|min|mins)|when you(?:'re| are) free|when you have time|when you get a chance|when you can)){0,2}))(?:\s+(?:please|thanks|thank you|ta))?)?\s*[?.!]*$"##
);
regex!(
    A_TALK_AT_A_TIME_ASK,
    r##"^(?:hi|hiya|hey|hello|so|ok|okay|and|but|just|right|um|er)?[,!. ]*(?:(?:can|could|shall|should|would|will)\s+(?:we|you|u|i)\s+(?:please\s+)?(?:talk|chat|speak|catch up|have a word|have a chat|have a talk)(?:\s+(?:to|with)\s+(?:you|u|me))?|i\s+(?:want|need|would like|wanna|have)\s+to\s+(?:talk|chat|speak)\s+to\s+(?:you|u))\s+(?:(?:(?:this |next |on |by )(?:(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend))|(?:(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend))(?:\s+(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed)))?|(?:this |in the |in |during the |tomorrow )(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed))|(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed))|later(?:\s+on)?|in a (?:bit|minute|while|mo)|in the week|during the week|sometime|some time|any time|at some point|at some stage|whenever|next week|this week|next month|this month|at\s+\d{1,2}(?:[.:]\d{2})?\s*(?:am|pm|a\.m\.|p\.m\.|o'?clock)?|at\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|noon|midday|midnight|teatime|lunchtime|half\s+\w+|quarter\s+\w+)|in\s+(?:a|one|two|three|half\s+an?|an?|\d{1,3})\s+(?:hour|hours|hr|hrs|minute|minutes|min|mins)|when you(?:'re| are) free|when you have time|when you get a chance|when you can)(?:\s+(?:(?:this |next |on |by )(?:(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend))|(?:(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend))(?:\s+(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed)))?|(?:this |in the |in |during the |tomorrow )(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed))|(?:(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed))|later(?:\s+on)?|in a (?:bit|minute|while|mo)|in the week|during the week|sometime|some time|any time|at some point|at some stage|whenever|next week|this week|next month|this month|at\s+\d{1,2}(?:[.:]\d{2})?\s*(?:am|pm|a\.m\.|p\.m\.|o'?clock)?|at\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|noon|midday|midnight|teatime|lunchtime|half\s+\w+|quarter\s+\w+)|in\s+(?:a|one|two|three|half\s+an?|an?|\d{1,3})\s+(?:hour|hours|hr|hrs|minute|minutes|min|mins)|when you(?:'re| are) free|when you have time|when you get a chance|when you can)){0,2})(?:\s+(?:please|thanks|thank you|ta))?\s*[?.!]*$"##
);
regex!(
    A_TALK_AT_A_TIME_HARM,
    r##"\b(?:kill myself|end it|end my life|suicid|self harm|hurt myself|overdose|\bdie\b|\bdying\b|dead|funeral|died|cancer|hospital|ambulance|999|911)\b"##
);
regex!(
    REACTION_SIGN_0,
    r##"\b(?:phew|finally|thank (?:god|goodness|heavens|christ|fuck))\b"##
);
regex!(
    REACTION_SIGN_1,
    r##"\b(?:ouch\w*|ow+|yikes|eek|oof|yowch|sucks|brutal|awful|terrible|horrible|rough|dammit|damn it|bugger|crap|shoot)\b"##
);
regex!(
    REACTION_SIGN_2,
    r##"\b(?:oops|whoops|oh (?:no|dear)|dear me)\b"##
);
regex!(
    REACTION_SIGN_3,
    r##"\b(?:seriously|are you serious|you(?:'?re| are)? (?:joking|kidding)|for real|no way|no chance|surely|really|honestly|wtf|what the (?:hell|heck|devil|fuck|blazes|dickens)|what (?:on|in) (?:earth|the world)|why (?:on|in) (?:earth|the world)|get out|you what|come on|wow+|woah|whoa|omg|oh my (?:god|word)|blimey|crikey)\b"##
);
regex!(REACTION_SIGN_4, r##"\b(?:huh|eh|pardon)\b"##);
