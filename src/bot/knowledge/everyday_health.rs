//! Condition-specific replies with the source registry's triggers and exclusions.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Reading, understanding::topics};

use super::guards::Context;

#[path = "vaccine_booster.rs"]
pub(super) mod vaccine;

macro_rules! pattern {
    ($name:ident, $pattern:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}

pattern!(
    YOUNG,
    r"\b(?:baby|babies|newborns?|infants?|toddlers?|children|child|kids?|my son|my daughter|my boy|my girl|little one|months? old|first teeth|new teeth|baby teeth|milk teeth|teething)\b"
);
pattern!(
    BABY_OR_FEED,
    r"\b(?:baby|babies|newborns?|infants?|toddlers?|my son|my daughter|my child|breastfeeding|bottle feeding)\b"
);
pattern!(
    HICCUP_DURATION,
    r"\b(?:48 hours|more than two days|for days|for (?:[2-9]|two|three|four|five|six|seven) days|weeks|months|comes? back|keeps? coming back)\b"
);
pattern!(
    GRINDING_DEFINITION,
    r"\b(?:what (?:is|are|does|do)|whats|define|definition|what it means|meaning)\b"
);
pattern!(
    HANGOVER_DURATION,
    r"\b(?:how long|how many (?:hours|days)|when (?:will|does|do|should)|how soon|lasts?)\b"
);
pattern!(
    ALCOHOL_EMERGENCY,
    r"\b(?:passed out|unconscious|can ?t wake|won ?t wake|hard to wake|cannot wake|not breathing|slow breathing|irregular breathing|blue lips|seizure|fit)\b"
);
pattern!(
    STROKE_SIGNS,
    r"\b(?:face droop\w*|face is droop\w*|facial droop\w*|slurred speech|speech is slurred|slurring|sudden (?:weakness|numbness|vision loss)|one sided (?:weakness|numbness)|cant speak|can t speak)\b"
);
pattern!(
    BREATHING_EMERGENCY,
    r"\b(?:struggling to breathe|gasping|cannot breathe|cant breathe|can t breathe|blue lips)\b"
);
pattern!(
    COLD_FOOD,
    r"\bcold (?:rice|pizza|chicken|pasta|potatoes?|noodles?|chips|meat|sausages?|beans|curry|stew|takeaway|sandwich)\b|\bleft ?overs?\b|\bleft over rice\b"
);
pattern!(
    HEARTBURN_PREGNANCY,
    r"\b(?:pregnant|pregnancy|expecting a baby|having a baby|with child)\b"
);
pattern!(
    HEARTBURN_RED_FLAGS,
    r"\b(?:hard to swallow|difficulty swallowing|food gets stuck|food sticking|unintended weight loss|losing weight without trying|keep vomiting)\b"
);
pattern!(
    CHEST_EMERGENCY,
    r"\b(?:chest pain|chest pressure|tight chest|crushing pain)\b"
);
pattern!(
    REFLUX_NAMED_IN_REPORT,
    r"\b([a-z]+(?: [a-z]+){0,2}) (?:gives? me|sets? off my|triggers? my|brings? on my) (?:heartburn|reflux|indigestion)\b|\b(?:heartburn|reflux|indigestion) (?:after eating|after drinking|when i eat|when i drink|from) ([a-z]+(?: [a-z]+){0,2})"
);
pattern!(
    REFLUX_GENERAL,
    r"\b(?:which foods?|what foods?|foods?|diet|triggers?|causes?|avoid|cut out|what should|list|what can i eat)\b"
);
pattern!(A_COLD, r"\b(?:colds?|flu|sniffles?)\b");
pattern!(
    SEVERE_ALLERGY,
    r"\b(?:throat (?:swelling|closing|tightness)|swollen (?:tongue|throat)|tongue (?:is )?swelling|trouble breathing|can ?t breathe|difficulty breathing)\b"
);
pattern!(
    ANTICOAGULANTS,
    r"\b(?:blood thinners?|warfarin|apixaban|rivaroxaban|dabigatran|edoxaban|anticoagulants?)\b"
);
pattern!(
    BRUISE_URGENT,
    r"\b(?:rapidly (?:growing|spreading)|severe (?:pain|bruising)|very painful|numbness|hard and tight|bleeding that won ?t stop|head injury|hit my head|blood in (?:my )?(?:urine|stool))\b"
);

const REFLUX_HELP: &str = "Try smaller meals, avoid your own triggers and leave 3 to 4 hours between eating and going to bed. A pharmacist can help choose short-term relief. See a GP if heartburn happens most days, treatment is failing or you have symptoms such as food sticking, repeated vomiting or unexplained weight loss.";
const BRUISE_CARE: &str = "For a minor bruise after a knock, rest and raise the area if you can. Wrap a cold pack in a cloth and use it for up to 20 minutes at a time during the first couple of days.";

pattern!(
    TONSILLITIS_SYMPTOMS,
    concat!(
        r"^(?:(?:so|ok|okay|and|but) )?(?:(?:what are |tell me (?:about )?)?(?:the )?",
        r"(?:common |usual |early )?(?:signs and symptoms|symptoms|signs) of tonsillitis",
        r"|(?:what are (?:the )?)?tonsillitis (?:signs and symptoms|symptoms|signs)",
        r"|what (?:symptoms|signs) (?:does tonsillitis cause|can tonsillitis cause)",
        r"|(?:can|could|does) tonsillitis cause (?:difficulty breathing|trouble breathing|difficulty swallowing))",
        r"(?: please)?$"
    )
);

pub(super) fn tonsillitis_symptom_subject(words: &[String]) -> Option<usize> {
    let subject = words.iter().position(|word| word == "tonsillitis")?;
    TONSILLITIS_SYMPTOMS
        .is_match(&words.join(" "))
        .then_some(subject)
}

// https://www.nhs.uk/conditions/tonsillitis/
pub(super) fn tonsillitis(reading: &Reading<'_>) -> &'static str {
    const SYMPTOMS: &str = "Common signs of tonsillitis include red, swollen tonsils, a sore throat, pain when you swallow and a fever. You may also have swollen glands in your neck or white patches on the tonsils. Other infections can look similar. Get NHS 111 advice now if you see pus-filled spots or if the pain makes it hard to eat or drink. Call 999 or go to A&E if you cannot swallow, have difficulty breathing or have severe symptoms that are getting worse quickly.";
    const GENERAL: &str = "Tonsillitis is an infection of the tonsils. Most cases are viral and improve on their own after a few days. Rest, cool drinks and suitable pain relief can help. Seek NHS 111 advice now if you see pus-filled spots or if the pain makes it hard to eat or drink. Call 999 or go to A&E if you cannot swallow, have difficulty breathing or have severe symptoms that are getting worse quickly. See a GP if it keeps coming back or has not cleared in a week.";
    let words = &reading.line.words;
    if tonsillitis_symptom_subject(words).is_some() {
        SYMPTOMS
    } else {
        GENERAL
    }
}

fn has(line: &str, choices: &[&str]) -> bool {
    choices.iter().any(|choice| line.contains(choice))
}

fn word(line: &str, choices: &[&str]) -> bool {
    choices.iter().any(|choice| {
        line.match_indices(choice).any(|(at, matched)| {
            !line[..at].ends_with(char::is_alphanumeric)
                && !line[at + matched.len()..].starts_with(char::is_alphanumeric)
        })
    })
}

pub(super) fn scab_definition(reading: &Reading<'_>) -> bool {
    !word(
        &reading.line.clean,
        &[
            "itch",
            "itches",
            "itchy",
            "itching",
            "pick",
            "picking",
            "scratch",
            "scratching",
        ],
    )
}

pub(super) fn cold_is_an_illness(reading: &Reading<'_>) -> bool {
    topics::warmth(&reading.line.norm).is_none()
        && !COLD_FOOD.is_match(&reading.line.text.to_lowercase())
}

// https://www.nhs.uk/symptoms/hiccups/
// https://www.nhs.uk/conditions/reflux-in-babies/
fn hiccups(clean: &str, context: &Context<'_>) -> &'static str {
    if BABY_OR_FEED.is_match(clean) {
        if context.earlier.iter().any(|turn| {
            turn.bot.starts_with("Hiccups in a baby") || turn.bot.starts_with("Hiccups can recur")
        }) {
            return "Hiccups can recur in babies, especially during or after feeds. Pause to wind your baby if they seem uncomfortable and hold them upright during feeds. Contact your health visitor or GP if feeding is difficult or hiccups come with persistent coughing or vomiting.";
        }
        return "Hiccups in a baby often settle by themselves. If they seem unsettled during feeds, pause to wind them and hold them upright while feeding. Ask a health visitor or GP if your baby often coughs or has trouble feeding.";
    }
    if HICCUP_DURATION.is_match(clean) {
        return "Contact your GP if hiccups have lasted over 48 hours or repeatedly come back and affect your life. Mention associated symptoms, such as pain or trouble swallowing.";
    }
    "Hiccups usually pass on their own. Try sipping cold water slowly or briefly holding your breath if comfortable. See a GP if they last over 48 hours or keep coming back and affect your life."
}

// https://www.nhs.uk/conditions/common-cold/
fn cold(clean: &str) -> &'static str {
    if BREATHING_EMERGENCY.is_match(clean) {
        return "Call 999 in the UK for severe breathing difficulty, gasping or blue lips. Get immediate help rather than treating this as an ordinary cold.";
    }
    if has(
        clean,
        &[
            "vitamin c",
            "vitamin",
            "zinc",
            "echinacea",
            "supplement",
            "tablets",
            "lemsip",
        ],
    ) {
        return "There is little evidence that supplements such as vitamin C or echinacea prevent a cold or speed recovery. A pharmacist can advise on symptom relief. Check the ingredients carefully, since cold remedies may already contain paracetamol or other medicines you take.";
    }
    if has(
        clean,
        &[
            "prevent",
            "ward off",
            "avoid",
            "keep away",
            "keeping away",
            "colds away",
            "stay well",
            "keeping well",
            "not catch",
            "coming on",
            "in its tracks",
            "catch",
            "not get",
            "dodge",
            "before it",
            "stop a cold",
            "stop this cold",
            "stop the cold",
            "stop my cold",
            "stops a cold",
        ],
    ) {
        return "A cold usually runs its course. To reduce the chance of picking up or spreading a cold, wash your hands, cover coughs and sneezes with tissues, and avoid sharing towels and cups. Rest and drink fluids while you recover.";
    }
    "Rest, drink fluids and ask a pharmacist what you can safely use for symptoms. Most colds begin to improve in 1 to 2 weeks. Warm lemon and honey may soothe a sore throat for anyone over 1. Check with a pharmacist before giving cold medicines to children. Seek advice if symptoms worsen, have not improved after about 10 days, or you have breathing difficulty or chest pain."
}

// https://www.nhs.uk/symptoms/dizziness/
// https://www.nhs.uk/conditions/stroke/symptoms/
fn dizzy(clean: &str, context: &Context<'_>) -> Option<&'static str> {
    if context
        .memory
        .is_some_and(|memory| memory.urgent_medical.is_some())
    {
        return None;
    }
    if STROKE_SIGNS.is_match(clean) {
        return Some(
            "Call 999 now in the UK if dizziness started with facial droop, speech trouble, weakness or numbness on one side or sudden loss of vision. Get emergency help even if those symptoms have stopped.",
        );
    }
    Some(
        "Sit or lie down until the dizziness passes, then get up slowly. Drink water and avoid driving, climbing or using machinery while dizzy. Contact a GP if it persists or returns, or if you notice hearing or vision changes, ringing in your ears or fainting. Sudden weakness, facial droop or speech trouble means call 999 in the UK.",
    )
}

// https://www.nhs.uk/symptoms/teeth-grinding/
fn teeth(clean: &str) -> &'static str {
    if YOUNG.is_match(clean) {
        "Children sometimes grind their teeth. Ask their dentist if it continues, wears or damages the teeth, causes pain, or affects sleep. Let the dentist decide whether any treatment is appropriate for their age."
    } else if GRINDING_DEFINITION.is_match(clean) {
        "Bruxism means grinding or clenching the teeth. It can happen during sleep or while awake, and may cause jaw pain, headaches or worn teeth. A dentist can check for damage and discuss ways to protect the teeth."
    } else {
        "Tell your dentist if grinding is wearing your teeth, waking you or causing jaw pain or headaches. A dentist can check for damage and fit a mouth guard if suitable. Relaxing your jaw when awake and reducing caffeine and alcohol may also help."
    }
}

// https://www.nhs.uk/conditions/sprains-and-strains/
// https://www.nhs.uk/medicines/anticoagulants/side-effects/
fn bruise(clean: &str) -> Cow<'static, str> {
    if BRUISE_URGENT.is_match(clean)
        || ANTICOAGULANTS.is_match(clean)
            && word(
                clean,
                &["large", "unexplained", "spreading", "growing", "bruising"],
            )
    {
        return Cow::Borrowed(
            "Get urgent medical advice for a bruise that is rapidly enlarging, very painful, tight or numb, or for severe or unexplained bruising while taking a blood thinner. Ask NHS 111 where to be seen in the UK. Get emergency help for heavy bleeding that will not stop.",
        );
    }
    let initial = if word(
        clean,
        &[
            "spread",
            "spreading",
            "spreads",
            "growing",
            "bigger",
            "larger",
            "getting worse",
        ],
    ) {
        "Bruise colour can spread as blood moves under the skin. Get medical advice promptly if it is still enlarging or gets more painful or swollen."
    } else if has(
        clean,
        &[
            "wont go",
            "won't go",
            "not going",
            "doesnt fade",
            "doesn't fade",
            "not fading",
            "still there",
            "still here",
            "keeps coming",
            "for weeks",
            "months",
            "lasts",
            "lasting",
        ],
    ) {
        "Many bruises fade over a couple of weeks, while larger bruises can take longer. Ask a GP about a bruise that is not improving, keeps recurring or appeared without an injury."
    } else if word(
        clean,
        &[
            "hurts",
            "hurt",
            "hurting",
            "pain",
            "painful",
            "throbbing",
            "sore",
            "aching",
            "ache",
        ],
    ) {
        "A bruise can be tender and swollen. Get advice if pain is getting worse, the injury limits movement or the area becomes hot and swollen."
    } else if word(
        clean,
        &[
            "faster",
            "quicker",
            "quickly",
            "quick",
            "get rid",
            "go away",
            "clear up",
            "heal",
            "healing",
            "fade",
            "treat",
            "treatment",
            "help",
        ],
    ) {
        "A cold pack may ease discomfort and swelling while the bruise heals in its own time. Ask a pharmacist if you need help choosing safe pain relief."
    } else {
        "A bruise is blood from small vessels that has leaked under the skin. Its colour changes as it heals. Ask a GP if you repeatedly bruise without a clear cause, bruise more easily than usual or have other unusual bleeding."
    };
    Cow::Owned(format!("{initial} {BRUISE_CARE}"))
}

// https://www.nhs.uk/conditions/heartburn-and-acid-reflux/
// https://www.nhs.uk/symptoms/chest-pain/
fn heartburn(clean: &str) -> Cow<'static, str> {
    const TREATMENT: &[&str] = &[
        "gaviscon",
        "antacid",
        "antacids",
        "ranitidine",
        "omeprazole",
        "lansoprazole",
        "esomeprazole",
        "ppi",
        "ppis",
        "alginate",
    ];
    const MEDICINES: &[&str] = &[
        "tablets",
        "tablet",
        "pills",
        "pill",
        "medication",
        "medicine",
    ];
    if CHEST_EMERGENCY.is_match(clean) {
        return Cow::Borrowed(
            "Call 999 in the UK for sudden or persistent chest pressure or pain, especially with sweating, nausea, breathlessness or pain spreading to the arm or jaw. A heart problem can feel like indigestion.",
        );
    }
    if HEARTBURN_RED_FLAGS.is_match(clean) {
        return Cow::Borrowed(
            "Contact a GP promptly if food sticks or swallowing is difficult, if you keep vomiting, or if you have lost weight without trying. Those symptoms need assessment even if they feel like reflux.",
        );
    }
    if HEARTBURN_PREGNANCY.is_match(clean) {
        return Cow::Borrowed(
            "Try smaller meals, stay upright after eating and avoid meals close to bedtime. Ask your midwife or pharmacist which heartburn medicine is suitable during pregnancy, especially if you take iron or folic acid.",
        );
    }
    if (word(clean, TREATMENT) || word(clean, MEDICINES))
        && has(
            clean,
            &[
                "not working",
                "arent working",
                "arent helping",
                "not helping",
                "doesnt work",
                "doesnt help",
                "no good",
                "stopped working",
                "isnt working",
                "not doing anything",
            ],
        )
    {
        return Cow::Borrowed(
            "If the remedy is no longer helping or you have heartburn most days, book a GP appointment. A pharmacist can review what you have tried and check for interactions. Follow the prescribed dose rather than increasing it on your own.",
        );
    }
    if word(clean, MEDICINES)
        || word(
            clean,
            &[
                "ibuprofen",
                "aspirin",
                "naproxen",
                "voltarol",
                "nurofen",
                "painkiller",
                "painkillers",
                "pain killer",
                "pain killers",
                "anti inflammatories",
                "anti inflammatory",
            ],
        )
    {
        return Cow::Borrowed(
            "Some medicines, including anti-inflammatory painkillers, can worsen reflux. Ask your pharmacist or prescriber to review what you take and discuss alternatives if needed. Keep taking prescribed medicine unless your prescriber advises a change.",
        );
    }
    if word(clean, TREATMENT) {
        return Cow::Borrowed(REFLUX_HELP);
    }
    for (aliases, label) in [
        (
            &[
                "tomato",
                "tomatoes",
                "tomato sauce",
                "passata",
                "ketchup",
                "bolognese",
                "pasta sauce",
            ][..],
            "Tomato-based foods",
        ),
        (
            &[
                "spicy",
                "spice",
                "spices",
                "curry",
                "curries",
                "chilli",
                "chilli sauce",
                "chili",
                "hot sauce",
                "hot food",
                "jalapeno",
                "jalapenos",
                "vindaloo",
                "tikka",
                "naga",
            ],
            "Spicy foods",
        ),
        (
            &[
                "citrus",
                "orange",
                "oranges",
                "orange juice",
                "lemon",
                "lemons",
                "grapefruit",
                "lime",
                "satsuma",
                "clementine",
            ],
            "Citrus foods and drinks",
        ),
        (
            &[
                "onion", "onions", "garlic", "shallot", "shallots", "leek", "leeks",
            ],
            "Onions and garlic",
        ),
        (
            &[
                "fried", "frying", "fatty", "greasy", "chips", "batter", "pizza", "cheese",
                "cream", "takeaway",
            ],
            "Fried or fatty foods",
        ),
        (
            &[
                "mint",
                "mint tea",
                "peppermint",
                "peppermint tea",
                "spearmint",
                "after dinner mint",
                "polos",
            ],
            "Mint and peppermint",
        ),
        (
            &[
                "coffee",
                "espresso",
                "latte",
                "cappuccino",
                "caffeine",
                "cola",
                "energy drink",
            ],
            "Coffee and other caffeinated drinks",
        ),
        (
            &[
                "wine",
                "beer",
                "lager",
                "alcohol",
                "vodka",
                "gin",
                "cider",
                "prosecco",
                "whisky",
                "whiskey",
                "brandy",
                "cocktail",
                "cocktails",
            ],
            "Alcoholic drinks",
        ),
        (&["chocolate"], "Foods containing chocolate"),
        (&["tea"], "Caffeinated teas"),
    ] {
        if word(clean, aliases) {
            return Cow::Owned(format!(
                "{label} can trigger symptoms in some people. Keep a record of when it happens and see whether cutting back on the suspected trigger helps. {REFLUX_HELP}"
            ));
        }
    }
    if has(
        clean,
        &[
            "what can i eat",
            "what should i eat",
            "what to eat",
            "what can i have",
            "what should i have",
            "foods i can eat",
            "food i can eat",
            "safe to eat",
        ],
    ) {
        return Cow::Borrowed(
            "Choose smaller meals with foods you tolerate. Track what you eat and when burning starts so you can reduce the foods that repeatedly trigger it. Leave time between eating and lying down. See a GP if symptoms happen most days.",
        );
    }
    if let Some(found) = REFLUX_NAMED_IN_REPORT.captures(clean)
        && let Some(food) = found.get(1).or_else(|| found.get(2))
        && !word(
            food.as_str(),
            &[
                "what", "why", "how", "which", "i", "my", "it", "anything", "foods",
            ],
        )
    {
        return Cow::Owned(format!(
            "If symptoms repeatedly follow {}, try reducing it and note whether the pattern changes. {REFLUX_HELP}",
            food.as_str()
        ));
    }
    if REFLUX_GENERAL.is_match(clean) {
        return Cow::Borrowed(
            "Common triggers include large meals, fatty or spicy foods, coffee, alcohol and chocolate. Your own pattern matters most. Keep a brief food and symptom record, then reduce the foods or drinks that reliably bring it on.",
        );
    }
    if word(clean, &["why", "what causes"]) {
        return Cow::Borrowed(
            "Heartburn is a burning feeling caused by stomach acid coming back into the food pipe. It can be worse after eating, lying down or bending over. See a GP if it happens most days, or promptly if food sticks or swallowing becomes difficult.",
        );
    }
    Cow::Borrowed(REFLUX_HELP)
}

// https://www.nhs.uk/conditions/hay-fever/
fn hayfever(clean: &str) -> &'static str {
    if SEVERE_ALLERGY.is_match(clean) {
        "Call 999 in the UK for sudden throat or tongue swelling or trouble breathing. Those can be signs of a severe allergic reaction. Use your adrenaline auto-injector if you have one and follow the call handler's advice."
    } else if A_COLD.is_match(clean) {
        "Itchy or watery eyes, sneezing and a runny nose that coincide with pollen exposure suggest hay fever. A cold commonly brings a sore throat or cough and usually starts improving within 1 to 2 weeks. Ask a pharmacist if you are unsure which it is or symptoms are affecting daily life."
    } else {
        "Check the pollen forecast, try wraparound glasses outside and shower and change clothes after high exposure. Ask a pharmacist about suitable antihistamines, eye drops or a nasal spray. See a GP if symptoms are worsening or pharmacy treatment is not helping."
    }
}

// https://www.nhs.uk/conditions/alcohol-poisoning/
fn hangover(clean: &str) -> &'static str {
    if ALCOHOL_EMERGENCY.is_match(clean) {
        "Call 999 now in the UK if someone who has been drinking cannot be woken, has slow or irregular breathing or has a seizure. Stay with them. If unconscious and breathing, put them in the recovery position. Follow the call handler's instructions."
    } else if HANGOVER_DURATION.is_match(clean) {
        "A hangover may last into the next day. Rest, sip water if you are fully awake, and eat something light if you can. Time is what lowers the alcohol level. Arrange another way to travel if you may still have alcohol in your system."
    } else {
        "Rest and sip water if you are fully awake, and try a light meal if your stomach can tolerate it. More alcohol can prolong the problem. Check with a pharmacist before taking a painkiller if you have been drinking heavily or take other medicines."
    }
}

pub(super) fn registered(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    let clean = &context.reading.line.clean;
    Some(Cow::Borrowed(match id {
        "body-what-scab" => {
            "A scab is dried blood and material from the clot, including fibrin, that forms over a cut while skin repairs itself. Leave it in place. Keep the wound clean and seek advice if redness or pain spreads, the area becomes hot or swollen or pus appears."
        }
        "hiccups" => hiccups(clean, context),
        "cold-remedy" => cold(clean),
        "dizzy-spell" => dizzy(clean, context)?,
        "teeth-grinding" => teeth(clean),
        "bruise" => return Some(bruise(clean)),
        "heartburn" => return Some(heartburn(clean)),
        "hayfever" => hayfever(clean),
        "hangover-morning" => hangover(clean),
        "vaccine-booster" => vaccine::booster(context)?,
        _ => return None,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn everyday_health_preserves_condition_branches() {
        assert!(!scab_definition(&Reading::new("Why does my scab itch?")));
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("What is a scab?");
        assert!(turn.reply.contains("fibrin"), "{}", turn.reply);
        assert!(cold("can vitamin c stop a cold").contains("little evidence"));
        assert!(!cold_is_an_illness(&Reading::new(
            "Can we use the cold rice?"
        )));
        assert!(heartburn("prawns trigger my heartburn").contains("follow prawns"));
        assert!(heartburn("my gaviscon isnt working for heartburn").contains("GP appointment"));
        assert!(bruise("my bruise is rapidly growing").contains("urgent medical advice"));
        assert!(hayfever("is it hay fever or a cold").contains("pollen exposure"));
        assert!(hangover("he was drinking and cannot wake up").contains("Call 999"));
        let reading = Reading::new("I feel dizzy and my speech is slurred");
        assert!(
            dizzy(&reading.line.clean, &Context::new(&reading, &[]))
                .unwrap()
                .contains("Call 999")
        );
    }

    #[test]
    fn everyday_health_boosters_respect_topic_and_wound_guidance() {
        let reading = Reading::new("Do I need a tetanus booster after a dirty nail puncture?");
        let reply = registered("vaccine-booster", &Context::new(&reading, &[])).unwrap();
        assert!(reply.contains("get it assessed promptly"));
        let reading = Reading::new("Should I get a booster?");
        let wifi = [Exchange {
            user: "The wifi in my bedroom keeps dropping".into(),
            bot: String::new(),
            module: "answers",
        }];
        assert!(registered("vaccine-booster", &Context::new(&reading, &wifi)).is_none());
        assert!(!vaccine::asks(&Reading::new(
            "When can I get a flu vaccine?"
        )));
    }
}
