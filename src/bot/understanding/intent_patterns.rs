// Regex and cue tables from the pinned upstream shape readers.

use super::{LazyLock, Regex};
pattern!(
    ASKS_FOR_MEANING,
    r#"(?i)\bwhat (?:is|are|was|were|'s)\b|\bwhat (?:does|do)\b(?:(?!\bwhat\b)[^?]){0,30}?\b(?:do|does|include|includes|cover|covers|contain|contains|show|shows|sound like)\b|\bwhat do you call\b|\b(?:mean|means|meaning|stand for|stands for|short for)\b|\bdifference between\b"#
);
pattern!(
    ASKS_ITS_OWN_YES_NO,
    r#"(?i)\b(?:does|do|will|would|can|could|should|did|is|are|was|were) (?:it|that|this|they|these|those)\b"#
);
pattern!(
    ASKS_OUT_LOUD,
    r#"(?i)^(?:what|which|who|whose|when|where|why|how|is|are|was|were|do|does|did|can|could|should|shall|would|will|any|anything|anyone|recommend|suggest|name|give me|tell me|got any)\b"#
);
pattern!(
    ASKS_TO_BE_TOLD_WHERE,
    r#"(?i)\b(?:tell (?:me|us)|know|knows|say|says|shows?|explain|remind|remember|any idea|wonder|find|check)\b[^?]{0,25}\bwhere\b"#
);
pattern!(
    ASKS_YES_OR_NO,
    r#"(?i)^\W*(?:is|are|was|were|am|do|does|did|should|shall|can|could|would|will|may|might|have|has|had)\b"#
);
pattern!(
    ASK_TO_ASK,
    r#"(?i)\bshould i (?:ask|explain|check|say|mention|bring up)\s+why\b"#
);
pattern!(
    A_DURATION_WORD,
    r#"(?i)\b(?:take|takes|took|taking|last|lasts|lasted|lasting|keep|keeps|kept|stay|stays|sit|sits|wait|waiting|seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|overnight|fortnight|fortnightly)\b"#
);
pattern!(
    A_FEW_THINGS,
    r#"(?i)\b(?:tips?|ideas?|suggestions?|recommendations?|pointers?|hacks?|options)\b"#
);
pattern!(
    A_FIGURE_NAMED,
    r#"(?i)\bwhat (?:age|price|cost|number|salary|pay|wage|rate|fee|dose|dosage|amount|percentage|percent|discount|charge|quote|score|mark|grade|figure|budget)\b"#
);
pattern!(
    A_FIGURE_OF_A_MONEY_VERB,
    r#"(?i)\bwhat\s+(?:should|shall|do|does|would|can|could|will)\s+(?:i|we|you|they|he|she)\s+(?:be\s+)?(?:pay|paying|spend|spending|earn|earning|budget)\b(?!\s+off\b)"#
);
pattern!(
    A_GOOD_TIME,
    r#"(?i)(?:^|[,;]\s*|\b(?:and|so|but|then|also)\s+)(?:is|are|was|were|would|could|will|do you think it is|do you think its)\b[^.?!]{0,60}?\b(?:a|an)\s+(?:good|bad|better|best|ok|okay|fine|safe|alright|decent|terrible|awful|busy|quiet)\s+time\b"#
);
pattern!(
    A_PRICE_ASK,
    r#"(?i)\b(?:is|are|was|were)\s+(?:it|that|this|these|those|they|there|one)\s+(?:(?:too|so|really|actually|very|quite)\s+)?(?:expensive|pricey|dear|cheap|costly)\b"#
);
pattern!(
    A_RECALL_ASK,
    r#"(?i)\bremind me (?:of |about |that |if |whether )|\bremind me (?:what|when|where|who|which|how) (?!to\b)|\bwhat (?:did|have) (?:i|we) (?:say|said|ask|asked|tell|told|mention|mentioned)\b|\bwhat (?:was|were) (?:my|our) (?:first|last|earlier|original)\b|\b(?:what|which) (?:was|is) the (?:first|last) (?:thing|line|question|message) i\b"#
);
pattern!(
    A_SIZE_LONG,
    r#"(?i)\b\d+(?:\.\d+)?\s*(?:mm|cm|m|km|metres?|meters?|feet|foot|ft|inches?|in|yards?|yds?)\s+(?:long|wide|high|deep|tall|thick)\b"#
);
pattern!(
    A_THING_TO_GIVE,
    r#"(?i)\b(?:gift|gifts|present|presents|pressie|pressies|stocking filler|secret santa)\b"#
);
pattern!(
    BARE_A_RECALL,
    r#"(?i)\b(?:again|earlier|before|yesterday)\s*$|\b(?:i|you)\s+said\b"#
);
pattern!(
    BARE_A_SECOND_QUESTION,
    r#"(?i)\b(?:why|how|where|when|who|which)\b"#
);
pub(super) const BARE_NOTHING_ASKED: &[&str] =
    &["going on", "happening", "it", "new", "on", "that", "up"];
pattern!(
    BARE_NOT_A_THING_ANYWHERE,
    r#"(?i)\d|\bcalled\b|\bname of\b|\bmeaning of\b|\blike\b|\bwrong\b|\bpower\s+of\b|\btimes\b|\bplus\b|\bminus\b|\bdivided\b|\bmultiplied\b|\bpercent\s+of\b|\bsquared\b|\bcubed\b|\btodays?\b|\btonights?\b|\btomorrows?\b|\bthis\s+(?:week|weekend|month|year)\b"#
);
pattern!(
    BARE_NOT_A_THING_AT_THE_START,
    r#"(?i)^(?:you|u|ur|your|yours|i|im|i'm|my|mine|our|ours|we|us|me|it|this|that|these|those|there|here|her|hers|his|their|theirs|its|up|on|at|for|with|under|inside|outside|behind|near|next|around|about|in|new|nice|good|great|better|best|cheap|cheapest|expensive|wrong|missing|left|leftover|easy|easiest|hard|hardest|quick|quickest|simple|simplest|safe|safest|healthy|healthiest|popular|common|normal|usual|fast|fastest|slow|slowest|big|biggest|small|smallest|right|going|doing|showing|happening|playing|coming|open|one|two|three|four|five|six|seven|eight|nine|ten|hundred|thousand|million|half|double)\b|^the\s+(?:\w+est|best|worst|most|least|furthest)\b|^the\s+(?:new|newest|latest|current|next|other|same|main|real)\b|^the\s+(?:time|date|day|capital|price|cost|charge|fee|number|address|name|answer|way|point|difference|matter)\b"#
);
pattern!(
    BARE_VALUE_NOUN,
    r#"(?i)^(?:the\s+)?(?:\w+\s+){0,3}(?:rate|rates|price|prices|cost|costs|charge|charges|fee|fees|number|numbers|phone|telephone|address|addresses|email|website|opening\s+hours|point|points|allowance|temperature|password|postcode|pin|score|limit|deadline|budget|balance|total|amount|figure|percentage|salary|wage|wages|weight|height|length|width|depth|distance|speed|age|population|capacity|mileage)\b"#
);
pattern!(BARE_WHAT_AS_IS, r#"(?i)^what(?:'s|s)\b"#);
pattern!(
    BARE_WHAT_IS_PLAIN,
    r#"(?i)^what\s+(?:is|are|was)\s+(?P<thing>[^\s,;:]+.*)$"#
);
pattern!(
    BEST_OF_MANY,
    r#"(?i)\bwhat (?:are|were) (?:the )?(?:best|main|top|nicest|cheapest|better)\b|\bwhat (?:are|were) (?:some|a few|any)\b|\bwhat (?:are|were) (?:the )?\w+ (?:to|for)\b"#
);
pattern!(
    BEST_WAY,
    r#"(?i)\b(?:the )?(?:best|good|right|proper|easiest|quickest|simplest|safest|cheapest|fastest) (?:way|method|approach) (?:to|of|for)\b"#
);
pattern!(
    COMPARED,
    r#"(?i)\b(?:better|worse|cheaper|dearer|quieter|louder|faster|slower|easier|harder|healthier|safer|simpler|tastier|stronger) than\b"#
);
pattern!(
    COMPARE_MYSELF,
    r#"(?i)\bcompar(?:e|ing)\b[^.?!]{0,14}\b(?:myself|ourselves)\b|\bcompare\s+(?:to|with)\s+(?:me|my|us|our|him|her|them|others)\b"#
);
pattern!(
    COMPARISON,
    r#"(?i)\bcompare[ds]?\b|\bcomparing\b|\bvs\.?(?=\s|$)|\bversus\b|\bdifferences? between\b|\bhow (?:is|are|does|do|did) [^.?!]{0,40}\b(?:different|differ|compare)\b|\bwhat sets? [^.?!]{0,30}\bapart\b|\bsets? [^.?!]{0,30} apart\b"#
);
pattern!(
    CONFIRM_CAUSE,
    r#"(?i)(?:^|[;,.]\s*|\b(?:and|so|but|then)\s+)(?:is|was) (?:that|this|it) why\b"#
);
pattern!(
    CONVERSION,
    r#"(?i)\bhow long (?:is|are|was)\b[^?]{0,30}?\b(?:in|to|as|into) (?:feet|foot|metres?|meters?|miles?|kilometres?|kilometers?|km|yards?|inches|inch|cm|mm)\b"#
);
pattern!(
    CONVERTED_FIGURE,
    r#"(?i)\b(?:what(?:'s| is|s)|whats)\s*\d+(?:\.\d+)?\s*(?:c|f|degrees?|deg|gas|ml|l|g|kg|oz|lb|lbs|cm|mm|in|inches|pints?|litres?|liters?|ft|feet|foot|metres?|meters?|m|miles?|mi|yards?|yd|km|stone|st)\b[^?]{0,25}?\b(?:in|to|into|as)\b|\bconvert\s+\d+(?:\.\d+)?\s*(?:c|f|degrees?|deg|ml|g|kg|oz|lb|lbs|cm|mm|in)\b[^?]{0,20}?\b(?:in|to|into)\b|\bwhat(?:'s| is|s)?\s*\d+(?:\.\d+)?\s+\w+(?:\s+\w+)?\s+(?:in|to)\s+(?:grams?|g|kg|ml|millilitres?|milliliters?|litres?|liters?|ounces?|oz|pounds?|lb|cups?|tablespoons?|teaspoons?|degrees?|c|f)\b"#
);
pattern!(
    COSTS_NOTHING,
    r#"(?i)\bcosts? (?:nothing|free|nowt)\b|\bfor free\b|\bthat (?:is|are|will be) free\b|\b(?:doesn'?t|does not|didn'?t|did not|don'?t|do not|won'?t|will not) costs?\b|\bcosts? (?:much|a lot|more|less|too much)\b"#
);
pattern!(
    COST_ASK,
    r#"(?i)\bwhat (?:does|do|will|would|did|is it going to) .{0,40}\bcosts?\b|\bwhat .{0,40}\bcosts?\b|\b(?:price|cost|charge|fee)s? (?:of|for)\b"#
);
pub(super) const COST_STANDS_IN: &[&str] = &[
    "definition",
    "list",
    "opinion",
    "question",
    "recommend",
    "what",
];
pattern!(
    COUNTED_STEPS,
    r#"(?i)\bhow many steps\b|\bsteps? per\b|\bsteps? in (?:a|one) (?:mile|km|day)\b|\bstep count\b|\b\d{3,} steps\b"#
);
pattern!(
    DIFFERENCE,
    r#"(?i)\bwhat(?:['’]?s| is| are| was| were)?\b(?:\s+[\w'-]+){0,3}\s+differences?\b|\bhow (?:is|are|does)\b[^.?!]{0,40}\b(?:different|differ)\b\s*(?:from|to)\b|^[\W]*(?:the )?differences? between\b"#
);
pattern!(
    DOING_WRONG,
    r#"(?i)\bwhat(?:'?s| is| was)?\s+(?:going\s+|gone\s+)?wrong\b|\bwhat (?:went|had gone|has gone|have gone) wrong\b|\bwhat (?:could|might|may|would) be (?:wrong|making|causing)\b|\bwhat (?:am i|are we|did i|did we|have i|have we) (?:do|doing|done|did|get|getting|got|have|having|had)\b.{0,25}\bwrong\b|\bwhat (?:do i|do we|does he|does she|do they) do wrong\b|\bwhere (?:am i|are we|did i|did we|do i|do we) going wrong\b|\bwhat(?:'?s| is| was)?\s+(?:the|my|our|his|her|their)\s+(?:problem|trouble|issue|fault)\s+(?:with|in)\b"#
);
pattern!(
    DO_WITH_A_PERSON,
    r#"(?i)\b(?:do|does|doing) with (?:myself|yourself|himself|herself|ourselves|yourselves|themselves|him|her|each other)\b"#
);
pattern!(
    EXPLICIT_COMPARE,
    r#"(?i)\bcompare[ds]?\b|\bcomparing\b|\bvs\.?(?=\s|$)|\bversus\b|\bdifferences? between\b"#
);
pattern!(
    FIX_ASK,
    r#"(?i)\b(?:what(?:'?s| is| was)?|any|the|a)\s+(?:quick |easy |simple |cheap |good |real )?(?:fix|cure|remedy)\s+(?:for|to)\b|\b(?:fix|cure|remedy) for (?:my|the|a|an|this|that)\b|\b(?:what(?:'?s| is| was)|any)\s+(?:the\s+)?(?:quick |easy |simple |cheap |good |real )?(?:fix|cure|remedy)\s*[?.!]*\s*$"#
);
pattern!(
    FOLLOWUP,
    r#"(?i)^\s*(?:and|so|then|but|ok(?:ay)?[,\s]+(?:and|so)|what\s+about|how\s+about|what\s+if|would\s+that|does\s+that|same\s+(?:for|with))\b"#
);
pattern!(
    GIVE_ME_AN_IDIOM,
    r#"(?i)\b(?:a|an|one)\s+(?:rough\s+idea|minute|minutes|second|seconds|sec|secs|mo|moment|break|chance|ring|rings|shout|call|lift|ride|hand|hands|hint|clue|tip|tips|way|method|reason|sign|excuse|rest|try|kiss|discount|picture|photo|image)\b"#
);
pattern!(
    GIVE_ME_A_FEW,
    r#"(?i)\b(?:gimme|give me|show me|send me)\s+(?:some|a few|couple of|a couple|any)\s+([a-z][a-z'-]*)"#
);
pub(super) const GIVE_ME_LEAD_IN: &[&str] = &[
    "alright", "also", "and", "but", "can", "could", "gimme", "hey", "hi", "if", "just", "maybe",
    "me", "now", "of", "ok", "okay", "please", "so", "then", "will", "would", "you",
];
pattern!(
    GIVE_ME_MADE_UP,
    r#"(?i)\b(?:limerick|poem|haiku|riddle|joke|story|tale|rhyme|tongue twister|fun fact|fact|quiz question|trivia question|general knowledge question|question|example|routine|prompt|toast|speech|plan|itinerary|list|recipe)\b"#
);
pattern!(
    GIVE_ME_NOT_A_THING,
    r#"(?i)\b(?:human|person|somebody|someone|real person|operator|manager)\b"#
);
pattern!(
    GIVE_ME_ONE,
    r#"(?i)\b(?:gimme|give me|show me|send me)\s+(?:a|an|one|another)\s+([a-z][a-z'-]*)"#
);
pattern!(
    GIVING_FRAME,
    r#"(?i)\b(?:for|to)\s+(?:a|an|the)?\s*(?:my|our|his|her|their|him|them|mum|mam|mom|dad|mother|father|sister|brother|son|daughter|nan|nana|grandma|grandad|granny|grandpa|gran|aunt|uncle|cousin|friend|mate|bestie|teacher|colleague|boss|partner|wife|husband|boyfriend|girlfriend|neighbours?|baby|newborn|toddler|child|kid|boy|girl|teenager|nephew|niece|in-laws?|flatmate|housemate|carer|\d+\s*(?:year|yr)s?\s*old)\b|\bwhat (?:should i |shall i |do i |can i |would you |to )?(?:get|buy|make|give)\b|\b(?:idea|ideas|suggest|suggestion|suggestions|recommend|recommendation)\b"#
);
pattern!(
    GOOD_FIGURE,
    r#"(?i)\b(?:good|average|normal|typical|usual|best) (?:\w+ ){0,3}(?:time|age|price|number|amount|cost|salary|wage|rate|temperature|weight|height|speed|distance|size|dose|percentage)\b"#
);
pattern!(
    GOOD_FIGURE_WORD,
    r#"(?i)\b(?:time|timeframe|rate|age|price|cost|number|amount|weight|height|length|salary|wage|dose|dosage|percentage|temperature|speed|distance|size|pressure|humidity|mileage|voltage|rent)\b"#
);
pattern!(
    GOOD_MEASURE,
    r#"(?i)\bwhat(?:'s|s| is)\s+(?:a |an |the )?(?:really |actually |roughly |sensible |sort of )?(?:good|decent|normal|typical|average|usual|realistic|fair)\s+(?:\w+\s+){0,2}(?:salary|wage|wages|pay|income|rate|deposit|budget|score|pension|pot|percentage|percent|tip(?!\s+(?:for|on|to|about)\b))\b"#
);
pattern!(GOOD_ONE, r#"(?i)\bwhat(?:'s|s| is) (?:a |an )?good\b"#);
pattern!(
    GOOD_THING,
    r#"(?i)\bwhat(?:'s|s| is| are)\s+(?:a|an|the|some)?\s*(?:really |actually |bit of a )?(?:good|great|nice|decent|cheap|budget|quick|easy|simple|sensible|healthy)\b"#
);
pattern!(GOOD_WAY, r#"(?i)\b(?:way|method|approach|routine)\b"#);
pattern!(
    HAVE_RECALL,
    r#"(?i)^\W*what (?:do|did) i have\b(?! (?:for|on|in the|tonight|to eat|to wear|to drink))"#
);
pattern!(
    HELP_ASK_ANY,
    r#"(?i)^\W*(?:(?:hi|hey|hello|ok|okay|so|right|please|sorry|and)\b[ ,]*)*(?:(?:can|could|would|will) you\s+)?(?:help (?:me|us)|i need help|i could do with (?:some )?help|give me a hand)\b(?!\s+(?:choose|choosing|pick|picking|decide|deciding|select|selecting|compare|comparing|name|naming|word|wording|come up with)\b|\s+between\b|\s+which\b)"#
);
pattern!(
    HELP_ASK_NO_TASK,
    r#"(?i)^[\s,.;:!?\-]*(?:(?:with|about|on)\s+(?:something|anything|stuff|it|this|that)\b)?[\s,.;:!?\-]*$|\b(?:dont|don'?t|do not) know (?:what|where|how)\b|\bnot sure what\b"#
);
pattern!(
    HELP_ME,
    r#"(?i)\b(?:can|could|would|will) you help me\b|\bhelp me (?:with|to|out)\b"#
);
pattern!(
    HELP_WORD,
    r#"(?i)\b(?:help(?: me| us)?|i need help|i could do with (?:some )?help|give me a hand)\b"#
);
pattern!(
    HOW_ARE_YOU,
    r#"(?i)^(?:(?:hey|hi|hiya|heya|hello|yo)(?: there)?[ ,]+)?how(?:'?s| is| are)?[ ,]+(?:you|u|ya|things|tricks|life|are you|are things|is it going|s it going|have you been|are we|your (?:day|week|evening|morning|night|head|family))\b(?!\s+(?:work|works|worked|working|go|goes|went|change|changes|changed|happen|happens|happened|fit|fits|seem|seems)\b)"#
);
pattern!(
    HOW_ASK_FORM,
    r#"(?i)\bhow (?:much|many|long|often|soon|far|tall|big|heavy|old|hot|cold)\s+(?:is|are|was|were|does|do|did|will|would|can|could|should|shall|much|many|far)\b"#
);
pattern!(HOW_COME, r#"(?i)\bhow come\b|\bhow is it that\b"#);
pattern!(HOW_I, r#"(?i)\bhow (?:do|can|could|should|would) i\b"#);
pattern!(
    HOW_IT_HAPPENED,
    r#"(?i)\bhow\s+(?:did|does|do)\s+(?:it|that|this)\s+(?:happen|happens|happened|come about|came about)\b"#
);
pattern!(
    HOW_IT_WENT,
    r#"(?i)\bhow\s+(?:did|does|was|were|is|are|'s)\s+(?:(?!\b(?:i|you|we|me|us)\b)\w+\s+){0,4}(?:go|goes|went|get on|got on|getting on|get along|turn out|turned out|work out|worked out)\b(?!\s+\w)|\bhow (?:it|that|this|things|everything) (?:goes|went|go|turns out|turned out)\b(?!\s+\w)|\bhow (?:was|were) (?:it|that|this|things|everything)\b(?!\s+\w)|\bhow goes it\b"#
);
pattern!(
    HOW_IT_WORKS,
    r#"(?i)\bhow (?:does|do|did) (?:(?!\b(?:i|you|we|im|me)\b)[\w']+ ){1,4}(?:work|works|working)\b"#
);
pattern!(
    HOW_MANY_TIME,
    r#"(?i)\bhow (?:many|much) (?:more |longer )?(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|nights?|time)\b"#
);
pattern!(
    HOW_MEASURE,
    r#"(?i)\bhow (?:tall|old|big|far|heavy|wide|deep|high|fast|hot|cold|warm|hard|loud|cheap|expensive|thick|thin|narrow|broad|shallow|steep|fine|coarse|tight|loose|many|much|often)\b"#
);
pattern!(
    HOW_MEASURE_HEAD,
    r#"(?i)^how (?:tall|old|big|far|heavy|wide|deep|high|fast|hot|cold|warm|hard|loud|cheap|expensive|thick|thin|ripe|tight|loose|full|low|slow|quick|short|small|large|near|soft|firm|dark|bright|sharp|rough|smooth|fine|coarse|steep|cool|fresh|dry|wet|salty|sweet|spicy|sour|mild|strong|weak|light|stiff)\b"#
);
pattern!(HOW_SOON, r#"(?i)\bhow (?:soon|early|quickly|long)\b"#);
pattern!(
    HOW_TO_ANSWER,
    r#"(?i)\b(?:how|what) (?:do|should|can|could|would) i (?:answer|reply to|explain|word|phrase)\b|\b(?:what|how) (?:do|should|would) you say when they ask\b"#
);
pattern!(
    HOW_TO_DO,
    r#"(?i)\bhow (?:do|can|could|should|would|will) (?:i|we|you) (?:explain|answer|say|tell|word|respond|reply|put it|break it|approach|handle)\b|\bhow to (?:explain|answer|say|tell|word|respond|reply|approach|handle)\b"#
);
pattern!(
    IF_IT_HAPPENS,
    r#"(?i)^\W*(?:and\s+|so\s+|but\s+|then\s+)?if\b"#
);
pattern!(
    JUDGED_MEASURE,
    r#"(?i)\b(?:good|average|normal|typical|usual|best|safe|right|ideal|healthy|cheap|correct|recommended|sensible|maximum|minimum|perfect|decent|acceptable)\s+(?:\w+\s+){0,3}(?:rate|age|price|cost|number|amount|weight|height|length|salary|wage|dose|dosage|percentage|temperature|speed|distance|size|pressure|humidity|mileage|rent|voltage)\b"#
);
pattern!(
    KINDS_OF,
    r#"(?i)\b(?:what|which) (?:kinds?|types?|sorts?|varieties|styles|brands|makes|models)(?: of (?:[\w'-]+ ){0,4})?(?:are there|are available|is there|exist|do you|would you|can|could)\b|\bwhat (?:are|were) the (?:kinds?|types?|sorts?|varieties|styles)\b|\bwhat (?:are|were) (?:my|the|our) (?:options|choices)\b|\bwhat (?:options|choices) (?:do i|do we|have i|are there)\b|\b(?:any|other|more) (?:options|choices|alternatives)\b|\bwhat else (?:is|are) (?:there|out there)\b|\bwhat (?:is|are) available\b"#
);
pattern!(
    LEADING_JOIN,
    r#"(?i)^(?:(?:and|so|but|then|also|ok|okay|right|now|well)\b[ ,]*)+"#
);
pattern!(
    LONG_TIME,
    r#"(?i)\blong time (?:no (?:see|speak|hear|talk)\b|since\b)"#
);
pattern!(
    MADE_OF,
    r#"(?i)\bwhat (?:is|are) (?:\w+ ){0,3}made (?:of|from|out of)\b"#
);
pub(super) const MEASURE_LABELS: &[&str] = &["how_long", "how_much", "how_often"];
pattern!(
    MEMBERS_ASK,
    r#"(?i)\bwhat (?:are|were) the (?:early |first |main |usual |common |classic |key |warning |typical |initial )?(?:signs?|symptoms?|stages?|phases?)\b|\bwhat (?:signs?|symptoms?|stages?)\b[^.?!]{0,30}\b(?:should|would|do|can|might) (?:i|you|we) (?:look|watch|expect|see)\b|\bwhat are the (?:warning|telltale) signs\b"#
);
pattern!(METHOD_OPENS, r#"(?i)^how (?:do|does|did) (?:i|we|you)\b"#);
pattern!(
    MONEY_VERB_NOT_A_PRICE,
    r#"(?i)\bpay(?:ing)?\s+attention\b|\b(?:pay|paying|spend|spending)\s+(?:it|them|this|that|the\s+money|the\s+rest|the\s+lot)\b|\b(?:pay|paying)\s+off\b|\bspend(?:ing)?\s+(?:(?:my|our|your|the)\s+)?(?:time|attention|effort|trouble|energy|evening|day|weekend|morning|afternoon|night|holiday|summer|hours|years|life)\b"#
);
pattern!(
    MY_THING_CALLED,
    r#"(?i)\bwhat (?:are|is|were|was) (?:my|our)\b[^?]{0,30}?\b(?:called|named)\b"#
);
pattern!(
    NEED_TO,
    r#"(?i)\bi (?:need|have|'ve|have got|got|'ll need|will need) to (?:(?:sort|register|ring|call|fill|send|tell|book|apply|arrange|cancel|close|write|deal)\b(?:\s+(?:out|up|off|in|on|with|for|about))?\s+[\w'-]+|(?:get|find|move|do|work|set|change)\b(?:\s+(?:out|up|off|in|to|with|for|about))?\s+(?:the|my|his|her|their|our|a|an|this|that|these|those|it|them|him|us|me)\b)"#
);
pattern!(
    NEED_TO_ASKED,
    r#"(?i)\b(?:do|does|did|will|would|should|shall|can|could|might|may|must|have)\s+(?:i|we)\s+(?:need|have|got|'ve)\s+to\b|\b(?:what|which|how|when|where|who|why)\s+(?:\w+\s+)?(?:do|does|did|will|would|should|shall|can|could)\s+(?:i|we)\s+(?:need|have|got|'ve)\s+to\b"#
);
pattern!(
    NEED_TO_DO,
    r#"(?i)\bwhat (?:do|does|will|would|should|shall|might|can) (?:i|we) (?:need|require) to\s+(?:do|know|say|ask|tell|learn|understand|work|watch|avoid|stop|prevent|fix|deal|look|sort|handle|prepare|plan)\b"#
);
pattern!(
    NEW_ASK,
    r#"(?i)\bwhat (?:do|does|should|shall|can|could|will|would) (?:i|we|you)\b"#
);
pattern!(
    NOT_A_COMPARISON,
    r#"(?i)\b(?:should i|shall i|worth (?:it|doing|the)|for me|which should i|which one should i|is it better to|would you)\b|\b(?:i|we) (?:always |still |keep |often |just )?compare\b|\bhow (?:do|can|should|would|could) (?:i|we)\b|\b(?:better|worse) than (?:i|we|you|they|she|he) (?:expected|thought|hoped|feared|looked)\b"#
);
pattern!(
    NOT_A_FIGURE,
    r#"(?i)\bhow (?:much|many) (?:do|did|can|could|would|have) you (?:know|remember|understand|recall)\b|\bhow (?:much|often) (?:does|do) (?:it|that|this|they) matter\b|\bhow much (?:should|do) (?:i|we) (?:worry|care|mind)\b"#
);
pattern!(
    NOT_A_GIVING_THING,
    r#"(?i)\bgift (?:card|cards|voucher|vouchers|wrap|wrapping|box|boxes|receipt|set)\b|\bpresent (?:tense|day|perfect)\b|\bgift (?:aid|tax|taxable)\b|\b(?:letter|note|thanks|thank you|receipt|return|returns)\b|\b(?:keeps?|kept|always)\b[^?]{0,25}\b(?:gift|present)s?\b|\b(?:buys|bought|gives|gave|brings|brought|receives|received)\b[^?]{0,20}\b(?:gift|present)s?\b"#
);
pattern!(
    NOT_A_PRICE_ASK,
    r#"(?i)\bworth\b|\bcheaper than\b|\bmore expensive than\b|\bafford"#
);
pattern!(
    NOT_KINDS_OF,
    r#"(?i)\bwhat (?:kind|type|sort) of (?:a )?(?:[\w'-]+ ){0,3}(?:is|was|are|were) (?:that|this|it|those|these|he|she)\b|\bwhat (?:kind|type|sort) of (?:[\w'-]+ ){0,3}(?:would|will|should|could|might|suits?|fits?)\b"#
);
pattern!(
    NOT_STEPS,
    r#"(?i)\bwhat (?:do|does|did) (?:i|you|we|he|she|they) do for (?:a )?living\b|\bwhat (?:do|does|did) (?:i|you|he|she|they) do for work\b"#
);
pattern!(
    ON_THE_CLOCK,
    r#"(?i)\b(?:time|times|hour|hours|minute|minutes)\b"#
);
pattern!(OPEN_WHO_HELP, r#"(?i)^\W*who\b[^.!?]{0,40}\bhelp\b"#);
pattern!(
    ORIGIN_ASK,
    r#"(?i)\bwhere (?:does|do|did) (?:\w+ ){0,3}(?:come from|grow)\b"#
);
pattern!(
    ORIGIN_PERSON,
    r#"(?i)\bwhere (?:does|do|did) (?:you|they|we|i|he|she|u)\b"#
);
pattern!(
    OTHER_QUESTION,
    r#"(?i)^\W*(?:where|when|who|whose|which|why|how|is|are|was|were|can|could|should|shall|would|will|does|did)\b(?!\s+not\b)"#
);
pattern!(
    PAIRING_ASK,
    r#"(?i)\bwhat (?:[\w'-]+ )?go(?:es)? (?:with|alongside|nicely with)\b|\bwhat (?:[\w'-]+ )?(?:wine|beer|drink|drinks|food|music|songs?|salad|sides?|dessert|cheese|bread|pudding|starter|sauce|film|book|present|gift)s? (?:for|with)\b|\bwhat should (?:i|we) name\b|\bwhat should (?:i|we) call (?:my|our|the|his|her|their)\b(?![^\n]*?\b(?:about|for|to|if|when)\b)"#
);
pattern!(
    PICK_ONE,
    r#"(?i)\bwhat (?:do|should|shall|can|could) (?:i|we|he|she|they) (?:cook|make|eat|have|drink|watch|read|listen to|wear|get|buy|order|play|study|plant|visit|do with|give|bring)\b|\bwhat (?:do|should|shall|can|could) (?:my|our|his|her|their) \w+ (?:cook|make|eat|have|drink|watch|read|wear|get|buy|order|play|study|plant|visit|give|bring)\b"#
);
pattern!(
    PICK_ONE_OF_A_KIND,
    r#"(?i)\bwhat (?!else\b|about\b|of\b|in\b|on\b|at\b|to\b|from\b|for\b|with\b|by\b|between\b|age\b|time\b|date\b|month\b|year\b|day\b|size\b|temp\b|temperature\b|price\b|cost\b|amount\b|number\b|help\b|support\b|percentage\b|share\b|spf\b|budget\b|salary\b|wage\b|dose\b|dosage\b|calorie\w*\b|weight\b|length\b|height\b|depth\b|speed\b|rate\b|fee\b|charge\b|rent\b|tip\b|discount\b|voltage\b|wattage\b|qualifications?\b|pets?\b|rights?\b|entitlements?\b|apr\b|interest\b|deposit\b|savings?\b)(?:(?:kind|sort|type|brand|make) of (?!price\b|cost\b|amount\b|salary\b|budget\b|time\b|size\b|weight\b))?(?:[\w'-]+ ){1,2}(?:should|shall) (?:i|we) [a-z]+\b"#
);
pub(super) const PICK_SIDES: &[&str] = &["choice", "compare", "which"];
pattern!(
    POINT_ASK,
    r#"(?i)\bwhat(?:'?s| is| was)?\s+the point (?:of|in)\s+(\w+(?: \w+){0,2})|^(?:any )?point (?:of|in)\s+(\w+(?: \w+){0,2})"#
);
pub(super) const POINT_NOT_A_THING: &[&str] = &[
    "all",
    "any",
    "anymore",
    "anything",
    "boiling",
    "carry",
    "carrying",
    "continuing",
    "everything",
    "freezing",
    "going",
    "honestly",
    "it",
    "life",
    "living",
    "me",
    "melting",
    "nothing",
    "now",
    "really",
    "smoke",
    "that",
    "them",
    "these",
    "this",
    "those",
    "us",
    "you",
];
pattern!(
    PRICE_ADJECTIVE,
    r#"(?i)\b(?:is|are|was|were|isn'?t|aren'?t|wasn'?t|weren'?t)\b[^?.!]{0,30}?\b(?:expensive|costly|pricey)\b|\b(?:will|would|won'?t|wouldn'?t)\b[^?.!]{0,24}?\bbe\b[^?.!]{0,14}?\b(?:expensive|costly|pricey)\b"#
);
pattern!(
    PRICE_ADJECTIVE_KEEPS,
    r#"(?i)\b(?:than|or|instead|versus|vs|worth|whether|rather)\b"#
);
pattern!(
    PRICE_ADJECTIVE_THE_ASK_IS_LATER,
    r#"(?i)\b(?:expensive|costly|pricey)\b[^?.!]{0,30}?\b(?:can|could|should|shall|would|will|do|does|did|am|is|are)\s+(?:i|we|you|he|she|they|it)\b"#
);
pub(super) const PRICE_ADJECTIVE_WHY: &[&str] = &["why"];
pattern!(
    PRICE_IDIOM,
    r#"(?i)\bwhat(?:'s|s| is| was)?\s+the (?:going|current|average|typical|usual) rate\b|\bset (?:me|us|you) back\b|\bexpect(?:ing)? to (?:pay|spend|be paying)\b|\b(?:usually|normally|typically|roughly|about) run(?:s)? to\b"#
);
pattern!(
    QUESTION_IN_THE_LINE,
    r#"(?i)\b(?:what|which|where|when|who|whom|whose|why|how)\s+(?:do|does|did|should|shall|can|could|will|would|must|is|are|am)\b|\bhow (?:long|often|many|much)\s+(?:do|does|did|should|shall|can|could|will|would|must)\b"#
);
pattern!(
    REASON_ASK,
    r#"(?i)\bhow come\b|\bwhat (?:causes|caused|causing|is causing)\b|\bwhat(?:'?s| is| was)?\s+(?:causing|making|the cause|the reason)\b|\bwhat (?:makes|made) (?:you|me|us|people|him|her|them|it|this|that|someone|my|your|our|their|his|its)\b|\bwhat (?:makes|made) (?:the |a |an )?[a-z']+(?:(?: and| or) (?:the |a |an )?[a-z']+)*\s*$"#
);
pattern!(
    RECALL_AMOUNT,
    r#"(?i)^(?:how\s+(?:much|many|old|heavy|tall|far|big)|what\s+age|what\s+size)\b"#
);
pub(super) const RECALL_KIND_NOUN: &[(&str, &str)] = &[
    ("age", "amount"),
    ("apr", "amount"),
    ("budget", "amount"),
    ("date", "time"),
    ("day", "time"),
    ("hours", "amount"),
    ("name", "person"),
    ("rent", "amount"),
    ("salary", "amount"),
    ("shift", "amount"),
    ("size", "amount"),
    ("target", "amount"),
    ("time", "time"),
    ("wage", "amount"),
];
pattern!(
    RECALL_KIND_NOUN_ASK,
    r#"(?i)^(?:my|our)\s+(budget|rent|hours|shift|salary|wage|target|apr|age|size|name|day|date|time)\b"#
);
pattern!(
    RECALL_NOT,
    r#"(?i)\b(?:what|anything|something)\s+(?:do|does|should|can|could)\s+(?:i|you)\s+(?:say|tell)\b|\bhow\s+(?:do|does|can|could|should|would)\s+(?:i|you|we|u|ya)\s+tell\b"#
);
pattern!(
    RECALL_OF_MINE,
    r#"(?i)\bremind(?:ing)? me (?:what|who|which|where|when)\b|\bwhat (?:did|do) i (?:say|told you|tell you|tell u|ask|asked|decide|choose)\b|\bwhat (?:was|am) i (?:saying|doing|about to say)\b"#
);
pattern!(
    RECALL_PERSON,
    r#"(?i)^(?:who|what\s+name|(?:her|his|their|my|our)\s+name)\b"#
);
pattern!(RECALL_PLACE, r#"(?i)^where\b"#);
pattern!(RECALL_PRONOUN, r#"(?i)^(?:you|me|us|it|that|then)\s+"#);
pattern!(
    RECALL_SAID,
    r#"(?i)\b(?:did|do)\s+(?:i|you)\s+(?:say|said|told|tell|mention|remember)\b"#
);
pattern!(
    RECALL_SHAPE_BEFORE,
    r#"(?i)(?:\bwhen|\bwhat\s+time|\bwhat\s+date|\bwhat\s+day|\bhow\s+(?:long|soon|often)|\bhow\s+many\s+(?:weeks|days|months|years|hours|minutes|nights|times)|\bhow\s+(?:much|many|old|far)|\bwhat\s+age|\bwhat\s+size|\bwhere|\bwho|\bwhat\s+name)\s*$"#
);
pattern!(
    RECALL_TIME,
    r#"(?i)^(?:when|what\s+time|what\s+date|what\s+day|how\s+(?:long|soon|often)|how\s+many\s+(?:weeks|days|months|years|hours|minutes|nights|times))\b"#
);
pattern!(
    REPORTED,
    r#"(?i)\b(?:she|he|they|mum|dad|mom|someone|somebody|my \w+)\b.{0,25}\b(?:asked|told|wonder\w*|wants to know|keeps asking)\b\s*(?:(?:me|us|him|her|them|that|about|to)\s*){0,2}(?:what|whether|if)\b"#
);
pub(super) const SPECIFIC_ASKS: &[&str] = &[
    "choice",
    "compare",
    "definition",
    "how_long",
    "how_much",
    "how_often",
    "list",
    "steps",
    "where",
    "which",
    "who",
    "why",
];
pattern!(
    STEPS_ARE_COUNTED,
    r#"(?i)\b\d[\d,]*\s*(?:k\s+)?steps?\b|\bsteps? count\b|\bsteps? (?:a|per|each|every) (?:day|week|month|hour|mile|km|kilometre)\b"#
);
pattern!(
    STEPS_ASK,
    r#"(?i)\bwhat (?:are|is) the steps\b|\bsteps (?:to|for|in)\b|\bstep[- ]by[- ]step\b|\bwalk me through\b|\bwhat(?:'s| is) the process\b|\bthe process (?:for|of|to)\b|\bhow do i go about\b|\bwhat order\b|\bin what order\b|\b(?:the )?(?:best|right|correct|proper) order (?:to|for|in)\b"#
);
pattern!(
    STEPS_AS_A_DISTANCE,
    r#"(?i)\bsteps? (?:in|into|to|per|versus|vs)\s+(?:a |one )?(?:mile|miles|km|kilometre|kilometres|kilometer|kilometers)\b|\b(?:mile|miles|km|kilometre)s? (?:in|to|from|of) (?:a |one )?\d[\d,]*\s*(?:k\s+)?steps?\b"#
);
pattern!(
    SUBSTITUTE,
    r#"(?i)\bwhat (?:do|should|shall|can|could) (?:i|we) use\b(?! (?:for|on|to)\b)|\bwhat (?:else )?(?:do|should|shall|can|could) (?:i|we) (?:use|try|have|make)\b.{0,25}\binstead\b"#
);
pattern!(
    SUM_ASKS,
    r#"(?i)\b(?:\d+|half|quarter|third|two thirds|three quarters|a few)\b.{0,30}\bin (?:grams?|gms?|kg|kilos?|kilograms?|ml|millilitres?|litres?|liters?|cups?|ounces?|oz|pounds?|lbs?|tablespoons?|tbsp|teaspoons?|tsp|pints?|centimetres?|cm|inches|feet|metres?)\b|\b(?:half|a quarter|a third|two thirds|three quarters|three quarters|a fifth) of \d+|\bas a percentage\b|\bpercent (?:off|on top)\b|\b\d+\s*(?:%|percent)\b.{0,25}\b(?:off|tip|on top|between|split)\b|\bsplit (?:the )?(?:bill|cost|rent|fare|total|it|\d|\u00a3)"#
);
pattern!(
    SUM_OUT_LOUD,
    r#"(?i)^(?:subtract|add|multiply|divide|solve|factorise|factorize|work out|convert|split|minus|plus|double|halve|times)\b|\bhow much is\b"#
);
pattern!(
    TAIL_VERDICT_ASK,
    r#"(?i)\b(?:is|was|are)\s+(?:it|that|this)\s+(?:worth it|worth|ok|okay|fine|right|normal|bad|a good (?:idea|plan|buy|choice))[\s.?!]*$"#
);
pattern!(
    THING_THAT_EXTENDS,
    r#"(?i)\bhow long (?:is|are|was|were|will be|would be) (?:(?:the|a|an|my|our|your|this|that)\s+)?(?:[a-z'-]+\s+){0,2}?(?:rivers?|nile|amazon|thames|severn|danube|rhine|mississippi|walls?|pitch(?:es)?|playing fields?|marathons?|bridges?|tunnels?|canals?|motorways?|roads?|runways?|coastlines?|coast|borders?|beaches?|whales?|snakes?|crocodiles?|sharks?|string|ropes?|hose|cables?|pools?|tracks?|corridors?|driveways?)\b(?=\s*(?:[.?!,;:]|$|\s+(?:of|in|on|at|from|to|by)\b))(?!\s+(?:pregnant|old|big|tall|wide|deep|long|away|till|until|now)\b)"#
);
pattern!(
    VERDICT_FRAME,
    r#"(?i)^\W*(?:is|was|are|were|does|do|did)\s+(?:that|this|it|these|those|they)\s+(?:a|an|the|too|really|actually|still|about|much)?\s*(?:good|normal|average|typical|usual|fair|ok|okay|alright|right|fine)\b"#
);
pattern!(WHAT_AGE, r#"(?i)\bwhat age\b"#);
pattern!(
    WHAT_ARE_MINE,
    r#"(?i)\bwhat(?:'s| is| are|s)\b[^?]{0,40}?\b(?:my|our) (?:[a-z]+ ){0,2}(?:rights?|entitlements?|obligations?)\b"#
);
pattern!(
    WHAT_ARE_TIMES,
    r#"(?i)^\W*(?:(?:so|and|ok|okay|oh|also|right)\s+)*what(?:'s| is| are|s)\b[^?]{0,60}?\b(?:opening|visiting|closing|trading|check ?in|check ?out|business|office|term|school|library|swimming|bus|train|ferry|flight|shop|bank|tip|pool|gym|leisure|surgery|pharmacy|restaurant|pub|museum) ?(?:times|hours|dates)\b"#
);
pattern!(WHAT_CAN_I_DO, r#"(?i)\bwhat (?:can|could) (?:i|we) do\b"#);
pattern!(
    WHAT_CAN_I_DO_WITH,
    r#"(?i)\bwhat (?:(?:can|could) (?:i|we)|to) do with\b"#
);
pattern!(
    WHAT_DOES_MEASURE,
    r#"(?i)\bwhat (?:does|do)\b(?:(?!\bwhat\b)[^?]){0,30}?\b(?:cost|costs|charge|charges|weigh|weighs)\b|\bwhat (?:does|do)\b(?:(?!\bwhat\b)[^?]){0,30}?\bcome(?:s)? (?:to|out at|out as)\b|\bwhat (?:do|does|would|will|should|can|could|did) .{0,40}?\b(?:charge|charges|cost|costs)\b"#
);
pattern!(
    WHAT_ELSE_STEPS,
    r#"(?i)\bwhat else (?:can|could|should|shall|do|would) (?:i|we|you) (?:do|try)\b"#
);
pattern!(
    WHAT_ELSE_WITH,
    r#"(?i)\bwhat else (?:can|could|should|shall|do|would) (?:i|we|you)\b[^.!?]{0,25}?\bwith\b"#
);
pattern!(
    WHAT_FIGURE,
    r#"(?i)\bwhat (?:temperature|temp|heat|gas mark|setting|speed|depth|size|weight|voltage)\b"#
);
pattern!(
    WHAT_HELPS,
    r#"(?i)\bwhat (?:helps|can help|might help|will help|would help|works)\b"#
);
pattern!(
    WHAT_IS_PER,
    r#"(?i)\bwhat(?:'s|s| is)\b[^?]{0,40}\b(?:a|an|per|each|every) (?:month|week|year|day|hour|litre|liter|kg|kilo|pint|mile|gb|gigabyte|night|visit|session|person|head|class|lesson)\b"#
);
pattern!(
    WHAT_I_GROW,
    r#"(?i)\bwhat (?:do|should|shall|can|could|would) (?:i|we) (?:grow|sow|plant|batch|bake|roast)\b"#
);
pattern!(
    WHAT_I_NEED,
    r#"(?i)\bwhat (?:do|does|will|would|should|shall|might|can) (?:i|we) (?:actually |really |even |truly |possibly |then |now )*(?:need|require)\b(?!\s+(?:to\s+)?(?:do|know|say|ask|tell|learn|understand|work|watch|avoid|stop|prevent|fix|deal|look|sort|handle|word|phrase)\b)"#
);
pattern!(
    WHAT_I_WRITE,
    r#"(?i)\bwhat (?:do|should|shall|can|could|would) (?:i|we) (?:write|say|text|reply|word|phrase|sign)\b"#
);
pattern!(
    WHAT_SHOULD_I_DO_WITH,
    r#"(?i)\bwhat (?:should|shall|do|did|would|might) (?:i|we) do with\b"#
);
pattern!(
    WHAT_THING_FRAGMENT,
    r#"(?i)(?:^|[,;]\s*)what (?!about\b|is\b|are\b|was\b|were\b|if\b|else\b|next\b|now\b|size\b|time\b|colour\b|color\b|kind\b|sort\b|type\b|age\b|happens\b|went\b|works\b)([a-z]{3,}s)\b\s*[.!?]*\s*$"#
);
pattern!(WHAT_TIME_NAMED, r#"(?i)\bwhat time\b"#);
pattern!(
    WHAT_TO_BAKE,
    r#"(?i)\bwhat (?:do|should|shall|can|could) (?:i|we|he|she|they) (?:bake|roast)\b|\bwhat (?:do|should|shall|can|could) (?:my|our|his|her|their) \w+ (?:bake|roast)\b|\bwhat to (?:bake|roast)\b"#
);
pattern!(
    WHAT_TO_DO,
    r#"(?i)\bwhat (?:do|should|shall|can|could|would) (?:i|we) (?:do|say|take|try|check|avoid|ask|tell|help|start|stop|keep)\b"#
);
pattern!(
    WHAT_TO_OPENS,
    r#"(?i)^(?:so |and |ok |okay |now |also |please |but )?what to\b|\b(?:i (?:don'?t|do not) know|not sure|no idea|dunno) what to\b"#
);
pattern!(
    WHAT_TO_PICK,
    r#"(?i)\bwhat to (?:cook|make|eat|have|drink|watch|read|listen to|wear|get|buy|order|play|study|plant|visit|give|bring|pack)\b"#
);
pattern!(
    WHAT_TO_STEPS,
    r#"(?i)\bwhat to (?:do|say|write|text|reply|feed|serve|take|try|check|avoid|ask|tell|keep)\b"#
);
pattern!(
    WHAT_YOU_DO,
    r#"(?i)\bwhat (?:do|does|would|should|can|could) (?:you|one) (?:do|try)\s+(?:about|with|if|when|for|to)\b"#
);
pattern!(
    WHAT_YOU_TAKE,
    r#"(?i)\bwhat (?:do|does|would|should|can|could|shall) (?:you|one) (?:take|bring|pack|give|get|need|buy)\b(?! (?:for|to do|to get|to make)\b)|\bwhat (?:do|does|would|should|can|could|shall) (?:i|we) (?:take|bring|pack)\b(?! (?:for|to do|to get|to make)\b)"#
);
pattern!(
    WHEN_ASKS,
    r#"(?i)^(?:when'?s|when|what time|what day|what date|what month|what year|which day|what weekday) (?:should|shall|is|are|was|were|will|would|does|do|did|can|could|to|am|has|have|of|had)\b"#
);
pattern!(
    WHERE_ALONE,
    r#"(?i)^(?:and |so |ok |okay )?where(?:'s|s| is| are) (?:it|that|this)$|^(?:and |so |ok |okay )?where (?:i|we|you) (?:live|work)$|^where (?:do|does|did) i (?:live|work)$"#
);
pattern!(
    WHERE_MONEY_GOES,
    r#"(?i)\bwhere (?:does|do|did) (?:it all |all )?(?:my|our|the|his|her|your) (?:money|pay|wages|salary|income|budget|earnings|time)\b[^?]{0,20}\b(?:go|goes|went)\b|\bwhere (?:my|our|the|his|her|your) (?:money|pay|wages|salary|income|budget|time) (?:goes|go|went)\b"#
);
pattern!(
    WHERE_POINT,
    r#"(?i)\bwhere(?:'s|s| is| was)\b[^?]{0,20}\b(?:the|any) point\b"#
);
pattern!(
    WHERE_RECALL,
    r#"(?i)\bwhere (?:am|was) i\b|\bwhere (?:were|are) we\b|\bwhere we (?:left|were|are)\b|\bremind me where\b|\b(?:know|knows|known|tell me|told me|remind|remember|forgot|forget|said)\b[^?]{0,25}\bwhere (?:i|we|you) (?:live|work|are|were|went|put)\b|\b(?:know|knows|known|tell me|told me|remind|remember|forgot|forget|said)\b[^?]{0,25}\bwhere (?:he|she|they) (?:live|lives|work|works|stay|stays)\b|\bwhere (?:does|did) (?!a |an |the )[a-z]{2,}(?: [a-z]{2,})? (?:live|work|stay)\b|\bwhere (?:do|does|did) (?:i|you|he|she|we|they) (?:live|work|stay)\b|\bwhere (?:do|does|did) (?:my|our|your|his|her|their) \w+(?: \w+)? (?:live|work|stay)\b|\bwhere (?:do|does|did) i (?:decide|say|tell|mean|think|go\b[^?]{0,12}\b(?:last|before))|\bwhere did we (?:leave off|left off|leave it|left it|get to|got to)\b|\bwhere (?:did|do) we (?:get up to|got up to)\b|\bwhere(?:'s|s| is| are| did i put| have i put| can i find)\b[^?]{0,20}\b(?:my|me|our|the)\s+(?:phone|keys|key|wallet|purse|glasses|bag|bags|parcel|nappy|nappies|coat|remote|passport|documents|bus pass|ticket|tickets)\b"#
);
pattern!(
    WHERE_RELATIVE,
    r#"(?i)\bwhere\s+(?:i|we|you|he|she|they|it|my|your|our|his|her|their)\b[^?.]{0,20}?\b(?:am|is|are|was|were|live|lives|stay|stays|sit|sits|work|works|go|goes)\b"#
);
pattern!(
    WHERE_START,
    r#"(?i)\bwhere (?:do|does|should|shall|can|could|would|will|to) (?:\w+ ){0,3}(?:start|begin)\b|\bwhere to (?:begin|start)\b"#
);
pattern!(
    WHICH_DAY_IS_BEST,
    r#"(?i)\b(?:best|better|rather|prefer|do you think|recommend)\b"#
);
pattern!(
    WHO_AND_MEASURE,
    r#"(?i)\bwho\b[^?]{0,40}\b(?:what|how much|how many)\b"#
);
pattern!(
    WHO_OF_A_NOUN,
    r#"(?i)\b(?:the|a|an|this|that|these|those|someone|somebody|anyone|anybody|everyone|everybody|no one|nobody|person|people|man|woman|boy|girl|child|kid|friend|colleague|neighbour|neighbor|boss|manager|teacher|nurse|doctor|carer|minder|babysitter)\s+who\b"#
);
pattern!(
    WHO_OF_A_PRONOUN,
    r#"(?i)\bwho (?:i|you|we|they|he|she|it) (?:am|are|is|was|were)\b"#
);
pattern!(
    WHO_OPENS,
    r#"(?i)^\W*(?:(?:and|so|but|then|or|also|now|ok|okay)\s+)?who(?:'s|s|se)?\b"#
);
pattern!(
    WRITTEN_THING,
    r#"(?i)\bhow long (?:should |would |will |does |do |can )?(?:a|an|my|our|your|the|this|that)\s+(?:[a-z]+\s+){0,2}(?:personal statement|statement|essay|dissertation|thesis|coursework|assignment|cover(?:ing)? letter|letter|cv|resume|blog post|post|article|abstract|summary|report|review|answer|paragraphs?|chapter|book|story|speech|presentation|email|text|message|comment|description|bio|application|proposal|synopsis|blurb|poem|script|personal profile|profile)\s+be\b"#
);
pattern!(
    WRONG_ASKED_ABOUT,
    r#"(?i)\b(?:mean|means|meaning|meant|called|spell|spelled|word)\b"#
);
pattern!(
    WRONG_RECALL,
    r#"(?i)\bremind me\b|\bwhat (?:was|were)\b[^?]{0,30}\b(?:wrong|problem|trouble|issue|fault)\b|\b(?:wrong|problem|trouble|issue|fault)\b[^?]{0,20}\bagain\b"#
);
pattern!(
    ASK_CLAUSE_OPENS,
    r#"(?i)^(?:when|what|which|how|where|who|why|whom|whose|should|shall|can|could|do|does|did|will|would|is|are|was|were|am|have|has|had)\b"#
);
pattern!(
    ASK_CLAUSE_SPLIT,
    r#"(?i)\s*(?:[,;:]|\band\b|\bbut\b|\bso\b|\bthen\b)\s*"#
);
pattern!(
    MEASURE_THEN_VERB,
    r#"(?i)\b(?:how long|how soon|what time)\s+(?:do|does|did|will|would|should|shall|can|could|is|are|was|were|to|till|until|before|in|on|for|per|at|with|by|after|into|under|over|about|from)\b"#
);
pattern!(
    TALKING_ABOUT_A_LENGTH,
    r#"(?i)\b(?:i|we|you|they|he|she)\s*(?:did ?n[o']t|do ?n[o']t|does ?n[o']t|never)?\s*(?:know|knew|realise|realize|remember|wonder|wondered|sure|idea)\b|\b(?:amazing|surprising|interesting|funny|odd|strange) how (?:long|soon|much|many|often)\b|\bno idea how\b|\bhow long it (?:takes|took|would take|will take)\b"#
);
pattern!(
    TRAILING_MEASURE,
    r#"(?i)\b(?:how long|how soon|what time)\s*[.?!]*$"#
);
pattern!(
    NOTHING_TO_A_THING,
    r#"(?i)\bnothing (?:to|for) (?:wear|put on|bring|take|pack|cook|make|eat|have|drink|watch|read|give|buy|get|play)\b"#
);
pattern!(
    WHAT_TO_LOOK,
    r#"(?i)\bwhat\s+(?:to|(?:do|should|shall|can|could|must)\s+(?:i|we))\s+(?:need to\s+|have to\s+|ought to\s+)?(?:look(?!\s+like\b)(?:\s+(?:out\s+|around\s+)?for)?|check(?:\s+for)?|watch\s+out\s+for|ask(?:\s+for)?|avoid)\b"#
);
pub(super) const OPENS_A_QUESTION: &[&str] = &["how", "what", "which"];
pattern!(ALTERNATIVES, r#"(?i)\bor\b"#);
pattern!(
    MONEY_VERB_ASK,
    r#"(?i)\bwhat should (?:i|we) (?:tip\b|spend (?:on)(?! it)\b|budget for\b|pay (?:for|a|an|my|the|him|her|them)\b)"#
);
pattern!(
    NOT_A_PRESENT_ASK,
    r#"(?i)\b(?:thank(?:s| you)|refund|receipt|return(?:ed|ing)?|buying|bought|got|gave|given|giving|keeps)\b"#
);
pattern!(
    PRESENT_LOOKING,
    r#"(?i)\b(?:present|gift|pressie)s?\b[^?.!]{0,20}?\bfor\b"#
);
pattern!(
    SPEND_IT_ON,
    r#"(?i)\bwhat (?:should|shall|do) (?:i|we) (?:spend|put) it (?:on|into|in)\b"#
);
pub(super) const LABEL_AFTER_COMPARATIVE: &[&str] = &[
    "actually",
    "after",
    "again",
    "already",
    "here",
    "honestly",
    "now",
    "since",
    "thank",
    "thanks",
    "though",
    "today",
    "tomorrow",
    "tonight",
    "yesterday",
    "yet",
];
pattern!(
    LABEL_ASKS_ABOUT_A_PERSON,
    r#"\bwhat (?:is|are|r) (?:you|u|ya|we|us|they)\b"#
);
pub(super) const LABEL_ASK_FOR_A_NAME: &[&str] = &[
    "find",
    "found",
    "knew",
    "know",
    "knows",
    "recommend",
    "recommends",
    "suggest",
    "suggests",
];
pattern!(
    LABEL_A_DATE_FIGURE,
    r#"(?i)\b\d{1,2}(?:st|nd|rd|th)\b|\b\d{1,2}\s+(?:of\s+)?(?:january|february|march|april|may|june|july|august|september|october|november|december)\b|\b(?:january|february|march|april|may|june|july|august|september|october|november|december)\s+\d{1,2}\b"#
);
pub(super) const LABEL_A_PERSON_AFTER: &[&str] = &[
    "brother", "bud", "buddy", "daughter", "father", "fella", "friend", "friend's", "friends",
    "friends'", "half", "husband", "lad", "man", "mate", "mate's", "mates", "matey", "men",
    "mother", "pal", "pals", "partner", "regards", "sister", "son", "wife", "wishes", "woman",
    "women",
];
pattern!(
    LABEL_BARE_DEFINITION,
    r#"(?i)^\s*(?:(?:and|but|so|then|ok|okay|right|oh|well|now|sorry|hi|hello|please)\s+)*what(?:'s|s| is| are| was)?\s+(?:is|are|was)\s+([a-z][a-z'’-]{1,28})(?![^?]*\b(?:like|for|worth|good|cheap|cheapest|easier|easiest|better|best|worse|worst|date|time|day|weather|price|cost|news|forecast|temperature|showing|playing|opening|score|length|size)\b)"#
);
pub(super) const LABEL_COMPARATIVE: &[&str] = &["best", "better"];
pub(super) const LABEL_COMPARE: &[&str] = &[
    "compare",
    "difference between",
    "different from",
    "or is it",
    "which one",
];
pattern!(
    LABEL_COMPARISON,
    r#"(?:\s+or\s+|\bbetter than\b|\binstead of\b|\bversus\b|\bvs\b)"#
);
pub(super) const LABEL_DEFINITION: &[&str] = &[
    "difference between",
    "explain",
    "meaning of",
    "what are",
    "what does",
    "what is a",
    "what is an",
];
pub(super) const LABEL_DESCRIBES: &[&str] = &[
    "am", "any", "are", "ate", "be", "been", "being", "cope", "coping", "did", "do", "does",
    "doing", "done", "eat", "eating", "feel", "feeling", "feels", "felt", "get", "gets", "getting",
    "got", "is", "look", "looking", "looks", "lot", "manage", "managing", "much", "seem", "seems",
    "sleep", "sleeping", "slept", "was", "were",
];
pattern!(
    LABEL_HELP_ASK,
    r#"\b(?:anything|something)\s+(?:else\s+)?(?:i|we|you)\s+(?:can|could|should|might|would)\s+(?:do|try|use|take|add|change)\b|\b(?:anything|something)\s+(?:else\s+)?(?:that|which)\s+(?:helps|would help|might help|will help|works)\s*[?.!]*\s*$|\bany\s+(?:way|ways)\s+(?:of|to|round)\b|\bis there any\s+(?:way|ways|thing)\s+(?:of|to|i|we|you|else)\b"#
);
pattern!(
    LABEL_HELP_ASK_NOT,
    r#"\b(?:no|not|isnt|isn't|wasnt|wasn't|arent|aren't|cant|can't|couldnt|couldn't|dont|don't|doesnt|doesn't|didnt|didn't|never|nothing|without|hardly)\b"#
);
pattern!(
    LABEL_HELP_ASK_OWN_SUBJECT,
    r#"\b(?:my|our|his|her|their|your|the|a|an)\s+[a-z]"#
);
pattern!(
    LABEL_HELP_ASK_SUBJECT,
    r#"\b(?:is|was|are|were|seems|seemed|feels|felt|sounds|sounded|helps|helped|works|worked|suits|suited|fits|fitted|does|did|its|thats|im|youre|(?:would|will|might|must|could|can|may) be)\b"#
);
pattern!(LABEL_HELP_ASK_TAIL_CLAUSE, r#"\|\s*\w+\s+\w+"#);
pub(super) const LABEL_MATH_WORDS: &[&str] = &[
    "average",
    "calculate",
    "divide",
    "divided",
    "double",
    "equals",
    "half",
    "minus",
    "percent",
    "percentage",
    "plus",
    "sum",
    "times",
    "total",
    "triple",
];
pub(super) const LABEL_NOT_A_DEFINITION_HEAD: &[&str] = &[
    "a",
    "about",
    "against",
    "all",
    "an",
    "any",
    "anyone",
    "anything",
    "at",
    "behind",
    "best",
    "better",
    "between",
    "biggest",
    "by",
    "cheapest",
    "coming",
    "cooking",
    "double",
    "easiest",
    "easy",
    "eating",
    "eight",
    "eighteen",
    "eighty",
    "eleven",
    "everyone",
    "everything",
    "fifteen",
    "fifty",
    "five",
    "for",
    "forty",
    "four",
    "fourteen",
    "from",
    "going",
    "good",
    "half",
    "happened",
    "happening",
    "he",
    "her",
    "his",
    "hundred",
    "i",
    "in",
    "inside",
    "it",
    "its",
    "just",
    "largest",
    "left",
    "longest",
    "million",
    "missing",
    "more",
    "most",
    "my",
    "near",
    "new",
    "next",
    "nice",
    "nine",
    "nineteen",
    "ninety",
    "none",
    "not",
    "nothing",
    "occurring",
    "of",
    "off",
    "on",
    "one",
    "our",
    "out",
    "over",
    "percent",
    "playing",
    "quarter",
    "quickest",
    "red",
    "seven",
    "seventeen",
    "seventy",
    "she",
    "shortest",
    "showing",
    "six",
    "sixteen",
    "sixty",
    "smallest",
    "so",
    "some",
    "someone",
    "something",
    "ten",
    "than",
    "that",
    "the",
    "their",
    "there",
    "these",
    "they",
    "third",
    "thirteen",
    "thirty",
    "this",
    "those",
    "thousand",
    "three",
    "to",
    "too",
    "triple",
    "twelve",
    "twenty",
    "two",
    "under",
    "up",
    "we",
    "with",
    "worth",
    "wrong",
    "you",
    "your",
];
pub(super) const LABEL_OPINION: &[&str] = &[
    "do you like",
    "do you prefer",
    "how about you",
    "in your opinion",
    "what about you",
    "what do you think",
    "what would you",
    "would you rather",
    "your favorite",
    "your favourite",
];
pattern!(
    LABEL_QUESTION_WORD,
    r#"\b(?:why|when|where|how|who|whom|whose|which|what)\b"#
);
pub(super) const LABEL_RECOMMEND: &[&str] = &[
    "advice",
    "best",
    "better",
    "choose",
    "idea",
    "ideas",
    "option",
    "options",
    "pick",
    "recommend",
    "recommendation",
    "suggest",
    "suggestion",
    "worth",
];
pub(super) const LABEL_RECOMMEND_PHRASES: &[&str] = &[
    "any good",
    "any ideas",
    "any suggestions",
    "anything good",
    "help me choose",
    "help me pick",
    "something good",
    "what do you recommend",
    "what should i read",
    "what should i watch",
    "what would you recommend",
    "worth reading",
    "worth watching",
];
pattern!(
    LABEL_REFUSED_PAIR,
    r#"^\s*(?:no|not|never|without|neither)\b|\b(?:dont|don'?t|do not|doesnt|doesn'?t|didnt|didn'?t|wont|won'?t|wouldnt|wouldn'?t|cant|can'?t|cannot|shouldnt|shouldn'?t)\s+(?:\w+\s+){0,2}?(?:want|wants|wanted|like|likes|love|eat|eating|have|having|drink|drinking|wear|do|doing|need|needs|use|fancy|stand|bother|manage)\b|\ballergic\b|\bhate\b"#
);
pattern!(
    LABEL_SCALE_ASK,
    r#"^\s*(?:is|are|was|were|will|would|should|when|what|how|which|be)\b"#
);
pub(super) const LABEL_STEPS: &[&str] = &[
    "get started",
    "how can i",
    "how do i",
    "how should i",
    "how to",
    "step by step",
    "walk me through",
    "what should i",
];
pattern!(LABEL_STEPS_WORD, r#"\bsteps\b"#);
pattern!(LABEL_TIPS, r#"\b(tips|hints|pointers|tip|hint|pointer)\b"#);
pattern!(LABEL_TIPS_ABOUT, r#"\s+(?:for|on|about|when|to)\s+\w"#);
pattern!(LABEL_TIPS_AMOUNT, r#"(?:\d|%|\b(?:a|the)\s+\w+\s*%)\s*$"#);
pub(super) const LABEL_TIPS_FRAME: &[&str] = &[
    "a", "about", "advice", "an", "and", "any", "are", "do", "few", "for", "get", "give", "got",
    "has", "have", "hint", "hints", "i", "is", "it", "look", "looking", "me", "my", "need",
    "needed", "needs", "on", "or", "please", "pls", "pointer", "pointers", "send", "share", "some",
    "thank", "thanks", "that", "the", "this", "tip", "tips", "to", "u", "us", "wanted", "you",
];
pattern!(
    LABEL_TIPS_SUBJECT,
    r#"\b(?:i|you|we|they|he|she|it|who|that|this|people|everyone|nobody|someone)\s*$"#
);
pattern!(
    LABEL_TIPS_WANTED,
    r#"\b(?:any|some|a few|few|got|need|needs|wanted|looking for|share|send|give(?: me)?)\s+(?:\w+\s+){0,2}$"#
);
pattern!(
    LABEL_VERDICT_ASK,
    r#"(?:and |so |but |ok |okay |also |sorry |oh |hmm )*(?:(?:is|was|are|were|does|do|did) (?:that|this|it)(?: all)?(?: (?:really|actually|very|too|that|so))?(?: (?:bad|normal|serious|dangerous|ok|okay|fine|safe|common|a|much of a) ?(?:problem|worry|issue|big deal)?| (?:going to|gonna) (?:get|be) worse| (?:going to|gonna) (?:spread|get worse))|(?:is|was) (?:that|this|it)(?: (?:going to|gonna))? (?:get worse|getting worse|spread|serious)|(?:is|was) (?:he|she|they) right|(?:are|were|is|was) (?:they|these|those) (?:safe|ok|okay|fine|normal|bad|a problem|a risk|any good)|(?:should|do|ought) i (?:be )?(?:worried|worry|worrying|bother)|does (?:that|this|it) matter|will (?:that|this|it) (?:get|be) (?:worse|ok|okay|fine))(?: then| though| honestly)?"#
);
pattern!(
    SIDE_PAIR_0,
    r#"(?i)\bbetween (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?) and (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?)"#
);
pattern!(
    SIDE_PAIR_1,
    r#"(?i)\bcompare[sd]? (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?) (?:and|with|to) (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?)"#
);
pattern!(
    SIDE_PAIR_2,
    r#"(?i)([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?) apart from (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?)"#
);
pattern!(
    SIDE_PAIR_3,
    r#"(?i)([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?) (?:vs\.?|versus) (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?)"#
);
pattern!(
    SIDE_PAIR_4,
    r#"(?i)([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?) compar(?:e|ed) (?:with|to) (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?)"#
);
pattern!(
    SIDE_PAIR_5,
    r#"(?i)([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?) (?:is |are |was |were )?(?:comparable to) (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?)"#
);
pattern!(
    SIDE_PAIR_6,
    r#"(?i)([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?) (?:is |are |was |were )?(?:different|differs?) (?:from|to) (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?)"#
);
pattern!(
    SIDE_PAIR_7,
    r#"(?i)([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?) (?:better|worse|cheaper|dearer|quieter|louder|faster|slower|easier|harder|healthier|safer|simpler|tastier|stronger) than (?:a |an |the )?([\w'-]+(?: (?!and\b|from\b|than\b|vs\b|versus\b)[\w'-]+)?)"#
);
pattern!(
    LABEL_OPPOSITE_PAIRS,
    r#"\b(?:(?:the |a |an )?early\s+or\s+(?:the |a |an )?late|(?:the |a |an )?late\s+or\s+(?:the |a |an )?early|(?:the |a |an )?earlier\s+or\s+(?:the |a |an )?later|(?:the |a |an )?later\s+or\s+(?:the |a |an )?earlier|(?:the |a |an )?sooner\s+or\s+(?:the |a |an )?later|(?:the |a |an )?later\s+or\s+(?:the |a |an )?sooner|(?:the |a |an )?soon\s+or\s+(?:the |a |an )?late|(?:the |a |an )?late\s+or\s+(?:the |a |an )?soon|(?:the |a |an )?hot\s+or\s+(?:the |a |an )?cold|(?:the |a |an )?cold\s+or\s+(?:the |a |an )?hot|(?:the |a |an )?warm\s+or\s+(?:the |a |an )?cold|(?:the |a |an )?cold\s+or\s+(?:the |a |an )?warm|(?:the |a |an )?big\s+or\s+(?:the |a |an )?small|(?:the |a |an )?small\s+or\s+(?:the |a |an )?big|(?:the |a |an )?large\s+or\s+(?:the |a |an )?small|(?:the |a |an )?small\s+or\s+(?:the |a |an )?large|(?:the |a |an )?long\s+or\s+(?:the |a |an )?short|(?:the |a |an )?short\s+or\s+(?:the |a |an )?long|(?:the |a |an )?high\s+or\s+(?:the |a |an )?low|(?:the |a |an )?low\s+or\s+(?:the |a |an )?high|(?:the |a |an )?good\s+or\s+(?:the |a |an )?bad|(?:the |a |an )?bad\s+or\s+(?:the |a |an )?good|(?:the |a |an )?better\s+or\s+(?:the |a |an )?worse|(?:the |a |an )?worse\s+or\s+(?:the |a |an )?better|(?:the |a |an )?wet\s+or\s+(?:the |a |an )?dry|(?:the |a |an )?dry\s+or\s+(?:the |a |an )?wet|(?:the |a |an )?sunny\s+or\s+(?:the |a |an )?rainy|(?:the |a |an )?rainy\s+or\s+(?:the |a |an )?sunny|(?:the |a |an )?fast\s+or\s+(?:the |a |an )?slow|(?:the |a |an )?slow\s+or\s+(?:the |a |an )?fast|(?:the |a |an )?quick\s+or\s+(?:the |a |an )?slow|(?:the |a |an )?slow\s+or\s+(?:the |a |an )?quick|(?:the |a |an )?cheap\s+or\s+(?:the |a |an )?expensive|(?:the |a |an )?expensive\s+or\s+(?:the |a |an )?cheap|(?:the |a |an )?busy\s+or\s+(?:the |a |an )?quiet|(?:the |a |an )?quiet\s+or\s+(?:the |a |an )?busy|(?:the |a |an )?loud\s+or\s+(?:the |a |an )?quiet|(?:the |a |an )?quiet\s+or\s+(?:the |a |an )?loud|(?:the |a |an )?full\s+or\s+(?:the |a |an )?empty|(?:the |a |an )?empty\s+or\s+(?:the |a |an )?full|(?:the |a |an )?deep\s+or\s+(?:the |a |an )?shallow|(?:the |a |an )?shallow\s+or\s+(?:the |a |an )?deep|(?:the |a |an )?heavy\s+or\s+(?:the |a |an )?light|(?:the |a |an )?light\s+or\s+(?:the |a |an )?heavy|(?:the |a |an )?thick\s+or\s+(?:the |a |an )?thin|(?:the |a |an )?thin\s+or\s+(?:the |a |an )?thick|(?:the |a |an )?wide\s+or\s+(?:the |a |an )?narrow|(?:the |a |an )?narrow\s+or\s+(?:the |a |an )?wide|(?:the |a |an )?hard\s+or\s+(?:the |a |an )?soft|(?:the |a |an )?soft\s+or\s+(?:the |a |an )?hard|(?:the |a |an )?early\s+or\s+(?:the |a |an )?on time|(?:the |a |an )?on time\s+or\s+(?:the |a |an )?early|(?:the |a |an )?in\s+or\s+(?:the |a |an )?out|(?:the |a |an )?out\s+or\s+(?:the |a |an )?in)\b"#
);
