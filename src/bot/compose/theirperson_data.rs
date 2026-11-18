//! Patterns and responses from bot/compose/theirperson.py.
pattern!(
    A_PERSON_OF_THEIRS,
    r###"(?i)\b(?:my|our)\s+(?:(?:best|closest|close|poor|old|little|big|lil|eldest|youngest|older|younger|grown[-\s]?up|step|new|other)\s+)*(?:close[\s-]?friend|stepdaughter|best[\s-]?friend|grandmother|grandfather|old[\s-]?friend|schoolmate|girlfriend|best[\s-]?mate|housemate|colleague|boyfriend|neighbour|flatmate|workmate|daughter|neighbor|children|brothers|partner|husband|fiancee|brother|sibling|stepson|stepmum|stepdad|grandma|grandad|grandpa|parents|sisters|friends|friend|fiance|mother|father|sister|granny|auntie|cousin|nephew|buddy|child|uncle|niece|mates|mate|boss|wife|baby|nana|gran|aunt|kids|pal|mum|mom|mam|dad|son|kid|nan|ex)\b"###
);
pattern!(
    A_CONDITION,
    r###"(?ix)
    ^\s*(?:
        (?:has|have|had|got)
      | ['\u2019]?s\s+got
      | (?:is|are|was|were)
      | (?:could|might|may|must)\s+be
      | (?:seems?|appears?)\s+(?:to\s+be)?
    )
    (?:\s+(?:really|very|probably|possibly|likely|also|now|actually|officially|just|
             only|maybe|perhaps|definitely|already|too))*\s*
    (?:a\s+bit\s+of\s+|a\s+touch\s+of\s+|an?\s+|mild\s+|suspected\s+)?
    (?P<condition>dyslexi\w*|dyspraxi\w*|dyscalculi\w*|adhd|add|autis\w*|asperger\w*|
      ocd|tourette\w*|non[\s-]?verbal|speech\s+delay|sensory\s+processing|deaf|
      hard\s+of\s+hearing|blind|partially\s+sighted|down'?s?\s+syndrome|
      learning\s+disabilit\w*|on\s+the\s+spectrum)\b
"###
);
pattern!(
    A_ROUGH_PATCH,
    r###"(?ix)
    ^\s*(?:
        (?:is|are|was|were|has|have|had)\b
      | s\s+been\b
      | been\b
    )
    # "really struggling", "still having a hard time", "been a bit down": the adverbs a person
    # puts in front of it, and "been" twice over ("has been really down").
    (?:\s+(?:really|very|so|just|still|properly|proper|completely|totally|quite|pretty|a\s+bit|
            well|dead|awful|genuinely|seriously|always|also|obviously|clearly|been|badly))*\s+
    (?:
        having\s+(?:a\s+|one\s+hell\s+of\s+a\s+|such\s+a\s+|the\s+worst\s+)?
          (?:(?:really|very|proper|properly|awful|bit\s+of\s+a|lot\s+of|terrible)\s+)?
          (?:hard|tough|rough|bad|rubbish|awful|terrible|difficult|stressful|horrible|lousy|
             miserable|shit|shitty|grim|dreadful)\s+
          (?:time|patch|spell|week|month|year|day|run)\b
      | struggling\b(?!\s+with\b)
      | finding\s+(?:it|things|everything|life|work|the\s+job)\s+
          (?:a\s+bit\s+|really\s+|very\s+|so\s+|quite\s+)?hard\b
      | going\s+through\s+(?:a\s+lot|a\s+(?:rough|hard|tough|bad)\s+(?:time|patch|spell)|hell|
                             it|something\s+bad|a\s+bad\s+patch)\b
      | in\s+a\s+bad\s+way\b
      | at\s+breaking\s+point\b
    )
"###
);
pattern!(
    A_NOT_COPING,
    r###"(?ix)
    ^\s*(?:(?:is|are|was|were)\s+not|isn'?t|aren'?t|wasn'?t|hasn'?t|havent|weren'?t|not|
            never)\b
    (?:\s+(?:really|very|so|just|still|been|at\s+the\s+moment|at\s+all))*\s+
    coping\b.*$
"###
);
pattern!(
    A_LOW,
    r###"(?ix)
    ^\s*(?:(?:is|are|was|were|has|have)\b|s\s+been\b|been\b)
    (?:\s+(?:really|very|so|a\s+bit|quite|pretty|proper|properly|just|always|still|been))*\s+
    (?:down|low)\b
    (?:\s+(?:at\s+the\s+moment|lately|these\s+days|just\s+now|right\s+now|again|too|still|today|
              this\s+week|since\s+[a-z0-9']+|about\s+[a-z0-9']+|in\s+(?:him|her)sel(?:f|ves)|
              and\s+[a-z0-9' ]+))?\s*[.!?]*\s*$
"###
);
pattern!(
    A_WONDERING,
    r###"(?i)^\s*(?:could|might|may|must)\s+(?:be|have)\b|\bmaybe\b"###
);
pattern!(
    STANDS_DOWN,
    r###"(?ix)
    \b(?:
        hip|knee|knees|back|shoulder|arm|arms|leg|legs|foot|feet|ankle|wrist|hand|hands|head|
        headache|migraine|stomach|belly|chest|throat|tooth|teeth|ear|ears|eye|eyes|skin|joint|
        joints|spine|neck|heart|lung|lungs|blood|pain|pains|hurt|hurts|hurting|ache|aches|aching|
        ill|sick|sickness|unwell|hospital|doctor|gp|surgery|operation|chemo|cancer|tumour|tumor|
        diagnos\w*|scan|scans|medication|tablets|pills|dose|prescription|depress\w*|anxiety|
        panic|suicid\w*|self[-\s]?harm|addiction|drinking|drunk|sober|withdraw\w*|
        died|dying|die|death|dead|funeral|bereave\w*|grief|grieving|passed\s+away|
        tests|exam|exams|redundancy|divorce|money|rent|bills
    )\b
    | \? | \b(?:what|how|should|advice|tips|help|why|when|where|who)\b
"###
);
pub(super) const A_CONDITION_STATED: &[&str] = &[
    "Thanks for telling me. Is there something about your {who} you're trying to work out?",
    "Good to know - I'll remember that about your {who}. Anything you're trying to sort out?",
    "Thanks. What's the part you're working on with your {who} just now?",
];
pub(super) const A_CONDITION_WONDERED: &[&str] = &[
    "That's the hard part, the waiting. What have you noticed so far?",
    "It's rough sitting with a maybe like that. Write down what you see and when - that is what anyone you ask will want first. What made you start wondering?",
    "Not knowing yet is the worst of it. What has made you wonder?",
];
pub(super) const POOL: &[&str] = &[
    "That's hard, watching someone you're close to go through it. What's been happening?",
    "Sorry to hear it. Being the one who notices is its own weight - how are they doing in themselves?",
    "That's a worry to be carrying for someone else. What's going on at the moment?",
    "Hard when you can see it and can't fix it. Do they know you've noticed?",
    "That's a lot for them - and for you to watch. What would help most: someone to listen, or something practical?",
    "I'm sorry. It's rough being the one at the side of it. What's the hardest part for you just now?",
];
pub(super) const NAMED: &[&str] = &[
    "That's hard, watching your {who} go through it. What's been happening?",
    "Sorry to hear it. How is your {who} in themselves?",
    "That's a lot for your {who} to carry, and for you to watch. What would help most?",
];
pub(super) const FILLER: &[&str] = &[
    "a", "about", "also", "am", "an", "and", "are", "as", "at", "be", "been", "being", "but", "by",
    "did", "do", "does", "done", "for", "from", "get", "gets", "got", "had", "has", "have", "here",
    "how", "i", "i'd", "i'm", "i've", "id", "if", "im", "in", "is", "it", "it's", "its", "ive",
    "just", "me", "my", "no", "not", "of", "ok", "okay", "on", "or", "our", "quite", "really",
    "right", "said", "say", "says", "so", "than", "that", "thats", "the", "then", "there", "these",
    "think", "this", "those", "to", "too", "us", "very", "was", "we", "well", "were", "what",
    "whats", "when", "who", "why", "with", "yeah", "yes", "you", "your",
];
