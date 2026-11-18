//! Scope guards for habit setbacks and the user's own phone distraction.

use crate::bot::Reading;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(concat!("(?i)", $source)).expect(stringify!($name)));
    };
}

pattern!(
    MACHINE,
    r"\b(?:washing machine|tumble dryer|dishwasher|extractor fan|garage door|door bell|doorbell|boiler|radiator|heating|hot water|shower|tap|toilet|cistern|fridge|freezer|oven|hob|microwave|kettle|toaster|vacuum|hoover|printer|router|wifi|broadband|laptop|phone|computer|screen|monitor|tablet|tv|television|camera|watch|headphones|earbuds|battery|charger|socket|plug|bulb|lights|fan|car|van|bike|engine|alarm|lock|window)s?\s+(?:keeps|keep|kept)\b"
);
pattern!(
    FOOD,
    r"\b(?:rice|pasta|noodles?|bread|dough|batter|cake|cakes|pastry|sauce|gravy|soup|stew|curry|porridge|oats|custard|jam|chocolate|caramel|fudge|icing|meringue|potato|potatoes|chips|meat|chicken|fish|egg|eggs|yoghurt|yogurt|milk|cream|tofu|beans|lentils|couscous|quinoa|pancakes?|scones?|biscuits?|crust|pizza|mayo|mayonnaise|hollandaise|risotto|stir fry|mash|omelette|scrambled eggs|steak|bacon|sausages?)s?\s+(?:keeps|keep|kept)\b"
);
pattern!(
    STATE,
    r"\b(?:cold|lukewarm|warm|hot|stale|staler|mouldy|moldy|off|flat|soggy|dry|curdled|lumpy|burnt|burned)\b"
);
pattern!(
    GOODS,
    r"\b(?:tea|coffee|soup|broth|stew|dinner|food|milk|bread|loaf|toast|cake|cuppa|brew|hot chocolate|porridge|rice|pasta|gravy|sauce|drink|pint|beer|wine|juice|cup|mug|pot|bath|shower water|washing|laundry)\b\s+(?:keeps|keep|kept)\b"
);
pattern!(KEEPS, r"\b([a-z]+)\s+(?:keeps|keep|kept)\b");

const HABIT_SUBJECTS: &str = concat!(
    "i we you they he she people everyone habit habits routine routines streak streaks resolution resolutions ",
    "goal goals plan plans planning effort efforts attempt attempts progress motivation discipline willpower ",
    "schedule diary calendar app tracker journal challenge programme program training practice savings budget ",
    "diet weight sleep mood mind thoughts anxiety work study revision job career friendship friendships marriage ",
    "relationship sobriety quit quitting smoking drinking gambling walking running swimming cycling writing ",
    "reading meditation exercise exercising yoga gym cooking tidying"
);

const MONEY_WORDS: &[&str] = &[
    "pay",
    "paid",
    "payment",
    "bill",
    "subscription",
    "subscribed",
    "cost",
    "price",
    "charge",
    "refund",
    "cancel",
    "free trial",
    "upgrade",
    "premium",
    "monthly",
    "direct debit",
    "invoice",
];

fn keeping_food(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let normalized = if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    };
    let padded = format!(" {normalized} ");
    super::KEEPING_WORDS
        .iter()
        .any(|word| padded.contains(word))
        && super::KEPT_FOODS.iter().any(|food| {
            if food.contains(' ') {
                padded.contains(food)
            } else {
                line.words.iter().any(|word| word == food) || padded.contains(&format!(" {food} "))
            }
        })
}

pub(super) fn habit_setback(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let said = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    !keeping_food(reading)
        && !MACHINE.is_match(said)
        && !FOOD.is_match(said)
        && !(GOODS.is_match(said) && STATE.is_match(said))
        && KEEPS.captures_iter(said).all(|subject| {
            HABIT_SUBJECTS
                .split_ascii_whitespace()
                .any(|habit| subject[1].eq_ignore_ascii_case(habit))
        })
}

pub(super) fn own_phone_distraction(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let said = format!(
        "{} {} {} {}",
        line.clean, line.norm_text, line.text, reading.raw
    )
    .to_lowercase();
    !MONEY_WORDS.iter().any(|word| said.contains(word))
        && !super::social_guards::screens_are_somebody_elses(reading)
}

#[cfg(test)]
mod tests {
    use crate::bot::{Reading, knowledge::lookup};

    #[test]
    fn habit_and_phone_answers_keep_their_subject() {
        for (text, card) in [
            ("i broke my streak", "habit-restart"),
            ("how do i make it stick", "habit-restart"),
            ("my phone habit is out of control", "phone-distraction"),
        ] {
            let found = lookup::lookup(text).expect(text);
            assert_eq!((found.domain, found.id), ("routines", card));
        }
        for text in [
            "my laptop keeps going black",
            "why does my tea keep going cold",
        ] {
            assert!(!super::habit_setback(&Reading::new(text)), "{text}");
        }
        for text in [
            "how do i stop paying for facebook",
            "my dad is always glued to his phone",
        ] {
            assert!(!super::own_phone_distraction(&Reading::new(text)), "{text}");
        }
    }
}
