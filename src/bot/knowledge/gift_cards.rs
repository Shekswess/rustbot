//! Gift choices by recipient, age, and tastes in the user's words.

use super::{
    GIFT_AGE_WORDS, GIFT_CARD_AFTER, GIFT_CHEAP_ADULT, GIFT_CHEAP_CHILD, GIFT_CHEAP_TEEN,
    GIFT_CHILD_GIFT, GIFT_CHILD_PEOPLE, GIFT_CHILD_WORDS, GIFT_FOLD, GIFT_GIFT_ANYONE,
    GIFT_GROWN_GIFT, GIFT_INTERESTS, GIFT_MONEY_VERDICT_YES, GIFT_NAMED_BY_PRONOUN,
    GIFT_NOT_THE_THING, GIFT_PERSON_WORDS, GIFT_PLURAL_WORDS, GIFT_TASTE_WORDS, GIFT_TEEN_GIFT,
    GIFT_TODDLER_GIFT, GIFT_VERDICT_YES, guards::Context,
};
use crate::bot::{
    Exchange, Memory, Reading, compose,
    knowledge::shopping_replies::{gift_avoids_alcohol, gift_condition},
    understanding::topics,
};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

pub(super) type InterestPicks = &'static [(&'static str, &'static [(&'static str, u16)])];

#[derive(Clone, Copy)]
pub(super) enum Reply {
    Condition,
    Anyone,
    Generic,
    Person(&'static str),
    Age,
    Cheap,
}

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(WORD, r"[a-z']+");
pattern!(LETTER_WORD, r"[a-z]+");
pattern!(VERDICT_ASK, super::GIFT_VERDICT_ASK);
pattern!(VERDICT_MONEY, super::GIFT_MONEY_VERDICT);
pattern!(VERDICT_OBJECT, super::GIFT_VERDICT_OBJECT);
pattern!(CARD_WORDING, super::GIFT_CARD_WORDING);
pattern!(OTHER_CARD, super::GIFT_OTHER_CARD);
pattern!(CHEAP_ASK, super::GIFT_CHEAP_ASK);
pattern!(PRICE_SAID, super::GIFT_PRICE_SAID);
pattern!(PLURAL_ONLY, super::GIFT_PLURAL_ONLY);
pattern!(TURNING, super::GIFT_TURNING);
pattern!(AGE_SAID, super::GIFT_AGE_SAID);
pattern!(CHILD_AGE, super::GIFT_CHILD_AGE);
pattern!(
    TODDLER,
    r"\b(?:toddler|toddlers|preschool|preschooler|baby|babies)\b"
);
pattern!(TEEN, r"\b(?:teen|teens|teenager|teenagers|teenage)\b");
pattern!(MALE, r"\b(?:him|his|he)\b");
pattern!(FEMALE, r"\b(?:her|hers|she)\b");
pattern!(
    GROUP_AGE,
    r"\b(?:they are|theyre|they're)\s+(\d{1,3})\b\s*(?:years? old\b|(?:in|on|next)\s+[a-z]+\b|[.!?]|$)"
);
pattern!(FOOD_ALLERGY, r"\b(?:allerg(?:y|ies|ic)|coeliac|celiac)\b");
pattern!(BORN, r"\bborn\b");
pattern!(BIRTH_YEAR, r"\b(?:19|20)\d\d\b");
pattern!(
    BIRTH,
    r"\b(?:had|has|having|gave|give|giving|born|arrived|arrives|delivered|induced|in\s+labour|in\s+labor|went\s+into\s+labour|went\s+into\s+labor)\b"
);
pattern!(BIRTH_WEIGHT, r"\b\d+\s*(?:lb|lbs|pound|pounds|oz|ounces)\b");
pattern!(
    BABY_WORD,
    r"\b(?:baby|babies|babydoll|newborn|babe|infant|toddler|twins|nappy|nappies|pram|buggy|cot|labour|labor|midwife|caesarean|csection|breastfeeding|bottle-feeding|christening)\b"
);
pattern!(
    ANIMAL,
    r"\b(?:puppy|kitten|dog|cat|lamb|calf|chick|seedling|plant)\b"
);
pattern!(VERDICT_LED, r"(?i)^(?:(?:that|it|this)\s+)+");
pattern!(ARTICLE, r"(?i)^(?:a|an|the|my|some)\s+");
pattern!(VERDICT_WORD, r"(?i)[a-z]{3}");

const CHILD_RELATIONS: &[&str] = &[
    "son",
    "daughter",
    "grandson",
    "granddaughter",
    "nephew",
    "niece",
    "stepson",
    "stepdaughter",
    "grandchild",
];
const MALE_RELATIONS: &[&str] = &[
    "son",
    "dad",
    "father",
    "daddy",
    "papa",
    "husband",
    "brother",
    "nephew",
    "grandad",
    "grandpa",
    "granddad",
    "grandfather",
    "stepdad",
    "stepson",
    "uncle",
    "boyfriend",
    "fiance",
    "grandson",
    "godson",
];
const FEMALE_RELATIONS: &[&str] = &[
    "daughter",
    "mum",
    "mom",
    "mam",
    "mummy",
    "mumma",
    "mother",
    "wife",
    "sister",
    "niece",
    "nan",
    "nana",
    "gran",
    "granny",
    "grandma",
    "grandmother",
    "stepmum",
    "stepdaughter",
    "aunt",
    "auntie",
    "girlfriend",
    "fiancee",
    "granddaughter",
    "goddaughter",
];
const OTHER_RELATIONS: &[&str] = &[
    "child",
    "kid",
    "baby",
    "toddler",
    "partner",
    "spouse",
    "sibling",
    "stepsister",
    "stepbrother",
    "grandparents",
    "grandparent",
    "cousin",
    "friend",
    "flatmate",
    "housemate",
    "neighbour",
    "neighbor",
    "landlord",
    "landlady",
    "lodger",
    "manager",
    "tutor",
    "childminder",
    "carer",
    "hairdresser",
    "barber",
    "instructor",
    "grandchild",
];
const BABY_GIFT: &str = "Under a year, buy the boring things the parents will use: clothes in the size for three to six months ahead, a pack of muslins, a board book, or a soft ball. Skip anything that needs batteries or a cot to hang it from, and skip newborn sizes, because everyone buys those and the baby grows out of them in a fortnight.";

static RELATION: LazyLock<Regex> = LazyLock::new(|| {
    let mut words = [MALE_RELATIONS, FEMALE_RELATIONS, OTHER_RELATIONS].concat();
    words.sort_by_key(|word| std::cmp::Reverse(word.len()));
    Regex::new(&format!(r"\b(?:my|our)\s+({})(?:'s|s)?\b", words.join("|")))
        .expect("named gift recipients")
});
static CHILD_WORD: LazyLock<Regex> = LazyLock::new(|| {
    let words = GIFT_CHILD_WORDS
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>();
    Regex::new(&format!(r"(?:^|[^a-z])(?:{})(?:[^a-z]|$)", words.join("|"))).expect("gift age cues")
});

fn named_relation(text: &str) -> Option<&str> {
    RELATION.captures(text)?.get(1).map(|word| word.as_str())
}

fn same_age(ages: impl Iterator<Item = u8>) -> Option<u8> {
    let mut ages = ages;
    let first = ages.next()?;
    ages.all(|age| age == first).then_some(first)
}

fn only_one<T>(mut values: impl Iterator<Item = T>) -> Option<T> {
    let first = values.next()?;
    values.next().is_none().then_some(first)
}

fn group_age(turns: &[Exchange]) -> Option<u8> {
    turns.iter().rev().find_map(|turn| {
        let low = turn.user.to_lowercase();
        let age = GROUP_AGE
            .captures(&low)?
            .get(1)?
            .as_str()
            .parse::<u8>()
            .ok()?;
        (0 < age && age < 120).then_some(age)
    })
}

fn ages<'a>(context: &'a Context<'_>) -> impl Iterator<Item = (&'static str, u8)> + 'a {
    context
        .memory
        .into_iter()
        .flat_map(|mem| mem.personal.gift_people())
        .filter_map(|(role, age)| age.filter(|&age| age > 0).map(|age| (role, age)))
}

fn held_age(context: &Context<'_>) -> Option<f64> {
    let said = &context.reading.line.clean;
    let age = if let Some(named) = named_relation(said) {
        ages(context).find_map(|(role, age)| (role == named).then_some(age))
    } else {
        match (MALE.is_match(said), FEMALE.is_match(said)) {
            (true, false) => only_one(
                ages(context)
                    .filter_map(|(role, age)| MALE_RELATIONS.contains(&role).then_some(age)),
            ),
            (false, true) => only_one(
                ages(context)
                    .filter_map(|(role, age)| FEMALE_RELATIONS.contains(&role).then_some(age)),
            ),
            (false, false) if PLURAL_ONLY.is_match(said) => {
                same_age(ages(context).map(|(_, age)| age)).or_else(|| group_age(context.earlier))
            }
            _ => None,
        }
    }?;
    Some(f64::from(age))
}

fn is_child(context: &Context<'_>) -> bool {
    let said = &context.reading.line.clean;
    if let Some(named) = named_relation(said) {
        return CHILD_RELATIONS.contains(&named)
            && !ages(context).any(|(role, age)| role == named && age >= 18);
    }
    let wanted = match (MALE.is_match(said), FEMALE.is_match(said)) {
        (true, false) => Some(MALE_RELATIONS),
        (false, true) => Some(FEMALE_RELATIONS),
        _ => None,
    };
    let filed = context
        .memory
        .into_iter()
        .flat_map(|mem| mem.personal.gift_people());
    let recent = context.earlier.iter().rev().find_map(|turn| {
        let low = turn.user.to_lowercase();
        named_relation(&low).map(str::to_owned)
    });
    let mut people = filed.collect::<Vec<_>>();
    if let Some(recent) = recent.as_deref()
        && !people.iter().any(|&(role, _)| role == recent)
    {
        people.push((recent, None));
    }
    let fits = |role: &str| wanted.is_none_or(|roles| roles.contains(&role));
    (wanted.is_none() || people.iter().filter(|&&(role, _)| fits(role)).count() <= 1)
        && people.iter().any(|&(role, age)| {
            fits(role) && CHILD_RELATIONS.contains(&role) && age.is_none_or(|age| age < 18)
        })
}

fn newborn(context: &Context<'_>) -> bool {
    if ANIMAL.is_match(&context.reading.line.clean) {
        return false;
    }
    let prior = context
        .earlier
        .iter()
        .rev()
        .take(3)
        .map(|turn| turn.user.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    if ANIMAL.is_match(&prior) {
        return false;
    }
    let birth = BIRTH.is_match(&prior);
    let weight = BIRTH_WEIGHT.is_match(&prior);
    BORN.is_match(&prior) && !BIRTH_YEAR.is_match(&prior)
        || BABY_WORD.is_match(&prior) && (birth || weight)
        || birth && weight
}

fn age_from_number(reading: &Reading<'_>) -> Option<f64> {
    reading
        .numbers
        .iter()
        .find_map(|number| match number.unit {
            Some("months") => Some(number.value / 12.),
            Some("years") => Some(number.value),
            _ => None,
        })
        .or_else(|| {
            let year = TURNING
                .captures(&reading.line.clean)?
                .name("age")?
                .as_str()
                .parse::<u8>()
                .ok()?;
            let year = f64::from(year);
            (year <= 120. && reading.numbers.iter().any(|number| number.value == year))
                .then_some(year)
        })
}

fn age_from_words(text: &str) -> Option<f64> {
    let found = AGE_SAID.captures(text)?;
    for name in ["months", "years", "near", "ordinal"] {
        if let Some(word) = found.name(name) {
            let word = word.as_str();
            let age = word.parse::<u8>().ok().or_else(|| {
                GIFT_AGE_WORDS
                    .iter()
                    .find_map(|&(name, age)| (word == name).then_some(age))
            })?;
            return if name == "months" {
                (age > 0).then_some(f64::from(age) / 12.)
            } else {
                Some(f64::from(age))
            };
        }
    }
    None
}

fn by_age(age: f64) -> &'static str {
    if age < 1. {
        BABY_GIFT
    } else if age >= 20. {
        GIFT_GROWN_GIFT
    } else if age >= 13. {
        GIFT_TEEN_GIFT
    } else if age <= 2. {
        GIFT_TODDLER_GIFT
    } else {
        GIFT_CHILD_GIFT
    }
}

pub(super) fn names_age(reading: &Reading<'_>) -> bool {
    CHILD_AGE.is_match(&reading.line.clean)
}

pub(super) fn age_card_applies(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    if text.is_empty() {
        return false;
    }
    let named = GIFT_CHILD_PEOPLE.iter().any(|w| {
        if w.contains(' ') {
            text.contains(w)
        } else {
            WORD.find_iter(text).any(|word| word.as_str() == *w)
        }
    });
    if names_age(reading) {
        !named || age_from_words(text).is_none_or(|age| age <= 19.)
    } else {
        !named
    }
}

pub(super) fn no_recipient_pronoun(reading: &Reading<'_>) -> bool {
    let words = if reading.line.words.is_empty() {
        &reading.line.norm
    } else {
        &reading.line.words
    };
    !words
        .iter()
        .any(|word| GIFT_NAMED_BY_PRONOUN.contains(&word.as_str()))
}

fn fold(word: &str) -> Cow<'_, str> {
    let known = |word: &str| GIFT_PERSON_WORDS.contains(&word) || GIFT_TASTE_WORDS.contains(&word);
    if known(word) {
        return Cow::Borrowed(word);
    }
    for &(suffix, replacement) in GIFT_FOLD {
        if let Some(base) = word.strip_suffix(suffix) {
            let folded = format!("{base}{replacement}");
            if known(&folded) {
                return Cow::Owned(folded);
            }
        }
    }
    Cow::Borrowed(word)
}

fn current_and_recent<'a>(context: &'a Context<'_>, limit: usize) -> impl Iterator<Item = &'a str> {
    std::iter::once(context.reading.line.text.as_str()).chain(
        context
            .earlier
            .iter()
            .rev()
            .take(limit)
            .map(|turn| turn.user.as_str()),
    )
}

fn knows_taste(context: &Context<'_>) -> bool {
    current_and_recent(context, 8).any(|said| {
        let low = said.to_lowercase();
        let words: Vec<_> = WORD.find_iter(&low).map(|w| fold(w.as_str())).collect();
        words
            .iter()
            .any(|w| GIFT_PERSON_WORDS.contains(&w.as_ref()))
            && words.iter().any(|w| GIFT_TASTE_WORDS.contains(&w.as_ref()))
    })
}

fn join_last<T: AsRef<str>>(items: &[T], separator: &str) -> String {
    let Some((end, prior)) = items.split_last() else {
        return String::new();
    };
    if prior.is_empty() {
        return end.as_ref().to_owned();
    }
    let leading = prior
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join(", ");
    format!("{leading}{separator}{}", end.as_ref())
}

fn present_for_taste(context: &Context<'_>) -> Option<String> {
    let interests = current_and_recent(context, 4).find_map(|said| {
        let mut interests = Vec::new();
        let low = said.to_lowercase();
        for word in WORD.find_iter(&low) {
            let folded = fold(word.as_str());
            let ing = format!("{folded}ing");
            for candidate in [folded.as_ref(), &ing] {
                if let Some(interest) = GIFT_INTERESTS.iter().find(|(name, _)| *name == candidate)
                    && !interests.contains(interest)
                {
                    interests.push(*interest);
                }
            }
        }
        (!interests.is_empty()).then_some(interests)
    })?;
    let mut picks = Vec::new();
    for (_, choices) in &interests {
        for &(item, price) in *choices {
            if !picks.iter().any(|&(known, _)| known == item) {
                picks.push((item, price));
            }
        }
    }
    picks.sort_by_key(|&(_, price)| price);
    if picks.is_empty() {
        return None;
    }
    let gifts = picks
        .iter()
        .take(3)
        .map(|(item, price)| format!("{item} (£{price})"))
        .collect::<Vec<_>>();
    let hobbies = interests.iter().map(|(name, _)| *name).collect::<Vec<_>>();
    let who = context
        .reading
        .line
        .words
        .iter()
        .find_map(|word| {
            let word = word.as_str();
            if [
                "dad",
                "father",
                "grandad",
                "grandpa",
                "grandfather",
                "stepdad",
                "brother",
                "husband",
                "boyfriend",
                "son",
                "uncle",
                "nephew",
                "bloke",
                "fella",
                "guy",
                "he",
                "him",
                "his",
            ]
            .contains(&word)
            {
                Some("he")
            } else if [
                "mum",
                "mother",
                "mam",
                "nan",
                "nana",
                "gran",
                "granny",
                "grandma",
                "grandmother",
                "stepmum",
                "sister",
                "wife",
                "girlfriend",
                "daughter",
                "aunt",
                "niece",
                "she",
                "her",
                "hers",
            ]
            .contains(&word)
            {
                Some("she")
            } else {
                None
            }
        })
        .unwrap_or("they");
    Some(format!(
        "For someone into {}: {}. Buy the one {who} will actually use rather than three {who} might, and put a real message in the card.",
        join_last(&hobbies, " and "),
        join_last(&gifts, " or ")
    ))
}

fn gift_with_condition(reply: Cow<'static, str>, context: &Context<'_>) -> Cow<'static, str> {
    gift_condition(reply, context.reading, context.earlier)
}

fn recipient_food_allergy(context: &Context<'_>) -> bool {
    let said = &context.reading.line.clean;
    if FOOD_ALLERGY.is_match(said) {
        return true;
    }
    let named = if let Some(relation) = named_relation(said) {
        Some(relation.to_owned())
    } else {
        let matching_roles = match (MALE.is_match(said), FEMALE.is_match(said)) {
            (true, false) => Some(MALE_RELATIONS),
            (false, true) => Some(FEMALE_RELATIONS),
            _ => None,
        };
        let stored = context
            .memory
            .into_iter()
            .flat_map(|mem| mem.personal.gift_people());
        matching_roles.and_then(|roles| {
            only_one(stored.filter_map(|(role, _)| roles.contains(&role).then_some(role)))
                .map(str::to_owned)
        })
    };
    if let Some(relation) = named {
        return context
            .memory
            .is_some_and(|mem| mem.personal.gift_food_allergy(&relation))
            || context.earlier.iter().rev().take(2).any(|turn| {
                let earlier = turn.user.to_lowercase();
                named_relation(&earlier) == Some(relation.as_str())
                    && FOOD_ALLERGY.is_match(&earlier)
            });
    }
    (MALE.is_match(said) || FEMALE.is_match(said) || PLURAL_ONLY.is_match(said))
        && context.earlier.iter().rev().take(2).any(|turn| {
            let earlier = turn.user.to_lowercase();
            if let Some(role) = named_relation(&earlier)
                && (MALE.is_match(said) && !MALE_RELATIONS.contains(&role)
                    || FEMALE.is_match(said) && !FEMALE_RELATIONS.contains(&role))
            {
                return false;
            }
            FOOD_ALLERGY.is_match(&earlier)
        })
}

// https://www.gov.uk/understanding-food-labelling/allergen-labelling
const NON_FOOD_GIFTS: &str = "For a low-cost gift, choose something non-food they can use: a book, a notebook, a pair of socks, or a voucher for an activity they like.";
// https://www.gov.uk/government/publications/toys-safety-regulations-2011/toys-safety-regulations-2011-great-britain
const GIFTS_UNDER_THREE: &str = "For a child under three, choose a board book or a toy marked for their age, such as chunky stacking cups. Check the packaging for age guidance and warnings, and skip toys with small or detachable parts.";

fn cheap(context: &Context<'_>, apply_condition: bool) -> Cow<'static, str> {
    let age = age_from_number(context.reading).or_else(|| held_age(context));
    let clean = &context.reading.line.clean;
    if age
        .or_else(|| age_from_words(clean))
        .is_some_and(|age| age < 3.)
        || age.is_none() && TODDLER.is_match(clean)
    {
        return Cow::Borrowed(GIFTS_UNDER_THREE);
    }
    if recipient_food_allergy(context) {
        return Cow::Borrowed(NON_FOOD_GIFTS);
    }
    let reply = Cow::Borrowed(match age {
        Some(age) if age < 13. => GIFT_CHEAP_CHILD,
        Some(age) if age < 20. => GIFT_CHEAP_TEEN,
        Some(_) => GIFT_CHEAP_ADULT,
        None if CHILD_WORD.is_match(clean) => GIFT_CHEAP_CHILD,
        None => GIFT_CHEAP_ADULT,
    });
    if apply_condition {
        gift_with_condition(reply, context)
    } else {
        reply
    }
}

fn leaving_gift(base: &'static str, context: &Context<'_>) -> Cow<'static, str> {
    if !base.starts_with("A leaving present")
        || !gift_avoids_alcohol(context.reading, context.earlier)
    {
        return gift_with_condition(Cow::Borrowed(base), context);
    }
    let reply = base
        .replace(", a decent bottle for the first Friday", "")
        .replace("a pint after work", "coffee after work");
    Cow::Owned(format!(
        "{reply} Keep the gift alcohol-free, including any hamper."
    ))
}

fn pick(pool: &[&'static str], context: &Context<'_>, line: &str) -> &'static str {
    if let Some(memory) = context.memory {
        compose::pick(pool, memory, line)
    } else {
        compose::pick(pool, &Memory::default(), line)
    }
}

fn verdict(context: &Context<'_>, line: &str) -> Option<String> {
    let said = VERDICT_OBJECT.captures(line)?.get(1)?.as_str();
    let said = said.split_whitespace().collect::<Vec<_>>().join(" ");
    let item = VERDICT_LED.replace(&said, "");
    let low = item.to_lowercase();
    if [
        "a",
        "an",
        "the",
        "my",
        "some",
        "a gift",
        "a present",
        "that",
        "it",
        "this",
        "them",
        "they",
    ]
    .contains(&low.as_str())
        || !VERDICT_WORD.is_match(&item)
    {
        return None;
    }
    let age = age_from_number(context.reading)
        .filter(|&age| age != 0.)
        .or_else(|| held_age(context))
        .or_else(|| {
            only_one(
                ages(context)
                    .map(|(_, age)| age)
                    .chain(group_age(context.earlier)),
            )
            .map(f64::from)
        })?;
    let item_words = ARTICLE.replace(&low, "");
    let pool = by_age(age).to_lowercase();
    if !LETTER_WORD
        .find_iter(&item_words)
        .map(|m| m.as_str())
        .any(|word| {
            word.len() > 2
                && !GIFT_NOT_THE_THING.contains(&word)
                && LETTER_WORD.find_iter(&pool).any(|candidate| {
                    candidate.as_str() == word || candidate.as_str().strip_suffix('s') == Some(word)
                })
        })
    {
        return None;
    }
    let age = if age < 1. {
        "a baby".to_owned()
    } else if age == 1. {
        "one".to_owned()
    } else {
        age.to_string()
    };
    let words = if VERDICT_MONEY.is_match(line) {
        GIFT_MONEY_VERDICT_YES
    } else {
        GIFT_VERDICT_YES
    };
    Some(
        pick(words, context, line)
            .replace("{age}", &age)
            .replace("{thing}", &item),
    )
}

fn generic(base: &'static str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    let said = &context.reading.line.text;
    if VERDICT_ASK.is_match(said) {
        return verdict(context, said).map(Cow::Owned);
    }
    if !topics::gift(&context.reading.line.norm) {
        return (!CARD_WORDING.is_match(said)).then_some(Cow::Borrowed(base));
    }
    if PRICE_SAID.is_match(said) {
        return None;
    }
    if CHEAP_ASK.is_match(said) {
        return Some(cheap(context, true));
    }
    let reply = if knows_taste(context) {
        present_for_taste(context).map_or(Cow::Borrowed(base), Cow::Owned)
    } else if newborn(context) {
        Cow::Borrowed(BABY_GIFT)
    } else if let Some(age) = age_from_number(context.reading)
        .filter(|&age| age != 0.)
        .or_else(|| held_age(context))
    {
        Cow::Borrowed(by_age(age))
    } else {
        if PLURAL_ONLY.is_match(said)
            && !WORD.find_iter(&said.to_lowercase()).any(|word| {
                let word = fold(word.as_str());
                GIFT_PERSON_WORDS.contains(&word.as_ref())
                    && !GIFT_PLURAL_WORDS.contains(&word.as_ref())
            })
        {
            return None;
        }
        Cow::Borrowed(base)
    };
    Some(gift_with_condition(reply, context))
}

fn anyone(context: &Context<'_>) -> Option<Cow<'static, str>> {
    if !topics::gift(&context.reading.line.norm) {
        return None;
    }
    if knows_taste(context) {
        return present_for_taste(context).map(Cow::Owned);
    }
    let reply = if let Some(age) = held_age(context) {
        by_age(age)
    } else if newborn(context) {
        BABY_GIFT
    } else if is_child(context) {
        GIFT_CHILD_GIFT
    } else {
        GIFT_GIFT_ANYONE
    };
    Some(gift_with_condition(Cow::Borrowed(reply), context))
}

fn age_gift(context: &Context<'_>) -> &'static str {
    let line = &context.reading.line.text;
    if CARD_WORDING.is_match(line) && !OTHER_CARD.is_match(line) {
        return pick(GIFT_CARD_AFTER, context, line);
    }
    let clean = &context.reading.line.clean;
    if let Some(age) = age_from_number(context.reading)
        .filter(|&age| age != 0.)
        .or_else(|| age_from_words(clean))
    {
        by_age(age)
    } else if TODDLER.is_match(clean) {
        GIFT_TODDLER_GIFT
    } else if TEEN.is_match(clean) {
        GIFT_TEEN_GIFT
    } else {
        GIFT_CHILD_GIFT
    }
}

pub(super) fn reply(
    rule: Reply,
    base: &'static str,
    context: &Context<'_>,
) -> Option<Cow<'static, str>> {
    match rule {
        Reply::Condition => Some(leaving_gift(base, context)),
        Reply::Anyone => anyone(context),
        Reply::Generic => generic(base, context),
        Reply::Age => Some(Cow::Borrowed(age_gift(context))),
        Reply::Cheap => Some(cheap(context, false)),
        Reply::Person(child) => {
            let child = WORD
                .find_iter(&context.reading.line.text.to_lowercase())
                .any(|w| {
                    let word = fold(w.as_str());
                    GIFT_CHILD_PEOPLE.contains(&word.as_ref())
                        || [
                            "teenager",
                            "teenagers",
                            "teen",
                            "teens",
                            "teenage",
                            "schoolkid",
                            "schoolkids",
                            "schoolboy",
                            "schoolgirl",
                        ]
                        .contains(&word.as_ref())
                })
                .then_some(child);
            generic(child.unwrap_or(base), context)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    fn direct(id: &str, prior: &[&str], prompt: &str) -> Option<Cow<'static, str>> {
        let mut bot = Bot::new(Some("0")).unwrap();
        for previous in prior {
            bot.turn(previous);
        }
        let reading = Reading::new(prompt);
        let mut context = Context::new(&reading, &bot.memory.turns);
        context.memory = Some(&bot.memory);
        let row = super::super::rows()
            .find(|row| row.domain == "shopping" && row.id == id)
            .expect("registered gift card");
        let subject = row.key.first().copied().unwrap_or_default();
        if !row.guard.passes(&context, id, subject) {
            return None;
        }
        row.answer(&context)
    }

    #[test]
    fn authored_gift_families_keep_their_recipients_and_deflections() {
        for (id, prior, prompt, expected) in [
            (
                "gift-someone",
                &[][..],
                "Gift for someone",
                Some(GIFT_GIFT_ANYONE),
            ),
            ("gift-her", &[][..], "Should I get her a taxi?", None),
            (
                "gift-her",
                &["My son is 4."][..],
                "What should I get him for his birthday?",
                Some(GIFT_CHILD_GIFT),
            ),
            (
                "gift-child-age",
                &[][..],
                "Gift ideas for a 4 year old",
                Some(GIFT_CHILD_GIFT),
            ),
            (
                "gift-child-age",
                &[][..],
                "What should I buy my son who is 30?",
                None,
            ),
            ("gift-generic", &[][..], "A good present for him", None),
            (
                "gift-generic",
                &[][..],
                "Gift ideas for someone under 25 pounds",
                None,
            ),
            (
                "gift-cheap",
                &[][..],
                "Secret Santa ideas",
                Some(GIFT_CHEAP_ADULT),
            ),
            (
                "gift-cheap",
                &[][..],
                "Stocking filler ideas for a 9 year old",
                Some(GIFT_CHEAP_CHILD),
            ),
            (
                "gift-cheap",
                &["My niece is 15 in July."][..],
                "Cheap present ideas for my niece",
                Some(GIFT_CHEAP_TEEN),
            ),
        ] {
            assert_eq!(
                direct(id, prior, prompt).as_deref(),
                expected,
                "{id}: {prompt}"
            );
        }
        let person = direct("gift-person", &[], "What should I get my nephew?").unwrap();
        assert!(person.starts_with("For a child the age is most of the answer"));
        let leaving = direct("gift-leaving-work", &[], "Leaving present ideas").unwrap();
        assert!(leaving.contains("a decent bottle for the first Friday"));
    }

    #[test]
    fn reviewed_gift_variants_use_alcohol_free_non_food_and_age_safe_suggestions() {
        for (id, prior, prompt, expected) in [
            (
                "gift-cheap",
                &[][..],
                "Secret Santa ideas for someone with a nut allergy",
                NON_FOOD_GIFTS,
            ),
            (
                "gift-generic",
                &[][..],
                "Cheap Christmas present ideas for a coeliac",
                NON_FOOD_GIFTS,
            ),
            (
                "gift-person",
                &["My niece is allergic to peanuts."][..],
                "Cheap present ideas for my niece",
                NON_FOOD_GIFTS,
            ),
            (
                "gift-cheap",
                &[][..],
                "Stocking filler ideas for a 2 year old",
                GIFTS_UNDER_THREE,
            ),
            (
                "gift-cheap",
                &[][..],
                "Stocking filler ideas for a 6 month old",
                GIFTS_UNDER_THREE,
            ),
            (
                "gift-generic",
                &[][..],
                "Cheap Christmas present ideas for my 1 year old",
                GIFTS_UNDER_THREE,
            ),
            (
                "gift-cheap",
                &["My son is 2."][..],
                "Stocking filler ideas for my son",
                GIFTS_UNDER_THREE,
            ),
        ] {
            assert_eq!(
                direct(id, prior, prompt).as_deref(),
                Some(expected),
                "{id}: {prompt}"
            );
        }
        let leaving = direct(
            "gift-leaving-work",
            &[],
            "Retirement present ideas, he is teetotal",
        )
        .unwrap();
        assert!(leaving.contains("some people would rather have coffee after work"));
        assert!(leaving.ends_with("Keep the gift alcohol-free, including any hamper."));
    }
}
