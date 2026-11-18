// Pattern data from the pinned compose/fit.py and compose/lead.py (Apache-2.0).

complex_pattern!(NAMED_CLOCK, concat!(
    r#"(?i)(?:\b(?:at|by|from|until|till|before|after|around|about|on)\s+(?:\d{1,2}[:.]\d{2}"#,
    r#"|\d{1,2}\s*(?:am|pm|am\.|pm\.|a\.m|p\.m|o'clock)|(?:the\s+)?\d{1,2}(?:st|nd|rd|th)\b"#,
    r#"|(?:the\s+)?\d{1,2}\s+of\s+(?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)|(?:half|quarter|ten"#,
    r#"|twenty|five|twenty-five)\s+(?:past|to)\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven"#,
    r#"|twelve)(?:\s*(?:am|pm|a\.m|p\.m))?)|\b(?:at|by|from|until|till|before|after)\s+(?:(?:one|two|three"#,
    r#"|four|five|six|seven|eight|nine|ten|eleven|twelve)(?:\s*(?:am|pm|a\.m|p\.m))?(?!\s+of\b)|(?:[01]?\d"#,
    r#"|2[0-4])(?![\d:])(?!\.\d)(?!\s*[-\u2013\u2014]\s*\d)(?!\s*(?:%|percent|per cent|g|kg|ml|l|p|pence"#,
    r#"|bar|cm|mm|km|m|ft|lb|oz|st|deg|metres?|meters?|miles?|yards?|feet|foot|litres?|liters?|grams?"#,
    r#"|kilos?|calories|kcal|acres?|grit|uses?|square|long|wide|tall)\b))|(?:half|quarter|ten|twenty|five"#,
    r#"|twenty-five)\s+(?:past|to)\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven"#,
    r#"|twelve)(?:\s*(?:am|pm|a\.m|p\.m))?)"#,
));

complex_pattern!(EDGED_PART_OF_DAY, concat!(
    r#"(?i)\b(?:mid|early|late)[- ]?(?:morning|afternoon|evening|night)\b|\b(?:first"#,
    r#"|last) thing\b(?!\s+(?:a|an|the|it|i|you|we|they|he|she|that|this|of|about)\b)|\bfirst light\b"#,
    r#"|\bdaytime\b|\b(?:at|by|before|after|until|till|around|about) (?:dawn|dusk|nightfall|sunset|sunrise"#,
    r#"|daybreak)\b|\bend of the (?:day|afternoon|evening)\b"#,
));

pattern!(RATE_PHRASE, concat!(
    r#"(?i)^(?:once|twice|(?:three|four|five|six|\d+)\s+times|every (?:\w+ )?(?:day|week|month|year|morning"#,
    r#"|evening|night)|daily|weekly|monthly|yearly|nightly)(?:\s+(?:a|an|per|each|every)\s+(?:day|week"#,
    r#"|month|year|morning|evening|night))?"#,
));

pattern!(RATE_OF_A_COUNT, concat!(
    r#"(?i)\b\d[\d,.]*\s*(?:cal|kcal|calorie|step|rep|unit|mile|km|kilomet|metre"#,
    r#"|meter)s?\b(?:\s+\w+){0,2}\s+(?:a|an|per|each|every)\s+(?:day|week|month|year|morning|evening"#,
    r#"|night)\b"#,
));

pattern!(NO_FIXED_TIME, r#"(?i)^(?:no|there'?s no|there is no) (?:fixed|set|exact|hard and fast) (?:time|length|deadline)\b"#);
pattern!(DURATION, concat!(
    r#"(?i)(?:(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen"#,
    r#"|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty"#,
    r#"|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third|a fifth|double|dozen)"#,
    r#"|\ba\b|\ban\b)[\s-]*(?:(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven"#,
    r#"|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty"#,
    r#"|sixty|seventy|eighty|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third"#,
    r#"|a fifth|double|dozen)[\s-]*)?(?:(?:working|clear|business|banking|bank|calendar|school|full|whole"#,
    r#"|solid|straight|extra|more|further)\s+)?(?:sec|second|min|minute|hr|hour|day|week|fortnight|month"#,
    r#"|year|night|morning|afternoon|evening)s?\b|\bovernight\b|\ball day\b|\ball night\b|\bfor a bit\b"#,
    r#"|\bwithin the hour\b|\bsoon\b|\b(?:for|in) (?:days|weeks|months|years)\b|\ba (?:full|whole) (?:hour"#,
    r#"|day|week|month|year)s?\b|\b(?:a couple of|a pair of|a number of|a handful of|several|many"#,
    r#"|numerous)\s+(?:sec|second|min|minute|hr|hour|day|week|fortnight|month|year|night|morning|afternoon"#,
    r#"|evening)s?\b|\bten minutes\b|\bnext day\b|\b(?:a )?couple of (?:secs?|seconds?|mins?|minutes?"#,
    r#"|hours?|days?|weeks?|months?|years?|nights?)\b"#,
));

complex_pattern!(CLOCK, concat!(
    r#"(?i)\b(?:mon|tues|wednes|thurs|fri|satur|sun)day\b|\b(?:january|february|april|june|july|august"#,
    r#"|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sept|sep|oct|nov|dec)\b\.?"#,
    r#"|\b(?-i:May|March)\b\.?|\b(?:may|march)\b\.?\s+\d{1,2}\b|\b\d{1,2}\s*(?:am|pm)\b|\bo'clock\b"#,
    r#"|\bmidday\b|\bmidnight\b|\btonight\b|\btomorrow\b|\b(?:same|next|this|that) (?:morning|afternoon"#,
    r#"|evening|night)\b|\b(?:at|by|until|till|before|after|from) \d{1,2}[.:]\d{2}(?![.:]?\d)|\b(?:in|since"#,
    r#"|until|till|from|by|during|after|before) (?:\d{3,4}s?\b|the \d{3,4}s?\b)(?!\s*(?:g|kg|ml|l|p|pence|%"#,
    r#"|bar|cm|mm|km|m|ft|lb|oz|st|deg|grit|uses?|pages?|words?|steps?|calories|kcal|people|guests?"#,
    r#"|servings?|slices?|eggs?|metres?|meters?|miles?|yards?|feet|foot|litres?|liters?|grams?|kilos?"#,
    r#"|acres?|square|long|wide|tall)\b)|\b\d{3,4}\s*(?:ad|bc|b\.c\.)\b|\b(?:1[0-9]{3}|20[0-9]{2})\s*(?:-"#,
    r#"|to|or|and)\s*(?:1[0-9]{3}|20[0-9]{2})\b|\bthe \d{1,2}(?:st|nd|rd|th)\b|\b\d{1,2}(?:st|nd|rd"#,
    r#"|th) of (?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)|\b(?:spring|summer|autumn|winter)\b"#,
));

pattern!(BARE_RANGE, concat!(
    r#"(?i)^[^\w]{0,3}(?:about |around |roughly )?(?:\d{1,3}|(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six"#,
    r#"|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen"#,
    r#"|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand|million|a couple|couple"#,
    r#"|a few|half|a quarter|a third|a fifth|double|dozen))\s*(?:-|to|or|\u2013)\s*(?:\d{1,3}"#,
    r#"|(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen"#,
    r#"|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety"#,
    r#"|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third|a fifth|double|dozen))\b"#,
));

pattern!(STAGE, concat!(
    r#"(?i)\b(?:for|in|over|through|during) the first (?:few |couple of |several )?(?:days|weeks|months"#,
    r#"|year|bit)\b|\bthe first (?:few |couple of )?(?:days|weeks|months)\b|\b(?:first|1st) birthday\b"#,
    r#"|\buntil (?:they|he|she) (?:are|is) \w+\b"#,
));

pattern!(NEVER, concat!(
    r#"(?i)\b(?:indefinitely|(?:does not|doesn't|do not|don't|will not|won't|never) (?:go(?:es)? off|expire"#,
    r#"|spoil|perish)|(?:last|lasts|keep|keeps|stay|stays|good) (?:good )?(?:for ever|forever"#,
    r#"|indefinitely))\b"#,
));

pattern!(MINUTES_AS_A_LENGTH, concat!(
    r#"(?i)\b(?:within|for)\s+(?:a few|a couple of|several|a|one|two|three|four|five|ten)\b[\s-]*(?:min"#,
    r#"|minute)s?\b|\ba few (?:min|minute)s\b|\bwithin (?:a few |a couple of )?(?:min|minute)s\b"#,
));

pattern!(INSTRUCTS, concat!(
    r#"(?i)^(?:open|book|ring|call|phone|top|wipe|soak|give|leave|put|take|turn|run|use|set|check|clean"#,
    r#"|wash|rinse|melt|stir|mix|fry|boil|bake|roast|grill|simmer|add|pour|spread|cut|chop|slice|peel|grate"#,
    r#"|whisk|beat|knead|roll|press|heat|cook|drain|freeze|cool|cover|store|keep|throw|bin|recycle|replace"#,
    r#"|repair|fit|fix|install|service|bleed|flush|tie|wrap|apply|ask|tell|send|email|visit|go|start|finish"#,
    r#"|stop|avoid|make|buy|order|measure|weigh|label|file|register|report|complain|sign|write|read|watch"#,
    r#"|listen|carry|hold|stand|sit|wait|test|taste|smell|look|choose|pick|swap|follow|keep|get|bring|put"#,
    r#"|note|note down|squeeze|rub|scrape|buff|dab|blot|scoop|trim|mist|sweep|pat|fold|split|delay|aim|try"#,
    r#"|hook|wedge|prop|secure|tape|glue|batch|preheat|defrost|pick over|sort out|go through|block|treat"#,
    r#"|air|cancel|renew|practise|practice)\b"#,
));

pattern!(NAMED_TIME, concat!(
    r#"(?i)\b(?:before|after|until|till|by|at|on|from|in|over|during)\s+(?:the\s+)?(?:first|second|third"#,
    r#"|next|last|that|their|his|her)?\s*(?:birthday|christmas|easter|new year|weekend|weekday|morning"#,
    r#"|afternoon|evening|night|breakfast|lunch|teatime|tea|dinner|supper|bedtime|school|school night|work"#,
    r#"|the day|arrival|takeoff|checkout|check-in)\b"#,
));

pattern!(DAY_NAME, r#"(?i)\b(?:christmas|easter|new year'?s?|bank holiday|birthday|half term|term time|the weekend)\b"#);
complex_pattern!(DATE, concat!(
    r#"(?i)(?:\b(?:in|from|since|until|till|by|of|during|between|after|before|around|about|circa|early|late"#,
    r#"|mid|and|to)\s+(?:the\s+)?|^)(?:1[0-9]{3}|20[0-2][0-9])\b(?!\s*(?:cal|kcal|calorie|step|mile|km"#,
    r#"|kilomet|metre|meter|gram|g\b|kg|ml|litre|liter|watt|lumen|gb|mb|megabyte|pound|dollar|euro|per cent"#,
    r#"|%)\w*)(?!\s+[a-z]{2,}s\b)|\b(?:in|from|since|during|of|the|by|until|early|late|mid)\s+(?:\d{4}s"#,
    r#"|\d{1,2}(?:st|nd|rd|th) century)\b"#,
));

pattern!(MONEY_RATE, concat!(
    r#"(?i)(?:[£$€]\s*\d[\d,.]*|\b\d[\d,.]*\s*(?:p|pence|pennies|quid|pounds?|bucks?|dollars?|euros?|cents?"#,
    r#"|gbp)\b)(?:\s*(?:to|-)\s*(?:[£$€]\s*)?\d[\d,.]*\s*(?:p|pence)?)?\s*(?:a|an|per|each)\s+(?:sec\w*"#,
    r#"|min\w*|hour|hr|hrs|day|week|month|year|night|visit|call|session|unit)s?\b"#,
));

pattern!(STAGE_WHEN, concat!(
    r#"(?i)(?:(?:\bwhen|\bas soon as)\s+(?:the|it|its|they|they're|there|a|your|their)\b|\bonce\s+(?:the|it"#,
    r#"|its|they|they're|there|your|their)\b)[^.?!]{0,70}?\b(?:has|have|had|is|are|was|were|shows?|showing"#,
    r#"|sets?|set|starts?|started|reaches?|reached|begins?|begun|appears?|emerges?|sprouts?|sprouted"#,
    r#"|flowered|flowers|warms?|warmed|workable|cooled|cools|dried|dries|firm|ready|grown|thickened"#,
    r#"|risen)\b"#,
));

pattern!(SEASON_WHEN, concat!(
    r#"(?i)\b(?:in|during|through|throughout|into|over)\s+(?:the\s+)?(?:growing|dormant|bare|planting"#,
    r#"|flowering|nesting|mating|wet|dry|rainy)\s+season\b|\b(?:the\s+)?growing months?\b|\b(?:in"#,
    r#"|during)\s+(?:a\s+|the\s+)?(?:dry|wet|cold|warm)\s+spells?\b"#,
));

pattern!(ASKS_WHEN, r#"(?i)^[\W]*(?:and |so |but |then )?(?:when|what time|what day|what date|what month|what year|which day)\b"#);
pattern!(EXTENT, concat!(
    r#"(?i)(?:(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen"#,
    r#"|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty"#,
    r#"|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third|a fifth|double|dozen)"#,
    r#"|\ba\b|\ban\b)[\s-]*(?:[a-z]+[\s-]+){0,3}(?:km|kilometres?|kilometers?|miles?|metres?|meters?|yards?"#,
    r#"|feet|foot|inches?|cm|mm|centimetres?|centimeters?|millimetres?|millimeters?|pages?|words?|lines?"#,
    r#"|songs?|tracks?|episodes?|chapters?|metres?|m)\b"#,
));

pattern!(LENGTH_ASKED, concat!(
    r#"(?i)\bhow (?:long|wide|tall|deep|high|far) (?:is|are|was|were)\b|\bhow (?:long|wide|tall|deep|high"#,
    r#"|far) (?:does|do|did) (?:[\w'-]+ ){0,3}(?:run|runs|stretch|stretches|measure|measures|reach|reaches"#,
    r#"|extend|extends|go|goes)\b"#,
));

pattern!(ASKS_AN_AGE, r#"(?i)\bwhat age\b|\bhow old\b"#);
pattern!(AN_AGE_YEARS, concat!(
    r#"(?i)(?:\d+|(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve"#,
    r#"|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty"#,
    r#"|seventy|eighty|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third|a fifth"#,
    r#"|double|dozen))\s*(?:-\s*)?years?\s*(?:old|of age)\b|\b(?:age|aged)\s+(?:of\s+)?(?:\d+"#,
    r#"|(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen"#,
    r#"|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety"#,
    r#"|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third|a fifth|double"#,
    r#"|dozen))\s*years?\b|^(?:\d+|(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten"#,
    r#"|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty"#,
    r#"|fifty|sixty|seventy|eighty|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter"#,
    r#"|a third|a fifth|double|dozen))\s*years?\b"#,
));

pattern!(AN_AGE_MONTHS, concat!(
    r#"(?i)(?:\d+|(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve"#,
    r#"|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty"#,
    r#"|seventy|eighty|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third|a fifth"#,
    r#"|double|dozen))\s*(?:-\s*)?(?:months?|weeks?)\s*old\b|\b(?:about|around|at|by|from|until|till|under"#,
    r#"|over|aged?)\s+(?:\d+|(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven"#,
    r#"|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty"#,
    r#"|sixty|seventy|eighty|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third"#,
    r#"|a fifth|double|dozen))\s*(?:months?|weeks?)\b|^(?:\d+|(?:\d+(?:[.,]\d+)?|one|two|three|four|five"#,
    r#"|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen"#,
    r#"|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand|million|a couple"#,
    r#"|couple|a few|half|a quarter|a third|a fifth|double|dozen))\s*(?:months?|weeks?)\b"#,
));

pattern!(AN_AGE_BIRTHDAY, r#"(?i)\b(?:\d+(?:st|nd|rd|th)|[a-z]+(?:st|nd|rd|th))\s+birthday\b"#);
pattern!(AN_AGE_NAMED, concat!(
    r#"(?i)\b(?:age|aged)\s+(?:\d+|(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten"#,
    r#"|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty"#,
    r#"|fifty|sixty|seventy|eighty|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter"#,
    r#"|a third|a fifth|double|dozen))\b"#,
));

complex_pattern!(AN_AGE_BARE, concat!(
    r#"(?i)(?<![£$€\d.,])\b\d{1,2}\b(?![.,]?\d)(?!\s*(?:%|minutes?\b|mins?\b|hours?\b|hrs?\b|days?\b"#,
    r#"|pounds?\b|pence\b|quid\b|times?\b|per\b|each\b|of them\b))"#,
));

complex_pattern!(AN_AGE_SPELLED_OPENS, concat!(
    r#"(?i)^(?:(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen"#,
    r#"|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty"#,
    r#"|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third|a fifth|double"#,
    r#"|dozen))\b(?=\s*(?:,|to\b|is\b|or\b|and\b|-|\u2013))"#,
));

complex_pattern!(FIGURE, concat!(
    r#"(?i)\d|\b(?!one of\b)(?:\d+(?:[.,]\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven"#,
    r#"|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty"#,
    r#"|sixty|seventy|eighty|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third"#,
    r#"|a fifth|double|dozen)\b"#,
));

pattern!(MONEY, concat!(
    r#"(?i)[£$€]|\b(?:pence|pounds?|quid|dollars?|euros?|cents?|free)\b|^(?:nothing|nil|nowt)\b"#,
    r#"|\bnothing (?:at all|extra|much|to pay|else|on top)\b|\bcosts? nothing\b"#,
));

pattern!(RATE, concat!(
    r#"(?i)\b(?:every (?:day|week|month|year|morning|evening|night|other (?:day|morning|evening|night))"#,
    r#"|every few (?:days|weeks)|most (?:days|mornings|evenings|nights|weekends)|once|twice|three times"#,
    r#"|daily|weekly|monthly|yearly|nightly|\d+ times)\b"#,
));

pattern!(PRICE_ASKED, concat!(
    r#"(?i)\b(?:cost|costs|priced?|prices?|pay|pays|paid|charged|charges? for|afford|fees?|quote|salary"#,
    r#"|rent)\b|[\u00a3$\u20ac]|\b(?:pounds|quid|euros?|dollars?|pence|tenner)\b"#,
));

pattern!(PRICE_GIVEN, concat!(
    r#"(?i)[\u00a3$\u20ac]|\b\d+p\b|\b(?:pounds|quid|pence|euros?|dollars?|cents?|free|costs?|prices?"#,
    r#"|charges?|fees?|paid|pay|spend|budget|afford|quote|refund|salary|rent|tenner)\b"#,
));

pattern!(CALENDAR_WORD, concat!(
    r#"(?i)\b(?:in|on|of|by|until|till|from|during|after|before|early|late|mid|and"#,
    r#"|to)\s+(?:the\s+)?(?:january|february|april|june|july|august|september|october|november|december|jan"#,
    r#"|feb|mar|apr|jun|jul|aug|sept|sep|oct|nov|dec)\b\.?|\b(?:in|on|of|by|until|till|from|during|after"#,
    r#"|before|early|late|mid)\s+(?-i:May|March)\b\.?|\b(?:mon|tues|wednes|thurs|fri|satur|sun)day\b\.?"#,
    r#"|\b\d{1,2}\s*(?:st|nd|rd|th)\b"#,
));

pattern!(PLACE_WORD, concat!(
    r#"(?i)\b(?:shop|store|supermarket|chemist|pharmac|post office|bank|building society|council|\bgp\b"#,
    r#"|doctor'?s|surgery|dentist|library|garage|station|market|website|web site|site|app|online|branch"#,
    r#"|office|centre|center|clinic|hospital|recycling|tip\b|dump|high street|department|service desk|desk"#,
    r#"|reception|kiosk|counter|aisle|dealer|showroom|garden centre|petrol station|airport|bus stop"#,
    r#"|platform|address|tesco|asda|sainsbury|aldi|lidl|morrisons|co-op|boots|argos|screwfix|ikea|amazon"#,
    r#"|ebay|gov\.uk|passport office|dvla|jobcentre|citizens advice|charity shop|car boot"#,
    r#"|facebook marketplace|freecycle|wherever|downstairs|upstairs|outside|indoors|in the garden"#,
    r#"|in the kitchen|in the loft|in the shed|drawer|cupboard|cabinet|wardrobe|shelf|shelves|rack|hook|peg"#,
    r#"|locker|safe|pocket|handbag|backpack|rucksack|suitcase|glove ?box|boot of the car|hallway|corridor"#,
    r#"|landing|garage|shed|loft|attic|basement|cellar|balcony|porch|kitchen|bedroom|bathroom|toilet"#,
    r#"|utility room|airing cupboard|under the sink|in the cupboard|in the drawer|on the shelf"#,
    r#"|in the fridge|in the freezer|in the bin|on the wall|on the door|in the hall|by the door"#,
    r#"|at the back of|at the front of|on top of|behind the|under the|next to the|beside the"#,
    r#"|inside the)(?:s|es)?\b"#,
));

complex_pattern!(PLACE_NAME, concat!(
    r#"\b(?:in|at|to|from|near|of|round|around|towards)\s+(?:(?:the\s+)?[A-Z]{2,4}(?![A-Za-z])|(?:the\s+"#,
    r#"|a\s+)?[A-Z][a-z]{2,})"#,
));

complex_pattern!(PLACE_NAME_ANY, r#"(?:(?<=[:;,]\s)|(?<=\bor\s)|(?<=\band\s))[A-Z][a-z]{2,}"#);
pattern!(SETTLED_ON, r#"(?i)\byou (?:settled on|decided on|went for|opted for|chose|picked|plumped for|said|mentioned|told me)\b"#);
pattern!(SPOT_OF_THE_THING, concat!(
    r#"(?i)\b(?:on|at|in|under|near)\s+the\s+(?:back|front|top|bottom|side|underside|base|rear|middle"#,
    r#"|centre|center|corner)\s+of\s+(?:the|a|an|its|your)\s+[\w'-]+"#,
));

pattern!(SPOT_ON_THE_THING, concat!(
    r#"(?i)\b(?:in|on|under|underneath|behind|inside|beneath|beside|alongside|next to|on top of|just under"#,
    r#"|just behind|halfway up|partway up)\s+(?:the|a|an|its|your|each|either|one)\s+([\w'-]+)"#,
));

pattern!(PLACE_KIND, concat!(
    r#"(?i)\b(?:shops|stores|supermarkets|pharmacies|station|stations|markets|cafes|cafés|booths|branches"#,
    r#"|clinics|charities|tip|depots|counters|showrooms|kiosks|market hall|market halls|food court"#,
    r#"|food courts|bakery|bakeries|street food|food van|food vans|multi storey|multistorey|multi-storey)\b"#,
));

pattern!(PLACE_AT_HOME, concat!(
    r#"(?i)\b(?:room|bedroom|bathroom|kitchen|utility|hall|hallway|landing|corridor|garage|loft|shed|porch"#,
    r#"|doorstep|doormat|doorway|door|stairs|staircase|step|floor|corner|wardrobe|cupboard|drawer|shelf"#,
    r#"|shelves|bookcase|bookshelf|mantelpiece|windowsill|window sill|bedside|nightstand|bed|sofa|settee"#,
    r#"|couch|cushion|chair|table|desk|hook|peg|pocket|bag|handbag|rucksack|backpack|glovebox|glove box"#,
    r#"|footwell|boot|bin|washing machine|tumble dryer|airing cupboard|street|side street|back street|road"#,
    r#"|car park|park and ride|lay-by|lay by|cul-de-sac|estate|alley|alleyway|walkway|underpass"#,
    r#"|carpark)s?\b|\b(?:on|in|under|behind|beside|next to|top of|bottom of|end of)\s+(?:the\s+|your\s+"#,
    r#"|my\s+)?(?:sofa|bed|table|desk|stairs|fridge|freezer|counter|shelf|cupboard|wardrobe|radiator|window"#,
    r#"|door|mat|hook|hall|car|boot|pushchair|pram|buggy)s?\b|\b(?:behind|under|underneath|beside|next to"#,
    r#"|on top of|in front of|at the bottom of|at the back of|at the front of|at the side of)\s+(?:the\s+"#,
    r#"|your\s+|my\s+|a\s+)?[a-z]{3,}|\b(?:in the open|up high|off the floor|in the middle of"#,
    r#"|away from the (?:wall|floor|heat))"#,
));

pattern!(PUNT, concat!(
    r#"(?i)^(?:i'?m not sure|i am not sure|i don'?t know|i do not know|i can'?t|i cannot|can'?t call"#,
    r#"|that'?s beyond|that is beyond|that'?s all|that is all|there is no more|beyond what i know"#,
    r#"|not mine to|not a call|not one i can call|a quick search|searching for|i'?d rather not"#,
    r#"|you haven't told me|that'?s a good question|good question|no idea|not something i|no (?:figure"#,
    r#"|number|price|answer|idea|verdict|view|opinion|guess) from me|say (?:what (?:you(?:'?re"#,
    r#"| are)? (?:asking|after|on about|worried about|unsure)|you mean|you want to know|has you (?:asking"#,
    r#"|wondering))|which|that again|more|me more|a bit more)\b|tell me (?:what|which|more) (?:you|has|is"#,
    r#"|part)\b|no (?:first[- ]hand )?(?:knowledge|idea|clue) (?:of|on|about|here|from me)\b|^nothing (?:in"#,
    r#"|about) (?:that|this|it|what you (?:said|told me|described))\b[^.!?]{0,60}?\b(?:say|says|said|tell"#,
    r#"|tells|told|show|shows|showed|suggest|suggests|indicate|indicates|go on)\b|^nothing here (?:to go on"#,
    r#"|that (?:says|tells))\b|^hard to (?:put a (?:scale|number) on|say how|judge|tell how|gauge)\b"#,
    r#"|i'?m not sure (?:what|how|which|whether|where) to|i'?m not sure without)"#,
));

complex_pattern!(HANDOFF, concat!(
    r#"(?i)^(?:(?:that'?s|that is|it'?s|it is) (?:your|a) call(?![a-z])|not (?:a )?call (?:i|we) (?:can"#,
    r#"|could|would)|(?:that'?s|that is) (?:one|a question|something) for (?:your|a|the)\b|(?:that'?s"#,
    r#"|that is) (?:everything|all) i (?:have|know)\b|(?:that'?s|that is|it'?s|it is) not (?:mine"#,
    r#"|for me) to (?:answer|say|tell|call|judge)\b|i would only repeat myself|nothing more i can (?:say"#,
    r#"|add|tell)\b|(?:that'?s|that is) not for me to|whether (?:it|that|they|this|[0-9]+|[a-z]+) (?:is|are"#,
    r#"|was|will be|would be) (?:worth|a lot|too much|too long|too far|any good|safe|normal|fine)\b"#,
    r#"|hold (?:it|that|this|the price|[0-9]+)\b[^.]{0,30}\bagainst (?:one|another|a|other)\b"#,
    r#"|only you can (?:weigh|judge|say|decide|call)\b|(?:that'?s|that is|it'?s|it is) yours to (?:weigh"#,
    r#"|say|decide|call|judge)\b|that part(?:'?s| is) (?:yours|up to you)\b|(?:i'?ll|i will) leave (?:that"#,
    r#"|this|it) to (?:your|a|the|somebody|someone)\b)"#,
));

pattern!(REFERS_BACK, concat!(
    r#"(?i)^\s*(?:i wouldn'?t call|i would not call|nothing in that|hard to|no way to"#,
    r#"|not something i can)\b[^.!?]{0,80}?\b(?:answer|step|thing|doing) above\b"#,
));

pattern!(POINTS_BACKWARDS, concat!(
    r#"(?i)^\s*(?:it|its|it's|that|that's|this|these|those|they|them|their|there\s+(?:it|they|you|we|that"#,
    r#"|those|these)\b|then|so|and|but|also|otherwise|still|though|however|either|neither|both|such"#,
    r#"|the same|which|who|if|unless|in that case|at that point|after that|for now)\b"#,
));

pattern!(OPENS_WITH_SYMPATHY, concat!(
    r#"(?i)^\s*(?:(?:i'm|i am|so|very|truly)\s+)*(?:sorry|apologies)\b|^\s*that'?s\s+(?:hard|awful|terrible"#,
    r#"|rough|grim|rotten|a lot)\b|^\s*i'?m\s+so\s+sorry\b|^\s*my\s+(?:heart|sympathies)\b"#,
));

pattern!(OPENS_WITH_A_WARNING, concat!(
    r#"(?i)^\s*(?:there'?s?\s+(?:is\s+)?no\s+safe\b|no\s+safe\b|never\b|do\s+not\b|don'?t\b|(?:call|ring"#,
    r#"|phone)\s+999\b|999\b|go\s+to\s+(?:a&e|the\s+hospital)\b)"#,
));


const SHARED_WORDS: &[&str] = &[
    "about",
    "and",
    "any",
    "are",
    "been",
    "being",
    "but",
    "can",
    "could",
    "did",
    "does",
    "doing",
    "for",
    "from",
    "get",
    "gets",
    "got",
    "has",
    "have",
    "how",
    "into",
    "is",
    "it",
    "just",
    "like",
    "long",
    "made",
    "make",
    "many",
    "more",
    "most",
    "much",
    "need",
    "needs",
    "not",
    "one",
    "out",
    "over",
    "should",
    "some",
    "take",
    "takes",
    "than",
    "that",
    "the",
    "their",
    "them",
    "then",
    "there",
    "they",
    "this",
    "very",
    "want",
    "wants",
    "was",
    "were",
    "what",
    "when",
    "where",
    "which",
    "who",
    "why",
    "will",
    "with",
    "would",
    "you",
    "your",
];

const SPOT_PARTS: &[&str] = &[
    "back",
    "badge",
    "bag",
    "bar",
    "base",
    "blade",
    "body",
    "bolt",
    "bottom",
    "box",
    "bracket",
    "brush",
    "button",
    "cable",
    "cap",
    "cartridge",
    "case",
    "casing",
    "center",
    "centre",
    "clamp",
    "clip",
    "compartment",
    "cord",
    "corner",
    "cover",
    "dial",
    "door",
    "doors",
    "drawer",
    "drum",
    "edge",
    "filter",
    "flap",
    "front",
    "grille",
    "guard",
    "handle",
    "handles",
    "hinge",
    "hose",
    "housing",
    "knob",
    "label",
    "labels",
    "leaflet",
    "lid",
    "manual",
    "middle",
    "nozzle",
    "nut",
    "packaging",
    "panel",
    "pipe",
    "plate",
    "plates",
    "plug",
    "port",
    "rack",
    "rear",
    "rim",
    "screen",
    "screw",
    "seal",
    "seals",
    "seam",
    "shelf",
    "shell",
    "side",
    "sides",
    "slot",
    "socket",
    "spout",
    "sticker",
    "stickers",
    "strap",
    "switch",
    "tie",
    "top",
    "tray",
    "trays",
    "trim",
    "tube",
    "underside",
    "valve",
    "vent",
    "wand",
    "wire",
];
