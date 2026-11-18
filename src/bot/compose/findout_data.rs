//! Source tables from the pinned findout and fallback composers.
use fancy_regex::Regex;
use std::sync::LazyLock;
pub(super) static ASKED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:is|are|will|any|does|do)\b|^\s*(?:any\s+)?(?:delays?|disruptions?|traffic|roadworks?|road\s+works|cancellations?|strikes?|engineering\s+works)\b"##).expect("ASKED")
});
pub(super) static ASKS_LOCAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bis\s+there\b|\bare\s+there\b|\bwhere\b|\bwheres\b|\bwhich\b|\bany\b|\banywhere\b|\bcan\s+(?:i|we|you)\b|\bcould\s+(?:i|we|you)\b|\bdo\s+you\s+know\b|\b(?:i|we)\s+(?:need|want|are\s+looking|am\s+looking)\b|\blooking\s+for\b|\brecommend\b|\bfind\b|\bwhats\s+good\b|\bgood\s+(?:place|places|spot|spots)\b"##).expect("ASKS_LOCAL")
});
#[rustfmt::skip]
pub(super) const ASK_STOP:&[&str]=&["a", "about", "allowed", "an", "and", "are", "be", "can", "could", "do", "does", "for", "i", "if", "illegal", "in", "is", "it", "law", "legal", "my", "of", "on", "our", "regulation", "regulations", "rule", "rules", "that", "the", "there", "this", "to", "we", "what", "whats", "you"];
pub(super) static AVAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:tickets?|seats?|tables?|rooms?|spaces?|places?|slots?|pitches?|appointments?)\s+left\b|\b(?:are|is)\s+there\s+(?:any\s+|a\s+)?(?:tickets?|seats?|tables?|rooms?|spaces?|pitches?)\b|\b(?:is\s+it|are\s+they)\s+(?:fully\s+)?(?:booked\s+up|full|sold\s+out)\b|\b(?:is|are|has|have|did|does|will)\b[^?]{0,20}?\bsold\s+out\b|\bcan\s+(?:i|we)\s+(?:still\s+)?(?:get|book)\s+(?:a|any)\s+(?:table|seat|ticket|room|space|pitch|slot)\b"##).expect("AVAIL")
});
#[rustfmt::skip]
pub(super) const AVAIL_LINES:&[&str]=&["What is left is on their booking system rather than anywhere I can see, and it changes by the hour. Their booking page shows what is still free, and the box office settles a sell-out in a second.", "Anything still free is in their own system, sorry - it moves too fast for a general answer. Their booking page shows it, and ringing is quicker if you are on your way."];
pub(super) static A_AND_E: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:a\s*&\s*e|a\s+and\s+e|a\s*and\s*e|casualty|emergency\s+department|emergency\s+room|urgent\s+care|walk[-\s]?in\s+cent\w+|minor\s+injuries)\b"##).expect("A_AND_E")
});
pub(super) static A_BACK_REFERENCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:it|that|they|them|those|this|the\s+same|still)\b"##)
        .expect("A_BACK_REFERENCE")
});
pub(super) static A_FOLLOW_UP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r##"^\s*(?:is|are|will|would|how\s+(?:long|much|bad|far)|what\s+about|and\b|does|do)\b"##,
    )
    .expect("A_FOLLOW_UP")
});
pub(super) static A_PERSON_SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:is|are|was|were|will|would|does|do|did)\s+(?:my\s+|our\s+|your\s+|the\s+)?(?:he|she|they|him|her|them|you|we|i|mum|mom|mother|mam|dad|father|da|boss|manager|sister|brother|son|daughter|wife|husband|partner|boyfriend|girlfriend|friend|mate|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|kid|kids|child|children|baby|lad|lass|boy|girl|colleague|neighbour|neighbor|landlord|tenant|staff|team|people|everyone|anyone|someone|nobody|somebody|bot|assistant|robot)\b"##).expect("A_PERSON_SUBJECT")
});
#[rustfmt::skip]
pub(super) const A_PLACE:&[&str]=&["airport", "aldi", "asda", "bank", "banks", "bar", "barber", "beach", "beaches", "bowling", "branch", "building", "cafe", "caff", "center", "centre", "chapel", "chemist", "chemists", "chip", "chippies", "chippy", "church", "cinema", "club", "co-op", "coop", "dental", "dentist", "doctors", "dump", "farm", "gallery", "garage", "gp", "ground", "gym", "hairdresser", "hairdressers", "hospital", "iceland", "karting", "kebab", "leisure", "libraries", "library", "lidl", "lido", "mall", "market", "merchants", "morrisons", "museum", "office", "park", "parks", "pharmacy", "pizza", "playground", "pool", "promenade", "pub", "pubs", "reception", "recycling", "restaurant", "rink", "sainsbury", "sainsburys", "salon", "sauna", "seafront", "services", "shop", "shops", "showroom", "society", "softplay", "spa", "stadium", "stall", "station", "store", "stores", "supermarket", "surgery", "takeaway", "tesco", "theatre", "tip", "vet", "vets", "waitrose", "warehouse", "zoo"];
pub(super) static A_PLACE_IN_A_TURN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:beach|seafront|promenade|gym|pool|leisure\s+cent\w+|tip|dump|hospital|casualty|clinic|surgery|pharmacy|chemist|supermarket|shop|store|market|pub|restaurant|cafe|takeaway|park|playground|museum|gallery|zoo|library|station|airport|car\s+park|high\s+street|town\s+cent\w+|mall|ground|stadium|theatre|cinema|venue|club|centre|sauna|spa|dentist|bank|services|attraction|soft\s*play|ice\s+rink|bowling)\b"##).expect("A_PLACE_IN_A_TURN")
});
pub(super) static A_PLACE_NAMED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:map app|maps|google maps|the council|council'?s?|gov\.uk|\.gov\b|government|operator'?s?|journey planner|traffic service|road agency|roads agency|national rail|pharmac(?:y|ist)|chemist|\bgp\b|doctor'?s|surgery|the ward|nurse|vet'?s?|veterinary|their (?:own )?(?:site|website|web page|page|app|account|office|desk|number)|the (?:official )?(?:site|website) of|official (?:page|site)|box office|ticket office|the office that|issuing office|passport office|dvla|registry office|landlord|letting agent|estate agent|employer|\bhr\b|payroll|your bank|the bank|insurer|insurance company|the airline|the hotel|the venue|the airline'?s?|one call|give them a ring|ring them|call them|their line)\b"##).expect("A_PLACE_NAMED")
});
pub(super) static A_REPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:every\s+(?:single\s+)?(?:day|week|morning|evening|night|time)|all\s+(?:week|weeks|month|months|year|day|days|morning|night|the\s+time)|for\s+(?:weeks|months|days|ages|hours)|since\s+(?:january|february|march|april|may|june|july|august|september|october|november|december|last|monday|tuesday|wednesday|thursday|friday|saturday|sunday|easter|christmas|then)|each\s+(?:day|week|morning)|the\s+whole\s+(?:week|month|day)|ruined|ruins|wrecked|wrecks|chaos|nightmare|fed\s+up|sick\s+of|so\s+tired|held\s+me\s+up|made\s+me\s+late|cost\s+me|missed\s+my)\b"##).expect("A_REPORT")
});
pub(super) static A_STATUS_SPENT_ON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^\s*(?:any\s+|the\s+)?(?:delays?|disruptions?|traffic|roadworks?|road\s+works|cancellations?|strikes?|engineering\s+works)\s+(?:on|at|near|around|by|into|outside|through)\b"##).expect("A_STATUS_SPENT_ON")
});
pub(super) static A_WAIT_NOUN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:wait|waiting|queue|queues|queuing|queueing|waiting\s+times?)\b"##)
        .expect("A_WAIT_NOUN")
});
pub(super) static BUSY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+busy\b|\bis\s+(?:it|the\s+[a-z]+)\s+busy\b|\bhow\s+long\s+(?:is|are)\s+(?:the\s+)?(?:wait|queue|line)\b|\bwhats\s+the\s+(?:wait|queue|line)\b|\bwaiting\s+times?\b|\bis\s+there\s+(?:a\s+|any\s+)?(?:queue|wait|line)\b|\bwill\s+there\s+be\s+(?:a\s+)?(?:queue|wait|line)\b|\bhow\s+(?:packed|crowded|heaving|rammed)\b|\bwhats\s+the\s+(?:waiting\s+time|wait\s+time)\b|\bis\s+the\s+(?:queue|wait|line)\s+(?:long|bad|big|short|quick|slow)\b|\bwhen\s+(?:is|are)\s+(?:\w+\s+){0,4}(?:quiet|emptiest|quietest|least\s+busy)\b|\b(?:is|are|was|were|will|would)\s+(?:\w+\s+){1,4}(?:busy|packed|heaving|rammed|crowded|manic)\b"##).expect("BUSY")
});
pub(super) static BUSY_ASKS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^\s*(?:is|are|was|were|will|would|do|does|did|can|could|should|any|how|what|whats|when|where|which|who)\b|\?\s*$"##).expect("BUSY_ASKS")
});
#[rustfmt::skip]
pub(super) const BUSY_LINES:&[&str]=&["A queue changes hour to hour: the weekend and school pick-up are the busy half of the week and a weekday morning the quiet one, and a wet Saturday pushes everybody indoors at once. Nobody publishes it everywhere, so one call settles today faster than anything I could guess, and their own page has a live wait if they keep one.", "It will be worse at the weekend and around school pick-up, and better first thing, but today's wait is only on their system. Ring them before you set off - the place itself is the only one that knows.", "The busy hours are much the same everywhere - late morning to mid afternoon at the weekend, and the hour after school lets out - and the weather moves the whole thing. Today's version is at the place: their own page keeps a live count where it keeps one, and one call settles the rest."];
#[rustfmt::skip]
pub(super) const BUSY_NO_PLACE:&[&str]=&["A wait is live and local, so the only figure that is right is the one at that place today: their own page keeps it where they keep one, and one call settles the rest. Waits are usually shortest first thing and longest from mid morning on. Which place is it?", "That changes hour to hour and place to place, so I can't put a number on it - their own page or phone has today's, and a map app's popular times shows the busy hours if you are picking when to go. First thing is usually the quiet end. Which place did you have in mind?"];
#[rustfmt::skip]
pub(super) const BUSY_NO_PLACE_AND_CONTEXT:&[&str]=&["A wait is live and local, so today's figure is only at the place itself: their own page keeps it where they keep one, one call settles the rest, and first thing is usually the quiet end of the day.", "That one moves hour to hour and place to place, so I can't put a number on it: their own page or phone has today's, and a map app's popular times shows the busy hours if you are choosing when to go."];
pub(super) static DELIVER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bdeliver(?:y|s|ed)?\s+(?:to|in)\b|\bdeliver\s+to\s+my\b|\bdo\s+(?:they|you)\s+deliver\b"##).expect("DELIVER")
});
#[rustfmt::skip]
pub(super) const DELIVER_LINES:&[&str]=&["Delivery depends on the postcode and the day, and only the shop's own checker knows it: put your postcode in on their site and it says yes or no in a second, with the earliest slot.", "That depends on the postcode, which their own delivery checker holds - it takes ten seconds with your postcode in, and it shows the earliest date too."];
#[rustfmt::skip]
pub(super) const DIED:&[&str]=&["cremated", "dead", "death", "died", "down", "euthanised", "passing", "put"];
pub(super) static FREE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bis\s+(?:the\s+|a\s+)?[a-z' -]{2,24}\s+free\b|\bis\s+(?:entry|admission|parking)\s+free\b|\bis\s+there\s+a\s+charge\b|\bpay\s+to\s+(?:park|get\s+in|go\s+in|enter|visit)\b|\bdo\s+(?:i|we)\s+have\s+to\s+pay\s+(?:to|for\s+(?:a|an|the|any|my|our))\b"##).expect("FREE_ASK")
});
#[rustfmt::skip]
pub(super) const FREE_COMPOUND:&[&str]=&["caffeine", "care", "contact", "cruelty", "dairy", "duty", "egg", "fat", "gluten", "hands", "lactose", "msg", "nut", "nuts", "peanut", "smoke", "soy", "soya", "sugar", "tax", "toll", "wheat"];
#[rustfmt::skip]
pub(super) const GAME:&[&str]=&["bingo", "chess", "cluedo", "darts", "dominoes", "monopoly", "poker", "risk", "rummy", "scrabble", "snooker", "uno"];
pub(super) static HOLDS_IT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:has|have|shows|lists|carries|publishes|updates)\b[^.]{0,30}\b(?:number|form|fee|price|prices|hours|wait|times|schedule|status|page|listing)\b"##).expect("HOLDS_IT")
});
#[rustfmt::skip]
pub(super) const HOURS:&[&str]=&["Opening hours are set by each branch rather than by the chain, and they shift at bank holidays and at short notice. The branch's own page or its map listing has today's - that's the bit that catches people out.", "A branch keeps its own hours and they move for holidays, a delivery day or a refit, so nothing said from here would be right today. A map app's listing for that branch is the quickest check, and a phone call settles it.", "Hours like that are local and they change: bank holidays, stock takes, a summer timetable. Ring the place or look at its map listing - both are updated when the hours move."];
pub(super) static HOURS_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:when|what\s+time[s]?)\s+(?:does|do|is|are|will|would|can|should)\b[^?]{0,44}?\b(?:open|opens|opening|close|closes|closing|shut|shuts|shutting)\b|\b(?:is|are)\b[^?]{0,32}?\bopen\b"##).expect("HOURS_ASK")
});
pub(super) static HOW_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+(?:do|can|should|would)\s+(?:i|we)\b|\bhow\s+to\b|\bwhat\s+do\s+i\s+need\s+to\b|\bwhats\s+the\s+best\s+way\b"##).expect("HOW_TO")
});
pub(super) static HOW_TO_OPEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+(?:do|to|can|should)\s+(?:i\s+)?open\b"##).expect("HOW_TO_OPEN")
});
pub(super) static KICK_OFF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwhat\s+time\s+(?:is|does|do|will)\b[^?]{0,24}?\bkick\s*-?\s*off\b|\bwhen\s+(?:is|does)\b[^?]{0,24}?\bkick\s*-?\s*off\b|\bwhat\s+time\s+(?:is|does)\s+the\s+(?:match|game|final)\b|\bis\s+the\s+(?:match|game)\s+on\b"##).expect("KICK_OFF")
});
#[rustfmt::skip]
pub(super) const KICK_OFF_LINES:&[&str]=&["Kick-off moves for television and it can move twice, so the club's own page or a sports app is the one to trust - anything said from here could be a week out of date.", "The fixture list is the club's own, and televised games get moved late. Their page or a sports app has the time and says if it has been moved."];
pub(super) static LATE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+late\b|\bopen\s+late\b|\bopen\s+early\b|\bopening\s+(?:hours|times?)\b|\bclosing\s+time\b|\btill\s+(?:when|what\s+time)\b"##).expect("LATE_ASK")
});
pub(super) static LOCAL_AREA: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:my|our)\s+(?:town|village|area|county|streets?|roads?|ward|district|neighbourhood|postcode|council|city)\b"##).expect("LOCAL_AREA")
});
pub(super) static LOCAL_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+many\b|\bpopulation\b|\bwho\s+is\b|\bwhich\s+band\b|\bhow\s+do\s+i\s+find\b|\bwhat\s+is\s+the\s+population\b"##).expect("LOCAL_ASK")
});
#[rustfmt::skip]
pub(super) const LOCAL_LINES:&[&str]=&["Figures about your own patch come from your council, and they publish them: their site has it - the box at the top takes your postcode - and one call to the main number works if the site buries it.", "Your council holds that and keeps it up to date. Their site has it, or one call to the main number answers it - the postcode does most of the work."];
pub(super) static NEARBY_AREA: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bnear\s?(?:me|us|here)\b|\bround\s+here\b|\bnear\sby\b|\bnearby\b|\bin\s+my\s+(?:area|town|village|city|street|road|postcode|neighbourhood)\b|\bmy\s+(?:postcode|area|town|village)\b"##).expect("NEARBY_AREA")
});
#[rustfmt::skip]
pub(super) const NEARBY_LINES:&[&str]=&["What is near you depends on your postcode, so a map app is the one that knows: it lists the closest in order of distance, with today's hours and a phone number for each one.", "A map app puts them in order of distance and carries the phone number, which is faster than any list I could give you from here."];
#[rustfmt::skip]
pub(super) const NEARBY_SERVICE:&[&str]=&["atm", "bank", "cafe", "cashpoint", "chemist", "chippy", "dentist", "doctors", "garage", "gym", "hospital", "launderette", "laundrette", "nursery", "park", "petrol", "pharmacy", "pool", "pub", "restaurant", "school", "shop", "store", "supermarket", "surgery", "takeaway", "tip", "vet"];
pub(super) static NEAREST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\b(?:nearest|closest)\s+(?!to\b)[a-z]+"##).expect("NEAREST"));
#[rustfmt::skip]
pub(super) const NOT_A_VENUE:&[&str]=&["app", "bottle", "box", "boxes", "browser", "can", "day", "door", "doors", "email", "envelope", "file", "folder", "gift", "jar", "letter", "packet", "parcel", "present", "run", "school", "shift", "tin", "week", "window", "windows", "work"];
pub(super) static NOT_THEIR_STATUS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:my|our|order|parcel|delivery|claim|refund)\b"##).expect("NOT_THEIR_STATUS")
});
pub(super) static OFFICIAL_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:whats|what\s+is|what\s+are)\s+the\s+(?:law|rules?|legal\s+position|regulations?)\b|\bis\s+it\s+(?:legal|allowed|illegal)\b|\bam\s+i\s+(?:allowed|legally)\b|\bdo\s+i\s+need\s+(?:a|an|any|to)\b|\bhow\s+do\s+i\s+(?:get|apply|renew|replace|register|order|book)\b|\bhow\s+do\s+i\s+(?:check|find\s+out)\s+(?:if|whether|who|what)\b"##).expect("OFFICIAL_ASK")
});
#[rustfmt::skip]
pub(super) const OFFICIAL_LINES:&[&str]=&["Anything with a form behind it is set by the office that issues it, and the fee and the paperwork change with the year. Their official page - GOV.UK for the national ones, your council for anything local - names what to send, what it costs and how long it takes.", "The form, the fee and the waiting time all come from the office that issues it, and they change. Their own page is the one to act on, and it says who to ring if your case does not fit the standard one.", "The rules differ by council and by country, so the version that counts is on the issuing office's own page: GOV.UK has the national forms, and the page names who to ring about the rest."];
#[rustfmt::skip]
pub(super) const OFFICIAL_THING:&[&str]=&["application", "badge", "benefit", "blue", "card", "certificate", "consent", "copy", "crb", "cscs", "dbs", "disabled", "exemption", "form", "grant", "licence", "license", "number", "passport", "permission", "permit", "planning", "record", "records", "register", "registered", "registration", "renewal", "skip", "visa"];
pub(super) static ON_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:whats?|what\s+is)\s+on\b|\blistings?\b|\bwhats\s+on\s+at\b|\bis\s+(?:it|that|the\s+[a-z]+)\s+on\b|\bstill\s+on\b"##).expect("ON_ASK")
});
#[rustfmt::skip]
pub(super) const ON_EVENT:&[&str]=&["boot", "cinema", "concert", "event", "events", "fair", "festival", "fete", "game", "gig", "gigs", "market", "markets", "match", "play", "sale", "screening", "show", "theatre"];
#[rustfmt::skip]
pub(super) const ON_WORDS:&[&str]=&["cinema", "cineworld", "concert", "festival", "film", "films", "game", "gig", "gigs", "listing", "listings", "match", "movie", "movies", "multiplex", "odeon", "pictures", "play", "screening", "show", "theater", "theatre", "venue", "vue"];
pub(super) static OPENS_ASKING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^\s*(?:is|are|will|any|does|do|what|when|how)\b"##).expect("OPENS_ASKING")
});
#[rustfmt::skip]
pub(super) const PET:&[&str]=&["budgie", "cat", "cats", "dog", "dogs", "hamster", "kitten", "parrot", "pet", "pets", "puppy", "rabbit"];
pub(super) static PRICE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+much\s+(?:is|are|does|do|will)\b|\bwhats?\s+the\s+(?:price|charge|cost|fee|rate)\b"##).expect("PRICE_ASK")
});
#[rustfmt::skip]
pub(super) const PRICE_LINES:&[&str]=&["Rates like that are set by whoever runs the place and they change with the season: their own site or one quick call has the figure they actually charge, and a map listing carries the number if there is no site.", "Admission and parking prices are set locally and move with the season, so the current one is on their website or through one call, and a map listing has the number."];
#[rustfmt::skip]
pub(super) const PRICE_THING:&[&str]=&["admission", "bus", "buses", "cab", "charge", "delivery", "entrance", "entry", "fare", "fares", "fee", "fees", "hire", "park", "parking", "post", "postage", "rental", "taxi", "ticket", "tickets", "train", "trains", "tube"];
pub(super) static ROAD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"(?<![a-z0-9])[am]\s?\d{1,3}(?![a-z0-9])"##).expect("ROAD"));
pub(super) static RULES_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:law|legal|allowed|rules?|regulations?|illegal)\b"##).expect("RULES_ASK")
});
#[rustfmt::skip]
pub(super) const RULES_LINES:&[&str]=&["Rules like that are set locally and the byelaws differ from one council to the next: your council's site has them, plus the permit if one is needed, and their phone line answers the odd case.", "The rules differ by council and by country, so the local version is the only one worth acting on: the council's site has it, and GOV.UK has the national half."];
#[rustfmt::skip]
pub(super) const STATUS:&[&str]=&["cancel", "canceled", "cancellation", "cancelled", "closed", "delay", "delayed", "delays", "disruption", "disruptions", "diversion", "diverted", "roadworks", "running", "shut", "strike", "strikes", "traffic"];
#[rustfmt::skip]
pub(super) const STATUS_PHRASES:&[&str]=&["engineering works", "called off", "on time", "bus replacement", "replacement bus", "road works"];
pub(super) static STORY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:was|were|got|had|did|have|has|theyve|weve|ive|again|yesterday|this\s+morning|last\s+night)\b"##).expect("STORY")
});
pub(super) static TERM_DATES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bterm\s+dates?\b|\binset\s+days?\b|\bacademic\s+year\b|\bwhen\s+do\s+(?:the\s+)?schools?\s+(?:break|finish|start)\b"##).expect("TERM_DATES")
});
#[rustfmt::skip]
pub(super) const TERM_LINES:&[&str]=&["Term dates are the school's own calendar, and the inset days are the bit a general answer always gets wrong. The school's site has both, and the council lists every school's in one place if you need to compare.", "The school sets its own term dates and its own inset days. Their page has both, and the council's site collects them all if you are looking at more than one school."];
pub(super) static THEIR_OWN_ONLINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:my|our)\s+(?:site|website|web\s*page|page|shop|store|storefront|listing|listings|blog|channel|profile|etsy|ebay|account|sales)\b"##).expect("THEIR_OWN_ONLINE")
});
#[rustfmt::skip]
pub(super) const THE_PEOPLE:&[&str]=&["baby", "boss", "boyfriend", "child", "children", "dad", "daughter", "father", "friend", "girlfriend", "grandad", "granddaughter", "grandma", "grandson", "husband", "kid", "kids", "manager", "mom", "mother", "mum", "nan", "partner", "son", "wife"];
#[rustfmt::skip]
pub(super) const TRAVEL:&[&str]=&["a-road", "bus", "buses", "by-pass", "bypass", "coach", "coaches", "ferries", "ferry", "flight", "flights", "junction", "line", "lines", "motorway", "motorways", "plane", "planes", "replacement", "route", "routes", "service", "services", "traffic", "train", "trains", "tram", "trams", "tube", "underground"];
#[rustfmt::skip]
pub(super) const TRAVEL_CLAIM:&[&str]=&["claim", "claimed", "claims", "compensate", "compensation", "hotel", "insurance", "insurer", "money", "owed", "pays", "rebook", "rebooking", "reclaim", "refund", "refunds", "reimburse"];
#[rustfmt::skip]
pub(super) const TRAVEL_LINES:&[&str]=&["Disruption can clear or spread within the hour, so only today's version is worth acting on: the operator's app or a journey planner has the next service, and for a road the traffic service names the stretch.", "Whether it is running today is on the operator's own system, and delays move. Their app or a journey planner shows the next one and any disruption - check it just before you set off."];
#[rustfmt::skip]
pub(super) const WHATS_ON_LINES:&[&str]=&["Listings turn over every week and seats sell out quietly, so the venue's own page or a listings app is the only schedule that is right today: times, prices and what is left.", "That is this week's timetable, and it changes more often than a general answer could keep up with. The venue's own page or a listings app has the times, plus the number if you want to check seats."];
pub(super) static WHAT_IS_GOOD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:whats?|what\s+is|anything)\s+(?:good|worth\s+(?:seeing|watching|a\s+look))\b|\bwhats?\s+(?:the\s+)?(?:new|latest)\s+(?:film|movie|play|show)\b|\bwhat\s+(?:film|films|movie|movies|play|plays)\s+(?:is|are)\s+(?:on|showing|worth)\b|\bwhats?\s+worth\s+(?:seeing|watching)\b"##).expect("WHAT_IS_GOOD")
});
#[rustfmt::skip]
pub(super) const LIVE_POOLS:&[&[&str]]=&[&["No bot can see your own numbers, sorry. A site's analytics and a search engine's count never agree exactly, because each counts a visitor its own way, so pick one and stay with it: the analytics on your site, or the admin page for a shop or a till, holds the figures. The trend over a month says more than any single day does.", "Your own visitors are counted twice over, and the two counts differ: the analytics on your site counts pages opened, a search engine counts clicks, so the figures only ever line up roughly. Either one, watched over a month, tells you what happened; a single day does not.", "One busy day is usually one link somewhere, so the figure worth having is the trend rather than today's. That trend lives in the analytics for your site, or in the admin page for a shop or a till, and no one outside your own account can see it."],&["Quiet traffic is usually one row in the stats: a listing that stopped being refreshed, a link that died, or the week after a busy one. The analytics for your shop shows which pages people came in on, and the one that fell away is the one to refresh.", "A drop is rarely the whole shop. It is normally the thing you last changed, or a listing that got stale while the rest of them were being looked after: the admin page tells you which page lost the visits, and that page is the one worth an hour.", "The usual explanation is that a busy week pulls the next one down, so a dip after a good week is not a fall. If it is not that, the admin page shows the page that lost visits, and a listing relisted or refreshed often picks up again within a few days."],&["I can't check the forecast from here, sorry - no live weather in me. A weather app has today's for your postcode, hour by hour, and for a trip later in the year the monthly averages are the useful thing.", "No live weather from me, sorry. A forecast app or the national weather site has what is actually coming where you are, and the monthly averages for a trip months away."],&["The opening hours are the first thing a venue changes and the last thing it tells anyone: a refurb, a bank holiday or new owners will do it. Their own page or map listing has the current ones.", "No opening hours in front of me. Whoever runs the place - a council tip, a chain, one landlord - announces a change itself, and its own page or phone number has today's."],&["I can't look up live prices, sorry. What a figure like that turns on is which one it is, where you buy it and the day, so two or three sellers compared all-in will beat the first price you see - their own site has today's number, and bring it back and I'll work out what it comes to a month.", "Prices move too fast for me to know. Which one you pick and where you buy it decide most of the figure, and a quick look at the seller's own site has today's number, all in with delivery - tell me what it says and I'll do the sums with you."],&["I can't see the news from here, sorry. A news site or app will have the latest; tell me the story and I'll help you make sense of it."],&["Traffic comes back at the same times on most weekdays, so a road that is slow at eight is usually slow at eight, while a closure or roadworks is a diversion and a different wait altogether. I can't see the roads from here, sorry: a maps app with live traffic, or the roads agency's own site, has today's jams, closures and roadworks as they happen.", "A jam clears from the front and a closure means a diversion, and those want different places looked at. I can't see the roads from here, sorry: the roads agency's own site has today's closures and roadworks, and a maps app with live traffic has the jams and how long each one is adding."],&["I can't check live times or delays, sorry. The operator's own app or site has the next departure and any disruption, and a journey planner puts the whole trip together."],&["The time a drive takes is mostly decided by the hour you set off and by the roads you use, since town streets and the last few miles are never motorway speed. I can't see either from here: a route planner has the figure for the hour you would actually leave, roadworks included, and the stops worth making on the way.", "However far it is, the drive will take longer than the distance suggests, because the first and last miles are the slow part and the hour you leave changes the rest. A route planner has it for your own leaving time; a maps app does the same from wherever you are."],&["The hour you set off and the roads you use decide most of a drive, and the first and last miles are the slow part wherever you are going. Long from where to where, and roughly when would you set off? I can plan the day around it once I know.", "The hour and the last few miles decide a journey, and either end can be the slow one. Where are you starting, where are you heading, and roughly when? I can work through the timing and the stops with you."],&["I can't measure a distance from where you are, sorry. A map app does it from your own door in one tap, walking and driving both.", "That is a distance I can't take from this end. Put it into a map app and it shows how far and how long, from wherever you are standing."],&["Rounds are set street by street, not by town, and a bank holiday moves them by a day or two - so I can't see your street's day from here. The council's own site takes a postcode and gives the next collection; that is also where the holiday change is announced.", "Which day a street is collected on is the council's own choice, and holidays shift it, so there is nothing to know from this end without the postcode. The council's site has the round for your address, and a missed bin is free to report there on the day."]];
