//! Source tables for the object, document and instruction-sheet reader.
use fancy_regex::Regex;
use std::sync::LazyLock;
pub(super) static NOT_HERE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:pain|hurts?|ache|aching|sore|swollen|swelling|rash|lump|bleeding|blood|dose|dosage|tablets?|pills?|medicines?|medication|prescription|symptoms?|sick|vomiting|dizzy|breath|chest|stomach|mum|mom|dad|son|daughter|wife|husband|partner|baby|child|kids?|gran|nan)\b"##).expect("NOT_HERE")
});
pub(super) static NOT_ABOUT_THE_CONVERSATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:what|which)\s+(?:was|were|did|have|had)\s+(?:i|we|you)\b|\bwhat\s+am\s+i\b|\bdo\s+you\s+remember\b|\bwhat\s+did\s+(?:i|we|you)\s+(?:say|ask|tell)\b|\bwhat\s+(?:i|we)\s+(?:said|asked|told)\b|\bremind\s+me\b"##).expect("NOT_ABOUT_THE_CONVERSATION")
});
pub(super) static NOT_A_LOOKUP_BUT_A_REQUEST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:help\s+me|can\s+you|could\s+you|would\s+you|will\s+you|shall\s+i)\s+(?:write|draft|word|put|phrase|text|reply|answer|compose|translate|summari[sz]e|rewrite|shorten|tidy|check|read)\b|\bwrit(?:e|ing)\s+(?:it|that|this|him|her|them)\b|\bdraft\s+(?:it|that|this|something)\b|\b(?:write|reply|text|email|message|answer)\s+(?:to\s+)?(?:him|her|them|the\s+\w+)\b|\bhow\s+(?:do|should)\s+i\s+(?:write|word|phrase|put)\b|\b(?:re)?word\s+(?:it|that|this)\b|^make\s+(?:it|this|that|them)\b|\bmake\s+it\s+(?:more|less|a\s+bit|much)\b|\b(?:add|drop|cut|keep)\s+(?:a|the)\s+(?:line|bit|sentence|word)\b"##).expect("NOT_A_LOOKUP_BUT_A_REQUEST")
});
pub(super) static A_JUDGEMENT_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bworth\b|\bvalue\b|\bany\s+good\b|\bgood\s+(?:idea|buy)\b|\bhow\s+much\b|\bcheaper\b|\bbetter\s+buy\b|\bshould\s+(?:i|we)\s+(?:buy|get)\b|\bwaste\s+of\s+money\b"##).expect("A_JUDGEMENT_ASK")
});
pub(super) static A_PLAN_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwhat\s+time\s+(?:should|do|shall|would)\s+(?:i|we)\b|\bwhen\s+should\s+(?:i|we)\b|\bshould\s+(?:i|we)\s+(?:start|begin|leave|set\s+off|get\s+up)\b|\bhow\s+long\s+(?:before|ahead)\b|^\W*shall\s+i\b|\bshall\s+i\s+(?:also|do|just|be)\b"##).expect("A_PLAN_ASK")
});
pub(super) static A_COOKING_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:cook|cooked|cooking|roast|bake|baking|fry|frying|boil|grill|simmer|steam|poach|marinate|season|seasoning|baste|skin|thighs?|breasts?|fillet|mince|pasta|rice|sauce|dough|pastry|chicken|beef|pork|lamb|turkey|fish|prawn|veg|vegetables|potatoes|onion|garlic)\b"##).expect("A_COOKING_LINE")
});
pub(super) static A_READING_JUST_GIVEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:is|was|are)\s+(?:that|this|it|those)\s+(?:too|about\s+right|ok\b|okay\b|normal\b|enough\b|much\b|low\b|high\b|fine\b)|\bis\s+\d[\d.,]*\s*\w*\s+(?:enough|too|ok\b|fine\b|normal\b)"##).expect("A_READING_JUST_GIVEN")
});
pub(super) static A_WHY_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bis\s+that\s+why\b|\bwhy\s+(?:does|do|is|are|would|did|has|have|am\s+i)\b|\bwhat\s+causes?\b|\bcould\s+that\s+be\s+(?:why|it)\b|\bis\s+that\s+(?:what|the\s+reason)\b"##).expect("A_WHY_ASK")
});
pub(super) static A_METHOD_OR_ADVICE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+(?:do|can|should|would)\s+(?:i|we|you)\s+(?:take|get|put|fit|remove|undo|open|change|replace|fix|clean|unblock|unblock|bleed|reset|wire|connect|assemble|dismantle|strip|hang|mount|seal|glue|screw|drill|paint|cut|adjust|tighten|loosen)\b|\bwhat\s+would\s+you\s+do\b|\bwhat\s+should\s+(?:i|we)\s+do\b|\bwhat\s+do\s+you\s+(?:think|reckon|suggest|recommend)\b|\bany\s+(?:advice|ideas|thoughts)\b"##).expect("A_METHOD_OR_ADVICE_ASK")
});
pub(super) static A_DECISION_ABOUT_GOING_ON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bshould\s+(?:i|we)\s+(?:stop|keep|carry\s+on|carry|bother|risk|go\s+ahead|wait|use|still|replace|fix|put\s+up\s+with)\b|\b(?:is|was)\s+(?:it|that)\s+(?:safe|ok\b|okay\b|fine\b|alright)\s+to\s+(?:keep|carry|use|go\s+on|carry\s+on)\b"##).expect("A_DECISION_ABOUT_GOING_ON")
});
pub(super) static A_NOW_OR_NEVER_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:now|today|tonight|tomorrow|this\s+evening|the\s+day\s+before|the\s+night\s+before|last\s+night|overnight)\b[^.?!]{0,40}\b(?:can|could|should|shall|will)\s+(?:i|we)\b|\b(?:can|could|should|shall)\s+(?:i|we)\b[^.?!]{0,40}\b(?:now|today|tonight|tomorrow|overnight|the\s+day\s+before)\b"##).expect("A_NOW_OR_NEVER_ASK")
});
pub(super) static NOT_ABOUT_THE_VISIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwill\s+(?:they|he|she|it)\s+need\b|\bneed\s+to\s+come\s+(?:in|inside|out|back|round)\b|\bcome\s+(?:in|inside|out|back|round)\b|\bneed\s+access\b|\bget\s+to\s+it\b|\bhow\s+long\s+will\s+(?:they|he|she|someone|the\s+(?:engineer|plumber|fitter|installer|technician|electrician|man))\b|\b(?:they|he|she)\s+(?:turn\s+up|arrive)\b|\b(?:have|get)\s+(?:it\s+|everything\s+|them\s+)?ready\b|\bready\s+(?:for|when)\s+(?:them|him|her|someone|the\s+\w+)\b|\bwhat\s+(?:do|should)\s+i\s+(?:tell|say\s+to|show|give|ask|bring)\b|\bwait\s+in\b|\bbe\s+(?:in|there)\s+(?:for|all)\s+(?:the\s+)?(?:day|morning|afternoon)\b"##).expect("NOT_ABOUT_THE_VISIT")
});
pub(super) static NOT_ON_THE_PLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bservic(?:e[sd]?|ing)\b|\bhow\s+often\b|\brinse\s+aid\b|\bsalt\b|\bdescal\w*|\badditive\b|\bsoftener\b|\b(?:which|what)\s+filter\b|\bfilter\s+(?:does|should)\b|\btablets?\b|\bpowder\b|\bdetergent\b|\bhow\s+(?:do|should)\s+(?:i|we)\s+(?:use|load|set|run|fill)\b|\bshould\s+(?:i|we)\s+(?:use|put|add|fill|run)\b"##).expect("NOT_ON_THE_PLATE")
});
pub(super) static A_RECEIPT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:got|bought|ordered|received|arrived|came|delivered|picked\s+up|sent\s+(?:me|us)|turned\s+up|was\s+delivered|was\s+sent|it's\s+here|its\s+here|came\s+with)\b"##).expect("A_RECEIPT")
});
pub(super) static A_JOB_ON_IT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bput(?:ting)?\s+(?:\w+\s+){0,4}(?:up|together)\b|\bassembl\w*|\binstall\w*|\bbuild\w*|\berecting\b|\bfitting\b"##).expect("A_JOB_ON_IT")
});
pub(super) static DETERMINER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(my|the|our|your|this|that|a|an|his|her|their)\s+$"##).expect("DETERMINER")
});
pub(super) static A_KIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:shed|wardrobe|bookcase|bookcases|shelves|shelving|shelf|flat\s*pack|flatpack|kit|units?|desk|bed|bunk|trampoline|greenhouse|climbing\s+frame|table|chair|chairs|cabinet|bbq|barbecue|swing\s*set|playhouse|sideboard|tiles?|flooring|laminate|decking|fence|fencing|gate|sheds|curtain\s+pole|blinds?|radiator\s+cover|sofa|settee|couch|mattress|sofa\s+bed|armchair|dining\s+table)\b"##).expect("A_KIT")
});
pub(super) static A_FIXTURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:tap|taps|sink|sinks|shower|showers|toilet|loo|cistern|ballcock|bath|bathroom\s+suite|stopcock|soil\s+pipe|waste\s+pipe)\b"##).expect("A_FIXTURE")
});
pub(super) static AN_APPLIANCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:boiler|washing\s+machine|washer|dishwasher|fridge|freezer|oven|cooker|hob|microwave|kettle|toaster|hoover|vacuum|dryer|tumble\s+dryer|radiator|thermostat|heater|dehumidifier|mower|lawnmower|strimmer|hedge\s+trimmer|drill|printer|laptop|phone|tablet|console|telly|tv|television|router|doorbell|camera|battery|charger|remote)\b"##).expect("AN_APPLIANCE")
});
pub(super) static PAPERWORK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:letter|form|invoice|bill|statement|contract|policy|insurance|quote|receipt|reference|application|claim|tenancy|lease|mortgage|payslip|p60|p45|council\s+tax|licence|license|passport|visa|permit|booking|confirmation|delivery|order|parcel|refund|warranty|guarantee|tracking)\b"##).expect("PAPERWORK")
});
pub(super) static A_COMMITMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:gym\s+membership|phone\s+contract|mobile\s+contract|care\s+home|home\s+care|lease\s+car|car\s+finance|direct\s+debit|times\s+share|gym|membership|contract|subscription|tariff|tenancy|lease|timeshare|nursery|course|classes|insurance|policy|broadband|loan|finance|hp)\b"##).expect("A_COMMITMENT")
});
pub(super) static ASK_GET_OUT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bget\s+out\s+of\b|\bhow\s+(?:do|can|should)\s+(?:i|we)\s+(?:cancel|stop|end|leave|quit|exit)\b|\bcancel\s+(?:it|my|the|this|that|our)\b|\bstop\s+paying\b|\bend\s+(?:the|my|this|our)\b|\bgive\s+notice\b|\bopt\s+out\b|\bcome\s+out\s+of\b|\bbreak\s+(?:the|my)\b"##).expect("ASK_GET_OUT")
});
pub(super) static ASK_TOOLS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:what\s+tools?|what\s+(?:do|will|would|might)\s+(?:i|we|you)\s+need|tools?\s+(?:do|will)|what\s+do\s+i\s+need\s+(?:to|for)|what\s+will\s+i\s+need)\b"##).expect("ASK_TOOLS")
});
pub(super) static ASK_HOW_LONG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+long\s+(?:does|do|will|would|should|to|is|takes?)\b|\bhow\s+many\s+hours\b|\bhow\s+long\s+(?:a|an|one|it|they|that)\b"##).expect("ASK_HOW_LONG")
});
pub(super) static ASK_SIZE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwhat\s+size\b|\bwill\s+it\s+fit\b|\bhow\s+big\b|\bwhat\s+(?:kind|sort)\s+of\s+\w+\s+(?:do|will)\s+(?:i|we)\b|\bwhich\s+(?:one|size|model)\b"##).expect("ASK_SIZE")
});
pub(super) static ASK_BE_IN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bbe\s+in\s+for\s+it\b|\bneed\s+(?:to\s+)?be\s+(?:in|here|home)\b|\bsign\s+for\s+it\b|\b(?:does|will)\s+(?:it|someone|they)\s+need\s+to\s+be\s+in\b|\bwaiting\s+in\b"##).expect("ASK_BE_IN")
});
pub(super) static ASK_COVERED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:covered|cover\s+(?:it|this|that|me)|tell\s+the\s+insurance|insurance\s+(?:about|know)|claim\s+(?:on|for)|on\s+the\s+policy|included|comes?\s+with\s+it)\b"##).expect("ASK_COVERED")
});
pub(super) static ASK_EXACT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwhich\s+(?:model|part|spare|size|one|type|version)\b|\bwhat\s+(?:model|number)\b|\bpart\s+number\b|\bwhich\s+one\s+(?:do|should|will)\b"##).expect("ASK_EXACT")
});
pub(super) static ASK_SEND_BACK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bsend\s+(?:it|them|that|this|the\s+old\s+one)\s+(?:back|off|in)\b|\bsend\s+(?:it\s+|them\s+)?(?:back|off)\b|\b(?:do|must|should|need\s+to|have\s+to)\s+(?:i\s+)?(?:return|post|send)\b|\breturn\s+(?:it|them|the\s+old\s+one)\b"##).expect("ASK_SEND_BACK")
});
pub(super) fn kinds() -> [(&'static Regex, &'static str); 5] {
    [
        (&A_FIXTURE, "fixture"),
        (&A_KIT, "kit"),
        (&AN_APPLIANCE, "appliance"),
        (&PAPERWORK, "paperwork"),
        (&A_COMMITMENT, "commitment"),
    ]
}
pub(super) fn asks() -> [(&'static Regex, &'static str); 8] {
    [
        (&ASK_GET_OUT, "get_out"),
        (&ASK_TOOLS, "tools"),
        (&ASK_HOW_LONG, "how_long"),
        (&ASK_SIZE, "size"),
        (&ASK_BE_IN, "be_in"),
        (&ASK_COVERED, "covered"),
        (&ASK_SEND_BACK, "send_back"),
        (&ASK_EXACT, "exact"),
    ]
}
pub(super) fn carriers(ask: &str, kind: &str) -> Option<&'static [&'static str]> {
    match (ask, kind) {
        ("tools", "kit") => Some(&[
            "The instruction sheet in the box lists the tools and the fixings, and says which of them you can do without",
            "The parts list that came with it names the tools it wants, and the fixings are usually in the box with them",
        ]),
        ("tools", "appliance") => Some(&[
            "The installation page for your model lists what it needs, and the plate on the machine says which model yours is",
            "The maker's own page for that model names the tools and whether the fixings come with it",
        ]),
        ("how_long", "kit") => Some(&[
            "The instruction sheet gives the number of hours for two people, and the maker's page usually repeats it with the same figure",
            "The maker's page for it has the build time, and the instructions break that down into what has to be done in order",
        ]),
        ("how_long", "appliance") => Some(&[
            "The maker's page for your model says how long the job should take, and the plate on it tells you the model",
            "Whoever services that make will give you a time on the phone from the model number alone, which is on the plate",
        ]),
        ("how_long", "paperwork") => Some(&[
            "The reference on the paperwork is what a straight answer is given against, and the office named on it is the one that holds the times",
            "Whoever issued it can say where yours has got to from the reference - and the letter or the confirmation says who that is",
        ]),
        ("be_in", "kit") => Some(&[
            "The order confirmation and the tracking page say whether it needs a signature, and they are what count on the day",
            "Whoever is delivering it puts the answer on the tracking page, and the confirmation says what happens if nobody is in",
        ]),
        ("size", "kit") => Some(&[
            "The maker's page for each version gives the footprint and the clearance it needs, and the size chart is the part that settles which one fits",
            "The maker's page has the dimensions for each version, so the sheet and the page together settle which one fits your space",
        ]),
        ("size", "appliance") => Some(&[
            "The maker's page for your model has the dimensions, and the plate on it tells you the model",
            "The plate on it gives the model number, and the maker's page has the size for that exact one",
        ]),
        ("be_in", "paperwork") => Some(&[
            "The order confirmation or the tracking page says whether it needs a signature, and that is the one that counts on the day",
            "Whoever is delivering it puts the answer on the tracking page, and the confirmation says what happens if nobody is in",
        ]),
        ("covered", "paperwork") => Some(&[
            "The policy or the reference says: what is covered, what the excess is, and what they ask you to tell them, all in the same document",
            "The terms that came with it settle it, and a phone call quoting the reference gets a yes or no from the people who apply them",
        ]),
        ("send_back", "paperwork") => Some(&[
            "What it wants back, in what state and by when is on the page it came with, and the reference on it is how you check they have got it",
            "Whatever came with it sets out what to return and what to keep; the office named on the paper will confirm against the reference",
        ]),
        ("get_out", "commitment") => Some(&[
            "The way out is written in the terms you were given: the notice period, whether it has to be in writing, and the date it renews on - and a written notice sent to the address on that paperwork is what they have to answer",
            "It comes down to the wording you signed: how much notice it wants, whether it renews itself, and who the notice has to go to. Notice in writing, sent to the address on the paperwork, is the version that gets an answer",
        ]),
        ("exact", "appliance") => Some(&[
            "The plate on it - inside the door or on the back - gives the model and the part number, and that is what the maker's page is searched by",
            "The model and serial on the plate are what any parts page wants, and they make the answer exact rather than near",
        ]),
        ("first_step", "kit") => Some(&[
            "The instruction sheet in the box and the maker's page for it between them cover what it needs and what to do when it will not - and the sheet is the one that came with your copy",
            "Start with what came with it: the sheet and the parts list say what it should do and what is missing, and the maker's page for that exact one says the rest",
        ]),
        ("first_step", "appliance") => Some(&[
            "The plate on it - inside the door or on the back - gives the model and the serial, and the maker's page for that model has the fault codes, the parts and who services it",
            "Whatever the question is, the model number on the plate is the way in: the maker's page for it, or the engineer on the phone, will take that and work from there",
        ]),
        ("first_step", "fixture") => Some(&[
            "The part that goes on those is the cheap one - a washer, a cartridge, a seal or a valve, a few pounds rather than a new unit - and the make settles which: it is stamped on the body of it or under the cap, and the maker's page for that exact one has the part number and the video for the job",
            "Most of what goes wrong on one is a seal, a washer or a valve, and none of them is dear - take the make and model off it, on the body or under the handle, and the maker's page for that model shows the part and how the head comes off",
        ]),
        ("tools", "fixture") => Some(&[
            "It is a spanner job rather than a kit: the maker's page for the model names the sizes, and a plumber's merchant has the washer or the cartridge",
            "The fittings come with the unit, and the maker's page for that model says which sizes you need for it",
        ]),
        ("how_long", "fixture") => Some(&[
            "With the part in hand it is a short job - the maker's page for the model gives the time, and it is usually the inside of an hour for a washer or a cartridge",
            "The maker's page for that model says how long it takes, and most of that is the water off and the old part out",
        ]),
        _ => None,
    }
}
pub(super) fn question_kind(ask: &str, kind: &str) -> Option<&'static [&'static str]> {
    match (ask, kind) {
        ("first_step", "kit") => Some(&[
            "Have you got the sheet that came with it?",
            "Is the sheet it came with still about?",
        ]),
        ("tools", "appliance") => Some(&[
            "What does the plate say your model is?",
            "Do the fixings come with it or are they extra?",
        ]),
        ("first_step", "fixture") => {
            Some(&["Do you know the make of it?", "Can you see the make on it?"])
        }
        ("tools", "fixture") => {
            Some(&["Do you know the make of it?", "Is the make on it anywhere?"])
        }
        _ => None,
    }
}
pub(super) fn question(ask: &str) -> Option<&'static [&'static str]> {
    match ask {
        "tools" => Some(&[
            "What does your sheet list?",
            "What came in the box with it?",
        ]),
        "how_long" => Some(&[
            "What does it say on the sheet?",
            "Does the sheet give a time for the job?",
        ]),
        "size" => Some(&[
            "What have you got to fit it in?",
            "What do the dimensions say on yours?",
        ]),
        "be_in" => Some(&[
            "What does the tracking say?",
            "Did they give you a time slot?",
        ]),
        "covered" => Some(&[
            "What does the paperwork say it covers?",
            "Do you have the reference to hand?",
        ]),
        "send_back" => Some(&[
            "What does it ask you to send?",
            "Have you got the reference on it?",
        ]),
        "get_out" => Some(&[
            "How much notice does it ask for?",
            "Do you still have the terms it came with?",
        ]),
        "exact" => Some(&[
            "What is the model number on it?",
            "Can you read the plate on it?",
        ]),
        "first_step" => Some(&[
            "What does the plate say on yours?",
            "Have you got the sheet that came with it?",
        ]),
        _ => None,
    }
}
pub(super) const OPENER: &[&str] = &[
    "I can't see {S} from here, sorry",
    "I haven't got {S} in front of me, sorry",
    "There is no way for me to check {S} from here, sorry",
];
pub(super) const OPENER_TERMS: &[&str] = &[
    "I can't see the terms on {S} from here, sorry",
    "I haven't got the small print for {S}, sorry",
    "I can't check what {S} says about getting out, sorry",
];
pub(super) const THEIR_OWN: &[&str] = &[
    "her", "his", "my", "our", "that", "the", "their", "these", "this", "those", "your",
];
pub(super) const THEIR_COPY_ASKS: &[&str] = &["be_in", "exact", "send_back"];
