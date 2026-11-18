//! Query guards for translated knowledge builders in the pinned explains domain.

use regex::Regex;
use std::{cell::OnceCell, sync::LazyLock};

use crate::bot::{
    Exchange, Memory, Reading,
    understanding::{intent, kinds::Kind},
};

use super::{
    ANIMAL_STRUCK_CUES, ANIMALS_ON_THE_ROAD, DURATION_ASK, HIGH_GLUCOSE_PATTERN,
    JOB_OFFER_ASK_PATTERN, TRACKING_WAIT_PATTERN,
};
use super::{
    BIRD_LIFE_PATTERN, KIT_NEEDS, KIT_TRIP_PATTERN, NUTRITION_DOSE_PATTERN,
    NUTRITION_EDIBILITY_PATTERN, NUTRITION_GOING_OFF_PATTERN, NUTRITION_NOT_SPOILAGE_PATTERN,
    NUTRITION_VERDICT_PATTERN, WEIGHT_UNITS,
};
use super::{
    COOK_FOOD_WORDS, COOK_PHRASES, DISPOSAL_CONSUMABLE, DISPOSAL_CONTAINERS,
    DISPOSAL_MACHINE_PATTERN, DISPOSAL_ONLY_A_MACHINE, DISPOSAL_PLACEMENT_PATTERN,
};
use super::{
    COOKED_KEEP_ASK, COOKED_KEEP_COOKING_PATTERN, COOKING_BLANK_FOOD_PATTERN, COST_APPLIANCE_NAMES,
    COST_HOW_TO_PATTERN, COST_PRICE_ASK_PATTERN, COST_STEPS_ASK_PATTERN,
};
use super::{DEFINITELY_CLOTHES, GROCERY_AMOUNT_PATTERN, GROCERY_PRICE_PATTERN, PUT_ON_OTHER};
use super::{
    FOOD_BAD_IS_A_STATE, FOOD_BEFORE_OFF, FOOD_GOOD_SMELL, FOOD_NAMED, FOOD_OFF_STATE_ASKS,
    FOOD_OFF_WORDS, FOOD_SMELL_VERBS, tokens,
};
use super::{
    KINDS_ASKS_ME_PATTERN, KINDS_ASKS_WITH_A_VERB_PATTERN, KINDS_NAMED_A_USE_PATTERN,
    KINDS_PREDICATE_PATTERN, KINDS_WORDS,
};
use super::{
    SWAP_ASKS_FOR_A_SWAP, SWAP_MAKE_ASK, SWAP_REFUSED_FOR, SWAP_SAID, SWAP_STOPS, SWAP_TASTE_SAID,
};

#[derive(Clone, Copy, Debug)]
pub(super) enum Guard {
    Pass,
    Mechanism,
    Acronym,
    PetRoutine,
    Swap,
    RentRaise,
    WorkRaise,
    FreshFood,
    ClothingPutOn,
    ClothingWeather,
    ClothingDegrees,
    ClothingZip,
    ClothingMeal,
    ClothingInterview,
    ClothingHair,
    ClothingColour,
    ClothingGlasses,
    KeptAnimal,
    KeptFish,
    CookingSwap,
    CookingClock,
    CookingLeavesOut(&'static [&'static str]),
    JointReplacement,
    GroceryPrice,
    PaintSubject,
    CarNamed,
    HotelRoom,
    BookedPlace,
    NotWorkLeave,
    Baby(super::babyfood::Guard),
    StoppedFixture,
    HeatingGauge,
    MaterialSubject,
    ChildOldEnoughToEat,
    NotASurfaceMark,
    BookGenre,
    CourseChange,
    AppointmentBooking,
    AppointmentPreparationFollowup,
    StatinFollowup,
    FamilyFollowup(&'static str),
    ScienceMemory(&'static str),
    CityEats,
    GamePick,
    Prep(super::kitchen_prep::Gate),
    PetsShareDinner(bool),
    FunQuiz,
    FunFact,
    FunOnlyAsk,
    SleepAid(super::sleep_aids::Shape),
    SoftFruitCooking,
    FoodOutOfFridge,
    CookedRice,
    PanSeason,
    NotKept,
    KeepingCut,
    KeepingThawed,
    NamedProduce,
    LifeAdmin(super::life_admin::Guard),
    Text(fn(&Reading<'_>) -> bool),
    TechnologyWifi,
    CarGlass,
    CarDamp,
    CarPractical(crate::bot::knowledge::car_practical::Gate),
    BikeBatteryFollowup,
    CarFault(bool),
    Disposal,
    NotCookAsk,
    Typed(&'static [&'static str]),
    RecipePot,
    TradeCost(bool),
    FoodNamed,
    FoodCooked,
    PlantCare,
    Kinds,
    Kit,
    NotWeight,
    FoodVerdict,
    NotDose,
    All(&'static [Guard]),
    BodyRegion,
    BackRegion,
    PersonPart,
    PersonNail,
    OwnBack,
    NamedBodyPart,
    IngredientChoice(&'static str),
    PotatoChoice,
    NotShopPlace,
    NoFilmMood,
    WantScare,
    RefusedSwap,
}

pub(super) struct Context<'a> {
    pub(super) reading: &'a Reading<'a>,
    pub(super) earlier: &'a [Exchange],
    pub(super) memory: Option<&'a Memory>,
    pub(super) select_only: bool,
    pub(super) excluded_id: Option<&'static str>,
    typed: OnceCell<String>,
    mechanism: OnceCell<bool>,
    routine: OnceCell<bool>,
    spoiled: OnceCell<bool>,
    baby_line: OnceCell<super::babyfood::Line<'a>>,
    baby_guard: OnceCell<bool>,
    baby_reply: OnceCell<bool>,
    fixture: OnceCell<Option<&'static str>>,
}

impl<'a> Context<'a> {
    pub(super) fn new(reading: &'a Reading<'a>, earlier: &'a [Exchange]) -> Self {
        Self {
            reading,
            earlier,
            memory: None,
            select_only: false,
            excluded_id: None,
            typed: OnceCell::new(),
            mechanism: OnceCell::new(),
            routine: OnceCell::new(),
            spoiled: OnceCell::new(),
            baby_line: OnceCell::new(),
            baby_guard: OnceCell::new(),
            baby_reply: OnceCell::new(),
            fixture: OnceCell::new(),
        }
    }

    pub(super) fn read(&self) -> &str {
        &self.reading.clean
    }

    pub(super) fn baby_line(&self) -> &super::babyfood::Line<'a> {
        self.baby_line
            .get_or_init(|| super::babyfood::Line::new(self.reading))
    }

    pub(super) fn baby_recipient(&self, use_turns: bool) -> bool {
        let value = if use_turns {
            &self.baby_reply
        } else {
            &self.baby_guard
        };
        *value.get_or_init(|| super::babyfood::feeds_someone(self.baby_line(), self, use_turns))
    }

    pub(super) fn stopped_fixture(&self) -> Option<&'static str> {
        *self.fixture.get_or_init(|| {
            let typed = self.typed();
            stopped_fixture(if typed.is_empty() {
                self.reading.folded()
            } else {
                typed
            })
        })
    }

    fn typed(&self) -> &str {
        self.typed.get_or_init(|| self.reading.line.words.join(" "))
    }
}

#[derive(Clone, Copy)]
pub(super) enum Shape {
    ClinicalDigits,
    DueMonth,
    BodyPregnancyWeeks,
    Text(fn(&Reading<'_>) -> bool),
    AnimalStruck,
    TrackingWait,
    HighGlucose,
    LowBattery,
    JobOfferAsk,
    DurationAsk,
    RecipeAlone,
    BodyPartBlocked,
    DistinctKeyWords,
}

impl Shape {
    pub(super) fn matches(self, context: &Context<'_>, subject: &[&str]) -> bool {
        let line = context.typed();
        match self {
            Self::ClinicalDigits => super::clinical_cards::has_digits(line),
            Self::DueMonth => super::pregnancy_cards::gives_month(line),
            Self::BodyPregnancyWeeks => super::body_triage::pregnancy_weeks(line),
            Self::Text(probe) => probe(context.reading),
            Self::AnimalStruck => {
                let named =
                    |choices: &[&str]| line.split_whitespace().any(|word| choices.contains(&word));
                named(ANIMALS_ON_THE_ROAD) && named(ANIMAL_STRUCK_CUES)
            }
            Self::TrackingWait => TRACKING_WAIT.is_match(line),
            Self::HighGlucose => high_glucose_reading(line),
            Self::LowBattery => LOW_BATTERY
                .captures_iter(line)
                .any(|capture| capture[1].parse::<u16>().is_ok_and(|value| value <= 20)),
            Self::JobOfferAsk => JOB_OFFER_ASK.is_match(line),
            Self::DurationAsk => DURATION_ASK.iter().any(|ask| line.contains(ask)),
            Self::RecipeAlone => super::recipes::alone(line, subject),
            Self::BodyPartBlocked => super::body::part_stops_activity(line),
            Self::DistinctKeyWords => {
                let mut named = line
                    .split_whitespace()
                    .filter(|word| subject.contains(word));
                named
                    .next()
                    .is_some_and(|first| named.any(|word| word != first))
            }
        }
    }
}

static TRACKING_WAIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(TRACKING_WAIT_PATTERN).expect("valid parcel wait shape"));
static HIGH_GLUCOSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(HIGH_GLUCOSE_PATTERN).expect("valid glucose measurement shape"));

pub(super) fn high_glucose_reading(text: &str) -> bool {
    HIGH_GLUCOSE
        .captures_iter(text)
        .any(|value| value[1].parse::<f64>().is_ok_and(|value| value >= 15.0))
}
static LOW_BATTERY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(\d{1,3})\s*(?:%|percent)").expect("valid battery charge shape")
});
static JOB_OFFER_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(JOB_OFFER_ASK_PATTERN).expect("valid job decision shape"));

impl Guard {
    pub(super) fn passes(self, context: &Context<'_>, id: &str, subject: &[&str]) -> bool {
        match self {
            Guard::Pass => true,
            Guard::Mechanism => *context.mechanism.get_or_init(|| mechanism(context.read())),
            Guard::Acronym => acronym(context.read(), subject),
            Guard::PetRoutine => !*context.routine.get_or_init(|| routine(context.read())),
            Guard::Swap => swap(context.read(), id, subject),
            Guard::RentRaise => !NOT_A_RENT_RAISE.is_match(context.read()),
            Guard::WorkRaise => !TENANCY.is_match(context.read()),
            Guard::FreshFood => !*context
                .spoiled
                .get_or_init(|| food_has_gone_off(context.read())),
            Guard::ClothingPutOn => put_on_is_clothes(&context.reading.line.words),
            Guard::ClothingWeather => super::clothing::wear_for_weather(context.reading),
            Guard::ClothingDegrees => {
                super::clothing::temperature(context.reading).is_some()
                    && super::clothing::wear_for_weather(context.reading)
            }
            Guard::ClothingZip => super::clothing::zip_fault(context.reading),
            Guard::ClothingMeal => !super::clothing::date_night_meal(context.reading),
            Guard::ClothingInterview => !super::clothing::mentions_interview(context.reading),
            Guard::ClothingHair => super::clothing::hair_line(context.reading),
            Guard::ClothingColour => !super::clothing::chooses_colour(context.reading),
            Guard::ClothingGlasses => super::clothing::spectacles(context.reading),
            Guard::KeptAnimal => keeping_animal(context.reading),
            Guard::KeptFish => pet_fish(&context.reading.line.clean),
            Guard::CookingSwap => super::recipes::swap_not_step(&context.reading.line.clean),
            Guard::CookingClock => super::recipes::keeping_not_cooking(context.reading),
            Guard::CookingLeavesOut(stems) => {
                super::recipes::uses_required_ingredients(&context.reading.line.clean, stems)
            }
            Guard::JointReplacement => super::joints::replacement(context.reading),
            Guard::PaintSubject => paint_subject(context),
            Guard::CarNamed => car_named(context.reading.folded()),
            Guard::HotelRoom => hotel_room(context.typed()),
            Guard::BookedPlace => BOOKED_PLACE.is_match(context.typed()),
            Guard::NotWorkLeave => !work_leave(context),
            Guard::Baby(guard) => guard.passes(context),
            Guard::StoppedFixture => context.stopped_fixture().is_some(),
            Guard::PetsShareDinner(shares) => {
                super::pet_feeding::ate_the_other_pets_dinner(&context.reading.line.words) == shares
            }
            Guard::FunQuiz => context.reading.features.wants_a_quiz,
            Guard::FunFact => super::fun::not_counting(context.reading.folded()),
            Guard::FunOnlyAsk => super::fun::only_ask(context.reading.folded()),
            Guard::SleepAid(shape) => shape.passes(context.reading.raw),
            Guard::NotKept => context.reading.line.kept.is_none(),
            Guard::KeepingCut => super::storage::cold_cut(context.reading),
            Guard::KeepingThawed => super::storage::cold_thawed(context.reading),
            Guard::NamedProduce => super::storage::names_produce(context.reading),
            Guard::LifeAdmin(guard) => guard.passes(context.reading),
            Guard::Text(check) => check(context.reading),
            Guard::TechnologyWifi => super::technology_setup::wifi_context(context),
            Guard::MaterialSubject => super::material_quantities::is_subject(context.reading),
            Guard::ChildOldEnoughToEat => super::picky_eater::old_enough(context.reading),
            Guard::NotASurfaceMark => super::diet_plan::is_not_a_mark(context.reading),
            Guard::SoftFruitCooking => {
                super::cooking_basics::soft_fruit_cooking(&context.reading.line.clean)
            }
            Guard::FoodOutOfFridge => {
                super::cooking_basics::is_out_of_the_fridge(&context.reading.line.clean)
            }
            Guard::CookedRice => super::cooking_basics::rice_cooked(&context.reading.line.clean),
            Guard::PanSeason => super::cooking_basics::not_a_tv_season(context.reading),
            Guard::HeatingGauge => {
                context
                    .reading
                    .line
                    .words
                    .iter()
                    .any(|word| super::HOME_PRESSURE_WORDS.contains(&word.as_str()))
                    || HEATING_GAUGE.is_match(&context.reading.line.clean)
            }
            Guard::GroceryPrice => grocery_price(&context.reading.line.clean),
            Guard::BookGenre => !super::books::genre_is_named(context.reading),
            Guard::CourseChange => asks_to_change_course(context.reading),
            Guard::AppointmentBooking => super::appointment::can_book(context.reading),
            Guard::AppointmentPreparationFollowup => {
                super::appointment::follows_appointment(context.earlier)
            }
            Guard::StatinFollowup => {
                super::medicine_cards::last_line_named_a_statin(context.earlier)
            }
            Guard::FamilyFollowup(id) => {
                super::family_followups::guard(id, context.reading, context.earlier)
            }
            Guard::ScienceMemory(id) => {
                super::science_wellbeing::memory_guard(id, context.reading, context.earlier)
            }
            Guard::CityEats => super::city_guides::is_eating(context.reading),
            Guard::GamePick => super::games::pick_context(context.reading, context.earlier),
            Guard::Prep(gate) => gate.passes(context),
            Guard::CarDamp => crate::bot::knowledge::car_practical::car_damp(context.reading),
            Guard::CarPractical(gate) => gate.passes(context.reading, context.earlier),
            Guard::BikeBatteryFollowup => {
                crate::bot::knowledge::updated_guidance::bike_battery_context(
                    context.reading,
                    context.earlier,
                )
            }
            Guard::CarGlass => {
                let reading = context.reading;
                let words = reading.line.words.join(" ");
                crate::bot::understanding::car_glass::claim_applies(&format!(
                    "{} {} {} {words}",
                    reading.line.text, reading.line.clean, reading.raw,
                ))
            }
            Guard::CarFault(required) => context.reading.car_fault == required,
            Guard::Disposal => disposal(context.reading),
            Guard::NotCookAsk => !cook_ask(context.reading),
            Guard::Typed(words) => words
                .iter()
                .any(|word| context.reading.line.clean.contains(word)),
            Guard::RecipePot => super::recipes::not_already_made(context.reading),
            Guard::TradeCost(fence) => trade_cost(context.reading, subject, fence),
            Guard::FoodNamed => !COOKING_BLANK_FOOD.is_match(&context.reading.line.clean),
            Guard::PlantCare => super::garden::plant_care(context.reading),
            Guard::Kinds => asks_kinds(context.reading),
            Guard::Kit => {
                let reading = context.reading;
                reading
                    .line
                    .words
                    .iter()
                    .any(|word| KIT_NEEDS.contains(&word.as_str()))
                    && !KIT_TRIP.is_match(reading.folded())
            }
            Guard::NotWeight => !context
                .reading
                .numbers
                .iter()
                .any(|number| number.unit.is_some_and(|unit| WEIGHT_UNITS.contains(&unit))),
            Guard::FoodVerdict => food_verdict(context.reading),
            Guard::NotDose => !NUTRITION_DOSE.is_match(&context.reading.line.clean),
            Guard::All(guards) => guards
                .iter()
                .all(|guard| guard.passes(context, id, subject)),
            Guard::BodyRegion => super::body::region(context.reading, false),
            Guard::BackRegion => super::body::region(context.reading, true),
            Guard::PersonPart => super::body::person_part(context.reading),
            Guard::PersonNail => super::body::person_nail(context.reading),
            Guard::OwnBack => super::body::own_back(context.reading),
            Guard::NamedBodyPart => super::body::named_usable_part(context.reading),
            Guard::IngredientChoice(pattern) => super::picking::chooses(context.reading, pattern),
            Guard::PotatoChoice => super::picking::chooses_potato(context.reading),
            Guard::NotShopPlace => !super::calendar::asks_shop_place(context.reading),
            Guard::NoFilmMood => !super::entertainment::reads_mood(context.reading),
            Guard::WantScare => !super::entertainment::refuses_scare(context.reading),
            Guard::RefusedSwap => asks_for_refused_swap(&context.reading.line.clean),
            Guard::FoodCooked => {
                let text = &context.reading.line.clean;
                let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
                !COOKING_BLANK_FOOD.is_match(text)
                    && (COOKED_KEEP_ASK.iter().any(|word| line.contains(word))
                        || !COOKED_KEEP_COOKING.is_match(&line))
            }
        }
    }
}

fn asks_to_change_course(reading: &Reading<'_>) -> bool {
    let text = if reading.line.plain.is_empty() {
        &reading.line.text
    } else {
        &reading.line.plain
    };
    let cooked = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let said = format!(" {cooked} ");
    let mut bare = said.trim_matches([' ', '.', ',', '!', '?', ';', ':', '-', '…']);
    while let Some(rest) = super::COURSE_CHANGE_LEADS.iter().find_map(|lead| {
        let rest = bare.strip_prefix(lead)?;
        (rest.is_empty() || rest.starts_with(' ')).then_some(rest)
    }) {
        bare = rest.trim_matches([' ', ',']);
    }
    !super::COURSE_CHANGE_ANNOUNCEMENTS.contains(&bare)
        && super::COURSE_CHANGE_ASK
            .iter()
            .any(|ask| said.contains(&format!(" {ask} ")))
}

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(KIT_TRIP, &format!("(?i){KIT_TRIP_PATTERN}"));
pattern!(BIRD_LIFE, BIRD_LIFE_PATTERN);
pattern!(NUTRITION_DOSE, NUTRITION_DOSE_PATTERN);
pattern!(NUTRITION_GOING_OFF, NUTRITION_GOING_OFF_PATTERN);
pattern!(NUTRITION_NOT_SPOILAGE, NUTRITION_NOT_SPOILAGE_PATTERN);
pattern!(NUTRITION_VERDICT, NUTRITION_VERDICT_PATTERN);
pattern!(NUTRITION_EDIBILITY, NUTRITION_EDIBILITY_PATTERN);

pub(super) fn past_trip(context: &Context<'_>) -> bool {
    let recent = &context.earlier[context.earlier.len().saturating_sub(4)..];
    let joined = recent
        .iter()
        .map(|turn| turn.user.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    KIT_TRIP.is_match(&joined)
}

pub(super) fn food_is_being_kept(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let text = if line.norm_text.is_empty() {
        line.clean.as_str()
    } else {
        line.norm_text.as_str()
    };
    let text = format!(" {} ", text.to_lowercase());
    if !super::KEEPING_WORDS.iter().any(|word| text.contains(word)) {
        return false;
    }
    super::KEPT_FOODS.iter().any(|food| {
        if food.contains(' ') {
            text.contains(food)
        } else {
            line.words
                .iter()
                .any(|word| word.eq_ignore_ascii_case(food))
                || text.contains(&format!(" {food} "))
        }
    })
}

pub(super) fn only_a_container(reading: &Reading<'_>, names: &[&str]) -> bool {
    let text = format!(" {} ", reading.line.words.join(" ").to_lowercase());
    let mut names = names
        .iter()
        .filter(|name| text.contains(&format!(" {name} ")));
    names.next().is_some_and(|first| {
        super::CONTAINERS.contains(first) && names.all(|name| super::CONTAINERS.contains(name))
    })
}

static GARMENT_THEN_FOOD: LazyLock<Regex> = LazyLock::new(|| {
    let names = super::ODOUR_GARMENTS
        .iter()
        .map(|name| regex::escape(name))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(
        r"\b(?:{names})\s+(?:potato|potatoes|baked potato|jacket potato)\b"
    ))
    .expect("valid garment and food names")
});

pub(super) fn garment_is_subject(reading: &Reading<'_>) -> bool {
    let text = reading.line.words.join(" ").to_lowercase();
    !(GARMENT_THEN_FOOD.is_match(&text)
        || food_is_being_kept(reading) && only_a_container(reading, super::ODOUR_GARMENTS))
}

pub(super) fn asks_about_odour(reading: &Reading<'_>) -> bool {
    let text = reading.line.words.join(" ").to_lowercase();
    !super::ODOUR_SOURCE_WORDS
        .iter()
        .any(|word| text.contains(word))
        || super::ODOUR_SMELL_WORDS
            .iter()
            .any(|word| text.contains(word))
}

pub(super) fn names_bird_life(reading: &Reading<'_>) -> bool {
    BIRD_LIFE.is_match(&reading.line.clean)
}

fn food_verdict(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    !NUTRITION_GOING_OFF.is_match(clean)
        || (NUTRITION_NOT_SPOILAGE.is_match(clean)
            && NUTRITION_VERDICT.is_match(clean)
            && !NUTRITION_EDIBILITY.is_match(clean))
}

static KINDS_ASKS_ME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(KINDS_ASKS_ME_PATTERN).expect("valid second-person kinds question pattern")
});
static KINDS_ASKS_WITH_A_VERB: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(KINDS_ASKS_WITH_A_VERB_PATTERN).expect("valid kinds asking verbs pattern")
});
static KINDS_NAMED_A_USE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(KINDS_NAMED_A_USE_PATTERN).expect("valid kinds named use pattern"));
static KINDS_PREDICATE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(KINDS_PREDICATE_PATTERN).expect("valid kinds predicate pattern"));

fn asks_kinds(reading: &Reading<'_>) -> bool {
    let said = &reading.line.clean;
    if KINDS_ASKS_ME.is_match(said) || KINDS_NAMED_A_USE.is_match(said) {
        return false;
    }
    let lowered = said.split_whitespace().collect::<Vec<_>>().join(" ");
    let says_it = reading.kind != Kind::Question
        && !KINDS_ASKS_WITH_A_VERB.is_match(&lowered)
        && KINDS_PREDICATE.is_match(&lowered);
    let words = said.replace(['[', ']', ',', '\''], " ");
    let words = format!(
        " {} ",
        words.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    (!says_it && KINDS_WORDS.iter().any(|kind| words.contains(kind)))
        || (reading.wants == Some("options")
            && intent::asks_only_what_or_yesno(&reading.line, reading.kind))
}

static COST_HOW_TO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(COST_HOW_TO_PATTERN).expect("valid trade steps pattern"));
static COST_STEPS_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(COST_STEPS_ASK_PATTERN).expect("valid trade ask pattern"));
static COST_PRICE_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(COST_PRICE_ASK_PATTERN).expect("valid trade price pattern"));
static COOKING_BLANK_FOOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(COOKING_BLANK_FOOD_PATTERN).expect("valid unnamed food pattern"));
static COOKED_KEEP_COOKING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(COOKED_KEEP_COOKING_PATTERN).expect("valid keeping cooking verbs pattern")
});

fn trade_cost(reading: &Reading<'_>, subjects: &[&str], needs_price: bool) -> bool {
    fn names(said: &str, appliance: &str) -> bool {
        said.match_indices(appliance).any(|(start, hit)| {
            !said[..start].ends_with(|c: char| c.is_ascii_alphanumeric())
                && !said[start + hit.len()..].starts_with(|c: char| c.is_ascii_alphanumeric())
        })
    }

    let clean = &reading.line.clean;
    if COST_HOW_TO.is_match(clean)
        || (needs_price && COST_STEPS_ASK.is_match(clean) && !COST_PRICE_ASK.is_match(clean))
    {
        return false;
    }
    !COST_APPLIANCE_NAMES.iter().any(|appliance| {
        subjects.iter().any(|subject| {
            !subject.trim().is_empty()
                && subject.len() < appliance.len()
                && appliance.contains(subject)
        }) && (names(clean, appliance)
            || names(reading.folded(), appliance)
            || reading.line.norm.iter().any(|word| names(word, appliance)))
    })
}

fn cook_ask(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let said = if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    };
    COOK_PHRASES.iter().any(|phrase| said.contains(phrase))
        && COOK_FOOD_WORDS.iter().any(|food| said.contains(food))
}

static DISPOSAL_PLACEMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(DISPOSAL_PLACEMENT_PATTERN).expect("valid rubbish placement pattern")
});
static DISPOSAL_MACHINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(DISPOSAL_MACHINE_PATTERN).expect("valid rubbish machine pattern"));
static DISPOSAL_PUT_INTO: LazyLock<Regex> = LazyLock::new(|| {
    let containers = DISPOSAL_CONTAINERS
        .iter()
        .map(|name| regex::escape(name))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(concat!(
        r"\b(?:put|puts|putting|place|places|placing|keep|keeps|keeping|store|stores|storing|add|",
        r"adding|load|loading|pour|pouring|stick|sticking|poke|poking|stand|standing)\b",
        r"[^.;?!]{{0,40}}?\b(?:in|into|inside|on|onto|under|behind|beside|near)\s+",
        r"(?:the\s+|my\s+|our\s+|a\s+|an\s+|your\s+|its\s+)?(?:{containers})\b",
    ), containers = containers)).expect("valid rubbish container pattern")
});

fn disposal(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let said = [&line.plain, &line.clean, &line.norm_text]
        .into_iter()
        .find(|view| !view.is_empty())
        .map_or("", String::as_str);
    if cook_ask(reading) || DISPOSAL_PUT_INTO.is_match(said) {
        return false;
    }
    !(DISPOSAL_PLACEMENT.is_match(said)
        && (DISPOSAL_ONLY_A_MACHINE
            .iter()
            .any(|word| said.contains(word))
            || (DISPOSAL_MACHINE.is_match(said)
                && DISPOSAL_CONSUMABLE.iter().any(|word| said.contains(word)))))
}

static HOW_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bwhat (?:do|would|can|could|should|shall) (?:i|we) do\b|\bwhat now\b")
        .expect("valid how-to question pattern")
});
static MECHANISM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\bhow\b|\bwork\b|\bexplain\b|\bmean\b|\binside\b|\bactually\b|\bwhat (?:is|are|s)\b",
    )
    .expect("valid mechanism question pattern")
});
static ASK_BEFORE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:what's|whats|what|explained|explain|define|meaning of|stand for|spelled|spelling|spell|how do you spell)\b")
        .expect("valid acronym prefix pattern")
});
static ABOUT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+about\b").expect("valid acronym exclusion pattern"));
static ASK_AFTER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[^.!?]{0,24}?(?:mean|means|meaning|stand for|stands for|stood for|short for|spell|spelled|spelling)\b")
        .expect("valid acronym suffix pattern")
});

static ROUTINE_SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:and |so |well |oh )?(?:i|i(?:'m|ve|d)|we|we(?:'re|ve))\b")
        .expect("valid routine subject pattern")
});
static ROUTINE_SHAPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:every (?:day|week|month|morning|evening|afternoon|night|weekend|other day)|",
        r"every (?:mon|tues|wednes|thurs|fri|satur|sun)day|",
        r"each (?:day|week|morning|evening)|most (?:days|weeks|mornings|evenings)|",
        r"(?:once|twice|three times|four times|a few times) a (?:day|week|month)|",
        r"on (?:a|an) (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|",
        r"at the weekend)\b",
    ))
    .expect("valid routine frequency pattern")
});
static ROUTINE_FAULT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:pain|pains|\w*ache|\w*aches|aching|hurts?|hurting|sore|sick|ill|unwell|worse|worst|",
        r"worried|anxious|panic|cry|crying|cried|shout|shouting|shouted|argue|arguing|fight|",
        r"fighting|bleed|bleeding|blood|swollen|rash|fever|cough|coughing|vomit|vomiting|",
        r"dizzy|numb|sneeze|sneezing|stopped|stopping|wont stop|won't stop|leak|leaking|",
        r"broken|broke|craving|crave)\b",
    ))
    .expect("valid routine complaint pattern")
});
static ROUTINE_PAST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:used to|didn'?t use to|would (?:always |often )?\w+|back then|years ago|in those days)\b")
        .expect("valid past routine pattern")
});
static NOT_A_RENT_RAISE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\brais(?:e|ed|es|ing)\s+(?:a|an|the|my|our|your|their|any|some)?\s*",
        r"(?:complaints?|concerns?|issues?|problems?|matters?|questions?|grievances?|objections?|",
        r"disputes?|alarms?|glass|glasses|toast|voices?|prices?|funds?|money|children|kids|",
        r"awareness|profile|hell)\b",
    ))
    .expect("valid raised object pattern")
});
static TENANCY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:landlords?|landlad(?:y|ies)|letting agents?|letting agency|lettings agency",
        r"|tenants?|tenancy|tenancies|housing association|council house)\b",
    ))
    .expect("valid tenancy terms")
});
static GROCERY_AMOUNT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(GROCERY_AMOUNT_PATTERN).expect("valid grocery amount pattern"));
static GROCERY_PRICE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(GROCERY_PRICE_PATTERN).expect("valid grocery price pattern"));

pub(super) fn mechanical_spring(reading: &Reading<'_>) -> bool {
    const GARDEN: &[&str] = &[
        "garden",
        "gardens",
        "lawn",
        "grass",
        "allotment",
        "allotments",
        "plot",
        "borders",
        "hedge",
        "hedges",
        "flowerbed",
        "flowerbeds",
        "mow",
        "mowing",
    ];
    !reading
        .topic
        .is_some_and(|(topic, _)| topic == "garden-and-plants")
        && !reading
            .topics
            .iter()
            .any(|&(topic, _)| topic == "garden-and-plants")
        && !reading
            .line
            .clean
            .split(|character: char| {
                !(character.is_ascii_lowercase() || character.is_ascii_digit() || character == '\'')
            })
            .any(|word| GARDEN.contains(&word))
}

pub(super) fn beans_available(reading: &Reading<'_>) -> bool {
    static SHORTAGE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(super::CHILLI_SHORTAGE).expect("authored chilli shortage regex")
    });
    let words = if reading.line.words.is_empty() {
        &reading.line.norm
    } else {
        &reading.line.words
    };
    SHORTAGE.captures_iter(&words.join(" ")).all(|captures| {
        captures[1].split_whitespace().take(3).any(|word| {
            super::CHILLI_BEAN_STEMS
                .iter()
                .any(|stem| word.starts_with(stem))
        })
    })
}

fn mechanism(read: &str) -> bool {
    !HOW_TO.is_match(read) || MECHANISM.is_match(read)
}

fn routine(read: &str) -> bool {
    !read.contains('?')
        && ROUTINE_SUBJECT.is_match(read)
        && ROUTINE_SHAPE.is_match(read)
        && !ROUTINE_FAULT.is_match(read)
        && !ROUTINE_PAST.is_match(read)
}

pub(super) fn put_on_is_clothes(words: &[String]) -> bool {
    let flat = words.join(" ");
    let puts_on = ["put on", "puts on", "putting on"]
        .iter()
        .any(|phrase| flat.contains(phrase));
    !puts_on
        || words
            .iter()
            .any(|word| DEFINITELY_CLOTHES.contains(&word.as_str()))
        || !words
            .iter()
            .any(|word| PUT_ON_OTHER.contains(&word.as_str()))
}

fn whole_phrases(phrases: &[&str]) -> Regex {
    let pattern = phrases
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"(?i)\b(?:{pattern})\b")).expect("valid guarded phrases")
}

fn keeping_animal(reading: &Reading<'_>) -> bool {
    static AVOID: LazyLock<Regex> = LazyLock::new(|| whole_phrases(super::KEPT_ANIMAL_AVOID));
    static ASKS: LazyLock<Regex> = LazyLock::new(|| whole_phrases(super::KEPT_ANIMAL_ASKS));
    !AVOID.is_match(reading.raw) && ASKS.is_match(reading.raw)
}

pattern!(BOOKED_PLACE, super::CONSUMER_BOOKED_PLACE_PATTERN);
pattern!(HEATING_GAUGE, super::HOME_GAUGE_PATTERN);
pattern!(DECORATED_SURFACE, super::DECORATED_SURFACE_PATTERN);
static PAINT: LazyLock<Regex> = LazyLock::new(|| whole_phrases(super::DECORATING_PAINT_WORDS));

fn stopped_fixture(typed: &str) -> Option<&'static str> {
    static CHECKS: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
        super::HOME_FIXTURE_CHECKS
            .iter()
            .map(|&(names, reply)| {
                let phrases = names
                    .iter()
                    .map(|name| format!("{}s?", regex::escape(name)))
                    .collect::<Vec<_>>()
                    .join("|");
                (
                    Regex::new(&format!(r"\b(?:{phrases})\b"))
                        .expect("valid stopped fixture names"),
                    reply,
                )
            })
            .collect()
    });
    if !super::HOME_FIXTURE_STOPPED
        .iter()
        .any(|word| typed.contains(word))
    {
        return None;
    }
    CHECKS
        .iter()
        .find_map(|(names, reply)| names.is_match(typed).then_some(*reply))
}

fn paint_subject(context: &Context<'_>) -> bool {
    let typed = context.typed();
    let reading = context.reading;
    let normalized = &reading.line.norm_text;
    let normalized = if normalized.is_empty() {
        reading.folded()
    } else {
        normalized
    };
    let padded = format!(" {normalized} ");
    let food_keeping = super::KEEPING_WORDS
        .iter()
        .any(|word| normalized.contains(word))
        && super::KEPT_FOODS.iter().any(|food| {
            if food.contains(' ') {
                normalized.contains(food)
            } else {
                reading.line.words.iter().any(|word| word == food)
                    || padded.contains(&format!(" {food} "))
            }
        });
    !food_keeping && (PAINT.is_match(typed) || DECORATED_SURFACE.is_match(typed))
}

fn car_named(text: &str) -> bool {
    let words = text
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '%'))
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    let joined = words.join(" ");
    super::CAR_NAMED_WORDS
        .iter()
        .any(|name| words.contains(name) || (name.contains(' ') && joined.contains(name)))
}

fn hotel_room(typed: &str) -> bool {
    let has = |phrases: &[&str]| phrases.iter().any(|phrase| typed.contains(phrase));
    if has(&["en suite", "ensuite", "wet room", "shower room"]) {
        return false;
    }
    has(super::CONSUMER_HOTEL_ROOM_WORDS)
        || !(has(super::CONSUMER_HOME_ROOM_WORDS)
            || (has(super::CONSUMER_DAMP_WORDS) && has(super::CONSUMER_FIX_WORDS)))
}

fn work_leave(context: &Context<'_>) -> bool {
    let text = format!(
        "{} {} {}",
        context.reading.folded(),
        context.reading.line.clean,
        context.typed()
    );
    let text = text.replace('\'', "").replace(',', " ");
    let text = format!(
        " {} ",
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    let has = |phrases: &[&str]| {
        phrases
            .iter()
            .any(|word| text.contains(&format!(" {word} ")))
    };
    !has(&[
        "flight",
        "flights",
        "airline",
        "airport",
        "hotel",
        "resort",
        "package",
        "abroad",
        "tour operator",
        "travel agent",
        "booking reference",
        "atol",
    ]) && has(super::CONSUMER_EMPLOYER_WORDS)
}

fn pet_fish(line: &str) -> bool {
    let mut words = line
        .split(|character: char| !(character.is_ascii_lowercase() || character == '\''))
        .filter(|word| !word.is_empty());
    words
        .clone()
        .any(|word| super::PETS_FISH_CARE.contains(&word))
        || !words.any(|word| super::PETS_FISHING_SPORT.contains(&word))
}

fn grocery_price(read: &str) -> bool {
    !read.is_empty() && !GROCERY_AMOUNT.is_match(read) && GROCERY_PRICE.is_match(read)
}

pub(super) fn food_has_gone_off(read: &str) -> bool {
    let words = tokens(read);
    if !words.iter().any(|word| FOOD_NAMED.contains(&word.as_str())) {
        return false;
    }
    if FOOD_OFF_STATE_ASKS
        .iter()
        .any(|phrase| read.contains(phrase))
    {
        return true;
    }
    for (position, word) in words.iter().enumerate() {
        if !FOOD_NAMED.contains(&word.as_str()) {
            continue;
        }
        let tail = &words[position + 1..];
        let Some(index) = tail
            .iter()
            .position(|word| !FOOD_BEFORE_OFF.contains(&word.as_str()))
        else {
            continue;
        };
        let state = tail[index].as_str();
        if state == "bad"
            && !tail[..=index]
                .iter()
                .any(|word| FOOD_BAD_IS_A_STATE.contains(&word.as_str()))
        {
            continue;
        }
        if FOOD_OFF_WORDS.contains(&state)
            || (FOOD_SMELL_VERBS.contains(&state)
                && tail
                    .get(index + 1)
                    .is_none_or(|next| !FOOD_GOOD_SMELL.contains(&next.as_str())))
        {
            return true;
        }
    }
    false
}

fn swapped_words<'a>(words: &'a [&'a str]) -> impl Iterator<Item = &'a str> {
    words
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| *pair == ["instead", "of"])
        .flat_map(|(start, _)| {
            words[start + 2..]
                .iter()
                .take_while(|word| !SWAP_STOPS.contains(word))
        })
        .copied()
}

pub(super) fn chilli_swap(read: &str) -> &'static str {
    static SHORT_OF_POWDER: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(?:no|without|out of|none left of)\s+(?:\w+\s+)?chilli (?:powder|flakes)\b")
            .expect("valid chilli shortage pattern")
    });
    let spaced = read.replace([',', '.'], " ");
    let words = spaced.split_whitespace().collect::<Vec<_>>();
    let powder =
        swapped_words(&words).any(|word| word.contains("powder") || word.contains("flakes"));
    if powder || SHORT_OF_POWDER.is_match(read) {
        "Paprika with a pinch of cayenne is the closest thing to chilli powder: about four parts paprika to one of cayenne, and a little more cayenne if you want it hot. For flakes, crushed dried chillies or a pinch of cayenne do the job."
    } else {
        "A quarter teaspoon of chilli powder or a pinch of flakes stands in for one fresh chilli, and hot sauce works in a sauce rather than where the chilli is sliced over the top."
    }
}

fn asks_for_refused_swap(read: &str) -> bool {
    let read = format!(" {read} ");
    SWAP_ASKS_FOR_A_SWAP
        .iter()
        .any(|phrase| read.contains(phrase))
        && !["instead of", "in place of", "substitute for"]
            .iter()
            .any(|phrase| {
                read.find(phrase).is_some_and(|at| {
                    SWAP_REFUSED_FOR
                        .iter()
                        .any(|(name, _)| read[at..].contains(name))
                })
            })
}

pub(super) fn refused_swap(reading: &Reading<'_>) -> Option<&'static str> {
    if !asks_for_refused_swap(&reading.line.clean) {
        return None;
    }
    let said = format!(" {} ", reading.line.words.join(" "));
    SWAP_REFUSED_FOR
        .iter()
        .find_map(|&(name, reply)| said.contains(&format!(" {name} ")).then_some(reply))
}

fn swap(read: &str, id: &str, subject: &[&str]) -> bool {
    let spaced = read.replace([',', '.'], " ");
    let words = spaced.split_whitespace().collect::<Vec<_>>();
    let replaced = swapped_words(&words).collect::<Vec<_>>();
    if !replaced.is_empty()
        && !subject
            .iter()
            .flat_map(|name| name.split_whitespace())
            .any(|own| {
                replaced.iter().any(|word| {
                    own.to_lowercase().trim_end_matches('s') == word.trim_end_matches('s')
                })
            })
    {
        return false;
    }
    let matches = |phrases: &[&str]| phrases.iter().any(|phrase| read.contains(phrase));
    if id != "swap-what-i-will-not-eat"
        && matches(SWAP_TASTE_SAID)
        && matches(SWAP_ASKS_FOR_A_SWAP)
        && subject
            .iter()
            .filter(|name| !name.contains(' '))
            .any(|name| {
                read.split_whitespace()
                    .any(|word| word == name.to_lowercase())
            })
    {
        return false;
    }
    !matches(SWAP_MAKE_ASK) || matches(SWAP_SAID)
}

fn acronym(read: &str, subject: &[&str]) -> bool {
    if !subject.iter().any(|spelling| as_written(read, spelling)) {
        return false;
    }
    subject
        .iter()
        .filter(|spelling| !spelling.trim().is_empty())
        .any(|spelling| {
            let spelling = spelling.to_lowercase();
            read.match_indices(&spelling).any(|(start, matched)| {
                let head = &read[..start];
                let tail = &read[start + matched.len()..];
                if head
                    .bytes()
                    .next_back()
                    .is_some_and(|byte| byte.is_ascii_alphanumeric())
                    || tail
                        .bytes()
                        .next()
                        .is_some_and(|byte| byte.is_ascii_alphanumeric())
                {
                    return false;
                }
                let before = head.rsplit(['.', '!', '?']).next().unwrap_or("");
                if ASK_BEFORE.find_iter(before).any(|ask| {
                    let gap = &before[ask.end()..];
                    if matches!(ask.as_str(), "what" | "whats" | "what's") && ABOUT.is_match(gap) {
                        return false;
                    }
                    gap.chars().count() <= 40
                        || (ask.as_str() == "what"
                            && [" do", " does", " is", " are"].iter().any(|verb| {
                                gap.strip_prefix(verb).is_some_and(|remaining| {
                                    remaining
                                        .starts_with(|c: char| !c.is_alphanumeric() && c != '_')
                                        && remaining.chars().count() <= 40
                                })
                            }))
                }) || ASK_AFTER.is_match(tail)
                {
                    return true;
                }
                let trim = |character| matches!(character, ' ' | '.' | '!' | '?' | ',' | '-');
                head.trim_matches(trim).is_empty() && tail.trim_matches(trim).is_empty()
            })
        })
}

fn as_written(read: &str, spelling: &str) -> bool {
    if spelling.is_empty() {
        return true; // An empty single alias matches at a boundary in the source's padded text.
    }
    read.match_indices(spelling).any(|(index, matched)| {
        let left = read[..index].bytes().next_back();
        let right = read[index + matched.len()..].bytes().next();
        if spelling.contains(' ') {
            left.is_none_or(|byte| byte == b' ') && right.is_none_or(|byte| byte == b' ')
        } else {
            left.is_none_or(|byte| !byte.is_ascii_lowercase())
                && right.is_none_or(|byte| !byte.is_ascii_lowercase())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stopped_fixture_requires_an_actual_failure_and_a_named_fixture() {
        let reading = Reading::new("my bathroom tap stopped working");
        let context = Context::new(&reading, &[]);
        assert!(Guard::StoppedFixture.passes(&context, "", &[]));
        assert!(
            context
                .stopped_fixture()
                .is_some_and(|reply| reply.contains("an isolator"))
        );
        assert!(stopped_fixture("how do i fit a new tap").is_none());
        assert!(stopped_fixture("my printer stopped working").is_none());
        let reading = Reading::new("which drop bars should i get for my bike");
        assert!(!Guard::HeatingGauge.passes(&Context::new(&reading, &[]), "", &[]));
    }

    #[test]
    fn mechanism_and_word_guards_reject_unrelated_asks() {
        assert!(!mechanism("my dog hates the hoover what do i do"));
        assert!(mechanism(
            "explain why my hoover works that way, what do i do"
        ));
        assert!(acronym("what does btw mean", &["btw"]));
        assert!(acronym(
            "what's the difference between a url and a link",
            &["url"]
        ));
        assert!(!acronym(
            "ok thanks. btw what should i watch tonight",
            &["btw"]
        ));
        assert!(!acronym(
            "what about only in the evenings wifi keep dropping",
            &["wifi"]
        ));
        assert!(!acronym("what should i be paying", &["paye"]));
    }

    #[test]
    fn swap_uses_the_ingredient_being_replaced_and_routine_has_a_person_as_subject() {
        assert!(swap(
            "can i use butter instead of oil",
            "swap-oil",
            &["oil"]
        ));
        assert!(!swap(
            "can i use butter instead of oil",
            "swap-butter",
            &["butter"]
        ));
        assert!(!swap(
            "what can i make for dinner without any cheese",
            "swap-cheese",
            &["cheese"]
        ));
        assert!(routine("i walk the dog every day"));
        assert!(!routine("my dog barks every day"));
        assert!(!routine("i get headaches most days"));
    }

    #[test]
    fn disposal_excludes_food_requests_and_placement_in_appliances() {
        assert!(disposal(&Reading::new("where do I put an old dishwasher?")));
        assert!(!disposal(&Reading::new(
            "where do I put salt in a dishwasher?"
        )));
        assert!(!disposal(&Reading::new(
            "what can I do with spinach in the fridge?"
        )));
    }

    #[test]
    fn trade_prices_exclude_instructions_and_parts_inside_named_appliances() {
        let allowed = |line| trade_cost(&Reading::new(line), &["heat pump"], false);
        assert!(allowed("how much does a heat pump cost?"));
        assert!(!allowed("how much does a heat pump dryer cost?"));
        assert!(!allowed("how to fit a heat pump"));
    }
}
