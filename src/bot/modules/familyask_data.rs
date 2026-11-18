//! Lexical classes and reply pools from PyBot bot/modules/familyask.py.

pub(super) struct Family {
    pub(super) relations: &'static [&'static str],
    pub(super) replies: &'static [&'static str],
}

pub(super) const LEAD_IN: &str = r#"(?:(?:and|so|but|oh|well|ok|okay|right|hey|hi|yeah|yes|no|nah|actually|honestly|just|quick|sorry|lol|haha|ha|eh|ah|hmm|huh|then|now)\b[,!.]?\s+)*"#;

const GRANDPARENT: Family = Family {
    relations: &[
        "gran",
        "grans",
        "granny",
        "grannies",
        "grandma",
        "grandmas",
        "grandad",
        "grandads",
        "grandpa",
        "grandpas",
        "nana",
        "nanas",
        "nan",
        "nans",
        "grandparent",
        "grandparents",
    ],
    replies: &[
        "No grandparents of mine - I'm a program, so nothing like that on my side.",
        "None, no: I'm a program, so no nan and grandad to go and see.",
        "I haven't got grandparents - I'm a program, so there's no family of mine at all.",
    ],
};

const PARENT: Family = Family {
    relations: &[
        "mum", "mums", "mam", "mams", "mom", "moms", "mother", "mothers", "dad", "dads", "father",
        "fathers", "parent", "parents", "mummy", "daddy",
    ],
    replies: &[
        "No mum and dad of my own - I'm a program, so there's no one who brought me up to be anywhere.",
        "None on my side: I'm a program, so no parents and no childhood to have them in.",
        "I haven't got parents - I'm a program, so there's no mum and dad at the end of a phone.",
    ],
};

const SIBLING: Family = Family {
    relations: &[
        "brother",
        "brothers",
        "sister",
        "sisters",
        "sibling",
        "siblings",
        "half brother",
        "half sister",
        "stepbrother",
        "stepsister",
        "step brother",
        "step sister",
    ],
    replies: &[
        "No brothers and no sisters - I'm a program, so no family of my own.",
        "None, no: I'm a program, so there's nobody to fall out with at Christmas.",
        "I haven't got siblings - I'm a program, so nobody grew up alongside me.",
    ],
};

const CHILD: Family = Family {
    relations: &[
        "kid",
        "kids",
        "child",
        "children",
        "son",
        "sons",
        "daughter",
        "daughters",
        "baby",
        "babies",
        "grandchild",
        "grandchildren",
        "grandson",
        "granddaughter",
    ],
    replies: &[
        "No children of my own - I'm a program, so nobody calls me anything but the bot.",
        "None, no: I'm a program, so no kids and no school run.",
        "I haven't got any - I'm a program, so there's no family of mine to bring up.",
    ],
};

const PARTNER: Family = Family {
    relations: &[
        "wife",
        "wives",
        "husband",
        "husbands",
        "partner",
        "partners",
        "girlfriend",
        "boyfriend",
        "fiance",
        "fiancee",
        "other half",
    ],
    replies: &[
        "Nobody like that - I'm a program, so no partner and no one to ring on the way home.",
        "None, no: I'm a program, so there's no other half on my side.",
        "I haven't got one - I'm a program, so nobody's waiting up for me.",
    ],
};

const EXTENDED: Family = Family {
    relations: &[
        "aunt",
        "aunts",
        "auntie",
        "aunties",
        "uncle",
        "uncles",
        "cousin",
        "cousins",
        "nephew",
        "nephews",
        "niece",
        "nieces",
        "in law",
        "in laws",
        "mother in law",
        "father in law",
        "brother in law",
        "sister in law",
    ],
    replies: &[
        "None of those on my side - I'm a program, so no aunts, uncles or cousins.",
        "Nobody like that: I'm a program, so there's no family of mine to be an aunt or uncle to.",
        "I haven't got any - I'm a program, so no relatives at all.",
    ],
};

pub(super) const GENERAL_FAMILY: Family = Family {
    relations: &[
        "family",
        "families",
        "relatives",
        "relations",
        "folks",
        "kin",
    ],
    replies: &[
        "No family of my own - I'm a program, so there is nobody on my side of it.",
        "None, no: I'm a program, so no family, no birthday list, nobody to ask after.",
        "I haven't got any - I'm a program, so there's no family of mine at all.",
    ],
};

pub(super) const FAMILIES: &[Family] = &[
    GRANDPARENT,
    PARENT,
    SIBLING,
    CHILD,
    PARTNER,
    EXTENDED,
    GENERAL_FAMILY,
];

pub(super) const HOW_IT_WENT: &[&str] = &[
    "Tell me about yours - how did it go?",
    "How did it go with yours?",
    "What about your lot - how did it go?",
];
pub(super) const THE_PEOPLE: &[&str] = &[
    "Tell me about yours though - do you see much of them?",
    "What about your lot?",
    "How are yours doing?",
];
