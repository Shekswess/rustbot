// Argument and follow-up patterns from the pinned PyBot chat module.
pattern!(
    ARGUMENT,
    r####"\b(?:i|we|me)\b[^.!?]{0,30}\b(?:argued|arguing|argue|rowed|had a row|falling out|(?:had|having|got into)\s+(?:an?\s+)?(?:(?:really|very|pretty|quite|totally|absolutely|such|kind of|sort of)\s+)?(?:(?:massive|big|huge|right|proper|real|serious|silly|stupid|terrible|awful|horrible|nasty|bitter|blazing|small|minor|tiny|daft)\s+)?(?:row|argument|fight|falling[- ]out)\b|fight|fighting|fought|had an argument|an argument|falling out|fell out|snapped at|shouted at|lost my temper|lost it with|not speaking|stopped talking|said something (?:i|that i) regret)\b|\b(?:me and|i and)\b[^.!?]{0,20}\b(?:fell out|had a row|argued)\b|\b(?:he|she|they)(?:s|'s| is| are|re|'re)?\s+(?:not|isn'?t|aren'?t|never)\s+(?:speaking|talking)\s+to me\b|\bwon'?t talk to me\b"####
);
pattern!(
    CLOSE_PERSON,
    r####"\b(?:mum|mom|mother|dad|father|sister|brother|wife|husband|partner|boyfriend|girlfriend|son|daughter|nan|gran|granny|grandma|grandad|grandpa|friend|mate|flatmate|housemate|aunt|uncle|cousin|stepdad|stepmum|parents|family|child|children|kid|kids)\b"####
);
pattern!(
    FEMININE,
    r####"\b(?:mum|mom|mother|sister|wife|girlfriend|daughter|nan|gran|granny|grandma|aunt|stepmum|she)\b"####
);
pattern!(
    MASCULINE,
    r####"\b(?:dad|father|brother|husband|boyfriend|son|grandad|grandpa|uncle|stepdad|he)\b"####
);
pattern!(
    ASK,
    r####"^\s*(?:what|whats|how|why|when|where|who|which|should|shall|can|could|do|does|did|is|are|will|would|any)\b|\b(?:what|how) (?:should|shall|can|could|do|does) (?:i|we)\b"####
);
pattern!(
    FALLING_OUT,
    r####"\b(?:fell out|fallen out|falling out|fall out|not talking|stopped talking|stopped speaking|made up|make up)\b"####
);
pattern!(
    DISTANT_PERSON,
    r####"\b(?:boss|manager|colleague|coworker|co-worker|workmate|landlord|letting agent|neighbour|neighbor|plumber|builder|electrician|decorator|doctor|nurse|teacher|tutor|lecturer|police|council|driver|shop|company|officer|agent|solicitor|bank|staff|gp|dentist|vet|postman|courier|stranger|client|customer)\b"####
);
pattern!(
    LOSS,
    r####"\b(?:died|dying|death|dead|passed away|funeral|burial|cremation|bereave\w*|lost my (?:mum|mom|dad|father|mother|wife|husband|son|daughter|brother|sister|partner|baby|child)|(?:mum|mom|dad|father|mother|wife|husband|son|daughter|brother|sister|partner|gran|nan|grandad)\s+(?:has|have|had)\s+(?:just )?(?:died|passed))\b"####
);
pattern!(
    EVENT_ID,
    r####"(?i)(?:^|-)(?:row|rows|argu\w*|fight\w*|fought|shout\w*|yell\w*|snap\w*|temper|sorry|apolog\w*|make-up|making-up)(?:-|$)"####
);
pattern!(WANTS_AN_APOLOGY, r####"\b(?:sorry|apolog\w*)\b"####);
pattern!(
    ASKS_FOR_WORDS,
    r####"\bwhat (?:should|do|can|could|would) i say\b|\bhow (?:do|should|can|could) i say\b|\bwhat to say\b|\bhow to say\b|\bhow do i word it\b|\bhow do i put it in words\b"####
);
pattern!(
    SUBJECT,
    r####"\b(?:about|over|because of|to do with|around)\s+(?:the |a |his |her |their |our |my )?([a-z']+)"####
);
pattern!(
    ASK_AFTER,
    r####"\b(?:should|shall|can|could|what|how|why|when|where|who|which|is|are|do|does|did|will|would|any)\b"####
);
pattern!(
    CONTACT,
    r####"\b(?:ring|call|text|message|phone|facetime|go round|pop round|go over|apologise|apologize|say sorry|make up|make it up|reach out|get in touch|speak to|talk to|leave (?:it|her|him|them)|give (?:her|him|them|it) (?:some )?(?:space|time)|wait|sit on it|sleep on it)\b"####
);
pattern!(
    CONTACT_ASK,
    r####"^\s*(?:should|shall|do you think|would you|is it|could)\b|\b(?:should|shall) (?:i|we)\b|\bdo you think (?:i|we)\b|\bis it (?:better|too soon)\b|\bor (?:not|wait|leave it)\b"####
);
pattern!(
    SPACE,
    r####"\bleave (?:it|her|him|them)\b|\bgive (?:her|him|them|it) (?:some )?(?:space|time)\b|\bwait\b|\bsit on it\b|\bsleep on it\b|\bnot (?:tonight|yet)\b"####
);
pattern!(
    SAID_SPACE,
    r####"\b(?:he|she|they|my (?:mum|mom|mother|dad|father|sister|brother|wife|husband|partner|boyfriend|girlfriend|friend|mate|flatmate|housemate))(?:'s|s| is|re|'re)?\s+(?:said|says|wants|needs|asked for|asked|thinks|reckons|told me)\b[^.!?]{0,30}\b(?:space|time|time to think|a few days|a break|breathing room|room)\b"####
);
pattern!(
    HOW,
    r####"^\s*how (?:do|can|should|would) (?:i|we)\s+(?:put (?:it|things) right|say sorry|apologi[sz]e|make (?:it )?up|fix (?:it|this|things)|sort (?:it|this) out|make (?:things|it) better|handle (?:it|this)|bring (?:it|things) up)\b|\bwhat (?:do|can|could|should|would) (?:i|we) (?:do|say)\b|\bany (?:advice|ideas)\b"####
);
pattern!(
    QUIET,
    r####"\b(?:wont|won't|doesnt|doesn't|never|stopped|not)\s+(?:talk|talks|talking|speak|speaks|speaking)\s+to\s+(?:me|us)\b"####
);
pattern!(
    CHILD,
    r####"\b(?:son|daughter|child|children|kid|kids|boy|girl|eldest|youngest|teen|teens|teenager|teenagers)\b"####
);
pattern!(
    TEEN,
    r####"\b(?:teen|teens|teenager|teenagers|teenage|teenaged|adolescent|adolescents|1[0-9])\b"####
);
pattern!(
    QUARREL,
    r####"\b(?:argued|arguing|argue|rowed|had a row|had an argument|an argument|fight|fighting|fought|falling out|fell out|fallen out|snapped at|shouted at|shouting|yelled|lost my temper|lost it with|said something (?:i|that i) regret|i apologised|i said sorry|i was wrong|my fault)\b"####
);
pattern!(
    BARE_MOVE,
    r####"(?i)^\s*(?:so\s+|and\s+|ok\s+|okay\s+)?what (?:do|can|could|should|would) (?:i|we) (?:do|say)\b[\s.?!]*$"####
);
pattern!(
    ENDING,
    r####"\b(?:hung up|put the phone down|slammed (?:the door|down the phone|the phone)|walked out|stormed off|blocked me|swore at me|shouted at me|went to bed|left the room|said nothing|slept (?:on the sofa|in the spare room)|(?:is|isnt|isn'?t|was|wasnt|wasn'?t) (?:not )?(?:talking|speaking) to me|hasn'?t spoken to me)\b"####
);
pattern!(
    HUNG_UP,
    r####"\b(?:hung up|put the phone down|slammed (?:the phone|down the phone)|blocked me|blocked my number)\b"####
);
const SUBJECT_PERSON: &[&str] = &[
    "anything",
    "boss",
    "boyfriend",
    "brother",
    "colleague",
    "dad",
    "daughter",
    "everything",
    "father",
    "flatmate",
    "friend",
    "girlfriend",
    "her",
    "him",
    "housemate",
    "husband",
    "it",
    "mam",
    "mate",
    "me",
    "mom",
    "mother",
    "mum",
    "neighbour",
    "nothing",
    "partner",
    "sister",
    "something",
    "son",
    "that",
    "them",
    "this",
    "us",
    "what",
    "wife",
    "you",
];
const OPENING: &[&str] = &[
    r####"Rows with the people we are closest to land hardest. Say the one thing you actually want {them} to hear and leave the rest of the case: being right is rarely the part that matters."####,
    r####"That kind of row is rarely about what it started as. If it is still raw, give it a night, then say your one true thing calmly."####,
    r####"Most fallings-out are two people each sure of their own version. Ask {them} how it sounded from {their} side of it - it is usually not what you meant."####,
    r####"Being at odds with someone close sits on you all day, so I would not leave it too long. One honest sentence does more than a long apology."####,
    r####"You do not have to fix it today. Say something true rather than nothing at all, and let the rest wait until you are both calmer."####,
];
const NEXT: &[&str] = &[
    r####"Yes, and keep it short: say the one thing you would want {them} to hear, not the case for why you were right. A message is easier than a call while it is still raw."####,
    r####"I would call rather than text, and let {them} hear your voice. One sentence about you - and if it goes to voicemail, that is not a no."####,
    r####"Say the thing you would be glad you said in a year, not the thing you have been rehearsing. And not after a drink, however much easier that feels."####,
    r####"Tonight is fine if you are both calmer than you were. Ring with one thing to say and let the rest wait: a row does not have to be settled in one call."####,
];
const HOW_TO: &[&str] = &[
    r####"Say the one thing you would want {them} to hear, and name your part in it rather than the case for it. One sentence does more than a long apology."####,
    r####"Keep it to what you did, not what {they} did: one specific thing, said plainly, then let {them} answer. A list of everything is a fresh row."####,
    r####"Do it in person or on the phone rather than in writing, and lead with the apology rather than the reason for it. The reason comes after, if it is wanted at all."####,
];
const PAUSE: &[&str] = &[
    r####"A couple of days does no harm if nobody is stewing on it - say so in one line first, so the silence is not read as the last word."####,
    r####"Space works better with a time on it: say you will leave it until the weekend, mean it, and then come back with the one thing you want {them} to hear."####,
    r####"A day or two is usually right. Much longer than that and it stops being space and starts being a way of not having the conversation."####,
];
const END: &[&str] = &[
    r####"That is the worst bit of it - being shut out leaves you with nothing to do with yourself. It usually means the night is needed, not that it is finished."####,
    r####"That sounds like it ended at the worst moment. Nothing gets better tonight by chasing it, so leave {them} be and say your one true thing when it is calmer."####,
    r####"A door, a silence or a walk out is a way of stopping the row, not a verdict on it. Let it settle, then say the one thing you actually wanted {them} to hear."####,
];
const END_HUNG_UP: &[&str] = &[
    r####"Hanging up is a way of stopping the row, not a verdict on it. Let it settle, then say the one thing you actually wanted {them} to hear."####,
];
