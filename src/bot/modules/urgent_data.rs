//! Urgent routing vocabulary from the pinned PyBot source at 7d63c7f.

pub(super) const FAMILY: &[&str] = &[
    "mum",
    "mam",
    "mom",
    "mother",
    "mummy",
    "mommy",
    "dad",
    "father",
    "daddy",
    "son",
    "daughter",
    "baby",
    "toddler",
    "child",
    "kid",
    "boy",
    "girl",
    "nan",
    "nana",
    "granny",
    "gran",
    "grandma",
    "grandad",
    "grandpa",
    "grandmother",
    "grandfather",
    "husband",
    "wife",
    "partner",
    "boyfriend",
    "girlfriend",
    "brother",
    "sister",
    "friend",
    "neighbour",
    "neighbor",
    "uncle",
    "aunt",
    "auntie",
    "cousin",
    "stepson",
    "stepdaughter",
    "stepdad",
    "stepmum",
    "fiance",
    "fiancee",
    "flatmate",
    "housemate",
    "carer",
    "patient",
    "niece",
    "nephew",
    "grandson",
    "granddaughter",
    "stepson",
    "stepdaughter",
    "godson",
    "goddaughter",
    "toddler",
    "newborn",
    "childminder",
    "flatmate",
    "teenager",
    "teen",
    "adolescent",
];

pub(super) const NOW: &[&str] = &[
    "unresponsive",
    "not responding",
    "no response",
    "wont wake",
    "won't wake",
    "cant wake",
    "can't wake",
    "not waking",
    "not breathing",
    "stopped breathing",
    "isnt breathing",
    "isn't breathing",
    "struggling to breathe",
    "cant breathe",
    "can't breathe",
    "gasping",
    "struggling for breath",
    "fighting for breath",
    "gasping for breath",
    "struggling for air",
    "unconscious",
    "unconcious",
    "collapsed",
    "limp",
    "floppy",
    "fitting",
    "seizure",
    "convulsing",
    "choking",
    "turning blue",
    "gone blue",
    "blue lips",
    "no pulse",
    "cardiac",
    "overdose",
    "took too many",
    "swallowed his tongue",
    "had a fit",
    "has a fit",
    "having a fit",
    "having fits",
    "keeps fitting",
    "kept fitting",
    "keeps having fits",
    "kept having fits",
    "isnt responding",
    "isn't responding",
    "isnt responsive",
    "isn't responsive",
    "not responsive",
];

pub(super) const TODAY: &[&str] = &[
    "confused",
    "drowsy",
    "very sleepy",
    "sleepy and",
    "temperature of 40",
    "40 degrees",
    "39 degrees",
    "39.5 degrees",
    "temperature of 39",
    "temperature of 39.5",
    "very hot to touch",
    "burning up",
    "hit his head",
    "hit her head",
    "hit their head",
    "banged his head",
    "banged her head",
    "fell and hit",
    "fell down the stairs",
    "wont stop bleeding",
    "wont stop vomit",
    "dehydrated",
    "not had a wee",
    "hasnt had a wee",
    "hasnt done a wee",
    "hasnt been for a wee",
    "no wet nappy",
    "dry nappy",
    "not passing urine",
];

pub(super) const CHEMICAL: &[&str] = &[
    "washing tablet",
    "washing pod",
    "laundry tablet",
    "laundry pod",
    "dishwasher tablet",
    "detergent",
    "bleach",
    "antifreeze",
    "drain cleaner",
    "oven cleaner",
    "disinfectant",
    "weedkiller",
    "weed killer",
    "screenwash",
    "screen wash",
    "paint thinner",
    "methanol",
    "hand sanitiser",
    "hand sanitizer",
    "white spirit",
    "caustic",
    "paint",
    "air freshener",
    "perfume",
    "aftershave",
    "mouthwash",
    "nail polish",
    "nail varnish",
    "polish remover",
    "vape",
    "e-liquid",
    "eliquid",
    "e-cigarette",
    "lighter fluid",
    "petrol",
    "paraffin",
    "turps",
    "turpentine",
    "caustic soda",
];

pub(super) const SMALL_OBJECT: &[&str] = &[
    "battery",
    "batteries",
    "magnet",
    "magnets",
    "coin",
    "marble",
    "bead",
    "button",
    "sim card",
    "screw",
    "drawing pin",
    "earring",
    "lego",
];

pub(super) const AMBIGUOUS: &[&str] = &[
    "unresponsive",
    "not responding",
    "no response",
    "limp",
    "floppy",
    "fitting",
    "confused",
    "drowsy",
    "burning up",
    "very hot to touch",
];

pub(super) const PRONOUN_SAFE: &[&str] = &[
    "had a fit",
    "has a fit",
    "having a fit",
    "having fits",
    "keeps fitting",
    "kept fitting",
    "keeps having fits",
    "kept having fits",
    "not breathing",
    "stopped breathing",
    "unconscious",
    "went limp",
    "gone limp",
    "limp in my arms",
    "limp in her arms",
    "limp in his arms",
    "no pulse",
    "seizure",
    "convulsing",
    "choking",
    "overdose",
    "took too many",
    "collapsed",
    "cardiac",
    "isnt breathing",
    "isn't breathing",
];

pub(super) const FIT_STILL_RINGS: &[&str] = &[
    "cant breathe",
    "can't breathe",
    "not breathing",
    "stopped breathing",
    "turning blue",
    "blue lips",
    "unconscious",
    "collapsed",
    "wont stop",
    "won't stop",
    "isnt stopping",
    "isn't stopping",
    "keeps going",
    "ambulance",
    "999",
    "on the floor",
    "shaking all over",
    "convulsing",
    "stiff",
];

pub(super) const SOFT_STATES: &[&str] = &[
    "floppy",
    "limp",
    "confused",
    "drowsy",
    "burning up",
    "very hot to touch",
    "very sleepy",
    "sleepy and",
];

pub(super) const TO_PAIN: &[&str] = &[
    "pain", "the pain", "any pain", "stimulus", "stimuli", "voice", "voices", "sound", "noise",
    "loud", "touch", "light", "nothing", "anything",
];

pub(super) const DEATH: &[&str] = &[
    "died",
    "dies",
    "dying",
    "passed away",
    "passed on",
    "funeral",
    "buried",
    "cremation",
    "bereave",
    "widow",
    "death",
    "killed",
];

pub(super) const INGEST: &str = "
    swallowed swallow swallowing ate eaten eating drank drunk drinking chewed chewing
    licked sucking sucked gulped gulping sipped downed
";

pub(super) const MEDICINE: &str = "
    tablet tablets pill pills capsule capsules paracetamol ibuprofen aspirin medicine medicines
    painkiller painkillers supplement supplements vitamins antidepressant antidepressants
    methadone insulin codeine morphine steroids
";

pub(super) const PLACE: &str = "
    floor ground bed bath sofa settee stairs street garden ward car cot pram pushchair chair
    table hospital home room arms lap arms shoulders knees chest hands
";

pub(super) const OBJECT_WORD: &str = "
    it them the a an my our his her their its this that these those new some carpet door floor
    tiles kitchen bathroom wardrobe blind window shelves shelf units sink sofa curtains decor
";

pub(super) const SYMPTOM: &str = "
    fever temperature temp cold flu cough rash pain headache seizure shivers sweat sweats
";

pub(super) const ORDINARY_STATE: &str = "
    tired sleepy asleep hungry teething crying cross grumpy upset angry excited playing bored
    told naughty tantrum tantrums whingey whinging
";

pub(super) const ONLINE: &str = "
    chat whatsapp messenger facebook instagram twitter tiktok app forum thread threads messages
    texts email emails phone phonecalls slack discord reddit linkedin zoom teams skype group
    groups feed forum list mailing
";

pub(super) const SMALL_PERSON: &str = "
    baby babies newborn infant toddler child kid son daughter boy girl little
";

pub(super) const A_FEED: &str = "
    milk formula feed feeds feeding bottle breast solids weaning
";

pub(super) const PETS: &str = "
    dog dogs puppy puppies cat cats kitten kittens rabbit rabbits bunny hamster gerbil guinea pig
    parrot budgie bird birds fish goldfish tortoise turtle snake lizard ferret pony horse horses
    chinchilla
";
