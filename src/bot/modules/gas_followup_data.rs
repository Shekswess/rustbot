// Gas-case follow-up rules and wording from the pinned safety module.

pattern!(
    CORRECTION_OF_PERSON,
    r#"(?:\b(?:sorry|no|actually|wait)?,?\s*i\s+mean\s+(?:my|his|her|the|its)\b)|(?:\bnot\s+(?:my|the)\s+\w+\s*,?\s*(?:it'?s|its|but)\b)|(?:\b(?:i\s+meant|i\s+said)\s+(?:my|the)\b)|(?:\bwrong\s+person\b)"#
);
pattern!(
    FOLLOW_UP,
    r#"(?:\bwhile\s+(?:we|i|they|you)\s+wait\b)|(?:\bwhat\s+(?:do|should)\s+(?:i|we)\s+do\b)|(?:\bwhat\s+now\b)|(?:\banything\s+else\b)|(?:\bis\s+(?:that|it|this)\s+(?:ok|okay|right|normal|safe|dangerous)\b)|(?:\bis\s+it\s+safe\s+to\b)|(?:\bthe\s+ambulance\b)|(?:\bthey'?re\s+(?:coming|on\s+their\s+way)\b)|(?:\bhelp\s+is\s+(?:coming|on\s+the\s+way)\b)|(?:\bstill\s+waiting\b)|(?:\bshould\s+i\s+(?:use|give|move|leave|go|call|keep|do|put|take|wash|clean|rinse|cover|wrap|bandage)\b)|(?:\bcan\s+i\s+(?:give|move|leave|go|use|do|put|take|wash|clean|rinse|cover|wrap|bandage|walk|stand|sit|lie|help|carry)\b)|(?:\bwhat\s+(?:else\s+)?can\s+i\s+do\b)|(?:\bkeep\s+(?:them|him|her)\s+(?:warm|still|awake)\b)|(?:\b(?:cpr|dispatcher|call\s+handler)\b)|(?:\b(?:breathing|feels?|looks?|seems?)\s+(?:better|easier|worse|normally|again)\b)|(?:\bthey\s+(?:seem|look|are)\s+(?:better|worse|okay|ok)\b)|(?:\b(?:i'm|im|we're|we\s+are|i\s+am)\s+(?:outside|out|safe|clear)\b)|(?:\b(?:fire\s+(?:brigade|service|fighters?)|paramedics?|ambulance)\b)|(?:\b(?:he|she|they|it)(?:'s|'re|\s+is|\s+are)\s+(?:awake|breathing|okay|ok)\b)|(?:\b(?:it|the\s+bleeding)\s+(?:has\s+)?stopped\b)|(?:\b(?:called|rang|phoned)\s+(?:999|911|112)\b)|(?:\bon\s+(?:the\s+phone|hold)\b)|(?:\b(?:still|really)\s+(?:scared|frightened|terrified|afraid|shaky|panicking|panicky)\b)|(?:\bstill\s+(?:soaking|bleeding|coming|pouring)\b)|(?:\bsoaking\s+through\b)|(?:\bit'?s\s+still\s+\w+ing\b)|(?:\bshould\s+i\s+(?:try|open|shut|close|turn|grab|hold|press|cover|warm|sit|lie|stand|walk|drive|eat|drink|wake|ring|phone|tell|say|text|message|put|take|wash|clean|rinse|wrap|bandage)\b)|(?:\bcan\s+i\s+(?:open|shut|close|turn|give|move|go|drive|eat|drink|wake|ring|phone|put|take|wash|clean|rinse|cover|wrap|bandage|walk|stand|sit|lie|help|leave|try|carry|hold|press)\b)|(?:\bis\s+it\s+(?:ok|okay|safe|alright|fine)\s+to\b)"#
);
pattern!(
    FOLLOW_UP_ANCHOR,
    r#"(?:\b(?:999|911|112|ambulance|paramedics?|fire|brigade|smoke|stairs|outside|safe|clear|breath\w*|cpr|pulse|conscious|awake|asleep|bleeding|blood|burn\w*|hospital|doctor|a&e|nurse|poison\w*|pills?|tablets?|medic\w*|medicine|epipen|inhaler|police|help|waiting|wait|call\w*|now|still|they|them|him|her|she|he|it|ok|okay|done|yes|no|thanks|anything|something|else|that|this|scared|frightened|terrified|afraid|soak\w*|through|still)\b)"#
);
pattern!(
    HELP_ARRIVED,
    r#"(?:\b(?:ambulance|paramedics?|fire\s+(?:brigade|service|engine)|police|crew)\b[^.?!]{0,25}\b(?:is|are|'s|was|were)\s+(?:here|outside|coming|on\s+(?:its|their|the)\s+way|arriving|arrived|pulling\s+up|at\s+the\s+door)\b)|(?:\b(?:they|someone|somebody|my\s+(?:mum|mom|dad|husband|wife|partner|sister|brother|son|daughter|friend|neighbour|neighbor))\b[^.?!]{0,15}\b(?:is|are|'s)\s+(?:here|outside|with\s+me|on\s+(?:their|his|her)\s+way|coming\s+(?:over|round))\b)|(?:\b(?:im|i'?m)\s+not\s+on\s+my\s+own\b)"#
);
pattern!(
    HELP_ASK_MORE,
    r#"(?:\?)|(?:\b(?:what|how|should|can|could|when|else|while|wait|waiting|now|do|anything)\b)|(?:\b(?:outside|stairs|smoke|breath\w*|bleeding|burn\w*|door)\b)"#
);
pattern!(
    WHEN_HELP_COMES,
    r#"(?:\bhow\s+long\b.{0,40}\b(?:ambulance|paramedics?|fire|police|help)\b)|(?:\bwhen\s+(?:will|are|do)\b.{0,40}\b(?:ambulance|paramedics?|fire\s+(?:brigade|service)|police|they)\b.{0,20}\b(?:coming|get|here|arrive)\b)|(?:\bare\s+they\s+(?:coming|on\s+their\s+way)\b)"#
);
pattern!(
    FRIGHTENED,
    r#"(?:\b(?:can\s+you\s+hear\s+me|are\s+you\s+(?:still\s+)?there|you\s+still\s+there)\b)|(?:\bi'?m\s+(?:(?:really|so|very|that|proper|properly|absolutely|still|dead|scared\s+witless)\s+)*(?:scared|frightened|terrified|panicking|shaking|bricking\s+it)\b)|(?:\bi\s+can'?t\s+stop\s+shaking\b)|(?:\b(?:going\s+to|gonna|might|may|could)\s+lose\s+(?:him|her|them)\b)|(?:\b(?:he|she|they)\s+(?:(?:might|may|could)\s+(?:not\s+)?(?:make\s+it|die|survive)|(?:is|are)\s+going\s+to\s+die)\b)|(?:\b(?:will|is)\s+(?:he|she|they)\s+(?:going\s+to\s+)?be\s+(?:ok|okay|alright|fine)\b)|(?:\b(?:am|did)\s+i\s+do(?:ing)?\s+(?:it|this)?\s*right\b)"#
);
pattern!(
    CALL_HELP,
    r#"(?:\bshould\s+i\s+(?:call|ring|phone|dial)\b.{0,25}\b(?:ambulance|999|911|112|help|doctor)\b)|(?:\bdo\s+(?:i|we)\s+need\s+(?:an\s+)?ambulance\b)|(?:\bshall\s+i\s+(?:call|ring|phone)\b)|(?:\bis\s+it\s+(?:bad\s+enough\s+)?(?:for\s+)?an\s+ambulance\b)"#
);
pattern!(
    DETAIL_AFTER_ESCALATION,
    r#"(?:\b(?:it|this|he|she|they)\s+(?:started|began|happened|came\s+on|kicked\s+off)\b)|(?:\b(?:this\s+morning|this\s+afternoon|this\s+evening|earlier\s+today|yesterday|last\s+night|an\s+hour\s+ago|\d+\s+(?:minutes?|hours?|days?)\s+ago)\b)|(?:\bsince\s+(?:this\s+morning|yesterday|last\s+night|about\s+\d)\b)"#
);
pattern!(
    ESCALATED_ALREADY,
    r#"(?:\bshould\s+i\b)|(?:\bwhat\s+(?:do|should)\s+i\b)|(?:\bshall\s+i\b)|(?:\bcan\s+i\b)|(?:\bhow\s+(?:do|long|much)\b)"#
);
pattern!(
    FEMALE_WORDS,
    r#"(?:\bm(?:u|o)m\b)|(?:\bmother\b)|(?:\bwife\b)|(?:\bgirlfriend\b)|(?:\bsister\b)|(?:\bdaughter\b)|(?:\blittle\s+girl\b)|(?:\bgirl\b)|(?:\baunt\b)|(?:\bnan\b)|(?:\bgran\b)|(?:\bgranny\b)|(?:\bgrandma\b)|(?:\bgrandmother\b)|(?:\bniece\b)|(?:\bshe\b)|(?:\bher\b)"#
);
pattern!(
    MALE_WORDS,
    r#"(?:\bdad\b)|(?:\bfather\b)|(?:\bhusband\b)|(?:\bboyfriend\b)|(?:\bbrother\b)|(?:\bson\b)|(?:\blittle\s+boy\b)|(?:\bboy\b)|(?:\buncle\b)|(?:\bgrandad\b)|(?:\bgrandpa\b)|(?:\bgrandfather\b)|(?:\bnephew\b)|(?:\bhe\b)|(?:\bhim\b)"#
);
pattern!(
    CHILD_ANON,
    r#"(?:\b(?:my|our|her|his|their|the)\s+(?:baby|babies|toddler|newborn|infant|child|kid|little\s+one|little\s+lad|little\s+girl|little\s+boy|\d+\s*(?:year|month)s?\s*old)\b)"#
);
pattern!(
    SAFE_NOW,
    r#"(?:\b(?:i'm|im|i\s+am|we're|we\s+are|they're|they\s+are|he's|she's)\s+(?:outside|out|safe|clear)\b)|(?:\b(?:breathing|feels?|looks?|seems?)\s+(?:better|easier|normally|again)\b)|(?:\bthey\s+(?:seem|look|are)\s+(?:better|okay|ok|awake)\b)|(?:\b(?:fire\s+(?:brigade|service|fighters?)|paramedics?|ambulance)\b.{0,25}\b(?:here|arrived|came|with\s+(?:us|me))\b)|(?:\b(?:it|the\s+bleeding)\s+(?:has\s+)?stopped\b)"#
);
pub(super) const WHEN_HELP_REPLY: &[&str] = &[
    "They're usually with you within minutes for something like this: keep the door unlocked, the phone on speaker, and let the crew straight in when they knock.",
    "They are usually there quickly for something like this: leave the door unlocked and the phone on speaker, and the crew will knock and come in.",
];
pub(super) const FRIGHTENED_REPLY: &[&str] = &[
    "I'm here. You're doing the right thing by staying with them: keep the 999 or 911 call handler on speaker, say out loud what you can see, and keep talking to them until the ambulance comes.",
    "That's a frightening thing to be in the middle of. Keep the call handler on speaker, tell the handler everything you notice, and stay with the person until the ambulance arrives.",
];
pub(super) const CALL_HELP_REPLY: &[&str] = &[
    "Yes - ring 999 or 911 now if you haven't already, say what you've seen and stay on the line until the call handler tells you what to do.",
    "If you haven't already, yes: ring 999 or 911 and describe exactly what's happening. Stay on the line - the call handler can talk you through the next step.",
];
pub(super) const DETAIL_REPLY: &[&str] = &[
    "Got that - tell the 999 or 911 handler it as well, and stay with them until the ambulance comes.",
    "Tell the 999 or 911 handler that too. Stay with them and keep them talking until the ambulance arrives.",
];
pattern!(
    CASE_WORDS,
    r#"\b(?:gas|meter|windows?|doors?|flames?|lights?|switches|smell|boiler|hob)\b"#
);
pub(super) const GAS_WAITING: &str = "Stay outside and ring the gas emergency line from there; don't use matches, lighters or doorbells until the gas engineer says it's safe.";
pub(super) const GAS_AFTER: &str = "Stay outside until the gas emergency engineer says it's safe, and don't go back in before that.";
