// Source patterns used by the upstream thread-detail heavy-news gate.

use super::{LazyLock, Regex};

pub(super) static MEDICAL: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
    r#"\b(?:diagnos\w+|cancer|chemo\w*|tumou?r|biopsy)\b"#,
    r#"(?<!\bwork )(?<!\bworks )(?<!\bworked )\bin (?:the )?hospitals?\b"#,
    r#"\b(?:a&e|icu|ambulances?|overdose|surgery|operation|anaesthetic)\b"#,
    r#"\b(?:strokes?|heart attacks?|seizures?|chest pains?)\b"#,
    r#"\bcan'?t keep (?:anything|any food|food|fluids?|liquids?|water|solids?) down\b"#,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird)\b[^.!?]{0,16}\bcut (?:her|his|their|its)self\b"#,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird)\b[^.!?]{0,16}\b(?:has|had|got) (?:a |an )?(?:\w+ )?(?:cut|wound|gash|burn)\b"#,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird)\b[^.!?]{0,16}\b(?:cut|burnt|burned|bleeding|bled)\b[^.!?]{0,12}\b(?:her|his|their)\b"#,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird)\b[^.!?]{0,24}\b(?:collaps(?:e|ed|ing)|fainted|passed out|blacked out|not breathing|can'?t breathe|choking|bleeding badly)\b"#,
    r#"\bi\b[^.!?]{0,16}\b(?:collapsed|fainted|passed out|blacked out|threw up|vomited|was sick all|been sick all)\b"#,
    r#"\b(?:kill myself|killing myself|end my life|take my own life|want to die|better off dead)\b(?!\s+(?:my|your|his|her|their|our|the|a|an|this|that|it)\b)"#,
    r#"\b(?:won'?t let me leave|hits me|hit me|hurts? me|hurting me|threatened me|took my phone|takes my phone)\b"#,
    r#"\b(?:on fire|smoke in|gas leak|smell(?:s)? (?:of )?gas)\b"#,
    r#"\b(?:took|taken) (?:all|too many|the rest of) (?:my|his|her|their|the) pills\b"#,
    r#"\b(?:broke up|split up|divorce\w*|redundant|laid off|evicted|stolen|robbed|scammed)\b"#,
    r#"\b(?:i|we|he|she|they|my [a-z]+|our [a-z]+)\s*(?:just\s+|only\s+|then\s+|now\s+|today\s+)?(?:'?ve|have|has|had|was|were|been|being|got|get|gets|getting|is|are|am)\s+(?:been\s+|being\s+)?(?:sacked|fired(?!\s+up\b)|dismissed)\b|\b(?:they|he|she|the company|the firm|the shop|work|my boss|my manager|my employer)\s+(?:just\s+)?(?:sacked|fired(?!\s+up\b)|dismissed)\s+(?:me|him|them|us)\b|\b(?:they|the company|the firm|the shop|work|my boss|my manager|my employer)\s+let\s+(?:me|him|her|them)\s+go\b(?!\s+(?:home|early|out|to|and|for a)\b)(?!\s+on\s+(?:holiday|strike|leave|a\b|the\b|maternity|paternity|benefit|tour|safari))|\b(?:got|gotten|gave me|handed me|give me)\s+the (?:sack|boot|push)\b(?!\s+of\b)"#,
    r#"(?:\b(?:he|she|they|my (?:wife|husband|partner|boyfriend|girlfriend|fianc\w*|ex))[^.!?]{0,24}\b(?:left me|leaving me|leave me|left and|walked out|dumped me|cheated on me)\b(?!\s+(?:her|his|their|the|a|an|this|that|alone|be)\b))"#,
    r#"\b(?:thinking (?:of|about)|thought (?:of|about)|want(?:s|ed)? to|wanting to|going to|planning (?:on|to)|plan to|decided to|ready to|about to|meant to|should i|shall i|do i)\s+(?:just\s+|actually\s+|really\s+)?(?:leave|leaving|walk out on|end)\s+(?:(?:my|our) (?:wife|husband|partner|boyfriend|girlfriend|fianc\w*|marriage|ex|other half)|him|her|them|us)\b|\b(?:i'?m|im|i am|we'?re|were|we are)\s+(?:just\s+|actually\s+)?(?:leaving|walking out on)\s+(?:my (?:wife|husband|partner|boyfriend|girlfriend|fianc\w*|ex|other half)|him|her|them)\b(?!\s+(?:a|an|the|some|his|her|my|our)\b)"#,
    r#"\b(?:in debt|arrears|behind on (?:the )?(?:rent|mortgage|payments|bills)|can'?t afford|can not afford)\b"#,
    r#"\b(?:didn't get|didn't work out|didn't make it|turned down|turned it down|no luck|wasn't picked|not chosen)\b"#,
    r#"\b(?:i|we|he|she|they|ive|i've|my [a-z]+|our [a-z]+)\s+(?:have\s+|had\s+)?(?:lost|losing)\s+(?:out on\s+)?(?:the |my |our |his |her |their |your |that |this |a |an )?(?:jobs?|work|role|position|match(?:es)?|games?|deals?|contracts?|shift|hours|clients?|customers?|accounts?|bids?|bets?|elections?|deposits?|offers?|interviews?|places?|spots?|funding|grants?|appeals?|races?|finals?|trials?|orders?|bookings?|tenanc(?:y|ies)|tenders?|placement|promotion|apprenticeship)\b"#,
    r#"\bmiscarriage(?:s)?\b(?! of justice)|\bmiscarried\b"#,
    r#"\bstill ?born\b|\bstill ?birth\b"#,
    r#"\b(?:lost|losing|we lost) (?:the|our|her|his|their) baby\b(?!\s+(?:weight|belly|tummy|fat|pounds?|stones?|kilos?|inches|clothes|things|stuff|room|pram|toys?|seat|carrier|monitor|name|bag|book)\b)"#,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird)\b[^.!?]{0,24}\bkilled (?:him|her|them)sel(?:f|ves)\b(?!\s+(?:laughing|giggling))"#,
    r#"\b(?:took|taken|taking) (?:his|her|their) own life\b|\bcommitted suicide\b"#,
    r#"\bpregnan\w+\b[^.!?]{0,24}\b(?:spotting|bleeding|cramping)\b"#,
    r#"\b(?:spotting|bleeding)\b[^.!?]{0,24}\bpregnan\w+\b"#,
].into_iter().map(|expression| Regex::new(expression).expect("medical pattern")).collect()
});
pub(super) static NAMED_NEWS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird|nephew|niece|stepson|stepdaughter|stepmum|stepdad|stepmother|stepfather|grandson|granddaughter|mother in law|father in law|son in law|daughter in law|grandchild|grandchildren)\b[^.!?]{0,20}\b(?:has|have|had|got|is|was|were|been|being)\b[^.!?]{0,24}\b(?:dementia|alzheim\w*|altzheim\w*|parkinson\w*|motor neurone|huntington\w*|multiple sclerosis|muscular dystrophy|leuka?emia|lymphoma|myeloma|mesothelioma|pancreatitis|copd|emphysema|cirrhosis|kidney failure|heart failure|brain (?:damage|injury)|cerebral palsy|terminal)\b"#,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird|nephew|niece|stepson|stepdaughter|stepmum|stepdad|stepmother|stepfather|grandson|granddaughter|mother in law|father in law|son in law|daughter in law|grandchild|grandchildren)\b[^.!?]{0,24}\b(?:is|are|was|were|has been|have been|had been|got|getting|become|becoming|been)\b[^.!?]{0,16}\b(?:ill|poorly|unwell|not well|off (?:it|colour|their food)|in a bad way|taken ill|gone downhill|deteriorat\w+)\b"#,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird|nephew|niece|stepson|stepdaughter|stepmum|stepdad|stepmother|stepfather|grandson|granddaughter|mother in law|father in law|son in law|daughter in law|grandchild|grandchildren)\b[^.!?]{0,24}\bd(?:iagnos\w+|x\b)"#,
    r#"\b(?:been|being|just|newly|recently)\s+(?:diagnos(?:ed|is)\w*)\b"#,
    r#"\b(?:not allowed|no longer allowed|allowed|banned|barred|stopped|prevented)\b[^.!?]{0,24}\b(?:see|seeing|visit|visiting|contact(?: with)?)\b[^.!?]{0,20}\b(?:kids?|children|child|son|daughter|grandchild\w*|baby)\b"#,
    r#"\b(?:pip|dla|esa|universal credit|housing benefit|child benefit|tax credits?|benefits?|allowance|pension|pay|wages|hours)\b[^.!?]{0,24}\b(?:stopped|cancell?ed|cut|suspended|frozen|refused|sanctioned|withdrawn|reduced|turned down|taken away|not renewed)\b"#,
    r#"\b(?:stopped|cancell?ed|cut|suspended|frozen|sanctioned|withdrawn|refused|reduced)\b[^.!?]{0,20}\b(?:my|our)\s+(?:pip|dla|esa|universal credit|benefits?|pay|wages|pension|allowance|hours)\b"#,
    r#"\b(?:got|gotten|been|being|was|were|just|now)\s+sanctioned\b(?!\s+(?:leave|holiday|absence|time off|day off|overtime|pay rise)\b)"#,
    r#"\b(?:have|has|had|having|got|gotten|getting|being (?:forced|made))\s*(?:to\s+)?(?:sell|give up|hand back|move out of)\b[^.!?]{0,24}\b(?:the|our|my|his|her)\s+(?:house|home|flat|bungalow|place)\b"#,
    r#"(?!.*\b(?:adopt\w*|adoption)\b)\b(?:got|gotten|had|received|been sent|sent me|sending me|came through|arrived)\b[^.!?]{0,28}\b(?:court (?:date|letter|order|summons|hearing)|summons|notice to quit|eviction notice|possession (?:claim|order)|letter before (?:claim|action))\b"#,
    r#"\bbailiffs?\b"#,
    r#"(?!.*\b(?:adopt\w*|adoption)\b)\b(?:going|have to go|has to go|got to go|due|off)\s+(?:in |to |before )?court\b"#,
    r#"\b(?:taking|took|taken|summoned|prosecuting|suing)\s+(?:me|us|him|her|them)\s+to court\b"#,
    r#"\b(?:i'?m|im|i am|we'?re|we are|i was|we were)\s+(?:being\s+)?(?:taken|summoned|prosecuted|sued)\s+(?:to|into|before)\s+court\b"#,
    r#"\b(?:hearing|trial|case)\s+(?:is |was )?(?:next|this)\s+(?:week|month|monday|tuesday|wednesday|thursday|friday|morning|afternoon)\b"#,
    r#"\b(?:going|went|moved|transferred|have to go|has to go|got to go|off|sent|sentenced|remanded|due)\b[^.!?]{0,16}\b(?:to |in |into )?(?:prison|jail|young offenders)\b(?!\s+(?:to\s+(?:visit|see|meet)|visiting|museum|tour|open\s+day))"#,
    r#"\b(?:is|are|was|were|been|being|stays?|sits?|sitting|kept|held|locked)\s+(?:up\s+|inside\s+|away\s+)?(?:in|at|inside)\s+(?:the\s+)?(?:prison|jail|jailhouse|young offenders|pentonville|wormwood scrubs|brixton|wandsworth)\b"#,
    r#"\bsent down\b[^.!?]{0,16}\b(?:for|to)\s+(?:\d+|two|three|four|five|six|seven|eight|nine|ten|twelve|eighteen|twenty|a\s+(?:year|month|stretch|while)|life)\b"#,
    r#"\b(?:was|were|is|are|been|being|got|gets|getting|keeps being)\s+sent down\b"#,
    r#"\b(?:taken|took|put|puts|removed|removes)\b[^.!?]{0,20}\b(?:into care|in care)\b"#,
    r#"\b(?:taken|took|removed)\b[^.!?]{0,24}\b(?:my|our|the)\s+(?:kids?|children|child|son|daughter|baby)\b[^.!?]{0,16}\baway\b"#,
    r#"\bsocial services\b[^.!?]{0,24}\b(?:took|taken|removed|involved|came)\b"#,
    r#"\b(?:losing|lose|lost|going to lose|about to lose|might lose|will lose|going to be losing)\b[^.!?]{0,12}\b(?:the|our|my)\s+(?:house|home|flat|bungalow)\b"#,
    r#"\b(?:got|get|getting|been|being|was|were)\s+sacked\b"#,
    r#"\b(?:bad|sad|awful|terrible|dreadful|grim|rotten|horrible|horrid)\s+(?:news|tidings)\b"#,
].into_iter().map(|expression| Regex::new(expression).expect("named_news pattern")).collect()
});
pub(super) static CLEAR_RESULT: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
    r#"(?i)\b(?:in|out\s+of)\s+(?:full\s+)?remission\b"#,
    r#"(?i)\b(?:scan|scans|results?|tests?|biopsy|mri|ct|x-?ray|bloods?|screening|checks?|check\s?up|swabs?)\b[^.!?]{0,24}?\b(?:came\s+back|come\s+back|was|were|is|are|looks?|looked)\s+(?:all\s+)?(?:clear|normal|fine|negative)\b"#,
    r#"(?i)\b(?:biopsy|lump|mole|tumou?r|cyst|polyp|growth|cells?|cells)\b[^.!?]{0,20}?\b(?:was|were|is|are|turned\s+out|come\s+back|came\s+back)\s+(?:benign|harmless|nothing\s+to\s+worry\s+about)\b"#,
    r#"(?i)\b(?:hasnt|hasn't|hadnt|hadn't|has|had)\s+not\s+spread\b|\bit\s+(?:hasnt|hasn't|hadnt|hadn't)\s+spread\b"#,
    r#"(?i)\bno\s+(?:sign|trace|evidence)\s+of\s+(?:it|any\s+cancer|the\s+cancer|the\s+disease)\b"#,
    r#"(?i)\b(?:treatment|chemo|chemotherapy|radiotherapy|surgery|operation|transplant)\b[^.!?]{0,24}?\b(?:is\s+working|worked|went\s+(?:well|fine)|was\s+a\s+success|has\s+worked)\b"#,
    r#"(?i)\b(?:tumou?r|lump|cancer|growth)s?\b[^.!?]{0,16}?\b(?:has|have)\s+(?:gone|shrunk|shrinking|stopped\s+growing)\b"#,
    r#"(?i)\b(?:its|it\s+is|it\s+was|hes|she's|theyre|they're)\s+(?:shrinking|stopped\s+growing|gone)\b"#,
].into_iter().map(|expression| Regex::new(expression).expect("clear_result pattern")).collect()
});
pub(super) static SUDDEN_LOSS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
    r#"\bmiscarriage(?:s)?\b(?! of justice)|\bmiscarried\b"#,
    r#"\bstill ?born\b|\bstill ?birth\b"#,
    r#"\b(?:lost|losing|we lost) (?:the|our|her|his|their) baby\b(?!\s+(?:weight|belly|tummy|fat|pounds?|stones?|kilos?|inches|clothes|things|stuff|room|pram|toys?|seat|carrier|monitor|name|bag|book)\b)"#,
    r#"\b(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird)\b[^.!?]{0,24}\bkilled (?:him|her|them)sel(?:f|ves)\b(?!\s+(?:laughing|giggling))"#,
    r#"\b(?:took|taken|taking) (?:his|her|their) own life\b|\bcommitted suicide\b"#,
].into_iter().map(|expression| Regex::new(expression).expect("sudden_loss pattern")).collect()
});
pattern!(
    JOKED,
    r#"\b(?:seizures?|choking|strokes?|heart attacks?|collapsed|passed out)\b[^.!?]{0,14}\b(?:laughing|giggl\w*|cracking up|in stitches)\b"#
);
pattern!(
    SUICIDE,
    r#"(?i)\b(?:(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird)|(?:boy|girl|lad|lass|kid|child|man|woman|person|someone|anyone|classmate|schoolmate|teammate|workmate|student|pupil|neighbour|neighbor|friend|mate|fella|guy|colleague|he|she|they|everyone|everybody))\b[^.!?]{0,26}\b(?:killed (?:him|her|them)sel(?:f|ves)|hanged (?:him|her|them)sel(?:f|ves)|took (?:his|her|their) own life|committed suicide)\b(?!\s+(?:laughing|giggling))"#
);
pattern!(
    DEATH_SAID,
    r#"\b(?:died(?!\s+(?:laughing|giggling|smiling|of\s+(?:embarrassment|boredom|shame|laughter|fear)|inside|a\s+(?:little|bit)|and\s+gone|(?:her|his|their|my|our|your|its)\s+hair))|dead(?!\s+(?:funny|good|nice|cheap|easy|serious|certain|jealous|tired|right|wrong|set|on\b|beat|last\b|chuffed|proud|pleased|happy|keen|excited|thrilled|impressed|lucky|jammy|buzzing|gutted|made\s+up|against|ahead|centre|straight|silence|giveaway|heat|calm|in\s+(?:the\s+middle|a\s+row)|ringer|to\s+(?:the\s+world|rights|me|us|him|her|them)\b))|passed away|killed himself|killed herself|killed themselves|hanged himself|hanged herself|hanged themselves|took his own life|took her own life|took their own lives|committed suicide|suicid\w*|found dead|passed on(?!\s+(?:the|a|an|my|his|her|their|our|your|its|it|this|that|these|those|some|all|both)\b)|didn'?t make it(?!\s+(?:(?:to|for)\s+\w|in\s+time|on\s+time))|did not make it(?!\s+(?:(?:to|for)\s+\w|in\s+time|on\s+time)))\b"#
);
pattern!(
    NEARLY,
    r#"(?i)\b(?:nearly|almost|practically|might\s+have|could\s+have|would\s+have|like\s+to\s+have)\b"#
);
pattern!(
    AFTER_IT,
    r#"(?i)^\s*(?:laughing|giggling|smiling|of\s+(?:embarrassment|boredom|shame|laughter|fear)|to\s+(?:me|us|him|her|them)\b)"#
);
pattern!(
    NAMES_ITS_SUBJECT,
    r#"(?i)^(?:killed|hanged)\s+(?:him|her|them)sel(?:f|ves)\b|^took\s+(?:his|her|their)\s+own\s+li(?:fe|ves)\b|^committed\s+suicide\b|^found\s+dead\b"#
);
pattern!(WORD, r#"[a-z']+"#);
pattern!(
    ASK_OPEN,
    r#"(?i)^(?:so|well|ok|okay|and|but|ah|oh|right|and)?[\s,]*(?:is|are|was|were|do|does|did|has|have|will|would|can|could|should|any)\b"#
);
pattern!(
    NOT_CLEAR,
    r#"(?i)\b(?:not|isnt|isn't|wasnt|wasn't|arent|aren't|werent|weren't|never|no)\s+(?:\w+\s+){0,2}?(?:\w+\s+)?(?:clear|benign|remission|normal|negative|gone|harmless)\b|\b(?:has|had)\s+spread\b|\bhasnt\s+(?:worked|helped)\b|\bdidnt\s+work\b|\bnot\s+(?:worked|working)\b"#
);
pattern!(
    NEGATED,
    r#"(?i)\b(?:dont|don't|doesnt|doesn't|didnt|didn't|did\s+not|do\s+not|isnt|isn't|wasnt|wasn't|not|never|no\s+longer|no)\s+(?:(?:really|very|much|so|still|any|getting|feel|feels|feeling|felt|seem|seems|be|is|was|am|a|bit|lot|longer)\s+){0,3}better\b"#
);
pattern!(
    CONDITIONAL,
    r#"(?i)\bbetter\b[^.!?]{0,24}?\b(?:when|whenever|if|unless|as\s+long\s+as)\b"#
);
pattern!(
    NOT_NEWS,
    r#"(?i)\b(?:did|do|does|will|would|are|is|was|were|can|could|shall|should|have|has)\s+(?:i|you|we|she|he|they)\b|\b(?:hope|hoping|wish|wishing|wonder|wondering|praying|fingers crossed|fingers are crossed)\b"#
);
pattern!(NOT_QUITE, r#"(?i)\b(?:almost|nearly)\b"#);
pattern!(
    CLEAR_TAIL,
    r#"(?i)\s*(?:(?:now|again|today|thanks|thank\s+god|thankfully|thank\s+fully|finally|at\s+last|anyway|so\s+far|great|brilliant|so\s+(?:relieved|happy|pleased|glad)|relieved|what\s+a\s+relief)\s*)*[.!]?$"#
);
pattern!(
    DREAM_NAMED,
    r#"(?i)\b(?:i|we)\s+(?:just\s+)?(?:had\s+(?:a\s+|the\s+|this\s+)?(?:dream|nightmare|bad\s+dream)|(?:had|have)\s+(?:a\s+|this\s+)?(?:dream|nightmare|nightmares|bad\s+dreams?|weird\s+dreams?)|dream(?:t|ed|ing)?(?:\s+about|\s+that|\s+of|,|\s+i\b)|keep\s+dreaming|kept\s+dreaming|nightmares?\s+(?:about|where|that)|(?:was|were)\s+dreaming)\b"#
);
pattern!(
    DREAM_NAMED_LOOSE,
    r#"(?i)\b(?:in\s+(?:my|a|the)\s+dream|in\s+my\s+nightmare|in\s+dreams?|dreamt|dreamed|dreaming|nightmares?|nightmare)\b"#
);
pattern!(
    A_WISH_TO_BE_GONE,
    r#"(?i)\bdream(?:s|t|ed|ing)?\s+(?:of|about)\s+(?:my\s+own\s+|me\s+|being\s+)?(?:ending\s+it(?:\s+all)?|ending\s+things|ending\s+my\s+life|ending\s+everything|kill(?:ing)?\s+myself|suicide|suicidal|dying|death|being\s+dead|dead|not\s+waking|not\s+being\s+here|disappearing|gone\s+for\s+good)\b"#
);
pattern!(
    A_SAID_WISH,
    r#"(?i)\b(?:ending\s+it\s+all|ending\s+things|kill(?:ing)?\s+myself|kill\s+me|suicide|suicidal|want(?:ed|ing)?\s+to\s+die|wanted\s+to\s+be\s+dead|wish(?:ed)?\s+i\s+(?:was|were)\s+dead|not\s+being\s+here|not\s+here\s+any\s+more|don'?t\s+want\s+to\s+be\s+here)\b"#
);
pattern!(
    A_DREAM_AHEAD,
    r#"\bdream(?:s)?\s+(?:job|jobs|career|role|home|house|flat|holiday|holidays|trip|car|wedding|team|life|garden|kitchen|body|figure|man|woman|partner|business|company|kitchen|room|wardrobe|outfit)\b"#
);
pattern!(DREAM_VERB, r#"\b(?:dreamt|dreamed|dreaming)\b"#);
pattern!(
    DREAM_IS_HAPPENING_NOW,
    r#"(?i)\b(?:woke\s+up|woken\s+up|wake\s+up|awake|awoken|still\s+(?:there|here|going|on|going\s+on|burning|bleeding|hurting|happening|smoking|happens|feel|feels|have|got)|right\s+now|just\s+now|this\s+minute|at\s+the\s+moment|and\s+i\s+(?:can'?t|cannot|cant)\s+(?:breathe|move|feel|stop)|it\s+(?:is|was)\s+real|i\s+(?:can|could)\s+smell\s+(?:smoke|gas|burning)|i\s+(?:have|took|taken|swallowed|drank|drunk)\s+(?:them|it|the|some|a\s+few|\d+|all\s+of)|\b\d+\s+(?:of\s+them|tablets|pills)|i\s+am\s+(?:bleeding|burning|stuck|trapped|drowning|having|on\s+fire)|i'?m\s+(?:bleeding|burning|stuck|trapped|drowning|having|on\s+fire|not\s+breathing)|my\s+(?:arm|leg|chest|head|back|side|stomach|belly|hand|foot|face)\s+(?:is\s+|feels\s+)?(?:numb|bleeding|swollen|hot|painful|hurts|broken|blue)|there\s+(?:is|are)\s+(?:smoke|flames?|blood|pain)|the\s+(?:smoke|flames?|alarm|bleeding)\s+(?:is|are)\s+(?:still|now|real)|(?:i\s+|we\s+)?(?:can\s+)?(?:smell|smells|smelling)\s+(?:smoke|gas|burning)|\b(?:smoke|flames?)\s+(?:is\s+|are\s+)?(?:coming|pouring|everywhere)|\balarm\s+(?:is\s+|keeps\s+)?(?:going|beeping|sounding)|\b(?:is|are)\s+(?:going|happening)\s+(?:on\s+)?now|\b(?:he|she|they|it|the\s+baby|my\s+baby)(?:'s|s|\s+is|\s+was|\s+are|\s+were)?\s+(?:not|isn'?t|wasn'?t|aren'?t|aint|ain'?t|no\s+longer)\s+(?:breathing|moving|responding|waking|conscious|with\s+us|alive)|\b(?:shes|hes|theyre|its)\s+(?:not|isn'?t)\s+(?:breathing|moving|responding|waking|conscious)|\b(?:lying|lay)\s+(?:there|on\s+the\s+(?:floor|ground|bed|road))\s+(?:now|still)|\b(?:there'?s|theres)\s+smoke\s+(?:in|coming|everywhere)|\b(?:i\s+can\s+hear|i\s+hear)\s+(?:the\s+alarm|smoke\s+alarm|sirens))\b"#
);
pattern!(
    A_DEATH_BEFORE_THE_DREAM,
    r#"(?i)\b(?:died|dying|death|dead|deaths|passed\s+away|passed\s+on|killed|funeral|buried|cremation|widow|widowed)\b[^.]*?\b(?:dream|dreams|dreamt|dreamed|dreaming|dreamed|nightmare|nightmares)\b"#
);
pub(super) const EMERGENCY_TYPOS: &[&str] = &[
    "ambulance",
    "bleeding",
    "breathing",
    "cancer",
    "choking",
    "collapsed",
    "diagnosed",
    "hospital",
    "overdose",
    "seizure",
    "stroke",
    "unconscious",
];
pub(super) const NOT_A_TYPO: &[&str] = &["collapse", "strike", "stroked", "strokes", "stroking"];
pub(super) const CARRIES: &[&str] = &[
    "a",
    "about",
    "after",
    "almost",
    "also",
    "am",
    "an",
    "and",
    "apparently",
    "are",
    "at",
    "basically",
    "be",
    "been",
    "before",
    "being",
    "but",
    "definitely",
    "finally",
    "for",
    "from",
    "had",
    "has",
    "have",
    "her",
    "his",
    "honestly",
    "if",
    "in",
    "is",
    "its",
    "just",
    "last",
    "like",
    "literally",
    "my",
    "nearly",
    "now",
    "obviously",
    "of",
    "on",
    "our",
    "peacefully",
    "presumably",
    "quietly",
    "really",
    "recently",
    "sadly",
    "so",
    "still",
    "suddenly",
    "that",
    "the",
    "their",
    "then",
    "this",
    "to",
    "today",
    "tragically",
    "unexpectedly",
    "unfortunately",
    "very",
    "was",
    "were",
    "when",
    "which",
    "while",
    "who",
    "with",
    "yesterday",
    "your",
];
pub(super) const PLACE: &[&str] = &[
    "class",
    "classes",
    "classroom",
    "club",
    "college",
    "company",
    "course",
    "estate",
    "family",
    "firm",
    "floor",
    "group",
    "gym",
    "hospital",
    "house",
    "nursery",
    "office",
    "road",
    "school",
    "shift",
    "shop",
    "site",
    "street",
    "team",
    "town",
    "uni",
    "university",
    "village",
    "ward",
    "work",
    "year",
];
pub(super) const PERSON: &[&str] = &[
    "anyone",
    "aunt",
    "baby",
    "bird",
    "boss",
    "boy",
    "boyfriend",
    "brother",
    "cat",
    "child",
    "classmate",
    "colleague",
    "cousin",
    "dad",
    "daughter",
    "dog",
    "everybody",
    "everyone",
    "ex",
    "father",
    "fella",
    "friend",
    "girl",
    "girlfriend",
    "gran",
    "grandad",
    "grandma",
    "grandpa",
    "granny",
    "guy",
    "he",
    "horse",
    "husband",
    "kid",
    "kitten",
    "lad",
    "lass",
    "man",
    "mate",
    "mom",
    "mother",
    "mum",
    "nan",
    "nana",
    "neighbor",
    "neighbour",
    "partner",
    "person",
    "pet",
    "pupil",
    "puppy",
    "rabbit",
    "schoolmate",
    "she",
    "sister",
    "someone",
    "son",
    "student",
    "teacher",
    "teammate",
    "they",
    "uncle",
    "wife",
    "woman",
    "workmate",
];
