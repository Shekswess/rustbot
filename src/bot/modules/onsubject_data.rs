//! Patterns and response pools from PyBot 7d63c7fbebb44c7e466e5c8b5c1f341ab767929e.
pub(super) const A_ASK_OPENER: &str = r###"^\s*(?:can|could|do|does|did|is|are|was|were|would|will|should|am|shall|have|has|had|what|when|where|which|who|why|how|if)\b"###;
pub(super) const A_BACK_REFERENCE: &str =
    r###"\b(?:it|that|this|there|he|she|they|him|her|them|one|those|these|hers|his|theirs)\b"###;
pub(super) const A_EMPTY_ASK_OPENER: &str = r###"^\s*(?:how|when)\b"###;
pub(super) const A_POINTER_ASK: &str = r###"^\s*(?:which|who)\b|^\s*how\s+come\b"###;
pub(super) const A_HELP_ASK: &str = r###"^\s*(?:what|anything)\s+(?:would\s+|will\s+|might\s+)?helps?\b|^\s*what\s+(?:can|should|could)\s+(?:i|we)\s+do\b|^\s*how\s+(?:do|can|should)\s+i\s+(?:help|calm|support|comfort|reassure)\b"###;
pub(super) const A_RECALL_ASK: &str = r###"^\s*(?:how many|how much)\b(?:(?!\bhow much (?:is|does|do|will|would)\b).)*?\b(?:do i have|have i got|do we have|did i|did we|did he|did she)\b|^\s*how many\b(?:\s+\w+){0,2}\s*$|^\s*how many\b.*\bagain\s*$|^\s*what (?:day|date|time)\b|^\s*whats the (?:day|date|time)\b|^\s*when (?:did|was that|were that|does my|is my|do i|are we)\b|^\s*did you (?:get|catch|hear|note|remember|write)\b|\b(?:what|how old|how long ago) (?:did|do|was) i (?:say|said|tell|told|mention|planning|planning to do|doing)\b|\bdid i (?:say|tell|mention)\b"###;
pub(super) const A_COMPARISON_ASK: &str = r###"\b(?:too (?:far|much|long|late|early|expensive|cheap|old|big|small)|worth it|a lot|any good|good value|cheaper|expensive|good for|reasonable|enough)\b|\b(?:is|was|would) (?:that|it|this) (?:good|bad|fair|ok|okay|decent)\b"###;
pub(super) const A_COOKING_TIME_ASK: &str = r###"\bhow long\b"###;
pub(super) const A_FOOD_PRONOUN: &str = r###"\b(?:it|them|those|that)\b"###;
pub(super) const A_ROASTING_WORD: &str =
    r###"\b(?:roast|roasting|roasted|bake|baking|baked|oven)\b"###;
pub(super) const A_BOILING_WORD: &str = r###"\b(?:boil|boiling|boiled|simmer|simmering)\b"###;
pub(super) const A_OTHER_METHOD: &str = r###"\b(?:grill|grilling|grilled|fry|frying|fried|rest|resting|steam|steaming|poach|poaching|barbecue|barbeque|bbq|microwave|air fryer|toast|toasting)\b"###;
pub(super) const A_NAMED_FOOD: &str = r###"\b(?:chicken|turkey|beef|lamb|pork|ham|gammon|duck|goose|joint|steak|sausage|sausages|fish|salmon|potato|potatoes|egg|eggs|rice|pasta|bread|cake|pastry|pie|casserole|stew|chops|chop|brisket|shoulder|leg|breast|thigh|fillet|kettle)\b"###;
pub(super) const A_COOKING_ASK: &str = r###"\b(?:cook|cooking|cooked|roast|roasting|bake|baking|boil|boiling|fry|frying|grill|oven|freezer|fridge|thaw|defrost|stuffing|gravy|rice|pasta|potato|potatoes|recipe|rest it)\b"###;
pub(super) const A_LOST_THING: &str = r###"\b(?:lost|mislaid|misplaced|can'?t find|cannot find)\b|\bleft (?:it|them|my|the|his|her)\b[^.?!]*\b(?:in|on|at|by)\b"###;
pub(super) const A_MACHINE_CUE: &str =
    r###"\b(?:laptop|computer|phone|tablet|charger|charging|screen|windows|macbook|pc)\b"###;
pub(super) const A_A_QUESTION_ASK: &str = r###"^\s*(?:what|whats|when|whens|where|wheres|which|who|whos|why|whys|how|hows|is|are|was|were|do|does|did|can|could|should|would|will|shall|have|has|am|any|anything)\b"###;
pub(super) const A_JUDGEMENT_ASK: &str =
    r###"\b(?:stupid|silly|daft|mad|unreasonable|out of order|selfish|allowed)\b"###;
pub(super) const A_JUDGEMENT_OF_AMOUNT: &str = r###"\b(?:is|was|would) (?:that|it|this) (?:good|bad|ok|okay|enough|fair|normal|reasonable|a lot|too much|too little|decent)\b"###;
pub(super) const A_WORDS_ASK: &str = r###"^\s*(?:what|whats|how|hows|should|shall|can you|could you|would you|do i need to|give me|help me)\b[^?]*\b(?:write|writing|word|words|say|saying|tell|telling|ask|asking|reply|answer|email|letter|phone|ring|call|complain|phrase)\b"###;
pub(super) const A_NAMING_ASK: &str = r###"\b(?:should|shall|will|would|can|could|do|to) (?:i|we|you) (?:call|name)\b|\b(?:call|name) (?:it|him|her|them)\b"###;
pub(super) const A_EXPLANATION: &str =
    r###"\b(?:thats why|that is why|because|so that|the reason)\b"###;
pub(super) const A_ITS_OWN_SUBJECT: &str = r###"\b(?:little one|little man|youngest|eldest|oldest|the baby)\b|\b(?:he|she|they|him|her|them|son|daughter|child|kid|boy|girl)\b[^?]{0,12}\b\d+\b"###;
pub(super) const A_DVLA_WORD: &str =
    r###"\b(?:mot|dvla|tax|sorn|licence|license|log ?book|v5|clamped)\b"###;
pub(super) const A_THING_VERB_ASK: &str =
    r###"\b(?:renew|renewing|cancel|cancelling|canceled|refund|rebook|upgrade|print|return)\b"###;
pub(super) const A_OPEN_ASK: &str = r###"\bwhat (?:do|should|shall|can|could|would) (?:i|we|you|they) do\b|\bwhat (?:happens|now|next)\b|\bwhat does (?:that|it|this) mean\b"###;
pub(super) const A_THERE: &str = r###"\bthere\b"###;
pub(super) const A_NAMED_PLACE: &str = r###"\b(?:in|to|at|from|visiting|visit)\s+([a-z][\w'-]*)"###;
pub(super) const A_REPEAT_LEAD: &str = r###"^same (?:as before|answer)\b[^:]{0,40}:"###;
pub(super) const A_EXISTENTIAL_THERE: &str = r###"\b(?:is|are|was|were|isn't|aren't|wasn't|weren't)\s+there\b|\bthere(?:'s| is| are| was| were)\b"###;
pub(super) const A_PLACE_WORD: &[&str] = &[
    r###"work"###,
    r###"school"###,
    r###"college"###,
    r###"hospital"###,
    r###"bank"###,
    r###"town"###,
    r###"city"###,
    r###"flat"###,
    r###"flats"###,
    r###"home"###,
    r###"house"###,
    r###"garage"###,
];
pub(super) const A_DETERMINER: &[&str] = &[
    r###"a"###,
    r###"an"###,
    r###"the"###,
    r###"my"###,
    r###"our"###,
    r###"your"###,
    r###"his"###,
    r###"her"###,
    r###"their"###,
    r###"this"###,
];
pub(super) const A_PLACE_PREP: &[&str] = &[
    r###"at"###,
    r###"to"###,
    r###"from"###,
    r###"in"###,
    r###"into"###,
    r###"near"###,
    r###"off"###,
    r###"for"###,
    r###"of"###,
    r###"on"###,
];
pub(super) const A_LIGHT_KEY: &[&str] = &[
    r###"leave"###,
    r###"pay"###,
    r###"order"###,
    r###"booking"###,
    r###"paint"###,
    r###"drill"###,
    r###"work"###,
    r###"delivery"###,
    r###"bid"###,
    r###"invoice"###,
    r###"credit"###,
    r###"deal"###,
    r###"cover"###,
    r###"fill"###,
    r###"charge"###,
    r###"term"###,
];
pub(super) const PLACES: &[&str] = &[
    r###"japan"###,
    r###"spain"###,
    r###"france"###,
    r###"italy"###,
    r###"greece"###,
    r###"portugal"###,
    r###"germany"###,
    r###"holland"###,
    r###"turkey"###,
    r###"egypt"###,
    r###"mexico"###,
    r###"canada"###,
    r###"america"###,
    r###"australia"###,
    r###"ireland"###,
    r###"scotland"###,
    r###"wales"###,
    r###"england"###,
    r###"london"###,
    r###"manchester"###,
    r###"birmingham"###,
    r###"edinburgh"###,
    r###"cardiff"###,
    r###"belfast"###,
    r###"dublin"###,
    r###"paris"###,
    r###"rome"###,
    r###"madrid"###,
    r###"lisbon"###,
    r###"new"###,
    r###"york"###,
    r###"florida"###,
    r###"california"###,
];
pub(super) const A_PASSPORT_WORD: &[&str] = &[
    r###"passport"###,
    r###"passports"###,
    r###"visa"###,
    r###"visas"###,
];
pub(super) const A_NOT_A_PLACE: &[&str] = &[
    r###"january"###,
    r###"february"###,
    r###"march"###,
    r###"april"###,
    r###"may"###,
    r###"june"###,
    r###"july"###,
    r###"august"###,
    r###"september"###,
    r###"october"###,
    r###"november"###,
    r###"december"###,
    r###"jan"###,
    r###"feb"###,
    r###"mar"###,
    r###"apr"###,
    r###"jun"###,
    r###"jul"###,
    r###"aug"###,
    r###"sep"###,
    r###"sept"###,
    r###"oct"###,
    r###"nov"###,
    r###"dec"###,
    r###"monday"###,
    r###"tuesday"###,
    r###"wednesday"###,
    r###"thursday"###,
    r###"friday"###,
    r###"saturday"###,
    r###"sunday"###,
    r###"week"###,
    r###"weeks"###,
    r###"weekend"###,
    r###"month"###,
    r###"months"###,
    r###"year"###,
    r###"years"###,
    r###"day"###,
    r###"days"###,
    r###"morning"###,
    r###"afternoon"###,
    r###"evening"###,
    r###"night"###,
    r###"spring"###,
    r###"summer"###,
    r###"autumn"###,
    r###"winter"###,
    r###"a"###,
    r###"an"###,
    r###"the"###,
    r###"my"###,
    r###"our"###,
    r###"your"###,
    r###"his"###,
    r###"her"###,
    r###"their"###,
    r###"this"###,
    r###"that"###,
    r###"these"###,
    r###"those"###,
    r###"it"###,
    r###"them"###,
    r###"all"###,
    r###"some"###,
    r###"any"###,
    r###"no"###,
    r###"one"###,
    r###"two"###,
    r###"three"###,
    r###"ten"###,
    r###"twenty"###,
    r###"future"###,
    r###"past"###,
    r###"total"###,
    r###"general"###,
    r###"fact"###,
    r###"particular"###,
    r###"touch"###,
    r###"writing"###,
    r###"person"###,
    r###"which"###,
    r###"what"###,
    r###"order"###,
    r###"case"###,
    r###"time"###,
    r###"front"###,
    r###"bed"###,
    r###"hospital"###,
    r###"work"###,
    r###"school"###,
];
pub(super) const STOP: &str = r#"
a an the and or but if so then than that this those these i me my mine you your yours we us our he
she it they them him her his hers their there here what which who whom whose when where why how do
does did done doing is are was were be been being am have has had having will would can could
should shall may might must not no nor yes ok okay just also too very really quite about for with
without on in into at by from to of as up down out off over under again more most much many any
some all both each either neither other another same such own only ever never always still yet
already even though although because while since until before after during against between among
right well thing things stuff get got give take make made say said tell told want wanted need
needed like liked know knew think thought feel felt go going went come came back put let try tried
use used find found work works working day days time times week weeks month months year years one
two three four five six seven eight nine ten bit lot lots sort kind way ways now today tomorrow
yesterday please thanks thank hi hello hey yeah yep nope um uh oh ah eh due anyway gonna wanna dont
doesnt didnt isnt wasnt cant couldnt wouldnt shouldnt havent hasnt im ive id ill youre youve theyre
thats whats heres theres whose wont
"#;
