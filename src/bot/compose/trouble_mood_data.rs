// Composer mood cues and guards from the pinned rule set.

pattern!(
    A_HARM_IDIOM,
    r#"(?i)\b(?:stroke of (?:luck|genius|good luck|the brush)|brush ?stroke|stroke of a pen|broke the ice|broke the news|broke even|broke the bank|broke a record|broke the rules|broke a promise|broke the silence|cracked a joke|cracked a smile|cracked on|cracked a window|slipped my mind|slipped away|tore a strip off|torn between)\b"#
);
pattern!(
    A_PART_OUT,
    r#"(?i)\b(?:(?:had|have|having|needed|need|getting|got)\s+)?(?:to have\s+|to get\s+)?(?:a|an|my|two|some|the)\s+(?:tooth|teeth|wisdom tooth|wisdom teeth|molars?|appendix|tonsils?|adenoids|gallbladder|gall bladder|kidney stone|cataract|splinter|ingrown nail|nail)\s+(?:out|removed|taken out|pulled|drained)\b"#
);
pattern!(
    A_PROGNOSIS,
    r#"(?i)\b(?:been given|given|been told (?:i (?:have|'ve|have got|got)|there (?:is|are))|told i (?:have|'ve|have got|got))\s+(?:about\s+|only\s+|maybe\s+|perhaps\s+)?(?:a|two|three|four|five|six|nine|ten|twelve|eighteen|a few|a couple of)\s+(?:more\s+)?(?:days|weeks|months|years)\b|\b(?:two|three|four|five|six|nine|ten|twelve|eighteen|a few|a couple of)\s+(?:more\s+)?(?:weeks|months|years)\s+(?:left|to live|at most|maybe)\b|\b(?:terminal(?:ly)? (?:ill|illness|cancer)|palliative care|in a hospice|end of life care)\b"#
);
pattern!(
    A_PROGNOSIS_THAT_IS_NOT,
    r#"(?i)\b(?:to pay|to repay|to clear|to save|to appeal|to file|to finish|to complete|notice|sentence|probation|tenancy|contract|lease|to build|to train|to study|to sell|to deliver|warranty|guarantee|to get out of)\b"#
);
pattern!(
    A_HURT_DONE,
    r#"(?i)\b(?:i|we)\s+(?:just\s+|nearly\s+|think\s+i\s+)?(?:have\s+|had\s+|ve\s+)?(?:broke|broken|fractured|cracked|dislocated|sprained|tore|torn|snapped|slipped|hurt|injured|done)\b"#
);
pattern!(
    A_BODY_WORD,
    r#"(?i)\b(?:back|knees?|legs?|feet|foot|ankles?|hips?|arms?|elbows?|wrists?|hands?|fingers?|thumbs?|shoulders?|ribs?|neck|toes?|heels?|shins?|calves|calf|thighs?|collarbone|jaw|nose|eyes?|head|spine|disc|kneecap|ligaments?|muscles?|tendons?|bones?|teeth|tooth|nail|shoulder blade|lip|lips|cheek|cheeks|chin|ears?|eyebrow|forehead|scalp)\b"#
);
pattern!(
    A_HURT_BY_SOMEONE,
    r#"(?i)\b(?:he|she|they|someone|somebody|anyone|(?:a|the)\s+(?:man|woman|lad|bloke|chap|guy|girl|boy|teen|carer|nurse|neighbour|neighbor)|(?:my|his|her|their|our)\s+\w+)\s+(?:\w+\s+){0,2}?(?:hit|punch(?:ed|es|ing)?|kick(?:ed|s|ing)?|slapp?(?:ed|ing)|smack(?:ed|ing)?|beat|beats|beating|beaten|headbutt(?:ed)?|bit|bites|burn(?:ed|t|s)|chok(?:ed|ing)|strangl(?:ed|ing)|stabbed|stabs|raped|molested|groped|dragg?(?:ed|ing)|whipp?(?:ed|ing)|belt(?:ed|ing))\s+(?:me|us)\b|\b(?:he|she|they|someone|somebody|anyone|(?:a|the)\s+(?:man|woman|lad|bloke|chap|guy|girl|boy|teen|carer|nurse|neighbour|neighbor)|(?:my|his|her|their|our)\s+\w+)\s+(?:\w+\s+){0,2}?(?:hit|punched|kicked|slapped|grabbed|pinched|twisted|yanked|wrenched|bit|bitten)\s+my\s+\w+|\b(?:he|she|they|someone|somebody|anyone|(?:a|the)\s+(?:man|woman|lad|bloke|chap|guy|girl|boy|teen|carer|nurse|neighbour|neighbor)|(?:my|his|her|their|our)\s+\w+)\s+(?:\w+\s+){0,2}?(?:push(?:ed|es|ing)?|shov(?:ed|es|ing)?)\s+(?:me|us)\s+(?:down|into|against|over|off|up|to the)\b|\b(?:he|she|they|someone|somebody|anyone|(?:a|the)\s+(?:man|woman|lad|bloke|chap|guy|girl|boy|teen|carer|nurse|neighbour|neighbor)|(?:my|his|her|their|our)\s+\w+)\s+(?:\w+\s+){0,2}?(?:threw|throws|tossed|spat|spits?)\b[^.?!]{0,25}?\bat\s+(?:me|us)\b|\b(?:held|holds|holding|put|puts|pointed|pointing|pressed|had|pulled)\s+(?:a|an|the|his|her|their)\s+(?:knife|gun|blade|scissors|hammer|bat|belt|bottle|glass|razor|needle|pillow|cushion)\s+(?:to|at|against)\s+(?:my|her|his|our)\s+\w+"#
);
pattern!(
    A_HURT_BY_SOMEONE_NOT,
    r#"(?i)\b(?:hit me up\b|hit on me\b|kicked me out\b|beat me (?:at|in|to)\b|bit my head off\b|grabbed my (?:attention|eye)\b|pushed (?:me|us) into \w+ing\b|burn(?:ed|t) me (?:out|a copy|a cd)\b)"#
);
pattern!(
    A_HARM_ILL,
    r#"(?i)\b(?:(?:i|we)\s+(?:(?:have|had|has)\s+been\s+)?(?:have|had|got|caught|was given|were given|been given|been told|was diagnosed with|diagnosed with)|(?:i've|we've|ive|weve)\s+(?:had|got|been given|been diagnosed with|caught))\s+(?:to have\s+|to get\s+)?(?:a|an|the|my|some|a few|a couple of|two|three|four|five|six|seven|eight|nine|ten|\d+)?\s*(?:[a-z]+\s+){0,2}?(?:infections?|bugs?|virus(?:es)?|flu|influenza|fever|colds?|cough|temperature|poisoning|disease|illness|ailment|condition|attack|migraines?|seizures?|sore throat|sickness|nausea|diarrhoea|diarrhea|rash|abscess|cyst|ulcer|thrush|gout|shingles|covid|coronavirus|malaria|tuberculosis|hepatitis|uti|dvt|copd|ibs|mrsa|strep|\w+(?:itis|osis|aemia|emia|iasis|pox|virus))\b"#
);
pattern!(
    A_HARM_ILL_NOT,
    r#"(?i)\b(?:cold (?:one|beer|drink|water|shower|bath|snap|call|weather|night|morning|day|evening|outside|front|shoulder|hands|feet)|bugs? (?:in|on) (?:my |the )?(?:code|computer|laptop|phone|app|program|software|system)|virus (?:on|in) (?:my |the )?(?:computer|laptop|phone)|temperature (?:setting|gauge|dial|outside|gauge)|sickness (?:policy|pay|record|note)|in (?:good|no|great|perfect|top) condition|condition of (?:the|my) (?:car|house|road)|attack of (?:the )?(?:nerves|conscience)|fever pitch|cabin fever|spring fever)\b"#
);
pattern!(
    A_HARM_HAD,
    r#"(?i)\b(?:(?:i|we)\s+(?:(?:have|had|has)\s+been\s+)?(?:have|had|got|caught|was given|were given|been given|been told|was diagnosed with|diagnosed with)|(?:i've|we've|ive|weve)\s+(?:had|got|been given|been diagnosed with))\s+(?:to have\s+|to get\s+)?(?:a|an|the|my|some|a few|a couple of|two|three|four|five|six|seven|eight|nine|ten|\d+)?\s*(?:concussion|concussions|stitches?|staples?|whiplash|pneumonia|sepsis|septicaemia|meningitis|covid|coronavirus|shingles|glandular fever|miscarriage|stillbirth|seizure|seizures|stroke|heart attack|blood clot|pulmonary embolism|breakdown|relapse|chemo|chemotherapy|radiotherapy)\b"#
);
pattern!(
    A_PART_WENT,
    r#"(?i)\bmy\s+(?:back|knee|knees|leg|arm|shoulder|hip|ankle|neck)\s+(?:just\s+|suddenly\s+)?(?:went|gone|gave way|gives way)\b"#
);
pattern!(
    A_HARM_KNOCK,
    r#"(?i)\b(?:knocked|thrown|shoved|swiped|swept)\s+off\s+(?:my|our|his|her|their|the|a)\b|\bhit\s+by\s+a\s+(?:car|van|lorry|truck|bus|taxi|bike|motorbike|cyclist)\b"#
);
pattern!(
    A_OUT_OF_SOCKET,
    r#"(?i)\bcame out of (?:its|the|my) socket\b|\bout of (?:its|the) socket\b|\b(?:my|our|his|her|their|the)\s+(?:shoulders?|knees?|kneecap|hips?|elbows?|jaw|ankles?|fingers?|thumbs?|wrist|toe)\s+(?:came|comes|went|goes|popped|pops) out\b"#
);
pattern!(A_IN_AGONY, r#"(?i)\bin (?:absolute |sheer |real )?agony\b"#);
pattern!(
    A_BREAK_THAT_IS_GOOD,
    r#"(?i)\bbroke(?:n)?\s+(?:a|the|my|our|his|her|their|new|world|personal|course|club|national)\s+(?:\w+\s+)?(?:records?|ice|sweat|ground|habit|cycle|deadlock|stalemate|personal\s+best)\b|\bbroke\s+even\b|\bbroke\s+through\b(?!\s+(?:the|a|an|my|our|his|her|their)\s)|\bbreak(?:ing)?\s+a\s+record\b"#
);
pub(super) const BREAK_WORDS: &[&str] = &["breaking", "breaks", "broke", "broken"];
pattern!(
    A_COMPARISON,
    r#"\b(?:than|beats?|more\s+than|as\s+much\s+as)\b"#
);
pattern!(
    NOTHING_THAT_LIMITS,
    r#"\bnothing\s+(?:with|that|too|but|except|without|involving|containing|featuring|before|after|under|over|like)\b"#
);
pattern!(
    PLEASURE_ABOUT_ONE_THING,
    r#"\b(?:happy|glad|pleased|excited|thrilled|proud)\s+about\s+(?:it|that|this|them|him|her|the\b|my\b|his\b|her\b|their\b|our\b)"#
);
pattern!(
    FEELING_NOTHING,
    r#"\bfeel(?:s|ing)?\s+nothing\b|\bnothing\s+matters\b|\bnothing\s+(?:makes\s+me\s+)?feels?\s+(?:good|right|nice|better)\b|\b(?:enjoy|enjoying|enjoyed)\s+nothing\b"#
);
pattern!(
    PLEASURE_GONE_OUT,
    r#"\b(?:fun|joy|pleasure|interest|point|enjoyment)\b\s+(?:has|have|had)\s+(?:gone|gone\s+right|all\s+gone)\s+out\s+of\b"#
);
pattern!(
    NOTHING_AND_A_PLEASURE,
    r#"\bnothing\b[^.?]{0,28}?\b(?:fun|funny|joy|joyful|pleasure|enjoy(?:s|ed|ing|able)?|happy|happiness|smile|laugh|mood|worth(?:while|\s+doing)?|interest(?:s|ed|ing)?|look\s+forward\s+to|content|exciting|excites?)\b"#
);
pattern!(
    NOTHING_NEGATED,
    r#"\b(?:no|not|never|dont|don'?t|doesnt|doesn'?t|cant|can'?t|cannot|stopped|without)\b[^.?]{0,24}?\b(?:fun|funny|joy|joyful|pleasure|enjoy(?:s|ed|ing|able)?|happy|happiness|smile|laugh|mood|worth(?:while|\s+doing)?|interest(?:s|ed|ing)?|look\s+forward\s+to|content|exciting|excites?)\b[^.?]{0,20}?\b(?:anything|nothing|much|things|life|it|myself|any\s?more|anymore|at\s+all)\b"#
);
pub(super) const POSITIVE_CUES: &[&str] = &[
    "accepted",
    "achieved",
    "adopted",
    "amazing",
    "brilliant",
    "celebrate",
    "delighted",
    "engaged",
    "excited",
    "finally",
    "finished",
    "happy",
    "hired",
    "improved",
    "pregnant",
    "promoted",
    "proud",
    "sorted",
    "succeeded",
    "thrilled",
    "won",
];
pub(super) const NEGATIVE_CUES: &[&str] = &[
    "afraid",
    "angry",
    "annoyed",
    "anxious",
    "argued",
    "arguing",
    "argument",
    "awful",
    "awkward",
    "broke",
    "cranky",
    "cried",
    "crying",
    "depressed",
    "drained",
    "exhausted",
    "fail",
    "failed",
    "failing",
    "fired",
    "frustrated",
    "frustrating",
    "grumpy",
    "hate",
    "hopeless",
    "horrible",
    "hurt",
    "hurts",
    "ignored",
    "irritable",
    "lonely",
    "lose",
    "lost",
    "miserable",
    "miss",
    "missing",
    "nervous",
    "numb",
    "overwhelmed",
    "painful",
    "panicking",
    "pointless",
    "ratty",
    "rejected",
    "rude",
    "sad",
    "scared",
    "sick",
    "snapping",
    "snappy",
    "stress",
    "stressed",
    "struggle",
    "struggling",
    "terrible",
    "tetchy",
    "tired",
    "unfair",
    "upset",
    "worried",
    "worry",
    "worst",
    "worthless",
];
pub(super) const NEARLY: &[&str] = &["almost", "nearly"];
pub(super) const DOUBTED: &[&str] = &[
    "doubt",
    "doubted",
    "doubting",
    "doubts",
    "uncertain",
    "unsure",
];
pub(super) const PAST_STATE_CUES: &[&str] = &["engaged", "pregnant"];
pub(super) const THE_WORDS_BETWEEN: &[&str] = &[
    "had", "has", "have", "is", "just", "now", "s", "was", "were",
];
pub(super) const THINGS_THAT_STOP: &[&str] = &[
    "broke",
    "broken",
    "collapsed",
    "conked",
    "dead",
    "died",
    "failed",
    "stopped",
];
pub(super) const THINGS_THAT_STOP_AFTER_UP: &[&str] = &["gave", "given", "packed", "went"];
pub(super) const A_WAIT_ENDING: &[&str] = &["finally"];
pattern!(
    SOMETHING_CAME_APART,
    r#"\b(?:died|dies|dying|dead|broke|broken|breaks|kaput|bust|busted|collapsed|flooded|leaking|snapped|conked|packed\s+in|packed\s+up|gave\s+up|given\s+up|gave\s+in|stopped\s+working|stopped\s+charging|fell\s+apart|had\s+it|written\s+off|beyond\s+repair|on\s+the\s+blink)\b"#
);
pub(super) const AIM_HEADS: &[&str] = &[
    "hope", "hoping", "need", "needs", "try", "trying", "want", "wanting", "wants",
];
pub(super) const STATE_CUES: &[&str] = &[
    "amazing",
    "brilliant",
    "delighted",
    "excited",
    "happy",
    "proud",
    "thrilled",
];
pub(super) const NOTHING_AT_ALL: &[&str] = &[
    "neither", "nobody", "none", "noone", "nor", "nothing", "nowhere",
];
pub(super) const CONTRAST: &[&str] = &[
    "although", "and", "because", "but", "cause", "cos", "however", "so", "still", "though",
    "when", "while", "yet",
];
pub(super) const CROWD: &[&str] = &[
    "everybody",
    "everyone",
    "everyones",
    "nobody",
    "others",
    "peoples",
];
pub(super) const PERSON_WORDS: &[&str] =
    &["me", "my", "myself", "our", "ourselves", "us", "we", "you"];
pub(super) const ABOUT_WORDS: &[&str] = &["about", "for", "of"];
pub(super) const YOU_SUBJECTS: &[&str] =
    &["i", "id", "ill", "im", "ive", "me", "myself", "us", "we"];
pub(super) const WAS: &[&str] = &["been", "used", "was", "were"];
pub(super) const ILL_SAID_OF_A_BODY: &[&str] = &[
    "am",
    "are",
    "badly",
    "been",
    "being",
    "chronically",
    "feel",
    "feeling",
    "feels",
    "fell",
    "felt",
    "get",
    "gets",
    "getting",
    "got",
    "im",
    "is",
    "look",
    "looks",
    "often",
    "seriously",
    "terribly",
    "was",
];
pub(super) const ILL_WILL_VERBS: &[&str] = &[
    "add", "ask", "be", "book", "bring", "buy", "call", "carry", "chat", "check", "clean", "come",
    "do", "drive", "drop", "eat", "email", "find", "finish", "fix", "get", "give", "go", "grab",
    "have", "head", "keep", "leave", "let", "look", "make", "meet", "message", "need", "order",
    "pay", "phone", "pick", "pop", "put", "read", "ring", "run", "say", "see", "sell", "send",
    "show", "sign", "sit", "sort", "speak", "start", "stay", "take", "tell", "text", "tidy", "try",
    "turn", "wait", "walk", "want", "wash", "write",
];
pub(super) const ILL_BEFORE: &[&str] = &[
    "always",
    "am",
    "are",
    "badly",
    "been",
    "being",
    "chronically",
    "feel",
    "feeling",
    "feels",
    "fell",
    "felt",
    "get",
    "gets",
    "getting",
    "got",
    "im",
    "is",
    "look",
    "looks",
    "often",
    "quite",
    "really",
    "seriously",
    "so",
    "still",
    "terribly",
    "too",
    "very",
    "was",
];
pub(super) const ILL_AFTER: &[&str] = &[
    "again",
    "all",
    "and",
    "at",
    "day",
    "days",
    "enough",
    "for",
    "from",
    "health",
    "in",
    "last",
    "lately",
    "month",
    "morning",
    "on",
    "since",
    "this",
    "today",
    "tonight",
    "week",
    "weeks",
    "with",
    "yesterday",
];
pattern!(
    AIM_BEFORE,
    r#"(?:\b(?:want|wants|wanting|trying|try|need|needs|plan|planning|hoping|hope|help|helps|starting|started)\s+(?:to\s+|me\s+to\s+)?|\b(?:how\s+do\s+i|how\s+can\s+i|how\s+to|best\s+way\s+to|ways\s+to|tips\s+to|tips\s+for|any\s+advice\s+on)\s+)$"#
);
pub(super) const AIM_NEGATORS: &[&str] = &[
    "cant", "didnt", "doesnt", "dont", "never", "no", "not", "wont",
];
pub(super) const AIM_FAILING: &[&str] = &[
    "doesnt", "hopeless", "isnt", "nothing", "nowhere", "stopped", "stuck", "wont",
];
pub(super) const JUNK_CUES: &[&str] = &["bleh", "crap", "crappy", "meh", "naff", "poo", "rubbish"];
pub(super) const COPULA: &[&str] = &[
    "am", "are", "be", "been", "being", "is", "seemed", "seems", "was", "were",
];
pub(super) const ABOUT_THEM_BEFORE_COPULA: &[&str] = &[
    "everything",
    "i",
    "i'm",
    "i've",
    "im",
    "it",
    "ive",
    "life",
    "me",
    "myself",
    "things",
    "this",
    "today",
    "us",
    "we",
    "we're",
    "were",
];
pub(super) const STRETCH_OF_TIME: &[&str] = &[
    "afternoon",
    "day",
    "days",
    "evening",
    "lately",
    "month",
    "months",
    "morning",
    "night",
    "shift",
    "term",
    "today",
    "week",
    "weekend",
    "weeks",
    "year",
];
pub(super) const MOOD_BEFORE: &[&str] = &[
    "absolutely",
    "am",
    "are",
    "be",
    "been",
    "being",
    "bit",
    "fairly",
    "feel",
    "feeling",
    "feels",
    "felt",
    "is",
    "pretty",
    "properly",
    "quite",
    "really",
    "seemed",
    "seems",
    "so",
    "such",
    "that",
    "this",
    "totally",
    "very",
    "was",
    "were",
];
pub(super) const MOOD_AFTER: &[&str] = &[
    "afternoon",
    "couple",
    "day",
    "days",
    "evening",
    "few",
    "month",
    "morning",
    "night",
    "shift",
    "start",
    "term",
    "time",
    "week",
    "weekend",
    "weeks",
    "year",
];
pub(super) const NEGATORS: &[&str] = &[
    "arent", "cannot", "cant", "didnt", "doesnt", "dont", "hadnt", "hasnt", "havent", "isnt",
    "never", "no", "not", "wasnt", "werent", "wont",
];
pub(super) const AUX: &[&str] = &[
    "ai", "are", "can", "could", "did", "do", "does", "had", "has", "have", "is", "might", "must",
    "need", "should", "was", "were", "wo", "would",
];
pub(super) const CONTRACTED_AUX: &[&str] = &[
    "ai", "am", "are", "ca", "could", "dare", "did", "do", "does", "had", "has", "have", "is",
    "might", "must", "need", "should", "was", "were", "wo", "would",
];
pub(super) const CANNOT: &[&str] = &["always", "cannot", "cant", "couldnt", "never"];
pub(super) const DOUBT: &[&str] = &[
    "believe", "believed", "imagine", "imagined", "think", "thought",
];
pattern!(BARE_ACHE, r#"(?i)\b\w*aches?\b"#);
pattern!(
    BARE_ILL,
    r#"(?i)\b(?:migraines?|heartburn|indigestion|tonsillitis|sciatica|cystitis|gout|sinusitis)\b"#
);
pattern!(
    BARE_BODY,
    r#"(?i)\b(?:head|throat|ear|ears|eye|eyes|tooth|teeth|back|knee|shoulder|arm|leg|legs|foot|feet|hand|hands|stomach|tummy|chest|neck|hip|ankle|wrist|elbow|toe|finger|skin|nose|sinus|jaw|jaws|cheek|cheeks|chin|lips?|tongue|gums?|mouth|face|forehead|temple|temples|scalp|spine|ribs?|shins?|calf|calves|thighs?|groin|kidneys?|bladder|nails?|heels?|palms?|soles?|thumbs?)\b"#
);
pattern!(
    BARE_HURTS,
    r#"(?i)\b(?:hurts?|hurting|aches?|aching|sore|pain|painful|agony|agonising|agonizing|killing me|is killing me|are killing me|tender|raw|swollen|swelling|puffy|stiff|throbbing|itchy|itches|burning|on fire|hot|bad|dodgy|constant|dull)\b"#
);
pub(super) const BARE_FILLER: &[&str] = &[
    "a",
    "actually",
    "all",
    "also",
    "an",
    "and",
    "awful",
    "bad",
    "bit",
    "both",
    "but",
    "day",
    "days",
    "ever",
    "god",
    "gosh",
    "got",
    "had",
    "honestly",
    "i",
    "im",
    "ive",
    "just",
    "left",
    "little",
    "lower",
    "me",
    "morning",
    "my",
    "nasty",
    "night",
    "now",
    "of",
    "off",
    "oh",
    "on",
    "proper",
    "really",
    "right",
    "since",
    "so",
    "still",
    "terrible",
    "that",
    "the",
    "this",
    "today",
    "tonight",
    "too",
    "top",
    "ugh",
    "upper",
    "urgh",
    "very",
    "week",
    "weeks",
    "worst",
    "yesterday",
];
pattern!(
    BARE_WORD_ONLY,
    r#"(?i)\b(?:is|are|was|were|have|has|had|got|get|feel|feeling|been|do|does|did|can|could|should|would|will|helps?|stop|stopping|cure|treat|remedy|remedies|take|need|want|make|makes|go|going|come|came|keep|keeps|said|says|tell|think|know|why|what|how|when)\b"#
);
pattern!(
    FEELING_BEFORE_SICK,
    r#"(?i)\b(?:worried|scared|frightened|terrified|bored|stressed|anxious)\s+sick\b"#
);
pattern!(
    SICK_AS_FEELING,
    r#"(?i)\b(?:sick|ill|unwell|nauseous|nauseated|queasy|shaking|shaky|trembling|dizzy)\s+(?:(?:with|from)\s+)?(worry|worrying|worried|stress|stress(?:ed)?\s+out|fear|fright|frightened|panic\w*|nerves|nervous|anxious|anxiety|guilt\w*|grief|shame|ashamed|dread\w*|anger|rage|jealousy|it\s+all)\b"#
);
pattern!(
    NO_LAUGH_OR_SCARES,
    r#"\b(?:no|not|without|except|minus)\s+(?:a\s+|the\s+|any\s+|too\s+|more\s+)?(?:laugh\s?track|laughter|laughing|comedy|comedies|jokes?|funny|scares?|scary|horror|gore|violence|subtitles|sad\s+ones?|sad\s+films?)\b"#
);
pub(super) const PEOPLE: &[&str] = &[
    "aunt",
    "auntie",
    "baby",
    "babysitter",
    "babysitters",
    "barber",
    "boy",
    "boyfriend",
    "boyfriends",
    "brother",
    "budgie",
    "caregiver",
    "caregivers",
    "carer",
    "carers",
    "cat",
    "cats",
    "child",
    "childminder",
    "childminders",
    "children",
    "classmate",
    "classmates",
    "cleaner",
    "cleaners",
    "colleague",
    "colleagues",
    "counsellor",
    "counsellors",
    "counselor",
    "counselors",
    "cousin",
    "dad",
    "daughter",
    "doctor",
    "doctors",
    "dog",
    "dogs",
    "ex",
    "family",
    "father",
    "ferret",
    "fiance",
    "fiancee",
    "fiances",
    "fiancé",
    "fiancée",
    "fil",
    "fish",
    "fishes",
    "flatmate",
    "flatmates",
    "friend",
    "friends",
    "gerbil",
    "girl",
    "girlfriend",
    "girlfriends",
    "godchild",
    "godchildren",
    "goddaughter",
    "goddaughters",
    "godfather",
    "godmother",
    "godson",
    "godsons",
    "goldfish",
    "gp",
    "gps",
    "gran",
    "grandad",
    "grandchild",
    "grandchildren",
    "granddad",
    "granddaughter",
    "granddaughters",
    "grandfather",
    "grandkids",
    "grandma",
    "grandmother",
    "grandpa",
    "grandparents",
    "grandson",
    "grandsons",
    "granny",
    "guardian",
    "guinea",
    "hairdresser",
    "halfbrother",
    "halfsister",
    "hamster",
    "horse",
    "housemate",
    "housemates",
    "husband",
    "imam",
    "in-laws",
    "kid",
    "kids",
    "kitten",
    "lizard",
    "lodger",
    "lodgers",
    "mam",
    "manager",
    "managers",
    "mate",
    "mates",
    "mentor",
    "midwife",
    "midwives",
    "mil",
    "mom",
    "mother",
    "mum",
    "nan",
    "nana",
    "nannies",
    "nanny",
    "neighbor",
    "neighbors",
    "neighbour",
    "neighbours",
    "nephew",
    "niece",
    "nurse",
    "nurses",
    "parent",
    "parents",
    "parrot",
    "partner",
    "pastor",
    "pet",
    "pets",
    "physio",
    "physiotherapist",
    "pony",
    "priest",
    "psychiatrist",
    "puppy",
    "rabbi",
    "rabbit",
    "relative",
    "relatives",
    "schoolfriend",
    "sibling",
    "siblings",
    "sister",
    "snake",
    "son",
    "spouse",
    "spouses",
    "stepbrother",
    "stepchild",
    "stepchildren",
    "stepdad",
    "stepdaughter",
    "stepdaughters",
    "stepmum",
    "stepsister",
    "stepson",
    "stepsons",
    "surgeon",
    "surgeons",
    "teacher",
    "teachers",
    "therapist",
    "therapists",
    "tortoise",
    "tutor",
    "tutors",
    "twin",
    "twins",
    "uncle",
    "vicar",
    "warden",
    "wife",
    "worker",
    "workers",
];
pub(super) const TURN_NEGATORS: &[&str] = &[
    "aint", "arent", "barely", "cannot", "cant", "couldnt", "didnt", "doesnt", "dont", "hardly",
    "hasnt", "havent", "isnt", "never", "no", "not", "nothing", "shouldnt", "wasnt", "werent",
    "without", "wont", "wouldnt",
];
pub(super) const MOOD_WITH_A_CATCH: &[(&str, &[&str])] = &[(
    "fired",
    &[
        "am", "are", "been", "being", "get", "gets", "getting", "got", "had", "has", "have",
        "i've", "is", "they've", "ve", "was", "we've", "were",
    ],
)];
pub(super) const FEELING_WORD: &[(&str, &str)] = &[
    ("anger", "angry"),
    ("anxiety", "anxious"),
    ("anxious", "anxious"),
    ("ashamed", "ashamed"),
    ("dread", "dreading"),
    ("dreading", "dreading"),
    ("fear", "frightened"),
    ("fright", "frightened"),
    ("frightened", "frightened"),
    ("grief", "grieving"),
    ("guilt", "guilty"),
    ("guilty", "guilty"),
    ("it all", "overwhelmed"),
    ("jealousy", "jealous"),
    ("nerve", "nervous"),
    ("nerves", "nervous"),
    ("nervous", "nervous"),
    ("panic", "panicky"),
    ("panicking", "panicky"),
    ("panicky", "panicky"),
    ("rage", "angry"),
    ("shame", "ashamed"),
    ("stress", "stressed"),
    ("stressed", "stressed"),
    ("stressed out", "stressed"),
    ("worried", "worried"),
    ("worry", "worried"),
    ("worrying", "worried"),
];
pattern!(BARE_ACHE_ONLY, r#"(?i)\A(?:\b\w*aches?\b)\z"#);
pattern!(
    BARE_ILL_ONLY,
    r#"(?i)\A(?:\b(?:migraines?|heartburn|indigestion|tonsillitis|sciatica|cystitis|gout|sinusitis)\b)\z"#
);
pattern!(
    BARE_BODY_ONLY,
    r#"(?i)\A(?:\b(?:head|throat|ear|ears|eye|eyes|tooth|teeth|back|knee|shoulder|arm|leg|legs|foot|feet|hand|hands|stomach|tummy|chest|neck|hip|ankle|wrist|elbow|toe|finger|skin|nose|sinus|jaw|jaws|cheek|cheeks|chin|lips?|tongue|gums?|mouth|face|forehead|temple|temples|scalp|spine|ribs?|shins?|calf|calves|thighs?|groin|kidneys?|bladder|nails?|heels?|palms?|soles?|thumbs?)\b)\z"#
);
pattern!(
    BARE_HURTS_ONLY,
    r#"(?i)\A(?:\b(?:hurts?|hurting|aches?|aching|sore|pain|painful|agony|agonising|agonizing|killing me|is killing me|are killing me|tender|raw|swollen|swelling|puffy|stiff|throbbing|itchy|itches|burning|on fire|hot|bad|dodgy|constant|dull)\b)\z"#
);
pattern!(
    SYMPTOM_0,
    r#"(?i)\b(?:i|we)(?:'ve|ve|'m|m| have| am| feel| felt| got| keep| woke up with)\b[^.!?]{0,25}?\b(?:sore|cough|coughing|headache|migraine|toothache|earache|rash|itchy|itching|infection|infected|cramps?|nauseous|nausea|dizzy|swollen|swelling|aching|aches?|throw up|throws up|threw up|throwing up|vomit\w*|constipated|bloated|diarrhoea|diarrhea|unwell|poorly|sick|temperature|flu|migraines|lumps?(?![\s-]+(?:sums?|of|sugar)\b)(?!\s+in\s+(?:my|your|his|her|their|our|the)\s+throat\b(?![^.!?]*\b(?:swallow|swallowing|voice|breath|breathing|breathe)\b))|growths?)"#
);
pattern!(
    SYMPTOM_1,
    r#"(?i)\bmy (?:head|throat|ear|ears|eye|eyes|tooth|teeth|back|knee|shoulder|arm|leg|legs|foot|feet|hand|hands|stomach|tummy|chest|neck|hip|ankle|wrist|elbow|toe|finger|skin|nose|sinus|jaw|jaws|cheek|cheeks|chin|lips?|tongue|gums?|mouth|face|forehead|temple|temples|scalp|spine|ribs?|shins?|calf|calves|thighs?|groin|kidneys?|bladder|nails?|heels?|palms?|soles?|thumbs?)\b[^.!?]{0,15}?\b(?:hurts?|hurting|aches?|aching|sore|pain|painful|agony|agonising|agonizing|killing me|is killing me|are killing me|tender|raw|swollen|swelling|puffy|stiff|throbbing|itchy|itches|burning|on fire|hot|bad)\b"#
);
pattern!(
    SYMPTOM_2,
    r#"(?i)\b(?:i|we)\s+(?:throw\w* up|vomit\w*|(?:am|are|'m|'re)?\s*being sick)\b"#
);
pattern!(
    SYMPTOM_3,
    r#"(?i)\b(?:my|a|the)\s+(?:stiff|sore|bad|dodgy|aching|swollen|gammy|sore)\s+(?:head|throat|ear|ears|eye|eyes|tooth|teeth|back|knee|shoulder|arm|leg|legs|foot|feet|hand|hands|stomach|tummy|chest|neck|hip|ankle|wrist|elbow|toe|finger|skin|nose|sinus|jaw|jaws|cheek|cheeks|chin|lips?|tongue|gums?|mouth|face|forehead|temple|temples|scalp|spine|ribs?|shins?|calf|calves|thighs?|groin|kidneys?|bladder|nails?|heels?|palms?|soles?|thumbs?)"#
);
pattern!(
    SYMPTOM_4,
    r#"(?i)\b(?:i|we)(?:'ve|ve|'m|m| have| am| feel| felt| got| keep| woke up with)\b[^.!?]{0,30}?\b(?:the runs|the trots|the squits|bad guts|bad belly|dicky (?:tummy|stomach)|dodgy (?:tummy|stomach)|upset (?:tummy|stomach)|(?:tummy|stomach|sick) bug|food poisoning|being sick|keep(?:t|s)? being sick|can'?t? keep anything down|loose stools|runny (?:tummy|bottom)|running to the (?:loo|toilet))\b"#
);
pattern!(
    SYMPTOM_5,
    r#"(?i)\bmy (?:stomach|tummy|belly|guts)\s+(?:is |feels |has been )?(?:off|queasy|bad|dodgy|playing up)\b"#
);
pattern!(
    SYMPTOM_6,
    r#"(?i)\b(?:i|we)\s+(?:found|felt|noticed|spotted)\b[^.!?]{0,20}?\b(?:lumps?|growths?)\b(?![\s-]+(?:sums?|of|sugar)\b)(?!\s+in\s+(?:my|your|his|her|their|our|the)\s+throat\b(?![^.!?]*\b(?:swallow|swallowing|voice|breath|breathing|breathe)\b))"#
);
pattern!(
    SYMPTOM_7,
    r#"(?i)\b(?:i|we)(?:'ve|ve|'m|m| have| am)?\s*(?:got|had|been|felt|feel)?\s*(?:a|an|the)\s+(?:bad |nasty |chesty |sore )?(?:cold|cough|temperature|sore throat|rash|migraine|headache|infection)\b"#
);
pub(super) const SYMPTOM: &[&std::sync::LazyLock<fancy_regex::Regex>] = &[
    &SYMPTOM_0, &SYMPTOM_1, &SYMPTOM_2, &SYMPTOM_3, &SYMPTOM_4, &SYMPTOM_5, &SYMPTOM_6, &SYMPTOM_7,
];
