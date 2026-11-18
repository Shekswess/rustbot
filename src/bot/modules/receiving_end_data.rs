//! Authored repeated-role patterns and reply pools from PyBot 7d63c7f.

pub(super) const A_PERSON_WHO_KEEPS: &str = r#"(?i)\b(?:(?:my|our|his|her|their|the)\s+(?:new\s+|old\s+|current\s+|ex\s+)?)?(?:boss|manager|line\s+manager|team\s+lead|team\s+leader|team\s+manager|supervisor|shift\s+lead|foreman|colleague|co-?worker|teacher|head\s?teacher|tutor|lecturer|landlord|landlady|letting\s+agent|social\s+worker|social\s+services|housing\s+officer|housing\s+association|jobcentre|council|school|nursery|college|university|gp\s+surgery|doctors\s+surgery|surgery|practice|hr|payroll|recruiter|agency)\b"#;

pub(super) const KEEPS_DOING_IT: &str = r#"(?i)\b(?:keeps?|always|never|constantly|every\s+(?:single\s+)?(?:time|week|day|shift|month|term)|again\s+and\s+again|over\s+and\s+over|all\s+the\s+time|ignores?|ignored|picks?\s+on\s+me|undermines?\s+me|talks?\s+over\s+me|belittles?\s+me|gaslights?)\b"#;

pub(super) const A_COMPLAINT_WORD: &str = r#"(?i)\b(?:never|not|no|wont|won'?t|doesnt|doesn'?t|didnt|didn'?t|ignores?|ignored|forgets?|forgot|losing|loses|lost|lies|lying|lied|shouts?|shouted|yell\w*|blames?|blamed|criticis\w*|undermin\w*|gaslight\w*|cancel\w*|clos\w*|chang\w*|changes|moves?|moved|goalposts|overrules?|shuts? me down|snaps? at me|rolls? (?:his|her|their) eyes|leaves? me out|rules|last minute|wrong|unfair|credit|favourites?|piling|pile|micromanag\w*|hiding|refuses?|refused|picks on|picked on|talks over|talked over|belittles?|belittled|mess(?:ed)? up|muck\w*|stops? me|stopped me|keeps me|kept me|delay\w*|chase\w*|impossible|nightmare|stress\w*|awful|horrible|useless|lazy|rude|bully\w*|unprofessional)\b"#;

pub(super) const A_WARM_MEMORY: &str = r#"(?i)\b(?:always|used\s+to)\s+(?:said|told|taught|gave|made\s+me|let\s+me|wanted|encouraged)\b|\b(?:was|were)\s+(?:great|lovely|kind|good|brilliant|fair|helpful)\b"#;

pub(super) const A_BRUSH_OFF: &str = r#"(?i)\b(?:says?\s+(?:that\s+)?(?:its|it'?s|thats|that'?s)\s+(?:just\s+)?(?:how|the\s+way)\b|says?\s+(?:that\s+)?(?:its|it'?s|thats|that'?s)\s+life\b|shrugs?\s+(?:it\s+off|it|and)\b|says?\s+(?:to\s+)?(?:just\s+)?(?:get\s+on\s+with\s+it|put\s+up\s+with\s+it)|says?\s+(?:everyone|everybody|we\s+all)\b[^.?!]{0,40}?(?:put[s]?\s+up\s+with|has\s+to)|keeps?\s+saying\s+(?:we'?ll\s+see|next\s+(?:month|week|year)|soon|in\s+a\s+(?:bit|minute|while))|won'?t\s+even\s+(?:say|answer|reply|discuss))\b"#;

pub(super) const A_BODY_LINE: &str = r#"(?i)\b(?:knife|gun|knives|weapon|throat|strangl\w*|chok\w*|punch\w*|beat (?:me|him|her)|hit (?:me|him|her|my)|hits? me|hurt(?:ing)? me|hurts? me|threaten\w*|threats?|kill me|kill(?:ed)? (?:him|her|them)|bruis\w*|abus\w*|assault\w*|rape\w*|black eye|broken (?:arm|nose|ribs?))\b"#;

pub(super) const ROLE_SAID_BACK: &[(&str, &str)] = &[
    ("hr", "HR"),
    ("gp surgery", "the surgery"),
    ("doctors surgery", "the surgery"),
    ("surgery", "the surgery"),
    ("school", "the school"),
    ("nursery", "the nursery"),
    ("council", "the council"),
    ("university", "the university"),
    ("college", "the college"),
    ("jobcentre", "the jobcentre"),
];

pub(super) const THEIR: &[(&str, &str)] = &[
    ("my", "your"),
    ("our", "your"),
    ("his", "their"),
    ("her", "their"),
    ("their", "their"),
];

pub(super) const THE_BRUSH_OFF_REPLIES: &[&str] = &[
    "That is a brush-off rather than an answer. Nothing to look up here, but the move that works is a short email: say what they said, what you want changed, and ask for a reply by a date. If the date passes, that email is what you take one rung above them, or to ACAS if it is a workplace.",
    "When the answer is that is just how it is, ask once more in writing - what you want changed and by when - and keep the reply. That email is what the next rung reads: ACAS for a workplace, or the complaints procedure every school, council and surgery has to follow.",
    "Nothing to look up here, but do not leave it spoken: put the request in writing with a date on it and keep a copy. If {role} will not answer that, the next step is one rung above them, and after that the ombudsman, which is free to use and is where a case finally gets looked at.",
    "There is a shape to these answers and it is worth naming: ask for the reason in writing, once, with the date you want it by. A recorded refusal is something you can challenge, and the complaints procedure is the route - it has to end somewhere, usually with the ombudsman.",
];

pub(super) const PERSON_WHO_KEEPS_REPLIES: &[&str] = &[
    "Nothing in that to look up, but the step is the same as for any pattern like this: put it in writing once - the date, what happened, what you want - and keep a copy. If it does not come back in a week, it goes one rung above {role}, and ACAS gives free advice on the wording if it is a workplace.",
    "When it happens again and again the wording decides it: dates, what you asked for, what happened next. Send that once to {role} and ask for an answer by a date. If it does not come, ask for the written complaints or grievance procedure and follow the step it names.",
    "Keep a short record - the date, what happened, what you asked for. A record is what makes {role} have to answer, and it is what the next rung reads. Every school, council and GP surgery has a complaints procedure with named steps, and asking for a copy is the first one.",
    "That is a pattern rather than one bad day, so put it in writing rather than say it again: what you want changed, with dates. Nothing to look up here, but the complaints procedure is where this goes next - ask for a copy of it and follow the step it names.",
];

pub(super) const THREAD_TURNS: usize = 12;

pub(super) const THE_THREAD_KEPT: usize = 3;

pub(super) const LOSS_OPENINGS: &[&str] = &[
    "I'm so sorry to hear that. It'",
    "I'm really sorry. Be gentle wi",
    "That's such sad news, I'm sorr",
    "I'm sorry, that's a real loss.",
];

pub(super) const FILLERS: &[&str] = &[
    "actually",
    "anyway",
    "basically",
    "even",
    "exactly",
    "honestly",
    "just",
    "literally",
    "maybe",
    "perhaps",
    "please",
    "really",
    "still",
];
