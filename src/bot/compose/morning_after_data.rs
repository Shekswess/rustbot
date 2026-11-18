//! Markers and replies for a night of drinking from the pinned composer.

context_pattern!(
    A_STRONG_STATE,
    r####"(?i)\b(?:drunk|drunken|pissed(?!\s+off)|plastered|bladdered|trolleyed|mullered|mangled|sozzled|paralytic|off\s+(?:my|his|her|their|our)\s+(?:face|head|tits)|on\s+the\s+lash|bender|(?:a\s+)?few\s+too\s+many|one\s+too\s+many|too\s+much\s+(?:to\s+drink|wine|beer|lager|booze)|too\s+much\s+last\s+night|heavy\s+night|big\s+night)\b"####
);
pattern!(
    A_WEAK_STATE,
    r####"(?i)\b(?:wasted|smashed|steaming|tipsy|boozy|lash|binge|bendered|drank|hammered|leathered|wrecked|half\s+cut|been\s+drinking|was\s+drinking|were\s+drinking)\b"####
);
pattern!(
    A_TIME_THAT_HAS_GONE,
    r####"(?i)\b(?:last\s+night|last\s+evening|yesterday|the\s+night\s+before|all\s+night|overnight|this\s+morning|woke\s+up|till\s+\d|til\s+\d|until\s+\d|(?:fri|satur|sun)day\s+night|at\s+the\s+weekend|night\s+out|at\s+the\s+(?:party|pub|club|do|wedding|works\s+do|leaving\s+do)|at\s+a\s+party|on\s+a\s+night\s+out)\b"####
);
pattern!(
    A_HANGOVER,
    r####"(?i)\b(?:hung\s?over|hangover|hanging|woke\s+up\s+rough|rough\s+as\s+a\s+badger)\b"####
);
pattern!(
    A_MORNING,
    r####"(?i)\b(?:this\s+morning|today|now|after\s+last\s+night)\b"####
);
pattern!(
    A_CONSEQUENCE,
    r####"(?i)\b(?:said|saying|told|texted|messaged|rang|rung|called|drove|drive|drove\s+home|hit|shouted|argued|arguing|kissed|cheated|spent|ruined|broke|broke\s+up|fell|fall|lost\s+my|got\s+in\s+a\s+fight|fight)\b"####
);
pattern!(
    A_STILL_TO_COME,
    r####"(?i)\b(?:tonight|tomorrow|next\s+(?:week|weekend|month|year)|going\s+to|gonna|this\s+weekend|on\s+friday|on\s+saturday)\b"####
);
pattern!(
    SOMEONE_ELSE,
    r####"(?i)\b(?:hes|shes|theyre)\b|\b(?:my|our|his|her|their|he|she|they|we|us)\s+(?:\w+\s+){0,2}?(?:was|were|got|gets|get|is|are|had|has|keeps?|been)\b"####
);
pattern!(
    AN_EMERGENCY,
    r####"(?i)\b(?:ambulance|999|911|111|hospital|a\s?&?\s?e\b|passed\s+out|passing\s+out|blacked\s+out|blackout|can'?t\s+wake|not\s+breathing|stopped\s+breathing|alcohol\s+poisoning|overdose|overdosed|choking|seizure|collapsed|unconscious)\b"####
);
pattern!(
    A_QUESTION_ABOUT_IT,
    r####"(?i)\b(?:should|shall|could|how\s+do|how\s+can|how\s+to|what\s+should|any\s+tips|advice|stop\s+drinking|give\s+up|cut\s+down|cutting\s+down|quitting|quitting|help)\b"####
);

pub(super) const A_MORNING_AFTER: &[&str] = &[
    "Ha - how is your head today?",
    "Water, something plain to eat and an early night is most of the cure. How is today going?",
    "That is a long night. Are you feeling it today, or did you get away with it?",
    "How are you this morning? A pint of water, food and time, in that order.",
    "How bad is it - banging head, or just tired and skint?",
    "That sounds like one of those nights. Are you able to take it easy today?",
    "Right. How are you feeling now?",
];
