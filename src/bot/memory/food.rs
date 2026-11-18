//! Explicitly reported allergies, dietary choices, and their corrections.
use super::{
    again,
    personal::{join_list, pick},
};
use crate::bot::{Memory, Reading};
use regex::{Captures, Regex};
use std::sync::LazyLock;

macro_rules! rule {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($source).unwrap());
    };
}

rule!(
    SUBJECT,
    r"\b(?:i(?:'m|'ve|m|ve)?|we(?:'re|'ve|re|ve)?|my|our|he(?:'s|s)?|she(?:'s|s)?|they(?:'re|re)?|your|their|his|her)\b"
);
rule!(
    QUESTION,
    r"^(?:(?:and|so|but|oh|right|ok|okay)[ ,]+)*(?:who|what|when|where|why|how|which|can|could|should|would|will|do|does|did|am|is|are|have|has)\b"
);
rule!(
    ALLERGIC,
    r"\ballergic to (?P<items>[a-z]+(?:[ ,]+(?:and )?[a-z]+){0,3})"
);
rule!(
    CANNOT_EAT,
    r"\b(?:can'?t|cannot|can not) (?:eat|have|drink|touch) (?P<items>[a-z]+(?:[ ,]+(?:and |or )?[a-z]+){0,3})"
);
rule!(ALLERGY_NOUN, r"\b(?P<items>[a-z]+) allerg(?:y|ies)\b");
rule!(
    CONTRAST,
    r"\b(?:allergic to|can'?t eat|cannot eat|can not eat) (?P<new>[a-z]+(?:(?:,| and| or) [a-z]+){0,3})[ ,]+not (?P<old>[a-z]+(?:(?:,| and| or) [a-z]+){0,3})"
);
rule!(
    NEGATION,
    r"\b(?:not|never|no longer|isn't|aren't|wasn't|isnt|arent|wasnt)(?:\s+\w+){0,2}\s*$"
);
rule!(
    LIST_END,
    r"\b(?:not|but|so|because|which|anymore|these days|again|after|before|since|until)\b"
);
rule!(LIST_SEP, r",|\s+(?:and|or)\s+");
rule!(
    ONE_MORE_HEAD,
    r"^(?:(?:it'?s|it is|that'?s|this is|these are|those are)|and|also|plus|oh|right|but|so|yeah|yes|then|actually|sorry|true|wait|no wait|hold on|i think|i reckon|i suppose|maybe|probably|possibly|i think maybe)\b[\s,]*"
);
rule!(
    ONE_MORE_TAIL,
    r"[\s,]*(?:\btoo\b|\bas well\b|\balso\b|\bi suppose\b)[\s.!?]*$"
);
rule!(
    DIET,
    r"\b(?:vegan|vegetarian|pescatarian|flexitarian|veggie|veg|plant[- ]based|gluten[- ]free|dairy[- ]free|low[- ]carb|keto|paleo|halal|kosher)\b"
);
rule!(
    DENIED_DIET,
    r"^(?:(?:actually|right|okay|well)[ ,]+)*(?:i(?:'?m| am)|we(?:'?re| are)) (?P<negative>not|no longer) (?P<way>[^.!?]+)[.!]*$"
);
rule!(ANY_MORE, r"\b(?:any ?more|no longer)\b");
rule!(
    DIET_CHANGE,
    r"^(?:(?:actually|right|okay|well)[ ,]+)*(?:(?:i|we)(?:'?ve| have)? (?:changed|switched|swapped|overhauled|tweaked|reworked) (?:my|our|the) diet|(?:my|our) diet (?:has changed|is (?:different|not the same)))(?: (?:now|recently|lately|these days|last (?:week|month|year)|a (?:week|month|year) ago))?[.!]*$"
);
rule!(
    EATING,
    r"^\s*(?:is|are|am|has been|have been|been)?\s*(?:a |an )?(?P<diet>.*)$"
);
rule!(
    NO_EAT,
    r"\b(?:don'?t|doesn'?t|do not|does not|never|no longer) eat (?:any |any more |more )?(?P<items>[a-z]+(?:(?:,| or| and) [a-z]+)?)"
);
rule!(
    OWN_NO_EAT,
    r"\b(?:i|we) ?(?:don'?t|do not|never|no longer) eat (?:any more |any |more )?(?P<items>[a-z]+(?:(?:,| or| and) [a-z]+)?)"
);
rule!(
    OWN_NO_DRINK,
    r"\b(?:i|we) (?:(?:really|just|actually) )?(?:don'?t|do not|never|no longer) (?:(?:really|just|actually) )?drink(?: (?:any ?more|at all|now))?[.!]*$"
);
rule!(
    OWN_NO_DRINK_THING,
    r"\b(?:i|we) (?:(?:really|just|actually) )?(?:don'?t|do not|never|no longer) drink (?P<thing>[a-z]+(?: (?:or|and) [a-z]+){0,3})"
);
rule!(
    NO_DRINK,
    r"\b(?:don'?t|doesn'?t|do not|does not|never|no longer) drink(?: (?P<thing>alcohol|wine|beer)| (?:any more|anymore|at all|now))?[.!]*$"
);
rule!(
    DIET_UNDERWAY,
    r"\b(?:i(?:'?m| am)|we(?:'?re| are)|i(?:'?ve| have))(?: been)? (?:(?:on|doing|trying|starting|started|back on|gone) (?:a |the |an )?(?P<way>slimming world|weight watchers|intermittent fasting|whole ?30|omad|carnivore|dukan|south beach|fast 800|5:2|16:8|4:3|keto|paleo|atkins|low[- ]carb|mediterranean|gluten[- ]free|dairy[- ]free|plant[- ]based|pescatarian|flexitarian|vegan|vegetarian|diet)|dieting)\b"
);
rule!(
    CONDITION,
    r"\b(?:lactose intolerant|gluten intolerant|coeliac|celiac)\b"
);
rule!(
    OWN_ALLERGY_ASK,
    r"^(?:(?:and|so|but|ok|okay|right)[ ,]+)*(?:what (?:am i|are we|was i) allergic to|what(?:'s| is| was) my allerg(?:y|ies)|(?:do|can) you (?:know|remember) what i(?:'m| am) allergic to)(?: (?:again|now|then))?[.?!]*$"
);
rule!(
    DIET_ASK,
    r"^(?:(?:and|so|but|ok|okay|right)[ ,]+)*(?:what(?:'s|s| is| was) (?:my|our) diet|what (?:did i say|have i told you) i (?:do not|don't|cannot|can't) eat|what (?:diet|way of eating) (?:am i|are we) (?:on|following))(?: (?:again|now|then))?[.?!]*$"
);
rule!(
    CANT_EAT_ASK,
    r"^(?:(?:and|so|but|ok|okay|right)[ ,]+)*what (?:can'?t|cannot|can not) (?:i|we) eat(?: (?:again|now))?[.?!]*$"
);
rule!(
    RELATION_FOOD_ASK,
    r"^what (?:is|are|was|were) (?P<subject>.+?) allergic to(?: (?:again|now|then))?$"
);
rule!(
    RELATION_DIET_ASK,
    r"^what(?:'s| is| was) (?P<subject>.+?) diet(?: (?:again|now|then))?$"
);
rule!(
    SHARE_OWN,
    r"^\W*(?:(?:oh|yeah|yes|and|haha|honestly|sorry|actually)[\s,]+)*(?:(?:im|i'm|i am|me|i)\s*(?:too|also|as well|aswell|the same|same|not alone|have (?:it|that|this|the same)(?: too| as well)?|do too|am too|don'?t either|can'?t either|couldn'?t either|haven'?t either|can too|could too|do as well|am as well)|same here|same for me|same for us|same|same as (?:me|him|her|them|that|my [a-z]+)|me neither|likewise|(?:that'?s|this is) me(?: too| as well)?|so (?:am|do|does|did|have|has|can|could|would) i|neither (?:am|do|does|did|have|has|can|could) i|nor (?:am|do|have|can) i)\W*$"
);
rule!(
    SHARE_PERSON,
    r"^\W*(?:(?:oh|yeah|yes|and|haha|honestly|sorry|actually)[\s,]+)*(?:my|our) (?P<who>[a-z][a-z'-]{1,14}) (?:(?:is|are|does|do|has|have|gets|got|wants)\s*)?(?:too|also|as well|aswell|the same|same|it too|that too|one too)\W*$"
);
rule!(
    SHARE_REVERSED,
    r"^\W*(?:(?:oh|yeah|yes|and|haha|honestly|sorry|actually)[\s,]+)*(?:so|neither|nor) (?:does|do|is|are|has|have|can|could|would) (?:my|our) (?P<who>[a-z][a-z'-]{1,14})\W*$"
);
rule!(
    SHARE_LAST,
    r"^\W*(?:(?:oh|yeah|yes|and|haha|honestly|sorry|actually)[\s,]+)*(?:he|she|they) (?:(?:is|are|does|do|has|have)\s*)?(?:too|also|as well|aswell|the same|same|it too|that too)\W*$"
);
rule!(
    TAKE_BACK_SHARED,
    r"^(?:(?:and|so|oh|but|ok|okay|no|actually)[ ,]+)*(?:she|he|they)(?: is| are|'s|s)(?:nt|n't| not)\b(?P<rest>.*)$"
);

const STOP: &str = "i we he she they it you have has had got also but so because which and the a an my our his her their anything something everything much many most lot food stuff no not few several some any real bad serious";
const NOT_FOOD: &str = "late night evening morning afternoon work home there here bed enough much more before after school outside inside stop finish start";
const NOT_ALLERGEN: &str = "get gets getting got gotten have has had having develop develops developed developing catch catches caught catching treat treats treated treating prevent prevented test tests tested testing cause causes caused causing trigger triggers triggered avoid avoids avoiding manage manages managing notice notices noticed noticing control controls controlled risk suffer suffers suffered experience make makes made become becomes became";
const ONE_MORE_FOODS: &str = "
    milk dairy cheese butter cream yoghurt yogurt lactose egg eggs
    nut nuts peanut peanuts walnut walnuts hazelnut hazelnuts cashew cashews almond almonds
    pistachio pistachios pecan pecans brazil macadamia chestnut chestnuts pine
    wheat gluten flour bread pasta semolina spelt barley rye oats oat couscous noodles
    soya soy tofu sesame tahini mustard celery celeriac lupin corn maize rice potato potatoes
    fish salmon cod tuna haddock prawn prawns shellfish crab crabs lobster mussels oysters
    squid scallops clam clams anchovy anchovies sardine sardines mackerel trout
    meat chicken beef pork lamb bacon ham sausage sausages turkey duck venison gelatine
    gelatin yeast sugar honey chocolate coffee caffeine alcohol wine beer vinegar
    feta halloumi parmesan mozzarella cheddar brie ricotta paneer stilton gouda edam
    strawberry strawberries kiwi kiwis berry berries banana bananas apple apples
    pear pears peach peaches plum plums cherry cherries grape grapes melon melons
    mango mangoes pineapple avocado coconut tomato tomatoes onion onions garlic
    pepper spices spice curry chilli chili cinnamon cloves ginger mint
    sulphites sulphite sulfites sulfate sulphur msg aspartame sweetener
    preservatives colourings colouring dye colour color additives";
const ONE_MORE_DETERMINERS: &str = "the a an my your his her their our this that these those some";
const NOT_AN_ADDITION: &str =
    "not no dont don't didnt didn't doesnt doesn't scratch forget never nor without";
const NO_EAT_QUALIFIER: &str = "red white processed raw oily fatty farmed";
const MEAT: &str = "meat meats beef chicken pork lamb bacon ham sausages sausage fish seafood prawns shellfish turkey duck goose venison steak mince gelatine gelatin";
const DRINK: &str = "alcohol booze wine wines beer beers lager lagers cider spirits gin vodka whisky whiskey rum prosecco champagne cocktails cocktail sherry brandy";
const PLAN: &str = "keto paleo atkins mediterranean low-carb slimming-world weight-watchers intermittent-fasting whole30 whole-30 omad carnivore dukan south-beach fast-800 5:2 16:8 4:3";

#[derive(Clone, Debug)]
enum Diet {
    Way(String),
    Avoid { thing: String, drink: bool },
}

impl Diet {
    fn words(&self, third_plural: bool, third: bool) -> String {
        match self {
            Self::Way(way) if !third && is_word(PLAN, way) => {
                format!("you're doing {}", way.replace('-', " "))
            }
            Self::Way(way) if !third => format!("you're {}", way.replace('-', " ")),
            Self::Way(way) => format!("{} {way}", if third_plural { "are" } else { "is" }),
            Self::Avoid { thing, drink } => {
                let consume = if *drink { "drink" } else { "eat" };
                let verb = if third && !third_plural {
                    "doesn't"
                } else {
                    "don't"
                };
                format!(
                    "{}{verb} {consume} {thing}",
                    if third { "" } else { "you " }
                )
            }
        }
    }

    fn former_words(&self) -> String {
        match self {
            Self::Way(way) => format!("being {}", way.replace('-', " ")),
            Self::Avoid { thing, .. } => format!("not eating {thing}"),
        }
    }

    fn before_change(&self) -> String {
        match self {
            Self::Way(way) => format!("You were {way}"),
            Self::Avoid { thing, .. } => format!("You don't eat {thing}"),
        }
    }

    fn kept_words(&self) -> String {
        match self {
            Self::Way(way) => way.clone(),
            Self::Avoid { thing, .. } => format!("no {thing}"),
        }
    }
}

#[derive(Debug)]
struct DeniedDiet {
    way: String,
    any_more: bool,
    summary: String,
}

#[derive(Default, Debug)]
pub(super) struct Food {
    allergies: Option<Vec<String>>,
    diet: Option<Diet>,
    changed_diet: Option<Diet>,
    denied_diet: Option<DeniedDiet>,
    conditions: Vec<String>,
}

#[derive(Default, Debug)]
pub(super) struct Change {
    allergies: Vec<String>,
    removed: Vec<String>,
    contrast: bool,
    diet: Option<Diet>,
    changed_diet: bool,
    denied_diet: Option<DeniedDiet>,
    conditions: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct Limits {
    allergies: Vec<String>,
    diet: Option<Diet>,
    conditions: Vec<String>,
}

pub(super) enum ShareTarget<'a> {
    Own,
    Person(&'a str),
    Last,
}

pub(crate) enum FoodAck {
    Diet(String),
    Allergy(Vec<String>),
    RelativeAllergy {
        relation: &'static str,
        allergens: Vec<String>,
    },
    Condition(Vec<String>),
    Additive(Vec<String>),
}

pub(crate) fn food_ack(raw: &str) -> Option<FoodAck> {
    let text = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if let Some(found) = Change::from(&text, true) {
        if !found.allergies.is_empty() {
            return Some(FoodAck::Allergy(found.allergies));
        }
        if let Some(diet) = found.diet {
            return Some(FoodAck::Diet(diet.words(false, false)));
        }
        if !found.conditions.is_empty() {
            return Some(FoodAck::Condition(found.conditions));
        }
    }
    if let Some((relation, change)) = super::personal::relative_food(&text)
        && !change.allergies.is_empty()
    {
        return Some(FoodAck::RelativeAllergy {
            relation,
            allergens: change.allergies,
        });
    }
    let more = one_more(&text);
    (!more.is_empty()).then_some(FoodAck::Additive(more))
}

pub(super) fn sharing(line: &str) -> Option<ShareTarget<'_>> {
    if line.split_whitespace().count() > 6 {
        return None;
    }
    if SHARE_OWN.is_match(line) {
        return Some(ShareTarget::Own);
    }
    if let Some(found) = SHARE_PERSON
        .captures(line)
        .or_else(|| SHARE_REVERSED.captures(line))
    {
        return Some(ShareTarget::Person(field(&found, "who")?));
    }
    SHARE_LAST.is_match(line).then_some(ShareTarget::Last)
}

impl Limits {
    pub(super) fn respond(&self, role: Option<&str>, memory: &Memory) -> Option<String> {
        if let Some((last, other)) = self.allergies.split_last() {
            let named = join_list(self.allergies.iter().map(String::as_str));
            let are = if !other.is_empty() || (last.ends_with('s') && !last.ends_with("ss")) {
                "are"
            } else {
                "is"
            };
            let replies = if let Some(role) = role {
                [
                    format!(
                        "Then {named} {are} off your {role}'s plate too — I'll keep both of you in mind."
                    ),
                    format!("Right, {named} out for your {role} as well, then."),
                    format!(
                        "So {named} {are} off the table for your {role} too. Both of you, then."
                    ),
                ]
            } else {
                [
                    format!("Then {named} {are} off your plate too — I'll answer for both of you."),
                    format!("Right, {named} out for you as well, then."),
                    format!("So {named} {are} off the table for you too."),
                ]
            };
            return Some(pick(&replies, memory, ""));
        }
        if let Some(diet) = &self.diet {
            let way = diet.kept_words();
            if let Some(role) = role {
                return Some(format!(
                    "Then {way} for your {role} as well — I'll plan around the two of you."
                ));
            }
            let replies = [
                format!("Then {way} for you as well — I'll plan around the two of you."),
                format!("Same for you, then: {way}. I'll keep that straight."),
            ];
            return Some(pick(&replies, memory, ""));
        }
        self.conditions.first()?;
        let named = join_list(self.conditions.iter().map(String::as_str));
        if let Some(role) = role {
            let are = if named.ends_with('s') && !named.ends_with("ss") {
                "are"
            } else {
                "is"
            };
            let replies = [
                format!(
                    "Then your {role} {are} {named} too — I'll keep it in mind for both of you."
                ),
                format!("Right, {named} for your {role} as well, then. Both of you."),
                format!("So your {role} has that too. I'll keep the two of you in mind."),
            ];
            return Some(pick(&replies, memory, ""));
        }
        let replies = [
            format!("Then you have the same thing, {named} — I'll keep that in mind."),
            format!("Same condition for you too, then: {named}."),
        ];
        Some(pick(&replies, memory, ""))
    }
}

fn is_word(words: &str, word: &str) -> bool {
    words.split_whitespace().any(|stop| stop == word)
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    let initial: String = chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .collect();
    format!("{initial}{}", chars.as_str())
}

fn items(chunk: &str) -> Vec<String> {
    let chunk = LIST_END
        .find(chunk)
        .map_or(chunk, |found| &chunk[..found.start()]);
    LIST_SEP
        .split(chunk)
        .map(str::trim)
        .filter(|item| {
            item.len() > 1
                && !item
                    .split_whitespace()
                    .any(|word| is_word(STOP, word) || is_word(NOT_FOOD, word))
        })
        .map(str::to_owned)
        .collect()
}

fn plural(item: &str) -> &str {
    match item {
        "nut" => "nuts",
        "peanut" => "peanuts",
        "walnut" => "walnuts",
        "hazelnut" => "hazelnuts",
        "cashew" => "cashews",
        "almond" => "almonds",
        "prawn" => "prawns",
        "egg" => "eggs",
        "strawberry" => "strawberries",
        "kiwi" => "kiwis",
        "berry" => "berries",
        _ => item,
    }
}

fn one_more(line: &str) -> Vec<String> {
    let text = line
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let mut said = text.as_str();
    for _ in 0..6 {
        let body = ONE_MORE_HEAD
            .find(said)
            .map_or(said, |head| &said[head.end()..]);
        let body = body.trim_matches([' ', ',']);
        let body = ONE_MORE_TAIL
            .find(body)
            .map_or(body, |tail| &body[..tail.start()])
            .trim_matches([' ', ',', '.', '!', '?']);
        if body == said {
            break;
        }
        said = body;
    }
    if said.is_empty() || said == text {
        return Vec::new();
    }
    LIST_SEP
        .split(said)
        .filter_map(|part| {
            let mut words = part
                .split_whitespace()
                .map(|word| word.trim_matches(['\'', '’']))
                .filter(|word| !word.is_empty())
                .peekable();
            if is_word(NOT_AN_ADDITION, words.peek()?) {
                return None;
            }
            let words: Vec<_> = words
                .skip_while(|word| is_word(ONE_MORE_DETERMINERS, word))
                .collect();
            match words.as_slice() {
                [item] if is_word(ONE_MORE_FOODS, item) => Some(plural(item).into()),
                [first, item] if is_word(ONE_MORE_FOODS, item) => {
                    Some(format!("{first} {}", plural(item)))
                }
                _ => None,
            }
        })
        .collect()
}

fn own_subject(line: &str, at: usize) -> bool {
    let before = &line[..at];
    SUBJECT.find_iter(before).last().is_none_or(|found| {
        let word = found.as_str();
        word.starts_with('i') || word.starts_with("we")
    })
}

fn field<'a>(groups: &Captures<'a>, name: &str) -> Option<&'a str> {
    groups.name(name).map(|value| value.as_str())
}

fn restricted_diet(phrase: &str) -> Option<Diet> {
    let statement = EATING.captures(phrase.trim())?;
    let value = field(&statement, "diet")?;
    let found = DIET.find(value)?;
    if found.start() != 0 {
        return None;
    }
    let value = match found.as_str() {
        "veggie" => "vegetarian",
        word => word,
    };
    Some(Diet::Way(value.replace(' ', "-")))
}

fn avoids_eating(line: &str, own: bool) -> Option<Diet> {
    let pattern = if own { &OWN_NO_EAT } else { &NO_EAT };
    let found = pattern.captures(line)?;
    let foods = field(&found, "items")?;
    let items = items(foods);
    let food = items.first()?;
    let thing = if items.len() > 1 {
        foods.to_owned()
    } else if is_word(NO_EAT_QUALIFIER, food)
        && let Some(next) = line[found.get(0)?.end()..]
            .split_whitespace()
            .next()
            .map(|word| word.trim_end_matches(['.', '!', '?', ',', ';']))
            .filter(|next| is_word(MEAT, next))
    {
        format!("{food} {next}")
    } else {
        food.to_owned()
    };
    Some(Diet::Avoid {
        thing,
        drink: false,
    })
}

fn avoids_drinking(line: &str, own: bool) -> Option<Diet> {
    let thing = if !own {
        let found = NO_DRINK.captures(line)?;
        field(&found, "thing").unwrap_or("alcohol")
    } else if OWN_NO_DRINK.is_match(line) {
        "alcohol"
    } else {
        let found = OWN_NO_DRINK_THING.captures(line)?;
        let drink = field(&found, "thing")?;
        if !drink
            .split_whitespace()
            .all(|word| matches!(word, "or" | "and") || is_word(DRINK, word))
        {
            return None;
        }
        drink
    };
    Some(Diet::Avoid {
        thing: thing.into(),
        drink: true,
    })
}

fn way_of_eating(line: &str, own: bool) -> Option<Diet> {
    if !own {
        return restricted_diet(line);
    }
    if let Some(found) = DIET_UNDERWAY.captures(line) {
        let way = field(&found, "way").unwrap_or("diet");
        return Some(Diet::Way(if way == "diet" {
            "on a diet".into()
        } else {
            way.replace(' ', "-")
        }));
    }
    SUBJECT
        .find_iter(line)
        .filter(|found| {
            let word = found.as_str();
            word.starts_with('i') || word.starts_with("we")
        })
        .find_map(|found| restricted_diet(&line[found.end()..]))
}

impl Change {
    pub(super) fn limits(&self) -> Option<Limits> {
        let limits = Limits {
            allergies: self.allergies.clone(),
            diet: self.diet.clone(),
            conditions: self.conditions.clone(),
        };
        (!limits.allergies.is_empty() || limits.diet.is_some() || !limits.conditions.is_empty())
            .then_some(limits)
    }

    pub(super) fn from(line: &str, own: bool) -> Option<Self> {
        let line = line.to_lowercase().replace(['’', '‘'], "'");
        if (own && QUESTION.is_match(&line)) || line.ends_with('?') {
            return None;
        }
        let applies = |at| !own || own_subject(&line, at);
        let mut result = Self::default();
        for (index, pattern) in [&ALLERGIC, &CANNOT_EAT, &ALLERGY_NOUN]
            .into_iter()
            .enumerate()
        {
            for found in pattern.captures_iter(&line) {
                let whole = found.get(0)?;
                if !applies(whole.start()) {
                    continue;
                }
                let value = field(&found, "items")?;
                if index == 2 && is_word(NOT_ALLERGEN, value) {
                    continue;
                }
                let mut names = items(value);
                if index == 2 {
                    names = names.into_iter().map(|name| plural(&name).into()).collect();
                }
                if index != 1 && NEGATION.is_match(&line[..whole.start()]) {
                    result.removed.extend(names);
                } else {
                    result.allergies.extend(names);
                }
            }
        }
        if let Some(found) = CONTRAST.captures(&line)
            && let Some(whole) = found.get(0)
            && applies(whole.start())
            && !NEGATION.is_match(&line[..whole.start()])
        {
            result.removed.extend(items(field(&found, "old")?));
            result.contrast = !result.allergies.is_empty() && !result.removed.is_empty();
        }
        result.diet = avoids_eating(&line, own)
            .or_else(|| way_of_eating(&line, own))
            .or_else(|| avoids_drinking(&line, own));
        if own && result.diet.is_none() {
            result.changed_diet = DIET_CHANGE.is_match(&line);
            result.denied_diet = DENIED_DIET.captures(&line).and_then(|found| {
                let statement = field(&found, "way")?;
                let way = ANY_MORE
                    .find(statement)
                    .map_or(statement, |tail| statement[..tail.start()].trim_end());
                let diet = DIET.find(way).filter(|diet| diet.as_str() == way)?;
                let any_more = ANY_MORE.is_match(&line);
                let negative = field(&found, "negative")?;
                Some(DeniedDiet {
                    way: diet.as_str().into(),
                    any_more,
                    summary: format!("you said you are {negative} {statement}"),
                })
            });
        }
        result.conditions = CONDITION
            .find_iter(&line)
            .filter(|found| applies(found.start()) && !NEGATION.is_match(&line[..found.start()]))
            .map(|found| found.as_str().to_owned())
            .collect();
        (!result.allergies.is_empty()
            || !result.removed.is_empty()
            || result.diet.is_some()
            || result.changed_diet
            || result.denied_diet.is_some()
            || !result.conditions.is_empty())
        .then_some(result)
    }
}

impl Food {
    pub(super) fn observe(&mut self, reading: &Reading<'_>) -> (Option<String>, Option<Limits>) {
        let mut change = Change::from(&reading.clean, true).unwrap_or_default();
        let limits = change.limits();
        if change.allergies.is_empty() && self.has_allergy() {
            change.allergies = one_more(reading.raw);
        }
        (self.apply(change, true), limits)
    }

    pub(super) fn share(&mut self, limits: &Limits) {
        self.apply(
            Change {
                allergies: limits.allergies.clone(),
                diet: limits.diet.clone(),
                conditions: limits.conditions.clone(),
                ..Change::default()
            },
            false,
        );
    }

    pub(super) fn take_back_shared(&mut self, line: &str) -> Option<String> {
        let found = TAKE_BACK_SHARED.captures(line.trim_end_matches(['?', '.', '!']))?;
        let rest = field(&found, "rest")?;
        let matches = |fact: &str| {
            let mut words = rest.split_whitespace();
            let Some(first) = words.next() else {
                return true;
            };
            const PADDING: &str = "either at all as well really actually";
            if is_word(PADDING, first) {
                return words.all(|word| is_word(PADDING, word));
            }
            fact.starts_with(first)
        };
        if let Some(allergies) = &self.allergies
            && !allergies.is_empty()
        {
            if !matches("allergic") {
                return None;
            }
            let named = join_list(allergies.iter().map(String::as_str));
            self.allergies = None;
            return Some(format!("allergic to {named}"));
        }
        if let Some(diet) = &self.diet {
            let named = diet.kept_words();
            if !matches(&named) {
                return None;
            }
            self.diet = None;
            return Some(named);
        }
        let named = self.conditions.first()?;
        if !matches(named) {
            return None;
        }
        let named = named.clone();
        self.conditions.clear();
        Some(named)
    }

    pub(super) fn apply(&mut self, change: Change, memory_reply: bool) -> Option<String> {
        let mut removed = Vec::new();
        if let Some(allergies) = self.allergies.as_mut() {
            allergies.retain(|value| {
                if change
                    .removed
                    .iter()
                    .any(|gone| plural(gone) == plural(value))
                {
                    removed.push(value.clone());
                    false
                } else {
                    true
                }
            });
        }
        let contrast = change
            .contrast
            .then(|| (change.allergies.join(", "), change.removed.join(", ")));
        if !change.allergies.is_empty() {
            let saw = self.allergies.get_or_insert_with(Vec::new);
            for item in change.allergies {
                if !saw.contains(&item) {
                    saw.push(item);
                }
            }
        }
        let diet_reply = if change.changed_diet {
            let old = self.diet.take();
            let reply = old.as_ref().map_or_else(
                || "Good to know. What does a normal day look like now?".into(),
                |diet| {
                    format!(
                        "{}, so I'll drop that - what does a normal day look like now?",
                        diet.before_change()
                    )
                },
            );
            if old.is_some() {
                self.changed_diet = old;
            }
            Some(reply)
        } else if let Some(denied) = change.denied_diet {
            let dropped = matches!(&self.diet, Some(Diet::Way(way)) if way == &denied.way);
            if dropped {
                self.diet = None;
            }
            self.denied_diet = Some(denied);
            dropped.then(|| "Right — meat is back on the table, then.".into())
        } else if change.diet.is_some() {
            self.diet = change.diet;
            None
        } else {
            None
        };
        for condition in change.conditions {
            if !self.conditions.contains(&condition) {
                self.conditions.push(condition);
            }
        }
        if !memory_reply {
            return None;
        }
        if let Some((new, old)) = contrast {
            return Some(format!("{}, not {old} — got it.", capitalize(&new)));
        }
        if removed.is_empty() {
            return diet_reply;
        }
        Some(format!(
            "{} back on the table, then.",
            capitalize(&removed.join(", "))
        ))
    }

    pub(super) fn latest_allergen(&self) -> Option<&str> {
        self.allergies.as_ref()?.last().map(String::as_str)
    }

    pub(super) fn reported_allergens(&self) -> impl Iterator<Item = &str> {
        self.allergies.iter().flatten().map(String::as_str)
    }

    fn allergens(&self) -> Option<String> {
        let known = self.allergies.as_ref()?;
        (!known.is_empty()).then(|| known.join(", "))
    }

    pub(super) fn allergy_fact(&self, who: Option<&str>) -> Option<String> {
        let known = self.allergens()?;
        let name = who.map_or_else(
            || "you're".into(),
            |who| {
                let verb = if who.ends_with('s') { "are" } else { "is" };
                format!("your {who} {verb}")
            },
        );
        Some(format!("{name} allergic to {known}"))
    }

    pub(super) fn diet_fact(&self, who: Option<&str>) -> Option<String> {
        let diet = self.diet.as_ref()?;
        Some(who.map_or_else(
            || diet.words(false, false),
            |who| format!("your {who} {}", diet.words(who.ends_with('s'), true)),
        ))
    }

    pub(super) fn condition_fact(&self) -> Option<String> {
        (!self.conditions.is_empty()).then(|| format!("you're {}", self.conditions.join(", ")))
    }

    pub(super) fn denied_diet_summary(&self) -> Option<&str> {
        self.diet
            .is_none()
            .then_some(self.denied_diet.as_ref()?.summary.as_str())
    }

    pub(super) fn relation_reply(&self, role: &str, allergy: bool) -> Option<String> {
        if allergy {
            return Some(self.allergy_fact(Some(role)).map_or_else(
                || format!("You haven't told me about an allergy of your {role}."),
                |fact| format!("{}.", capitalize(&fact)),
            ));
        }
        Some(format!("{}.", capitalize(&self.diet_fact(Some(role))?)))
    }

    pub(super) fn respond(
        &self,
        reading: &Reading<'_>,
        memory: &Memory,
        other: Option<(&str, &Food)>,
    ) -> Option<String> {
        let ask = &reading.clean;
        if OWN_ALLERGY_ASK.is_match(ask) {
            if let Some(allergies) = self.allergens() {
                let plain = format!("You told me you're allergic to {allergies}.");
                if !memory.turns.iter().any(|turn| turn.bot == plain) {
                    return Some(plain);
                }
                let options = [
                    format!("It's {allergies} — that's what you said."),
                    format!("Still {allergies}."),
                    format!("Your allergy is {allergies}."),
                ];
                return Some(again::vary(&options.each_ref().map(String::as_str), memory));
            }
            if self.allergies.is_some() {
                return Some("Nothing on the list now.".into());
            }
            if let Some((who, food)) = other
                && let Some(allergies) = food.allergens()
            {
                return Some(format!(
                    "You haven't told me one — the allergy to {allergies} is your {who}'s, not yours."
                ));
            }
            return Some("You haven't mentioned an allergy to me.".into());
        }
        if DIET_ASK.is_match(ask) {
            return self
                .diet_fact(None)
                .or_else(|| self.allergy_fact(None))
                .or_else(|| self.condition_fact())
                .map(|fact| format!("You told me {fact}."))
                .or_else(|| {
                    self.changed_diet.as_ref().map(|diet| {
                        format!(
                            "You told me your diet had changed from {}. What is it now?",
                            diet.former_words()
                        )
                    })
                })
                .or_else(|| {
                    self.denied_diet.as_ref().map(|diet| {
                        let tail = if diet.any_more { " any more" } else { "" };
                        format!("You told me you're not {}{tail}.", diet.way)
                    })
                });
        }
        if CANT_EAT_ASK.is_match(ask)
            && self
                .conditions
                .iter()
                .any(|value| value == "lactose intolerant")
            && self.allergens().is_none()
        {
            return Some("Nothing with dairy in it — you're lactose intolerant.".into());
        }
        None
    }

    pub(super) fn has_allergy(&self) -> bool {
        self.allergies
            .as_ref()
            .is_some_and(|known| !known.is_empty())
    }

    pub(super) fn has_coeliac_disease(&self) -> bool {
        self.conditions.iter().any(|condition| {
            matches!(
                condition.as_str(),
                "coeliac" | "celiac" | "gluten intolerant"
            )
        })
    }

    pub(super) fn avoids_meat_or_fish(&self) -> bool {
        match &self.diet {
            Some(Diet::Way(way)) => matches!(
                way.as_str(),
                "vegetarian" | "veggie" | "vegan" | "plant-based" | "pescatarian"
            ),
            Some(Diet::Avoid {
                thing,
                drink: false,
            }) => {
                matches!(thing.as_str(), "meat" | "meat or fish" | "fish" | "seafood")
            }
            _ => false,
        }
    }

    pub(super) fn allows_plant_menu(&self, vegan: bool) -> bool {
        !self.has_allergy()
            && self.conditions.is_empty()
            && match &self.diet {
                None => true,
                Some(Diet::Way(way)) => match way.as_str() {
                    "vegetarian" | "veggie" | "pescatarian" => true,
                    "vegan" | "plant-based" => vegan,
                    _ => false,
                },
                Some(Diet::Avoid {
                    thing,
                    drink: false,
                }) => {
                    matches!(thing.as_str(), "meat" | "meat or fish" | "fish" | "seafood")
                        || vegan
                            && matches!(
                                thing.as_str(),
                                "milk" | "dairy" | "cheese" | "egg" | "eggs"
                            )
                }
                Some(Diet::Avoid { drink: true, .. }) => false,
            }
    }

    pub(super) fn has_limits(&self) -> bool {
        self.has_allergy() || self.diet.is_some() || !self.conditions.is_empty()
    }
}

pub(super) fn asked_relation(line: &str) -> Option<(&str, bool)> {
    if let Some(found) = RELATION_FOOD_ASK.captures(line) {
        return Some((field(&found, "subject")?, true));
    }
    let found = RELATION_DIET_ASK.captures(line)?;
    Some((field(&found, "subject")?, false))
}

#[cfg(test)]
mod tests {
    use super::{FoodAck, food_ack};

    #[test]
    fn dietary_tellings_keep_their_actual_subject_time_and_wording() {
        assert!(food_ack("Don't eat fish.").is_none());
        assert!(food_ack("I was vegetarian.").is_none());
        for (text, expected) in [
            ("I don't eat meat or fish", "you don't eat meat or fish"),
            ("I don't drink beer", "you don't drink beer"),
            ("I'm doing keto", "you're doing keto"),
        ] {
            assert!(
                matches!(food_ack(text), Some(FoodAck::Diet(said)) if said == expected),
                "{text}"
            );
        }
    }

    #[test]
    fn an_allergy_correction_keeps_other_allergens_and_people_separate() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My sister is allergic to shellfish.");
        bot.turn("I'm allergic to peanuts and milk.");
        bot.turn("Actually, I'm allergic to soya, not peanuts.");
        assert_eq!(
            bot.turn("What am I allergic to?").reply,
            "You told me you're allergic to milk, soya."
        );
        assert_eq!(
            bot.turn("What is my sister allergic to?").reply,
            "Your sister is allergic to shellfish."
        );
    }

    #[test]
    fn an_allergen_can_be_added_to_an_existing_list() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("Plus cheese");
        bot.turn("Im allergic to peanuts");
        bot.turn("And milk too");
        bot.turn("And my sister too");
        bot.turn("Soy");
        assert_eq!(
            bot.turn("What am I allergic to").reply,
            "You told me you're allergic to peanuts, milk."
        );
    }

    #[test]
    fn a_relative_allergy_does_not_answer_an_own_allergy_question() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My sister is allergic to shellfish.");
        let result = bot.turn("What am I allergic to?");
        assert_eq!(result.module, Some("memory_store"));
        assert_eq!(
            result.reply,
            "You haven't told me one — the allergy to shellfish is your sister's, not yours."
        );
        bot.turn("I'm lactose intolerant.");
        let result = bot.turn("What am I allergic to?");
        assert_eq!(result.module, Some("memory_store"));
        assert_eq!(
            result.reply,
            "Same as before: You haven't told me one — the allergy to shellfish is your sister's, not yours."
        );
    }

    #[test]
    fn an_explicit_share_copies_only_the_limit_from_the_previous_telling() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My son is allergic to nuts");
        assert_eq!(
            bot.turn("Im too").reply,
            "Then nuts are off your plate too — I'll answer for both of you."
        );
        assert_eq!(
            bot.turn("What am I allergic to?").reply,
            "You told me you're allergic to nuts."
        );

        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My son is allergic to nuts");
        bot.turn("It's raining");
        bot.turn("Im too");
        assert_eq!(
            bot.turn("What am I allergic to?").reply,
            "You haven't told me one — the allergy to nuts is your son's, not yours."
        );
    }

    #[test]
    fn a_relative_can_share_a_reported_fact_and_then_take_it_back() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("Im allergic to nuts");
        assert_eq!(
            bot.turn("And my wife is too").reply,
            "Then nuts are off your wife's plate too — I'll keep both of you in mind."
        );
        assert_eq!(
            bot.turn("She isnt").reply,
            "Your wife isn't allergic to nuts, then - I've taken that back off her."
        );
        assert_eq!(
            bot.turn("What is my wife allergic to?").reply,
            "You haven't told me about an allergy of your wife."
        );
    }

    #[test]
    fn a_recap_includes_reported_restrictions_without_a_previous_intention() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        for line in [
            "My name is Ana.",
            "I live in Leeds.",
            "I can't eat peanuts.",
            "My sister is Maya.",
            "I was going to paint the spare room but I haven't done it yet.",
            "I finished painting yesterday.",
        ] {
            bot.turn(line);
        }
        assert_eq!(
            bot.turn("what have I told you so far?").reply,
            "You've told me you're Ana; you live in Leeds; you're allergic to peanuts; your sister is called Maya; you said you finished painting yesterday."
        );
    }

    #[test]
    fn a_diet_retraction_and_a_diet_change_keep_different_histories() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("Im vegetarian");
        assert_eq!(
            bot.turn("Im not vegetarian anymore").reply,
            "Right — meat is back on the table, then."
        );
        assert_eq!(
            bot.turn("Whats my diet").reply,
            "You told me you're not vegetarian any more."
        );
        assert_eq!(
            bot.turn("What do you know about me").reply,
            "So far I know you said you are not vegetarian anymore."
        );

        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("Im vegetarian");
        assert_eq!(
            bot.turn("I changed my diet").reply,
            "You were vegetarian, so I'll drop that - what does a normal day look like now?"
        );
        assert_eq!(
            bot.turn("Whats my diet").reply,
            "You told me your diet had changed from being vegetarian. What is it now?"
        );
    }
}
