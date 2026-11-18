// Reference wording and recognition predicates from bot/compose/fallback.py.
pub(super) const NEGATIVE: &[&str] = &[
    "That sounds rough. If you want, I can help you think through what to do next.",
    "Ugh, that's a lot to carry. What would make today a bit easier?",
    "I'm sorry — that's frustrating. Want a hand working out the next step?",
    "Sorry to hear that. What would help most right now?",
];
pub(super) const NAMED_NEGATIVE: &[&str] = &[
    "That sounds rough. If you want, I can help you think through what to do next.",
    "Ugh, that's a lot to carry. What would make today a bit easier?",
    "Sorry to hear that. What would help most right now?",
];
pub(super) const SOMEONE_SERIOUSLY_ILL: &[&str] = &[
    "That's frightening news about someone close. How are you doing?",
    "That is the kind of news that puts everything else on hold. How is it going now?",
    "Worrying about someone you love is its own kind of tiring. What do you need first?",
    "That's a lot to have land at once. What is the hardest part of it today?",
    "I'm sorry. That is frightening, and it is a lot to carry. Who is looking after you in it?",
];
pub(super) const EMERGENCY_AFTER: &[&str] = &[
    "If you haven't already, ring 999 or 911 now, and follow what they tell you — they can hear what I can't.",
    "Do what the emergency line says over anything I could offer, and call back the moment something changes.",
    "Stay in touch with them and say if anything gets worse — don't wait to see if it settles.",
];
rule!(NOT_THEIR_NEWS, r###"(?i)\b(?:sponsor\w*|fundrais\w*|raising|raise money|awareness|charity|marathon|race for life|donat\w*|nurses?|nursing|(?:works?|working) (?:at|in|for)|shift)\b"###);
rule!(CLOSE_OWNER, r###"(?i)\b(?:my|our)\s+(?:(?:mum|mom|mother|mam|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|son|daughter|child|kid|baby|boy|girl|toddler|teenager|little one|wife|husband|partner|boyfriend|girlfriend|fiance|fiancee|brother|sister|uncle|aunt|(?:(?:\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|a|an)[\s-]+(?:month|months|week|weeks|year|years)[\s-]+old(?:[\s-]+(?:baby|boy|girl|son|daughter|child|kid|toddler|little one))?)))\b"###);
rule!(SERIOUS_NEWS, r###"(?i)\b(?:heart attack|cardiac|stroke|blood clot|aneurysm|cancer|tumou?rs?|leuka?emia|lymphoma|melanoma|chemo(?:therapy)?|radiotherapy|dialysis|transplant|bypass|sepsis|septica?emia|pneumonia|meningitis|intensive care|high dependency|life support|resuscitat\w+|operation|surgery|amputat\w+|admitted|in hospital|into hospital|hospital (?:last|this) (?:night|week|morning)|collapsed|collapse|seizures?|convulsions?|diagnosed|diagnosis|terminal|palliative|hospice)\b"###);
rule!(NOT_YET_NEWS, r###"(?i)\b(?:i think|i thought|might|could have|maybe|may have|suspect\w*|probably|possibly|what if|is it possible|worried (?:it|that|he|she|they)|terrified (?:it|that)|scared (?:it|that)|afraid (?:it|that)|waiting (?:on|for) (?:the )?results)\b"###);
rule!(ILL_RELATIVE, r###"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird|nephew|niece|stepson|stepdaughter|stepmum|stepdad|stepmother|stepfather|grandson|granddaughter|mother in law|father in law|son in law|daughter in law|grandchild|grandchildren)\b[^.!?]{0,24}\b(?:is|are|was|were|has been|have been|had been|got|getting|become|becoming|been)\b[^.!?]{0,16}\b(?:ill|poorly|unwell|not well|off (?:it|colour|their food)|in a bad way|taken ill|gone downhill|deteriorat\w+)\b"###);
