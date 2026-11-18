//! Carer-follow-up cues from the pinned PyBot source at 7d63c7f.

pub(super) const ROUTE_NAMED: &str = concat!(
    r#"\b(?:999|911|112|111|a\s?&\s?e|a and e|accident and emergency|ambulance|paramedic|"#,
    r#"urgent care|out of hours|x-ray)\b"#,
);

pub(super) const THEY_ARE_EASING: &str = concat!(
    r#"\b(?:he|she|they|dad|mum|mom|nan|gran|granny|grandma|grandad|grandpa|son|daughter|"#,
    r#"husband|wife|partner|brother|sister|baby|kid|child|boy|girl|little one|friend|neighbour)"#,
    r#"(?:'s|s| is| are| was| were| has| had)?\b\s+"#,
    r#"(?:(?:come|came|coming)\s+(?:round|to|back round|back to)\b"#,
    r#"|(?:awake|conscious|alert|with it|responsive|breathing (?:ok|okay|normally|fine|easier|"#,
    r#"better|properly))\b"#,
    r#"|(?:sat|sitting) up\b"#,
    r#"|(?:talking|chatting|eating|drinking) (?:now|again)\b"#,
    r#"|(?:asleep|sleeping|snoozing|dozing|resting)\b"#,
    r#"|(?:up and about|on the mend|out of the woods|over the worst)\b)"#,
);

pub(super) const EASING_UNDONE: &str = concat!(
    r#"\b(?:confus\w*|not (?:making sense|himself|herself|themselves|with it|right)|"#,
    r#"blue|grey|gray|pale|clammy|shaking|shaky|dizzy|feverish|burning up|bleeding|sick|"#,
    r#"vomit\w*|wont wake|won't wake|cant wake|can't wake|unresponsive|limp|floppy|slurr\w*|"#,
    r#"breathing (?:oddly|strangely|badly|funny))\b"#,
);

pub(super) const THEM_ASKING: &str =
    r#"^(?:is|are|was|were|has|have|had|did|does|do|will|would|can|could|should|shall|any)\b"#;

pub(super) const EASING_NOT_ONE: &str = concat!(
    r#"\b(?:not|isnt|isn't|arent|aren't|wasnt|wasn't|wont|won't|cant|can't|couldnt|didnt|doesnt|"#,
    r#"hasnt|havent|still|worse|but|except|though|shaking|shaky|confused|blue|grey|pale|burning|"#,
    r#"hot|fever|bleeding|sick|vomit\w*|dizzy|gone)\b|\?"#,
);

pub(super) const MOVE_ON: &str = concat!(
    r#"\b(?:anyway|what about|how about|what if|should i|shall i|do i|can i|im going to|"#,
    r#"ill (?:go|ring|call|take)|what do i (?:do|say))\b"#,
);

pub(super) const THEY_SAY_ITS_FINE: &str = concat!(
    r#"\b(?:says?|said|telling|tells?|told|insists?|insisted|reckons?|thinks?|swears?|"#,
    r#"claims?|assures?|keeping\s+on\s+about|keeps?\s+saying)\b[^.!?]{0,26}?"#,
    r#"\b(?:its|it's|hes|he's|shes|she's|theyre|they're|im|i'm)\b\s*"#,
    r#"(?:nothing|fine|ok|okay|alright|all\s+right|not\s+(?:that|too|so)\s+bad|no\s+big\s+deal|"#,
    r#"nothing\s+to\s+worry\s+about|just|only|a\s+bit|a\s+little|bruis\w*|"#,
    r#"a\s+scratch|a\s+bump)\b"#,
    r#"|\b(?:told|asked|telling|asking|begged)\s+me\s+not\s+to\s+(?:worry|fuss|make\s+a\s+fuss|"#,
    r#"bother|think\s+about\s+it)\b"#,
    r#"|\b(?:said|says|told\s+me|kept\s+saying|keeps?\s+saying)\s+nothing\s+(?:to\s+worry|"#,
    r#"was\s+wrong|is\s+wrong|wrong)\b"#,
);

pub(super) const ABUSE_ANSWER: &str = concat!(
    r#"domestic abuse|abuse helpline|0808 2000 247|1-800-799-7233|"#,
    r#"what's happening to you is abuse|what is happening to you is abuse"#,
);

pub(super) const ABUSER_ANSWER: &str = concat!(
    r#"0808 802 4040|people\s+who\s+are\s+violent|hitting\s+someone\s+you\s+live\s+with\s+is\s+"#,
    r#"abuse|that\s+is\s+abuse,\s+and"#,
);

pub(super) const THE_APOLOGY: &str = concat!(
    r#"\b(?:sorry|apolog\w+|wont\s+happen\s+again|won't\s+happen\s+again|"#,
    r#"promis(?:e|ed|es)|will\s+change|has\s+changed|is\s+changing|getting\s+help|"#,
    r#"doing\s+something\s+about\s+it|didn'?t\s+mean|didnt\s+mean|not\s+like\s+(?:him|her|that)|"#,
    r#"only\s+did\s+it\s+once)\b"#,
);

pub(super) const THE_KINDNESS: &str = concat!(
    r#"(?:says|said|told\s+me)\s+(?:that\s+)?(?:he|she)\s+loves\s+me|"#,
    r#"(?:he|she)\s+loves\s+me\s+really|bought\s+me|flowers|"#,
    r#"being\s+(?:really\s+)?(?:nice|sweet|lovely)\s+now|making\s+up\s+for\s+it|"#,
    r#"only\s+does\s+it\s+when|been\s+(?:really\s+)?(?:good|lovely)\s+(?:since|this\s+week)"#,
);

pub(super) const THE_DRINK: &str = concat!(
    r#"\b(?:was|were|is|gets?|been)\s+(?:drunk|drinking|on\s+something)\b|"#,
    r#"\bhe\s+drinks\b|\bwhen\s+he\s+drinks\b|\bdrinks\s+every\s+(?:day|night)\b"#,
);

pub(super) const THEIR_OWN_FAULT: &str = concat!(
    r#"\b(?:was|is|wasnt|isnt)\s+it\s+(?:my|mine)\b|\bdid\s+i\s+(?:do\s+something|make|cause|"#,
    r#"provoke|push|wind)\b|\bmy\s+fault\b|\b(?:i|we)\s+provoked\b|\bwhat\s+did\s+i\s+do\b|"#,
    r#"\b(?:i|we)\s+(?:made|make|caused|provoked|wound\s+up|annoyed)\s+"#,
    r#"(?:him|her|them|my\s+\w+)\s+(?:angry|mad|furious|do\s+it|lose\s+it|lash\s+out|hit|upset)\b|"#,
    r#"\bmaybe\s+it\s+was\s+me\b|\bprobably\s+my\b|"#,
    r#"\bhow\s+(?:do|can|should)\s+i\s+(?:make\s+(?:it|things)\s+up|make\s+it\s+right|"#,
    r#"put\s+it\s+right|make\s+amends)\b|"#,
    r#"\b(?:should|shall|do)\s+i\s+(?:apologise|apologize|say\s+sorry)\b|"#,
    r#"\bi\s+(?:should|ought\s+to)\s+(?:apologise|apologize|say\s+sorry)\b|"#,
    r#"\bmake\s+(?:it|things)\s+up\s+to\s+(?:him|her|them|my\s+\w+)\b"#,
);

pub(super) const THREAT_AGAIN: &str = concat!(
    r#"\b(?:will|would|going\s+to|gonna|wont|is\s+going\s+to)\s+(?:do\s+it|kill|hurt|hit|"#,
    r#"beat)\s+(?:me\s+)?again\b|\bnext\s+time\b|\bthreaten\w*|\bhe\s+said\s+he.?d\s+"#,
    r#"(?:kill|hurt|hit)\b|\bsaid\s+hed\s+(?:kill|hurt|hit)\b|\bhell\s+do\s+it\s+again\b"#,
);

pub(super) const NODDING_ALONG: &str = concat!(
    r#"^\W*(?:ok|okay|k|kk|thanks|thank\s+you|cheers|ta|right|yeah|yes|no|bye|goodnight|"#,
    r#"night|fair\s+enough|got\s+it)\W*$"#,
);

pub(super) const THE_TROUBLE_IS_OVER: &str = concat!(
    r#"\b(?:(?:we(?:'re| are)?|were|weve)\s+(?:okay|ok|fine|good)|all\s+good\b|"#,
    r#"its\s+fine\s+now\b|"#,
    r#"(?:he|she|they)(?:'s|s|\s+is|\s+are)?\s+(?:fine|okay|ok|alright)\s+now\b|"#,
    r#"no\s+worries\b|sorted\s+now\b)"#,
);

pub(super) const NO_WAY_OUT: &str = concat!(
    r#"\bnowhere\s+(?:to\s+go|to\s+stay|else\s+to\s+go|safe)\b|\bno\s+way\s+out\b|"#,
    r#"\bno\s+money\s+to\s+(?:leave|go)\b|\bcan'?t\s+(?:leave|afford\s+to\s+leave)\b|"#,
    r#"\bcant\s+(?:leave|afford\s+to\s+leave)\b|\bno\s+one\s+to\s+turn\s+to\b|"#,
    r#"\bdont\s+have\s+anywhere\b|\bdon'?t\s+have\s+anywhere\b"#,
);

pub(super) const SWALLOWED_NOT_RELIEF: &str = concat!(
    r#"\b(?:overdos\w*|poison\w*|swallow\w*|paracetamol|ibuprofen|bleach|antifreeze|pills|"#,
    r#"tablets|capsules|carbon\s+monoxide|took\s+(?:too|a\s+load|a\s+bunch|loads))\b"#,
);

pub(super) const TOO_MUCH_TO_CARRY: &str = concat!(
    r#"\b(?:i|we)\s+(?:can'?t|cant|couldn'?t|couldnt)\s+(?:take|do|keep\s+(?:doing\s+)?|carry|"#,
    r#"handle)\s*(?:it|this|any\s*more|anymore|much\s+more)\b"#,
    r#"|\b(?:i|we)\s+(?:can'?t|cant)\s+(?:take|do)\s+(?:any\s*)?more\b"#,
    r#"|\bi\s+(?:can'?t|cant)\s+cope\b"#,
    r#"|\b(?:its|it'?s|it\s+is)\s+too\s+much\s+(?:for\s+me|to\s+(?:take|carry|bear))\b"#,
);

pub(super) const THE_ANSWER_ASKED_A_QUESTION: &str = r#"^[^.!]*\?"#;

pub(super) const ANSWERED: &str = concat!(
    r#"^\W*(?:yes|yeah|yep|no|nope|not|its\s+just|it\s+is\s+just|just|only|nothing|"#,
    r#"bruis\w*|she\s+says|he\s+says|they\s+say|i\s+think\s+it|i\s+checked)"#,
);

pub(super) const ABOUT_THE_THREAD: &str = concat!(
    r#"\b(?:he|him|his|she|her|hers|they|them|their|we|us|our|my\s+(?:partner|husband|wife|"#,
    r#"boyfriend|girlfriend|ex|dad|mum|father|mother|son|daughter|brother|sister|friend)\b)|"#,
    r#"\b(?:what\s+should\s+i\s+do|what\s+do\s+i\s+do|how\s+do\s+i\s+get\s+out|"#,
    r#"who\s+can\s+i\s+call|where\s+can\s+i\s+go|should\s+i\s+leave)\b"#,
    r#"(?:\s+(?:now|next|though|then))?\s*(?:[.?!,]|$)"#,
);

pub(super) const CHANGE_THE_SUBJECT: &str = concat!(
    r#"\b(?:can|could|shall|would)\s+(?:we|i)\s+(?:please\s+|just\s+)?"#,
    r#"(?:talk|chat|speak)\s+about\s+(?:something|anything|summat|another\s+thing|"#,
    r#"a\s+different\s+(?:thing|subject))\b"#,
    r#"|\b(?:can|could|shall)\s+we\s+(?:please\s+|just\s+)?"#,
    r#"(?:change|drop|leave)\s+(?:the\s+subject|it|this|that)\b"#,
    r#"|\bi\s*(?:'| a)?m\s+not\s+(?:sure\s+i\s+want|going)\s+to\s+talk\s+about\b"#,
    r#"|\bi\s+do\s?n'?o?t\s+want\s+to\s+talk\s+about\s+(?:it|this|that|him|her|them)\b"#,
    r#"|\blet'?s\s+(?:not\s+)?(?:talk|chat)\s+about\s+(?:something|anything)\s+else\b"#,
    r#"|\blet'?s\s+not\s+(?:talk|chat)\s+about\s+(?:it|this|that)\b"#,
    r#"|\benough\s+about\s+(?:that|it|this)\b"#,
    r#"|\bmoving\s+on\s+from\s+(?:this|that|it)\b"#,
    r#"|\b(?:something|anything)\s+else\s+(?:instead|for\s+a\s+bit|for\s+now)\b"#,
    r#"|\b(?:im|i\s+am|ive)\s+(?:done|finished)\s+talking\s+about\b"#,
    r#"|\b(?:drop|leave|park)\s+it\s+(?:for\s+now|please)\b"#,
);

pub(super) const ABOUT_THE_INCIDENT: &str = concat!(
    r#"\b(?:its\s+not\s+that\s+bad|not\s+that\s+bad|it\s+wasn't\s+that\s+bad|only\s+once|"#,
    r#"it\s+was\s+my\s+fault|it\s+was\s+an\s+accident|he\s+didn'?t\s+mean|"#,
    r#"she\s+didn'?t\s+mean|it\s+just\s+happened)\b"#,
);

pub(super) const PACKING: &str = concat!(
    r#"\bwhat (?:should|do|can|could|shall) i (?:take|bring|pack)\b"#,
    r#"|\bwhat (?:do|will|should) i (?:need|want) to (?:take|bring|pack)\b|\bpack a bag\b"#,
    r#"|\bwhat (?:should|do) i (?:put|get) in (?:the|my) bag\b"#,
    r#"|\bwhat do i take with me\b"#,
);

pub(super) const THE_FEAR_IT_REPEATS: &str = concat!(
    r#"\bwill (?:it|this|he|she|they) (?:get|be) worse\b|\bis it going to get worse\b"#,
    r#"|\bwill (?:it|this) (?:happen|get) again\b|\bwill (?:he|she) do it again\b"#,
    r#"|\bdoes it get worse\b"#,
);
