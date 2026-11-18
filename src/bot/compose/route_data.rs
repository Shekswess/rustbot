//! Venue, policy and service route vocabularies from routes, findout and where_it_answers.
use fancy_regex::Regex;
use std::sync::LazyLock;
pub(crate) static INSURANCE_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:insurance|insurer|insured|insure|insures|policy|policies|excess|excesses|claim|claims|cover note)\b"##).expect("INSURANCE_WORD")
});
pub(crate) static TERMS_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:insurance|policy|cover note|pass|railcard|ticket|tickets|warranty|guarantee|membership|subscription|tariff|contract|lease|tenancy|terms|small print|conditions|excess|excesses|deposit|exclusions|breakdown|breakdown cover|recovery|roadside|contents cover|gadget cover|extended warranty)\b"##).expect("TERMS_THING")
});
pub(crate) static PRICE_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:price|prices|rate|rates|cost|costs|booking|room|rooms|hotel|holiday|package|deal|hire|fee|fees|quote|breakfast|meals?|parking|wifi|transfer|transfers|extras?|tax|taxes|vat|service charge|delivery|postage and packing|p&p)\b"##).expect("PRICE_THING")
});
pub(crate) static INCLUDED_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:is|are|was|were|does|do)\b[^?]{0,35}\b(?:include|includes|included|extra|free with|come with|comes with)\b"##).expect("INCLUDED_ASK")
});
pub(crate) static COVERED_PHRASE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:am i|are we|are they|is it|is that|is this|is my|is our|is the|is that phone|are my)\b[^?]{0,30}\bcovered\b"##).expect("COVERED_PHRASE")
});
pub(crate) static TERMS_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:cover|covers|covered|include|includes|included|excluded|excludes|apply|applies|count|counts|valid|honour|honoured|pay out|payout|work|works|worked|accepted|accept|any use|use it|use on)\b"##).expect("TERMS_ASK")
});
pub(crate) static WORKING_OUT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\bwork(?:s|ed|ing)?\s+out\b"##).expect("WORKING_OUT"));
pub(crate) static ALLOWED_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:is|are|was|were|am i|are we|can i|can we|could i|may i)\b[^?]{0,30}\b(?:allowed|permitted|legal|against the rules)\b"##).expect("ALLOWED_ASK")
});
pub(crate) static TAKE_ON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:can|could|may|am i allowed to|are we allowed to)\s+(?:i\s+|we\s+)?(?:take|bring|carry|pack|board with)\b[^?]{0,60}\b(?:on|onto|into|in|aboard)\b"##).expect("TAKE_ON")
});
pub(crate) static TAKE_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:can|could|may)\s+(?:i|we)\s+(?:bring|take|carry)\b[^?]{0,60}\bto\b"##)
        .expect("TAKE_TO")
});
pub(crate) static NEEDED_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:licen[cs]e|licen[cs]es|licen[cs]ing|licen[cs]ed|permit|permits|visa|visas|register|registration|registered|certificate|certification|medical|inspection|qualification|qualifications|background check|dbs check|crhb|health and safety|passport|passports)\b"##).expect("NEEDED_THING")
});
pub(crate) static NEEDED_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:need|needs|needed|require|requires|required|have to|has to|must|renew|renewing|speed up|speed it up|expedite|fast track|pay extra|pay more)\b"##).expect("NEEDED_ASK")
});
pub(crate) static AGREEMENT_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:tenancy|tenant|lease|landlord|letting agent|contract|membership|gym|subscription|phone|mobile|broadband|internet|insurance|policy|booking|holiday|rental|hire|course)\b"##).expect("AGREEMENT_WORD")
});
pub(crate) static NOT_AN_AGREEMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:card|meeting|dinner|lunch|reservation|restaurant|appointment|appointments|plans?|party|birthday|barbecue|bbq)\b"##).expect("NOT_AN_AGREEMENT")
});
pub(crate) static LEAVING_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:cancel(?:ling)?\s+early|leave\s+early|break the contract|end the contract|charge\w*\s+me\s+to\s+(?:leave|cancel|go)|how much to cancel|notice period|early termination)\b"##).expect("LEAVING_ASK")
});
pub(crate) static GET_OUT_OF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\bget(?:ting)?\s+out\s+of\b"##).expect("GET_OUT_OF"));
pub(crate) static SHIFT_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:rota|shift|shifts|boss|manager|supervisor)\b"##).expect("SHIFT_WORD")
});
pub(crate) static LEAVE_EARLY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\bleav(?:e|ing)\s+early\b"##).expect("LEAVE_EARLY"));
pub(crate) static CANCEL_TERMS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:cancellation (?:fee|fees|charge|charges|policy)|how much to cancel|fee to cancel|cost to cancel|charge to cancel|refund policy)\b"##).expect("CANCEL_TERMS")
});
pub(crate) static HARM_NOUN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:damage|damaged|flood|flooding|flooded|storm|storms|wind|water|fire|theft|thief|thieves|stolen|stole|accidental|accident|liquid|injury|injuries|illness|subsidence|burst|vermin)\b"##).expect("HARM_NOUN")
});
pub(crate) static TERM_SHAPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow long\b[^?]{0,30}\b(?:warranty|guarantee|guaranteed|cover|policy|contract|lease|tenancy|membership|subscription|protection)\b|\b(?:cancellation|refund|returns?|claims?|exchange) polic(?:y|ies)\b|\binsur(?:e|ed|es|ing|ance)\b|\b(?:what|which|whos|who) (?:is |are )?(?:not )?cover(?:ed|s)?\b"##).expect("TERM_SHAPE")
});
pub(crate) static PRICE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:how much|how many|cheapest|cheaper|quote|quotes?|priced?|prices?|cost|costs|costly|roughly|budget)\b"##).expect("PRICE_ASK")
});
pub(crate) static TERM_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:policy|policies|excess|excesses|claim|claims|terms|small print|conditions|exclusions|warranty|guarantee|deposit|premium|premiums)\b"##).expect("TERM_WORD")
});
pub(crate) static COUNT_AS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bcount(?:s|ed|ing)?\s+(?:towards|toward|as)\b"##).expect("COUNT_AS")
});
pub(crate) static ALLOWANCE_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:luggage|baggage|allowance|limit|weight|fare|fares|ticket|tickets|quota|carry on|cabin bag|hand luggage|bag|bags|pushchair|buggy|pram)\b"##).expect("ALLOWANCE_WORD")
});
pub(crate) static NAMED_OBJECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:take|bring|carry|pack|board with)\s+(?:it|this|that|them|those|one)\b"##)
        .expect("NAMED_OBJECT")
});
pub(crate) static VENUE_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:ferry|plane|flight|train|bus|coach|tube|tram|boat|cruise|hotel|restaurant|cafe|pub|camping|campsite|camp site|festival|stadium|ground|museum|gallery|cinema|pool|park|beach|venue|abroad|airport|airline|airlines|on board|carry on|cabin bag|hand luggage|luggage|baggage|suitcase|arena|theatre|gig|club|conference|showground|racecourse)\b"##).expect("VENUE_WORDS")
});
pub(crate) static CONTRACT_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:landlord|letting agent|tenancy|tenant|deposit|notice|contract|employer|employee|boss|redundancy|dismissed|sacked|refund|fined|fine|penalty|course of action|small claims|rent|evict|evicted|eviction|housing|mortgage|arrears|work|working|job|jobs|shift|shifts|annual leave|time off|day off|holiday ?pay|sick pay|maternity|paternity|overtime|rota)\b"##).expect("CONTRACT_WORDS")
});
#[rustfmt::skip]
pub(crate) const ISSUER: &[&str] = &["A licence, a permit or a visa is granted on conditions the issuing body writes, and most publish them as a short list: what you need, what it costs, how long it takes. The council grants licences and permits, and the government's site has passports, visas and driving licences. Their page first, their phone line if your own case is unusual.", "Conditions come from whoever grants the permission, and they are on its own site rather than in the form. The same body can tell you whether your own case needs one at all, and a phone call about that is usually quicker than filling anything in."];
#[rustfmt::skip]
pub(crate) const LOCAL: &[&str] = &["Council rules are written for a street or a postcode, which is exactly why nobody else can answer them for you: a parking permit, the bin day, what the tip takes and planning are all on your own council's site. Their phone line gets you a straight answer about your address.", "Parking, bins, a permit, a hedge, planning: the council writes those rules and enforces them, and they are on its site under those headings. A ring with your address sorts out which one applies to you."];
pub(crate) static QUESTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^\s*(?:is|are|was|were|does|do|did|will|would|can|could|should|shall|may|am|have|has|what|whats|which)\b|\b(?:is it|is that|is this|is she|is he|is my|is the|are they|are we|does it|does my|does the|do they|do i|will it|will my|would it|can i|can we|should i)\b|\bhow (?:much|long|do i|can i|does|would)\b"##).expect("QUESTION")
});
pub(crate) static WARD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:hospital|ward|icu|hdu|itu|nicu|scbu|intensive care|high dependency|patient|patients|ward sister|nurse in charge)\b"##).expect("WARD")
});
pub(crate) static VISITOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\b(?:visitors?|visiting|visit)\b"##).expect("VISITOR"));
pub(crate) static SOMEONES_PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:my|his|her|our|their)\s+(?:mum|mom|mam|dad|mother|father|nan|nana|gran|granny|son|daughter|sister|brother|wife|husband|partner|friend|aunt|uncle|cousin|neighbour|neighbor|grandad|grandpa|grandma)\b"##).expect("SOMEONES_PERSON")
});
pub(crate) static LAW_NAMED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:the law|in law|against the law|legal|illegal|legally|unlawful|statute|statutory|consumer rights|gov\.?uk|offence|offense|criminal|solicitor|lawyers?)\b"##).expect("LAW_NAMED")
});
pub(crate) static CARRYING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:take|taking|bring|bringing|carry|carrying|come in|walk in|get in)\b"##)
        .expect("CARRYING")
});
pub(crate) static TO_INSIDE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:into|inside|on board|aboard|through security|in the cabin)\b"##)
        .expect("TO_INSIDE")
});
pub(crate) static PERSON_ALLOWED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:she|he|my son|my daughter|my kid|my kids|my child|my boy|my girl|the kids)\b[^?]{0,20}\ballowed\b"##).expect("PERSON_ALLOWED")
});
pub(crate) static COURT_ARM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:summons|summoned|magistrate|court date|court hearing|tribunal|prosecut\w*|penalty charge notice|pcn|fixed penalty|fine notice|appeal the fine|contest the fine)\b"##).expect("COURT_ARM")
});
pub(crate) static LOCAL_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:parking|parked|park on|permit|pavement|kerb|dropped kerb|bins?|bin day|fly tip|fly tipping|recycling|planning permission|noise|neighbour's hedge|footpath|right of way|council|tip|skip permit|dog fouling|street light)\b"##).expect("LOCAL_WORDS")
});
pub(crate) static SERVICE_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:bins?|wheelie|recycling|bulky waste|household waste|garden waste|the tip(?! of)|recycling cent(?:re|er)|street lights?|street lamps?|potholes?|kerbs?|gritting|fly ?tipping|(?:parking|resident|visitor) permits?|planning permission|planning application|allotments?|council tax)\b"##).expect("SERVICE_WORDS")
});
pub(crate) static NOISE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\bnoise\b|\bnoisy\b"##).expect("NOISE"));
pub(crate) static COMPLAINT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:neighbours?|next door|loud music|nuisance|complain\w*|report\w*|environmental health|council)\b"##).expect("COMPLAINT")
});
pub(crate) static NOT_A_SERVICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bbin(?:ned|ning)?\s+(?:it|that|this|them|those|these)\b|\b(?:should|shall|do|can)\s+(?:i|we)\s+bin\b|\b(?:bread|kitchen|pedal|nappy|compost|diaper|sensor|swing) bins?\b|\brecycle bin\b|\brecycling bin\b|\bwindows?\b|\blaptop\b|\bdesktop\b"##).expect("NOT_A_SERVICE")
});
pub(crate) static ASKING_SHAPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:what|how|where|who|when|why|which|can|could|should|do|does|did|is|are|will|would)\b|\?"##).expect("ASKING_SHAPE")
});
pub(crate) static BODY_STATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:on an empty stomach|empty stomach|with food|with water|before food|after food|before eating|after eating|on antibiotics|with alcohol|on a full stomach)\b"##).expect("BODY_STATE")
});
pub(crate) static COVER_IT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:should|shall|do|can)\s+(?:i|we)\s+(?:cover|covering)\b"##)
        .expect("COVER_IT")
});
pub(crate) static COVERED_IN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\bcovered\s+(?:in|with)\b"##).expect("COVERED_IN"));
#[rustfmt::skip]
pub(crate) const TERMS: &[&str] = &["Cover sits in a section headed what is included and what is excluded, and the exclusions are often the longer list. That section is the one that counts; if the page is vague, the letter or email they sent you repeats the same words, and asking them in writing leaves you with an answer you can point to.", "This turns on one clause: what the policy calls the thing, and whether your loss is on its list of exclusions. Find that section before you rely on it, and ask them to confirm it in writing, because a written answer is the one you can hold them to.", "What applies is what the wording says, and it is written in the same three places: the section on what is included, the exclusions, and the letter they sent when you took it out. A call to whoever sold it you settles the last doubt, and their answer in writing settles it for good."];
#[rustfmt::skip]
pub(crate) const INCLUDED: &[&str] = &["What a price covers is written where it is sold: the booking page lists what comes with it, and anything it does not list is charged on top, often at the desk rather than before. Ask for the total in writing before you pay, and keep that email for the day itself.", "Included and not included are two lists, and the second is the one that costs you: it is on the page you book on, then at the desk, where the extras get added. Read the first list before you book and ask for the total by email, because that is the version they have to honour.", "A room, a package or a rate names what comes with it in its own terms, and whatever it does not name is charged separately - parking, breakfast and the wifi are the usual three. Check that list before you book, and ring to confirm what applies to your own booking."];
#[rustfmt::skip]
pub(crate) const LOCAL_SERVICE: &[&str] = &["Bins, the tip, permits, a street light, a complaint about noise: each one is your local authority's own service, decided for a whole street at a time and published by the council or town hall that runs it. Their site takes your postcode and gives the page for your address, and their phone line with the address answers faster than a search will. Tell me what they say and I'll help with the next step.", "That one is settled per address rather than in general, so I can't see your street from here: the council or town hall for your address decides it for its own postcodes, and publishes what it wants from you - the address, the date, and usually one photo. Their page first, then their phone line if your case does not fit it. What is it you are trying to get done?", "Your council or town hall runs that service and writes the rule for it street by street, so its site with your postcode in it is the only place that is right for your address. Ring them if the page does not cover your case - they answer for one address faster than a search will. What have you got so far?"];
pub(crate) static TICKET_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:railcard|rail pass|ticket|tickets|season ticket|oyster|travelcard|travel card|boarding pass|day pass|bus pass|fare|fares|rover|interrail)\b"##).expect("TICKET_WORD")
});
#[rustfmt::skip]
pub(crate) const OPERATOR: &[&str] = &["A ticket usually names on its face the route, the operator and the times it is good for, and the full conditions live on the operator's page rather than at the station. Check the ticket, then their page on what the fare covers; the guard on board checks the one you have.", "What a pass is good on is set by the operator whose buses or trains it is for, and their page on what is included lists it by name: the routes, the times and the extras. A call to them settles your own ticket in a few minutes.", "Fares and passes come with conditions written by the company that runs the service, and they are published with the timetable rather than with the ticket. Their page on what the fare covers is the one that counts, and the guard on the day has the last word."];
#[rustfmt::skip]
pub(crate) const INSURER: &[&str] = &["A policy is decided by its own wording and its list of exclusions, not by the kind of claim it sounds like. Read the exclusions first, then ring the insurer to confirm your own case before you claim or pay for anything: they have to tell you, and that answer is the one that counts.", "Insurers decide this from the wording in your schedule, which is written when the policy is taken out and does not change with the claim. Their claims line will tell you straight away, and it is worth asking for it by email so you have it in writing.", "Whether a claim is met is the policy's own test: what is included, then what is excluded, then the excess. The insurer is the only one who can apply that to your case, and asking before you spend anything is the cheap way round."];
#[rustfmt::skip]
pub(crate) const ALLOWED: &[&str] = &["Places and companies write these rules down, usually under a heading like what you can bring or what is not allowed, and the person on the door has the last word on the day. Find that page, then ask them by email so you have the answer to show.", "This is somebody else's rule to make, and they are the only ones who can apply it to your case. An email asking whether it is allowed is worth more than a phone call, because you can point to it later.", "Rules like this are set by whoever runs the thing rather than by the law or by me, and they are written where you book. If the page is vague, ring before you set off: being turned away at the door is the expensive way to find out."];
#[rustfmt::skip]
pub(crate) const VENUE: &[&str] = &["A ferry, an arena, a campsite: whoever runs the place writes what you can bring on, and it is on their own page about what is allowed in, not in the booking email. Worth a call before you travel if the page is vague.", "What you can bring on with you is decided by whoever runs the place, and it is written for travellers rather than for lawyers - on their page about what is allowed in. A quick call settles it before you pack.", "Their rule, written where you book: most places publish what is allowed in and what has to go in the hold or be left at the door. The page first, then a ring to confirm your own case."];
#[rustfmt::skip]
pub(crate) const CONTRACT: &[&str] = &["That is set by what you agreed to: the section on notice and fees says what it costs to end it early, and it is usually a fixed sum or the rest of the term. If it is a tenancy or a job, Citizens Advice, or ACAS for work, will read it beside you for free.", "The notice and the fee are on the paper you signed, or in the account page if you joined online, and it is usually one short section headed ending the agreement. Ask them for the figure in writing, because that is the one you can hold them to.", "Leaving early usually costs a fixed sum or the rest of the term, and the section that says which is in the terms you agreed. If the wording is not clear, Citizens Advice will go through a tenancy or an employment contract with you for free."];
pub(crate) static A_PLACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:hotel|guest ?house|b\s?and\s?b|hostel|campsite|caravan park|holiday park|cottage|villa|apartment|airport|station|venue|concert hall|leisure cent(?:re|er)|swimming pool|gym|restaurant|pub|club|theatre|cinema|hospital|surgery|office|rooms?|bedrooms?|the flat|the house|the cottage|the caravan|flat we(?:ve| have) booked|the building|building|the premises|the shop|the salon|the clinic|the garage)\b"##).expect("A_PLACE")
});
pub(crate) static A_FACILITY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:parking|car park|wifi|wi-?fi|internet|pool|gym|spa|sauna|breakfast|air ?con(?:ditioning)?|kitchen(?:ette)?|lift|elevator|shower|bath|towels?|hairdryer|hair dryer|kettle|microwave|fridge|freezer|washing machine|iron|tv|television|safe|balcony|room service|charger|plug sockets?|toaster|crockery|cutlery|pans|a cot|cot bed|high ?chair|towels|bedding|linen|accessible|wheelchair|dog friendly|disabled access)\b"##).expect("A_FACILITY")
});
pub(crate) static A_ROOM_FACILITY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:parking|car park|wifi|wi-?fi|internet|air ?con(?:ditioning)?|kitchen(?:ette)?|lift|elevator|shower|bath|towels?|hairdryer|hair dryer|kettle|microwave|fridge|washing machine|iron|tv|television|a safe|the safe|balcony|toaster|a cot|cot bed|high ?chair|plug sockets?|room service)\b"##).expect("A_ROOM_FACILITY")
});
pub(crate) static A_HAS_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:is there|are there|is there any|do they have|does (?:it|this|that|the [a-z]+) have|have (?:they|you) got|does it come with|will there be|is it got|got a|do (?:the|these|those|your) [a-z]+ have)\b"##).expect("A_HAS_ASK")
});
pub(crate) static NOT_A_HAS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:open|opens|opening|closed?|closing|hours|what time|how much|price|cost|charge|fee|parking permit|bus|train|tube|tram|roadworks|traffic)\b"##).expect("NOT_A_HAS")
});
pub(crate) static THE_OLD_ONE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:the|my|our|that|this|these|those) old (?:one|ones|unit|units|boiler|fridge|freezer|fridge ?freezer|washing machine|dishwasher|tumble dryer|cooker|oven|hob|bath|shower|toilet|radiators?|windows?|doors?|carpets?|sofa|settee|bed|mattress|tyres?|batteries|cupboard|wardrobe|kitchen|bathroom|fire|fireplace|water tank|tank)\b"##).expect("THE_OLD_ONE")
});
pub(crate) static MOVING_THE_OLD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:keep|keeping|take\b[^,.;]{0,30}\baway|getting? rid|get rid|dispose|disposal|recycl\w*|scrap\w*|trade ?in|remove|removed|removal|throw (?:it|them) out|skip|tip|leave (?:it|them)|what happens to)\b"##).expect("MOVING_THE_OLD")
});
pub(crate) static NOT_THE_OLD_ONE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:rice|potatoes?|bread|milk|chicken|meat|curry|soup|leftovers?|stock|pasta|veg|cat|dog|horse|rabbit|hamster|fish)\b"##).expect("NOT_THE_OLD_ONE")
});
#[rustfmt::skip]
pub(crate) const A_PLACE_HAS: &[&str] = &["Whether a place has a thing is decided by that place and not by its name: the same chain differs from one branch to the next, and a station, an office and a hotel each answer for themselves. Its own site or the page you book on lists what comes as standard and what costs extra, and one call to the place confirms what the page leaves out - ask for it in writing if it matters on the day.", "Facilities belong to whoever runs the place, so the current list is on their own page rather than anywhere general, and anything charged for usually sits on a line of its own, so look for both before you decide. If the page is silent, the place itself is the one that knows.", "That turns on the building rather than the name above the door, and the people there know it exactly. Ring or email the place itself and ask them to confirm it for the day you are going, so you have it in writing if it turns out not to be there."];
#[rustfmt::skip]
pub(crate) const THE_OLD_ONE_POOL: &[&str] = &["The old one is usually the fitter's to take: whoever installs the new one carries the old away for a small fee if you ask when the job is quoted, and a shop that only delivers will not touch it. Ask about it with the quote and it is one less thing on the day.", "Whether you can keep it turns on who owns it now: bought outright it is yours, and rented or on finance it is not - that is in the agreement, not on any page. If it is yours and nobody wants it, the council's own site lists the bulky collection, what they take and what it costs.", "Two things settle it: who owns it after the swap, and who will move it. The installer answers the first when they quote, and a scrap merchant or the council's bulky waste service answers the second - both are worth asking before the day rather than on it."];
pub(crate) static BUSY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+busy\b|\bis\s+(?:it|the\s+[a-z]+)\s+busy\b|\bhow\s+long\s+(?:is|are)\s+(?:the\s+)?(?:wait|queue|line)\b|\bwhats\s+the\s+(?:wait|queue|line)\b|\bwaiting\s+times?\b|\bis\s+there\s+(?:a\s+|any\s+)?(?:queue|wait|line)\b|\bwill\s+there\s+be\s+(?:a\s+)?(?:queue|wait|line)\b|\bhow\s+(?:packed|crowded|heaving|rammed)\b|\bwhats\s+the\s+(?:waiting\s+time|wait\s+time)\b|\bis\s+the\s+(?:queue|wait|line)\s+(?:long|bad|big|short|quick|slow)\b|\bwhen\s+(?:is|are)\s+(?:\w+\s+){0,4}(?:quiet|emptiest|quietest|least\s+busy)\b|\b(?:is|are|was|were|will|would)\s+(?:\w+\s+){1,4}(?:busy|packed|heaving|rammed|crowded|manic)\b"##).expect("BUSY")
});
pub(crate) static BUSY_ASKS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^\s*(?:is|are|was|were|will|would|do|does|did|can|could|should|any|how|what|whats|when|where|which|who)\b|\?\s*$"##).expect("BUSY_ASKS")
});
pub(crate) static PROPOSAL_SEARCH_ONLY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:beats searching|a search (?:should|will)|quick search|searching for it will|quickest route to a straight answer|would rather not guess|beyond what i know|not sure about that one)\b"##).expect("PROPOSAL_SEARCH_ONLY")
});
pub(crate) static PROPOSAL_A_YES_OR_NO_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:is there|are there|do they have|does (?:it|this|that|the [a-z]+) have|have they got)\b"##).expect("PROPOSAL_A_YES_OR_NO_ASK")
});
pub(crate) static PROPOSAL_LIVE_ONLY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)(timetables? and listings|change by the hour|that is live|live, sorry|today's included|the current one, today)"##).expect("PROPOSAL_LIVE_ONLY")
});
pub(crate) static PROPOSAL_INCLUDED_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r##"(?i)\b(?:includes?|included|in with|comes? with|part of the price|on top|extra)\b"##,
    )
    .expect("PROPOSAL_INCLUDED_ASK")
});
pub(crate) static PROPOSAL_A_FIGURE_REPLY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)(?:comes from a quote or a listing|no list price|price the same description|ask two firms to price|compare what each one leaves out)"##).expect("PROPOSAL_A_FIGURE_REPLY")
});
pub(crate) static PROPOSAL_MONEY_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:rate|rates|charge|charges|price|prices|pricing|cost|costs|fee|fees|tariff|per hour|per day|per night)\b"##).expect("PROPOSAL_MONEY_WORDS")
});
#[rustfmt::skip]
pub(crate) const PROPOSAL_QUESTION_FILLER: &[&str] = &["a", "about", "again", "all", "also", "always", "am", "an", "and", "another", "any", "are", "as", "at", "be", "been", "being", "bit", "both", "but", "by", "came", "can", "come", "comes", "coming", "could", "did", "do", "does", "down", "each", "either", "ever", "every", "far", "for", "from", "get", "gets", "getting", "go", "goes", "going", "got", "had", "has", "have", "having", "he", "her", "here", "hers", "him", "his", "how", "i", "if", "in", "into", "is", "it", "just", "like", "long", "lot", "many", "may", "me", "might", "mine", "more", "most", "much", "must", "my", "neither", "never", "no", "nor", "not", "of", "off", "often", "on", "one", "ones", "only", "or", "other", "our", "out", "over", "own", "per", "quite", "rather", "really", "same", "shall", "she", "should", "so", "some", "still", "such", "take", "taken", "takes", "taking", "than", "that", "the", "their", "theirs", "them", "then", "there", "these", "they", "thing", "things", "this", "those", "to", "too", "took", "under", "up", "us", "very", "was", "we", "went", "were", "what", "when", "where", "which", "who", "whom", "whose", "why", "will", "with", "would", "yes", "yet", "you", "your", "yours"];
#[rustfmt::skip]
pub(crate) const PROPOSAL_OLD_ONE_VERBS: &[&str] = &["away", "disposal", "dispose", "get", "gets", "getting", "happen", "happened", "happens", "keep", "keeping", "kept", "leave", "leaving", "rid", "take", "taken", "taking", "throw", "throwing", "want", "wanted", "wants"];
pub(crate) static PAST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s*(?:was|were)\b"##).expect("PAST"));
pub(crate) static EARLY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\bearly\b"##).expect("EARLY"));
pub(crate) static COVER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\bcover(?:ed|s)?\b"##).expect("COVER"));
#[rustfmt::skip]
pub(crate) const HEDGE_HEADS: &[&str] = &["A cab is metered, so it is the clock and", "A job is priced by the quote, not by a l", "A number I invented would be worse than ", "A postcode, a phone number and opening h", "A taxi fare is the meter, so the time of", "A time is a rate times a size: minutes p", "Booked far enough ahead, a fare is a dif", "Can't call that one for you, sorry. Say ", "Every one of these is quoted per person ", "Give me the measurements - the area's le", "How long it takes comes from how big the", "How much you need comes from how much on", "How much you need is per head, and wheth", "I can do that sum if you give me the num", "I can draft that: tell me who it's for a", "I can't see their system, but this order", "I cannot weigh that one safely, sorry. R", "I don't know enough about your money to ", "I don't know enough to answer that hones", "I'm not sure about that one, sorry — I'd", "I'm not sure what to say without the det", "If it's on your mind enough to ask, it's", "Miles and minutes depend on the roads on", "Not a call I can make for you, sorry: an", "Not mine to decide, sorry. Tell me what ", "Not one I can call for you, sorry: I can", "Not unreasonable at all. How you feel ab", "Oven times go by weight and heat: minute", "Prices move by exactly what you are buyi", "Reviews beat a guess from me about {thin", "Reviews beat anything I could say about ", "Stock moves by the hour and a phone call", "Stock moves by the hour, so a list from ", "That is a keeping question rather than a", "That is an adviser's call rather than mi", "That is impossible to call from one numb", "That one is a coverage sum rather than a", "That one is a minutes-per-500g job at a ", "That one is arithmetic rather than knowl", "That one is priced by the head and by th", "That price turns on the day, the time an", "That's beyond what I know well, sorry. S", "That's your call, not mine — I don't kno", "The Wikipedia page for it answers that i", "The date on a figure matters as much as ", "The day is what sets that one, not the s", "The label says what one pack covers and ", "The place's own page is the one with tod", "The recent reviews are what tells you an", "The record is the only place it is writt", "The roads decide it rather than the map,", "The wait and the fee are the parts that ", "The weight and the oven heat decide it, ", "Their system isn't visible from here, so", "There is no one price for it. What it co", "Time is the part I can work out but cann", "Two things decide that, and neither is o", "What a figure like that turns on is whic", "What it comes to turns on the count and ", "Whether it is still good turns on two th", "Whether that is dear depends on which on", "Whether they're taking new people and ho", "Who is any good and who is free changes ", "Work it from the number of people, not f", "You're not being silly. Feelings don't n"];
#[rustfmt::skip]
pub(crate) const HEDGE_PREFIXES: &[&str] = &["Reviews beat a guess from me about ", "Reviews beat anything I could say about ", "Stock moves by the hour and a phone call is the only thing that checks it, so I can't see who has ", "Stock moves by the hour, so a list from me would be wrong by the time you got there: a map app or the chain's own site will say who has ", "The recent reviews are what tells you and I can't see them from here: a map app search for ", "The wait and the fee are the parts that decide it and I can't see either from here: a map app or the official finder for ", "Who is any good and who is free changes month to month, so I can't vouch for one: a directory or map app search for "];
