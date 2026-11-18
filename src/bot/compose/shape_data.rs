//! Patterns and wording from the pinned shape composer and its thread readers.
use fancy_regex::Regex;
use std::sync::LazyLock;
#[rustfmt::skip]
pub(super) const PICK_LINES: &[&str] = &["I can't pick for you, but two questions settle most choices like this: which one you would still choose in a year, and which is cheaper to undo if it turns out wrong. Which way are you leaning?", "There is no right answer I can point you at. What every pick like this comes down to is what you will use most, and whether the cheaper one is good enough for that. What are you down to choosing between?", "Two ways to choose: the one that is best at the main thing, or the one that is decent at everything. Most people regret the second. What would it have to be good at?"];
#[rustfmt::skip]
pub(super) const PICK_LINES_NAMED: &[&str] = &["I can't pick for you, but two questions settle most choices like this: which one you would still choose in a year, and which is cheaper to undo if it turns out wrong. Which way are you leaning?", "Two ways to choose: the one that is best at the main thing, or the one that is decent at everything. Most people regret the second. What would it have to be good at?"];
#[rustfmt::skip]
pub(super) const OPTIONS_LINES: &[&str] = &["I have no list to hand you, and I would rather not invent one. What makes a shortlist worth anything is that the choices are actually different: one cheap, one that lasts, one that is least effort. Which of those three are you after?", "Nothing in front of me to suggest from. Tell me the two or three things that are fixed - where, when, how much - and I can say what sort of option fits them. Which is the one that cannot move?", "I can't give you a list without making it up. The useful question is what you would say no to: once that is settled there are usually only two or three options left. What is a no?"];
#[rustfmt::skip]
pub(super) const AMOUNT_LINES: &[&str] = &["I haven't got a figure for that, and guessing one would be worse than none. A number like this is worked out from two or three others rather than looked up - give me those and I will do the sums with you. What have you got to go on?", "That is arithmetic more than a fact. Tell me the size of it, how many there are, and what it is per one, and I will work the total out with the steps shown so you can check them. Which of those do you have?", "No number from me without the figures it comes from - that is what stops it being a guess. What numbers do you already have?"];
#[rustfmt::skip]
pub(super) const TIME_LINES: &[&str] = &["I don't know the size of the job, so any time I gave you would be a guess. Most things like this are minutes for the small version and a day or more for the big one, and which one you have is the whole question. How big is it?", "That depends on what somebody finds once they start, which is why nobody will give you a firm number before they look. Ask for the usual time and add half again. Who is doing it?", "No honest number from me without knowing the size and who does it. If it is the simple version it is quick; if anything has to be taken out first it is not. What has to happen?"];
#[rustfmt::skip]
pub(super) const YESNO_LINES: &[&str] = &["I can't say yes or no to that honestly. The test I would use is the worst that happens if it goes wrong, and how easily you could undo it. If both are small, it is worth a try - how bad is the worst case?", "That is yours to weigh, and I don't have the details to weigh it for you. It comes down to what you would lose, how likely that is, and whether you could put it back. What would you lose at worst?", "No verdict from me on this one - I would be guessing. Say what would make you change your mind, and if nothing would, that is your answer already. What would change it?"];
#[rustfmt::skip]
pub(super) const REASON_LINES: &[&str] = &["I don't know this one well enough to say why, and a made-up reason is worse than none. Cases like it usually have two or three causes, and the way to tell which is yours is what else happens at the same moment. What else have you noticed?", "No cause I could name honestly. The reliable way to find one is to change a single thing at a time and see whether it goes away - two changes at once tells you nothing. What have you tried?", "I would be guessing at the reason. Whatever it is, it will have a before and after: what was different the first time it happened?"];
#[rustfmt::skip]
pub(super) const ADVICE_LINES: &[&str] = &["I can't tell you what to do here - I haven't got the details, and it is yours to decide. Which is the option you keep coming back to? I will argue the other side properly.", "No advice worth the name from me on this. The question that usually settles it is the one you would regret not asking. Which is it?", "I don't know enough to say what is right. Put it into one sentence - the choice, and what it costs - and I will help you look at it from the outside. What is the choice?"];
#[rustfmt::skip]
pub(super) const VAGUE_LINES: &[&str] = &["Give me a little more and I'll answer that part properly - what are you after?", "Happy to keep going - which part do you want next?", "That one is too open for me to answer well, and a guess would waste your time. Which bit matters most?", "One more line and I'll have something useful for you. What are you after now?"];
pub(super) static A_VAGUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:and|so|ok|okay|right|well|hmm|um)?\s*,?\s*(?:now\s+what|what\s+now|now)\b[\s?!.]*$|^what should i (?:think about|be doing(?: now)?|do next|do first)\b[\s?!.]*$|^what do i do now\b[\s?!.]*$"##).expect("_A_VAGUE")
});
pub(super) static WHAT_TO_DO_WITH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwhat (?:should|shall|do|would|can|will) (?:i|we) (?:do|use|put)\b[^?.!]{0,20}?\b(?:with|about)\b"##).expect("_WHAT_TO_DO_WITH")
});
pub(super) static A_WHAT_WAS_IT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^\s*(?:and\s+|so\s+)?(?:what|which)\s+(?:was|is|were|are)?\s*(?:the\s+)?(?:year|one|name|number|date|day|time|age|colour|color|brand|make|model|price|word|thing)\b|\bwhat\b[^?.!]{0,24}?\b(?:did|do) (?:i|we) (?:say|tell|mention)\b|\bremind me\b|\bwhat did (?:i|we|he|she|they|you) (?:say|tell|decide|choose|pick)\b|\b(?:what|which)\b[^?.!]{0,30}?\bagain\b"##).expect("_A_WHAT_WAS_IT")
});
pub(super) static A_THEY_SAID_SO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:i|we)\s+(?:say|said|say\s+it|told\s+you|mention|mentioned)\b|\bagain\b|\bremind\s+me\b"##).expect("_A_THEY_SAID_SO")
});
pub(super) static A_FREQUENCY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow (?:many|often)\b[^?.!]{0,24}?\b(?:days|nights|times|weeks|months|years)\b[^?.!]{0,12}?\b(?:a|per|each)\b"##).expect("_A_FREQUENCY")
});
pub(super) static AN_ASK_AT_THE_FRONT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:ok\s+|okay\s+|and\s+|so\s+|but\s+|also\s+)?(?:which|what|should|shall|would|could|can|do|did|is|are|am|i|we|im|i'm|were|we're|cant|can't|help|how|any|got)\b"##).expect("_AN_ASK_AT_THE_FRONT")
});
pub(super) static WHICH_I_NEED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwhich\b[^?.!]{0,24}?\b(?:do|would|will|should|shall)\s+(?:i|we)\s+(?:need|use|get for|ask for|call)\b"##).expect("_WHICH_I_NEED")
});
#[rustfmt::skip]
pub(super) const RECALL_LINES: &[&str] = &["That is one for between us rather than a search - say it once more and I will answer from what you tell me. What was it?", "I would rather have it from you than guess: say it again and I will hold on to it. Which one was it?", "Say it once more and I will keep it this time, then answer straight from that. What was it?"];
#[rustfmt::skip]
pub(super) const PAPERWORK_LINES: &[&str] = &["That one is on the paperwork, not out where I can see it - the renewal date sits near the policy number on the letter or in your account online. What does it say?", "The date is in their hands rather than mine: it is on the notice they last sent you, beside the reference number. Have you still got it?", "I can't see your own documents, so the date is on the letter or in the app under the plan. What date does it give?"];
#[rustfmt::skip]
pub(super) const LINES: &[&str] = &["pick", "options", "amount", "time", "yesno", "vague", "recall", "paperwork", "reason", "advice"];
pub(super) static HEAVY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:died|dying|death|funeral|cancer|hospice|hospital|miscarr\w*|suicide|kill myself|abuse|assault|overdose|relapse|diagnos\w*|terminal)\b"##).expect("_HEAVY")
});
pub(super) static TWO_THINGS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(\w[\w'-]*)\W{0,2}\s+or\s+\W{0,2}(\w[\w'-]*)"##).expect("_TWO_THINGS")
});
#[rustfmt::skip]
pub(super) const NOT_AN_OPTION: &[&str] = &["and", "are", "but", "can", "could", "did", "do", "does", "get", "go", "going", "got", "had", "has", "have", "he", "her", "him", "i", "if", "is", "it", "make", "may", "me", "might", "my", "no", "not", "our", "put", "shall", "she", "should", "so", "take", "that", "their", "them", "then", "there", "these", "they", "this", "those", "us", "was", "we", "were", "will", "would", "yes", "you", "your"];
pub(super) static HOW_MANY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:and\s+|so\s+|but\s+)?how\s+(?:many|much)\s+(?:\w+\s+){0,2}?(\w{3,})"##)
        .expect("_HOW_MANY")
});
#[rustfmt::skip]
pub(super) const NOT_THE_THING: &[&str] = &["bad", "big", "early", "far", "good", "late", "long", "many", "more", "much", "new", "often", "old", "quick", "soon", "time", "worth"];
pub(super) static A_TIME_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:how\s+far\s+ahead|how\s+soon|how\s+long|how\s+quick|when\s+should|what\s+time)\b"##).expect("_A_TIME_WORD")
});
pub(super) static A_FACT_ABOUT_TIME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwhen (?:does|do|did|will) (?:the|it|they|this|that|a|my|his|her|your)\b|\bwhen (?:does|do|did|will)\s+(?!i\b|we\b|you\b)\w|\bwhen (?:does|do) it\b|\bwhat time (?:does|do|is|are|will)\b|\bhow far (?:ahead|in advance)\b|\bhow long (?:before|in advance)\b|\b(?:should|do|can) i (?:need to )?book\b"##).expect("_A_FACT_ABOUT_TIME")
});
pub(super) static A_STORED_COUNT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow many (?:episodes|seasons|chapter|chapters|pages|tracks|songs|albums|volumes|parts|instalments|films|series|books|tablets|doses|rounds)\b|\bhow many \w+ (?:are|were) there\b"##).expect("_A_STORED_COUNT")
});
pub(super) static A_FIT_STATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:ok|okay|alright|all\s+right|fine|safe|sound|sound\s+enough|still\s+good|still\s+ok|still\s+fine|gone\s+off|gone\s+bad|in\s+date|off|spoiled|spoilt|edible|contaminated|damaged|broken|worn|leaking)\b"##).expect("_A_FIT_STATE")
});
pub(super) static A_POINTS_AT_A_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:it|that|this|those|these|they|them|the|mine|ours)\b"##)
        .expect("_A_POINTS_AT_A_THING")
});
pub(super) static A_VERDICT_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:worth|enough|too\s+many|too\s+much|better|worse|help|helps|work|works|risk|safe|okay|ok|fine|point|bother|afford|matter|matters)\b"##).expect("_A_VERDICT_WORD")
});
pub(super) static A_DECISION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:should\s+(?:i|we)|shall\s+(?:i|we)|is\s+it\s+worth|worth\s+it|worth|do\s+you\s+think|would\s+(?:i|we)|am\s+i\s+better|enough|too\s+much|too\s+many|better\s+off|bother|afford|for\s+that|(?:would|will|does|do|did|could|can)\s+(?:a|an|another|one|two|the)\b[^?.!]{0,25}?\b(?:help|be\s+(?:better|good|worth))\b)"##).expect("_A_DECISION")
});
pub(super) static A_WHOSE_THING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\b(?:my|our|mine|his|her|their)\b"##).expect("_A_WHOSE_THING"));
pub(super) static A_DATE_ON_IT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:when|what\s+date)\b[^?.!]{0,30}?\b(?:run\s+out|runs\s+out|expire|expires|renew|renews|renewal|end|ends|due|give\s+notice|notice\s+period)\b|\b(?:when|what\s+date)\b[^?.!]{0,24}?\b(?:do|should)\s+(?:i|we)\s+(?:need\s+to\s+)?renew\b"##).expect("_A_DATE_ON_IT")
});
pub(super) static THEIR_THINGS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:polic(?:y|ies)|insurance|contract|warranty|guarantee|mot|lease|tenancy|licence|license|passport|membership|subscription|pass|agreement|cover|plan|booking|reservation|permit|filing|return)\b"##).expect("_THEIR_THINGS")
});
pub(super) static AN_ENOUGH_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:is|are|will|would|do|does|did)\b[^?.!]{0,15}?\benough\b"##)
        .expect("_AN_ENOUGH_ASK")
});
pub(super) static A_PICK_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:ok\s+|and\s+|so\s+)?(?:what|which)\s+(?:should|shall|do|would|can)\s+(?:i|we)\s+(?:go\s+for|pick|choose|get|buy|start\s+with|have)\b"##).expect("_A_PICK_ASK")
});
pub(super) static A_STRAIGHT_YESNO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:ok\s+|and\s+|so\s+)?(?:should|shall|do|does|did|would|will|is|are|can|could)\s+(?:i|we|it|that|this)\b"##).expect("_A_STRAIGHT_YESNO")
});
pub(super) static A_WORK_OUT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bhow (?:do|does|would|can|could|shall) (?:i|we)\b[^?.!]{0,30}?\b(?:work|figure|sort)\s+out\b|\bhow (?:do|would|can|could) (?:i|we) (?:know|tell|estimate|budget)\b[^?.!]{0,30}?\b(?:much|many|cost|costs|afford|price|budget|total)\b"##).expect("_A_WORK_OUT")
});
pub(super) static ON_PAPER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:polic(?:y|ies)|renewal|insurance|contract|agreement|licence|license|passport|warrant(?:y|ies)|booking|ticket|statement|invoice|receipt|letter|form|certificate|account|mortgage|tenancy|lease|quote|registration|mot|v5c)\b"##).expect("_ON_PAPER")
});
pub(super) static BARE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:ok\s+|and\s+|so\s+)?(?:what|which|where|whats|what's)\s+(?:should|shall|do|can|could|would)\s+(?:i|we)\s+(?:visit|see|do|go|eat|drink|get|buy|put|pick|choose|start|try|book)\b"##).expect("_BARE_ASK")
});
pub(super) static LEANS_ON_THE_THREAD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:and\s+|so\s+|but\s+|ok\s+)?(?:does|do|did|is|are|was|were|will|would|can|could|should)\s+(?:that|this|it|they|those|these)\b"##).expect("_LEANS_ON_THE_THREAD")
});
pub(super) static BODY_WORD_FOR_A_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:back|shoulders?|arms?|foot|feet|head|chest|hips?|ribs?|spine|belly|knees?|neck|hands?|legs?|elbows?|waist)\s+(?:doors?|gardens?|yards?|walls?|rooms?|gates?|fences?|windows?|lawn|beds?|paths?|steps?|porch|seats?|lights?|lanes?|roads?|alleys?|shed|deck|patio|fields?|cars?|wheels?|sides?|covers?|pages?|pockets?|boards?|racks?|rest|cushions?|chairs?|tables?|freezer|drawers?|cupboards?|wardrobes?|stairs?|staircase|lines?|notes?|rows?|bands?|straps?|ties?|muffs?|warmers?|of\s+the\s+(?:stairs|bed|sofa|room|house|boat|car|garden))\b"##).expect("BODY_WORD_FOR_A_THING")
});
pub(super) static FORECAST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bwill\s+(?:it|that|this|they|he|she|i|we|my\s+\w+|the\s+\w+)\b[^?.!]{0,40}?\b(?:get\s+better|get\s+worse|be\s+(?:ok|okay|all\s+right|alright|fine)|\s*come\s+back|come\s+right|come\s+down|go\s+(?:away|down)|recover|heal|clear\s+up|ease|\s*settle|pass|stop|wear\s+off|sort\s+(?:itself|themselves)\s+out|\s*pull\s+through|make\s+it)\b|\b(?:is|are|will)\s+(?:he|she|it|that|this|they|my\s+\w+)\b[^?.!]{0,30}?\b(?:going\s+to|gonna)\s+(?:be\b|get\s+(?:better|worse)|heal|recover|\s*clear\s+up|come\s+(?:back|right)|settle|ease|\s*make\s+it|pull\s+through)\b|\bwill\s+(?:he|she|they|my\s+\w+|the\s+\w+)\b[^?.!]{0,20}?\bbe\s+(?:out|home|back|walking|mobile|up\s+and\s+about|better|on\s+the\s+mend)\b\s*(?:by|before|for|on|in\s+time\s+for|this|next)\b"##).expect("FORECAST")
});
#[rustfmt::skip]
pub(super) const CARE_MODULES: &[&str] = &["grief", "loss", "safety", "urgent"];
#[rustfmt::skip]
pub(super) const QUIET_MODULES: &[&str] = &["chat", "greeting"];
#[rustfmt::skip]
pub(super) const CRISIS_WORDS: &[&str] = &["116 123", "988", "crisis", "helpline", "samaritans", "stay with you"];
#[rustfmt::skip]
pub(super) const SYMPTOM_WORDS: &[&str] = &["ache", "aches", "aching", "bled", "bleed", "bleeding", "breath", "breathe", "breathing", "bruise", "bruised", "confused", "cough", "coughing", "cramps", "dizziness", "dizzy", "exhausted", "fever", "forgetful", "forgetting", "headache", "headaches", "hurt", "hurting", "hurts", "infection", "lump", "lumps", "memory", "migraine", "migraines", "nausea", "nauseous", "pain", "painful", "pains", "palpitations", "pregnant", "rash", "sick", "sore", "swelling", "swollen", "temperature", "tired", "vomit", "vomiting", "wheezing"];
#[rustfmt::skip]
pub(super) const BODY_PARTS: &[&str] = &["ankle", "ankles", "arm", "arms", "back", "belly", "chest", "ear", "ears", "eye", "eyes", "feet", "finger", "fingers", "foot", "gum", "gums", "hand", "hands", "head", "hip", "hips", "jaw", "knee", "knees", "leg", "legs", "neck", "shoulder", "shoulders", "sinus", "sinuses", "skin", "stomach", "teeth", "throat", "toe", "toes", "tooth", "wrist", "wrists"];
#[rustfmt::skip]
pub(super) const HEALTH_WORDS: &[&str] = &["111", "999", "a&e", "allergic", "allergy", "antibiotic", "antibiotics", "antihistamine", "antihistamines", "aspirin", "casualty", "clinic", "doctor", "doctors", "dosage", "dose", "flu", "gp", "gps", "hospital", "ibuprofen", "infection", "inhaler", "insulin", "jab", "jabs", "medication", "medicine", "nurse", "ointment", "pain", "painful", "painkiller", "painkillers", "paracetamol", "pharmacist", "pill", "pills", "prescription", "reaction", "steroid", "steroids", "surgery", "swollen", "symptom", "symptoms", "syringed", "tablet", "tablets", "unwell", "vaccination", "vaccine", "vaccines", "vitamin", "vitamins"];
#[rustfmt::skip]
pub(super) const IDIOM_PHRASES: &[&str] = &["a drop in the ocean", "a grey area", "a pain in the backside", "a pain in the neck", "a piece of cake", "a shot in the arm", "a storm in a teacup", "at my wits end", "at our wits end", "at wits end", "back on her feet", "back on his feet", "back on its feet", "back on my feet", "back on our feet", "back on their feet", "cherry on the cake", "cold feet", "cold turkey", "dead on her feet", "dead on his feet", "dead on my feet", "dead on their feet", "does my head in", "doing her head in", "doing his head in", "doing my head in", "doing my nut in", "doing our head in", "easy as pie", "find my feet", "finding my feet", "found my feet", "half baked idea", "icing on the cake", "in the same boat", "in two minds", "music to my ears", "my cup of tea", "not my cup of tea", "off the boil", "off the rails", "on its last legs", "on my feet all day", "on my feet all week", "on our feet all day", "on the fence", "on the same page", "on their last legs", "out of my depth", "out of our depth", "over the moon", "piece of cake", "pushed off my feet", "ran off my feet", "ran ragged", "run off my feet", "run off our feet", "run off your feet", "run ragged", "running off my feet", "running ragged", "rushed off my feet", "rushed off our feet", "snowed under", "snowed under with", "spill the beans", "spilled the beans", "spilling the beans", "swept me off my feet", "swept off my feet", "the icing on the cake", "the last straw", "the tip of the iceberg", "two left feet", "under my belt", "under the weather", "up on my feet"];
