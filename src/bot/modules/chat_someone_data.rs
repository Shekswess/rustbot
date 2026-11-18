// Person tracking and focused follow-up wording from the pinned readers.

pattern!(WORD, r#"[a-z0-9]+(?:'[a-z]+)?"#);
pattern!(
    WHO_SAID,
    r#"\b(?:my|our)\s+(stepdaughter|grandparents|childminder|grandfather|grandparent|stepbrother|hairdresser|grandmother|stepsister|instructor|girlfriend|housemate|boyfriend|neighbour|landlord|landlady|granddad|neighbor|flatmate|daughter|sibling|partner|toddler|husband|grandma|grandpa|stepson|grandad|fiancee|manager|brother|granny|barber|sister|father|friend|mother|auntie|spouse|nephew|cousin|lodger|fiance|uncle|mumma|carer|niece|tutor|child|daddy|mummy|baby|wife|aunt|nana|gran|papa|nan|kid|mum|mam|dad|son|mom)(?:'s|s)?\b"#
);
pattern!(
    WHO_SAID_THIRD,
    r#"\b(?:his|her|their)\s+(stepdaughter|grandparents|childminder|grandfather|grandparent|stepbrother|hairdresser|grandmother|stepsister|instructor|girlfriend|housemate|boyfriend|neighbour|landlord|landlady|granddad|neighbor|flatmate|daughter|sibling|partner|toddler|husband|grandma|grandpa|stepson|grandad|fiancee|manager|brother|granny|barber|sister|father|friend|mother|auntie|spouse|nephew|cousin|lodger|fiance|uncle|mumma|carer|niece|tutor|child|daddy|mummy|baby|wife|aunt|nana|gran|papa|nan|kid|mum|mam|dad|son|mom)(?:'s|s)?\b"#
);
pattern!(
    A_BARE_PRONOUN_SUBJECT,
    r#"\A(?:and|but|so|then|now|also|still)?\s*(?:he|she|they|hes|shes|theyre)(?:'s|'re)?\b"#
);
pattern!(
    A_QUESTION_THEY_ASK,
    r#"\b(?:asks?|asked|asking|keeps? asking|kept asking|wonders?|wondered|wondering|wants? to know|wanted to know|says?|said|tells?|told|thinks?|thought|forgets?|forgot|remembers?|remembered)\b[^.?!]*?\b(?:where|what|when|why|how|who|if|whether)\b"#
);
pattern!(
    A_JOB_AT,
    r#"\b(?:works?|working|worked|starts?|started|starting|got a job|gets a job|getting a job|hired|employed|shifts?|night shift|day shift|late shift|early shift|job|jobs|career|boss|colleague|colleagues|manager|salary|wages|pay|overtime|contract)\b[^.!?]{0,40}?\b(?:at|in|on|for|as)\b[^.!?]{0,14}?\b(?:the\s+)?(?:hospital|ward|unit|icu|itu|a&e|a and e|care home|nursing home|hospice|clinic|surgery|pharmacy|practice)\b|\b(?:my|his|her|their|our)\s+(?:new\s+)?(?:job|work|shift)\b|\b(?:works?|working|worked)\s+(?:there|here|nights|days|shifts|full time|part time)\b"#
);
pattern!(
    SOMETHING_WRONG,
    r#"\b(?:ill|unwell|poorly|sick|sickness|pain|painful|hurts?|hurting|aching|aches|worse|worst|fell|fallen|falling|fainted|collapsed|diagnosed|diagnosis|dementia|alzheimers|cancer|operation|surgery|admitted|in hospital|on the ward|intensive care|icu|a&e|cant|cannot|can'?t|unable|stopped|keeps|kept|forgetting|forgetful|confused|confusion|weak|weaker|breathless|bleeding|infection|temperature|tests?|results?|scan|appointment|worried|worrying|worry|decline|declining|not (?:been )?(?:himself|herself|themselves)|unsteady|dizzy)\b"#
);
pattern!(
    CHANGE,
    r#"\b(?:new|newly|worse|worsening|suddenly|all of a sudden|since|these days|lately|recently|stopped|no longer|used to|gone downhill|going downhill|declin(?:e|ed|ing)|getting (?:worse|more|confused|forgetful)|(?:this|last) (?:week|month|couple of|few)|over the (?:last|past))\b"#
);
pattern!(
    REPEATED_SAID,
    r#"\brepeat(?:s|ing)? the same (?:story|tale|question)|\bsay(?:s|ing)? (?:me |us )?the same (?:thing|story|tale|question)|\btel(?:l|ls|ling) (?:me |us )?the same (?:thing|story|tale)|\bask(?:s|ing)? the same question\b"#
);
pattern!(
    A_CHILDS_PERSON,
    r#"\b(?:son|daughter|kid|kids|child|children|little one|little boy|little girl|baby|toddler|newborn|schoolboy|schoolgirl)\b|\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|\d{1,2})[\s-]*(?:year|yr)s?[\s-]*old\b"#
);
pattern!(
    A_GROWN_AGE,
    r#"\b(?:1[89]|[2-9]\d)\b|\b(?:eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)\b"#
);
pattern!(
    A_WORRY_AFTER,
    r#"\b(?:but|however|though|except)\b|\bstill\s+(?:very\s+|quite\s+|really\s+)?(?:weak|poorly|unwell|bad|badly|tired|no better|not himself|not herself|not themselves)\b"#
);
pattern!(
    YOUNG_PERSON,
    r#"\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|[1-9]|1[0-2])\s*(?:months?|years?)?\s*old\b|\bis (?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b|\b(?:baby|toddler|newborn|little one|little boy|little girl|schoolboy|schoolgirl|reception class)\b"#
);
pattern!(
    FALLS_FINE,
    r#"\b(?:laughed|laughed it off|no harm|wasn'?t hurt|was not hurt|unhurt|not a scratch|slept it off|got straight (?:back )?up|jumped (?:straight )?up|ran off|carried on|waved it off|no damage)\b|\b(?:he|she|they|it|hes|shes|theyre)(?:'s|'re)?\s+(?:is |was |seems |looks )?(?:fine|ok|okay|alright)\b"#
);
pattern!(
    FELL_OUT,
    r#"\bf(?:ell|all|alls|alling|allen)\s+out\b(?!\s+of\b)|\bfell\s+about\b"#
);
pub(super) const INSIDE_A_SAFETY_THREAD: &[&str] = &["carers_turn", "safety_cards"];
pattern!(
    PERSON_TROUBLE_0,
    r#"\b(?:out of|home from|back from|discharged from|released from|let out of|came out of|come out of|got out of|getting out of)\s+(?:the\s+)?hospital\b"#
);
pattern!(
    PERSON_TROUBLE_1,
    r#"\b(?:admitted|taken|took|rushed|whisked|brought|moved|transferred)\s+(?:in|into|to|back to)\s+(?:the\s+)?(?:hospital|ward|icu|itu|a&e|a and e|resus)\b|\b(?:went|gone|been|is|are|was|were|kept|staying|stuck)\s+(?:in|into|back in|back into)\s+(?:the\s+)?hospital\b|\bwent (?:in)?to (?:the )?hospital\b(?![^.?!]{0,18}\b(?:visit|see|collect|pick|drop|meet)\b)|(?<!work )(?<!works )(?:on|in)\s+the\s+(?:ward|stroke unit|coronary care)\b|\bkept in (?:overnight|in|for observation|for tests)\b|\b(?:in|into) (?:intensive care|icu|itu|resus)\b|\bdiagnosed (?:with|as having)\s+(?:cancer|a (?:brain |lung |bowel |breast |bone )?tumou?r|leukaemia|leukemia|lymphoma|motor neurone|parkinsons?|a terminal|something serious)\b"#
);
pattern!(
    PERSON_TROUBLE_2,
    r#"\b(?:in|at|on|still in) (?:the )?hospital\b|\bhad an operation\b|\boperation\b|\bsurgery\b|\bbeing (?:seen|treated|looked after)\b"#
);
pattern!(
    PERSON_TROUBLE_3,
    r#"\b(?:diagnosed|diagnosis|has dementia|has alzheimers|already (?:has|had|sees)|since the diagnosis|under the (?:doctor|hospital|memory clinic))\b"#
);
pattern!(
    PERSON_TROUBLE_4,
    r#"\b(?:care home|nursing home|residential home|in a home now|a home now)\b"#
);
pattern!(
    PERSON_TROUBLE_5,
    r#"\b(?:forget(?:s|ting|ful)?|forgot|memory|remember(?:ing)? (?:things|where|names)|confus(?:ed|ing)|muddled|dementia|alzheimers?|wandering|getting lost|gets? lost|got lost|went missing|lost (?:her|his|their) way|wander(?:s|ed|ing)? off|took a wrong turn|losing track|repeat(?:s|ing)? (?:himself|herself|themselves|the same (?:thing|story|tale|question))|say(?:s|ing)? (?:me |us )?the same (?:thing|story|tale|question)|tel(?:l|ls|ling) (?:me |us )?the same (?:thing|story|tale)|ask(?:s|ing)? the same question|wrong (?:name|word|words|thing)|call(?:s|ed)? (?:me|him|her|us|them) (?!(?:all|every|each|when|if|about|back|later|again|whenever|once|twice|from|at|in|on|to|for|with|so|yesterday|today|tomorrow|constantly|loads|lots|more|less|too|up|out|off|over|and|or|but|urgent)\b)\w+|mixed up|says? the same thing|told me the same|the wrong day)\b"#
);
pattern!(
    PERSON_TROUBLE_6,
    r#"\b(?:fell|fallen|fall|falls|falling|tripped|tripping|trip|unsteady|wobbly on (?:his|her|their) feet|balance)\b"#
);
pattern!(
    PERSON_TROUBLE_7,
    r#"\b(?:low mood|low spirits|feeling low|seems? low|so low|very low|a bit low|won'?t go out|wont go out|not eating|off (?:his|her|their) food|not themselves|no appetite)\b"#
);
pattern!(
    PERSON_TROUBLE_8,
    r#"\b(?:not sleeping|not sleeping through|sleeping badly|not sleeping well|up all night|awake all night|waking (?:up )?(?:all|every) night|not sleeping at night)\b"#
);
pattern!(
    PERSON_TROUBLE_9,
    r#"\b(?:poorly|unwell|chest infection|keeps (?:getting|having) (?:a |an |the )?(?:\w+ ){0,2}(?:infections?|colds?|coughs?|cough|temperatures?|chest|bugs?|viruses?|virus|migraines?|headaches?|nosebleeds?|faints?|blackouts?|tremors?|shakes?|dizzy|dizziness|breathless|sick|unwell|worse|pain|pains|swollen)|getting worse|got worse|in pain|aches?|aching|breathless|swollen|dizzy|dizziness|doctor said|hospital|diagnosed)\b"#
);
pub(super) const PERSON_TROUBLE: &[(
    super::TroubleKind,
    &std::sync::LazyLock<fancy_regex::Regex>,
)] = &[
    (super::TroubleKind::Homeward, &PERSON_TROUBLE_0),
    (super::TroubleKind::Hospital, &PERSON_TROUBLE_1),
    (super::TroubleKind::Hospital, &PERSON_TROUBLE_2),
    (super::TroubleKind::Known, &PERSON_TROUBLE_3),
    (super::TroubleKind::Care, &PERSON_TROUBLE_4),
    (super::TroubleKind::Memory, &PERSON_TROUBLE_5),
    (super::TroubleKind::Falls, &PERSON_TROUBLE_6),
    (super::TroubleKind::Low, &PERSON_TROUBLE_7),
    (super::TroubleKind::Sleep, &PERSON_TROUBLE_8),
    (super::TroubleKind::Ill, &PERSON_TROUBLE_9),
];
pub(super) const SOMEONE_REPLY: &[(&str, &[&str])] = &[
    (
        "care",
        &[
            "Ask the staff what they notice day to day and get it written in the home's notes — they see more of your {who} than you can, and a GP takes their record seriously. Visit at different times of day so you see it for yourself too.",
            "A home keeps a daily record, so ask what is in it about your {who}: eating, sleep, and anything that has changed. That is the account a GP or the manager can act on, and it costs you one conversation at the desk.",
        ],
    ),
    (
        "diagnosed",
        &[
            "I'm sorry - a diagnosis like that changes the shape of a week. How is your {who} in themselves?",
            "Worrying news. What have they said about what happens next, and who is looking after your {who}'s care?",
            "That is a lot to take in, and it takes time to sink in. Has anyone walked you through the treatment yet?",
        ],
    ),
    (
        "falls",
        &[
            "A fall is worth getting looked at even when nothing hurts, and a second one is worth a GP visit: balance, eyesight and medicines are the usual causes and all three can be helped. Clear the loose rugs and put a lamp within reach of the bed in the meantime.",
            "Falls are rarely just age, and that is the good news: strength, eyesight and medication are the places to look, and a GP can start on all three. Until then, good light on the stairs and nothing to trip over on the way to the bathroom.",
        ],
    ),
    (
        "homeward",
        &[
            "That's a relief — home is where most of the getting better happens. How is your {who} in themselves?",
            "Good news. Keep the first week small: short visits, early nights, and let your {who} set the pace rather than the other way round.",
            "Home, then — that is the bit that matters. The first days back are the ones to keep quiet, because everyone tries to do too much in them.",
            "Glad to hear it, and home beats a ward for getting better. Ask what to watch for in your {who}'s first fortnight, and who to ring if something changes.",
        ],
    ),
    (
        "hospital",
        &[
            "Sorry to hear that. How is your {who} doing?",
            "That is a worrying place for your {who} to be. What has the hospital said?",
            "Worrying, and waiting is the worst part of it. How is your {who} in themselves?",
        ],
    ),
    (
        "ill",
        &[
            "Get your {who} looked at rather than waiting it out, and sooner if it is getting worse — a change like that in someone you are looking after is worth a proper check.",
            "Worth a GP appointment for your {who} this week rather than next month, and keep a note of what has changed and when. If it turns sudden, that is the same day.",
        ],
    ),
    (
        "known",
        &[
            "Since there is already a diagnosis, make this change the GP's problem rather than yours to watch: a step down like that is often something treatable, and an infection is the usual one they check for first.",
            "Ring the GP about your {who} rather than waiting for the next appointment — a change that comes on over days rather than months is often something with a fix, and worth saying today.",
        ],
    ),
    (
        "low",
        &[
            "Going quiet and off things is worth saying out loud to a GP — it is common, it is treatable, and people often do not mention it themselves. One small thing a week that gets your {who} out, even a short walk, does more than a big plan.",
            "That kind of withdrawal is worth a GP appointment rather than a wait: low mood in later life is under-reported and it responds to help. In the meantime, keep the visits going even when they say no — company is the bit that works.",
        ],
    ),
    (
        "memory",
        &[
            "Losing track of things in a familiar place is worth a word with your {who}'s GP, and soon: it can be a lot of things and several of them are fixable. A note of when it started, and anything else that has changed, is what they ask for first.",
            "That is worth mentioning to a GP rather than waiting to see if it settles. Write down what you have noticed about your {who} and when it started, and take that with you — and if it gets suddenly worse, that is the same day rather than next week.",
            "Worth a GP appointment with your {who}, and take this line with you: forgetting things in a place they know well is the kind of change that gets looked at properly, and the sooner the easier it usually is.",
        ],
    ),
    (
        "sleep",
        &[
            "Broken sleep is worth reading rather than enduring: pain, a new tablet, drinking later, a room that is too warm, worry — one of those is usually behind it, and the fix is different for each. If it has been weeks, or they are worn out all day, ring the GP: a blood test rules out the physical ones.",
            "A night that is nothing like their usual is a change worth having looked at, and sooner if it came on suddenly. Ring the GP this week and say when it started and what is different about the nights now.",
        ],
    ),
];
pub(super) const SOMEONE_NORMAL: &[(&str, &str)] = &[
    (
        "falls",
        "One fall can be bad luck. Two, or one with no obvious cause, is not something to keep an eye on alone — balance and medication are worth a proper look.",
    ),
    (
        "hospital",
        "However it looks today, ask the ward what they expect next — they would rather answer a question than have you guess.",
    ),
    (
        "ill",
        "A change that comes on over days is the one to act on quickly; over months, it still wants a check rather than waiting for it to pass.",
    ),
    (
        "known",
        "A step down over days is worth a call the same day; the slow changes are the ones to raise at the next appointment.",
    ),
    (
        "low",
        "Going quiet for a few weeks after a change is common. Months of it, or losing interest in everything, deserves an appointment rather than patience.",
    ),
    (
        "memory",
        "Some forgetting is ordinary with age; getting lost in a room they know well is the kind that gets looked at, and a GP can tell the difference properly.",
    ),
    (
        "sleep",
        "Sleep goes in patches — a bad week after a change is common, and so is a night that is nothing like the usual. Weeks of it, or days that are wrecked by it, is worth a look rather than patience.",
    ),
];
pub(super) const SOMEONE_NEXT: &[(&str, &str)] = &[
    (
        "falls",
        "Ask about balance and a medication review in the same appointment, and get the eyes checked: those three cover most of it. At home, good light on the stairs and nothing to trip over on the way to the bathroom.",
    ),
    (
        "hospital",
        "Ask the ward what the plan is and who to ring if something changes at night; write it down, because it is hard to remember later.",
    ),
    (
        "ill",
        "Take a note of what has changed and when, and ring rather than wait for it to get worse. Sudden and getting worse is the same day.",
    ),
    (
        "known",
        "Note anything that changed in the last few days — eating, drinking, sleep, a temperature — and ring the GP with it; those are the questions they ask first.",
    ),
    (
        "low",
        "Go along if your {who} will let you, and say the things they will not. Keep the visits and the small invitations going — they matter more than advice.",
    ),
    (
        "memory",
        "Go with your {who} and take a written note: what you have noticed, when it started, and anything that changed around then. The GP will ask for exactly that, and you will be the one who knows.",
    ),
    (
        "sleep",
        "Same wake-up time every day, no caffeine after early afternoon, a cool dark room, and out of bed if they are awake more than twenty minutes. If it is weeks, ask the GP to check their bloods and what they take.",
    ),
];
pub(super) const SOMEONE_AGAIN: &[(&str, &str)] = &[
    (
        "falls",
        "Note where and when it happened, and whether anything was different - a new tablet, a bad night, a rush to the loo. That pattern is the useful bit.",
    ),
    (
        "hospital",
        "Ask them, while you are there: what the operation was for, and what normal looks like afterwards.",
    ),
    (
        "ill",
        "Keep a note of the date and what changed: three of anything in one winter is worth saying out loud to the GP.",
    ),
    (
        "known",
        "Write it down as it happens, with the date: the ones that pile up are what they ask about.",
    ),
    (
        "low",
        "Note what has changed around the same time - a move, a loss, a new tablet - and say how many days it has been like that.",
    ),
    (
        "memory",
        "Note that one down with the date: the examples are what a GP works from, and a few of them over a few weeks say far more than one on its own.",
    ),
    (
        "sleep",
        "Note the nights down - when they woke, how long for, and what they had to drink or take before bed. Three nights written down says more than a month remembered.",
    ),
];
pattern!(
    SOMEONE_TROUBLE_ANY,
    r#"\b(?:keeps?|kept) (?:forgetting|getting|losing|falling|waking|saying|coughing|stopping|forgetting|crying|asking|being)\b|\b(?:wont|won'?t|cant|can'?t|refuses? to|has stopped|stopped)\b|\bnot (?:been\s+)?(?:himself|herself|themselves|coping|eating|drinking|sleeping|walking|right)\b|\b(?:hasn'?t|hadn'?t|haven'?t)\s+been\s+(?:himself|herself|themselves|coping|eating|drinking|sleeping|walking|right)\b|\b(?:went|been) downhill\b|\bhas been (?:poorly|unwell|off|poorly)\b"#
);
pattern!(
    SOMEONE_AGES_IT,
    r#"\b(?:is|was|are) (?:that|this|it) (?:(?:just|only|part of|down to|because of) (?:getting |being )?(?:old age|her age|his age|their age|age|old)|(?:normal|usual|expected) for (?:her|his|their) age)\b|\bjust (?:her|his|their) age\b|\bat (?:her|his|their) age\b|\bold age\b"#
);
pattern!(
    SOMEONE_IS_IT_NORMAL,
    r#"^(?:and |but |so )?(?:is|was|are) (?:that|this|it) (?:normal|ok|okay|fine|serious|bad|dangerous|common|a worry)\b|\bshould i (?:be )?worried\b|\bis that a sign\b|\bwhen (?:should|do|would) i (?:start to )?worry\b|\bhow worried should i be\b|\b(?:is|was|are) (?:that|this|it) (?:(?:just|only|part of|down to|because of) (?:getting |being )?(?:old age|her age|his age|their age|age|old)|(?:normal|usual|expected) for (?:her|his|their) age)\b|\bjust (?:her|his|their) age\b|\bat (?:her|his|their) age\b|\bold age\b"#
);
pattern!(
    YOUNG,
    r#"\b(?:is|only|just|nearly|almost) (?:one|two|three|four|five|[1-5])\b|\b\d+ (?:months?|years?)\b|\bbaby|toddler|newborn\b"#
);
pattern!(
    SOMEONE_NEXT_STEP,
    r#"\bwhat (?:should|can|do) i (?:be )?(?:look(?:ing)? out for|watch(?:ing)? (?:for|out) for|expect|do(?: next)?|do now|ask(?: them| him| her| the doctor| the gp| the ward)?|say(?: to them)?|bring|take with me)\b|\bshould i (?:go|come|be there) with\b|\bwhat happens (?:now|next)\b|\bhow do i (?:help|manage|handle)\b|\bis there anything i can do\b"#
);
pub(super) const SOMEONE_NORMAL_ANY: &str = "Some of it is age and some is not, and the difference is what changed and how fast: a slower, steadier version of the same person is the ordinary kind, and a change over days or weeks, or a stop in something they always did, is worth a GP appointment. A note of what you have seen and when is what makes that appointment useful.";
pattern!(
    SOMEONE_MORE,
    r#"\b(?:another|again|twice|three times|second time|every (?:day|night|week|morning)|now he|now she|hes done it|shes done it|same again)\b"#
);
pattern!(
    LIMIT,
    r#"\bcan'?t\b|\bcant\b|\bisn'?t able\b|\bnot able\b|\bonly manage\b"#
);
pattern!(
    PLAN_ASK,
    r#"\bwhat (?:could|can|should|shall) (?:i|we) do\b|\bwhat would you do\b|\bwhere (?:could|can|should) (?:i|we) (?:go|take)\b|\bany ideas\b"#
);
pattern!(
    MAYBE_SOMEONE,
    r#"^(?:and |but |so |well |also |see |then |now |oh )*(?:she|he|they)\b|\b(?:my|our)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|baby|partner|husband|wife|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|friend|flatmate|neighbour|neighbor)\b"#
);
pattern!(
    LOSS,
    r#"\b(?:died|dying|death|dead|passed away|funeral|burial|cremation|bereave\w*|lost my (?:mum|mom|dad|father|mother|wife|husband|son|daughter|brother|sister|partner|baby|child)|(?:mum|mom|dad|father|mother|wife|husband|son|daughter|brother|sister|partner|gran|nan|grandad)\s+(?:has|have|had)\s+(?:just )?(?:died|passed))\b"#
);
pattern!(
    END_OF_ROPE,
    r#"\b(?:can'?t|cannot) (?:do this|go on|take it|cope|keep going)\b|\bend(?:ing)? it\b|\bkill myself\b|\bhurt myself\b|\bwant to die\b|\bno point\b|\bhearing things\b|\bseeing things\b|\bnot worth it\b"#
);
pattern!(
    FEAR,
    r#"\b(?:worried|worrying|nervous|anxious|scared|afraid|frightened|terrified|dreading)\s+(?:about|of|that)\b"#
);
pattern!(
    SERIOUS_DIAGNOSIS,
    r#"\b(?:diagnosed|diagnosis) (?:with|of|as having)\s+(?:cancer|a (?:brain |lung |bowel |breast |bone )?tumou?r|leukaemia|leukemia|lymphoma|motor neurone|parkinsons?|a terminal|something serious)\b"#
);
pattern!(
    SYMPTOM,
    r#"\b(?:pain|painful|ache|aches|aching|achy|hurt|hurts|hurting|cramp|cramps|cough|coughing|tooth|toothache|toothaches|headache|headaches|migraine|migraines|earache|earaches|backache|stomachache|tummyache|heartburn|reflux|nausea|nauseous|sick|fever|itch|itching|sore|swollen|stiff)\b"#
);
pub(super) const SOMEONE_ASK: &[&str] = &[
    "How long has that been going on with your {who}?",
    "What has your {who} said about it — and how long has it been like this?",
    "Has your {who} seen anyone about it, or is that the next thing?",
];
pattern!(
    SOMEONE_BRING_SOMEONE,
    r#"\bwhat (?:should|shall|can|do) i (?:bring|take)\s+(?:(?:for|to)\s+)?(?:him|her|them)\b"#
);
pattern!(FIRST_PERSON, r#"\b(?:i|im|i'm|me|my|mine|us|our|we)\b"#);
pattern!(
    THING_BEFORE_CANNOT,
    r#"\b(?:it|it's|its|that|that's|thats|this|there|there's|theres|which|the \w+|a \w+|an \w+|any \w+)\s+(?:probably |still |just |really |even )?(?:can'?t|cant|cannot|can not|won'?t|wont|will not)\b"#
);
pattern!(
    SAID_WRONG,
    r#"\bi (?:(?:keep|always|still|just|also|often|usually|sometimes|constantly)\s+){0,2}(?:said|say|told|tell)\s+(?:the wrong thing|something\s+(?:stupid|awful|daft|silly|dumb|mean|horrible|wrong|embarrassing|terrible)|it badly|that badly)\b|\bi (?:put|putting|put my)\b[^.]{0,12}\bfoot in it\b|\bi (?:really |kind of |sort of )?(?:snapped|shouted|yelled|swore)\s+(?:at|to|with)\s+(?:myself|yourself|ourselves)\b|\bi (?:was|been|were) (?:rude|horrible|mean|cruel|sharp|short|unfair|awful|dreadful|terrible|nasty|unkind) (?:to|with)\b(?! be\b)|\bi (?:was|been|were) off with\b|\bi (?:upset|offended|insulted|embarrassed|hurt)\s+(?:my|his|her|their|our)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|baby|partner|husband|wife|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|friend|flatmate|neighbour|neighbor|boss|colleague|team)\b|\bi (?:hurt|upset|offended) (?:her|him|them|someone|somebody)(?:'s)? feelings\b|\bi (?:said|told)\b[^.]{0,40}\b(?:was|were|is) (?:awful|horrible|stupid|mean|rude|unkind|dreadful|terrible|nasty|rubbish|disgusting|pathetic)\b"#
);
pattern!(
    SAID_WRONG_NOT,
    r#"\bi (?:hurt|upset|pulled|broke|twisted|strained|cut|burnt|burned) my\b|\bi snapped my\b|\bi said (?:nothing|no|yes|okay|ok|bye|hello|hi)\b"#
);
pattern!(
    SAID_WRONG_LOST,
    r#"\b(?:didnt|didn't|did not|never)\s+(?:get|got|win|won|pass|passed|make|made)\b|\b(?:got|been) (?:fired|sacked|dumped|rejected|turned down)\b|\bi (?:lost|lose|failed)\b|\b(?:she|he|they|we)\s+(?:left|walked out|split|broke up|stopped talking|moved out)\b|\b(?:wont|won't|wouldnt|wouldn't) (?:talk|speak) to me\b|\bthere'?s no fixing\b|\bit'?s too late\b|\bno longer (?:speaks?|talks?|friends?)\b"#
);
pattern!(
    SAID_WRONG_TO_AN_ANIMAL,
    r#"\b(?:to|with|at)\s+(?:my|our|the|a)\s+(?:dog|cat|puppy|kitten|horse|pony|rabbit|hamster|guinea pig|budgie|parrot|pet|fish|bird|chickens?|sheep|cow|goat|lamb)\b"#
);
pattern!(
    SAID_WRONG_TO_A_PROFESSIONAL,
    r#"\b(?:to|at|with)\s+(?:my|our|the)\s+(?:doctor|gp|nurse|consultant|surgeon|midwife|health visitor|dentist|orthodontist|physio(?:therapist)?|pharmacist|optician|vet|solicitor|accountant|bank|landlord|letting agent)\b[^.?!]*\babout\b"#
);
pattern!(
    I_SAID,
    r#"\bi (?:(?:keep|keeps|always|still|just|also|then|really|often|usually|sometimes|constantly|forever|never)\s+){0,2}(?:said|say|saying|told|tell|telling|admit|admitted)\b"#
);
pattern!(
    SAID_UNKINDLY,
    r#"\b(?:said|says|saying|told|tells|shouted|yelled|snapped|swore|swears|texted|messaged)\b[^.]{0,30}\b(?P<word>wrong thing|stupid|awful|daft|silly|dumb|mean|horrible|unkind|rude|dreadful|terrible|nasty|hurtful|disrespectful)\b"#
);
pattern!(
    FEELING_CLAUSE,
    r#"(?:^|[.;!?]|\band|\bbut|\bso|\bbecause|,)\s*(?:i|we)\s+(?:(?:just|really|also|still|honestly|actually|then)\s+)?(?:feel|felt)\s+(?:so |very |really |quite |pretty |a bit |kind of |like |such )?$"#
);
pattern!(
    CLAUSE_BOUNDARY,
    r#"[.;!?,]|\band\b|\bbut\b|\bso\b|\bbecause\b|\bthen\b|\bwhich\b"#
);
pattern!(
    FIRST_PERSON_DEED,
    r#"^\s*(?:and\s+|but\s+|so\s+|then\s+|because\s+)?(?:i|we)\b"#
);
pattern!(
    A_DATE_OF_OURS,
    r#"\b(?:forget(?:s|ting|ful)?|forgot)\b[^.]{0,24}?\b(?:our|my)\s+(?:anniversary|birthdays?|dates?(?: night)?|valentine'?s|wedding day|christmas|plans?)\b"#
);
pattern!(
    A_WORRY_ABOUT_THEM,
    r#"\b(?:something|summat)\s+(?:is|seems|feels)\s+(?:wrong|off|not right)\b|\bnot (?:like|acting like) (?:him|her|them|himself|herself|themselves)\b|\bnot (?:quite |really |been )?(?:himself|herself|themselves)\b|\bnot (?:his|her|their) usual (?:self|selves)\b|\b(?:worse|worrying)\b|\bworried about (?:him|her|them)\b|\bshould i (?:be )?worr(?:y|ied)\b|\bdementia\b|\balzheim\w*\b|\bconfus(?:ed|ion)\b|\bmemory\b"#
);
pub(super) const SUDDEN_LOSS: &[&str] = &[
    "I'm so sorry. That's a real loss, and it deserves to be taken seriously — who have you got around you this week?",
    "I'm really sorry. Take the time you need, and tell me what would help today.",
    "That's heartbreaking, and I'm sorry. Is there someone you can be with today?",
];
pattern!(
    AMENDS,
    r#"\b(?:make|making)\s+(?:[\w']+\s+){0,6}up\s+to\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b|\b(?:make|making|made)\s+(?:it\s+)?up\s+to\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b|\bmake\s+(?:it\s+)?up\s+with\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b|\bmake\s+(?:some\s+)?amends\b|\bput\s+(?:it|things)\s+right\s+(?:with|to)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b"#
);
pattern!(
    PET_OR_PROFESSIONAL,
    r#"\b(?:to|with|at)\s+(?:my|our|the|a)\s+(?:dog|cat|puppy|kitten|horse|pony|rabbit|hamster|guinea pig|budgie|parrot|pet|fish|bird|chickens?|sheep|cow|goat|lamb|doctor|gp|nurse|consultant|surgeon|midwife|health visitor|dentist|orthodontist|physio(?:therapist)?|pharmacist|optician|vet|solicitor|accountant|landlord|letting agent|bank)\b"#
);
pattern!(
    PHYSICAL_HARM,
    r#"\b(?:hit|hits|hitting|punch|punched|slap|slapped|push|pushed|shove|shoved|shook|shake|shaking|kick|kicked|throw|threw|thumped|strangle|strangled|bit|biting|dragged|grabbed)\b|\b(?:hurt|hurting|injured|burned|burnt|bruised)\s+(?:my|her|his|their|our)\s+(?:baby|child|kid|kids|son|daughter|wife|husband|partner|girlfriend|boyfriend|mum|mom|dad|mother|father|sister|brother)\b(?!'?s?\s+feelings)"#
);
pattern!(
    GRIEF,
    r#"\b(?:died|dies|dying|death|deaths|passed away|passed on|funeral|burial|cremation|grief|grieving|bereav\w*|widow|widower|widowed|estate|the will|his will|her will)\b"#
);
pattern!(
    A_BABY,
    r#"\b(?:baby|babies|newborn|newborns|infant|infants)\b"#
);
pattern!(
    REGRET_FIRST,
    r#"^\s*(?:(?:honestly|god|tbh|ugh|so|ok|okay)\s+)*i\s+(?:just\s+|really\s+|deeply\s+|so\s+|still\s+)*regret\w*\b"#
);
pattern!(
    NOT_A_DEED,
    r#"\b(?:let|letting)\s+(?:my|her|his|their|your)\s+hair\s+down\b|\bi (?:hurt|upset|pulled|broke|twisted|strained|cut|burnt|burned|dislocated|sprained)\s+my\s+(?:back|knee|ankle|shoulder|arm|leg|foot|feet|hand|finger|wrist|neck|head|hip|stomach|toe|ribs?|thumb|elbow|eye|teeth|tooth|ear|throat|chest|spine|hamstring|calves|calf|shin|jaw|skin|appetite|weight|sleep|hair)\b|\bi (?:lost|lose|losing)\s+my\s+(?:keys|phone|wallet|purse|job|way|mind|trainers|glasses|vape|earbuds?)\b|\bi (?:am|m|feel|felt|been)\s+(?:so |really |very |a bit )?(?:lost|stuck|short of|out of)\b|\bi (?:never|don'?t|didnt|didn'?t|hardly ever|rarely)\s+(?:let|lied|lie|forget|lose|lost|shout|yell|snap|swear|cancel|ignore|blank)\b"#
);
pattern!(
    NOBODY_TO_REPAIR,
    r#"\b(?:didnt|didn'?t|did not|never)\s+(?:get|got|win|won|pass|passed|make|made)\b|\b(?:got|been) (?:fired|sacked|dumped|rejected|turned down)\b|\bthere'?s no fixing\b|\bit'?s too late\b"#
);
pattern!(
    REPAIR_LAPSES_0,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:forgot|forget|forgetting|have forgotten|ve forgotten)\s+to\s+(?:call(?:ed|ing)?|ring|rang|rung|ringing|text(?:ed|ing)?|messag(?:e|ed|ing)|repl(?:y|ied|ying)|writ(?:e|ing|ten)|visit(?:ed|ing)?|wish(?:ed|ing)?|send|sent|sending)\b[^.!?]{0,30}?(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)"#
);
pattern!(
    REPAIR_LAPSES_1,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:didnt|didn'?t|did not|never|havent|haven'?t|have not|hadnt|hadn'?t|had not|hasnt|hasn'?t|has not)\s+(?:call(?:ed|ing)?|ring|rang|rung|ringing|text(?:ed|ing)?|messag(?:e|ed|ing)|repl(?:y|ied|ying)|writ(?:e|ing|ten)|visit(?:ed|ing)?|wish(?:ed|ing)?|send|sent|sending)(?:\s+to)?\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)'?s?\b"#
);
pattern!(
    REPAIR_LAPSES_2,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:havent|haven'?t|have not|hadnt|hadn'?t|had not)\s+(?:seen|saw|visited|been\s+to\s+see)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\s+in\b"#
);
pattern!(
    REPAIR_LAPSES_3,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:didnt|didn'?t|did not|never)\s+(?:get|got|buy|bought|send|sent)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\s+(?:anything|nothing|a\s+(?:present|gift|card))"#
);
pub(super) const REPAIR_LAPSES: &[&std::sync::LazyLock<fancy_regex::Regex>] = &[
    &REPAIR_LAPSES_0,
    &REPAIR_LAPSES_1,
    &REPAIR_LAPSES_2,
    &REPAIR_LAPSES_3,
];
pattern!(
    REPAIR_DEEDS_0,
    r#"(?:\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}|\b(?:about|for|after|of)\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4})(?:let|lets|letting)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\s+down\b"#
);
pattern!(
    REPAIR_DEEDS_1,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:lied|lie|lying|lies)\b[^.!?]{0,20}?\bto\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b"#
);
pattern!(
    REPAIR_DEEDS_2,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:told|tell|have told|ve told)\s+(?:(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\s+)?(?:a\s+)?lie\b"#
);
pattern!(
    REPAIR_DEEDS_3,
    r#"\bi\s+(?:(?:keep|always|completely|totally|just|nearly|almost|went and|went)\s+){0,3}(?:forgot|forget|forgetting|forgotten)\b[^.!?]{0,40}?\b(?:birthday|anniversary)\b"#
);
pattern!(
    REPAIR_DEEDS_4,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:missed|miss)\s+(?:(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)'?s?\s+)?(?:birthday|anniversary)\b"#
);
pattern!(
    REPAIR_DEEDS_5,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:lost|lose|losing|have lost|ve lost)\s+my\s+temper\b"#
);
pattern!(
    REPAIR_DEEDS_6,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:upset|hurt|offended|insulted|embarrassed|annoyed|disappointed|angered)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b"#
);
pattern!(
    REPAIR_DEEDS_7,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:hurt|upset|offended|annoyed)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)'?s?\s+feelings\b"#
);
pattern!(
    REPAIR_DEEDS_8,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:was|been|am|have been|ve been|were)\s+(?:a bit |a little |really |very |so |quite |pretty |just |being |really being |so being )*(?:rude|horrible|mean|cruel|unkind|nasty|sharp|short|unfair|dismissive|snappy|off|offhand|harsh|awful|dreadful|terrible|disrespectful|cheeky|abrupt)\s+(?:to|with|towards|toward)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b"#
);
pattern!(
    REPAIR_DEEDS_9,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:shouted|yelled|screamed|snapped|swore|sworn)\s+(?:at|with)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b"#
);
pattern!(
    REPAIR_DEEDS_10,
    r#"\bi\s+(?:(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4})?(?:said|say|saying|told|tell|telling)\s+(?:the wrong thing|the wrong words|something\s+(?:stupid|awful|daft|silly|dumb|mean|horrible|wrong|embarrassing|terrible|harsh|unkind))\b"#
);
pattern!(
    REPAIR_DEEDS_11,
    r#"\bi\s+(?:(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4})?(?:said|told)\b[^.!?]{0,40}\b(?:was|were|is) (?:awful|horrible|stupid|mean|rude|unkind|dreadful|terrible|nasty|rubbish|disgusting|pathetic)\b"#
);
pattern!(
    REPAIR_DEEDS_12,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:missed|miss)\s+(?:(?:(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody))'?s?\s+|(?:my|his|her|their|our)\s+)(?:(?:school|first|big|little|football|netball|swimming|music|dance|christmas|xmas|summer|end of term|leavers?)\s+)?(?:play|concert|recital|match|game|show|performance|assembly|sports day|parents evening|parents' evening|nativity|graduation|prize giving|prizegiving|first day|open day|appointment)\b"#
);
pattern!(
    REPAIR_DEEDS_13,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:broke|broken|break|breaks|have broken|ve broken)\s+(?:a\s+|my\s+|the\s+)?promise\b"#
);
pattern!(
    REPAIR_DEEDS_14,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:stood|stand|standing)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\s+up\b"#
);
pattern!(
    REPAIR_DEEDS_15,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:cancelled|canceled|cancel|cancelling|canceling)\s+(?:on\s+)?(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b"#
);
pattern!(
    REPAIR_DEEDS_16,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:ignored|ignore|ignoring|blanked|blank)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\b"#
);
pattern!(
    REPAIR_DEEDS_17,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:ignored|ignore|ignoring|blanked|blank)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)'?s?\s+(?:calls?|texts?|messages?|emails?|letters?|invites?)\b"#
);
pattern!(
    REPAIR_DEEDS_18,
    r#"\bi\s+(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\s+){0,4}(?:made|make|making|have made|ve made)\s+(?:(?:my|his|her|their|our|the)\s+(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)|him|her|them|someone|somebody)\s+(?:cry|cried|crying|sad|miserable|upset|tearful)\b"#
);
pub(super) const REPAIR_DEEDS: &[&std::sync::LazyLock<fancy_regex::Regex>] = &[
    &REPAIR_DEEDS_0,
    &REPAIR_DEEDS_1,
    &REPAIR_DEEDS_2,
    &REPAIR_DEEDS_3,
    &REPAIR_DEEDS_4,
    &REPAIR_DEEDS_5,
    &REPAIR_DEEDS_6,
    &REPAIR_DEEDS_7,
    &REPAIR_DEEDS_8,
    &REPAIR_DEEDS_9,
    &REPAIR_DEEDS_10,
    &REPAIR_DEEDS_11,
    &REPAIR_DEEDS_12,
    &REPAIR_DEEDS_13,
    &REPAIR_DEEDS_14,
    &REPAIR_DEEDS_15,
    &REPAIR_DEEDS_16,
    &REPAIR_DEEDS_17,
    &REPAIR_DEEDS_18,
];
pattern!(
    A_LIMIT,
    r#"(?i)\b(?:nothing|no|not|never|nowt)\s+(?:too\s+|very\s+|much\s+|any\s+)?(?P<move>tiring|strenuous|energetic|demanding|exhausting|exhaustive|walking|walk|walks|long walks?|hikes?|hiking|hills?|stairs?|standing|climbing|lifting)\b|\b(?:can'?t|cannot|can not|isn'?t able to|is not able to|not able to|only manage)\s+(?:walk|climb|manage)\s+(?:far|long|much)\b|\b(?:can'?t|cannot|can not|isn'?t able to|is not able to|not able to|only manage)\s+(do|manage|get\s+up|get\s+up\s+and\s+down|get\s+up\s+the|use|cope\s+with)\s+(?:the\s+|a\s+|any\s+)?(?:more\s+)?(?:stairs?|steps?|hills?|walks?|walking|climbing|standing)\b|\b(?:can'?t|cannot|can not|isn'?t able to|is not able to|not able to|only manage)\s+(?:stand|sit)\s+(?:for\s+)?(?:long|far)\b|\b(?:keep (?:it|the cost|costs) (?:cheap|low|small)|on a (?:tight )?budget)\b|\b(?:nothing|no|not)\s+(?:too\s+)?(?:expensive|pricey|fancy)\b"#
);
pattern!(
    PLAN,
    r#"(?i)\b(?:(?:i(?:'m| am)|we(?:'re| are)|im|were)\s+(?:going to|planning|thinking of|off to|about to|hoping to|trying to|aiming to|aim to|meaning to|learning to|working on|writing|training|moving|buying|looking for|starting|organising|organizing|preparing|building|making|booking|saving|decorating|redecorating|painting|renovating|tiling|plastering|sorting|packing|cleaning|planting|digging|sewing|knitting|baking|cooking)(?: up| out| around| through)?|i want to|i need to|we want to|we need to|i(?:'?ve| have) got|i(?:'?ve| have) just got)\b"#
);
pattern!(
    YES_NO_ASK,
    r#"(?i)^\W*(?:is|are|was|were|does|did|do|can|could|has|have|had|will|would|should|may|might)\s+(?:she|he|they|his|her|their|my|your)\b"#
);
pattern!(
    YES_NO_FRAME,
    r#"(?i)^\W*(?:is|are|was|were|does|did|do|can|could|has|have|had|will|would|should|may|might)\s+[^?.!;]*[?.!;]+\s*"#
);
pattern!(
    TELLING_AFTER_ASK,
    r#"(?i)\b(?:he|she|they|it|we|my\s+(?:granddaughter|stepdaughter|grandmother|grandfather|stepbrother|grandmother|grandfather|girlfriend|stepmother|stepfather|stepsister|grandchild|neighbour|boyfriend|stepchild|daughter|neighbor|grandmum|grandmom|granddad|grandson|granddad|partner|husband|brother|parents|grandma|grandad|grandpa|fiancee|stepmum|stepmom|stepdad|stepson|mother|father|sister|granny|auntie|nephew|cousin|friend|fiance|missus|twins|nanna|aunty|uncle|niece|hubby|wife|gran|nana|aunt|gran|son|mum|mom|dad|nan|gfs|bfs|gf|bf))\s+(?:is|are|was|were|doesnt|does not|dont|do not|cant|cannot|wont|will not|has|have|had|likes?|loves?|eats?|drinks?|never|always|usually|only|really|prefers?|hates?)\b"#
);
pattern!(
    REMIND_QUESTION,
    r#"(?i)\bremind me\b[^.!?]{0,24}\b(?:what|when|where|who|whom|whose|how|which|why|if)\b|\bremind me\b(?:[\s,]*(?:again|please))*[\s,]*$"#
);
pub(super) const SAFETY_MODULES: &[&str] = &["safety"];
pub(super) const FALLBACK_MODULES: &[&str] = &["greeting"];
pub(super) const THREAD_TURNS: usize = 6;
pattern!(
    FUNERAL,
    r#"\b(funeral|memorial|service|viewing|burial|cremation)\b"#
);
pattern!(
    FUNERALS,
    r#"\b(?:funerals|memorials|burials|cremations|crematoria|crematorium|graveside|gravesides)\b"#
);
pattern!(
    WAKE,
    r#"\b(?:his|her|their|the|a|mum's|dad's|nan's|grandad's|gran's)\s+wake(?!\s*(?:up|me|him|her|them|you|us|at|before|earlier|by|and|of)\b)\b"#
);
pattern!(
    GRIEF_CUE,
    r#"\b(feel|feels|feeling|feelings|cope|coping|sleep|sleeping|crying|cry|normal|miss|missing|heartbroken|numb|empty|guilt|guilty|angry|lonely)\b"#
);
pattern!(
    ASK_FORM,
    r#"\b(should i|do i|can i|how do i|shall i|would you)\b"#
);
pattern!(
    A_PART_OF_THE_DAY,
    r#"(?i)\b(?:what|which|any)\b(?:\W+\w+){0,5}\W+(?:music|songs?|hymns?|tunes?|readings?|reads?|recit(?:e|ed|ing)|poems?|psalms?|flowers?|wreaths?|bouquets?|order of service|kids?|children|child|baby|toddler|grandchildren|grandkids)\b|\b(?:should|shall|can|could|do)\s+(?:i|we|you)\b(?:\W+\w+){0,6}\W+(?:music|songs?|hymns?|tunes?|readings?|reads?|recit(?:e|ed|ing)|poems?|psalms?|flowers?|wreaths?|bouquets?|order of service|kids?|children|child|baby|toddler|grandchildren|grandkids)\b|\b(?:should|shall|can|could|do|does)\s+(?:(?:i|we|you|a|the|any|my|our)\s+)?(?:kids?|children|child|baby|toddler|grandchildren|grandkids)\b|\b(?:good|nice|right|best|suitable)\s+(?:songs?|hymns?|tunes?|music|readings?|poems?|psalms?)\b|\b(?:should|shall|can|could)\s+(?:my|our)\b(?:\W+\w+){0,4}\W+(?:music|songs?|hymns?|tunes?|readings?|reads?|recit(?:e|ed|ing)|poems?|psalms?|flowers?|wreaths?|bouquets?|order of service|kids?|children|child|baby|toddler|grandchildren|grandkids)\b|\bhow\s+long\s+(?:is|does|do|are|will|would)\b(?:\W+\w+){0,4}\W+(?:funeral|service|cremation|burial|wake|memorial)\b|\bhow\s+long\s+(?!until|till|to|before)\w*(?:\W+\w+){0,6}\W+\bfuneral\b|\bhow\s+many\s+(?:days|weeks)\b(?:\W+\w+){0,6}\W+\bfuneral\b|\bhow\s+soon\b(?:\W+\w+){0,4}\W+\bfuneral\b|\bis\s+it\s+(?:ok|okay|fine|alright)\b(?:\W+\w+){0,6}\W+(?:music|songs?|hymns?|tunes?|readings?|reads?|recit(?:e|ed|ing)|poems?|psalms?|flowers?|wreaths?|bouquets?|order of service|kids?|children|child|baby|toddler|grandchildren|grandkids)\b"#
);
pattern!(
    SAY_THE_WORDS,
    r#"(?i)\b(?:what to say|what do i say|what can i say|what should i say|what shall i say|say at|say to|words for|eulogy|eulogies|speech)\b"#
);
pattern!(
    ASK_WE,
    r#"\b(should we|can we|do we|how do we|shall we|where do we|what do we|when do we|who do we)\b"#
);
pattern!(
    ASK_TAIL,
    r#",\s*(?:is|are|was|were|do|does|did|should|can|how|what|where|when|who|will|would|shall)\b"#
);
pattern!(
    PET_ARRANGEMENT,
    r#"\b(bury|buried|burying|burial|body|bodies|dead|ashes|cremat\w*|cremate|urn|plot|grave|graveside|headstone|memorial|paw\s?print|lock\s+of\s+(?:fur|hair)|coffin|casket|collect\w*|pick\w*\s+(?:\w+\s+){0,2}up|collect\s+\w+\s+from\s+the\s+vet)\b"#
);
pattern!(
    PET,
    r#"\b(cat|dog|puppy|kitten|rabbit|hamster|guinea\s?pig|horse|pony|budgie|parrot|pet|pets|ferret|gerbil|rat|mouse|tortoise|snake|lizard|fish|goldfish)\b"#
);
pattern!(
    PAPER_ASK,
    r#"\b(probate|grant of probate|letters of administration|estate|paperwork|solicitor|wills|(?:the|his|her|their|my|our|a) will|inheritance tax|death certificate|register (?:the |his |her |their )?death)\b"#
);
pattern!(
    PRACTICAL,
    r#"\b(cost|costs|price|prices|pricing|cheap|cheapest|expensive|afford\w*|pay|pays|paid|paying|payment|payments|cover|covers|covered|how much (?:is|are|was|were|does|do|will|did)|probate|paperwork|document\w*|register|registered|registering|certificate|death certificate|wills|estate|insurance|solicitor|lawyer|undertaker|funeral director|coffin|crematorium|cook|cooking|cooks|food|sandwiches|buffet|catering|caterer|drinks|flowers|order|book|booking|berea\w+ leave|time off|back to work|allowance|benefit|benefits|grant|quotes?|forms?)\b"#
);
pattern!(
    ASK_WORDS,
    r#"\b(what do i (?:say|write|put|tell)|what to (?:say|write|put)|what should i (?:say|write|put|tell)|what can i say|how do i (?:say|comfort|help|write|tell)|how to (?:comfort|write|word)|any ideas for what to say|words for|what do you say)\b"#
);
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
    "really",
    "still",
];
pub(super) const LOSS_REPLIES: &[&str] = &[
    "A cremation with no service is the cheapest of the lot, and a burial is the dearest: the plot and the headstone are the difference. Prices move a lot by area, so get two itemised quotes in writing before you decide.",
    "A habit that was for them takes a while to notice as a habit. Do it, or stop, whichever comes first - neither is a betrayal of them.",
    "A house with one fewer person in it feels wrong before it feels like anything else. Be slow with it.",
    "An empty house is the hardest part of the day to fill. It does get less loud, and the quiet is not a measure of anything.",
    "Anger is part of grief for most people, and it does not have to make sense. Let it out somewhere safe, then tell someone who knew them what you are angry about.",
    "Ask a relative or the funeral director for the time and the meeting place — most services are at the church or chapel first, then the crematorium — and get there a little early.",
    "Being angry at them, or at the world, or at whoever was there is common after a death. Say it out loud to one person you trust rather than keeping it in.",
    "Check the notice the family put out, or ring the funeral director: they will tell you the time and where to meet. Ask whether to go straight to the crematorium or meet at the house.",
    "Claim the DWP funeral expenses payment within six months of the funeral: it covers the cremation or burial fees and some of the rest, and it goes to whoever arranged it. If you are on a low income, ask the council too - they pay for a funeral when there is no money at all.",
    "Cook their dish or get their favourite in, and raise a glass to them: the point is to mark the day on purpose instead of letting it hit you sideways.",
    "Cremation usually comes out cheaper than burial: the plot, the grave being dug and often a headstone are what the difference is made of. Ask two funeral directors for a written, itemised quote and compare the coffin, the cars and the cremation fee as separate lines.",
    "Cremation, yes - a burial adds the plot, the digging and usually a headstone, and that is where the difference is. The rest of the bill is much the same either way, so ask for the itemised quote and look at the coffin first.",
    "Crying over them, or thinking of nothing else, is the plainest way of saying they mattered. It comes in waves - let it, and be kind to yourself after.",
    "Do it smaller rather than skip it: the same lunch or the same cake, with one or two people, and say out loud why you are having it. A day you have planned for is easier than one that just arrives.",
    "Feeling relief does not make you cold. If they were ill for a long time it is a normal thing to feel, and it can sit alongside missing them.",
    "Go if you can, even just for the service: turning up matters far more than anything you say. Take a card, say the plain thing, and leave when you need to.",
    "Guilt turns up with grief for most people, whether or not there is anything to be guilty about. Say the thing you keep going over out loud to someone who knew them; it weighs less that way.",
    "Half of it is the small jobs your hands still do. There is nothing wrong with any of them.",
    "Hello again. I'm still here - how are you doing today?",
    "Hello. I'm here whenever you want to talk about it, or about anything else.",
    "Hi again. How has the day been?",
    "Hi. No need to say anything in particular - how are you holding up?",
    "I'm glad you said that out loud. Say as much or as little as you like — I'm here.",
    "I'm here. Take your time with it — tell me what it has been like, if you want to.",
    "I'm really sorry. Be gentle with yourself today, and tell me if there's anything you want to talk through.",
    "I'm so sorry to hear that. It's a hard thing to carry — I'm here if you want to talk about it, or if anything practical would help.",
    "I'm sorry, that's a real loss. Take today slowly, and shout if there's anything I can help you sort out.",
    "If that is all you can do today, it is enough for today. It eases, and it keeps its own timetable.",
    "If you can make it, go — families remember who came. You don't have to speak; sitting with them is the thing.",
    "Keep it plain and warm: \"I'm so sorry\". Use the person's name, don't rush to fill the silence, and offer one concrete thing — a lift, a meal, covering a shift.",
    "Keep it short and warm: \"I'm so sorry for your loss. Thinking of you and your family.\" If you knew them, add one small memory — that is the part people keep.",
    "Keep the ritual and shrink it: the food they loved, their place at the table, and one story about them told to whoever is there. Doing something is what marks the day.",
    "Keep the shape of the day: up at the usual time, food at the usual times, bed at night. The routine carries you on the days you cannot carry yourself.",
    "Laying it out for them is the routine still running. Put it away when you want to, not because anyone says it is time.",
    "Make the day about them: the meal they would have chosen, the people they liked, and the one thing you would have told them. If it all feels too heavy, do a small version and stop there.",
    "Missing like that makes sense. Those days can be long ones — I'm around if talking would help.",
    "Most people find the same, and it does get easier. Be gentle with yourself — and see your GP if it has not lifted.",
    "Numb is one of the ways grief shows up first. Nothing has to be decided today — just keep eating and sleeping, and let someone know how you are.",
    "Of course you do. It goes in waves, and the waves get further apart.",
    "One piece at a time is the only way anyone does it. Pick the next thing - a shower, a sandwich, one phone call - and let the rest wait until that is done.",
    "Pick one thing to do at the time you would normally have spent together - the lunch, a walk, a visit - and tell someone you are doing it, so you are not carrying it on your own.",
    "Relief after a death is more common than people admit, and it does not cancel the grief. Say it to someone who will not judge it — that is usually enough to make it sit easier.",
    "Say you're sorry, name them if you can, and offer something practical. \"I'll bring dinner on Thursday\" helps far more than \"let me know if you need anything\".",
    "Seeing them out of the corner of your eye is the habit outliving them. Most people who have lost someone do it for months, and it says nothing about how you are.",
    "Seeing them, hearing them, waiting for them - it is a habit that had years to set. It fades slowly, and it is not a sign you are losing them.",
    "Some days the missing is louder than others. No need to do anything with it here; I'm just listening.",
    "Some part of the day is still theirs. Doing the job anyway is how a lot of people get through the first year, and there is nothing wrong with it.",
    "Tell one person how you are actually doing today and let them do one job for you. Most people want to help and do not know how to ask.",
    "That ache is real, and it doesn't keep office hours. Say more whenever you want to.",
    "That guilt is a common part of it, and it is rarely fair. Tell one person who knew them what you keep turning over — you do not have to carry it on your own.",
    "That is grief doing its work, and it is not something to get on top of. I am here for as much of it as you want to say.",
    "That is normal after a loss, and it eases slowly for most people. If it is still this heavy, your GP is the right person to talk to.",
    "That is part of grief for most people. If it stays like this, tell your GP; there is real help for it.",
    "That numb feeling is common early on; it is not coldness. Be gentle with yourself today, and tell me or one person close to you if it stays like this.",
    "That reflex is the shape of them still in your day. There is nothing to do about it, and it goes with how much of the day they were in.",
    "That sounds heavy to be carrying. I'm listening, however much or little you want to say.",
    "That's such sad news, I'm sorry. Some days will be heavier than others — I'm here if you want to talk about it, or about anything else.",
    "The first days are the hardest and they are not a forecast of the rest. If you cannot eat or sleep, or you cannot see a way forward, tell your GP - they can help with that.",
    "The hands keep the routine for a while - the second bowl, the place at the table. That is muscle memory, not madness, and you can let it go when it stops being a comfort.",
    "The jobs you did for them do not stop on the day. The hands keep going for a while, and that is not a sign you are stuck.",
    "The ones we miss are the ones who mattered. Say as much or as little as you like.",
    "The quiet is the whole of it, and it is worst in the evenings. Nobody gets to tell you there is a timetable for that.",
    "The time and place are on the family's notice or the funeral director's page — ask a family member or the funeral director if you can't find it, nobody minds being asked.",
    "There is help with it: the DWP funeral expenses payment covers part of the bill if you are on certain benefits, and you claim it after the funeral, within six months. Keep every receipt, and ring the council as well - they have a fund of their own for people who cannot pay.",
    "There's no rush to talk about it. I'm here when you want to say more.",
    "Two or three lines is plenty: name the person, say you are sorry, and add a practical offer — a meal, a lift, a hand with the children.",
    "You can apply for the funeral expenses payment from the DWP after the funeral - within six months, and only on certain benefits - so tell the funeral director to invoice you and keep the receipts. The council's bereavement service has a separate fund, and Citizens Advice will help you fill the forms in.",
    "You don't need the perfect words. \"I'm so sorry\" and then listening is most of it; add one specific offer of help this week.",
];
pattern!(
    NEWS_WITH_A_NAME_0,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird|nephew|niece|stepson|stepdaughter|stepmum|stepdad|stepmother|stepfather|grandson|granddaughter|mother in law|father in law|son in law|daughter in law|grandchild|grandchildren)\b[^.!?]{0,20}\b(?:has|have|had|got|is|was|were|been|being)\b[^.!?]{0,24}\b(?:dementia|alzheim\w*|altzheim\w*|parkinson\w*|motor neurone|huntington\w*|multiple sclerosis|muscular dystrophy|leuka?emia|lymphoma|myeloma|mesothelioma|pancreatitis|copd|emphysema|cirrhosis|kidney failure|heart failure|brain (?:damage|injury)|cerebral palsy|terminal)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_1,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird|nephew|niece|stepson|stepdaughter|stepmum|stepdad|stepmother|stepfather|grandson|granddaughter|mother in law|father in law|son in law|daughter in law|grandchild|grandchildren)\b[^.!?]{0,24}\b(?:is|are|was|were|has been|have been|had been|got|getting|become|becoming|been)\b[^.!?]{0,16}\b(?:ill|poorly|unwell|not well|off (?:it|colour|their food)|in a bad way|taken ill|gone downhill|deteriorat\w+)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_2,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird|nephew|niece|stepson|stepdaughter|stepmum|stepdad|stepmother|stepfather|grandson|granddaughter|mother in law|father in law|son in law|daughter in law|grandchild|grandchildren)\b[^.!?]{0,24}\bd(?:iagnos\w+|x\b)"#
);
pattern!(
    NEWS_WITH_A_NAME_3,
    r#"\b(?:been|being|just|newly|recently)\s+(?:diagnos(?:ed|is)\w*)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_4,
    r#"\b(?:not allowed|no longer allowed|allowed|banned|barred|stopped|prevented)\b[^.!?]{0,24}\b(?:see|seeing|visit|visiting|contact(?: with)?)\b[^.!?]{0,20}\b(?:kids?|children|child|son|daughter|grandchild\w*|baby)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_5,
    r#"\b(?:pip|dla|esa|universal credit|housing benefit|child benefit|tax credits?|benefits?|allowance|pension|pay|wages|hours)\b[^.!?]{0,24}\b(?:stopped|cancell?ed|cut|suspended|frozen|refused|sanctioned|withdrawn|reduced|turned down|taken away|not renewed)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_6,
    r#"\b(?:stopped|cancell?ed|cut|suspended|frozen|sanctioned|withdrawn|refused|reduced)\b[^.!?]{0,20}\b(?:my|our)\s+(?:pip|dla|esa|universal credit|benefits?|pay|wages|pension|allowance|hours)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_7,
    r#"\b(?:got|gotten|been|being|was|were|just|now)\s+sanctioned\b(?!\s+(?:leave|holiday|absence|time off|day off|overtime|pay rise)\b)"#
);
pattern!(
    NEWS_WITH_A_NAME_8,
    r#"\b(?:have|has|had|having|got|gotten|getting|being (?:forced|made))\s*(?:to\s+)?(?:sell|give up|hand back|move out of)\b[^.!?]{0,24}\b(?:the|our|my|his|her)\s+(?:house|home|flat|bungalow|place)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_9,
    r#"(?!.*\b(?:adopt\w*|adoption)\b)\b(?:got|gotten|had|received|been sent|sent me|sending me|came through|arrived)\b[^.!?]{0,28}\b(?:court (?:date|letter|order|summons|hearing)|summons|notice to quit|eviction notice|possession (?:claim|order)|letter before (?:claim|action))\b"#
);
pattern!(NEWS_WITH_A_NAME_10, r#"\bbailiffs?\b"#);
pattern!(
    NEWS_WITH_A_NAME_11,
    r#"(?!.*\b(?:adopt\w*|adoption)\b)\b(?:going|have to go|has to go|got to go|due|off)\s+(?:in |to |before )?court\b"#
);
pattern!(
    NEWS_WITH_A_NAME_12,
    r#"\b(?:taking|took|taken|summoned|prosecuting|suing)\s+(?:me|us|him|her|them)\s+to court\b"#
);
pattern!(
    NEWS_WITH_A_NAME_13,
    r#"\b(?:i'?m|im|i am|we'?re|we are|i was|we were)\s+(?:being\s+)?(?:taken|summoned|prosecuted|sued)\s+(?:to|into|before)\s+court\b"#
);
pattern!(
    NEWS_WITH_A_NAME_14,
    r#"\b(?:hearing|trial|case)\s+(?:is |was )?(?:next|this)\s+(?:week|month|monday|tuesday|wednesday|thursday|friday|morning|afternoon)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_15,
    r#"\b(?:going|went|moved|transferred|have to go|has to go|got to go|off|sent|sentenced|remanded|due)\b[^.!?]{0,16}\b(?:to |in |into )?(?:prison|jail|young offenders)\b(?!\s+(?:to\s+(?:visit|see|meet)|visiting|museum|tour|open\s+day))"#
);
pattern!(
    NEWS_WITH_A_NAME_16,
    r#"\b(?:is|are|was|were|been|being|stays?|sits?|sitting|kept|held|locked)\s+(?:up\s+|inside\s+|away\s+)?(?:in|at|inside)\s+(?:the\s+)?(?:prison|jail|jailhouse|young offenders|pentonville|wormwood scrubs|brixton|wandsworth)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_17,
    r#"\bsent down\b[^.!?]{0,16}\b(?:for|to)\s+(?:\d+|two|three|four|five|six|seven|eight|nine|ten|twelve|eighteen|twenty|a\s+(?:year|month|stretch|while)|life)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_18,
    r#"\b(?:was|were|is|are|been|being|got|gets|getting|keeps being)\s+sent down\b"#
);
pattern!(
    NEWS_WITH_A_NAME_19,
    r#"\b(?:taken|took|put|puts|removed|removes)\b[^.!?]{0,20}\b(?:into care|in care)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_20,
    r#"\b(?:taken|took|removed)\b[^.!?]{0,24}\b(?:my|our|the)\s+(?:kids?|children|child|son|daughter|baby)\b[^.!?]{0,16}\baway\b"#
);
pattern!(
    NEWS_WITH_A_NAME_21,
    r#"\bsocial services\b[^.!?]{0,24}\b(?:took|taken|removed|involved|came)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_22,
    r#"\b(?:losing|lose|lost|going to lose|about to lose|might lose|will lose|going to be losing)\b[^.!?]{0,12}\b(?:the|our|my)\s+(?:house|home|flat|bungalow)\b"#
);
pattern!(
    NEWS_WITH_A_NAME_23,
    r#"\b(?:got|get|getting|been|being|was|were)\s+sacked\b"#
);
pattern!(
    NEWS_WITH_A_NAME_24,
    r#"\b(?:bad|sad|awful|terrible|dreadful|grim|rotten|horrible|horrid)\s+(?:news|tidings)\b"#
);
pub(super) const NEWS_WITH_A_NAME: &[&std::sync::LazyLock<fancy_regex::Regex>] = &[
    &NEWS_WITH_A_NAME_0,
    &NEWS_WITH_A_NAME_1,
    &NEWS_WITH_A_NAME_2,
    &NEWS_WITH_A_NAME_3,
    &NEWS_WITH_A_NAME_4,
    &NEWS_WITH_A_NAME_5,
    &NEWS_WITH_A_NAME_6,
    &NEWS_WITH_A_NAME_7,
    &NEWS_WITH_A_NAME_8,
    &NEWS_WITH_A_NAME_9,
    &NEWS_WITH_A_NAME_10,
    &NEWS_WITH_A_NAME_11,
    &NEWS_WITH_A_NAME_12,
    &NEWS_WITH_A_NAME_13,
    &NEWS_WITH_A_NAME_14,
    &NEWS_WITH_A_NAME_15,
    &NEWS_WITH_A_NAME_16,
    &NEWS_WITH_A_NAME_17,
    &NEWS_WITH_A_NAME_18,
    &NEWS_WITH_A_NAME_19,
    &NEWS_WITH_A_NAME_20,
    &NEWS_WITH_A_NAME_21,
    &NEWS_WITH_A_NAME_22,
    &NEWS_WITH_A_NAME_23,
    &NEWS_WITH_A_NAME_24,
];
