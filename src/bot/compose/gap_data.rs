//! Regexes and answer pools from the pinned gap composer.
use fancy_regex::Regex;
use std::sync::LazyLock;
pub(super) static A_MOVING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:exchange|interest|mortgage|savings|base|bank) ?rates?\b|\brate of exchange\b|\bfuel prices?\b|\b(?:petrol|diesel) prices?\b|\bprice of (?:petrol|diesel|fuel)\b|\bhow much is (?:petrol|diesel|fuel)\b"##).expect("A_MOVING")
});
pub(super) static A_PRICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow much\b|\bwhat(?:'s| is| does| would| will)?\b[^.?!]*\bcosts?\b|\bcost of\b|\bprice of\b|\bprices?\b|\bcheaper\b|\bcheapest\b|\bset me back\b|\bbudget for\b|\bgoing rate\b|\bthe rate for\b|\bwhat do you pay for\b"##).expect("A_PRICE")
});
pub(super) static NAME_ONLY_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r##"(?i)^\s*(?:the\s+)?(?:price|cost)s?\s+(?:of|for)\b|\bgoing rate\b|\bthe rate for\b"##,
    )
    .expect("NAME_ONLY_ASK")
});
pub(super) static NOT_A_PRICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow much\b[^.?!]{0,32}\b(?:weigh|weighs|weight|sleep|eat|drink|take|give|feed|exercise|water|milk|protein|salt|sugar|fibre|calories|units|grams|ml|litres|paracetamol|ibuprofen|is safe|are safe|is too much|is normal|is enough)\b|\bhow much\b[^.?!]{0,20}\b(?:can|should|could) i\b"##).expect("NOT_A_PRICE")
});
pub(super) static A_LONGER_SPAN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:in|over|for|across)\s+a\s+year\b|\ba\s+year\b|\bper\s+year\b|\bannual(?:ly)?\b|\bin\s+(?:twelve|12)\s+months\b"##).expect("A_LONGER_SPAN")
});
pub(super) static ASKS_A_COST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow much\b|\bcosts?\b|\bcosting\b|\bspend\b|\bspent\b|\bafford\b|\bexpensive\b|\bset me back\b|\bcome to\b|\bcomes to\b"##).expect("ASKS_A_COST")
});
pub(super) static AN_AMOUNT_ALREADY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)[£$]|\b\d+\s*(?:p|pence|pounds?|quid|bucks|euros?)\b|\b(?:fiver|tenner|twenty)\b|\b(?:one|two|three|four|five|six|seven|eight|nine|ten)\s+(?:pounds?|quid)\b"##).expect("AN_AMOUNT_ALREADY")
});
pub(super) static A_COUNT_IN_A_SPAN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(\d+|one|two|three|four|five|six|seven|twice|three times|four times|five times)\s*(?:days?|times|nights?|lunches|cups?|pints?|cigarettes?|takeaways?|journeys?|trips?|rides?|lessons?|bottles?|coffees?)?\s*(?:a|per|each|every)\s+(day|week|month)\b"##).expect("A_COUNT_IN_A_SPAN")
});
pub(super) static A_EVERY_SPAN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:every|each)\s+(day|week|month)\b"##).expect("A_EVERY_SPAN")
});
pub(super) static A_RATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:parking|car ?park|to park|park there|entry|admission|tickets?|fares?|day pass|week pass|season ticket|membership|toll|ferry|crossing|per night|a night|per hour|per day|day rate|hourly rate)\b"##).expect("A_RATE")
});
pub(super) static A_WORTH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bworth it\b|\bworth the money\b|\bworth buying\b|\bworth the extra\b|\bworth paying\b|\bworth the price\b|\bgood buy\b|\bis it worth\b"##).expect("A_WORTH")
});
pub(super) static A_REPAIR_OR_BUY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:fix|fixes|fixing|fixed|mend|mends|mending|mended|repair|repairs|repairing|repaired|service|services|servicing|serviced|replace|replaces|replacing|replacement|replaced|new one|second hand|secondhand|refurb(?:ish|ished)?|upgrade|upgrading|claim|claims|claiming|claimed|install|installs|installing|buy|buys|buying|bought|worth the money|worth buying|good buy|worth paying|worth the price|paying extra|pay extra)\b"##).expect("A_REPAIR_OR_BUY")
});
pub(super) static A_BROKEN_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:broken|breaking|broke|not working|stopped working|gone wrong|last legs|leaking|faulty|dead|dead as|knackered|worn out|on the blink|cracked|rusting|needs? a repair|needs? fixing|needs? replacing|needs? a service|mend it|fix it|replace it|repair it)\b"##).expect("A_BROKEN_THING")
});
pub(super) static A_CHOICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bwhich is better\b|\bwhich one is better\b|\bbetter to (?:get|buy|go for)\b|\bor should i\b|\bshould i (?:get|buy|go for)\b[^.?!]{0,40}\bor\b|\bvs\b|\b(?:pick|choose|decide|select) between\b"##).expect("A_CHOICE")
});
pub(super) static A_DISTANCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow far\b|\bmiles? from\b|\bmiles? to\b|\bhow long is the (?:drive|journey|walk)\b|\bdistance (?:from|to|between)\b"##).expect("A_DISTANCE")
});
pub(super) static A_DURATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow long (?:does|do|will|would|should|until|before|after)\b|\bhow long until\b|\btakes? how long\b|\bhow soon\b|\bhow many (?:hours|days|weeks|months|years)\b[^.?!]{0,20}\b(?:does|do|will|until|before|after|should i wait)\b"##).expect("A_DURATION")
});
pub(super) static BARE_DURATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)^(?:and |so |but |ok |okay )?(?:how long|how soon)\b[\s\w]{0,24}?\b(?:it|that|this|they|them|one)\b[\s\w]{0,10}?\??$"##).expect("BARE_DURATION")
});
pub(super) static A_QUEUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:claim|claims|application|apply|form|forms|paperwork|order|delivery|courier|post|posting|post office|queue|backlog|repair|repairs|service|servicing|mot|survey|surveyor|solicitor|conveyanc|completion|exchange of contracts|offer|mortgage|valuation|passport|licence|license|visa|council|benefit|benefits|universal credit|pip|esa|refund|return|returns|dispute|appeal|hearing|decision|assessment|builder|builders|plasterer|electrician|plumber|joiner|installer|install|installation|dispatch|boiler|house move|sale|sell|selling|buying|purchase|probate|will|tenancy|landlord|deposit|dbs|background check|bank|payment|transfer|clearing|money|clears|clearing|wait|waits|waiting)\b"##).expect("A_QUEUE")
});
pub(super) static A_LOAD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b\d+\s*-?\s*amp\b|\bextension lead\b|\bplug socket\b|\bfuse\b|\btripp(?:ed|ing)\b|\btrips?\b\s+(?:the\s+|a\s+|our\s+|my\s+|your\s+)?(?:fuse|circuit|breaker|mcb|rcd|socket|supply|switch|electric\w*|mains|ring)\b|\b(?:off|from|on|into)\s+(?:one|the same|a single)\s+(?:socket|plug|circuit|extension)\b"##).expect("A_LOAD")
});
pub(super) static A_FIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:will|would|does|do|can)\b[^?!]{0,30}\b(?:fit|fits)\b|\b(?:work|works|go) with\b[^.?!]{0,20}\b(?:battery|charger|drill|hose|part|filter|attachment|unit|set|kit)\b|\bwhich\s+(?:size|model|part|one)\b[^?!]{0,20}\bdo i need\b|\bwhat size\b|\bis (?:it|this|that) compatible\b"##).expect("A_FIT")
});
pub(super) static A_RULE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bam i allowed\b|\bare we allowed\b|\bis it legal\b|\bis that legal\b|\bdo i need (?:a |an |permission|to )\b|\bcan i be made to\b|\bcan they make me\b|\bhave i got the right\b|\bwhat are my rights\b|\bdo i have to\b|\bam i entitled\b|\bis it against the law\b|\bcan my (?:landlord|employer|boss)\b"##).expect("A_RULE")
});
pub(super) static NOT_A_RULE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:uno|monopoly|chess|poker|scrabble|darts|snooker|board game|bishop|knight|rook|pawn|checkmate|the pieces)\b"##).expect("NOT_A_RULE")
});
pub(super) static A_DO_I_NEED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bdo i need (?:a |an |to )\b|\bdo i have to\b|\bdo we need to\b"##)
        .expect("A_DO_I_NEED")
});
pub(super) static A_RULE_SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:licen[cs]e|licen[cs]ed|permit|permission|visa|insurance|insurer|mot\b|tax|taxed|deposit|notice|contract|landlord|tenant|letting agent|employer|boss|\bhr\b|council|council tax|hmrc|gov\.uk|government|court|probate|benefits?|universal credit|housing benefit|claim|claiming|form|forms|declaration|statement|registration|register|passport|inspection|consent|referral|prescription|legal|law|rights|entitled|mortgage|solicitor|conveyanc\w*|certificate|crime number|vet\b|policy|warranty|guarantee|refund|return it|duty|obliged|obligation)\b"##).expect("A_RULE_SUBJECT")
});
pub(super) static A_STAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow many (?:people|residents|households|of them|are there)\b|\bwhat percentage\b|\bwhat proportion\b|\bwhat(?:'s|s| is)?\s*(?:is\s+)?the population\b|\bpopulation of\b"##).expect("A_STAT")
});
pub(super) static A_HOWTO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow (?:do|can|should|could|would) (?:i|you|we)\b|\bhow to\b|\bbest way to\b|\bany (?:tips|advice) (?:for|on)\b|\bwhat should i do about\b|\bwhere do i start\b|\bhow would you go about\b"##).expect("A_HOWTO")
});
pub(super) static A_HOWTO_HAS_A_SHAPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:stuck|stubborn|tight|seized|jammed|wedged|won'?t|wont|not working|stopped working|broke|broken|leak|drip|damp|mould|mold|smell|stink|stain|crease|creak|squeak|squeal|rattl|wobbl|vibrat|noisy|block|blocked|slow|stiff|rust|scratch|burn|fade|crack|chip|snap|worn|tangled|knotted|fog|cloudy|flat|dead|flicker|trip|overflow|keeps (?:going|coming|stopping|turning))|\b(?:will|probate|benefits?|universal credit|council tax|licence|license|permit|passport|visa|tax return|self assessment|pension|redundancy|redundant|apply|application|appeal|complain|complaint|refund|return|cancel|cancelling|mortgage|insurance|insure|claim|registration|register|deregister|loyalty|warranty|guarantee|bill|bills|direct debit|standing order|switch|switching|comparison|quote|quotes|contract|agreement|tenancy|deposit|moving house|house move|move house|solicitor|notary|deed|title|land registry)\b"##).expect("A_HOWTO_HAS_A_SHAPE")
});
pub(super) static NOT_A_HOWTO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:feel|feeling|feelings|anxious|anxiety|depressed|depression|panic|grief|grieving|lonely|loneliness|scared|worried|worry|miscarriage|divorce|separating)\b|\b(?:tell|talk to|speak to|say to|ask|apologise to|apologize to|comfort|support|help|explain to|break it to|be there for|deal with|do about)\b[^?!]{0,24}\b(?:my|his|her|their)\s+(?:mum|mom|dad|son|daughter|wife|husband|partner|boyfriend|girlfriend|friend|brother|sister|boss|child|kid|parents|family)\b"##).expect("NOT_A_HOWTO")
});
pub(super) static A_RECORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:deposit|deeds?|title plan|land registry|warrant(?:y|ies)|guarantee|receipt|invoice|policy|insurance|cover note|booking|reservation|tenancy agreement|assured shorthold|certificate|statement|reference number|reference|paperwork|the small print|terms|boundary|boundaries)\b"##).expect("A_RECORD")
});
pub(super) static NOT_A_RECORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:whats|what is|what are|what does|what do|meaning of)\s+(?:a|an)\b|\bwhat\s+(?:does|do|is|are|did|can)\s+(?:the|this|that)\b[^.?!]{0,40}\b(?:mean|means|meaning|stand for|definition)\b|\b(?:how do i|how to|should i (?:take out|buy|get)|worth (?:getting|it)|cheapest|cancel my|switch my|compare)\b|\bwhats? the point of\b|\bwhat is the point of\b|\bsick of\b|\bfed up\b|\bdrowning in\b|\bcan'?t be (?:bothered|arsed)\b"##).expect("NOT_A_RECORD")
});
pub(super) static A_LOOSE_KIND_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:policy|policies|statement|statements|terms|paperwork|insurance|boundar(?:y|ies)|contract|agreement)\b"##).expect("A_LOOSE_KIND_WORD")
});
pub(super) static THEIR_OWN_PAPER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:my|our|his|her|their|your|yours|ours|theirs)\b"##)
        .expect("THEIR_OWN_PAPER")
});
pub(super) static A_NAMED_DOCUMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:terms and conditions|terms of (?:the|my|our|their|his|her|your)\b|insurance polic(?:y|ies)|policy wording|policy document|policy schedule|bank statement|statement of (?:the|my|our|their)\b|tenancy agreement|assured shorthold|contract of employment|the small print)\b"##).expect("A_NAMED_DOCUMENT")
});
pub(super) static NOT_HERE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:999|911|112|111|ambulance|a&e|emergency|bleeding|not breathing|chest pain|overdose|suicide|kill myself|end my life|self harm|died|death|funeral|buried|cremation|passed away|grief|grieving)\b"##).expect("NOT_HERE")
});
pub(super) static ASKS_SOMETHING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:how|whats?|what's|which|when|where|who|why|can|could|should|would|will|is|are|was|were|do|does|did|am|has|have|had|may|might|shall)\b"##).expect("ASKS_SOMETHING")
});
pub(super) static A_POINTER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\ba quick search\b|\bsearch online\b|\bsearching for it\b|\bquick look\b|\ba search\b|\blook (?:it|that) up\b|\bwikipedia\b|\bprice comparison\b|\bpaste what it says\b|\b(?:official|seller'?s|their|its|the)\b[^.?!]{0,20}?\b(?:site|page|listing|app)\b[^.?!]{0,24}?\b(?:will|has|have)\b"##).expect("A_POINTER")
});
pub(super) static THE_PAPER_ITSELF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bthe (?:receipt|deed|deeds|wording|document|documents|paperwork|certificate|policy|contract|agreement|statement|invoice|booking|label|manual|terms)\b|\b(?:date|name|reference|number) on it\b"##).expect("THE_PAPER_ITSELF")
});
pub(super) static A_NEED_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bdo (?:i|we) need (?:to |a |an )\b"##).expect("A_NEED_TO")
});
pub(super) static A_WORRY_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bdo (?:i|we) (?:need|have) to\b[^.?!]{0,24}\b(?:worry|panick?\w*|stress\w*|be anxious|be scared|be (?:worried|afraid)|fret)\b"##).expect("A_WORRY_ASK")
});
pub(super) const MOVING_LINES: &[&str] = &[
    "That figure is republished every day and two places rarely quote the same one, because each adds its own margin or terms on top: compare two of them and go with the one whose small print fits what you are actually doing, checking the date printed with the number.",
    "Nobody keeps that number in their head - it moves daily and the version printed next to it is the only one that counts. Read two offers rather than one, and add on the fee or the terms before you decide which is really cheaper.",
];
pub(super) const PRICE_LINES: &[&str] = &[
    "There is no list price for that: what it comes to depends on the size of the job, the state of the thing and when you want it, so ask two firms to price the same description in writing and compare what each one leaves out rather than the total.",
    "The figure comes from a quote or a listing, not from me: get the same description from two places, in writing, and check what each one includes - the parts, the labour, the call-out and the VAT.",
];
pub(super) const RATE_LINES: &[&str] = &[
    "That is a published rate rather than a quote: whoever runs the place sets it, it moves with the day and the season, and a cap or a longer stay usually costs less per hour. Their own page or listing has today's, so ask what the whole stretch comes to rather than the headline figure.",
    "The headline rate is only half of it: an evening or weekend price, a day cap, a return or a longer booking often costs less than paying by the hour, and the operator's own page lists them all. Check today's figure there before you pay.",
];
pub(super) const WORTH_LINES: &[&str] = &[
    "What decides it is the condition, what it would cost to put right and what the new one costs: if the repair comes to more than about half of new, it usually is not worth it. Get the repair cost in writing first, then compare that with the price of a new one.",
    "It earns its price when what it needs doing comes to less than replacing it, and a second one of the same age has its own faults. Ask for the repair cost in writing, and price the new one before you decide rather than after.",
];
pub(super) const CHOICE_LINES: &[&str] = &[
    "Both work for somebody, so it comes down to what you would miss if you had to give the other one back, and which of them fits the week you actually have. Write down what each one is for before you compare what they cost, and the one you have to keep justifying is usually the answer.",
    "The way to settle it is which one you would still be glad of in a year, and whether either of them brings a running cost with it - so list what you would use each one for, then compare the whole cost rather than the price on the ticket.",
];
pub(super) const DISTANCE_LINES: &[&str] = &[
    "The straight-line miles and the road miles are two different numbers, and the time bends with the hour you travel, so put both ends into a map app and read the route it gives: it carries the miles, the time and the way round the slow bit.",
    "For a journey by road that is a map app, and for one by train it is the operator's own planner: both give the miles or the connections for the hour you would actually go, which is the part a rough figure from anyone else gets wrong.",
];
pub(super) const DURATION_LINES: &[&str] = &[
    "How long that takes depends on the queue, how much there is to do and the time of year, so ask the body that runs it for their current wait and whether the date they give includes the slow part at the end - a range from them beats any single figure.",
    "The honest answer is a range: the size of it and how busy they are move it by weeks, so ask whoever handles it what they are running at now, and treat the earliest date they give as the best case rather than the promise.",
];
pub(super) const LOAD_LINES: &[&str] = &[
    "The labels settle it: the watts are printed on the base of each thing, and a 13-amp socket gives about 3,000 watts in total, so the sum is what counts rather than the number of plugs. Add up the labels and keep the total under that - a kettle or a heater on its own takes most of it.",
    "Add up the watts on the labels of everything that would be on at once and keep the total under about 3,000 for a 13-amp socket, because a heater or a kettle alone takes most of the headroom. The rating is usually moulded into the plug or the base, not on the box.",
];
pub(super) const FIT_LINES: &[&str] = &[
    "The size and the model on the label decide it rather than the picture: the model number is usually on a plate underneath, in the manual or on the maker's page for it. Read that before anything is ordered, because the same name comes in more than one size.",
    "It is the measurements and the model number that settle it, and both are on the thing itself, so note them down and match them against the spec on the maker's own page for the part - that is where the fit is stated rather than guessed at.",
];
pub(super) const RULE_LINES: &[&str] = &[
    "Whether that is allowed turns on the rules as they stand this year and on your own case, so the current wording decides it rather than what anyone has heard. GOV.UK has the national version and your council's site the local one, and the office that runs it will put the conditions for your situation in writing - keep that letter.",
    "It depends on what your own agreement says and on the version of the rules in force, and the gap between those two is usually the whole answer. The issuing office's own page has the current wording, and one call to them settles the case the page does not cover.",
];
pub(super) const STAT_LINES: &[&str] = &[
    "Figures like that come from whoever counts them, and the year and the boundary change the answer: \"the town\" and \"the wider area\" are two different numbers. The official statistics page or the council's own figures have the current one, with the year it is for printed beside it.",
    "That number moves every year and it depends where the line is drawn, so the body that publishes it is the one to quote: the national statistics site has it with the year, and the local council publishes its own for the area it covers.",
];
pub(super) const HOWTO_LINES: &[&str] = &[
    "The first steps are the same for most of these: work out what is actually causing it before you treat what you can see, change one thing at a time, and give each change a day before the next so you know which one worked. Write down what you tried and when.",
    "Do it in the order that costs least first: what you already have in the cupboard or the shed, then the cheap part or product, then the trade. Before you buy anything, check the manual or the maker's page for the exact model, because it says what is safe on that one and what is not.",
];
pub(super) const RECORD_LINES: &[&str] = &[
    "What settles that is the entry itself rather than the general rule - the date on it, the terms that were in force then, and the name it is in. The office or firm that holds it can read it back to you, so ask for it in writing and quote the reference on it: the answer that counts is the one on paper, not the one given over the phone.",
    "That turns on the wording of the document, not on what is usually done: what it says, what date it came into force and whose name is on it. Whoever issued it keeps the record, and the one thing worth doing first is asking them to confirm in writing what their copy says.",
    "Nobody can answer that from the description of it, because the answer is in the paperwork - the date, the terms and the name on it. Ask the body holding it for the entry in writing with the reference on it, and keep what they send: a written answer is the one that can be relied on later.",
];
pub(super) const NEED_TO_LINES: &[&str] = &[
    "Depends what it is for. Say the thing you are doing and I will tell you what it takes - and where a rule rather than common sense decides it, who to ask.",
    "It turns on the thing itself and on what it is for - a trip, a form, a job and a pan of something all want different things. Tell me which it is and I will give you the short version, rather than the general one that fits none of them.",
    "That depends on the thing itself and what you are doing with it. Say which and I will tell you, rather than guess at it.",
];
pub(super) const WORRY_LINES: &[&str] = &[
    "Worry about what? Tell me what is coming up and I will say whether there is anything to do about it, because most of what people brace for turns out to be somebody else's rule.",
    "Say what it is and I will tell you straight whether there is anything to do about it. If there is, it is usually one form or one call, and if there is not, you can stop carrying it.",
];
pub(super) fn kinds() -> [(&'static str, &'static Regex, &'static [&'static str]); 15] {
    [
        ("worried", &A_WORRY_ASK, WORRY_LINES),
        ("choice", &A_CHOICE, CHOICE_LINES),
        ("howto", &A_HOWTO, HOWTO_LINES),
        ("worth", &A_WORTH, WORTH_LINES),
        ("distance", &A_DISTANCE, DISTANCE_LINES),
        ("load", &A_LOAD, LOAD_LINES),
        ("fit", &A_FIT, FIT_LINES),
        ("rule", &A_RULE, RULE_LINES),
        ("figure", &A_STAT, STAT_LINES),
        ("moving", &A_MOVING, MOVING_LINES),
        ("rate", &A_RATE, RATE_LINES),
        ("price", &A_PRICE, PRICE_LINES),
        ("duration", &A_DURATION, DURATION_LINES),
        ("record", &A_RECORD, RECORD_LINES),
        ("needto", &A_NEED_TO, NEED_TO_LINES),
    ]
}
