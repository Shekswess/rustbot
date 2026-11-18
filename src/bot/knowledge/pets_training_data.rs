//! Authored text and vocabulary from PyBot 7d63c7f, bot/knowledge/pets.py.

pub(super) const HANDS_OR_PLAY: &[&str] = &[
    "hand",
    "hands",
    "finger",
    "fingers",
    "thumb",
    "wrist",
    "play",
    "playing",
    "played",
    "game",
    "games",
    "mouth",
    "mouths",
    "mouthing",
    "mouthed",
    "nip",
    "nips",
    "nipping",
    "nipped",
    "toy",
    "toys",
    "trousers",
    "sleeve",
    "sleeves",
    "shoelaces",
    "ankles",
];

pub(super) const NAME_ASK: &str = r#"\b(?:what|whats|what's|which)\b.{0,30}\b(?:name|called|named)\b|\bwhat\s+(?:should|shall|do|would|will)\s+(?:i|we)\s+(?:call|name)\b|\b(?:is|are|was|were)\s+(?:called|named)\b"#;

pub(super) const CALLED_NAME: &str = r#"\b(?:called|named)\s+([a-z]+)|\b(?:we|i|they|she|he)\s+call\s+(?:our|my|the|their|his|her)\s+[a-z]+\s+([a-z]+)"#;

pub(super) const AGE_PATTERN: &str = r#"\b(\d{1,2}|half a|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty)\s*(years?|yrs?|yo|months?|mths?|weeks?|wks?)\s*(?:old\b|of age\b)"#;

pub(super) const CALLED_NOT_A_NAME: &[&str] = &[
    "a",
    "again",
    "an",
    "and",
    "anything",
    "at",
    "back",
    "because",
    "but",
    "by",
    "each",
    "every",
    "everything",
    "for",
    "he",
    "her",
    "him",
    "how",
    "if",
    "in",
    "it",
    "me",
    "nothing",
    "off",
    "on",
    "out",
    "over",
    "she",
    "so",
    "something",
    "the",
    "them",
    "they",
    "to",
    "twice",
    "up",
    "us",
    "what",
    "when",
    "which",
    "who",
    "why",
    "yet",
];

pub(super) const DOG_RECALL_REPLY: &str = "Practise coming back somewhere boring, on a long line, saying his name once and rewarding with something better than the treats he gets otherwise: cheese or sausage. Never call him to something he dislikes, like a bath or the end of a walk, or the word stops working.";

pub(super) const WARY_ONE: &[(&str, &str)] = &[
    ("husband", "your husband"),
    ("wife", "your wife"),
    ("partner", "your partner"),
    ("boyfriend", "your boyfriend"),
    ("girlfriend", "your girlfriend"),
    ("brother", "your brother"),
    ("sister", "your sister"),
    ("son", "your son"),
    ("daughter", "your daughter"),
    ("dad", "your dad"),
    ("father", "your dad"),
    ("mum", "your mum"),
    ("mother", "your mum"),
    ("grandad", "your grandad"),
    ("grandma", "your gran"),
    ("postman", "the postman"),
    ("neighbour", "your neighbour"),
    ("neighbor", "your neighbour"),
    ("friend", "your friend"),
    ("baby", "the baby"),
    ("toddler", "the toddler"),
    ("child", "the child"),
    ("man", "that man"),
    ("someone", "that person"),
];

pub(super) const WARY_MANY: &[(&str, &str)] = &[
    ("men", "men"),
    ("strangers", "strangers"),
    ("people", "people"),
    ("visitors", "visitors"),
    ("visitor", "that visitor"),
    ("guests", "guests"),
    ("guest", "that guest"),
    ("children", "children"),
    ("kids", "children"),
    ("teenagers", "teenagers"),
    ("babies", "babies"),
    ("toddlers", "toddlers"),
    ("friends", "your friends"),
    ("new people", "new people"),
];

pub(super) const OTHER_PETS: &[&str] = &[
    "guinea pig",
    "guinea pigs",
    "guinea",
    "hamster",
    "hamsters",
    "gerbil",
    "gerbils",
    "chinchilla",
    "chinchillas",
    "ferret",
    "ferrets",
    "parrot",
    "parrots",
    "budgie",
    "budgies",
    "parakeet",
    "cockatiel",
    "tortoise",
    "tortoises",
    "lizard",
    "lizards",
    "snake",
    "snakes",
    "gecko",
    "geckos",
    "fish",
    "goldfish",
    "rat",
    "rats",
    "mouse",
    "mice",
    "degus",
    "hedgehog",
    "hedgehogs",
    "chicken",
    "chickens",
    "duck",
];

pub(super) const AGE_WORDS: &[(&str, f32)] = &[
    ("one", 1.0),
    ("two", 2.0),
    ("three", 3.0),
    ("four", 4.0),
    ("five", 5.0),
    ("six", 6.0),
    ("seven", 7.0),
    ("eight", 8.0),
    ("nine", 9.0),
    ("ten", 10.0),
    ("eleven", 11.0),
    ("twelve", 12.0),
    ("thirteen", 13.0),
    ("fourteen", 14.0),
    ("fifteen", 15.0),
    ("sixteen", 16.0),
    ("seventeen", 17.0),
    ("eighteen", 18.0),
    ("nineteen", 19.0),
    ("twenty", 20.0),
    ("a", 1.0),
    ("half a", 0.5),
];

pub(super) const NIGHT_AGE: &str = r#"\b(?:(?:is|aged|about|nearly|almost|turned|turning|over|just over)\s+(\d{1,2})\b|(\d{1,2})\s*(?:years?\s+old|years?|yrs?|yo)\b)"#;

pub(super) const YOUNG_CHEW: &str = "Chewing is normal while teeth change, so redirect it: plenty of chew toys, a frozen carrot or stuffed Kong, and pick everything else up out of reach. Puppies usually grow out of it by seven or eight months.";

pub(super) const ADULT_CHEW: &str = "An adult dog chews for a reason and teeth are not it: boredom, a walk that is too short, or being left alone longer than it can take. Give it a proper sniffy walk before you go out, leave a stuffed Kong or a long-lasting chew, and block off the sofa for a fortnight while the habit breaks. If it has just started in a dog that never did it, get its teeth and gums checked, because a sore mouth makes a dog chew to soothe it.";

pub(super) const BARKING: &str = "Find what starts it and block that first: a curtain over the window he watches, or the post put through a box away from the door. Then reward the quiet - a treat the moment he stops - and don't shout, because to a dog shouting sounds like you joining in. If he barks for hours when he is left, that is the being-left problem rather than the barking one.";

pub(super) const BARKING_CAUSE: &str = "Barking is usually the alarm going off, so find what starts it and block that first: a curtain over the window he watches, or the post put through a box away from the door. Then reward the quiet - a treat the moment he stops - and don't shout, because to a dog shouting sounds like you joining in. If he barks for hours when he is left, that is the being-left problem rather than the barking one.";

pub(super) const WARY_DEFAULT: &str = "Let him come to the person he is wary of on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while that person sits nearby and ignores the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

pub(super) const CAT_NOISE: &str = "Let a cat leave the room and shut the door - walking away is what settles them - then switch it on for a few seconds at a time with the food bowl down, stopping before they bolt. Don't hold a cat still in front of it: a frightened animal that can't get away gets worse, not braver. If they shake for hours or won't eat afterwards, ask the vet.";

pub(super) const DOG_NOISE: &str = "Let a dog leave the room and shut the door - walking away is what settles them - then switch it on for a few seconds at a time with the food bowl down, stopping before they bolt. Don't hold a dog still in front of it: a frightened animal that can't get away gets worse, not braver. If they shake for hours or won't eat afterwards, ask the vet.";

pub(super) const RABBIT_NOISE: &str = "Let a rabbit leave the room and shut the door - walking away is what settles them - then switch it on for a few seconds at a time with the food bowl down, stopping before they bolt. Don't hold a rabbit still in front of it: a frightened animal that can't get away gets worse, not braver. If they shake for hours or won't eat afterwards, ask the vet.";

pub(super) const PUPPY_NIGHT: &str = "A puppy cannot hold it all night: at eight weeks expect to get up once, around 2 or 3am, and take him out just before you go to bed. Keep the last meal early and the bed beside yours so he can smell you, and answer the crying quietly and briefly with no lights and no play, so night stays boring. Most puppies sleep seven hours by about four months.";

pub(super) const SENIOR_NIGHT: &str = "An older dog who starts waking at night is usually in discomfort rather than being naughty: arthritis makes lying still painful, and a dog who is stiff, pacing, panting or confused in the evening needs the vet rather than a training plan. Book a check, ask about pain relief, put a soft bed in a warm room with a night light, and take him out once last thing; sudden confusion in an old dog is worth a same-day call.";

pub(super) const DOG_NIGHT: &str = "Keep the night dull and the day busy: a last toilet trip on the lead, no lights, no chat and no play when he wakes, and a long walk plus some training in the afternoon so he is tired at bedtime. If he is sleeping a lot in the day, wake him gently for a short walk. A dog who starts waking at night after years of sleeping through is worth a vet check, especially if he is drinking more or seems stiff.";

pub(super) const CAT_BORED: &str = "Play with a wand toy for five or ten minutes, two or three times a day, and let her stalk and pounce before you let her catch it, then give her a meal so the hunt has an end. The rest of the day wants things rather than toys: a food puzzle, a cardboard box, a paper bag, somewhere high to sit and a window to watch. Rotate the toys and put half of them away, because a basket she has seen for months is furniture.";

pub(super) const RABBIT_BORED: &str = "A rabbit is bored when it has nothing to chew and nowhere to dig: give it a run of a few hours a day outside a hutch, a box of hay to root through, willow or apple sticks to chew and something to hide under. A lone rabbit wants a rabbit friend more than it wants toys, and the two are happiest neutered and introduced slowly.";

pub(super) const DOG_BORED_WORK: &str = "Four hours at a time is about the usual comfortable limit for an adult dog, so build the time up in short chunks and have someone look in at lunchtime for a puppy. Then fill the day: half his dinner scattered on the lawn or in a snuffle mat, something legal to chew, the radio on, and a walk where he is allowed to sniff when you get back. A dog left with nothing to do chews the furniture and paces, and that is boredom rather than badness.";

pub(super) const DOG_BORED: &str = "A dog is bored because the day is short on sniffing and chewing, not because he needs more toys. One walk where he is allowed to stop and sniff does more than a long march, and a scatter of his dinner on the lawn or in a snuffle mat takes ten minutes off the clock. Add five minutes of training a day, something legal to chew and a game of tug, and break the day up rather than saving it all for the evening.";

pub(super) const CAT_NAME: &str = "Yes - a cat can pick her own name out of a list of similar words, so she knows it about as well as a dog does; what she does not do is come running when she hears it. Say it before the food or the wand toy rather than after, and she will start looking up when you use it. A cat that stops answering her name may have gone deaf, which a vet can check, or may just have decided the sofa is better.";

pub(super) const RABBIT_NAME: &str = "A rabbit learns its name as a sound that means food, not the way a dog does: say it just before you put the bowl down or open the run, and it will start turning towards you. They are prey animals and will not usually come when called, and a rabbit that ignores its name has not forgotten it.";

pub(super) const DOG_NAME: &str = "Yes - most family dogs understand a few dozen words, and the best of them well over a hundred, and their own name is the first. Say the name once and then the cue - 'Biscuit, sit' - and reward, so the name becomes the thing to look at before something good happens. Never use it to tell him off: a name that means trouble is a name he learns to ignore.";
