// Ownership and animal cues from the pinned loss reader.
loss_pattern!(
    CHILDS_OWN_LOSS,
    r####"(?i)\b(?:my|our)\s+(?:son|sons|daughter|daughters|child|children|kid|kids|boy|girl|lad|lass|teenager|baby|toddler|stepsons?|stepdaughters?|stepchildren)\b[^.]{0,40}\b(?:his|her|their)\s+(?:dad|daddy|mum|mummy|father|mother|parent|parents|brother|sister|grandad|grandads|granddad|grandma|grandmas|gran|grans|nan|nans|grandpa|grandparents)\b"####
);
loss_pattern!(
    A_STRANGERS_ANIMAL,
    r####"\b(?:the|a|an|this|that)\s+(?:[a-z]+\s+){0,2}?(?:budgie|cat|cats|chicken|collie|dog|dogs|ferret|fish|fishes|gerbil|goldfish|greyhound|guinea|hamster|hen|horse|kitten|kittens|labrador|lizard|lurcher|mouse|parrot|pet|pets|pony|puppies|puppy|rabbit|rat|snake|tortoise)s?\s+(?:[a-z]+\s+){0,4}?\b(?:next door|over (?:the )?road|upstairs|downstairs|in (?:our|the|my) (?:street|road|close|lane|block|crescent|estate))\b"####
);
loss_pattern!(
    SOMEONE_ELSES,
    r####"\b(?:my|our|a|an|the|one of my)\s+(?:friend|friends|mate|mates|colleague|colleagues|neighbour|neighbors|neighbours|neighbor|boss|landlord|girlfriend|boyfriend|ex|coworker|workmate)(?:'s|s)?\s+(?:(?:of mine|of ours)\s+)?(?:whose\s+|with\s+|who\s+has\s+)?(?:\w+\s+){0,2}?(?:mum|mom|mother|dad|father|son|daughter|wife|husband|partner|nan|gran|grandma|grandad|grandpa|brother|sister|aunt|uncle|cousin|child|baby|family)\b"####
);
loss_pattern!(
    HIS_HERS,
    r####"\b(?:his|her|their)\s+(?:mum|mom|mother|dad|father|son|daughter|wife|husband|partner|nan|gran|grandma|grandad|grandpa|brother|sister|aunt|uncle|cousin|child|baby|dog|cat|funeral|wake|memorial|service|viewing|burial)\b"####
);
loss_pattern!(
    SOMEONE_ELSES_OWN,
    r####"\b(?:my|our)\s+[a-z]+(?:'s|s'|s)\s+(?:[a-z]+\s+){0,2}?(?P<who>aunt|auntie|baby|babysitter|babysitters|barber|boy|boyfriend|boyfriends|brother|budgie|caregiver|caregivers|carer|carers|cat|cats|child|childminder|childminders|children|classmate|classmates|cleaner|cleaners|colleague|colleagues|counsellor|counsellors|counselor|counselors|cousin|dad|daughter|doctor|doctors|dog|dogs|ex|family|father|ferret|fiance|fiancee|fiances|fiancé|fiancée|fil|fish|fishes|flatmate|flatmates|friend|friends|gerbil|girl|girlfriend|girlfriends|godchild|godchildren|goddaughter|goddaughters|godfather|godmother|godson|godsons|goldfish|gp|gps|gran|grandad|grandchild|grandchildren|granddad|granddaughter|granddaughters|grandfather|grandkids|grandma|grandmother|grandpa|grandparents|grandson|grandsons|granny|guardian|guinea|hairdresser|halfbrother|halfsister|hamster|horse|housemate|housemates|husband|imam|in-laws|kid|kids|kitten|lizard|lodger|lodgers|mam|manager|managers|mate|mates|mentor|midwife|midwives|mil|mom|mother|mum|nan|nana|nannies|nanny|neighbor|neighbors|neighbour|neighbours|nephew|niece|nurse|nurses|parent|parents|parrot|partner|pastor|pet|pets|physio|physiotherapist|pony|priest|psychiatrist|puppy|rabbi|rabbit|relative|relatives|schoolfriend|sibling|siblings|sister|snake|son|spouse|spouses|stepbrother|stepchild|stepchildren|stepdad|stepdaughter|stepdaughters|stepmum|stepsister|stepson|stepsons|surgeon|surgeons|teacher|teachers|therapist|therapists|tortoise|tutor|tutors|twin|twins|uncle|vicar|warden|wife|worker|workers)\b"####
);
loss_pattern!(
    DEATH_WORD,
    r####"\b(?:died|dies|dying|dead|death|passed|put\s+down|put\s+to\s+sleep|euthanised|euthanized|killed|pts|gone|loss|lost)\b"####
);
