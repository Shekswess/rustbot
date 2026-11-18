//! Wording, organization names and patterns from the pinned chasing_a_body module.
use fancy_regex::Regex;
use std::sync::LazyLock;
#[rustfmt::skip]
pub(super) const A_PERSON_AFTER_THE_NAME: &[&str] = &["aunt", "bloke", "boss", "boyfriend", "brother", "colleague", "cousin", "dad", "daughter", "father", "fella", "fiance", "fiancee", "flatmate", "friend", "girl", "girlfriend", "husband", "lad", "lodger", "mam", "mate", "mom", "mother", "mum", "neighbor", "neighbour", "nephew", "niece", "partner", "roommate", "sister", "son", "stepdaughter", "stepson", "uncle", "wife", "workmate"];
pub(super) static NOT_DONE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:havent|hasnt|hadnt|ha(?:ve|s|d)\s+not|didnt|did\s+not|not\s+(?:yet\s+)?(?:replied|heard|fixed|paid|sent|done|come|turned\s+up)|no\s+(?:reply|response|answer|word)\b|still\s+(?:waiting|no|nothing)|nothing\s+(?:back|yet|since|at\s+all)|ignoring|ignored|wont|won't|refuse[sd]?|too\s+long|weeks\s+now|months\s+now|and\s+nothing\b|heard\s+nothing|nothing\s+(?:ever\s+)?came\s+back|never\s+(?:did|bothered|rang|replied|came|got\s+back|gave|sent|turned\s+up)|never\s+(?:heard|came)\s+back|no\s+one\s+(?:(?:has|had)\s+)?(?:got|gotten|come)\s+back|nobody\s+(?:(?:has|had)\s+)?(?:got|gotten|come)\s+back|has\s+not\s+come\s+back|have\s+not\s+come\s+back|(?:nobody|no\s+one)\s+(?:\w+\s+){0,2}(?:rang|rung|phoned|called|replied|answered|bothered))"##).unwrap()
});
pub(super) static HOW_TO_CHASE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:chase|chasing|escalate|escalation|ombudsman|complain|complaint|follow\s+(?:it\s+)?up|how\s+long\s+(?:should|do|before|until|to\s+wait|till)|who\s+(?:do\s+i|should\s+i|can\s+i)\s+(?:ask|ring|call|contact|chase|escalate)|what\s+(?:can|do|should)\s+i\s+do\s+(?:about|now)|take\s+(?:it|this)\s+further)\b"##).unwrap()
});
#[rustfmt::skip]
pub(super) const THE_SHAPE: &[&str] = &["Have {reference} in front of you, and put the ask in writing once: what has not happened, when you first raised it, and the day you want an answer by. {body}'s complaints page names the timescale and the next stage, usually an ombudsman after eight weeks, and that page is the part I cannot see.", "First get {reference} together, then write once, with what has not happened, the date you first raised it and the day you need to hear by. {body}'s own complaints page has the timescale and the escalation route, and what it says for your case is the part I cannot see.", "Start with {reference}, then write once: what has not happened, when you first raised it, and the day you need to hear by. {body}'s complaints page says how long they take and where it goes next, and that and your letter are what I cannot see from here."];
pub(super) static RAISED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:complain(?:ed|ing|t)?|letters?|form|emails?|emailed|wrote|written|write|applied|application|claim(?:ed|s)?|asked|rang|phoned|called|contacted|chased|reported|requested|sent|posted|appealed|emailed\s+them|in\s+touch|heard\s+from\s+them|repairs?|refunds?|payments?|booking|appointment|orders?|delivery|deposits?|quote|invoices?|reimburs\w*)"##).unwrap()
});
#[rustfmt::skip]
pub(super) const THE_SHAPE_UNNAMED: &[&str] = &["Write once with the reference they gave you, what has not happened, the date you first raised it and the day you need to hear by, and keep a copy. Past that day a complaint goes to an ombudsman, usually after eight weeks, and their own complaints page has the timescale - the part I cannot see from here.", "The letter only needs the reference, what has not happened, the date you first asked and the day you want an answer by. That copy is what makes the next step easy: their complaints page names the timescale and who takes it on after it, and I cannot see that page from here.", "Put it in writing once, with the reference, the dates and the day you need to hear by. Complaints usually go on to an ombudsman after eight weeks if nothing comes back, and the page that says how long they allow for this case is the one I cannot see."];
pub(super) static IVE_TRIED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:i(?:ve|\s+have)\s+(?:rung|rang|called|phoned|emailed|written|been\s+in|been\s+down|asked|chased|tried)|(?:rung|rang|called|phoned|emailed)\s+(?:them|him|her|the\s+\w+)|no\s+one\s+(?:answers|picks\s+up|rings\s+back)|keeps?\s+passing\s+me\s+(?:round|around))"##).unwrap()
});
#[rustfmt::skip]
pub(super) const IVE_TRIED_LEAD: &[&str] = &["A call leaves nothing they have to answer, so the step that works now is one line in writing.", "Phoning again will get the same answer; what counts is the ask in writing.", "That is the point to stop ringing and put it in writing, because a call is not a record."];
pub(super) static BARE_CONTINUATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:so|ok|okay|and|but|right|well)?\s*(?:what\s+now|what\s+next|now\s+what|what\s+do\s+i\s+do(?:\s+now)?|where\s+do\s+i\s+start|how\s+do\s+i\s+start)\s*[?.!]*$"##).unwrap()
});
pub(super) static THE_TIMESCALE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow\s+long\b|\bhow\s+many\s+(?:weeks|days|months|working\s+days)\b|\bhow\s+soon\b|\bwhen\s+should\s+i\s+(?:chase|escalate|complain|write)\b|\bwait\s+(?:before|until)\b|\bhow\s+much\s+longer\b"##).unwrap()
});
#[rustfmt::skip]
pub(super) const THE_TIMESCALE: &[&str] = &["Eight weeks is the usual marker: until then {body} works to its own published timescale, and after it a complaint can go to the ombudsman. Put the day you need an answer by in the letter, keep the copy, and the date you wrote down is what makes it their deadline.", "Most schemes give {body} eight weeks to answer before a complaint can be escalated to an ombudsman, and its own page has the shorter figure it works to day to day. Your letter is what starts the clock, so give a date and keep a copy of it."];
#[rustfmt::skip]
pub(super) const THE_NEXT_STEP: &[&str] = &["Then it is two short paragraphs: what has not happened, the date you first raised it and the day you want an answer by, sent to the address on their letter with {reference} in it so there is a copy. If the date passes or the reply is a no, {body}'s complaints page names the next stage, which is usually an ombudsman after eight weeks.", "Put {reference} at the top, say in two or three lines what has not happened and when you first asked, and ask for an answer by a named day. Keep the copy: {body}'s own page has the timescale, and the letter it sends back names who takes it on if they miss it.", "Write it once, with {reference}, the date you first raised it and the day you need to hear by, and keep a copy. {body} publishes how long it should take and where a complaint goes next after that, and the date you put in the letter is what makes it their deadline."];
#[rustfmt::skip]
pub(super) const THE_NEXT_STEP_UNNAMED: &[&str] = &["Then write it once: the reference they gave you, what has not happened, the date you first raised it and the day you need to hear by, and keep a copy of it. Past that day a complaint usually goes on to an ombudsman after eight weeks, and their own page has the timescale for your case - the part I cannot see.", "One short letter: the reference, what has not happened, the date you first asked and the day you want an answer by, with a copy kept. Their complaints page says how long they allow and who takes it on after that, and your copy is what proves the date you gave them."];
pub(super) static A_PERSON_OWES_A_REPLY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:my|our)\s+(?:\w+\s+){0,1}(?:best\s+|old\s+|oldest\s+|closest\s+|own\s+|little\s+|big\s+|eldest\s+|younger\s+|school\s+|uni\s+|work\s+|childhood\s+)*(?:friends?|mates?|brothers?|sisters?|mum|mom|mam|mother|dad|father|sons?|daughters?|husbands?|wifes?|wives|partners?|boyfriends?|girlfriends?|cousins?|uncle|aunt|nephews?|nieces?|flatmates?|housemates?|roommates?|colleagues?|workmates?|neighbours?|neighbors?|ex|ex-?(?:wife|husband|partner|boyfriend|girlfriend))\b[^.?!]{0,40}?\b(?:hasnt|havent|hadnt|has\s+not|have\s+not|didnt|did\s+not|doesnt|does\s+not|isnt|isn't|is\s+not|wasnt|wasn't|was\s+not|never|wont|won't|refuses?\s+to)\s+(?:\w+\s+){0,2}(?:text(?:s|ed|ing)?|messag(?:e|es|ed|ing)|call(?:s|ed|ing)?|ring(?:s|ing)?|rang|rung|repl(?:y|ies|ied|ying)|answer(?:s|ed|ing)?|respond(?:s|ed|ing)?|pick(?:s|ed|ing)?\s+up|ignor(?:e|es|ed|ing)|open(?:s|ed|ing)?|writ(?:e|es|ten|ing)?|wrote|(?:be|been)\s+in\s+touch|(?:come|came|get|gets|getting|got)\s+back)\b(?:\s+(?:me|us|back)){0,2}"##).unwrap()
});
pub(super) static A_PRONOUN_OWES_A_REPLY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)^\W*(?:(?:so|and|but|then|ok|okay|well)\s+)*(?:(?:he|she|they)(?:s|'s|re|'re|ve|'ve)?)\b[^.?!]{0,30}?\b(?:(?:hasnt|havent|hadnt|has\s+not|have\s+not|didnt|did\s+not|doesnt|does\s+not|isnt|isn't|is\s+not|wasnt|wasn't|was\s+not|never|wont|won't|refuses?\s+to)|\bnot)\s+(?:\w+\s+){0,2}(?:text(?:s|ed|ing)?|messag(?:e|es|ed|ing)|call(?:s|ed|ing)?|ring(?:s|ing)?|rang|rung|repl(?:y|ies|ied|ying)|answer(?:s|ed|ing)?|respond(?:s|ed|ing)?|pick(?:s|ed|ing)?\s+up|ignor(?:e|es|ed|ing)|open(?:s|ed|ing)?|writ(?:e|es|ten|ing)?|wrote|(?:be|been)\s+in\s+touch|(?:come|came|get|gets|getting|got)\s+back)\b(?:\s+(?:me|us|back)){0,2}"##).unwrap()
});
pub(super) static A_PRONOUN_DID_THE_SILENT_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)^\W*(?:(?:so|and|but|then|ok|okay|well)\s+)*(?:(?:he|she|they)(?:s|'s|re|'re|ve|'ve)?)\b[^.?!]{0,30}?\b(?:ignor(?:ed|es)|ghost(?:ed|s)|blank(?:ed|s)\s+me|left\s+me\s+on\s+read|gave\s+me\s+the\s+silent\s+treatment)\b"##).unwrap()
});
pub(super) static ANSWERING_SOMETHING_ELSE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:the\s+)?(?:door|doorbell|buzzer|intercom|gate)\b"##).unwrap()
});
pub(super) static A_PERSON_DID_THE_SILENT_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:my|our)\s+(?:\w+\s+){0,1}(?:best\s+|old\s+|oldest\s+|closest\s+|own\s+|little\s+|big\s+|eldest\s+|younger\s+|school\s+|uni\s+|work\s+|childhood\s+)*(?:friends?|mates?|brothers?|sisters?|mum|mom|mam|mother|dad|father|sons?|daughters?|husbands?|wifes?|wives|partners?|boyfriends?|girlfriends?|cousins?|uncle|aunt|nephews?|nieces?|flatmates?|housemates?|roommates?|colleagues?|workmates?|neighbours?|neighbors?|ex|ex-?(?:wife|husband|partner|boyfriend|girlfriend))\b[^.?!]{0,30}?\b(?:ignor(?:ed|es)|ghost(?:ed|s)|blank(?:ed|s)\s+me|left\s+me\s+on\s+read|gave\s+me\s+the\s+silent\s+treatment)\b"##).unwrap()
});
#[rustfmt::skip]
pub(super) const THE_WAIT_FOR_A_PERSON: &[&str] = &["No word back, and the waiting is the worst of it. One plain message - \"no rush, just checking you're ok\" - beats three follow-ups, and it is easy to answer, which matters more than saying it perfectly.", "A silence like that does the talking on its own, so it is worth one clear message rather than guessing: say you would like to hear from them and that there is no pressure, then leave the ball with them. A first nudge reads as care and a third reads as a chase.", "Waiting on somebody to answer is its own kind of tired. Send one message that is easy to answer - no list of complaints, no \"we need to talk\" - and then let a day or two pass before you read anything into it: people go quiet for reasons that have nothing to do with you."];
pub(super) static A_PERSON_WHO_WORKS_THERE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:my|our)\s+(?:\w+\s+){0,1}(?:best\s+|old\s+|oldest\s+|closest\s+|own\s+|little\s+|big\s+|eldest\s+|younger\s+|school\s+|uni\s+|work\s+|childhood\s+)*(?:friends?|mates?|brothers?|sisters?|mum|mom|mam|mother|dad|father|sons?|daughters?|husbands?|wifes?|wives|partners?|boyfriends?|girlfriends?|cousins?|uncle|aunt|nephews?|nieces?|flatmates?|housemates?|roommates?|colleagues?|workmates?|neighbours?|neighbors?|ex|ex-?(?:wife|husband|partner|boyfriend|girlfriend))\b[^.?!]{0,25}?\b(?:works?|working|worked|is\s+employed|has\s+a\s+job|got\s+a\s+job|does\s+a\s+job)\s+(?:for|at|in|with)\b"##).unwrap()
});
pub(super) static A_PERSON_NOT_A_BODY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:brother|sister|mum|mom|mam|dad|father|mother|son|daughter|wife|husband|partner|friend|mate|cousin|uncle|aunt|nephew|niece|boss|colleague|neighbour|neighbor|stepson|stepdaughter|flatmate|roommate|landlady|lad)\b"##).unwrap()
});
pub(super) static OWN_PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:my|our)\s+(?:best\s+|old\s+|oldest\s+|closest\s+|own\s+|little\s+|big\s+|eldest\s+|younger\s+|school\s+|uni\s+|work\s+|childhood\s+)*(?:friends?|mates?|brothers?|sisters?|mum|mom|mam|mother|dad|father|sons?|daughters?|husbands?|wifes?|wives|partners?|boyfriends?|girlfriends?|cousins?|uncle|aunt|nephews?|nieces?|flatmates?|housemates?|roommates?|colleagues?|workmates?|neighbours?|neighbors?|ex|ex-?(?:wife|husband|partner|boyfriend|girlfriend))\b"##).unwrap()
});
#[rustfmt::skip]
pub(super) const ORGANISATIONS : &[(&[&str],&str,&str)] = &[
(&["council tax"],"the council tax","the account number on the bill"),
(&["council", "local authority", "town hall"],"the council","the reference on their letter"),
(&["housing association", "housing office", "housing trust"],"the housing association","their repair or complaint reference"),
(&["landlord", "letting agent", "landlady"],"the landlord","the date you reported it and your photos"),
(&["dwp", "jobcentre", "job centre", "universal credit", "benefits office"],"the DWP","the reference on the decision letter"),
(&["hmrc", "tax office", "inland revenue"],"HMRC","your taxpayer reference and the tax year"),
(&["gp", "doctor", "surgery", "health centre", "dentist"],"the surgery","the practice manager's name and the date you first asked"),
(&["hospital", "nhs trust", "clinic"],"the hospital","your appointment or referral reference"),
(&["energy supplier", "electricity", "gas company", "water company", "water board"],"the supplier","your account number and the date of the bill"),
(&["bank", "building society"],"the bank","the complaint reference they gave you"),
(&["employer", "payroll", "hr"],"your employer","your payslip dates and the hours you were short"),
(&["school", "head teacher", "headteacher", "academy trust"],"the school","the date you raised it and their letter"),
(&["university", "college", "student finance"],"the university","your student number and the decision reference"),
(&["dvla", "passport office", "home office", "ukvi"],"the office","the application reference on your receipt"),
(&["parcel", "delivery", "courier", "postage", "the post", "my order", "the order"],"the seller","the order number and the delivery date they gave you"),
];
