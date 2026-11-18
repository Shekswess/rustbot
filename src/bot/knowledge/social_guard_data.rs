//! Conditions and replies from the pinned friends, habits and wording modules (Apache-2.0).

pub(super) const PERSON_OR_TALK: &[&str] = &[
    "my husband",
    "my wife",
    "my partner",
    "my boyfriend",
    "my girlfriend",
    "my fiance",
    "my friend",
    "my best friend",
    "my mate",
    "my mum",
    "my mom",
    "my mother",
    "my dad",
    "my father",
    "my parents",
    "my sister",
    "my brother",
    "my son",
    "my daughter",
    "my child",
    "my teenager",
    "my flatmate",
    "my housemate",
    "my roommate",
    "my landlord",
    "my boss",
    "my manager",
    "my colleague",
    "my coworker",
    "my team",
    "my neighbour",
    "my neighbor",
    "my mother in law",
    "my mother-in-law",
    "my in laws",
    "my ex",
    "my family",
    "a friend",
    "a colleague",
    "a coworker",
    "a flatmate",
    "a neighbour",
    "a family member",
    "someone at work",
    "with her",
    "with him",
    "with them",
    "to her",
    "to him",
    "to them",
    "tell her",
    "tell him",
    "tell them",
    "ask her",
    "ask him",
    "ask them",
    "conversation",
    "bring it up",
    "bring something up",
    "bring this up",
    "have it out",
    "open up",
    "my feelings",
    "about my feelings",
    "how i feel",
    "how i really feel",
    "confront",
    "confronting",
    "confrontation",
    "the subject",
    "broach",
    "hard talk",
];

pub(super) const SOMEBODY_AT_HOME: &[&str] = &[
    "my mum",
    "my mom",
    "my mother",
    "my dad",
    "my father",
    "my husband",
    "my wife",
    "my partner",
    "my girlfriend",
    "my boyfriend",
    "my stepson",
    "my stepdaughter",
    "my flatmate",
    "my housemate",
    "my room mate",
    "my roommate",
    "my friend",
    "my mate",
    "my brother",
    "my sister",
    "my nan",
    "my nana",
    "my gran",
    "my granny",
    "my grandma",
    "my grandad",
    "my grandpa",
    "my uncle",
    "my aunt",
    "my auntie",
    "my cousin",
    "my boss",
    "my colleague",
    "my staff",
];

pub(super) const A_RELATIVE: &str = concat!(
    r#"\b(?:my|his|her|their|our)\s+(?:mum|mom|mother|dad|father|parents|gran|granny|nan|nana|grandma"#,
    r#"|grandmother|grandad|grandpa|grandfather|grandparents?|wife|husband|partner|spouse|other half|son"#,
    r#"|daughter|brother|sister|aunt|auntie|uncle|cousin|boyfriend|girlfriend|friend|mate|neighbour|neighbor"#,
    r#"|mother in law|father in law)\b"#,
);

pub(super) const SOMEBODY_ELSE: &str = concat!(
    r#"\b(?:he|she|they)\s+(?:keeps?\s+forgetting|forgets|is\s+forgetful|cant\s+remember|can't\s+remember"#,
    r#"|cant\s+recall|can't\s+recall|repeats|is\s+confused)"#,
);

pub(super) const SOMETHING_FORGOTTEN: &str = concat!(
    r#"\b(?:forgett?ing|forgets|forgetful|remember|remembers|recall|repeat(?:s|ing)?|confused|gets lost|losing"#,
    r#"|loses)\b"#,
);

pub(super) const MY_OWN_SCREENS: &str =
    r#"\b(?:my|the)\s+(?:phone|phones)\b|\bim\s+(?:always\s+)?on\b|\bi'm\s+(?:always\s+)?on\b"#;
pub(super) const THEIR_SCREENS: &str =
    r#"\b(?:his|her|their)\s+(?:phone|phones|screens?|tablet|ipad|ipads)\b"#;
pub(super) const SOMEONE_HURT_A_PART: &str = concat!(
    r#"(?i)\bhurt\w*\s+(?:[a-z-]+\s+){0,2}(?:his|her|their|its)\s+(?:[a-z-]+\s+){0,2}(?:back|arm|arms|leg|legs"#,
    r#"|knee|knees|ankle|ankles|shoulder|shoulders|wrist|wrists|hand|hands|finger|fingers|thumb|toe|toes|foot"#,
    r#"|feet|hip|hips|head|neck|ribs?|rib|shin|shins|calf|elbow|elbows|eye|jaw|collarbone|collarbones|spine)\b"#,
    r#"|\bhurt\w*\s+(?:my|his|her|their|your)\s+(?:back|arm|arms|leg|legs|knee|knees|ankle|ankles|shoulder"#,
    r#"|shoulders|wrist|wrists|hand|hands|finger|fingers|thumb|toe|toes|foot|feet|hip|hips|head|neck|ribs?|rib"#,
    r#"|shin|shins|calf|elbow|elbows|eye|jaw|collarbone|collarbones|spine)\b|\bhurt\w*\s+(?:himself|herself"#,
    r#"|themselves|myself)\b"#,
);

pub(super) const RAISE_CAT_OUTSIDE: &str = r#"(?i)\b(?:cat|cats|kitten|kittens)\b(?=[^.]*\bgarden\b)|\bgarden\b(?=[^.]*\b(?:cat|cats|kitten|kittens)\b)"#;
pub(super) const RAISE_CAT: &str = r#"(?i)\b(?:cat|cats|kitten|kittens)\b"#;
pub(super) const RAISE_ANIMAL: &str =
    r#"(?i)\b(?:dog|dogs|puppy|pets?|animal|horse|rabbit|parrot)\b"#;
pub(super) const RAISE_CHILD: &str =
    r#"(?i)\b(?:child|children|kid|kids|son|daughter|baby|toddler|niece|nephew|boys|girls)\b"#;
pub(super) const RAISE_SMOKE: &str = r#"(?i)\bsmok\w*|\bcigarette\w*|\bvap\w*|\bsmoking\b"#;
pub(super) const RAISE_HOUSE: &str =
    r#"(?i)\b(?:noise|noisy|music|loud|party|parking|garden|hedge|fence|bin|bins|rubbish|alarm)\b"#;
pub(super) const RAISE_MONEY: &str = r#"(?i)\bmoney\b|\blent\b|\blend\w*\b|\bborrow\w*\b|\bowe\w*\b|\bdebt\w*\b|\bpay\w*\b|\bquid\b|£"#;
pub(super) const TALKS_OVER: &str = concat!(
    r#"(?i)\b(?:talks?|talking|talk) over (?:me|my)\b|\bkeeps? (?:talking|interrupting|cutting)\b"#,
    r#"|\b(?:interrupts?|interrupting) (?:me|my)\b|\bcuts? me off\b|\bkeeps? cutting me off\b|\b(?:butts?"#,
    r#"|butting) in\b|\bfinishes? my sentences?\b|\btalks? over people\b"#,
);

pub(super) const MOVING_OUT: &str = r#"(?i)\bmoving out\b|\bmoving (?:house|flat|home)\b|\bleaving home\b|\bmoved out\b|\bmoving in with\b"#;
pub(super) const MOVING_AWAY: &str = concat!(
    r#"(?i)\bmoving away\b|\bemigrat\w*\b|\bgoing travelling\b|\bmoving abroad\b|\bmoving to (?:australia|canada"#,
    r#"|spain|france|new zealand|america|the us|germany|ireland|scotland)"#,
);

pub(super) const BABY: &str =
    r#"(?i)\bpregnan\w*\b|\bexpecting a baby\b|\bbaby on the way\b|\bhaving a baby\b"#;
pub(super) const MESS: &str = concat!(
    r#"(?i)\bclean\w*\b|\bwashing up\b|\bwashing-up\b|\bdishes\b|\bmess\b|\btidy\w*\b|\bbins?\b|\bhoover\w*\b"#,
    r#"|\bload of washing\b"#,
);

pub(super) const SHARED_HOUSE: &str = r#"(?i)\b(?:housemate|flatmate|roommate|house ?mate)s?\b"#;
pub(super) const MONEY: &str =
    r#"(?i)\bmoney\b|\bfinances?\b|\bdebts?\b|\bsalary\b|\bbills?\b|\bwages?\b"#;
pub(super) const SHARE_MONEY: &str =
    r#"(?i)\bpay\w*\b|\brent\b|\bbills?\b|\bshare\b|\bowe\w*\b|\bsplit\b|\bcouncil tax\b"#;
pub(super) const PARENT_MONEY: &str =
    r#"(?i)\b(?:my )?(?:parents?|mum|mom|mam|mother|dad|father|family|in ?laws?)\b"#;
pub(super) const HOUSEHOLD_MONEY: &str = concat!(
    r#"(?i)\b(?:we|our|us|partner|husband|wife|spouse|bills?|rent|split|share|sharing|housemates?|flatmates?"#,
    r#"|roommates?|household|money with|finances)\b"#,
);

pub(super) const SAID_IT_IS_NEWS: &str =
    r#"(?i)\bnews\b|\bannounc\w*\b|\bbreak\s+(?:it|this|the)\b|\bbreak\s+bad\b"#;
pub(super) const NO_FUSS: &str = concat!(
    r#"\b(hates?|hated|doesnt|does not|doesn't|dont|don't|not|no|never|isnt|isn't)\b[^.?!]{0,30}\b(fuss"#,
    r#"|attention|centre of attention|center of attention|spotlight|big deal|surprise|surprises|party|to do"#,
    r#"|big do|a do|night out|speech|faff)\b|\b(quiet|low key|low-key|private|small|low"#,
    r#"|no fuss)\b[^.?!]{0,16}\b(birthday|do|party|meal|dinner|card|thing)\b"#,
);

pub(super) const A_COLLEAGUE: &str = concat!(
    r#"\b(?:colleagues?|co-?workers?|workmates?|boss|manager|staff|team|teacher|neighbou?r|acquaintance"#,
    r#"|someone at work|people at work|the office)\b"#,
);

pub(super) const A_CLOSE_ONE: &str = concat!(
    r#"\b(?:nan|nana|gran|granny|grandma|grandmother|grandad|grandpa|grandfather|mum|mom|mam|mother|dad|father"#,
    r#"|sister|brother|aunt|auntie|uncle|cousin|wife|husband|partner|girlfriend|boyfriend|friends?|mates?"#,
    r#"|best mate|son|daughter|grandson|granddaughter)\b"#,
);

pub(super) const ASKS_FOR_THE_WORDS: &str = concat!(
    r#"\b(?:what|how)\b[^.?!]{0,14}?\b(?:do|should|shall|can|could|would|will|might|to"#,
    r#"|am i)\b[^.?!]{0,10}?\b(?:write|writing|say|saying|put|word|phrase|sign off)\b[^.?!]{0,24}?\b(?:card"#,
    r#"|cards|note|notes|message|messages|letter|letters|inside|envelope)\b|\b(?:card|cards|note|notes|message"#,
    r#"|messages|letter|letters)\b[^.?!]{0,24}?\b(?:what|how)\b[^.?!]{0,12}?\b(?:write|say|put|word|phrase)\b"#,
);

pub(super) const RAISE_EXAMPLES: &[&str] = &[
    "\"your cat keeps using my veg bed - could you keep her in at night?\"",
    concat!(
        "\"Mog swiped at Nell again on Saturday - could you keep her out of the room while we ",
        "are there?\"",
    ),
    concat!(
        "\"Rex jumped up at Nell again on Saturday - could you keep him in the kitchen while she",
        " is here?\"",
    ),
    concat!(
        "\"the boys were a bit rough with Nell on Saturday - could you keep an eye when they are",
        " in the garden?\"",
    ),
    "\"the smoke comes through the wall - could you smoke by the back door?\"",
    "\"the music was still going at half one - could we call it midnight next time?\"",
    "\"I lent you £200 and I have been unsure how to bring it up\"",
];

pub(super) const RAISE_PLAIN: &str = "\"I felt left out when the plans changed\"";

pub(super) const TELL_TALKS_OVER: &str = concat!(
    "Say it once, plainly, and not in the moment it is happening: \"Can I finish this one? ",
    "You jump in and I lose the thread.\" Name what they do rather than what they are - one ",
    "line, no list of past times - and give them the benefit of the doubt once, because ",
    "most people who talk over you are excited or nervous rather than rude. Then keep a ",
    "short version for the moment itself, a hand up and \"hang on, let me finish this bit\", ",
    "since that is the only place the habit can change. If it carries on after you have ",
    "said it plainly, say it plainly a second time and say how it lands; that is not ",
    "nagging, it is the second half of the same sentence.",
);

pub(super) const TELL_MOVING_OUT: &str = concat!(
    "Give them the plan, not a request for permission: \"I've found a flat and I'm moving ",
    "out on the 14th - I wanted you to hear it from me first.\" Then let them react without ",
    "defending it, and come back to the practical bits - how often you will visit, what you",
    " need - once the first conversation has cooled.",
);

pub(super) const TELL_MOVING_AWAY: &str = concat!(
    "Tell the people you will miss before the ones who will hear it second hand, and give ",
    "them a date: \"I'm moving to Bristol in March - I didn't want you to find out from ",
    "someone else.\" Then say what you want to keep going, a call every couple of weeks or a",
    " visit in the summer, so the news comes with a plan rather than a goodbye.",
);

pub(super) const TELL_BABY: &str = concat!(
    "Tell them in person or on a call rather than in a message, and lead with the news: ",
    "\"I'm pregnant - it's early, and I wanted you to know.\" Then say what you want from ",
    "them now, help or time before anyone else is told; that one sentence saves the ",
    "guessing.",
);

pub(super) const TELL_HOUSEHOLD_START: &str = concat!(
    "Ask for the one thing, name when it happens and say what you want instead, once and in",
    " private: ",
);

pub(super) const TELL_HOUSEHOLD_END: &str = concat!(
    " Not the whole list and not in the moment it is annoying you, and leave them room to ",
    "say why it is hard; a habit that is asked for plainly usually bends a little, where a ",
    "rule handed down does not.",
);

pub(super) const TELL_MESS: &str = concat!(
    "Ask for the one job rather than the whole mess, and say when: \"Could you do the ",
    "washing up tonight? I did it the last four nights and it is stacking up.\" Name the ",
    "thing, the reason and the deadline in one breath - a general complaint about the flat ",
    "gets an apology and no change.",
);

pub(super) const TELL_SHARE_MONEY: &str = concat!(
    "Ask once, with the amount and the date, and leave the history out of it: \"The ",
    "broadband comes to 32 a month and your half is 16 - could you send it by Friday?\" A ",
    "plain ask reads as fair; hinting over the washing up reads as a buildup.",
);

pub(super) const TELL_JOINT_MONEY: &str = concat!(
    "Pick a time that is not the moment a bill lands, and put the numbers side by side ",
    "rather than asking for a verdict: what comes in, what goes out, and what each of you ",
    "wants to be able to afford. Open with a question - \"how do you feel about the way we ",
    "split things?\" - and leave one decision for next time.",
);

pub(super) const TELL_NEWS: &str = concat!(
    "Say it in one flat sentence, then stop: the news, the reason if there is one, and what",
    " you want from them. The wait for the perfect moment is usually longer and worse than ",
    "the conversation itself.",
);

pub(super) const PLAIN_CARD_QUIET: &str = concat!(
    "Keep it plain and quiet, because that is the card this person actually wants: \"Happy ",
    "birthday - no fuss, I'm just glad you're in my life.\" One true thing about them, no ",
    "joke and no speech, and give the present quietly rather than in front of everyone.",
);

pub(super) const PLAIN_CARD: &str = concat!(
    "Name one real thing about them and one wish, then stop - two lines is a card: ",
    "\"Thinking of you both - hope the year is a good one.\" If you do not know them well, ",
    "warm and plain beats clever, and sign it with your full name.",
);

pub(super) const BIRTHDAY_QUIET: &str = concat!(
    "Keep it plain and quiet, because that is the card a person who hates a fuss actually ",
    "wants: \"Happy %s - no fuss, I'm just glad you're in my life.\" One true thing about ",
    "them, no joke and no speech, and give the present quietly rather than in front of ",
    "everyone.",
);

pub(super) const BIRTHDAY_OPENING: &str =
    "Name one thing you like about them and skip the round-number joke: ";

pub(super) const BIRTHDAY_CHILD: &str = concat!(
    "\"Happy %s - the year you finally beat me at cards.\" One true thing they did beats any ",
    "joke, and for a small child the card is really for the parent: keep it short and kind.",
);

pub(super) const BIRTHDAY_TEEN: &str = concat!(
    "\"Happy %s - still the best person to be stuck in a car with.\" One thing the two of you",
    " actually do together beats any joke about the number, and a teenager reads a short ",
    "card: no advice, no speech, and the money is welcome.",
);

pub(super) const BIRTHDAY_COLLEAGUE: &str = concat!(
    "\"Happy %s - hope you get spoiled.\" With someone you do not know well the warm plain ",
    "line is the card itself: no in-joke, and put the money or gift honestly (\"Have a drink",
    " on me\").",
);

pub(super) const BIRTHDAY_CLOSE: &str = concat!(
    "\"Happy %s - I still tell people about the night you drove us home at 3am.\" They are ",
    "close enough for the in-joke, so use the one only the two of you would get.",
);

pub(super) const BIRTHDAY_UNKNOWN: &str = concat!(
    "\"Happy %s - I still tell people about the night you drove us home at 3am.\" For a ",
    "colleague or someone you do not know well, a warm plain line is enough (\"Happy ",
    "birthday - hope you get spoiled\"), and put the money or gift honestly (\"Have a drink ",
    "on me\").",
);

pub(super) const AGE_ORDINAL: &str = r#"\b(\d{1,3})(?:st|nd|rd|th)\b"#;
pub(super) const AGE_IN_FRONT: &str =
    r#"(?:birthday|turning|turns|going on|hits?)\D{0,12}(\d{1,3})\b"#;
pub(super) const AGE_AHEAD: &str = concat!(
    r#"\b(\d{2,3})\b\s*(?:next|this|in)\s*(?:week|month|year|weekend|june|july|august|september|october|november"#,
    r#"|december|january|february|march|april|may|christmas|summer|spring|autumn|winter|easter)\b"#,
);

pub(super) const AGE_YEARS: &str = r#"\b(\d{1,2})\s*(?:year|yr)s?[ -]?old\b"#;
