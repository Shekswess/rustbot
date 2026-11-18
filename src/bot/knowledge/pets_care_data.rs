//! Authored text and matching shapes from PyBot 7d63c7f, bot/knowledge/pets.py.

// bot/memory/topics.py's lexical stopwords for deciding where a topic starts.
pub(super) const TOPIC_STOPWORDS: &str = "\"his\" \"sh\", \"she \"shed\" \"shes \"shes\" # - 84\" 84, A So The a about about. about: actually after again all alone. already also am an and answer answered. any apostrophe are aren't as at basically be been before being between both but by can can't chat could did didn't do do\" does doesn't doing don't down dropped during each entry even falls few follow-up for from further get got had has have having he hedge her here herself hes him himself his honestly how i i'd i'll i'm i've id if ill im in into is isn't it it's it, its itself ive just left like line literally lot lots make makes many may maybe me might more most much must my naming need needs no nor not of off off, ok okay on once one ones only only: or other our out over own own, perhaps pronoun pronoun, pronouns quite read really right same search set shall she shes should simply so some spelt stems still subject such than that that's the their them themselves then there these they theyd theyll theyre theyve thing this those to too truly typed up us very want was wasn't we well went were weren't what when where which while who whom whose why will with without won't word world would written yeah you youd your youre";

pub(super) const WEIGHING_UP: &str = r#"\b(?:should|shall)\s+(?:i|we)\s+(?:get|have|buy)\s+(?:a|an|another|one)\s+(?:cat|kitten)\b|\b(?:thinking|thought|toying|tempted|torn|unsure|wondering)\b[\w\s,']{0,25}\b(?:of|about|between|on)\b[\w\s,']{0,25}\b(?:getting|having|buying|cat|kitten)\b|\b(?:is it |it )?worth\s+(?:getting|having|buying)\s+(?:a|an|another)?\s*(?:cat|kitten)\b|\b(?:on the fence|two minds|not sure whether|deciding whether)\b[\w\s,']{0,30}\b(?:cat|kitten)\b"#;

pub(super) const FOR_THE_CAT: &str = r#"\b(?:need|needs|buy|buying|shop|shopping|list|checklist|supplies|kit|need to get|have to get)\b|\b(?:flap|litter|tray|trays|bed|beds|carrier|bowl|bowls|food|toys?|collar|microchip)\b|\bwhat (?:should|do) i (?:get|need)\b|\bwhat to (?:get|buy)\b"#;

pub(super) const TWO_OF_THEM: &str = r#"(?i)\btwo (?:cats|kittens)\b|\bpair of (?:cats|kittens)\b"#;

pub(super) const CLEANING_ASK: &str = r#"\b(?:clean|cleans|cleaning|clean out|clean it out|cleaned|scrub|disinfect|smell|smells|smelly|stinks|wash|washing)\b"#;

pub(super) const WHERE_IT_LANDED: &str = r#"\b(rug|rugs|carpet|carpets|bed|beds|duvet|duvets|mattress|sofa|sofas|settee|couch|armchair|chair|floor|floorboards|lino|laminate|bath\s?mat|bathmat|bath|shower|sink|hall|hallway|landing|house|home|indoors|inside|laundry|washing basket|curtains|blinds|spare room|box room|corner|stairs|radiator|wardrobe|cupboard)\b"#;

pub(super) const A_RABBIT_SAID: &str = r#"\b(rabbit|rabbits|bunny|bunnies)\b"#;

pub(super) const A_GUINEA_SAID: &str = r#"\b(guinea|guinea pigs?|cavy|cavies)\b"#;

pub(super) const A_RODENT_SAID: &str =
    r#"\b(hamster|hamsters|gerbil|gerbils|rat|rats|mouse|mice|chinchilla|chinchillas)\b"#;

pub(super) const A_FERRET_SAID: &str = r#"\b(ferret|ferrets)\b"#;

pub(super) const A_RODENT_OR_GUINEA: &str = r#"\b(hamster|hamsters|gerbil|gerbils|guinea|guinea pigs?|cavy|cavies|rat|rats|mouse|mice|chinchilla|chinchillas)\b"#;

pub(super) const A_HOW_TO_HOLD: &str = r#"(?i)\b(?:how|what'?s? the best way|what is the best way|best way|properly|should i)\b[^.!?]{0,40}?\b(?:hold|holding|pick (?:him|her|them|it)?\s?up|picking (?:him|her|them)?\s?up|lift|lifting|handle|handling|carry)\b"#;

pub(super) const A_WONT_BE_HELD: &str = r#"(?i)\b(?:wont|won't|will not|refuses?|refusing|refused)\b[^.!?]{0,24}?\b(?:hold|held|handle|handled|pick (?:him|her|them)?\s?up|picking|being (?:held|picked))\b"#;

pub(super) const A_HURT_OR_FRIGHT: &str = r#"(?i)\b(?:flinch\w*|squeal\w*|squeak\w*|scream\w*|screech\w*|cry|cries|crying|yelp\w*|kick\w*|struggl\w*|squirm\w*|wriggl\w*|scared|frightened|afraid|nervous|hates?|hate being|bit|bites|biting|nip\w*|snapp\w*|pain\w*|hurt\w*|sore|injur\w*)\b"#;

pub(super) const CARE_OF_A_NEW_PET: &str = r#"\b(?:look after|looking after|care for|caring for|care of|bring home|bringing home|brought home|just got|we got|we've got|ive got|i've got|settle|settling|get used to|advice|tips|what do i need to know|first few days|first week|what do i do)\b"#;

pub(super) const A_SHOPPING_ASK: &str = r#"(?i)\b(?:what|which)\b[^?]{0,24}\b(?:do|should|shall|would)\s+i\s+(?:need|buy|get)\b|\bneed(?:s|ed)?\s+to\s+(?:buy|get|pick up)\b|\bshopping list\b|\bstarter kit\b"#;

pub(super) const PET_SETUP: &[(&str, &str)] = &[
    (
        "guinea pig",
        "A guinea pig needs a hutch with a big run attached, or a shed with floor space, hay all day and a handful of nuggets with vitamin C in them, since they cannot make their own. Keep two rather than one - a guinea pig on its own is a lonely animal - and skip the small cage.",
    ),
    (
        "rabbit",
        "A rabbit needs a hutch with a run attached, or a big shed, plus hay all day, a water bottle, a litter tray, bedding, a carrier for the vet and a friend of its own kind. A small cage with an hour out is not enough for a rabbit, and hay is most of what it eats: it wears the teeth down and keeps the gut moving.",
    ),
    (
        "hamster",
        "A hamster needs a deep bed of paper or wood-shaving bedding, a solid wheel and a sand bath, in a cage at least 80cm by 40cm on one level rather than tubes. Spot clean the wet corner daily, change the bedding every few weeks, and give a water bottle rather than a bowl. They are awake at night, so keep the cage out of the bedroom.",
    ),
];

pub(super) const PET_CLEANING: &[(&str, &str)] = &[
    (
        "rabbit",
        "Spot-clean the wet corner and the litter tray every day, change the bedding once a week and give the hutch a deep clean once a month: rabbit urine soaks into wood and the ammonia it gives off damages their lungs. Use a pet-safe disinfectant or a 1:10 white vinegar solution, rinse and dry it before the rabbit goes back in, and skip the phenolic ones like Jeyes fluid - they are toxic to small animals. Vinegar is also what takes the chalky white urine marks off the floor.",
    ),
    (
        "guinea pig",
        "Spot-clean the wet corner and the litter tray every day, change the bedding once a week and give the cage a deep clean once a month: guinea-pig urine soaks into wood and the ammonia it gives off damages their lungs. Use a pet-safe disinfectant or a 1:10 white vinegar solution, rinse and dry it before the guinea pigs go back in, and skip phenolic ones like Jeyes fluid. Keep a handful of the old bedding to put back, because a cage that smells completely new makes them nervous.",
    ),
    (
        "hamster",
        "Spot-clean the wet corner every day and change the bedding every few weeks, with a full scrub of the cage once a month: hamster urine is concentrated and the ammonia builds up fast in a small cage. Hot soapy water or a pet-safe cleaner, rinsed and dried right through before the bedding goes back, because a damp cage gives a hamster a chill and a chest infection. Put a handful of the old bedding back in the nest so the cage still smells of home.",
    ),
];

pub(super) const PET_LITTER: &[(&str, &str)] = &[
    (
        "cat",
        "Scoop the solids out twice a day and change the whole tray of clumping litter once a week - a tray that is cleaned that often does not need disinfecting every time, just hot water and a drop of unscented washing-up liquid. More trays than cats (three for two cats) is what stops accidents, and an unwashed tray is the commonest reason a cat starts going elsewhere. Never mix bleach with cat urine: it gives off a gas that hurts their lungs and yours.",
    ),
    (
        "rabbit",
        "A rabbit litter-trains easily: a tray in the corner it already uses, lined with paper or wood pellets - never clumping cat litter, which sets hard if a rabbit eats it. Lift the wet litter out every day, change the whole tray once or twice a week, and give a pair two trays, because a rabbit likes to eat hay and go in the same spot. Under a hay rack is where it will use it most.",
    ),
    (
        "guinea pig",
        "A guinea pig picks a corner of the cage to use, so a tray there lined with paper or wood pellets catches most of it - never clumping cat litter, which sets hard if it is eaten. Lift the wet litter out every day and change the tray twice a week, and put the hay rack over it, because a guinea pig toilets where it eats.",
    ),
    (
        "hamster",
        "A hamster picks one corner of the cage to pee in, so a small tray there with sand or paper pellets catches it: empty it every day or two and rinse it with hot water. Change the whole cage bedding every few weeks, not every week, because a hamster needs its nest to keep smelling of itself.",
    ),
];

pub(super) const SMALL_PET_ALONE: &[(&str, &str)] = &[
    (
        "rabbit",
        "Rabbits are herd animals, so one on its own is the exception: the best setup is a neutered pair in a hutch with a run attached, and a lone rabbit gets withdrawn, over-grooms and chews the bars. A few hours alone in the day is fine, a whole day and night with nobody in is not. Bonding needs neutral ground, two of everything - bowls, trays, hidey holes - and short meetings that build over a week or two, never one rabbit dropped into the other's hutch. Neuter both first, or they will fight and then breed.",
    ),
    (
        "guinea pig",
        "Guinea pigs are herd animals, so a single one is the exception: two sows, or a neutered boar with a sow, in a big cage or a hutch with a run, is the normal setup, and a guinea pig kept alone gets withdrawn and can stop eating. A few hours alone in the day is fine, a whole day and night with nobody in is not. Bond on neutral ground with two of everything and short meetings, and never drop a new one into the other's cage. A rabbit is not a substitute: they eat different food and a rabbit can injure a guinea pig.",
    ),
];

pub(super) const WHERE_THE_CAT_SLEEPS: &str = "Put it where she already sleeps - the top of a wardrobe, a windowsill, the end of the sofa - and away from the draught between the door and the radiator: a cat picks height and a view over a soft bed on the floor, so a bed in the middle of a busy room stays empty. Warm it with a worn jumper of yours for the first week, and if she ignores it, move it rather than buy another one.";

pub(super) const WHERE_THE_DOG_SLEEPS: &str = "Put it in the quiet corner of the room he already settles in, out of the walkway and away from the radiator he will overheat beside: a dog rests best where he can see the door without lying across the path everyone takes, so the middle of a busy room stays empty. Warm it with a worn jumper of yours for the first week, and if he will not use it, move it to where he already lies down rather than buy another one.";

pub(super) const WHERE_EITHER_SLEEPS: &str = "Put it where she already sleeps - the top of a wardrobe, a windowsill, the end of the sofa - and away from the draught between the door and the radiator: a cat picks height and a view over a soft bed on the floor, so a bed in the middle of a busy room stays empty. Warm it with a worn jumper of yours for the first week, and if she ignores it, move it rather than buy another one. Put it in the quiet corner of the room he already settles in, out of the walkway and away from the radiator he will overheat beside: a dog rests best where he can see the door without lying across the path everyone takes, so the middle of a busy room stays empty. Warm it with a worn jumper of yours for the first week, and if he will not use it, move it to where he already lies down rather than buy another one.";

pub(super) const DOG_SLEEP_WORDS: &[&str] = &[
    "alsatian",
    "beagle",
    "bichon",
    "border collie",
    "bulldog",
    "bullmastiff",
    "cavapoo",
    "cavoodle",
    "chihuahua",
    "cockapoo",
    "cocker spaniel",
    "collie",
    "corgi",
    "cross breed",
    "crossbreed",
    "dachshund",
    "dalmatian",
    "doberman",
    "dog",
    "doggy",
    "dogs",
    "german shepherd",
    "golden retriever",
    "goldendoodle",
    "great dane",
    "greyhound",
    "husky",
    "jack russell",
    "labradoodle",
    "labrador",
    "lurcher",
    "maltese",
    "mastiff",
    "mongrel",
    "pomeranian",
    "poodle",
    "pug",
    "pugs",
    "pup",
    "puppies",
    "puppy",
    "pups",
    "retriever",
    "rottweiler",
    "saint bernard",
    "sausage dog",
    "schnauzer",
    "sheepdog",
    "shih tzu",
    "spaniel",
    "springer",
    "sprocker",
    "st bernard",
    "staffie",
    "staffordshire",
    "terrier",
    "whippet",
    "yorkie",
    "yorkshire terrier",
];

pub(super) const CAT_SLEEP_WORDS: &[&str] = &[
    "bengal",
    "british shorthair",
    "burmese",
    "calico",
    "cat",
    "cats",
    "kitten",
    "kittens",
    "maine coon",
    "moggie",
    "persian",
    "ragdoll",
    "russian blue",
    "siamese",
    "sphynx",
    "tortoiseshell",
];

pub(super) const LITTER_REPLY: &str = "One tray per cat plus one, in quiet places, scooped daily and washed weekly with unscented soap. If it still smells after that, the plastic has picked it up: replace the tray and the litter, which is cheaper than fighting it. If a litter-trained cat suddenly goes outside the tray, a vet check comes first, because it is often a urine infection.";

pub(super) const PET_NAMES: &[(&str, &str)] = &[
    ("guinea pig", r#"\b(guinea ?pigs?|guineas?|cav(?:y|ies))\b"#),
    ("rabbit", r#"\b(rabbits?|bunn(?:y|ies))\b"#),
    ("hamster", r#"\b(hamsters?|syrian|dwarf hamster)\b"#),
    ("tortoise", r#"\b(tortoises?|tort|terrapins?)\b"#),
    (
        "bird",
        r#"\b(budgies?|parrots?|cockatiels?|canar(?:y|ies)|parakeets?)\b"#,
    ),
    ("fish", r#"\b(goldfish|fish|koi|betta|gupp(?:y|ies))\b"#),
    ("cat", r#"\b(cats?|kittens?)\b"#),
    ("dog", r#"\b(dogs?|pupp(?:y|ies))\b"#),
];

pub(super) const KITTEN_KIT: &str = "Litter tray and litter, two bowls, a scratching post, a carrier, a soft bed and kitten food, plus a vet visit booked for the first week. A second litter tray and a few toys save a lot of trouble, and nothing stringy or small enough to swallow.";

pub(super) const CAT_KIT: &str = "Litter tray and litter, two bowls, a scratching post, a carrier, a soft bed and food for its age, plus a vet visit booked for the first week. A second litter tray and a few toys save a lot of trouble, and nothing stringy or small enough to swallow.";

pub(super) const RESCUE_CAT: &str = "A rescue cat comes with a known character, is past the kitten climbing and biting, and settles in a week or two; a kitten is months of work but you shape it from the start. Out at work all day, or a first cat: the adult rescue is the kinder start.";

pub(super) const RESCUE_DOG: &str = "A rescue adult usually comes with a known character, is past the chewing, and settles in a week or two; a puppy is a year of work but you shape everything from the start. First dog, or a busy household: the rescue is the kinder start.";

pub(super) const LITTER_SPOT: &str = "Clean the spot properly first, because the smell is what takes her back to it: soak it with an enzyme cleaner, leave it to work and let it dry, and skip bleach and ammonia, because both smell like wee to a cat. Then the tray - one per cat plus one, scooped daily, unscented litter, in a quiet corner away from the food - and whatever changed in the house: a move, a new pet or a baby, building work, a door that now stays shut. A cat who was always clean and has suddenly started needs the vet first, because a urine infection or a bladder problem sits behind a lot of it.";

pub(super) const GUINEA_HANDLING: &str = "A guinea pig is scooped up in both hands from underneath, one hand under the hindquarters and the other across the shoulders, and held low over your lap - never by the scruff, the ears or a leg, and never up at chest height, because a guinea pig that is dropped or that jumps from a height breaks its back. Sit on the floor with him on your lap for a few minutes a day and a bit of food in your hand, and put him down before he starts to struggle, so being picked up stops being a surprise. A pet that squeals, flinches or struggles at being handled when he did not before needs a vet - a sore tooth, an injured back or mites is the usual reason.";

pub(super) const RODENT_HANDLING: &str = "A hamster, gerbil, rat or mouse is cupped in both hands, one under the chest and one over the back, and never picked up by the tail or the scruff. Sit on the floor with him on your lap for a few minutes a day and a bit of food in your hand, and put him down before he starts to struggle, so being picked up stops being a surprise. A pet that squeals, flinches or struggles at being handled when he did not before needs a vet - a sore tooth, an injured back or mites is the usual reason.";

pub(super) const FERRET_HANDLING: &str = "A ferret is picked up round the shoulders with the hindquarters supported in your other hand, and never by the tail or the ears. Sit on the floor with him on your lap for a few minutes a day and a bit of food in your hand, and put him down before he starts to struggle, so being picked up stops being a surprise. A pet that squeals, flinches or struggles at being handled when he did not before needs a vet - a sore tooth, an injured back or mites is the usual reason.";

pub(super) const RABBIT_HANDLING: &str = "A rabbit is picked up with one hand under the chest and the other under the hindquarters, held against your body so the back legs cannot kick out - never by the ears, the scruff or the back of the neck, because a rabbit that kicks while nothing is holding its back end can break its own spine. Sit on the floor with him on your lap for a few minutes a day and a bit of food in your hand, and put him down before he starts to struggle, so being picked up stops being a surprise. A pet that squeals, flinches or struggles at being handled when he did not before needs a vet - a sore tooth, an injured back or mites is the usual reason and a rabbit that stops eating after it is worth ringing the vet the same day.";
