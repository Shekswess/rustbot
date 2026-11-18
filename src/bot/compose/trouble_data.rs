// Patterns, vocabularies and pools from the pinned trouble reader.

pub(super) const OTHERS: &[&str] = &[
    "aunt",
    "baby",
    "boss",
    "boy",
    "boyfriend",
    "brother",
    "carer",
    "cat",
    "child",
    "colleague",
    "cousin",
    "dad",
    "daddy",
    "daughter",
    "dog",
    "father",
    "fiance",
    "fiancee",
    "flatmate",
    "friend",
    "girl",
    "girlfriend",
    "gran",
    "grandad",
    "grandma",
    "grandpa",
    "grandparents",
    "granny",
    "he",
    "her",
    "hers",
    "him",
    "his",
    "housemate",
    "husband",
    "kid",
    "kids",
    "kitten",
    "landlady",
    "landlord",
    "mam",
    "manager",
    "mate",
    "mom",
    "mother",
    "mum",
    "mummy",
    "nan",
    "nana",
    "neighbor",
    "neighbour",
    "nephew",
    "niece",
    "nurse",
    "parent",
    "partner",
    "pet",
    "pup",
    "puppy",
    "she",
    "sibling",
    "sister",
    "son",
    "teenager",
    "tenant",
    "their",
    "theirs",
    "them",
    "they",
    "toddler",
    "uncle",
    "vet",
    "wife",
];
pattern!(
    THING,
    r#"(?x)
    \b(?:
        wont | won't | doesn'?t\s+work | isnt\s+working | isn'?t\s+working
      | not\s+(?:working|responding|turning\s+on|charging|draining|connecting|booting)
      | stopped\s+working | stopped\s+charging
      | broke | broken | broke\s+down | bust | faulty | on\s+the\s+blink | playing\s+up
      | leak(?:s|ing|ed)? | drain(?:s|ing)? | soaking | dripping | blocked | clogged | jammed | stuck
      | keeps?\s+(?:freezing|crashing|cutting|dropping|stopping|going\s+off|calling|
                    restarting|turning\s+off|coming\s+up|showing|flashing)
      | cutting\s+out | crashing | froze | frozen | freezes | collapses? | collapsed
      | makes?\s+a\s+noise | making\s+a\s+noise | noisy | rattl(?:es|ing|ed)
      | die(?:s|d)? | dead | flat\s+(?:tyre|battery) | slow | sluggish | overheats? | overheating
      | error | glitch(?:y|es)? | corrupt(?:ed)? | wont\s+connect | no\s+signal
      | needs?\s+(?:a\s+)?(?:fixing|servicing)
      | on\s+its\s+way\s+out
      | on\s+(?:its|his|her|their)\s+last\s+legs
      | clapped\s+out | seen\s+better\s+days | knackered
      | went\s+bang | blew\s+(?:a\s+)?fuse

      | (?:its|it's|mine|ours)\s+had\s+(?:it|its\s+day)
      | (?:mine|ours)\s+(?:has|have|had)\s+had\s+(?:it|its\s+day)
      | (?:my|our|the|this|that|these|those)\s+(?:[\w']+\s+){0,3}
          (?:has|have|had)\s+had\s+(?:it|its\s+day)
      | (?:my|our|the|this|that|these|those)\s+(?:[\w']+\s+){0,3}
          (?:is|are|was|were)\s+done\s+for

      | (?:mine|ours)\s+(?:has|have|had)?\s*(?:packed\s+(?:up|in)|(?:gave|given)\s+up)
      | (?:the|my|our|this|that|his|her|their|it|its)\s+(?:[\w']+\s+){0,3}
          (?:has|have|had)?\s*(?:packed\s+(?:up|in)|(?:gave|given)\s+up)

      | 
    (?:has|have|had|is|are|was|were)\s+
      (?:(?:gone|got|become|turned)\s+)?
      (?:all\s+|a\s+bit\s+|a\s+lot\s+|really\s+|very\s+|quite\s+|so\s+|completely\s+
         |totally\s+|properly\s+|now\s+)?
      (?:a\s+|an\s+)?
      (?:
          black|brown|green|rusty|mouldy|moldy|mildewed|lumpy|soggy|slimy|sticky|greasy
        | furry|furred|scaled|pitted|warped|brittle|cracked|frayed|threadbare|bubbled
        | blistered|peeling|discoloured|discolored|crusty
        | burnt|burned|scorched|seized|rusted|corroded|perished|split|snapped|melted|bent
        | stuck|jammed|solid|knackered|kaput|buggered|stiff|dropped|flat
        | yellow|patchy|faded|bald|rip(?![\s-]+off\b)
        | crack|hole|tear|dent|patch
      )
      (?:\s+(?:on|up|through|off|shut|together|solid|apart|open|down|in|out))?
  | burnt\s+on
  | blown\s+(?:down|off|out|over|away)
  | (?:hanging|fell|fallen|come|coming|pulled)\s+(?:off|away|loose|apart|down|open)
  | conked\s+(?:out|off)

    )\b
"#
);
pattern!(
    CANT_GET,
    r#"(?x)
    \b(?:can'?t|cannot|can\s+not|couldn'?t|could\s+not|unable\s+to|not\s+able\s+to)
      (?:\s+seem\s+to)?
      \s+(?:get|getting|make|making|keep|keeping|stop|stopping)\b
      (?:\s+[\w']+){1,4}?      # the thing: "my washing machine", "the app", "this printer"
      \s+(?:to\s+(?:work|job|run|go|start|stop|spin|turn|open|close|load|boot|charge
                      |connect|pair|sync|print|scan|save|drain|heat|light|fire|play|update
                      |install|download|upload|respond|unlock|switch|move|boil|cook|rise
                      |freeze|cool|record|log\s?in)
          |working|going|started|running|charged|connected|opening|booting)
"#
);
pattern!(
    FLAT,
    r#"\b(?:is|are|was|were|has|have|had|went|goes|just|all|now|so|really|completely|totally|kind\s+of|sort\s+of)?\s*(?:gone|went|died|dead|faded|fading|vanished|disappeared|dried\s+up|evaporated|left\s+me)\b(?!\s+(?:up|down)\b)"#
);
pub(super) const FAULT_FILLERS: &[&str] = &[
    "a",
    "again",
    "also",
    "always",
    "am",
    "an",
    "and",
    "are",
    "be",
    "been",
    "being",
    "bit",
    "but",
    "completely",
    "getting",
    "gone",
    "had",
    "hadnt",
    "half",
    "has",
    "hasnt",
    "have",
    "is",
    "isnt",
    "it",
    "its",
    "just",
    "keep",
    "keeps",
    "kept",
    "little",
    "mostly",
    "never",
    "no",
    "not",
    "now",
    "of",
    "partly",
    "properly",
    "really",
    "so",
    "sometimes",
    "still",
    "the",
    "then",
    "today",
    "too",
    "totally",
    "very",
    "was",
    "went",
    "were",
    "yesterday",
];
pattern!(
    DURATION,
    r#"(?x)
    \b(?:
        for\s+(?:about\s+|nearly\s+|over\s+|almost\s+)?(?:\w+\s+){0,2}
            (?:days?|nights?|weeks?|months?|years?)
      | since\s+(?:\w+\s+){0,3}(?:days?|weeks?|months?|years?|monday|tuesday|wednesday|
            thursday|friday|saturday|sunday|christmas|january|february|march|april|may|june|
            july|august|september|october|november|december)
      | going\s+on\s+(?:\w+\s+){0,2}(?:days?|weeks?|months?|years?)
      | (?:started|began)\s+(?:\w+\s+){0,2}(?:yesterday|today|last\s+week|last\s+night|
            this\s+morning|on\s+monday|a\s+(?:week|month|few\s+days)\s+ago|
            (?:\d+|two|three|four|five|six|seven|eight|nine|ten)\s+days\s+ago)
      | \b(?:two|three|four|five|six|seven|eight|nine|ten|twenty|thirty|a\s+few|several|many)
            \s+(?:days?|weeks?|months?|years?)
      | \b\d+\s*(?:days?|weeks?|months?|years?)
    )\b
"#
);
pub(super) const BROKEN: &[&str] = &[
    "That's a pain. What's it doing exactly, and when did it start?",
    "Annoying. Is it every time, or only now and then?",
    "Right. What have you tried so far?",
    "That would bother me too. Tell me what it does and I'll work out the next step with you.",
    "Ugh, those are fiddly. How long has it been like that?",
];
pub(super) const BROKEN_TIME: &[&str] = &[
    "That's a pain. What's it doing exactly, and when did it start?",
    "Annoying. Is it every time, or only now and then?",
    "Right. What have you tried so far?",
    "That would bother me too. Tell me what it does and I'll work out the next step with you.",
];
pattern!(
    A_DECLINE,
    r#"(?x)\b(?:
    on\s+its\s+way\s+out
  | on\s+(?:its|his|her|their)\s+last\s+legs
  | clapped\s+out | seen\s+better\s+days

      | (?:its|it's|mine|ours)\s+had\s+(?:it|its\s+day)
      | (?:mine|ours)\s+(?:has|have|had)\s+had\s+(?:it|its\s+day)
      | (?:my|our|the|this|that|these|those)\s+(?:[\w']+\s+){0,3}
          (?:has|have|had)\s+had\s+(?:it|its\s+day)
      | (?:my|our|the|this|that|these|those)\s+(?:[\w']+\s+){0,3}
          (?:is|are|was|were)\s+done\s+for

      | (?:mine|ours)\s+(?:has|have|had)?\s*(?:packed\s+(?:up|in)|(?:gave|given)\s+up)
      | (?:the|my|our|this|that|his|her|their|it|its)\s+(?:[\w']+\s+){0,3}
          (?:has|have|had)?\s*(?:packed\s+(?:up|in)|(?:gave|given)\s+up)

)\b"#
);
pub(super) const DECLINE: &[&str] = &[
    "That's a pain. What's it doing exactly, and when did it start?",
    "Annoying. What's it doing when it goes wrong?",
    "That would bother me too. Tell me what it does and I'll work out the next step with you.",
    "Right. What is it doing that makes you think it is on the way out?",
];
pub(super) const DECLINE_TIME: &[&str] = &[
    "Annoying. What's it doing when it goes wrong?",
    "That would bother me too. Tell me what it does and I'll work out the next step with you.",
    "Right. What is it doing that makes you think it is on the way out?",
];
pattern!(
    STILL_FAILING,
    r#"\b(?:on\s+its\s+way\s+out|on\s+(?:its|his|her|their)\s+last\s+legs|clapped\s+out|seen\s+better\s+days)\b"#
);
pub(super) const SORE: &[&str] = &[
    "That sounds wearing. Is it getting better, worse, or about the same?",
    "Does it come and go, or is it there all the time?",
    "Right. What have you tried for it so far?",
    "That's a nuisance to be carrying round. How long has it been like that?",
];
pub(super) const SORE_TIME: &[&str] = &[
    "That's a nuisance to be carrying round. What's it like at its worst?",
    "That sounds wearing. Is it getting better, worse, or about the same?",
    "Does it come and go, or is it there all the time?",
    "Right. What have you tried for it so far?",
];
pub(super) const STRAIN: &[&str] = &[
    "That sounds wearing. What's gone wrong, do you think?",
    "Right, that is a lot to carry. What is it like at the moment?",
    "That would weigh on me too. What has changed?",
    "Ugh, that is a hard one. How long has it been like that?",
    "Right. What would make the most difference, if it changed?",
];
pub(super) const STRAIN_TIME: &[&str] = &[
    "That sounds wearing. What's gone wrong, do you think?",
    "Right, that is a lot to carry. What is it like at the moment?",
    "That would weigh on me too. What has changed?",
    "Right. What would make the most difference, if it changed?",
];
pub(super) const SORE_NEXT: &[&str] = &[
    "And what makes it worse - moving about, sitting still, or nothing in particular?",
    "Is it the same all day, or worse at certain times?",
    "Has anything helped - a warm bath, a painkiller, or just resting it?",
];
pub(super) const BROKEN_NEXT: &[&str] = &[
    "Does it do it every time, or only now and then?",
    "Has anything you tried made a difference?",
    "What's it doing when it goes wrong?",
];
pub(super) const STRAIN_NEXT: &[&str] = &[
    "Is it one part of it, or the whole thing?",
    "Has anything helped, even a little?",
    "What would better look like, if it came?",
];
pattern!(
    PLEA,
    r#"\b(?:tips?|advice|suggestions?|recommendations?|what\s+should\s+i|how\s+(?:do|can|should)\s+i|any\s+ideas?|help\s+me)\b"#
);
pattern!(MINE, r#"^(?:i\b|i'm\b|im\b|i've\b|ive\b|my\b|our\b)"#);
pub(super) const OWNER_END: &[&str] = &["'s", "s'", "’s", "s"];
pattern!(
    DETAIL,
    r#"(?x)\b(?:
    sore | sorer | hurting | hurts? | ache | aches | aching | achy | stiff | stiffness
  | swollen | swelling | tender | throb\w* | sharp | burning | numb | tight | bruised
  | worse | worst | better | eased? | eases | easier | helped | helps | relief
)\b"#
);
pattern!(
    TRIGGER,
    r#"(?x)\b(?:
    when\s+i | whenever\s+i | if\s+i | after\s+i | since\s+i
  | only\s+when | only\s+if | when\s+it | whenever\s+it | if\s+it
  | in\s+the\s+(?:morning|morning|evening|evenings|night)
  | first\s+thing | all\s+day | overnight
  | every\s+time | each\s+time | all\s+the\s+time | now\s+and\s+(?:then|again)
  | sometimes | most\s+(?:days|mornings|nights) | lately
  | sitting | standing | walking | running | lifting | bending | lying | sleeping | driving
)\b"#
);
pattern!(
    NOT_A_STRAIN_DETAIL,
    r#"^(?:so |and |but |well |ok |okay )*(?:how|what|why|when|where|which|who|should|can|could|would|will|is|are|do|does|did|shall)\b|\b(?:broke|broken|died|dead|ill|hospital|accident|leaking|not working|stopped working)\b"#
);
pattern!(
    A_SENTENCE,
    r#"\b(?:am|is|are|was|were|be|been|being|has|have|had|do|does|did|can|could|will|would|shall|should|may|might|must|i'm|im|i've|ive|i'd|we're|we've|he's|she's|there's|get|gets|got|go|cant|cannot|can't|dont|don't|didnt|didn't|wont|won't|isnt|isn't|wasnt|wasn't|havent|haven't|hasnt|hasn't|couldnt|couldn't|wouldnt|wouldn't|doesnt|doesn't|arent|aren't|aint|goes|going|went|keep|keeps|kept|want|wants|need|needs|feel|feels|felt|think|thinks|seem|seems|seemed|know|knows|make|makes|made|work|works|worked|start|starts|started|stop|stops|stopped|try|tries|tried|lost|lose|loses|die|died|broke|broken|hurt|hurts)\b"#
);
pub(super) const A_HEDGE: &[&str] = &[
    "do",
    "don't",
    "dont",
    "dunno",
    "i",
    "idea",
    "idk",
    "know",
    "maybe",
    "meh",
    "no",
    "not",
    "nothing",
    "nothing's",
    "nothings",
    "sure",
    "whatever",
];
pattern!(
    A_LENGTH_OF_TIME,
    r#"\b(?:\d+|a|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a couple of|a few|several)\s+(?:days?|weeks?|months?|years?)\b|\b(?:days|weeks|months|years|ages)\b(?:\s+(?:now|too|already))*\W*$|\bsince\s+(?:yesterday|this morning|last night|the weekend|christmas|the summer|last \w+|monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december)\b|\bfor\s+(?:years|months|weeks|ages|a long time)\b|\b(?:ages|a long time|all (?:week|month|year))\b"#
);
pattern!(
    ALREADY_TRIED,
    r#"\b(?:i|we|ive|i've|weve|we've|hes|he's|shes|she's|they|they've)\s+(?:already\s+|just\s+|even\s+|also\s+)?(?:\w+ly\s+)?(?:tried|did|had a go|gave it a go|used|\w+ed)\b"#
);
pattern!(IT, r#"\b(?:it|its|it's|this|that|these|those)\b"#);
pattern!(MINE_AGAIN, r#"\bmy\s+([a-z']+)"#);
pattern!(
    WHAT_IT_DOES,
    r#"(?i)what\s+it\s+does|what's\s+it\s+doing|what\s+is\s+it\s+doing"#
);
pattern!(
    WANTING_OR_PRAISE,
    r#"(?x)
    \b(?:
        to\s+die\s+for
      | worth\s+dying\s+for
      | (?:would|could|might|will)\s+(?:die|kill)\s+for
      | (?:i|you|we|they|he|she)'?d\s+(?:die|kill)\s+for
      | (?:would|could|will)?\s*die[sd]?\s+on\s+(?:that|this|a|the)\s+hill
      | dy(?:ing|in)\s+(?:for|to)\b
    )
"#
);
pub(super) const DEAD_AS_VERY_AFTER: &[&str] = &[
    "ahead",
    "annoying",
    "bad",
    "boring",
    "bright",
    "broke",
    "certain",
    "cheap",
    "cheerful",
    "chuffed",
    "clean",
    "cold",
    "comfortable",
    "comfy",
    "cosy",
    "cozy",
    "dark",
    "easy",
    "embarrassing",
    "fast",
    "friendly",
    "fun",
    "funny",
    "fussy",
    "gentle",
    "glad",
    "good",
    "happy",
    "hard",
    "heavy",
    "jealous",
    "keen",
    "knackered",
    "loud",
    "lucky",
    "mint",
    "nice",
    "obvious",
    "pretty",
    "proud",
    "quick",
    "quiet",
    "relaxed",
    "rude",
    "scary",
    "serious",
    "shattered",
    "silent",
    "simple",
    "skint",
    "slow",
    "smart",
    "smooth",
    "soft",
    "straight",
    "strong",
    "sure",
    "sweet",
    "tired",
    "tricky",
    "warm",
    "wrong",
];
pub(super) const ODD_MARK: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~’";
pub(super) const ANIMAL_MODIFIERS: &[&str] = &[
    "bird", "birds", "budgie", "cat", "cats", "chicken", "chickens", "dog", "dogs", "ferret",
    "fish", "guinea", "hamster", "hamsters", "horse", "horses", "kitten", "kittens", "lizard",
    "mouse", "parrot", "pet", "pets", "pig", "pigs", "pup", "puppies", "puppy", "pups", "rabbit",
    "rabbits", "rat", "rats", "snake", "tortoise",
];
pub(super) const NOT_A_MODIFIER: &[&str] = &[
    "am", "and", "are", "be", "been", "being", "big", "but", "cannot", "cant", "could", "dead",
    "didnt", "die", "died", "dies", "doesnt", "get", "gets", "getting", "goes", "going", "got",
    "had", "has", "have", "ill", "is", "isnt", "keep", "keeps", "kept", "little", "may", "might",
    "must", "need", "needs", "old", "should", "sick", "small", "so", "that", "to", "was", "wasnt",
    "went", "were", "which", "who", "will", "wont", "would", "young",
];
pub(super) const JOINING: &[&str] = &[
    "a",
    "again",
    "all",
    "always",
    "an",
    "and",
    "are",
    "be",
    "been",
    "being",
    "bit",
    "but",
    "completely",
    "feel",
    "feels",
    "felt",
    "gets",
    "got",
    "had",
    "has",
    "have",
    "here",
    "is",
    "it",
    "it's",
    "its",
    "just",
    "keep",
    "keeps",
    "kept",
    "kind",
    "little",
    "looks",
    "lot",
    "now",
    "off",
    "on",
    "one",
    "really",
    "seem",
    "seems",
    "so",
    "sort",
    "sounds",
    "still",
    "suddenly",
    "the",
    "then",
    "there",
    "totally",
    "twice",
    "two",
    "up",
    "very",
    "was",
    "were",
];
pattern!(
    OWNS,
    r#"\b(?:my|our|mine|ours|the|this|that|it|its|it's)\b"#
);
pub(super) const BODY_PART: &[&str] = &[
    "ankle",
    "ankles",
    "arm",
    "arms",
    "back",
    "belly",
    "calf",
    "calves",
    "cheek",
    "cheeks",
    "chest",
    "chin",
    "collarbone",
    "collarbones",
    "ear",
    "ears",
    "elbow",
    "elbows",
    "eye",
    "eyebrow",
    "eyebrows",
    "eyes",
    "feet",
    "finger",
    "fingers",
    "foot",
    "forearm",
    "forearms",
    "groin",
    "gum",
    "gums",
    "hamstring",
    "hamstrings",
    "hand",
    "hands",
    "head",
    "heart",
    "heel",
    "heels",
    "hip",
    "hips",
    "jaw",
    "knee",
    "knees",
    "knuckle",
    "knuckles",
    "leg",
    "legs",
    "neck",
    "rib",
    "ribs",
    "shin",
    "shins",
    "shoulder",
    "shoulder blade",
    "shoulders",
    "sinus",
    "sinuses",
    "skin",
    "spine",
    "stomach",
    "tailbone",
    "teeth",
    "thigh",
    "thighs",
    "throat",
    "toe",
    "toes",
    "tonsils",
    "tooth",
    "wrist",
    "wrists",
];
pattern!(
    NUISANCE,
    r#"\b(?:my|our)\s+\w+(?:\s+\w+){0,2}\s+(?:is|are|was|were|'s)\s+(?:(?:a|real|right|such|quite|so|very|bit\s+of|proper|little)\s+){0,2}(?:pain|pains|painful|sore|stiff)\b"#
);
pattern!(
    THING_STATE_ONLY,
    r#"(?x)\b(?:
    (?:has|have|had|is|are|was|were)\s+
      (?:(?:gone|got|become|turned)\s+)?
      (?:all\s+|a\s+bit\s+|a\s+lot\s+|really\s+|very\s+|quite\s+|so\s+|completely\s+
         |totally\s+|properly\s+|now\s+)?
      (?:a\s+|an\s+)?
      (?:
          black|brown|green|rusty|mouldy|moldy|mildewed|lumpy|soggy|slimy|sticky|greasy
        | furry|furred|scaled|pitted|warped|brittle|cracked|frayed|threadbare|bubbled
        | blistered|peeling|discoloured|discolored|crusty
        | burnt|burned|scorched|seized|rusted|corroded|perished|split|snapped|melted|bent
        | stuck|jammed|solid|knackered|kaput|buggered|stiff|dropped|flat
        | yellow|patchy|faded|bald|rip(?![\s-]+off\b)
        | crack|hole|tear|dent|patch
      )
      (?:\s+(?:on|up|through|off|shut|together|solid|apart|open|down|in|out))?
  | burnt\s+on
  | blown\s+(?:down|off|out|over|away)
  | (?:hanging|fell|fallen|come|coming|pulled)\s+(?:off|away|loose|apart|down|open)
  | conked\s+(?:out|off)
)\b"#
);
pub(super) const GROWING: &[&str] = &[
    "allotment",
    "bush",
    "bushes",
    "crops",
    "field",
    "fields",
    "flower",
    "flowers",
    "fruit",
    "garden",
    "grass",
    "hedge",
    "hedgerow",
    "hedges",
    "ivy",
    "lawn",
    "lawns",
    "leaf",
    "leaves",
    "meadow",
    "moss",
    "plant",
    "plants",
    "potatoes",
    "shrub",
    "shrubs",
    "tomato",
    "tomatoes",
    "tree",
    "trees",
    "veg",
    "vegetables",
    "weed",
    "weeds",
];
pattern!(
    DEATHWISH,
    r#"\b(?:want|wanted|wanting|going|gonna|wish|wished|rather)\s+(?:to\s+)?(?:die|dead)\b|\b(?:die|dye)\s+my\s+hair\b"#
);
pattern!(
    ACUTE,
    r#"(?x)
    \b(?:
        burn(?:s|ed|t)? | scald(?:s|ed)? | swell(?:s|ing|ed)? | swellings?
      | throat | lips? | tongue | breath(?:e|es|ing|less)? | choking | choked
      | bleed(?:s|ing)? | bled | numb | tingling | vision | sight | blurred
      | fell | fallen | fall(?:s|ing)? | collapsed | collapse | unconscious | dizzy | fainted
      | cut(?:s)? | gash | wound | crushed | twisted | sting(?:s|ing)?
      | rash(?:es|y)? | pois(?:on|oned|oning) | overdose | pill(?:s)? | seizure | sezure
      | can'?t\s+see | cant\s+see | can'?t\s+move | cant\s+move | can'?t\s+breathe
    )\b
"#
);
pattern!(FRACTURE, r#"\b(?:broken|fractures?|fractured)\b"#);
pub(super) const BONE: &[&str] = &[
    "ankle",
    "bone",
    "bones",
    "brow",
    "cheek",
    "cheekbone",
    "chin",
    "collar",
    "collarbone",
    "elbow",
    "elbows",
    "eyebrow",
    "face",
    "heel",
    "hip",
    "jaw",
    "kneecap",
    "knuckle",
    "knuckles",
    "lip",
    "mouth",
    "nose",
    "pelvis",
    "rib",
    "ribs",
    "shin",
    "shoulder",
    "skull",
    "spine",
    "tailbone",
    "teeth",
    "temple",
    "thigh",
    "tooth",
    "wrist",
];
pattern!(
    BREAKS,
    r#"\b(?:broke|broken|broke\s+down|breaks|breaking|bust|busted)\b"#
);
pub(super) const NOT_A_THING: &[&str] = &[
    "body",
    "business",
    "career",
    "childhood",
    "confidence",
    "day",
    "faith",
    "family",
    "friendship",
    "heart",
    "home",
    "job",
    "life",
    "marriage",
    "mind",
    "mood",
    "relationship",
    "sleep",
    "soul",
    "spirit",
    "trust",
    "voice",
    "week",
    "work",
    "year",
];
pattern!(
    BODY,
    r#"(?x)
    \b(?:
        hurt(?:s|ing)? | ach(?:e|es|ing|y) | sore(?:ness)? | pain(?:s|ful)? | stiff(?:ness)?
      | swell(?:s|ing|ings|en)? | tender(?:ness)? | numb | itch(?:y|es|ing)? | throbbing
      | migraines? | headaches? | toothaches? | cough(?:ing)? | rashes?
      | cramp(?:s|ing)? | bleeding | bled | sprain(?:s|ed)? | bruis(?:e|es|ed)
      | stubbed | burnt | burned | infection
    )\b
"#
);
pattern!(
    FIGURATIVE,
    r#"\b(?:pain(?:s|ful)?|sore(?:ness)?|stiff(?:ness)?)\b"#
);
pub(super) const FLAT_SUBJECT: &[&str] = &[
    "appetite",
    "bubbliness",
    "buzz",
    "confidence",
    "creativity",
    "energy",
    "enthusiasm",
    "friendship",
    "hope",
    "hopes",
    "humor",
    "humour",
    "inspiration",
    "interest",
    "intimacy",
    "joy",
    "laugh",
    "laughter",
    "libido",
    "marriage",
    "mojo",
    "motivation",
    "passion",
    "patience",
    "romance",
    "spark",
    "trust",
    "zest",
];
pattern!(
    ASKS_TO_CHOOSE,
    r#"(?x)\b(?:should\s+i|shall\s+i|is\s+it\s+worth|is\s+it\s+better|
    would\s+it\s+be\s+better|which\s+(?:one\s+)?should\s+i|do\s+you\s+think\s+i\s+should|
    what\s+would\s+you\s+do|worth\s+(?:repairing|fixing|it)|new\s+one|replace\s+it|
    get\s+rid\s+of\s+it|throw\s+it\s+(?:out|away))\b"#
);
pattern!(KEEPS_AT_IT, r#"\b(?:keeps?|kept)\s+([a-z]{3,}ing)\b"#);
pub(super) const KEEPS_FINE: &[&str] = &[
    "arguing",
    "arriving",
    "asking",
    "baking",
    "bringing",
    "buying",
    "changing",
    "charging",
    "cleaning",
    "comes",
    "coming",
    "cooking",
    "counts",
    "crying",
    "does",
    "doing",
    "drinking",
    "eating",
    "feeling",
    "fighting",
    "gets",
    "getting",
    "gives",
    "giving",
    "goes",
    "going",
    "growing",
    "happening",
    "helping",
    "hoping",
    "improving",
    "lasts",
    "laughing",
    "learning",
    "listening",
    "lives",
    "looking",
    "makes",
    "making",
    "matters",
    "means",
    "moving",
    "needing",
    "noticing",
    "paying",
    "playing",
    "practicing",
    "practising",
    "reading",
    "running",
    "runs",
    "saving",
    "saying",
    "scrolling",
    "seems",
    "selling",
    "sending",
    "shouting",
    "sitting",
    "sleeping",
    "smiling",
    "sounds",
    "spending",
    "standing",
    "takes",
    "taking",
    "talking",
    "telling",
    "thinking",
    "tries",
    "trying",
    "visiting",
    "waiting",
    "walking",
    "wanting",
    "washing",
    "watching",
    "wearing",
    "wondering",
    "working",
    "works",
    "worrying",
    "writing",
];
pub(super) const KEEPS_THEIRS: &[&str] = &["my", "our", "the", "this"];
pub(super) const NOT_THE_THING: &[&str] = &[
    "all", "also", "always", "and", "any", "are", "but", "can", "cannot", "cant", "could", "did",
    "do", "does", "he", "how", "i", "if", "is", "it", "just", "may", "might", "much", "must",
    "never", "no", "not", "one", "only", "she", "should", "so", "still", "that", "then", "there",
    "they", "this", "to", "up", "was", "we", "were", "what", "when", "which", "who", "why", "will",
    "wont", "would", "you",
];
pub(super) const KEEPS_NOT_A_THING: &[&str] = &[
    "anyone",
    "barber",
    "beagle",
    "bird",
    "birds",
    "bloke",
    "boss",
    "boy",
    "budgie",
    "builder",
    "carer",
    "chihuahua",
    "cleaner",
    "client",
    "co-worker",
    "colleague",
    "colleagues",
    "collie",
    "coworker",
    "crew",
    "customer",
    "dentist",
    "doctor",
    "electrician",
    "everyone",
    "ex",
    "exes",
    "fella",
    "ferret",
    "fox",
    "foxes",
    "girl",
    "goldfish",
    "guinea",
    "guy",
    "hairdresser",
    "hamster",
    "horse",
    "kid",
    "kitten",
    "labrador",
    "lad",
    "landlord",
    "lass",
    "man",
    "manager",
    "mechanic",
    "mice",
    "mouse",
    "nobody",
    "nurse",
    "old",
    "optician",
    "parrot",
    "people",
    "person",
    "plumber",
    "pony",
    "poodle",
    "postie",
    "puppy",
    "rabbit",
    "rat",
    "rats",
    "receptionist",
    "retriever",
    "somebody",
    "someone",
    "spaniel",
    "squirrel",
    "squirrels",
    "staff",
    "supplier",
    "teacher",
    "team",
    "teen",
    "teenager",
    "terrier",
    "tortoise",
    "tutor",
    "vet",
    "vet's",
    "woman",
    "workmate",
    "year",
    "years",
];
pub(super) const KEEPS_NOT_THEIR_THING: &[&str] = &[
    "airline",
    "airport",
    "bank",
    "bar",
    "bus",
    "buses",
    "cafe",
    "café",
    "centre",
    "chemist",
    "chippy",
    "clinic",
    "council",
    "delivery",
    "garage",
    "gym",
    "hospital",
    "hotel",
    "library",
    "mall",
    "market",
    "nursery",
    "office",
    "pharmacy",
    "pool",
    "post",
    "pub",
    "pubs",
    "restaurant",
    "school",
    "shop",
    "shops",
    "station",
    "store",
    "stores",
    "supermarket",
    "surgery",
    "takeaway",
    "taxi",
    "train",
    "trains",
    "tram",
    "tube",
];
pub(super) const KEEPS_POSSESSIVE: &[&str] = &["my", "our"];
pub(super) const KEEPS_OF_A_HOUSE: &[&str] = &[
    "adaptor",
    "alarm",
    "appliance",
    "battery",
    "bed",
    "bell",
    "bicycle",
    "bike",
    "blind",
    "blinds",
    "board",
    "boiler",
    "box",
    "brake",
    "brakes",
    "broadband",
    "bulb",
    "buzzer",
    "cable",
    "car",
    "charger",
    "cistern",
    "clothes",
    "clutch",
    "computer",
    "connection",
    "console",
    "cooker",
    "curtain",
    "curtains",
    "damp",
    "decking",
    "detector",
    "dishwasher",
    "door",
    "doorbell",
    "doors",
    "drain",
    "drains",
    "drive",
    "dryer",
    "engine",
    "exhaust",
    "extractor",
    "fan",
    "fence",
    "flap",
    "freezer",
    "fridge",
    "fridge-freezer",
    "fuse",
    "garage",
    "gate",
    "gears",
    "gutter",
    "guttering",
    "handle",
    "headlight",
    "heater",
    "heating",
    "hinge",
    "hinges",
    "hob",
    "hoover",
    "hub",
    "immersion",
    "indicator",
    "intercom",
    "internet",
    "ipad",
    "kettle",
    "keyboard",
    "lamp",
    "landline",
    "laptop",
    "lawnmower",
    "lead",
    "light",
    "lights",
    "lock",
    "locks",
    "machine",
    "mattress",
    "meter",
    "microwave",
    "mobile",
    "monitor",
    "mouse",
    "mower",
    "oven",
    "panel",
    "pc",
    "phone",
    "pipe",
    "pipes",
    "plug",
    "printer",
    "pump",
    "radiator",
    "radio",
    "remote",
    "roof",
    "router",
    "scooter",
    "screen",
    "sensor",
    "shed",
    "shower",
    "shutter",
    "shutters",
    "signal",
    "sink",
    "smoke",
    "socket",
    "speaker",
    "strimmer",
    "switch",
    "system",
    "tablet",
    "tank",
    "tap",
    "taps",
    "television",
    "thermostat",
    "timer",
    "toaster",
    "toilet",
    "tumble",
    "tv",
    "tyre",
    "tyres",
    "unit",
    "vacuum",
    "valve",
    "van",
    "washer",
    "washer-dryer",
    "wheel",
    "wheels",
    "wifi",
    "window",
    "windows",
    "wiper",
    "wipers",
];
pub(super) const NOT_OWNED: &[&str] = &[
    "a", "an", "and", "as", "at", "but", "for", "from", "her", "his", "if", "in", "its", "like",
    "of", "on", "or", "than", "that", "the", "their", "then", "these", "this", "those", "to",
    "when", "with", "your",
];
pub(super) const STANDING_HEADS: &[&str] = &[
    "a few weeks of it is the point to say it out loud to a gp ra",
    "a mood that has only just started is not a verdict on anythi",
    "a rough patch like that is not a verdict. sleep, food and da",
    "carrying that for that long is tiring on its own, and it's f",
    "early days on its own is common enough. what would make it w",
    "months is a long time to carry a mood on your own. book a lo",
    "months is long enough that it deserves a proper look — ask f",
    "months is usually where people stop mentioning it and just g",
    "months of feeling like that is the point to ask for help: a ",
    "months of that is worth taking seriously rather than riding ",
    "that long is worth a longer appointment, and saying exactly ",
    "that long is worth asking for talking therapy by name — a gp",
    "that's short enough to wait on. if it's the same this time n",
    "that's still early days — keep a note of when it's worse and",
    "two or three weeks is long enough to stop waiting it out. sa",
    "weeks is where it stops being bad luck and starts being wort",
    "weeks of feeling like that is worth telling someone — a gp f",
    "weeks of it is the point where a gp is worth the trip, even ",
    "years of it is not something to keep managing alone. say how",
    "years of putting up with it is a lot. write the length down ",
];
pattern!(
    CAUGHT,
    r#"(?i)\b(?:cold(?!\s+(?:sore|sores|feet|hands|fingers|toes|air|weather|room|drink|water|days?|morning|night|outside|shower|call))|cough|coughing|flu|virus|temperature|head cold|sniffles?|sneez\w*)\b"#
);
pattern!(
    FRIGHTENED,
    r#"\b(?:scared|afraid|frightened|terrified|fear|fearful|panicking|panic|shaking|shaken|trembling|dreading|dread|shocked|in shock|crying|in tears)\b"#
);
