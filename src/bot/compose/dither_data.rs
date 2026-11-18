//! Decision markers and replies from the pinned composer.

pattern!(
    FACT_BETWEEN,
    r####"\b(?:difference|differ|gap|distance|space|time|line|border|connection|link)\s+(?:\w+\s+)?between\b"####
);
pattern!(
    PERSON_NOT_A_DECISION,
    r####"\bwhat to do (?:about|with) (?:my|his|her|our|their|the)\s+(?:husband|wife|partner|boyfriend|girlfriend|brother|sister|son|daughter|child|kid|boy|girl|baby|mum|mom|mother|dad|father|nan|gran|grandma|grandad|grandpa|friend|boss|colleague|neighbour|neighbor|in laws|inlaws|teenager|family|teacher|carer|relative|cousin|uncle|aunt|nephew|niece|ex|stepson|stepdaughter)\b"####
);
pattern!(
    PRONOUN_NOT_A_DECISION,
    r####"\bwhat to do (?:about|with) (?:it|this|that|him|her|them|us|me|everything|anything|nothing)\b"####
);
pattern!(
    HELP_SERVICE,
    r####"\b(?:go|going|went|get|getting|take|taking|took|bring|bringing|drive|driving|rush|rushing|walk|walking)\b(?:\s+\w+){0,3}?\s+(?:to\s+|into\s+)(?:a and e|a&e|999|111|casualty|the hospital|hospital|the doctor|a doctor|doctor|the doctors|doctors|the gp|gp|my gp|the surgery|the vets|vets|the vet|vet|walk in centre|urgent care|out of hours|the pharmacy)\b|\b(?:call|called|calling|ring|ringing|rang|phone|phoned|phoning|see|seeing|saw|book|booking|booked)\b\s+(?:999|111|an ambulance|the doctor|a doctor|the gp|the surgery|the vets|the vet|a vet|urgent care)\b"####
);
context_pattern!(
    DITHER,
    r####"\b(?:not sure|unsure|dont know|do not know|no idea|cant decide|cannot decide|cant make up my mind|cannot make up my mind|torn between|(?<!not )torn about|(?<!not )torn over|(?<!not )torn on|(?<!not )torn|conflicted|dithering|undecided|in two minds|two minds about|of two minds|going back and forth|keep changing my mind)\b"####
);
context_pattern!(
    DITHER_STRONG,
    r####"\b(?:(?<!not )torn between|(?:i|im|i m|i am|i was|i ve been|ive been|i feel|i felt|feel|feeling|we|we re|we are|we were|he|he s|he is|he was|she|she s|she is|she was|they|they re|theyre|they are|they were|you|you re|youre|hes|shes|you are|you were|seems|seemed|sounds|sounded|felt)\s+(?:so\s+|really\s+|very\s+|quite\s+|a bit\s+|still\s+)?torn\b(?! apart)|conflicted|dithering|undecided|two minds|going back and forth|cant decide|cannot decide|cant make up my mind|cannot make up my mind|keep changing my mind)\b"####
);
context_pattern!(
    DITHER_SUBJECT,
    r####"\b(?:whether to (?!laugh|cry|scream|shout|smile)\w+|if i (?:should|shall) \w+|if i can (?:afford|manage|keep|take|handle|fit|cope|swing)\b|between \w+|which \w+ to (?:pick|choose|take|try|go)|what to do (?:about|with|next|now)\b|(?:decid(?:e|ed|ing)|think(?:ing)?|thought|wonder(?:ing)?|consider(?:ing)?|want)s?\s+(?:about\s+|of\s+)?(?:whether\s+)?to (?:go|take|move|stay|leave|quit|apply|accept|buy|sell)\b|i (?:want|wanted|dont want) to (?:go|take|move|stay|leave|quit|apply|accept|buy|sell)\b)"####
);
pattern!(
    DITHER_TWO_WAY,
    r####"\b(?:whether|if|between|or|which)\b"####
);
pattern!(
    LIFE_DIRECTION,
    r####"\b(?:with|out of|from) my life\b|\bwhat i (?:actually )?want to do\b|\b(?:twenties|thirties|forties|20s|30s|40s)\b|\bnothing (?:feels|seems) right\b|\bwhat (?:am i|im) (?:even )?doing with my life\b"####
);
pattern!(
    A_STRETCH_OF_TIME,
    r####"\bwhat (?:to|should i|can i|could i|shall i) do (?:with|on|for)\s+(?:my|the|this|these)\s+(?:afternoon|evening|morning|day off|day|weekend|week|time|night|days off|holidays?)\b"####
);
pattern!(DITHER_YES_NO, r####"\b(?:whether|if)\b"####);
pattern!(
    DITHER_OPEN,
    r####"\bwhat to do\b|\bwhat (?:i|you) (?:should|can|ought to) do\b|\bwhat next\b"####
);
pattern!(
    HEALTH_MODIFIER,
    r####"\b(?:hospital|doctors?|surgery|clinic|vets?)\s+(?:job|jobs|shift|shifts|appointment|appointments|letter|letter|bill|parking|car park|contract|rota|placement|course|uniform|policy|department|ward)\b"####
);

pub(super) const UNKNOWN_HEALTH: &[&str] = &[
    "Not one I can call for you, sorry: I cannot examine anything from here, and this is the kind of question where a guess does real harm. A GP, a pharmacist or 111 will tell you in a few minutes, and if it is getting worse, do not wait for an appointment.",
    "I cannot weigh that one safely, sorry. Ring 111 or your surgery and describe it as you did to me - they will say whether it needs looking at today, and it costs one phone call.",
];

pub(super) const DITHER_DECIDE: &[&str] = &[
    "What's pulling you each way? Even a rough version will do, and I'll lay both out with you.",
    "Which way are you leaning when nobody's watching? Start there, then tell me the bit that worries you.",
    "Say both versions out loud, the one where you do it and the one where you don't, and I'll help you find the one you'd regret less.",
    "What would have to be true for the answer to be yes? That is usually where the snag is.",
    "What's the thing you would lose if you said yes? Name it and we can look at it properly.",
];

pub(super) const DITHER_OPEN_LINES: &[&str] = &[
    "What's the part of it that won't move? Start there and the rest tends to come loose.",
    "Say what you'd want if nothing was in the way, and we can work back to the first step.",
    "Which bit of it is heaviest? Take that one first rather than the whole thing at once.",
];

pub(super) const DITHER_TWO_WAY_LINES: &[&str] = &[
    "What's pulling you each way? Even a rough version will do, and I'll lay both out with you.",
    "Which way are you leaning when nobody's watching? Start there, then tell me the bit that worries you.",
    "Say both versions out loud, the one where you do it and the one where you don't, and I'll help you find the one you'd regret less.",
];

pub(super) const CHECK_PHRASES: &[&str] = &[
    "get it checked",
    "get checked",
    "get it looked at",
    "get it seen to",
    "see someone about it",
    "get it seen",
];

pub(super) const HEALTH_WORDS: &[&str] = &[
    "111",
    "999",
    "a&e",
    "allergic",
    "allergy",
    "antibiotic",
    "antibiotics",
    "antihistamine",
    "antihistamines",
    "aspirin",
    "casualty",
    "clinic",
    "doctor",
    "doctors",
    "dosage",
    "dose",
    "flu",
    "gp",
    "gps",
    "hospital",
    "ibuprofen",
    "infection",
    "inhaler",
    "insulin",
    "jab",
    "jabs",
    "medication",
    "medicine",
    "nurse",
    "ointment",
    "pain",
    "painful",
    "painkiller",
    "painkillers",
    "paracetamol",
    "pharmacist",
    "pill",
    "pills",
    "prescription",
    "reaction",
    "steroid",
    "steroids",
    "surgery",
    "swollen",
    "symptom",
    "symptoms",
    "syringed",
    "tablet",
    "tablets",
    "unwell",
    "vaccination",
    "vaccine",
    "vaccines",
    "vitamin",
    "vitamins",
];
