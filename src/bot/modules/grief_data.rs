// Bereavement patterns and ordinary follow-ups from pinned PyBot.
pattern!(
    OTHER_PERSON,
    r####"\bmy (?:baby|boy|girl|son|daughter|kid|child|toddler|teenager|husband|wife)\b|\b\d{1,2} year old\b"####
);
pattern!(
    EMERGENCY,
    r####"\b(?:wont wake|cant wake|can'?t wake|unresponsive|not breathing|ambulance|999|911|chest pain|a and e|a&e|temperature|floppy|seizure|overdose|suicidal|bleeding|collapsed)\b"####
);
pattern!(
    FAMILY,
    r####"\b(?:brother|sister|dad|mum|stepdad|stepmum|uncle|aunt|nan|grandad|family|relatives|in-laws|siblings)\b[^.]{0,40}?\b(?:argue|arguing|argument|row|rows|rowing|fight|fighting|falling out|fell out|not talking|won'?t talk)\b"####
);
pattern!(
    WORK_PRESSURE,
    r####"(?:(?=.*\b(?:work|job|boss|manager|office|shift|shifts|colleagues?|team|employer|hr)\b).*?\b(?:back to work|off work|cant face|can't face|cant go back|can't go back|dread|too much|(?:keep|keeps|are) asking|everyone (?:keeps )?asking)\b|\b(?:people|everyone|folks|the team) at work\b)"####
);
pattern!(
    WORK,
    r####"\b(?:bereavement leave|compassionate leave|leave from work|(?:ask|tell|ring|call|email) (?:my|the) (?:boss|manager|work|employer|hr)\b|\btell work\b|\bwhat do i tell work\b)"####
);
pattern!(
    TELL,
    r####"\b(?:who (?:do|should|must|would) (?:i|we) (?:have to |need to |ought to )?(?:tell|notify|inform|ring|contact)|who (?:needs? to know|needs? to be told|has to know|should (?:know|be told))|who to (?:tell|notify|inform)|(?:do|should) (?:i|we) (?:have to |need to )?(?:tell|notify|inform|ring) (?:everyone|everybody|anyone|people|the (?:council|bank|pension|tax office|dwp|hmrc|authorities|employer))|tell us once|who do i inform)\b"####
);
pattern!(
    TELLING,
    r####"\bhow do i (?:tell|say|break)\b|\bhow to tell\b|\b(?:do|should) i tell\b|\bwhat (?:do|should|can) i tell\b|\bhow do i let .{0,20} know\b"####
);
pattern!(
    SAY_TO,
    r####"\bwhat (?:do|should|can) i say\b|\bwhat to say\b"####
);
pattern!(
    REGISTER,
    r####"\b(?:register (?:the|her|his) death|registration|register office|registrar|death certificate|medical certificate)\b"####
);
pattern!(
    COST,
    r####"\b(?:cost|costs|price|prices|priced|how much (?:does|is|will|do|are)|expensive|afford|cheap\w*|quote|pay for)\b"####
);
pattern!(
    PROBATE,
    r####"\b(?:probate|grant of probate|letters of administration)\b"####
);
pattern!(
    PAPERWORK,
    r####"\b(?:solicitor|paperwork|(?:the|her|his|their) will|(?:her|his|their) (?:bank|pension|insurance|accounts)|bereavement|the estate)\b"####
);
pattern!(
    ADMIN,
    r####"\b(?:paperwork|the papers|(?:a|any|one|the|that) forms?|filling in|fill in|(?:his|her|their|the) (?:bank|banks|accounts?|pension|pensions|insurance|policies|policy|passport|driving licence|licence|mobile|phone contract|utilities|bills?|mortgage|tenancy|landlord|affairs)|council tax|(?:his|her|their) (?:benefits?|tax credits)|universal credit|hmrc|the dwp|the tax office|dvla|the council|(?:cancel|close|stop|sort out|sorted out|sorted|deal with|notify|inform|tell)\b[^.]{0,36}?\b(?:subscription\w*|direct debits?|accounts?|pension|insurance|policy|doctor'?s (?:surgery|practice)|dentist|the vet|employer|the council|the bank|the tax office|his phone|her phone|the phone|the house|the mortgage|the tenancy)\b)\b"####
);
pattern!(
    ARRANGEMENTS,
    r####"\b(?:funeral|burial|cremation|eulogy|coffin|undertaker|funeral director|headstone|graveside|cemetery|order of service)\b"####
);
pattern!(
    BELONGINGS,
    r####"\b(?:her|his|their|the)\s+(?:things|stuff|clothes|belongings|flat|house|room|wardrobe|shed)\b|\bpack(?:ing)? (?:it|them|her|his) up\b|\bclear(?:ing)? (?:out|it|them)\b|\bempty(?:ing)? (?:it|them|the)\b"####
);
pattern!(
    EXPECTED,
    r####"\b(?:it was expected|was expected|we knew (?:it|this) was coming|we saw it coming|long illness|not a surprise|no surprise|been ill for (?:a|years|months))\b"####
);
pattern!(
    SUDDEN,
    r####"\b(?:sudden|suddenly|out of the blue|no warning|shock|shocked)\b"####
);
pattern!(
    SLEEP,
    r####"\b(?:cant|can'?t|couldnt|couldn'?t|cannot|unable to|barely|hardly|not|never)\s+(?:sleep|sleeping|slept|stay asleep|get to sleep|get any sleep)\b|\b(?:havent|haven'?t|hasnt|hasn'?t|hadnt|hadn'?t|didnt|didn'?t)\s+(?:slept|been sleeping|had (?:any |much |much of any )?sleep)\b|\b(?:keep|kept|still)\s+waking\b|\bwaking (?:up )?(?:at|every|in the)\b|\bwoke (?:up )?at\b|\bawake (?:at|all|most of the)\b|\b(?:insomnia|sleepless|lying awake)\b|\b(?:3|4|5)\s?am\b"####
);
pattern!(
    HARD,
    r####"\b(?:it|that|yesterday|today|this morning|the day|last night)\s+(?:was|is|has been|'s been)\s+(?:so |really |very |pretty |just )?(?:hard|tough|awful|horrible|brutal|rough|difficult|shit|hell|the worst)\b"####
);
pattern!(
    TALK_ABOUT,
    r####"\b(?:cant|can't|couldnt|couldn't|could not|dont|don't|do not|not able to|not ready to|struggl\w* to|hard to|too (?:hard|painful|raw|much) to)\s+(?:really\s+|even\s+|ever\s+)?talk(?:ing)?\s+(?:about|of)\s+(?:it|him|her|them|that|this|any of it|any of that)\b"####
);
pattern!(
    TALK_TO_SOMEONE,
    r####"\b(?:talk|talking|speak|speaking)\s+(?:to|with)\s+(?:physiotherapist|granddaughters|stepdaughters|grandchildren|granddaughter|grandparents|childminders|goddaughters|psychiatrist|schoolfriend|stepdaughter|stepchildren|godchildren|babysitters|goddaughter|grandfather|childminder|halfbrother|hairdresser|stepbrother|grandmother|counsellors|girlfriends|classmates|caregivers|boyfriends|neighbours|babysitter|therapists|counselors|colleagues|girlfriend|counsellor|grandchild|housemates|stepsister|halfsister|grandkids|caregiver|godmother|relatives|neighbour|grandsons|godfather|greyhound|counselor|boyfriend|colleague|stepchild|housemate|classmate|flatmates|neighbors|therapist|daughter|goldfish|midwives|labrador|flatmate|stepsons|surgeons|siblings|managers|tortoise|godchild|children|relative|granddad|neighbor|guardian|teachers|cleaners|grandson|puppies|grandpa|stepmum|lodgers|kittens|friends|teacher|workers|spouses|doctors|surgeon|fiancee|cleaner|partner|grandad|stepson|sibling|stepdad|grandma|midwife|in-laws|hamster|godsons|parents|manager|brother|husband|fiancée|nannies|lurcher|fiances|chicken|nurses|kitten|budgie|spouse|fishes|sister|carers|friend|auntie|family|fiance|worker|mentor|parent|fiancé|mother|guinea|tutors|granny|nephew|pastor|collie|warden|doctor|father|cousin|rabbit|parrot|lizard|lodger|barber|ferret|priest|godson|gerbil|physio|snake|mouse|child|rabbi|nanny|uncle|twins|vicar|carer|tutor|horse|niece|nurse|mates|puppy|mate|fish|kids|cats|pets|imam|twin|wife|girl|dogs|nana|gran|baby|aunt|pony|mom|mam|boy|pet|dog|son|gps|fil|mum|dad|rat|nan|kid|cat|mil|hen|gp|ex|my|our|the|his|her|a|an)\b|\babout (?:it|that|this)\s+(?:with|to)\s+(?:physiotherapist|granddaughters|stepdaughters|grandchildren|granddaughter|grandparents|childminders|goddaughters|psychiatrist|schoolfriend|stepdaughter|stepchildren|godchildren|babysitters|goddaughter|grandfather|childminder|halfbrother|hairdresser|stepbrother|grandmother|counsellors|girlfriends|classmates|caregivers|boyfriends|neighbours|babysitter|therapists|counselors|colleagues|girlfriend|counsellor|grandchild|housemates|stepsister|halfsister|grandkids|caregiver|godmother|relatives|neighbour|grandsons|godfather|greyhound|counselor|boyfriend|colleague|stepchild|housemate|classmate|flatmates|neighbors|therapist|daughter|goldfish|midwives|labrador|flatmate|stepsons|surgeons|siblings|managers|tortoise|godchild|children|relative|granddad|neighbor|guardian|teachers|cleaners|grandson|puppies|grandpa|stepmum|lodgers|kittens|friends|teacher|workers|spouses|doctors|surgeon|fiancee|cleaner|partner|grandad|stepson|sibling|stepdad|grandma|midwife|in-laws|hamster|godsons|parents|manager|brother|husband|fiancée|nannies|lurcher|fiances|chicken|nurses|kitten|budgie|spouse|fishes|sister|carers|friend|auntie|family|fiance|worker|mentor|parent|fiancé|mother|guinea|tutors|granny|nephew|pastor|collie|warden|doctor|father|cousin|rabbit|parrot|lizard|lodger|barber|ferret|priest|godson|gerbil|physio|snake|mouse|child|rabbi|nanny|uncle|twins|vicar|carer|tutor|horse|niece|nurse|mates|puppy|mate|fish|kids|cats|pets|imam|twin|wife|girl|dogs|nana|gran|baby|aunt|pony|mom|mam|boy|pet|dog|son|gps|fil|mum|dad|rat|nan|kid|cat|mil|hen|gp|ex|my|our|the|his|her|a|an)\b"####
);
pattern!(
    GUILT,
    r####"\b(?:should have|should've|shouldnt have|blame myself|my fault|regret|if only|wish i (?:had|could|was there)|didn'?t visit|didn'?t go|guilty (?:about|that|for)|so guilty i|more time with)\b"####
);
pattern!(
    CRYING,
    r####"\b(?:cry|crying|cried|tears|sobbing)\b|\bi broke down\b"####
);
pattern!(
    AGE,
    r####"\b(?:he|she|it|they)\s+was\s+\d{1,2}\b(?!\s*(?:degrees|miles|kilos|kg|pounds|quid|minutes|hours|days|weeks|months|years|per cent|%))|\baged\s+\d{1,2}\b|\bin (?:her|his) (?:70s|80s|90s)\b"####
);
pattern!(
    AGE_NUMBER,
    r####"\b(?:he|she|it|they)\s+was\s+(\d{1,2})\b|\baged\s+(\d{1,2})\b|\bin (?:her|his) (\d)0s\b"####
);
pattern!(
    SIGNS,
    r####"(?:still|keep) (?:look|looking|listen|listening|wait|waiting|expect|expecting) for (?:her|him|them)|(?:still|keep) (?:expect|expecting) (?:her|him|them)|(?:still|keep) (?:talk|talking|say|saying|tell|telling)\b[^.]{0,20}?\b(?:her|him|them)\b|think(?:ing)? i (?:see|hear) (?:her|him)|half expect|hear (?:her|him) voice"####
);
pattern!(
    MISSING,
    r####"\b(?:miss (?:her|him|them)|i miss my (?:physiotherapist|granddaughters|stepdaughters|grandchildren|granddaughter|grandparents|childminders|goddaughters|psychiatrist|schoolfriend|stepdaughter|stepchildren|godchildren|babysitters|goddaughter|grandfather|childminder|halfbrother|hairdresser|stepbrother|grandmother|counsellors|girlfriends|classmates|caregivers|boyfriends|neighbours|babysitter|therapists|counselors|colleagues|girlfriend|counsellor|grandchild|housemates|stepsister|halfsister|grandkids|caregiver|godmother|relatives|neighbour|grandsons|godfather|greyhound|counselor|boyfriend|colleague|stepchild|housemate|classmate|flatmates|neighbors|therapist|daughter|goldfish|midwives|labrador|flatmate|stepsons|surgeons|siblings|managers|tortoise|godchild|children|relative|granddad|neighbor|guardian|teachers|cleaners|grandson|puppies|grandpa|stepmum|lodgers|kittens|friends|teacher|workers|spouses|doctors|surgeon|fiancee|cleaner|partner|grandad|stepson|sibling|stepdad|grandma|midwife|in-laws|hamster|godsons|parents|manager|brother|husband|fiancée|nannies|lurcher|fiances|chicken|nurses|kitten|budgie|spouse|fishes|sister|carers|friend|auntie|family|fiance|worker|mentor|parent|fiancé|mother|guinea|tutors|granny|nephew|pastor|collie|warden|doctor|father|cousin|rabbit|parrot|lizard|lodger|barber|ferret|priest|godson|gerbil|physio|snake|mouse|child|rabbi|nanny|uncle|twins|vicar|carer|tutor|horse|niece|nurse|mates|puppy|mate|fish|kids|cats|pets|imam|twin|wife|girl|dogs|nana|gran|baby|aunt|pony|mom|mam|boy|pet|dog|son|gps|fil|mum|dad|rat|nan|kid|cat|mil|hen|gp|ex)|think about (?:her|him|them)|(?:the|her|his|their) birthday|anniversary|dreamt? about (?:her|him)|photos? of (?:her|him|them))\b"####
);
pattern!(
    CANT_TALK,
    r####"\b(?:i|we)\s+(?:still\s+|just\s+|really\s+)?(?:cant|can't|couldnt|couldn't|cannot|won't|wont|dont|don't|do not|wouldnt|wouldn't|am not|not)\b[^.!?]{0,24}\b(?:talk|talking|speak|speaking|say|saying)\b[^.!?]{0,16}\b(?:about (?:it|that|this|him|her|them)|yet|now|today)\b|\b(?:its|it's|that's|thats)\s+(?:too\s+(?:soon|raw|early|fresh)|too hard)\b|\bnot ready to talk\b|\bcan we not talk about\b|\blets not talk about\b"####
);
pattern!(
    THANKS_LINE,
    r####"^(?:ok(?:ay)?[,.!]?\s*)?(?:(?:thanks|thank you|thankyou|ta)(?:\s+(?:for|so much|very much)\s*(?:listening|talking|that|the chat|your help)?)?|(?:im|i'm|i am)\s+(?:ok|okay|fine|alright)(?:\s+now)?|ok(?:ay)?)[.!, ]*$"####
);
const FAMILY_REPLIES: &[&str] = &[
    r####"That happens in nearly every family, and it's the grief talking as much as anything — everyone is at a different speed. Split the jobs so you're not doing the same one: you take the house, they take the paperwork."####,
    r####"It's worth naming it out loud, once, when nobody's tired: "we're going to snap at each other and that's fine, but let's not do it over the house". Then give each other room."####,
];
const EXPECTED_REPLIES: &[&str] = &[
    r####"Even when you know it's coming, it still lands hard — knowing and being ready are two different things. There's nothing wrong with being knocked sideways by it."####,
    r####"Expected doesn't make it easier, it just means you spent a long time watching it happen. That takes it out of you too, so rest when you can."####,
];
const SUDDEN_REPLIES: &[&str] = &[
    r####"Sudden is the hardest kind — nothing gets you ready for it. It comes in waves for weeks, so don't be surprised when a good hour turns into a bad one."####,
    r####"That kind of shock takes ages to settle, and there's no doing it faster. Go as slowly as you can, and let people do the small things for you."####,
];
const SLEEP_REPLIES: &[&str] = &[
    r####"Sleep is the first thing to go with this. Don't lie there fighting it: get up, warm drink, something dull, and go back when you're sleepy."####,
    r####"Hardly sleeping is the usual pattern after a loss, and it does ease. Keep the room cool and dark, get up rather than lie there, and catch up in the day if you can."####,
];
const HARD_REPLIES: &[&str] = &[
    r####"It is, and there's no getting round how hard it is. Do less today than you think you should, and let people bring you things."####,
    r####"It's allowed to be that hard. Nothing about the first days is normal, so don't measure yourself against anyone else getting through it."####,
];
const TALK_ABOUT_REPLIES: &[&str] = &[
    r####"You don't have to. Say as much or as little as you want, and I'm here for the parts that aren't words."####,
    r####"That's alright. Nothing has to be said here - tell me when you want to, and not before."####,
];
const GUILT_REPLIES: &[&str] = &[
    r####"That guilt is a very common part of this, and it is almost never deserved. If it helps, say out loud what you'd want {who} to say to you now."####,
    r####"Everyone finds something to blame themselves for afterwards. {Who} would not want you carrying that."####,
];
const CRYING_REPLIES: &[&str] = &[
    r####"Crying is the pressure valve, not a setback. Let it come, then get outside for ten minutes if you can."####,
    r####"That's normal, and better out than held in. It gets further apart, not gone."####,
];
const MISSING_REPLIES: &[&str] = &[
    r####"That's a big miss, and it doesn't fade quickly. The stories keep {who} close, so go on telling them."####,
    r####"The dates are the hard ones — the birthday, Christmas, the day itself. Doing one small thing for {who}, a photo out or a meal they liked, can take the edge off."####,
];
const CANT_TALK_REPLIES: &[&str] = &[
    r####"That's alright, you don't have to — not now, not with me. I'm here whenever it helps, and we can talk about anything else in the meantime."####,
    r####"Then we won't. Say as much or as little as you want, and we can leave {who} out of it for now."####,
    r####"Fair enough. Nothing to explain, and no rush: tell me about something ordinary instead if that's easier today."####,
];
const COMPANY_REPLIES: &[&str] = &[
    r####"I'm here. Say as much or as little as you like about {who}."####,
    r####"No need to be all right about it with me. If you'd rather talk about something else for a while, that's fine too."####,
];
const PET_YOUNG_REPLIES: &[&str] = &[
    r####"{Age} is no age for a {animal}. Nothing about it is fair, and it's alright to say so out loud."####,
    r####"{Who} should have had years more than that. It was far too soon, and there is no sense to be made of it."####,
];
const PET_OLD_REPLIES: &[&str] = &[
    r####"{Age} is a good age for a {animal}, and it still isn't enough. Be as sad as you need to be about {who}."####,
    r####"That's a long life for a {animal}. It doesn't make the house any quieter, and it isn't meant to."####,
];
const YOUNG_AGE_REPLIES: &[&str] = &[
    r####"That's far too young, and there is no sense to be made of it. You don't have to try tonight."####,
    r####"{Who} should have had years more than that. Nothing about it is fair, and it's alright to say so out loud."####,
];
const AGE_REPLIES: &[&str] = &[
    r####"That's a long time together. It's the routines that catch you out more than the big days: the same chair at the same time, nobody in it."####,
    r####"A good long life still leaves a big hole. Be as sad as you need to be about it."####,
];
const ANIMAL_SPAN: &[(&str, u8)] = &[
    ("chinchilla", 10),
    ("guinea pig", 6),
    ("cockatiel", 15),
    ("tortoise", 60),
    ("hamster", 2),
    ("kitten", 15),
    ("rabbit", 8),
    ("guinea", 6),
    ("budgie", 8),
    ("parrot", 40),
    ("ferret", 8),
    ("lizard", 10),
    ("gerbil", 3),
    ("puppy", 12),
    ("bunny", 8),
    ("horse", 25),
    ("snake", 15),
    ("pony", 25),
    ("fish", 5),
    ("dog", 12),
    ("cat", 15),
    ("rat", 2),
];
