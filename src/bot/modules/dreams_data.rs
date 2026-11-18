//! Dream frames, ordered subjects and reply pools from the pinned PyBot source.

pub(super) const SOMEONE_DIED: &str = r#"\b(?:died|dying|dead|death|passed (?:away|on)|killed|funeral|buried|gone|run over|put (?:him|her|it|them) (?:down|to sleep)|euthanis\w*|euthaniz\w*)\b"#;
pub(super) const A_FIRE: &str =
    r#"\b(?:fire|burning|burnt|burned|flames|smoke|flood|flooded|collapsed|crashing down)\b"#;
pub(super) const A_BREAK_IN: &str = r#"\b(?:robbed|robbery|burglar|burglars|burgled|intruder|break(?:ing)? in|broke in|broken into|house was broken)\b"#;
pub(super) const CHASED_OR_FALLING: &str = r#"\b(?:drown\w*|falling|fell|chased|chasing|running away|trapped|stuck|suffocat\w*|strangled|being held|can(?:no|')?t move|paralys\w*|crash\w*|accident|hit by a (?:car|bus|truck|train)|run over)\b"#;
pub(super) const ILL_IN_THE_DREAM: &str = r#"\b(?:cancer|tumou?r|stroke|heart attack|dementia|alzheimers|aids|hiv|diagnos\w*|ill|dying of|got (?:sick|ill)|terminal)\b"#;
pub(super) const A_GOOD_ONE: &str = r#"\b(?:fly|flies|flying|flew|float\w*|won|lotto|lottery|holiday|beach|love\w*|kiss\w*|my (?:nan|gran|grandma|grandad|mum|dad|dog|cat) (?:was|gave|said|came)|laughing|happy|best day|pregnant)\b"#;
pub(super) const DIED_MYSELF: &str = r#"\bi\s+(?:was\s+)?(?:died|dead|dying|killed|drowned|buried|hanging|hung|overdos\w*|killed myself|hurt myself|took my own life)\b|\bmy own (?:death|funeral)\b|\b(?:suicide|overdose)\b"#;
pub(super) const ATTACKED: &str = r#"\b(?:attacking|attacked|attacks me|hurting me|hurt me|beating me|beat me|strangling me|stabbed|stabbing|shot|shooting|raped|raping|grabbed me|somebody in (?:my|the) (?:house|room)|someone in (?:my|the) (?:house|room)|broke into|breaking in|burglar|intruder|following me|chasing me)\b"#;
pub(super) const TEETH: &str = r#"\b(?:teeth|tooth|molar|gums)\b"#;
pub(super) const AN_EX: &str = r#"\b(?:ex|ex-boyfriend|ex-girlfriend|ex-husband|ex-wife|my ex)\b"#;
pub(super) const A_BABY: &str = r#"\b(?:baby|babies|newborn|toddler|pregnant|infant)\b"#;
pub(super) const PEOPLE_HURT: &str = r#"\b(?:my (?:son|daughter|boy|girl|child|kid|children|mum|mom|mother|dad|father|partner|husband|wife|brother|sister|nan|gran|grandma|grandad|friend|niece|nephew)\b)"#;
pub(super) const A_VISIT: &str = r#"\b(?:was|were|is|are)\s+(?:in\s+(?:my|the)\s+room|here|with\s+me|still\s+alive|standing\s+(?:there|in))|came\s+to\s+see\s+me|visited\s+me|sat\s+with\s+me|spoke\s+to\s+me|said\s+hello|hugged\s+me|holding\s+my\s+hand\b"#;
pub(super) const DREAM: &str = r#"\b(dreamt|dreamed|dreaming|dream|dreams)\b"#;
pub(super) const FIRST_PERSON: &str = r#"\b(?:i|im|i'm|ive|i've|id|i'd|my|me)\b"#;
pub(super) const NAMED_A_NIGHTMARE: &str =
    r#"\b(?:nightmares?|night terrors?|(?:bad|scary|terrible|awful|horrible)\s+dreams?)\b"#;
pub(super) const A_WISH: &str = r#"\bdream(?:s|ing)?\s+of\b|\b(?:i|we)\s+have\s+a\s+dream\b(?![^.]*\b(?:last|other)\s+night)\b"#;
pub(super) const A_SCENE_OF_NOTHING: &str = r#"^(?:about\s+)?(?:it|that|this|him|her|them|one)?$"#;
pub(super) const SETTLED_IT: &str = r#"(?i)\b(?:just|only)\s+(?:a\s+|an\s+)?(?:bad\s+|weird\s+|strange\s+|silly\s+|daft\s+)?(?:dream|nightmare)\b|\b(?:it|that|this)\s+(?:was|were|is)\s+(?:only\s+|just\s+)?(?:a\s+)?(?:dream|nightmare)\b|\bnot\s+real\b|\bwasn'?t\s+real\b|\bnever\s+happened\b|\b(?:he|she|they|it)(?:'s|\s+is|\s+was|\s+were)?\s+(?:fine|ok|okay|alright|all\s+right|safe|still\s+here|still\s+alive|alive)\b"#;
pub(super) const DREAM_IS_HAPPENING_NOW: &str = r#"(?i)\b(?:woke\s+up|woken\s+up|wake\s+up|awake|awoken|still\s+(?:there|here|going|on|going\s+on|burning|bleeding|hurting|happening|smoking|happens|feel|feels|have|got)|right\s+now|just\s+now|this\s+minute|at\s+the\s+moment|and\s+i\s+(?:can'?t|cannot|cant)\s+(?:breathe|move|feel|stop)|it\s+(?:is|was)\s+real|i\s+(?:can|could)\s+smell\s+(?:smoke|gas|burning)|i\s+(?:have|took|taken|swallowed|drank|drunk)\s+(?:them|it|the|some|a\s+few|\d+|all\s+of)|\b\d+\s+(?:of\s+them|tablets|pills)|i\s+am\s+(?:bleeding|burning|stuck|trapped|drowning|having|on\s+fire)|i'?m\s+(?:bleeding|burning|stuck|trapped|drowning|having|on\s+fire|not\s+breathing)|my\s+(?:arm|leg|chest|head|back|side|stomach|belly|hand|foot|face)\s+(?:is\s+|feels\s+)?(?:numb|bleeding|swollen|hot|painful|hurts|broken|blue)|there\s+(?:is|are)\s+(?:smoke|flames?|blood|pain)|the\s+(?:smoke|flames?|alarm|bleeding)\s+(?:is|are)\s+(?:still|now|real)|(?:i\s+|we\s+)?(?:can\s+)?(?:smell|smells|smelling)\s+(?:smoke|gas|burning)|\b(?:smoke|flames?)\s+(?:is\s+|are\s+)?(?:coming|pouring|everywhere)|\balarm\s+(?:is\s+|keeps\s+)?(?:going|beeping|sounding)|\b(?:is|are)\s+(?:going|happening)\s+(?:on\s+)?now|\b(?:he|she|they|it|the\s+baby|my\s+baby)(?:'s|s|\s+is|\s+was|\s+are|\s+were)?\s+(?:not|isn'?t|wasn'?t|aren'?t|aint|ain'?t|no\s+longer)\s+(?:breathing|moving|responding|waking|conscious|with\s+us|alive)|\b(?:shes|hes|theyre|its)\s+(?:not|isn'?t)\s+(?:breathing|moving|responding|waking|conscious)|\b(?:lying|lay)\s+(?:there|on\s+the\s+(?:floor|ground|bed|road))\s+(?:now|still)|\b(?:there'?s|theres)\s+smoke\s+(?:in|coming|everywhere)|\b(?:i\s+can\s+hear|i\s+hear)\s+(?:the\s+alarm|smoke\s+alarm|sirens))\b"#;

pub(super) const DEATH_LINES: &[&str] = &[
    "Dreaming that someone died is one of the commonest dreams there is, and it is not a sign about them: it usually turns up when they have been on your mind, or when something between you is shifting. The dream tells you nothing about how that person is now.",
    "That dream is not an omen, it is your head working on the person: most people who dream of a death say the person had been on their mind, or that something with them was changing. Nothing in the dream is a warning about them.",
];

pub(super) const BREAK_IN_LINES: &[&str] = &[
    "A house broken into in a dream is the worry about being safe at home, not a warning that somebody is coming: it turns up in weeks when the flat, the money or the door has been on your mind. Check the lock in daylight and leave it there.",
    "Dreaming about a break-in is what the head does with feeling got at, and it is one of the commonest dreams there is. The dream does not tell you whether there is a safety problem now.",
    "That dream is about safety rather than about burglars: the nights after a scare often bring it. What usually helps is the dull, practical thing - a light on a timer, a bolt on the door - rather than going over the dream again.",
];

pub(super) const FIRE_LINES: &[&str] = &[
    "A fire in a dream usually reads as worry about the place rather than anything about fire - a bill, a repair, something you cannot quite keep an eye on. It tends to fade on its own; if the same one keeps coming back, write it down in the morning.",
    "Dreaming the house is on fire is a stress dream, and the thing behind it is usually a practical worry rather than any danger - money, a job, a repair waiting on somebody. If you notice actual smoke or fire after waking, get outside and call 999.",
];

pub(super) const CHASE_LINES: &[&str] = &[
    "Being chased, drowning or falling is the standard stress dream, and it usually follows a stretch where something is out of your hands. A cooler room, less alcohol and half an hour off the screen before bed take most of the edge off it.",
    "That is the shape a busy week takes in sleep - out of control, no footing, running late. The dream follows the week rather than the other way round, so it eases when the week does.",
];

pub(super) const TEETH_LINES: &[&str] = &[
    "Dreaming your teeth fall out is one of the most reported dreams there is, and it is not about your teeth: it tends to go with a spell of worry about how you come across, or a change you cannot control. It stops when that does.",
    "That dream is about being seen, not about your teeth - it is one of the commonest there is and it clusters in the weeks when somebody is judging you or a change is coming. If you wake with real tooth symptoms, ask a dentist about them.",
];

pub(super) const EX_LINES: &[&str] = &[
    "Dreaming about an ex does not mean you want them back: your head digs up whatever is left unfinished, and it often surfaces when something in your current life is unsettled. It says more about this week than about them.",
    "An ex in a dream is usually unfinished business rather than a wish - the brain reaches for an old face when something now feels unresolved. Nothing to act on.",
];

pub(super) const BABY_LINES: &[&str] = &[
    "A dream about the baby being hurt is horrible to wake from and very common on broken sleep, and nothing in it is a warning. The waking every couple of hours is what makes those dreams so vivid and so hard to shake.",
    "That one comes with the tiredness rather than with any danger: fragmented nights make frightening dreams much more likely. It gets better as the nights get longer.",
];

pub(super) const GOOD_LINES: &[&str] = &[
    "Nothing in a dream is a warning, and that one is the good kind: flying and the people you love turn up in sleep when things are going alright. Enjoy it and let it go.",
    "That is the sort the brain keeps - flying, a win, somebody you love turning up - and it usually follows a stretch that felt light. Nothing to do with it but have it.",
];

pub(super) const OWN_DEATH_LINES: &[&str] = &[
    "Dying, or being hurt, in a dream is frightening and very common, and it is not a sign about your health: it tends to come when something in life is ending - a job, a habit, a way of doing things - rather than anything about the body.",
    "Dreaming your own death reads as a change rather than a warning, and it clusters when something in the day to day is finishing. Anything about the body belongs to a doctor, not to a dream.",
];

pub(super) const ATTACK_LINES: &[&str] = &[
    "Being attacked in a dream is not a sign of anything coming: it is what a day that felt unsafe or watched turns into at night. Uneasy sleep makes it more likely, so a door checked and a proper wind-down before bed does more for it than reading anything into it.",
    "A dream like that follows a stretch of feeling got at rather than any real threat - a difficult person, a job, a row left hanging. Nothing about it is a warning, and it eases when the day does.",
];

pub(super) const VISIT_LINES: &[&str] = &[
    "Dreaming that somebody you love is with you is one of the kinder ones, and it usually comes when they have been on your mind. There is nothing to read into it, and nothing wrong with enjoying it.",
    "That sort of dream is the good kind: somebody you love, there and ordinary, while you sleep. It turns up when they are on your mind, and it means nothing more than that.",
];

pub(super) const PEOPLE_HURT_LINES: &[&str] = &[
    "Dreaming that somebody you love is hurt is the worry dream almost everybody has, and it is not a premonition: it turns up in the weeks when you are carrying them in your head. They are where they were when you went to bed.",
    "That dream is the shape worry takes at night rather than anything coming to them. It is worth noting what was on your mind about them that day, because that is what the dream was made of.",
];

pub(super) const ILL_LINES: &[&str] = &[
    "A dream about being ill frightens people because it names a disease, and it is not a sign about your body: dreams like that follow a scare, a wait for results, or somebody close being unwell. Anything you have actually noticed in the day to day is a doctor's question, not a dream's.",
    "Nothing in a dream diagnoses anything. That one usually comes after a health scare in the family or a wait for a test, and it fades once the worry does.",
];

pub(super) const PLAIN_LINES: &[&str] = &[
    "Dreams like that are usually the brain sorting the day out, and they come on strongest in a week with something on your mind. Writing it down in the morning is the usual trick for one that keeps coming back.",
    "Nothing in a dream is a warning - it is the day being filed, and a vivid one usually means a lot was going on. A pen by the bed and a few lines in the morning take the sting out of the ones that keep coming back.",
];

pub(super) const SETTLED_LINES: &[&str] = &[
    "Then nobody in it is any different for it - a dream is the head clearing, not a message about them. If the same one keeps coming back, that is worth saying to a GP, because broken sleep is usually the whole of it.",
    "Good. Dreams like that are the worry being worked through at night, and they ease off when the worry does. Sleeping badly makes them louder, so a steadier night is the thing that quiets them.",
    "That is the whole of it, then: a dream, and everyone in it where they were. Worth writing down if it comes back - the mornings after are what show whether it is a worry to see someone about or just a run of bad nights.",
];
