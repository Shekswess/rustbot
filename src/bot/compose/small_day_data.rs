//! Reading and acknowledgement of a plain account from the pinned composer.

pattern!(
    A_BAD_THING_NAMED,
    r####"\b(?:bad|sad|awful|terrible|dreadful|grim|rotten|lousy|horrible|horrid|rubbish|tough)\s+(?:news|tidings)\b"####
);
pattern!(
    A_DAY_SUBJECT,
    r####"\b(?:i|i'm|im|ive|i've|we|we're|weve|we've|my|our|it|work|school|the weather|the day)\b"####
);
context_pattern!(
    A_DAY_WENT_WRONG,
    r####"\b(?:burgl\w*|broken into|broke into|break-in|break in|robbed|robbery|mugged|mugging|stolen|stole|theft|shoplifted|nicked|pinched|keyed|vandalised|vandalized|scratched the car|written off|wrote it off|wrote the car off|caught fire|went on fire|on fire|burst pipe|water damage|smoke damage|gone missing|went missing|disappeared|vanished|crash|car crash|had a prang|a prang|in a smash|was rear-ended|rear-ended|hit by a car|fell off|come off my bike|came off my bike|fell down the stairs)\b|\bcrash(?:ed)?\b(?!\s+out)\b|\bflood(?:ed)?\b(?!\s+with)\b"####
);
pattern!(
    A_HAND_OUT,
    r####"\b(?:asked|ask|asking|looked|looking|reached|reaching)\s+(?:for|out for)\s+(?:help|support|a hand|some help)\b"####
);
context_pattern!(
    A_HURT_MYSELF,
    r####"\b(?:i|i've|ive)\b\s*(?:just\s+|almost\s+|nearly\s+|really\s+|actually\s+)?(?:cut|cutting|hurt|hurting|harm|harmed|harming|burn|burned|burnt|burning|hit|hitting|punish|punished|punishing|starve|starved|starving|poison|poisoned|poisoning)\s+myself\b(?!\s+(?:a|an|the|some|another|either|neither|one|two|three)\b)"####
);
pattern!(
    A_LOOKING_BACK,
    r####"(?i)^(?:and\s+|but\s+|so\s+|well\s+)?(?:like\s+|as\s+)?i\s+(?:already\s+|did\s+)?(?:said|told\s+you|mention\w*)\b[^.!?]{0,50}?\b(?:it|that|this)\s+(?:only\s+|just\s+|actually\s+|never\s+)?(?:was|were|is|had|started|began)\b"####
);
pattern!(
    A_SYMPTOM_PHRASE,
    r####"(?i)\b(?:hot|warm)\s+(?:and\s+)?(?:red|redder|swollen|tender|puffy|angry)\b|\bred(?:ness)?\s+(?:and\s+)?(?:hot|warm|spreading|swollen|tender|angry|streaky)\b|\bhot to (?:the )?touch\b|\bweeping\b|\boozing\b|\boozed\b|\bpus\b|\bswollen (?:and|or) (?:red|hot|tender)\b"####
);
context_pattern!(
    BAD_DAY_ANYWAY,
    r####"\b(?:hospital|a&e|ambulance|fell over|a fall|funeral|died|death|test results|biopsy|crash|crashed|accident|collision|flood|flooded|burst pipe|fire|burgled|robbed|stolen|wrote off|killed|killing|(?:massive|big|huge|proper|right|awful|terrible) row|row (?:about|over|with)|chemo|miscarriage|evicted|lost my job|sacked|fight|row with|panic|anxiety attack|ticket|morphine|diamorphine|midazolam|syringe driver|palliative|end of life|hospice|terminal|clamped|fined|argued|argument|shouted|cried|upset|worried|worries|sick|threw up|migraine|rained|soaked|drenched|wet through|late for|missed my|missed the|clinic|dentist|surgery|wrong|mistake|messed up|screwed up|waited|waiting|queued|queue|on hold|hung around|hung about|two hours|three hours|four hours|an hour|all night|headache|aching|exhausted|shattered|knackered|gutted|a row|an argument|strange|grim|full on|fell flat|meh|came\s+off|smashed|smash|flooded|flooding|stacked\s+it\b|stacked\s+(?:my|the|his|her|their)\s+(?:bike|motorbike|moped|scooter|motorcycle|car|van|horse)\b|wrecked|dented|chipped|snapped\s+(?:at|my|the|his|her|their)\b|went up in flames|caught fire|blew up|blew a fuse|went bang|collapsed|fell apart|packed in|conked out|stopped working|wouldnt start|wouldn't start|broke down|leaking|came back (?:short|lower|worse)|under (?:our|the) (?:offer|asking)|over budget|went over budget|(?:is|are|was|were) (?:still )?closed|told\s+[^,.;!?]{0,25}?\boff\b|had a go at me|had a right go|boot on (?:my|the|our) (?:car|van|motor|bike)\b|fell in (?!love)|choked(?!\s+up)|short of breath|blackened|burnt|burned|scorched|stained|rusted|scratched|slipping up|slipped up|messed up|mucking up|mucked up|knocked\s+over|tipped\s+over)\b"####
);
pattern!(
    DAY_PAST,
    r####"\b(?:today|tonight|last night|yesterday|this morning|this afternoon|this evening|at the weekend|over the weekend|this weekend|last weekend|all day|all morning|the other day|earlier)\b"####
);
pattern!(
    NOT_A_DAY_DONE,
    r####"\b(?:tomorrow|next (?:week|weekend|month|year|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|gonna|going to|i will|i'll|ill|want to|wanna|plan to|planning to|should i|shall i|can i|could i|do i|did i|have i|am i|are we|named|renamed)\b"####
);
pattern!(
    NOT_A_DAY_SAID,
    r####"\b(?:not|never|no|isn['’]?t|aren['’]?t|don['’]?t|didn['’]?t|wasn['’]?t|weren['’]?t|can['’]?t|couldn['’]?t|won['’]?t|hardly|cannot|haven['’]?t|hasn['’]?t|hadn['’]?t|wouldn['’]?t|shouldn['’]?t|mustn['’]?t|needn['’]?t|ain['’]?t|daren['’]?t|shan['’]?t|oughtn['’]?t)\b"####
);
context_pattern!(
    PAST_DOING,
    r####"\b(?:had|got|saw|was|were|went|did|made|took|met|ate|drank|bought|brought|read|played|watched|walked|cooked|cleaned|painted|cut|sorted|finished|started|spent|sat|ran|came|left|gave|found|booked|visited|helped|learned|learnt|worked|slept|drove|caught|wore|won|felt|heard|told|said|sold|paid|sent|kept|put|let|set|fell|held|stood|thought|knew|began|broke|spoke|rang|sang|wrote|built|swam|saw|ran)\b|\b(?:(?!red\b|bed\b|shed\b|need\b|feed\b|bleed\b|speed\b|weed\b|seed\b|breed\b|proceed\b|succeed\b|exceed\b|indeed\b|hundred\b|greed\b|creed\b)\w+ed)\b"####
);
pattern!(
    SOMEONE_DONE_WRONG,
    r####"\b(?:called (?:me|him|her|them|us) names|call (?:me|him|her|them|us) names|called names|name[- ]calling|called (?:me|him|her|them|us) a \w+ name|got called names|got called (?:a |an |the )?(?:\w+ )?names?\b|(?:was|were|is|are|being) called (?:a |an |the )?(?:\w+ )?names?\b|teased (?:me|him|her|them|us)|teasing (?:me|him|her|them|us)|bullied|bullying|bullies|picked on|picking on|picked last|picking (?:me|him|her|them) last|left (?:me|him|her|them|us) out|leaving (?:me|him|her|them|us) out|laughed at (?:me|him|her|them|us)|laughing at (?:me|him|her|them|us)|made fun of (?:me|him|her|them|us)|making fun of (?:me|him|her|them|us)|mean to (?:me|him|her|them|us)|nasty to (?:me|him|her|them|us)|horrible to (?:me|him|her|them|us)|(?:would|wouldnt|wouldn't|wont|won't|didnt|didn't) let (?:me|him|her|them) play|nobody (?:will|would|wants to) play with|snatched (?:it|the \w+) (?:off|from) (?:me|him|her|them))\b"####
);
pattern!(
    AN_ATTEMPT_AT_SOMEONE,
    r####"\b(?:asked|told|texted|messaged|phoned|rang|called|emailed|chased|reminded|warned|complained|spoke|talked|written|wrote)(?: to)?\s+(?:him|her|them|us)\b|\b(?:asked|told|texted|messaged|phoned|rang|called|emailed|chased|reminded|warned|complained|spoke|talked|written|wrote)\b(?:\s+\w+){0,4}?\s+(?:twice|three times|four times|five times|repeatedly|again|once more|for the (?:second|third|fourth|fifth) time)\b"####
);
pattern!(
    A_BEING_DONE_NOW,
    r####"(?i)\b(?:i|we|he|she|they|it|you|the|my|our|his|her|their|\w+s)\s+(?:am|is|are|was|were|been)\s+being\b|\b(?:i'm|im|we're|they're|he's|she's|it's)\s+being\b"####
);
pattern!(
    A_BETRAYAL_FOUND_OUT,
    r####"\b(?:i |we )?(?:just |only |now )?(?:found|find|findin[g]|turned|turn) (?:it )?out\b[^.!?]{0,45}\b(?:ly(?:ing|ed)|lies|lie\b|cheat\w*|hiding|hidden|stole|stealing|steals|gambl\w*|affair|seeing someone|texting (?:his|her|their) ex|back with (?:his|her|their) ex|drinking again|on the (?:booze|drugs) again|using again|owed? (?:me|us) money|borrowed money|spent (?:it|the|our|my) .{0,20}money|gone behind my back)\b|\b(?:turns|turned) out\b[^.!?]{0,45}\b(?:ly(?:ing|ed)|lies|lie\b|cheat\w*|hiding|hidden|stole|stealing|steals|gambl\w*|affair|seeing someone|texting (?:his|her|their) ex|back with (?:his|her|their) ex|drinking again|on the (?:booze|drugs) again|using again|owed? (?:me|us) money|borrowed money|spent (?:it|the|our|my) .{0,20}money|gone behind my back)\b|\b(?:i )?(?:just )?(?:heard|learned|learnt|discovered)\b[^.!?]{0,40}\b(?:s?he|they|my \w+) (?:has been|had been|was|were|is|cheated|lied|stole)\b"####
);
pattern!(
    A_DECISION_NOT_A_DAY,
    r####"(?i)\b(?:approved|granted|confirmed)\b"####
);
pattern!(
    A_NAME_IS_NOT_A_DAY,
    r####"\b(?:is|are|was|were)\s+(?:called|named)\b"####
);
pattern!(
    A_WISH,
    r####"(?i)\b(?:i|we)\s+(?:really\s+|just\s+|so\s+|do\s+)?wish\b|\bif only\b"####
);
pattern!(
    PLAN_THAT_FELL_THROUGH,
    r####"\b(?:was|were|is|are|had|have)\s+(?:meant|supposed)\s+to\s+[a-z]+"####
);
pattern!(
    RESULT_WENT_WRONG,
    r####"\b(?:came out|comes out|turned out|turns out|came back|comes back|ended up|went|goes)\s+(?:a bit\s+|a little\s+|all\s+|really\s+|quite\s+|so\s+)?\s*(?:grey|gray|dry|soggy|burnt|burned|raw|tough|dense|rubbery|pale|lumpy|stodgy|stale|bland|wrong|ruined|awful|horrible|terrible|overdone|undercooked|flat|sour|bitter|curdled|split|seized)\b"####
);
pattern!(
    GOOD_VERDICT,
    r####"(?ix)
    ^\W*(?P<subject>(?:it|its|it\'s|that|thats|that\'s|this|the\s+\w+|my\s+\w+|our\s+\w+)
         (?:\s+(?:was|is|were|are|has\s+been|\'s\s+been))?)\s+
    (?:really\s+|so\s+|proper\s+|pretty\s+|dead\s+|quite\s+|absolutely\s+|well\s+)*
    (?:good|great|lovely|nice|brilliant|amazing|wonderful|lush|smashing|fantastic|superb|
        class|fun)\b
    (?:\s+(?:actually|honestly|tbh|though|to\s+be\s+fair))?\W*$
"####
);
pattern!(
    GOOD_DAY_BARE,
    r####"^(?:what |such |a )?(?:really |so |pretty |proper |quite )?(?:great|lovely|brilliant|amazing|wonderful|fantastic|lush|smashing|perfect)\s+(?:day|week|weekend|evening|afternoon|morning|night|shift)\b"####
);
pattern!(
    GOOD_DAY_OURS,
    r####"^(?:(?:i|we|ive|i've|weve|we've|(?:my|our|the|his|her|their)(?:\s+\w+'s)?(?:\s+\w+){0,2})\s+)?(?:had|have had|ve had|has been having|am having|are having)\s+(?:a\s+|such a\s+|a really\s+|a proper\s+|a pretty\s+)?(?:really\s+|properly\s+|pretty\s+|so\s+|such\s+|super\s+)?(?:good|great|lovely|nice|brilliant|amazing|wonderful|lush|smashing|fantastic)\s+(?:day|week|weekend|evening|afternoon|morning|night|shift|time|walk|swim|run|ride|cycle|lunch|dinner|meal|picnic|bath|shower|breakfast|coffee|nap|sleep|lie in|read)\b"####
);
pattern!(
    GOOD_DAY_WENT,
    r####"\b(?:today|yesterday|the day|the week|the weekend|last night|this (?:morning|afternoon|evening)|my day|my week|my weekend)\s+(?:was|has been|went|'s been)\s+(?:a\s+)?(?:really\s+|so\s+|proper\s+|pretty\s+|such\s+|quite\s+)?(?:good|great|lovely|nice|brilliant|amazing|wonderful|fine|alright|lush|fantastic)\b|\bbest\s+(?:day|week|weekend|evening|night)\b"####
);
pattern!(
    GOOD_TIME_IT,
    r####"^(?:it|that|this)\s+(?:was|were|has been|s been|turned out)\s+(?:(?:a|an|such a|such)\s+)?(?:(?:really|so|pretty|proper|properly|quite|absolutely|dead|very)\s+)?(?:good|great|lovely|nice|brilliant|amazing|wonderful|fantastic|lush|smashing)\b(?:\s+(?:fun|time|day|night|evening|weekend|week|meal|trip|holiday|break|wedding|party|gig|show|film|game|match|outing|afternoon|morning|christmas|birthday|dinner|lunch|barbecue|do|out|away|all|overall|actually|though|thanks|in the end)\b)*\s*$"####
);
pattern!(
    GOOD_TIME_OURS,
    r####"\bwe (?:had|have had) (?:a |such a )?(?:great|lovely|brilliant|good|amazing|wonderful|fantastic|smashing)\b"####
);
pattern!(
    GOOD_TIME_SUBJECT,
    r####"\b(?:everyone|everybody|all of them|(?:the|my|our|his|her|their|your|\w+'s?)(?:\s+\w+){0,2}\s+(?:party|wedding|meal|trip|holiday|gig|match|game|concert|night|day|weekend|evening|dinner|barbecue|bbq|bash|hen|stag))\b"####
);
pattern!(
    HARD_WEEK,
    r####"\b(?:fall|fell|fallen|tripped|collapsed|collapse|seizure|stroke|overdose|accident|crash|crashed|put down|passed away|in hospital|operation|surgery|stitches|broken|broke (?:my|his|her|their))\b"####
);
pattern!(
    A_CHILDHOOD_MEMORY,
    r####"\bwhen i was (?:\d+|little|young|a kid|a child|at school|small)\b"####
);
pattern!(
    A_VISIT,
    r####"\b(?:came (?:round|over|by|in)|come (?:round|over)|coming (?:round|over|by)|popped (?:round|in|over)|stopped by|visiting|visited|rang|phoned|called|texted|facetimed|stayed over)\b"####
);
pattern!(
    SOMEONE_ELSE_NAMED,
    r####"\b(?:my|our|his|her|their)\s+(?:mum|mam|mom|mother|dad|father|son|daughter|wife|husband|partner|boyfriend|girlfriend|brother|sister|mate|friend|boss|marriage|relationship|nan|nana|gran|granny|grandad|grandma|uncle|aunt|cousin|neighbour|neighbor|child|kid|baby|mother-in-law|father-in-law|in-laws|ex|ex-wife|ex-husband|ex-partner|ex-boyfriend|ex-girlfriend|flatmate|housemate|roommate|landlord|stepson|stepdaughter|stepmum|stepdad)\b|\b(?:someone|somebody|a man|a woman|a bloke|a lad)\b"####
);
pattern!(
    THEIR_OWN_DOING,
    r####"\b(?:i|im|i'm|ive|i've|we|we're|weve|we've)\b"####
);
pattern!(
    A_THING_GOT_AHEAD,
    r####"\b(?:i|we)(?:'?ve| have)?\s+(?:just\s+|finally\s+)?(?:got|booked|secured|bagged|landed|sorted)\s+(?:myself\s+|us\s+)?(?:a|an|the|my|our|two|three|four|some|another|front row|cheap)?\s*(?:audition|tickets?|viewing|viewings|booking|slot|reservations?|table booked)\b"####
);
pattern!(
    GOT_AHEAD_GONE,
    r####"(?i)\b(?:yesterday|last (?:night|week|month|year|saturday|sunday|monday|tuesday|wednesday|thursday|friday)|this morning|earlier|the other day|on (?:mon|tues|wednes|thurs|fri|satur|sun)day)\b"####
);
pattern!(
    GOT_AHEAD_NOT_A_DATE,
    r####"(?i)\b(?:parking|speeding|penalty|fine|fined|clamped|bus|train|plane|coach|tube|ferry|tram|lottery|raffle|on the way)\b"####
);
pattern!(
    PROBLEM_STATED,
    r####"^(?:so |ok |okay |right |and |hi |hiya |hello |hey |um |er |honestly |well |but )*(?:(?:i|we)(?:'?ve|'?m| have| am| got|'?ve got)\s+|there (?:is|'?s) |theres |its |it'?s )?(?:a |an |one )?(?:(?:bit of a|small|big|massive|real|proper|slight|little|serious|tiny|huge|nasty|right|bit of|wee|awkward) )?(?:problem|issue|trouble)(?: (?:here|really|actually|now|again|too|i (?:need|could do with) (?:some )?help(?: with)?|that i (?:need|could do with) (?:some )?help(?: with)?|i (?:need|could do with) (?:some )?advice))?$"####
);
pattern!(
    SOMETHING_UP,
    r####"^(?:so |ok |okay |right |and |hi |hello |hey |um |er |honestly |well |but )*(?:(?:there (?:is|'?s) |theres )?(?:(?:something|somethings|sommat|summat)(?: really| bad| weird| odd| serious| not right)?(?:'?s)? )?(?:is |was |keeps )?(?:wrong|up|not right|off|the matter)|(?:something|somethings|sommat|summat)\s+(?:has |had |just )?(?:happened|gone wrong|come up)|(?:something|somethings|sommat|summat)\s+(?:is |was |keeps )?(?:bothering|worrying|upsetting|bugging|nagging)\s+me)(?: (?:here|today|yesterday|tonight|last night|this morning|this week|at work|at home|again|right now|at the moment)){0,2}$"####
);
context_pattern!(
    MISHAP_ANYWHERE,
    r####"(?:\b(?:i|we|he|she|they|the\s+(?:baby|child|kid|boy|girl|little\s+one|toddler|old\s+lady|old\s+man)|(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|toddler|husband|wife|partner|boyfriend|girlfriend|brother|sister|nan|nana|gran|granny|grandad|grandpa|grandma|grandmother|grandfather|cousin|friend|mate|neighbour|neighbor|uncle|aunt|nephew|niece|little\s+one|lad|lass)|(?:my|our|his|her|their|the)\s+\d+\s*(?:month|week|year)s?\s*old)\b[^.?!]{0,24}?\b(?:fell|rolled|tumbled|toppled|tripped|slipped|went|come|came)\s+(?:(?:right|straight|back|clean|smack|flat|head)\s+)?(?:over|off|down|from|onto|into|through|out\s+of|on\s+the|down\s+the|off\s+the)\b|\b(?:i|we|he|she|they|the\s+(?:baby|child|kid|boy|girl|little\s+one|toddler|old\s+lady|old\s+man)|(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|toddler|husband|wife|partner|boyfriend|girlfriend|brother|sister|nan|nana|gran|granny|grandad|grandpa|grandma|grandmother|grandfather|cousin|friend|mate|neighbour|neighbor|uncle|aunt|nephew|niece|little\s+one|lad|lass)|(?:my|our|his|her|their|the)\s+\d+\s*(?:month|week|year)s?\s*old)\b[^.?!]{0,24}?\b(?:fell|rolled|tumbled|tripped|slipped|went)\s+on\s+the\s+(?:stairs|stair|steps|ice|pavement|paving|kerb|curb|path|floor|ground|road|decking|tiles|lino|linoleum|concrete|gravel|deck)\b|\b(?:i|we|he|she|they|the\s+(?:baby|child|kid|boy|girl|little\s+one|toddler|old\s+lady|old\s+man)|(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|toddler|husband|wife|partner|boyfriend|girlfriend|brother|sister|nan|nana|gran|granny|grandad|grandpa|grandma|grandmother|grandfather|cousin|friend|mate|neighbour|neighbor|uncle|aunt|nephew|niece|little\s+one|lad|lass)|(?:my|our|his|her|their|the)\s+\d+\s*(?:month|week|year)s?\s*old)\b[^.?!]{0,18}?\bhad\s+a\s+(?:bad\s+|nasty\s+|really\s+)?(?:fall|tumble|trip)\b|\b(?:i|we|he|she|they|the\s+(?:baby|child|kid|boy|girl|little\s+one|toddler|old\s+lady|old\s+man)|(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|toddler|husband|wife|partner|boyfriend|girlfriend|brother|sister|nan|nana|gran|granny|grandad|grandpa|grandma|grandmother|grandfather|cousin|friend|mate|neighbour|neighbor|uncle|aunt|nephew|niece|little\s+one|lad|lass)|(?:my|our|his|her|their|the)\s+\d+\s*(?:month|week|year)s?\s*old)\b[^.?!]{0,18}?\bfell\b(?=\s*(?:[.,!]|$|and\b|but\b)))|(?:\bi\b[^.?!]{0,20}?\b(?:stepped|stood|trod|walked|planted|knelt)\b[^.?!]{0,6}?\b(?:on|in(?:to)?)\b[^.?!]{0,6}?(?:a|an|some|the|broken|bits\s+of|a\s+bit\s+of|two)\s+(?:nail|nails|glass|needle|needles|lego|thorn|thorns|screw|screws|tack|tacks|pin|pins|shard|shards|spike|spikes|drawing\s+pin|sharp\s+thing|rusty\s+nail)\b|\b(?:got|had)\s+(?:a\s+)?(?:bit\s+of\s+|piece\s+of\s+|shard\s+of\s+)?(?:glass|grit|metal|a\s+nail|a\s+thorn)\s+(?:in|into)\s+my\b|\b(?:a\s+)?(?:nail|screw|thorn|needle|splinter|bit\s+of\s+glass)\s+(?:went\s+|stuck\s+)?(?:through|in(?:to)?)\s+my\b)|(?:\bmy\s+(?:vision|sight|hearing)\s+(?:has\s+|had\s+|suddenly\s+)?(?:gone|went)\b|\b(?:went|gone|going)\s+(?:blind|deaf)\b|\b(?:cant|cannot|can'?t)\s+see\s+(?:out\s+of|anything\s+out\s+of)\b)"####
);
pattern!(
    MISHAP_OVER,
    r####"\b(?:last\s+(?:year|summer|winter|spring|autumn|christmas|month)|years?\s+ago|when\s+i\s+was|as\s+a\s+(?:kid|child|teen|youngster|little\s+one)|used\s+to|back\s+in\s+(?:the\s+)?(?:day|\d{4}|nineteen|twenty))\b|\b(?:nearly|almost|close\s+to)\b|\b(?:but|and|though)\s+(?:he|she|they|it|we|i)\s*(?:'?s|re|is|was|were|am)?\s*(?:ok|okay|alright|all\s+right|fine)\b|\b(?:over|on)\s+(?:my|his|her|their|our|your|the)\s+(?:own\s+)?(?:words|tongue)\b"####
);
pattern!(
    SENSE_GONE,
    r####"\bmy\s+(?:vision|sight|hearing)\s+(?:has\s+|had\s+|suddenly\s+)?(?:gone|went)\b|\b(?:went|gone|going)\s+(?:blind|deaf)\b|\b(?:cant|cannot|can'?t)\s+see\s+(?:out\s+of|anything\s+out\s+of)\b"####
);
pattern!(
    A_ASKING_WORD,
    r####"(?i)^(?:did|do|does|have|has|had|are|is|was|were|can|could|would|will|should|shall|what|whats|how|why|when|where|who|which|any)\b"####
);
pattern!(
    A_NEWS_IS_A_CHANGE,
    r####"(?i)\b(?:clos(?:e|es|ed|ing|ure)|cancel\w*|chang(?:e|es|ed|ing)|charg(?:e|es|ed|ing)|rules?|ban(?:s|ned|ning)?|cuts?|ris(?:e|es|ing)|increas(?:e|es|ed|ing)|delay\w*|roadworks?|strikes?|walkout|fees?|prices?|tax(?:es)?|fines?|shut|new\s+(?:rules|charges|prices|fares)|from\s+(?:monday|next\s+week|january|april))\b"####
);
pattern!(
    A_NEWS_IS_WEATHER,
    r####"(?i)\b(?:storm|storms|gale|gales|wind|winds|windy|rain|snow|ice|sleet|flood|floods|freeze|freezing|frost|heatwave|thunder|lightning|hurricane|tornado|weather)\b"####
);
pattern!(
    A_REPORTED_NOT_LIVED,
    r####"(?i)\b(?:i|we)\s+(?:saw|seen|heard|read|watched)\s+(?:that\b|there(?:'s|s| is| are)\b|on\s+the\s+\w+|in\s+the\s+\w+|the\s+\w+|somebody\b|someone\b|they\b)|\b(?:on|in)\s+the\s+(?:news|paper|radio|telly|local news)\b|\bthey\s+say\b|\bive\s+been\s+told\b|\b(?:says|said|told\s+me)\s+(?:that\b|there(?:'s|s| is| are)\b)"####
);
pattern!(
    BARE_STATE,
    r####"\b(?:was|were|been|is|are|am|im|i'm|its|it's)\b"####
);
pattern!(
    STATE_NOT_A_DOING,
    r####"\b(?:self[- ]?employed|unemployed|retired|married|divorced|engaged|separated|pregnant|based|interested|tired|exhausted|worried|bothered|pleased|delighted|annoyed|scared|frightened|settled|sorted|done|on a diet|on benefits|in debt|in credit|on the dole|a vegetarian|a vegan|on medication|on tablets)\b"####
);
pattern!(
    BOT_WAS_SORRY,
    r####"(?i)\b(?:sorry|that sounds (?:rough|awful|horrible|hard|heavy|tough)|that is (?:rough|awful|hard|heavy|tough)|that's (?:rough|awful|horrible|hard|heavy|tough)|rough one|hard one)\b"####
);
pattern!(
    _A_BODY_UNWELL,
    r####"\b(?:ill|illness|unwell|poorly|sick|infection)\b"####
);
pattern!(
    _A_DETAIL_OPENING,
    r####"^(?:it|that|this|there)\s+(?:was|were|is|had|happened)\b"####
);
pattern!(
    _A_SYMPATHY,
    r####"\b(?:sorry|condolences|sad news|real loss)\b"####
);
pattern!(
    A_DAY_SAID_BADLY,
    r####"\b(?:bad day|bad one|rough day|rough one|rubbish day|rubbish|awful day|horrible day|terrible day|hard day|hard one|long day|tough day|tough one|strange day|grim|draining|it kicked off|wiped out|hectic|full on|knackered|exhausted|shattered|gutted|a row|an argument)\b"####
);
pattern!(
    A_GROUP_OF_PEOPLE,
    r####"\b(?:my|our|the|his|her|their)\s+(?:team|side|club|lot|lads|boys|girls|mates|friends|family|kids|children|crowd|opposition|squad|band)\b"####
);
pattern!(
    A_NIGHT_OUT,
    r####"\b(?:party|pub|night\s+out|works\s+do|wedding|stag|hen|club|bar|drinks|round|celebration)\b"####
);
pattern!(SMASHED, r####"\bsmashed\b"####);
pattern!(
    SMASHED_OF_A_THING,
    r####"\b(?:my|our|the|his|her|their|its)\s+(?:\w+\s+){0,3}?(?:got|was|were|has\s+been|had\s+been|smashed|broken|shattered)\b"####
);
pattern!(
    A_PERSON_LOST,
    r####"\b(?:went\s+missing|gone\s+missing|disappeared|vanished)\b"####
);
pattern!(
    A_PERSON_OF_THEIRS,
    r####"\b(?:my|our)\s+(?:boss|manager|mum|mom|mother|dad|father|son|daughter|boy|girl|kid|kids|child|children|husband|wife|partner|boyfriend|girlfriend|ex|brother|sister|mate|mates|friend|friends|colleague|colleagues|coworker|coworkers|nana|nan|gran|grandad|grandma|grandpa|aunt|uncle|cousin|neighbour|neighbor|landlord|landlady|flatmate|housemate|teenager|baby|toddler)\b"####
);
pattern!(
    A_THING_CARRIED_OFF,
    r####"\b(?:ran|ran\s+off|ran\s+away|got\s+out|escaped|went\s+off|cracked|smashed|broke)\s+(?:with|out)\s+(?:my|our|the)\b|\b(?:cracked|smashed|broke|dented|scratched|chipped)\s+(?:open\s+)?(?:my|our)\b"####
);
pattern!(
    CRACKED_A_JOKE,
    r####"\bcracked\s+(?:a|up|his|her|their|its)\s*(?:joke|smile|grin|laugh|voice|wise)|\bcracked\s+(?:on|it)\b"####
);
pattern!(
    DONE_TO_ME,
    r####"\b(?:got|was|were|has been|have been|had been)\s+(?:\w+\s+){0,2}?(?:pickpocketed|pick-?pocketed|keyed|scratch(?:ed)?|dented|chipped|cracked|smashed|slashed|vandalised|vandalized|damaged|towed|clamped)\b|\b(?:my|our)\s+(?:\w+\s+){0,3}?(?:ran off|ran away|bolted|escaped|got out|wandered off|went missing|gone missing|disappeared|vanished|cracked|stopped working|packed in)\b|\bi\s+(?:chipped|cracked|broke|knocked out|lost)\s+(?:a|an|my|one of my|two)\s+\w+\b"####
);
pattern!(
    THEIR_NAME,
    r####"^my (?P<person>(?:mum|mom|mam|mother|dad|father|parents|sister|brother|son|daughter|kid|kids|child|children|boy|girl|eldest|youngest|baby|baby boy|baby girl|little one|little boy|little girl|little man|little lad|little miss|oldest|firstborn|toddler|gran|granny|nan|grandma|grandmother|grandad|grandpa|grandfather|grandparents|nana|aunt|uncle|cousin|partner|husband|wife|niece|nephew|grandson|granddaughter|grandchildren|grandkids|grandsons|granddaughters|stepson|stepdaughter|folks|inlaws|in-laws|friend|friends|mate|mates|best friend|best mate|boss|colleague|workmate|neighbour|neighbor|neighbours|neighbors|flatmate|housemate|roommate))(?:'?s)? (?:is called|is named|name is|name's) (?P<name>[a-z][a-z'-]{1,14})$"####
);
pattern!(
    THEIR_NAME_AGE,
    r####"^my (?:(?:other|eldest|oldest|youngest|little|big|small|wee|first|second|third|middle|baby|grown up|grownup|step|adopted|foster) )?(?P<person>(?:son|daughter|kid|kids|child|children|boy|girl|eldest|youngest|niece|baby|baby boy|baby girl|little one|little boy|little girl|little man|little lad|little miss|oldest|firstborn|toddler|nephew|grandson|granddaughter|grandkids|grandchildren|stepson|stepdaughter|baby|babies|toddler|toddlers|newborn|newborns|infant|infants|little one|little lad|little man|little fella|little fellow|little girl|little boy|wee one|wee man|wee lad|small one))(?:'?s)? (?:is called|is named|name is|name's|called|named) (?P<name>[a-z][a-z'-]{1,14})(?: and |, )?(?:she|he|they|it)(?:'?s| is| are) (?:just |only |nearly )?(?P<age>\d{1,2})$"####
);
pattern!(
    A_MEDS,
    r####"(?i)\b(?:meds|medication|medications|medicine|medicines|tablets?|pills?|capsules?|dose|doses|dosage|prescription|inhaler|inhalers|insulin|antidepressants?|antibiotics?|painkillers?|statins?|steroids?|hormones?|patch|patches|injections?)\b"####
);
pattern!(
    A_MEDS_CHANGE,
    r####"(?i)\b(?:changed|change|changes|changing|new|started|starting|start|switched|switch|moved|put me on|now on|instead of|upped|increased|lowered|reduced|stopped|come off|came off|different|put on|been put|gone up|gone down|went up|went down|twice a day|once a day|three times a day|swapped|swap|halved|halve|doubled|higher|lower|stronger|weaker)\b"####
);
pattern!(
    A_MEDS_MINE,
    r####"(?i)^(?:so |and |but |ok |okay |well |anyway |yeah |yeh |also )*(?:my\s+(?:\w+\s+)?(?:meds|medications?|medicines?|tablets?|pills?|dose|dosage|prescription|inhaler|insulin|antidepressants?|antibiotics?|painkillers?|statins?|steroids?|hormones?|patch|patches|injections?)|i(?:'?m| am)\s+(?:now\s+)?(?:on|taking)|ive|i(?:'?ve)?\s+been\s+(?:put\s+)?on|i\s+(?:started|start|switched|changed|stopped|come off|came off)|i\s+take\b|i(?:'?ve|\s+have|\s+got)\s+(?:some\s+)?(?:new\s+|different\s+|another\s+)?(?:meds|medication|medicines?|tablets?|pills?|capsules?|inhaler|insulin)(?:\s+to\s+take)?|(?:the|my)\s+(?:doctor|gp|consultant|nurse|psychiatrist)\s+(?:has\s+)?(?:changed|put me on|switched|started me|gave me)|i(?:'?m| am)\s+\w+\s+and\s+my\s+\w+\s+(?:have|has|was|were|is)\s+\w+ed|(?:they|he|she|the\s+(?:doctor|gp|consultant|nurse|psychiatrist|chemist|pharmacist|hospital|surgery))\s+(?:have\s+|has\s+)?(?:changed|switched|swapped|put me on|started me on|given me|gave me|moved me))"####
);
pattern!(
    A_MEDS_SYMPTOM,
    r####"(?i)\b(?:wheez\w*|breath\w*|chest|bleed\w*|rash|swollen|swelling|pain|hurts?|hurting|dizzy|faint\w*|sick|vomit\w*|temperature|fever|numb|collapse\w*|seizure|fit|too many|double|twice the|overdose|took \d|swallow\w*)\b"####
);

pub(super) const GRIEVANCE_PEOPLE: &[&str] = &[
    "boss",
    "boyfriend",
    "brother",
    "colleague",
    "coworker",
    "dad",
    "ex",
    "exes",
    "father",
    "flatmate",
    "friend",
    "friends",
    "girlfriend",
    "housemate",
    "husband",
    "in-laws",
    "landlady",
    "landlord",
    "manager",
    "mom",
    "mother",
    "mum",
    "neighbor",
    "neighbour",
    "parent",
    "parents",
    "partner",
    "people",
    "roommate",
    "sister",
    "somebody",
    "someone",
    "wife",
];

pub(super) const NOT_A_VERDICT_SUBJECT: &[&str] = &[
    "a&e",
    "ambulance",
    "anaesthetic",
    "anaesthetics",
    "ashes",
    "burial",
    "carer",
    "carers",
    "casket",
    "casualty",
    "cemetery",
    "chapel",
    "chemo",
    "chemotherapy",
    "clinic",
    "condolences",
    "consultant",
    "counsellor",
    "counsellors",
    "cremation",
    "crematorium",
    "dialysis",
    "doctor",
    "doctors",
    "eulogies",
    "eulogy",
    "firefighters",
    "firemen",
    "flowers",
    "funeral",
    "graveside",
    "hearse",
    "hospice",
    "hospital",
    "hymn",
    "hymns",
    "icu",
    "itu",
    "mass",
    "matron",
    "memorial",
    "midwife",
    "nurse",
    "nurses",
    "obituary",
    "paramedic",
    "paramedics",
    "physio",
    "physiotherapist",
    "police",
    "reading",
    "readings",
    "results",
    "rosary",
    "scan",
    "send-off",
    "sendoff",
    "sermon",
    "service",
    "services",
    "servicio",
    "staff",
    "surgeon",
    "surgery",
    "tests",
    "theatre",
    "tribute",
    "tributes",
    "vet",
    "vets",
    "vigil",
    "wake",
    "ward",
];

pub(super) const GOOD_TIME_WORDS: &[&str] = &[
    "amazing",
    "brilliant",
    "class",
    "crackin",
    "delicious",
    "fantastic",
    "fun",
    "good",
    "gorgeous",
    "great",
    "lovely",
    "memorable",
    "nice",
    "smashing",
    "superb",
    "tasty",
    "wonderful",
];

pub(super) const A_BARE_MODIFIER: &[&str] = &[
    "a", "again", "all", "already", "also", "an", "anyway", "as", "at", "back", "before", "either",
    "even", "for", "i", "in", "instead", "it", "just", "me", "my", "myself", "now", "of", "on",
    "our", "really", "so", "still", "the", "then", "though", "to", "too", "we", "well", "with",
    "yet",
];

pub(super) const A_BAD_TIME_TOLD: &[&str] = &[
    "Sounds like a rough one. What happened?",
    "Ugh, that is a shame. Do you want to get into it?",
    "That is a nuisance. Is there anything that would help?",
    "How are you doing with it?",
    "Sorry to hear it. What would make today easier?",
];

pub(super) const A_DAY_ASKED_PLAIN: &[&str] = &[
    "How did that go, overall?",
    "What was it like?",
    "How did you find it?",
];

pub(super) const A_PLAN_THAT_FELL_THROUGH: &[&str] = &[
    "That's a shame. Is it off altogether, or moved?",
    "Gutted. What got in the way?",
    "Ah no. What stopped it?",
    "Sorry to hear it. Can it still happen?",
    "That is a shame. Do you get another go at it?",
];

pub(super) const A_THING_CAME_OUT_WRONG: &[&str] = &[
    "That's a shame. What went wrong with it?",
    "Ah no. Can it be rescued, or is it a start again?",
    "Annoying when that happens. What was it you made?",
    "Gutted. Had you made it before?",
    "That is a nuisance. Do you know what did it?",
];

pub(super) const LOW_DAY_REPLIES: &[&str] = &[
    "That sounds like a day to get through rather than enjoy. What made it that way?",
    "Sorry - that is a heavy one to carry. What has been the worst part of it?",
    "Long days like that take it out of you. Do you want to talk it through, or would you rather think about something else for a bit?",
    "That is a rough one. Is it the kind that lifts in a day or two, or one that has been building?",
    "Ugh. What happened?",
];

pub(super) const NEGATIVE: &[&str] = &[
    "That sounds rough. If you want, I can help you think through what to do next.",
    "Ugh, that's a lot to carry. What would make today a bit easier?",
    "I'm sorry — that's frustrating. Want a hand working out the next step?",
    "Sorry to hear that. What would help most right now?",
];

pub(super) const A_SMALL_DAY: &[&str] = &[
    "Sounds alright. Anything good in it?",
    "Nice one. How did the rest of it go?",
    "Good to hear. What was the best bit?",
    "How was it, overall?",
];

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
    A_DOING_GONE_BADLY,
    r####"\b(?:badly|too\s+much|too\s+many|too\s+often|not\s+enough|nothing\s+but|binged|overdid\s+it|junk|crap|rubbish)\b"####
);

pattern!(
    A_THING_SKIPPED,
    r####"\bskipped\b(?:\s+\w+){0,4}\s+(?:all\s+(?:week|month|weekend)|this\s+week|last\s+week|lately|every\s+(?:day|morning|week)|again)\b"####
);

pattern!(
    A_MEAL_SKIPPED,
    r####"\bskipped\b(?:\s+\w+){0,2}\s+(?:a\s+|any\s+|my\s+|the\s+)?(?:breakfast|lunch|lunches|dinner|dinners|tea|supper|meals?|food)\b"####
);
