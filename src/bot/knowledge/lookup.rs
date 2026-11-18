//! Indexed matching for translated PyBot knowledge cards.

use crate::bot::understanding::{
    breeds,
    normalize::{engine_reads, forms, one_edit, stem, tokens},
};
use crate::bot::{Memory, Reading, compose};
use simple_pantry::PantryDish;
use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

#[path = "index_storage.rs"]
mod index_storage;
use index_storage::{Arena, Group, Key, Phrase, PostingIndex, Span, StoredPhrase, Word};

#[path = "animals.rs"]
mod animals;
#[path = "appointment.rs"]
mod appointment;
#[path = "authored_rank.rs"]
mod authored_rank;
#[path = "babyfood.rs"]
mod babyfood;
#[path = "body.rs"]
mod body;
#[path = "body_care.rs"]
mod body_care;
#[path = "body_triage.rs"]
mod body_triage;
#[path = "books.rs"]
mod books;
#[path = "calendar.rs"]
mod calendar;
#[path = "child_bedtime.rs"]
mod child_bedtime;
#[path = "city_guides.rs"]
mod city_guides;
#[path = "cleaning_guards.rs"]
mod cleaning_guards;
#[path = "cleaning_stains.rs"]
mod cleaning_stains;
#[path = "clinical_cards.rs"]
mod clinical_cards;
#[path = "clothing.rs"]
mod clothing;
#[path = "clothing_care.rs"]
mod clothing_care;
#[path = "clothing_lenses.rs"]
mod clothing_lenses;
#[path = "clothing_wear.rs"]
mod clothing_wear;
#[path = "complaint_cards.rs"]
mod complaint_cards;
#[path = "cooking_basics.rs"]
mod cooking_basics;
#[path = "cooking_cards.rs"]
mod cooking_cards;
#[path = "cooking_crowds.rs"]
mod cooking_crowds;
#[path = "cooking_extras.rs"]
mod cooking_extras;
#[path = "cooking_fats.rs"]
mod cooking_fats;
#[path = "cooking_methods.rs"]
mod cooking_methods;
#[path = "cooking_nutrition.rs"]
mod cooking_nutrition;
#[path = "cooking_pairings.rs"]
mod cooking_pairings;
#[path = "cooking_storage.rs"]
mod cooking_storage;
#[path = "craft.rs"]
mod craft;
#[path = "dates.rs"]
mod dates;
#[path = "device_cards.rs"]
mod device_cards;
#[path = "diet_nutrition.rs"]
mod diet_nutrition;
#[path = "diet_plan.rs"]
mod diet_plan;
#[path = "dietary_menus.rs"]
pub(crate) mod dietary_menus;
#[path = "differences.rs"]
mod differences;
#[path = "dish_choices.rs"]
mod dish_choices;
#[path = "dish_plan.rs"]
mod dish_plan;
#[path = "dish_texture.rs"]
mod dish_texture;
#[path = "drinks.rs"]
mod drinks;
#[path = "entertainment.rs"]
mod entertainment;
#[path = "entertainment_programmes.rs"]
mod entertainment_programmes;
#[path = "entertainment_time.rs"]
mod entertainment_time;
#[path = "entertainment_titles.rs"]
mod entertainment_titles;
#[path = "everyday_health.rs"]
mod everyday_health;
#[path = "family_cards.rs"]
mod family_cards;
#[path = "family_followups.rs"]
mod family_followups;
#[path = "family_guards.rs"]
mod family_guards;
#[path = "family_repair.rs"]
mod family_repair;
#[path = "film_genres.rs"]
mod film_genres;
#[path = "film_shelves.rs"]
mod film_shelves;
#[path = "food_fold.rs"]
mod food_fold;
#[path = "freshness_cards.rs"]
mod freshness_cards;
#[path = "fruit_cards.rs"]
mod fruit_cards;
#[path = "fun.rs"]
mod fun;
#[path = "games.rs"]
mod games;
#[path = "garden.rs"]
mod garden;
#[path = "garden_faults.rs"]
mod garden_faults;
#[path = "garden_more.rs"]
mod garden_more;
#[path = "garden_seasons.rs"]
mod garden_seasons;
#[path = "garments.rs"]
mod garments;
#[path = "gift_cards.rs"]
mod gift_cards;
#[path = "guards.rs"]
mod guards;
#[path = "health_checks.rs"]
mod health_checks;
#[path = "history_cards.rs"]
mod history_cards;
#[path = "home_appliance.rs"]
mod home_appliance;
#[path = "home_consumables.rs"]
mod home_consumables;
#[path = "home_equipment.rs"]
mod home_equipment;
#[path = "home_fittings.rs"]
mod home_fittings;
#[path = "home_heating.rs"]
mod home_heating;
#[path = "home_household.rs"]
mod home_household;
#[path = "home_maintenance.rs"]
mod home_maintenance;
#[path = "home_practical.rs"]
mod home_practical;
#[path = "home_repair.rs"]
mod home_repair;
#[path = "home_upkeep.rs"]
mod home_upkeep;
#[path = "instruments.rs"]
mod instruments;
#[path = "joints.rs"]
mod joints;
#[path = "kitchen_measures.rs"]
mod kitchen_measures;
#[path = "kitchen_prep.rs"]
mod kitchen_prep;
#[path = "landscape_materials.rs"]
mod landscape_materials;
#[path = "language_terms.rs"]
mod language_terms;
#[path = "leading.rs"]
mod leading;
#[path = "leisure.rs"]
mod leisure;
#[path = "life_admin.rs"]
mod life_admin;
#[path = "material_care.rs"]
mod material_care;
#[path = "material_quantities.rs"]
mod material_quantities;
#[path = "maths_registry.rs"]
mod maths_registry;
#[path = "meal_scenes.rs"]
mod meal_scenes;
#[path = "meal_windows.rs"]
mod meal_windows;
#[path = "medicine_cards.rs"]
mod medicine_cards;
#[path = "monthly_cost.rs"]
mod monthly_cost;
#[path = "mood_cards.rs"]
mod mood_cards;
#[path = "music.rs"]
mod music;
#[path = "music_context.rs"]
mod music_context;
#[path = "nature_cards.rs"]
mod nature_cards;
#[path = "newborn_cards.rs"]
mod newborn_cards;
#[path = "occasion_food.rs"]
mod occasion_food;
#[path = "opinions.rs"]
mod opinions;
#[path = "painting_faults.rs"]
mod painting_faults;
#[path = "pantry_intake.rs"]
mod pantry_intake;
#[path = "pantry_names.rs"]
mod pantry_names;
#[path = "pregnancy_cards.rs"]
mod pregnancy_cards;
#[path = "science_wellbeing.rs"]
mod science_wellbeing;
#[path = "shopping_comparisons.rs"]
mod shopping_comparisons;
#[path = "still_edible.rs"]
mod still_edible;
#[path = "stock_swaps.rs"]
mod stock_swaps;
#[path = "travel_health_cards.rs"]
mod travel_health_cards;
pub(crate) use pantry_intake::PantryIntake;
#[path = "pasta.rs"]
mod pasta;
#[path = "pay_cards.rs"]
mod pay_cards;
#[path = "pet_costs.rs"]
mod pet_costs;
#[path = "pet_feeding.rs"]
mod pet_feeding;
#[path = "pet_food.rs"]
mod pet_food;
#[path = "pet_health.rs"]
mod pet_health;
#[path = "pet_routines.rs"]
mod pet_routines;
#[path = "pet_symptoms.rs"]
mod pet_symptoms;
pub(in crate::bot) use pet_symptoms::BREATHING_EMERGENCY as PET_BREATHING_EMERGENCY;
#[path = "pet_wellbeing.rs"]
mod pet_wellbeing;
#[path = "pets_care.rs"]
mod pets_care;
#[path = "pets_training.rs"]
mod pets_training;
#[path = "picking.rs"]
mod picking;
#[path = "picky_eater.rs"]
mod picky_eater;
#[path = "planning.rs"]
mod planning;
#[path = "practical_followups.rs"]
mod practical_followups;
#[path = "presentation.rs"]
mod presentation;
#[path = "reading_picks.rs"]
mod reading_picks;
#[path = "recipe_reading.rs"]
mod recipe_reading;
#[path = "recipes.rs"]
mod recipes;
#[path = "recycling.rs"]
mod recycling;
#[path = "rice.rs"]
mod rice;
#[path = "ripening.rs"]
mod ripening;
#[path = "roasting.rs"]
mod roasting;
#[path = "routines.rs"]
mod routines;
#[path = "sauces.rs"]
mod sauces;
#[path = "school_absences.rs"]
mod school_absences;
#[path = "school_breaks.rs"]
mod school_breaks;
#[path = "school_science.rs"]
mod school_science;
#[path = "science_counts.rs"]
mod science_counts;
#[path = "science_records.rs"]
mod science_records;
#[path = "seasoning.rs"]
mod seasoning;
#[path = "simple_pantry.rs"]
mod simple_pantry;
#[path = "sleep_aids.rs"]
mod sleep_aids;
#[path = "social_guards.rs"]
mod social_guards;
#[path = "sport_match.rs"]
mod sport_match;
#[path = "sport_practice.rs"]
mod sport_practice;
#[path = "sport_racing.rs"]
mod sport_racing;
#[path = "storage.rs"]
mod storage;
#[path = "tea_water.rs"]
mod tea_water;
#[path = "technology_desktop.rs"]
mod technology_desktop;
#[path = "technology_phone.rs"]
mod technology_phone;
#[path = "technology_setup.rs"]
mod technology_setup;
#[path = "templates.rs"]
mod templates;
#[path = "travel_airport.rs"]
mod travel_airport;
#[path = "travel_clock.rs"]
mod travel_clock;
#[path = "travel_destinations.rs"]
mod travel_destinations;
#[path = "travel_duration.rs"]
mod travel_duration;
#[path = "travel_logistics.rs"]
mod travel_logistics;
#[path = "travel_places.rs"]
mod travel_places;
#[path = "travel_plan.rs"]
mod travel_plan;
#[path = "travel_practical.rs"]
mod travel_practical;
#[path = "travel_stays.rs"]
mod travel_stays;
#[path = "vegetables.rs"]
mod vegetables;
#[path = "wake_at_night.rs"]
mod wake_at_night;
#[path = "wording_practical.rs"]
mod wording_practical;
#[path = "writing.rs"]
mod writing;

#[derive(Clone, Debug)]
pub struct KnowledgeMatch {
    pub id: &'static str,
    pub domain: &'static str,
    pub reply: Cow<'static, str>,
    pub outcomes: &'static [(&'static str, &'static str)],
    pub fault: bool,
    lead: leading::Lead,
    key: &'static [&'static [&'static str]],
}

impl KnowledgeMatch {
    pub(crate) fn keys(&self) -> &'static [&'static [&'static str]] {
        self.key
    }
}

struct RawRow {
    id: &'static str,
    domain: &'static str,
    reply: &'static str,
    key: &'static [&'static [&'static str]],
    grouped: bool,
    avoid: &'static [&'static str],
    ask: &'static [&'static [&'static str]],
    slips: &'static [&'static str],
    bias: i32,
    explain: bool,
    report: bool,
    fault: bool,
    outcomes: &'static [(&'static str, &'static str)],
    guard: guards::Guard,
    callback: Callback,
    shape: &'static [guards::Shape],
    lead: leading::Lead,
}

#[derive(Clone, Copy)]
enum Callback {
    Fixed,
    Text(fn(&Reading<'_>) -> Option<Cow<'static, str>>),
    Comfort,
    NightWaking,
    TechnologyVacuumFilter,
    BakeFault,
    BakeRecurring,
    BakeRecipe,
    Drink(drinks::Ask),
    Appointment,
    AppointmentPreparation,
    Pharmacy,
    HealthChecks,
    NewbornWeight,
    BodyTriage,
    FamilyFollowup,
    PracticalFollowup,
    CityGuide(CityGuideKind),
    ClothingCare(clothing_care::Reply),
    ClothingWear(clothing_wear::Reply),
    ClothingScratch,
    Difference,
    DishDefinition,
    Geography,
    Writing(writing::Form),
    SchoolBreak(school_breaks::Break),
    Music(music::Reply),
    Game(games::Reply),
    PetsCare(pets_care::Reply),
    PetSymptoms,
    PetFood,
    PetHealth,
    PetWellbeing,
    PetsTraining(pets_training::Reply),
    AskingOut,
    UpdatedGuidance,
    Freshness,
    ConsumerPension,
    TechDesktop(technology_desktop::Reply),
    Language,
    Study,
    StudyLearning,
    BakingGuide,
    CakePlanning,
    CookingFaults,
    CookingCrowds,
    CookingStorage,
    CookingMethods,
    DietNutrition,
    CookingExtras,
    RestMeat,
    PetAccidentCleaning,
    CookingFat,
    CookingPairings,
    DishPlan,
    WorkDialogue,
    MoneyPay,
    MoneySavings,
    SchoolScience(school_science::Rule, &'static [&'static str]),
    DishTexture(dish_texture::Rule, &'static [&'static str]),
    Gift(gift_cards::Reply),
    Planning,
    ShoppingComparisons,
    ShoppingReplies,
    CarPractical,
    MealShortlist,
    MealClock,
    MealCatalogue,
    MealScene,
    OccasionFood,
    DishChoices,
    VegetableIdeas(bool),
    BookRecommendation,
    OtherPick,
    Comics,
    Landscape(landscape_materials::Rule),
    TravelCapital,
    TravelDestination,
    TravelLogistics,
    TravelStays,
    TravelClock,
    TravelAirport,
    TravelPlan,
    TravelPractical,
    TravelDuration,
    TravelPlaces,
    HomeUpkeep,
    HistoryPast,
    NatureLeaves(&'static str),
    HomePractical(home_practical::Craft),
    HomeFilter(&'static [&'static str]),
    HomeTrade,
    HomeMould,
    HomeRepair(home_repair::Kind),
    Garden(garden_more::Rule, &'static [&'static str]),
    GardenSeason(garden_seasons::Rule),
    EverydayHealth,
    GardenPests(&'static [&'static str]),
    GardenCondition,
    SportRace(sport_racing::Reply),
    SportMatch(sport_match::Reply),
    HospitalVisit,
    KitchenMeasure(kitchen_measures::Ask),
    KitchenWithout,
    Home(&'static str),
    Prep(kitchen_prep::Reply),
    Recipe,
    SodaBreadRecipe,
    Ripen {
        trick_first: &'static str,
        keep: &'static str,
    },
    RipenFromChat,
    Opinion,
    OpinionThen(&'static Callback),
    Choice,
    ChoiceThen(&'static Callback),
    Sowing {
        first: u8,
        last: u8,
        late: &'static str,
        late_overrides_month: bool,
    },
    Vegetable(&'static [vegetables::Reply]),
    VegetableRoast,
    VegetableWatery,
    BirdLifespan,
    BeachKit,
    BodyWeeks,
    BodyPersistent,
    Pick(&'static [(&'static str, &'static str)]),
    Sauce {
        keeping: Option<&'static str>,
        yoghurt: Option<&'static str>,
    },
    Calendar,
    FilmList,
    FilmSuggestion,
    FilmShelf,
    FilmGenre,
    Mood(&'static [(&'static str, &'static str)]),
    Pool(&'static [&'static str]),
    AboutPerson {
        male: &'static [&'static str],
        female: &'static [&'static str],
    },
    NameFault {
        faults: &'static [(&'static str, &'static str)],
        default: &'static str,
    },
    ChilliSwap,
    RefusedSwap,
    ClothingTemperature,
    ClothingFit,
    ClothingChoice {
        past_fence: &'static [&'static str],
        choices: &'static [(&'static [&'static str], &'static str)],
    },
    JointTimeline,
    JointImplant,
    JointSoreness,
    JointSleeping,
    JointNoise,
    JointLock,
    Baby(babyfood::Reply),
    StoppedFixture,
    MaterialQuantity(material_quantities::Material),
    PickyEater,
    PresentationNerves,
    PlanDiet,
    PlanVeggie,
    MealDietAgain,
    Instrument(instruments::Ask),
    GadgetView,
    ChristmasTree,
    WhoWrote,
    BookLike,
    BookForAge,
    FamilyFilm(&'static [&'static str]),
    Pasta(pasta::Ask),
    Rice,
    PetCost,
    SmallPetCost,
    DogFeed,
    SeededPool(&'static [&'static str]),
    FunFact,
    FunToast,
    FunGame,
    FunName,
    FunHear,
    AnimalFact(animals::Fact),
    AnimalWord(animals::Word),
    AnimalDiet,
    AnimalRecord,
    FruitCooking,
    BasicCooking(cooking_basics::Rule, &'static [&'static str]),
    OatsStorage,
    SlimyMushroom,
    CrispStorage,
    YoghurtWhey,
    FlourLumps,
    CheeseMould,
    SauceStorage(&'static [&'static str]),
    CupboardStorage,
    FreezeSweet,
    SpoiledProduce,
    Compost,
    LifeAdmin(life_admin::Decision, &'static [&'static str]),
    AdminCancelOrder,
    AdminStolenBag,
    AdminStolenThing,
    AdminTenancyCopy,
    AdminPost,
    AdminLegalWord,
    AdminPassport(&'static [(&'static str, Option<&'static str>)]),
    AdminTipping(&'static [&'static str]),
}

#[derive(Clone, Copy)]
enum CityGuideKind {
    Guide,
    Next,
    Followup,
    CheapEats,
}

impl Callback {
    fn answers_statements(self) -> bool {
        matches!(
            self,
            Self::BakeFault
                | Self::BakeRecurring
                | Self::Recipe
                | Self::SodaBreadRecipe
                | Self::Difference
                | Self::DishDefinition
                | Self::Opinion
                | Self::OpinionThen(_)
                | Self::Choice
                | Self::ChoiceThen(_)
                | Self::Rice
        )
    }
}

type Form = u32;
type Match<'a> = (Entry<'a>, Cow<'static, str>);

struct StoredEntry {
    row: &'static RawRow,
    key: Span,
    avoid: Span,
    ask: Span,
    opening: StoredPhrase,
}

struct Entry<'a> {
    row: &'static RawRow,
    key: Key<'a>,
    avoid: Group<'a>,
    ask: Group<'a>,
    opening: Phrase<'a>,
}

struct Database {
    entries: Vec<StoredEntry>,
    arena: Arena,
    by_first_form: PostingIndex,
    forms: HashMap<String, Form>,
    word_use: HashMap<String, usize>,
    known_key_forms: HashSet<Form>,
    typos: HashMap<(u8, usize), Vec<TypoWord>>,
}

#[derive(Clone, Copy)]
struct Posting(u32);

impl Posting {
    fn new(index: usize, can_start_score: bool) -> Self {
        // The low bit keeps both lookup modes in a single postings list.
        let index = u32::try_from(index).expect("knowledge card index overflow");
        assert!(index <= u32::MAX >> 1, "knowledge posting overflow");
        Self((index << 1) | u32::from(can_start_score))
    }

    fn index(self) -> usize {
        (self.0 >> 1) as usize
    }

    fn can_start_score(self) -> bool {
        self.0 & 1 != 0
    }
}

#[derive(Clone, Copy)]
enum CandidateKeys {
    All,
    Scoring,
}

struct CandidateIds(Vec<u64>);

impl CandidateIds {
    const BITS_PER_WORD: usize = u64::BITS as usize;

    fn from_indices(entry_count: usize, indices: impl Iterator<Item = usize>) -> Self {
        let mut indices = indices.peekable();
        if indices.peek().is_none() {
            return Self(Vec::new());
        }
        let mut words = vec![0; entry_count.div_ceil(Self::BITS_PER_WORD)];
        for index in indices {
            words[index / Self::BITS_PER_WORD] |= 1 << (index % Self::BITS_PER_WORD);
        }
        Self(words)
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        // Read low bits first so equally ranked cards keep their authored order.
        self.0
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(word_index, mut bits)| {
                std::iter::from_fn(move || {
                    if bits == 0 {
                        return None;
                    }
                    let index = word_index * Self::BITS_PER_WORD + bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    Some(index)
                })
            })
    }
}

struct QuestionFocus {
    words: Vec<Vec<Form>>,
    names: HashSet<Form>,
}

impl QuestionFocus {
    fn longest_key_run(&self, entry: &Entry<'_>) -> usize {
        entry
            .key
            .iter()
            .flat_map(Group::iter)
            .filter(|phrase| {
                phrase
                    .words()
                    .flat_map(|forms| forms.iter())
                    .any(|form| self.names.contains(form))
            })
            .filter(|phrase| phrase.occurs(&self.words))
            .map(Phrase::len)
            .max()
            .unwrap_or_default()
    }
}

struct TypoWord {
    spelling: String,
    forms: Word,
}

#[derive(Default)]
struct CompileCache {
    words: HashMap<String, Word>,
    phrases: HashMap<&'static str, Span>,
}

include!("lookup_rules.rs");

static DATABASE: LazyLock<Database> = LazyLock::new(|| {
    assert_eq!(KNOWLEDGE_REVISION, crate::PYBOT_REVISION);
    Database::new(rows())
});

pub(crate) fn entry_count() -> usize {
    KNOWLEDGE_SEGMENTS.iter().map(|rows| rows.len()).sum()
}

pub(crate) fn single_ingredient_meal(reading: &Reading<'_>) -> Option<&'static str> {
    simple_pantry::dairy_free(reading)
}

pub(crate) fn pantry_combination(reading: &Reading<'_>, memory: &Memory) -> Option<&'static str> {
    simple_pantry::combination(reading, memory)
}

pub(crate) fn pantry_recipe_ingredients(reply: &str) -> Option<&'static [&'static str]> {
    PANTRY_DISHES
        .iter()
        .find(|dish| dish.reply == reply)
        .map(|dish| dish.uses)
}

pub(crate) fn domains() -> Vec<String> {
    rows()
        .map(|row| row.domain)
        .chain(KNOWLEDGE_WITHOUT_INDEXED_ROWS.iter().copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn rows() -> impl Clone + Iterator<Item = &'static RawRow> {
    KNOWLEDGE_SEGMENTS.iter().flat_map(|segment| segment.iter())
}

/// Registered domain and card identifiers in matcher order.
pub fn knowledge_ids() -> impl Iterator<Item = (&'static str, &'static str)> {
    rows().map(|row| (row.domain, row.id))
}

pub fn lookup(text: &str) -> Option<KnowledgeMatch> {
    lookup_reading(&Reading::new(text))
}

#[cfg(test)]
#[test]
fn caring_for_people_and_pets_goes_to_the_authored_readers() {
    let human = lookup("I am an unpaid carer and exhausted.").unwrap();
    assert_eq!((human.domain, human.id), ("caring", "carer-burnout"));
    assert!(human.reply.contains("ask for is a carer's assessment"));
    assert!(
        lookup("I care for my older dog and feel exhausted.")
            .is_none_or(|card| card.id != "carer-burnout")
    );
}

pub(crate) fn lookup_reading(reading: &Reading<'_>) -> Option<KnowledgeMatch> {
    lookup_from(reading, None)
}

pub(crate) fn lookup_in_context(reading: &Reading<'_>, memory: &Memory) -> Option<KnowledgeMatch> {
    lookup_from(reading, Some(memory))
}

pub(crate) fn child_cough_emergency(reading: &Reading<'_>, memory: &Memory) -> bool {
    lookup_in_context(reading, memory).is_some_and(|card| {
        (card.domain, card.id) == ("family", "child-cough")
            && card.reply == family_guards::CHILD_COUGH_EMERGENCY
    })
}

/// Select a card by its authored key and guard before invoking its reply callback.
pub(crate) fn matched_id(reading: &Reading<'_>) -> Option<&'static str> {
    matched_card(reading).map(|entry| entry.id)
}

/// Select a card for its domain and authored key before rendering its reply.
pub(crate) fn matched_card(reading: &Reading<'_>) -> Option<KnowledgeMatch> {
    if super::withheld::applies(reading.raw, None) {
        return None;
    }
    let (entry, reply) = DATABASE.find(reading, None, true)?;
    Some(entry.row.as_match(reply))
}

pub(crate) fn is_meal_shortlist(domain: &str, id: &str) -> bool {
    matches!(
        (domain, id),
        ("cooking", "meal-ideas") | ("meals", "meal-tonight" | "meal-ideas-anything")
    )
}

pub(crate) fn probes_meal_shortlist(reading: &Reading<'_>) -> bool {
    let lexical = Reading::new(reading.raw);
    if super::meal_shortlist::other_length_probe(&lexical) {
        return false;
    }
    matched_card(&lexical).is_none_or(|card| is_meal_shortlist(card.domain, card.id))
}

pub(crate) fn dog_feed_followup(reading: &Reading<'_>, memory: &Memory) -> Option<&'static str> {
    pet_feeding::followup(reading, &memory.turns)
}

pub(crate) fn cat_urinary_emergency(reading: &Reading<'_>) -> Option<&'static str> {
    pets_care::cat_urinary_emergency(reading)
}

pub(crate) fn sleep_aid(text: &str) -> Option<&'static str> {
    sleep_aids::answer(text)
}

pub(crate) fn trip_question(text: &str, memory: &Memory) -> Option<String> {
    city_guides::trip_question(text, &memory.turns)
}

pub(crate) fn trip_sight_question(text: &str, memory: &Memory) -> Option<String> {
    city_guides::trip_sight_question(text, &memory.turns)
}

pub(crate) fn move_answer_sentence(reading: &Reading<'_>, reply: &str) -> Option<String> {
    leading::move_answer_sentence(reading, reply)
}

pub(crate) fn apply_lead(
    reading: &Reading<'_>,
    card: &KnowledgeMatch,
    reply: &str,
) -> Option<String> {
    leading::apply(card.lead, reading, card.reply.as_ref(), reply)
}

pub(super) fn pet_card_covers(reading: &Reading<'_>, carried: &str) -> bool {
    let words = tokens(carried)
        .into_iter()
        .chain(reading.line.words.iter().cloned())
        .collect::<Vec<_>>();
    let forms = words
        .iter()
        .map(|word| DATABASE.indexed_forms(word))
        .collect::<Vec<_>>();
    let context = guards::Context::new(reading, &[]);
    DATABASE
        .candidates(&forms, CandidateKeys::All)
        .iter()
        .any(|index| {
            let entry = DATABASE.entry(index);
            entry.row.domain == "pets" && entry.score(&forms, &words, false, &context).is_some()
        })
}

fn lookup_from(reading: &Reading<'_>, memory: Option<&Memory>) -> Option<KnowledgeMatch> {
    if super::withheld::applies(reading.raw, memory) {
        return None;
    }
    if let Some(reply) = super::dishes::followup::named_salmon_doneness(reading)
        && let Some(row) = rows().find(|row| (row.domain, row.id) == ("dishes", "plan-done"))
    {
        return Some(row.as_match(Cow::Borrowed(reply)));
    }
    let selected = DATABASE.find(reading, memory, false);
    if selected
        .as_ref()
        .is_some_and(|(entry, _)| entry.row.id == "a-mood-anywhere")
        && let Some(memory) = memory
        && let Some(id) = entertainment::family_film_qualifier(reading, memory)
        && let Some(row) = rows().find(|row| (row.domain, row.id) == ("entertainment", id))
        && let Some(reply) = row.answer(&guards::Context::new(reading, &memory.turns))
    {
        return Some(row.as_match(reply));
    }
    if meal_scenes::vegan_carry(reading)
        && selected
            .as_ref()
            .is_none_or(|(_, reply)| meal_scenes::disagrees_with_vegan_carry(reply))
        && let Some(row) = rows().find(|row| (row.domain, row.id) == ("meals", "meal-carry"))
    {
        let earlier = memory.map_or(&[][..], |memory| memory.turns.as_slice());
        let mut context = guards::Context::new(reading, earlier);
        context.memory = memory;
        return row.answer(&context).map(|reply| row.as_match(reply));
    }
    let (entry, reply) = selected?;
    Some(entry.row.as_match(reply))
}

/// Retrieve a card's metadata and authored default reply by source identity.
pub fn by_id(domain: &str, id: &str) -> Option<KnowledgeMatch> {
    rows()
        .find(|row| row.domain == domain && row.id == id)
        .map(|row| row.as_match(Cow::Borrowed(row.reply)))
}

impl RawRow {
    fn as_match(&'static self, reply: Cow<'static, str>) -> KnowledgeMatch {
        let (reply, outcomes) = match (self.domain, self.id) {
            ("technology", "phone-water") => (
                Cow::Borrowed(super::phone_water::EXPOSURE),
                super::phone_water::OUTCOMES,
            ),
            ("lifeadmin", "register-death") => (
                Cow::Borrowed(super::updated_guidance::REGISTER_DEATH),
                self.outcomes,
            ),
            _ => (reply, self.outcomes),
        };
        KnowledgeMatch {
            id: self.id,
            domain: self.domain,
            reply,
            outcomes,
            fault: self.fault,
            lead: self.lead,
            key: self.key,
        }
    }

    fn answer(&self, context: &guards::Context<'_>) -> Option<Cow<'static, str>> {
        if context.select_only {
            // The planner can decline a key match after reading the actual ask.
            // A declined plan must not claim the dinner shortlist's length probe.
            if matches!(
                self.callback,
                Callback::DishPlan
                    | Callback::PlanVeggie
                    | Callback::MealDietAgain
                    | Callback::MealCatalogue
            ) {
                self.answer_with(self.callback, context)?;
            }
            return Some(Cow::Borrowed(self.reply));
        }
        self.answer_with(self.callback, context)
            .map(|reply| stock_swaps::fit(reply, context))
            .filter(|reply| dietary_menus::indexed_offer(self.domain, self.id, reply, context))
    }

    fn answer_with(
        &self,
        callback: Callback,
        context: &guards::Context<'_>,
    ) -> Option<Cow<'static, str>> {
        use super::bake_faults;
        if self.domain == "travel"
            && self.id == "airport-security"
            && super::airport_arrival::is_live_queue(&context.reading.line.clean)
        {
            return None;
        }
        let reply = match callback {
            Callback::Fixed if (self.domain, self.id) == ("health", "tonsillitis") => {
                Some(everyday_health::tonsillitis(context.reading))
            }
            Callback::Fixed => Some(self.reply),
            Callback::Text(render) => {
                return render(context.reading).filter(|reply| !reply.is_empty());
            }
            Callback::MealScene => return meal_scenes::registered(self.id, context),
            Callback::OccasionFood => return occasion_food::bring(context),
            Callback::DishChoices => return dish_choices::registered(self.id, context),
            Callback::MealCatalogue => {
                return super::meal_shortlist::catalogue::indexed(
                    self.id,
                    context.reading,
                    context.memory,
                )
                .map(Cow::Owned);
            }
            Callback::MealClock => {
                return super::meal_shortlist::clock::indexed_reply(
                    self.domain,
                    self.id,
                    context.reading,
                    context.memory,
                )
                .map(Cow::Owned);
            }
            Callback::MealShortlist => {
                if super::meal_shortlist::clock::indexed_request(
                    self.domain,
                    self.id,
                    context.reading,
                    context.memory,
                ) {
                    return super::meal_shortlist::clock::indexed_reply(
                        self.domain,
                        self.id,
                        context.reading,
                        context.memory,
                    )
                    .map(Cow::Owned);
                }
                if self.domain == "cooking"
                    && super::meal_shortlist::plant_ask(context.reading).is_some()
                {
                    return super::meal_shortlist::plant_card(context.reading).map(Cow::Owned);
                }
                return super::meal_shortlist::open_dinner(context.reading, context.earlier.len())
                    .map(|draft| Cow::Owned(draft.reply));
            }
            Callback::Comfort => wording_practical::comfort(context.reading, context.earlier),
            Callback::NightWaking => wake_at_night::allows(context).then_some(self.reply),
            Callback::TechnologyVacuumFilter => return technology_setup::vacuum_filter(context),
            Callback::BakeFault => bake_faults::fault(&context.reading.line.words),
            Callback::BakeRecipe => recipes::bake(context.reading),
            Callback::Drink(ask) => return drinks::answer(ask, context.reading),
            Callback::Appointment => return appointment::answer(context.reading, context.earlier),
            Callback::AppointmentPreparation => {
                return appointment::preparation(context.reading, context.earlier);
            }
            Callback::Pharmacy => {
                return medicine_cards::pharmacy(self.id, context.reading, context.earlier);
            }
            Callback::HealthChecks => {
                return health_checks::reply(self.id, context.reading, context.earlier);
            }
            Callback::NewbornWeight => {
                return newborn_cards::birth_weight(context.reading, context.earlier);
            }
            Callback::BodyTriage => {
                return body_triage::reply(self.id, context.reading, context.earlier);
            }
            Callback::FamilyFollowup => {
                return family_followups::reply(self.id, context.reading, context.earlier);
            }
            Callback::PracticalFollowup => {
                return practical_followups::reply(self.domain, context.reading, context.earlier);
            }
            Callback::ClothingCare(kind) => {
                return clothing_care::reply(kind, context.reading, context.earlier);
            }
            Callback::ClothingWear(kind) => {
                return clothing_wear::reply(kind, context.reading, context.earlier);
            }
            Callback::ClothingScratch => {
                return clothing_lenses::scratch(context.reading, context.earlier);
            }
            Callback::CityGuide(kind) => {
                return match kind {
                    CityGuideKind::Guide => city_guides::guide(context.reading, context.earlier),
                    CityGuideKind::Next => city_guides::next(context.reading, context.earlier),
                    CityGuideKind::CheapEats => {
                        city_guides::cheap_eats(context.reading, context.earlier)
                    }
                    CityGuideKind::Followup => {
                        city_guides::followup(context.reading, context.earlier)
                    }
                };
            }
            Callback::Difference => differences::answer(context.reading),
            Callback::DishDefinition => {
                return super::dishes::definition(&context.reading.line.clean).map(Cow::Owned);
            }
            Callback::Geography => return super::geography::registered(self.id, context.reading),
            Callback::Writing(form) => {
                return writing::reply(form, context.reading, context.earlier);
            }
            Callback::SchoolBreak(which) => {
                return Some(school_breaks::answer(
                    which,
                    context.reading,
                    context.earlier,
                ));
            }
            Callback::Music(which) => return music::reply(which, context.reading),
            Callback::Game(which) => return games::reply(which, context.reading, context.earlier),
            Callback::PetWellbeing => {
                return pet_wellbeing::registered(self.id, context.reading, context.earlier);
            }
            Callback::PetHealth => {
                return pet_health::registered(self.id, context.reading, context.earlier);
            }
            Callback::PetFood => return pet_food::registered(self.id, context.reading),
            Callback::PetSymptoms => {
                return pet_symptoms::registered(self.id, context.reading, context.earlier)
                    .map(Cow::Borrowed);
            }
            Callback::PetsCare(form) => {
                return pets_care::reply(form, context.reading, context.earlier);
            }
            Callback::PetsTraining(form) => {
                return pets_training::reply(form, context.reading, context.earlier);
            }
            Callback::AskingOut => {
                return family_cards::asking_out(context.reading, context.earlier);
            }
            Callback::Freshness => return freshness_cards::registered(self.id, context),
            Callback::UpdatedGuidance => {
                return super::updated_guidance::registered(self.domain, self.id, context.reading);
            }
            Callback::ConsumerPension => {
                return Some(super::consumer_rights::pension(
                    context.reading,
                    context.earlier,
                ));
            }
            Callback::TechDesktop(form) => return technology_desktop::reply(form, context.reading),
            Callback::Language => return language_terms::registered(self.id, context.reading),
            Callback::Study => return super::study_replies::registered(self.id, context.reading),
            Callback::StudyLearning => {
                return super::study_learning::registered(
                    self.id,
                    context.reading,
                    context.earlier,
                );
            }
            Callback::BakingGuide => super::baking_guides::registered(self.id, context.reading),
            Callback::CakePlanning => {
                return super::cake_planning::registered(self.id, context.reading);
            }
            Callback::CookingCrowds => {
                return cooking_crowds::registered(self.id, context.reading, context.earlier);
            }
            Callback::RestMeat => return roasting::rest_the_meat(context),
            Callback::PetAccidentCleaning => {
                return cleaning_stains::the_place_a_pet_used(context.reading, context.earlier);
            }
            Callback::CookingExtras => {
                return if self.id == "veg-gone-soft" {
                    cooking_extras::soft_veg(context)
                } else {
                    cooking_extras::chilli_keep(self.id, context)
                };
            }
            Callback::DietNutrition => return diet_nutrition::eat_veg(context),
            Callback::CookingMethods => {
                return cooking_methods::registered(self.id, context.reading, context.earlier)
                    .or_else(|| meal_windows::registered(self.id, context));
            }
            Callback::CookingStorage => {
                return cooking_storage::registered(self.id, context.reading, context.earlier);
            }
            Callback::CookingFaults => super::cooking_faults::registered(self.id, context.reading),
            Callback::CookingPairings => {
                cooking_pairings::registered(self.id, context.reading, context.earlier).filter(
                    |reply| {
                        self.id != "meal-sides"
                            || context.memory.is_none()
                            || cooking_pairings::fits_plant_dish(context.reading, reply)
                    },
                )
            }
            Callback::DishPlan => {
                return dish_plan::registered(
                    self.id,
                    context.reading,
                    context.earlier,
                    context.memory,
                );
            }
            Callback::CookingFat => cooking_fats::reply(context.memory),
            Callback::MoneySavings => {
                return super::money_savings::reply(self.id, context.reading, context.earlier);
            }
            Callback::MoneyPay => {
                return super::money_tax::take_home(context.reading, context.earlier);
            }
            Callback::WorkDialogue => {
                return super::work_dialogue::answer(self.id, context.reading, context.earlier);
            }
            Callback::SchoolScience(rule, replies) => {
                school_science::reply(rule, context.reading, replies)
            }
            Callback::DishTexture(rule, replies) => {
                dish_texture::reply(rule, context.reading, replies)
            }
            Callback::Planning => return planning::registered(self.id, context),
            Callback::ShoppingComparisons => {
                return shopping_comparisons::registered(self.id, context.reading, context.earlier);
            }
            Callback::ShoppingReplies => {
                return super::shopping_replies::registered(
                    self.id,
                    context.reading,
                    context.earlier,
                );
            }
            Callback::Gift(kind) => return gift_cards::reply(kind, self.reply, context),
            Callback::CarPractical => {
                return super::car_practical::registered(self.id, context.reading, context.earlier);
            }
            Callback::TravelLogistics => {
                travel_logistics::registered(self.id, context.reading, context.earlier)
            }
            Callback::TravelAirport => {
                return travel_airport::registered(self.id, context.reading, context.earlier);
            }
            Callback::TravelPractical => {
                return travel_practical::registered(self.id, context.reading, context.earlier);
            }
            Callback::TravelPlan => {
                return travel_plan::registered(
                    self.id,
                    context.reading,
                    context.earlier,
                    context.memory.map(|state| &state.stated_plans),
                );
            }
            Callback::TravelClock => {
                return travel_clock::registered(
                    self.id,
                    context.reading,
                    context.earlier,
                    context.memory.map(|state| &state.stated_plans),
                );
            }
            Callback::TravelStays => travel_stays::registered(
                self.id,
                context.reading,
                context.earlier,
                context.memory.map(|state| &state.stated_plans),
            ),
            Callback::TravelDuration => {
                travel_duration::registered(self.id, context.reading, context.earlier)
            }
            Callback::TravelPlaces => {
                return travel_places::registered(
                    self.id,
                    context.reading,
                    context.earlier,
                    context.memory.map(|state| &state.stated_plans),
                );
            }
            Callback::HomeUpkeep => {
                return home_upkeep::registered(
                    self.id,
                    context.reading,
                    context.earlier,
                    context.memory,
                );
            }
            Callback::HistoryPast => history_cards::past(context.reading, context.earlier),
            Callback::NatureLeaves(reply) => nature_cards::leaves(context.earlier, reply),
            Callback::VegetableIdeas(child_required) => {
                cooking_cards::vegetable_ideas(context.reading, context.earlier, child_required)
            }
            Callback::BookRecommendation => {
                return reading_picks::recommend(context.reading, context.earlier);
            }
            Callback::OtherPick => return entertainment::other_pick(context).map(Cow::Owned),
            Callback::Comics => return reading_picks::comics(context.reading, context.earlier),
            Callback::Landscape(rule) => {
                return Some(landscape_materials::answer(
                    rule,
                    context.reading,
                    context.earlier,
                ));
            }
            Callback::TravelCapital => super::travel::capital_reply(&context.reading.line.clean),
            Callback::TravelDestination => {
                travel_destinations::registered(self.id, context.reading, context.earlier)
            }
            Callback::HomePractical(kind) => return kind.answer(context),
            Callback::HomeFilter(names) => home_maintenance::filter(names, self.reply, context),
            Callback::HomeTrade => home_fittings::trade_to_call(context),
            Callback::HomeMould => home_household::mould(context),
            Callback::HomeRepair(kind) => return kind.reply(context),
            Callback::Garden(rule, texts) => garden_more::reply(rule, context.reading, texts),
            Callback::GardenSeason(rule) => return garden_seasons::answer(rule, context.reading),
            Callback::EverydayHealth => return everyday_health::registered(self.id, context),
            Callback::GardenPests(replies) => {
                garden_faults::pests(context.reading, context.earlier, replies)
            }
            Callback::GardenCondition => {
                garden_faults::condition(self.id, context.reading, context.earlier)
            }
            Callback::SportRace(kind) => {
                return sport_racing::reply(kind, context.reading, context.earlier);
            }
            Callback::SportMatch(kind) => return sport_match::reply(kind, context.reading),
            Callback::HospitalVisit => {
                return family_cards::hospital_visits(context.reading, context.earlier);
            }
            Callback::KitchenMeasure(which) => {
                return kitchen_measures::answer(which, context.reading);
            }
            Callback::KitchenWithout => return cooking_basics::kitchen_without(context),
            Callback::Home(id) => {
                return super::home_guards::registered(id, context.reading, context.earlier);
            }
            Callback::Prep(prep) => prep.answer(context),
            Callback::BakeRecurring => {
                bake_faults::recurring(context.reading.raw, &context.reading.line.words)
            }
            Callback::Recipe | Callback::SodaBreadRecipe => recipes::answer(
                context,
                self.key[0],
                self.reply,
                matches!(callback, Callback::SodaBreadRecipe),
            ),
            Callback::Ripen { trick_first, keep } => {
                Some(ripening::answer(context, self.reply, trick_first, keep))
            }
            Callback::RipenFromChat => Some(ripening::from_chat(context)),
            Callback::Opinion => opinions::asked(context.reading).then_some(self.reply),
            Callback::OpinionThen(next) => {
                return if opinions::asked(context.reading) {
                    self.answer_with(*next, context)
                } else {
                    None
                };
            }
            Callback::Choice => opinions::either(context.reading, self.key).then_some(self.reply),
            Callback::ChoiceThen(next) => {
                return if opinions::either(context.reading, self.key) {
                    self.answer_with(*next, context)
                } else {
                    None
                };
            }
            Callback::Sowing {
                first,
                last,
                late,
                late_overrides_month,
            } => garden::sowing(
                context.reading,
                first..=last,
                self.reply,
                late,
                late_overrides_month,
            ),
            Callback::Vegetable(aliases) => vegetables::answer(context.reading.raw, aliases),
            Callback::VegetableRoast => {
                return vegetables::roast(context.reading.raw).map(Cow::Owned);
            }
            Callback::VegetableWatery => vegetables::watery(context.reading),
            Callback::BirdLifespan => {
                guards::names_bird_life(context.reading).then_some(self.reply)
            }
            Callback::BeachKit => (!guards::past_trip(context)).then_some(self.reply),
            Callback::BodyWeeks => body::weeks(context),
            Callback::BodyPersistent => Some(body::persistent(context.reading)),
            Callback::Pick(choices) => Some(picking::answer(context.reading, choices, self.reply)),
            Callback::Sauce { keeping, yoghurt } => {
                sauces::answer(context.reading, self.reply, keeping, yoghurt)
            }
            Callback::Calendar => {
                calendar::asks_when(context.reading, self.key[0]).then_some(self.reply)
            }
            Callback::FilmList => {
                return Some(entertainment::film_list(context.reading, self.reply));
            }
            Callback::FilmSuggestion => return entertainment::suggestion(self.id, context),
            Callback::FilmShelf => return film_shelves::registered(self.id, context),
            Callback::FilmGenre => return film_genres::registered(self.id, context),
            Callback::Mood(picks) => return entertainment::pick_mood(context.reading, picks),
            Callback::Pool(replies) => Some(match context.memory {
                Some(memory) => compose::pick(replies, memory, ""),
                None => compose::pick(replies, &Memory::default(), ""),
            }),
            Callback::AboutPerson { male, female } => {
                return Some(Cow::Owned(templates::about_person(
                    &context.reading.line.words,
                    self.reply,
                    male,
                    female,
                )));
            }
            Callback::NameFault { faults, default } => {
                return Some(templates::name_fault(
                    context.reading.folded(),
                    self.reply,
                    faults,
                    default,
                ));
            }
            Callback::ChilliSwap => Some(guards::chilli_swap(&context.reading.line.clean)),
            Callback::RefusedSwap => guards::refused_swap(context.reading),
            Callback::ClothingTemperature => clothing::dress_for_temperature(context.reading),
            Callback::ClothingFit => Some(clothing::choose_garment(self.reply, context.earlier)),
            Callback::ClothingChoice {
                past_fence,
                choices,
            } => clothing::choose_branch(
                context.reading,
                context.earlier,
                past_fence,
                choices,
                self.reply,
            ),
            Callback::JointTimeline => Some(joints::joint_reply(
                context.reading,
                JOINT_TIMELINE_REPLY,
                JOINT_SHOULDER_TIMELINE_REPLY,
            )),
            Callback::JointImplant => Some(joints::joint_reply(
                context.reading,
                JOINT_IMPLANT_REPLY,
                JOINT_SHOULDER_IMPLANT_REPLY,
            )),
            Callback::JointSoreness => Some(joints::joint_reply(
                context.reading,
                JOINT_SORE_REPLY,
                JOINT_SHOULDER_SORE_REPLY,
            )),
            Callback::JointSleeping => Some(joints::joint_reply(
                context.reading,
                JOINT_SLEEPING_REPLY,
                JOINT_SHOULDER_SLEEPING_REPLY,
            )),
            Callback::JointNoise => return Some(Cow::Owned(joints::noisy(context.reading))),
            Callback::JointLock => return Some(joints::locked(context.reading)),
            Callback::Baby(reply) => return reply.answer(context, self.reply, self.key[0]),
            Callback::StoppedFixture => context.stopped_fixture(),
            Callback::MaterialQuantity(material) => {
                return Some(material_quantities::answer(material, context));
            }
            Callback::PickyEater => picky_eater::answer(context.reading, self.reply),
            Callback::PresentationNerves => {
                return Some(presentation::reply(context.reading, self.reply));
            }
            Callback::PlanDiet => diet_plan::swaps(context),
            Callback::PlanVeggie => dietary_menus::plan(context),
            Callback::MealDietAgain => {
                return super::meal_shortlist::plant_proposal(context.reading, context.memory?)
                    .map(|proposal| Cow::Owned(proposal.reply));
            }
            Callback::Instrument(ask) => return instruments::answer(ask, context),
            Callback::GadgetView => Some(opinions::gadget_view(context.reading)),
            Callback::ChristmasTree => {
                opinions::about_a_tree(context.reading).then_some(self.reply)
            }
            Callback::WhoWrote => return books::who_wrote(context.reading).map(Cow::Owned),
            Callback::BookLike => return books::similar(context.reading),
            Callback::BookForAge => return Some(books::for_age(context.reading)),
            Callback::FamilyFilm(replies) => {
                return entertainment::family_film(context.reading, replies);
            }
            Callback::Pasta(ask) => return Some(pasta::answer(ask, context.reading)),
            Callback::Rice => return Some(rice::reply(context)),
            Callback::PetCost => Some(pet_costs::dog_or_cat(context.reading)),
            Callback::SmallPetCost => pet_costs::small_pet(context.reading),
            Callback::SeededPool(pool) => Some(fun::pick(context, pool, context.reading.raw)),
            Callback::FunFact => fun::fact(context),
            Callback::FunToast => Some(fun::toast(context)),
            Callback::FunGame => Some(fun::group_game(context)),
            Callback::FunName => return Some(fun::name_ideas(context)),
            Callback::FunHear => fun::something_to_hear(context),
            Callback::AnimalFact(fact) => fact.answer(context),
            Callback::AnimalWord(word) => return word.answer(context),
            Callback::AnimalDiet => animals::eats(context),
            Callback::AnimalRecord => animals::record(context),
            Callback::FruitCooking => cooking_basics::fruit_cooking(&context.reading.line.clean),
            Callback::OatsStorage => Some(storage::oats(context.reading)),
            Callback::SlimyMushroom => return Some(storage::mushroom(context.reading)),
            Callback::CrispStorage => return Some(storage::crunch(context.reading)),
            Callback::YoghurtWhey => return Some(storage::yoghurt(context.reading)),
            Callback::FlourLumps => Some(storage::flour(context.reading)),
            Callback::CheeseMould => {
                return Some(storage::cheese_mould(context.reading, self.reply));
            }
            Callback::SauceStorage(replies) => storage::sauce(context.reading, replies),
            Callback::CupboardStorage => Some(storage::cupboard(context.reading, self.reply)),
            Callback::FreezeSweet => storage::freeze_sweet(context.reading),
            Callback::SpoiledProduce => {
                return Some(storage::spoiled_produce(context.reading, self.reply));
            }
            Callback::Compost => return Some(garden::compost(context.reading)),
            Callback::LifeAdmin(decision, replies) => decision.answer(context.reading, replies),
            Callback::AdminCancelOrder => return Some(life_admin::cancel_order(context.reading)),
            Callback::AdminStolenBag => Some(life_admin::stolen_bag(context.reading)),
            Callback::AdminStolenThing => Some(life_admin::stolen_thing(context.reading)),
            Callback::AdminPost => Some(life_admin::missing_post(context.reading)),
            Callback::AdminLegalWord => life_admin::legal_word(context.reading),
            Callback::AdminPassport(sections) => {
                Some(life_admin::passport(context.reading, sections, self.reply))
            }
            Callback::AdminTipping(replies) => {
                return life_admin::tipping(context.reading, replies);
            }
            Callback::AdminTenancyCopy => {
                Some(life_admin::tenancy_copy(context.reading, self.reply))
            }
            Callback::BasicCooking(rule, texts) => {
                cooking_basics::reply(context.reading, rule, texts)
            }
            Callback::DogFeed => Some(pet_feeding::dog_feed(
                context.reading.folded(),
                context.earlier,
            )),
        };
        reply.map(Cow::Borrowed)
    }
}

impl Database {
    fn new(rows: impl Clone + Iterator<Item = &'static RawRow>) -> Self {
        let mut database = Self {
            entries: Vec::with_capacity(rows.clone().count()),
            arena: Arena::default(),
            by_first_form: PostingIndex::default(),
            forms: HashMap::new(),
            word_use: HashMap::new(),
            known_key_forms: HashSet::new(),
            typos: HashMap::new(),
        };
        let mut cache = CompileCache::default();
        let mut postings = Vec::new();
        let mut domain_words = HashMap::<&str, HashMap<String, usize>>::new();
        for row in rows.clone() {
            let words: HashSet<_> = row
                .key
                .iter()
                .flat_map(|group| group.iter())
                .flat_map(|phrase| phrase.split_whitespace())
                .map(str::to_lowercase)
                .collect();
            let counts = domain_words.entry(row.domain).or_default();
            for word in words {
                *database.word_use.entry(word.clone()).or_default() += 1;
                *counts.entry(word).or_default() += 1;
            }
        }
        // The unregistered texture card still affects rarity in PyBot's vegetable domain.
        if !rows
            .clone()
            .any(|row| row.domain == "vegetables" && row.id == "veg-gone-soft")
        {
            let counts = domain_words.entry("vegetables").or_default();
            for &word in VEG_GONE_SOFT_KEY_WORDS {
                *counts.entry(word.to_owned()).or_default() += 1;
            }
        }
        let mut key_groups = HashMap::new();
        let mut avoid_groups = HashMap::new();
        let mut ask_groups = HashMap::new();
        let mut first_forms = HashMap::new();
        for row in rows.clone() {
            let counts = &domain_words[row.domain];
            let group_start = database.arena.groups.len();
            for &group in row.key {
                // The same group can have different rarity in another domain.
                let group_span = *key_groups.entry((row.domain, group)).or_insert_with(|| {
                    database.compile_group(group.iter().copied(), Some(counts), &mut cache)
                });
                database.arena.groups.push(group_span);
            }
            let key = Span::added(&database.arena.groups, group_start);
            let avoid = *avoid_groups.entry(row.avoid).or_insert_with(|| {
                database.compile_group(row.avoid.iter().copied(), None, &mut cache)
            });
            let ask = *ask_groups.entry(row.ask).or_insert_with(|| {
                let words = row.ask.iter().flat_map(|group| group.iter().copied());
                database.compile_group(words, None, &mut cache)
            });
            let opening = database.compile(
                row.reply.split(['.', '!', '?']).next().unwrap_or(""),
                &mut cache,
            );
            let entry_index = database.entries.len();

            // Grouped scoring requires a hit in the first group. The spelling
            // comparison can score the original typed forms on a repaired pass.
            let needs_first_group = row.grouped && row.id != "spelling-pick-one";
            for (index, group) in database.arena.key(key).iter().enumerate() {
                let can_start_score = !needs_first_group || index == 0;
                for alternative in group.iter() {
                    if let Some(first) = alternative.word(0) {
                        for &form in first {
                            first_forms.entry(form).or_insert(can_start_score);
                        }
                    }
                }
            }
            for (form, can_start_score) in first_forms.drain() {
                postings.push((form, Posting::new(entry_index, can_start_score)));
            }
            database.entries.push(StoredEntry {
                row,
                key,
                avoid,
                ask,
                opening,
            });
        }
        drop((domain_words, key_groups, avoid_groups, ask_groups));
        drop(cache.phrases);
        database.by_first_form = PostingIndex::pack(postings, database.forms.len());
        database.arena.shrink_to_fit();
        database.index_typos(rows, &cache.words);
        database
    }

    fn compile_group(
        &mut self,
        alternatives: impl Iterator<Item = &'static str>,
        counts: Option<&HashMap<String, usize>>,
        cache: &mut CompileCache,
    ) -> Span {
        let start = self.arena.phrases.len();
        for text in alternatives {
            let mut phrase = self.compile(text, cache);
            if let Some(counts) = counts {
                let unique = text
                    .split_whitespace()
                    .filter(|word| counts.get(&word.to_lowercase()) == Some(&1))
                    .count();
                phrase.unique_in_domain =
                    u32::try_from(unique).expect("knowledge phrase rarity overflow");
            }
            self.arena.phrases.push(phrase);
        }
        Span::added(&self.arena.phrases, start)
    }

    fn compile(&mut self, phrase: &'static str, cache: &mut CompileCache) -> StoredPhrase {
        let words = *cache.phrases.entry(phrase).or_insert_with(|| {
            let start = self.arena.phrase_words.len();
            for text in tokens(phrase) {
                let word = *cache.words.entry(text).or_insert_with_key(|text| {
                    let start = self.arena.forms.len();
                    for form in forms(text) {
                        let next = Form::try_from(self.forms.len())
                            .expect("knowledge form index overflow");
                        self.arena
                            .forms
                            .push(*self.forms.entry(form).or_insert(next));
                    }
                    let word = Word::try_from(self.arena.words.len())
                        .expect("knowledge word index overflow");
                    self.arena.words.push(Span::added(&self.arena.forms, start));
                    word
                });
                self.arena.phrase_words.push(word);
            }
            Span::added(&self.arena.phrase_words, start)
        });
        StoredPhrase {
            words,
            unique_in_domain: 0,
        }
    }

    fn entry(&self, index: usize) -> Entry<'_> {
        let entry = &self.entries[index];
        Entry {
            row: entry.row,
            key: self.arena.key(entry.key),
            ask: self.arena.group(entry.ask),
            avoid: self.arena.group(entry.avoid),
            opening: self.arena.phrase(entry.opening),
        }
    }

    fn index_typos(
        &mut self,
        rows: impl Clone + Iterator<Item = &'static RawRow>,
        cache: &HashMap<String, Word>,
    ) {
        let marked: HashSet<_> = rows
            .clone()
            .flat_map(|row| row.slips.iter())
            .flat_map(|slip| tokens(slip))
            .collect();
        let vocabulary: HashSet<_> = rows
            .flat_map(|row| row.key.iter().flat_map(|group| group.iter()))
            .flat_map(|phrase| tokens(phrase))
            .filter(|word| !marked.contains(word))
            .collect();
        for word in vocabulary {
            let Some(&word_id) = cache.get(&word) else {
                continue;
            };
            self.known_key_forms
                .extend(self.arena.word(word_id).iter().copied());
            if let Some(bucket) = typo_bucket(&word) {
                self.typos.entry(bucket).or_default().push(TypoWord {
                    spelling: word,
                    forms: word_id,
                });
            }
        }
    }

    fn typo_forms(&self, word: &str, read_forms: &[Form]) -> Option<Vec<Form>> {
        let (first, len) = typo_bucket(word)?;
        if len < 4
            || read_forms
                .iter()
                .any(|form| self.known_key_forms.contains(form))
            || engine_reads(word)
        {
            return None;
        }
        let mut family = None;
        let mut forms = Vec::new();
        for candidate in (len - 1..=len + 1)
            .filter_map(|candidate_len| self.typos.get(&(first, candidate_len)))
            .flatten()
            .filter(|candidate| one_edit(word, &candidate.spelling))
        {
            let this = stem(&candidate.spelling);
            if family.is_some_and(|previous| previous != this) {
                return None;
            }
            family = Some(this);
            forms.extend_from_slice(self.arena.word(candidate.forms));
        }
        family?;
        forms.sort_unstable();
        forms.dedup();
        Some(forms)
    }

    fn find(
        &self,
        reading: &Reading<'_>,
        memory: Option<&Memory>,
        select_only: bool,
    ) -> Option<Match<'_>> {
        self.find_excluding(reading, memory, select_only, None)
    }

    fn find_excluding(
        &self,
        reading: &Reading<'_>,
        memory: Option<&Memory>,
        select_only: bool,
        excluded_id: Option<&'static str>,
    ) -> Option<Match<'_>> {
        let observed = reading.lexical_words();
        let words =
            breeds::expand(observed.map_or_else(|| tokens(reading.raw), |words| words.to_vec()));
        // Exact stateful/rhetorical decisions require the wider reading system.
        if words.is_empty() || words.len() > 256 {
            return None;
        }
        let earlier = memory
            .map(|memory| memory.turns.as_slice())
            .unwrap_or_default();
        let mut context = guards::Context::new(reading, earlier);
        context.memory = memory;
        context.select_only = select_only;
        context.excluded_id = excluded_id;
        if let Some(answer) = self.find_words(&words, &context) {
            return Some(answer);
        }
        if observed.is_some() {
            return None;
        }
        // The understanding lexer has alternate wordings for spelling and chat
        // contractions. Let these reach a card only when the typed line yielded none.
        let repaired = breeds::expand(tokens(&reading.line.plain));
        if repaired != words
            && let Some(answer) = self.find_words(&repaired, &context)
        {
            return Some(answer);
        }
        let normalized = breeds::expand(reading.line.norm.clone());
        if normalized != words && normalized != repaired {
            return self.find_words(&normalized, &context);
        }
        None
    }

    fn find_words(&self, words: &[String], context: &guards::Context<'_>) -> Option<Match<'_>> {
        let first = words.first()?;
        if words.len() > 256 {
            return None;
        }
        // The pinned named-condition reader accepts "symptoms of [condition]"
        // as the ask frame around the condition's authored name.
        let definition = everyday_health::tonsillitis_symptom_subject(words)
            .map(|subject| (subject, subject + 1, false))
            .or_else(|| definition_subject(words));
        let statement =
            definition.is_none() && !context.reading.raw.contains('?') && !question_opener(first);
        let mut word_forms: Vec<Vec<Form>> =
            words.iter().map(|word| self.indexed_forms(word)).collect();
        if statement {
            return self.best_scored(
                &self.candidates(&word_forms, CandidateKeys::Scoring),
                &word_forms,
                words,
                true,
                context,
                |_| true,
            );
        }
        if let Some(answer) = self.find_with_forms(words, &word_forms, definition, context) {
            return Some(answer);
        }
        // Try one unambiguous correction only after a literal reading found no card.
        // The vocabulary contains only the translated subset, so limit the rescue to
        // one altered word rather than guessing at a substantially different line.
        let mut repairs =
            words
                .iter()
                .zip(&word_forms)
                .enumerate()
                .filter_map(|(index, (word, known))| {
                    self.typo_forms(word, known).map(|forms| (index, forms))
                });
        let (index, forms) = repairs.next()?;
        if repairs.next().is_some() {
            return None;
        }
        word_forms[index].extend(forms);
        self.find_with_forms(words, &word_forms, definition, context)
    }

    fn candidates(&self, word_forms: &[Vec<Form>], keys: CandidateKeys) -> CandidateIds {
        let indices = word_forms
            .iter()
            .flatten()
            .flat_map(|&form| self.by_first_form.get(form))
            .copied()
            .filter(|posting| matches!(keys, CandidateKeys::All) || posting.can_start_score())
            .map(Posting::index);
        CandidateIds::from_indices(self.entries.len(), indices)
    }

    fn question_focus(&self, reading: &Reading<'_>) -> Option<QuestionFocus> {
        let focus = &reading.line.focus;
        let (last, earlier) = reading.line.clauses.split_last()?;
        if reading.features.asks.is_empty()
            || focus.is_empty()
            || last.is_empty()
            || !earlier.iter().any(|clause| clause.len() >= 4)
        {
            return None;
        }
        let sets = |word: &str| {
            let mut values = forms(word);
            values.sort_unstable();
            values
        };
        if focus != last {
            let last_forms: HashSet<_> = last.iter().map(|word| sets(word)).collect();
            let earlier_forms: HashSet<_> =
                earlier.iter().flatten().map(|word| sets(word)).collect();
            if focus
                .iter()
                .map(|word| sets(word))
                .any(|forms| !last_forms.contains(&forms) && earlier_forms.contains(&forms))
            {
                return None;
            }
        }
        let names_below = |count| {
            focus
                .iter()
                .filter(|word| {
                    word.chars().count() >= 3
                        && !LOOKUP_NOT_A_SUBJECT.contains(&word.as_str())
                        && !LOOKUP_ASK_FRAME_WORDS.contains(&word.as_str())
                        && self
                            .word_use
                            .get(word.as_str())
                            .copied()
                            .unwrap_or_default()
                            <= count
                })
                .flat_map(|word| self.indexed_forms(word))
                .collect::<HashSet<_>>()
        };
        let mut names = names_below(40);
        if names.is_empty() && !reading.features.asks.contains(&"yesno") {
            names = names_below(400);
        }
        (!names.is_empty()).then(|| QuestionFocus {
            words: focus.iter().map(|word| self.indexed_forms(word)).collect(),
            names,
        })
    }

    fn indexed_forms(&self, word: &str) -> Vec<Form> {
        forms(word)
            .iter()
            .filter_map(|form| self.forms.get(form).copied())
            .collect()
    }

    fn find_with_forms(
        &self,
        words: &[String],
        word_forms: &[Vec<Form>],
        definition: Option<(usize, usize, bool)>,
        context: &guards::Context<'_>,
    ) -> Option<Match<'_>> {
        if let Some((start, end, tail)) = material_subject(words) {
            let wanted = &word_forms[start..end];
            let cue = &word_forms[end..tail];
            let candidates = self.candidates(wanted, CandidateKeys::All);
            let names_subject = |entry: &Entry<'_>| entry.names_subject(wanted);
            // Prefer the full authored cue. "Made from" is an origin cue
            // for electricity, while "made" alone also keys its inventor.
            if let Some(answer) = self
                .best_scored(&candidates, word_forms, words, false, context, |entry| {
                    names_subject(entry)
                        && entry
                            .key
                            .iter()
                            .skip(1)
                            .flat_map(Group::iter)
                            .any(|key| key.matches(cue))
                })
                .or_else(|| {
                    self.best_scored(&candidates, word_forms, words, false, context, |entry| {
                        names_subject(entry)
                            && (entry.row.id == "body-what-tonsils"
                                || entry.row.key.iter().skip(1).any(|group| {
                                    group.contains(&"made") || group.contains(&"composition")
                                }))
                    })
                })
            {
                return Some(answer);
            }
        }
        if let Some((start, end, purpose)) = definition {
            let wanted = &word_forms[start..end];
            let mut best = None;
            let mut best_rank = None;
            let candidates = self.candidates(wanted, CandidateKeys::All);
            for index in candidates.iter() {
                let entry = self.entry(index);
                if context.excluded_id != Some(entry.row.id)
                    && let Some(rank) = entry.definition_rank(wanted, purpose, word_forms, self)
                    && best_rank.is_none_or(|current| current < rank)
                    && entry.guard_passes(context)
                    && let Some(reply) = entry.row.answer(context)
                {
                    best = Some((entry, reply));
                    best_rank = Some(rank);
                }
            }
            if best
                .as_ref()
                .is_some_and(|(entry, _)| !(entry.row.explain && entry.row.report))
            {
                return best;
            }
            // An authored card can answer directly without naming itself in the
            // opening sentence ("Plants use sunlight..." for photosynthesis).
            // Its first key group must name this exact subject, and another group
            // must explicitly account for the wording of the question.
            // A condition that also accepts reports can give a definition from
            // one signal. Give a card with both authored signals first refusal.
            return self
                .best_scored(&candidates, word_forms, words, false, context, |entry| {
                    entry.names_subject(wanted)
                        || matches!(
                            entry.row.callback,
                            Callback::HomePractical(home_practical::Craft::Pattern)
                        ) && home_practical::named_pattern_term(context.reading).is_some()
                        || context.select_only
                            && matches!(entry.row.callback, Callback::DishDefinition)
                })
                .or(best);
        }

        self.best_scored(
            &self.candidates(word_forms, CandidateKeys::Scoring),
            word_forms,
            words,
            false,
            context,
            |_| true,
        )
    }

    fn best_scored(
        &self,
        candidates: &CandidateIds,
        word_forms: &[Vec<Form>],
        words: &[String],
        statement: bool,
        context: &guards::Context<'_>,
        matches_subject: impl Fn(&Entry<'_>) -> bool,
    ) -> Option<Match<'_>> {
        let mut best = None;
        let mut best_score = None;
        let focus = self.question_focus(context.reading);
        let question_scene = context
            .reading
            .context_fields
            .question_scene
            .and_then(|scene| scene.names(&context.reading.entities));
        for index in candidates.iter() {
            let entry = self.entry(index);
            if context.excluded_id == Some(entry.row.id) || !matches_subject(&entry) {
                continue;
            }
            let typed = &context.reading.line.words;
            let rank = if entry.row.id == "spelling-pick-one" && words != typed {
                let typed_forms = typed
                    .iter()
                    .map(|word| self.indexed_forms(word))
                    .collect::<Vec<_>>();
                entry.score(&typed_forms, typed, statement, context)
            } else {
                entry.score(word_forms, words, statement, context)
            };
            let Some(score) = rank else {
                continue;
            };
            let mut answers_ask_without_scene = true;
            if let Some((asked, setting)) = question_scene
                && entry.about_subject(setting)
            {
                if !entry.about_subject(asked) {
                    continue;
                }
                // Both subjects can be necessary to answer. Compare these
                // cards if no card about the asking clause alone answers.
                answers_ask_without_scene = false;
            }
            let score = (
                answers_ask_without_scene,
                entry.row.domain == "travel"
                    && entry.row.id == "airport-arrival"
                    && super::airport_arrival::asks_when_to_arrive(context.reading)
                    || (entry.row.domain, entry.row.id) == ("pets", "pet-chews-cables"),
                focus
                    .as_ref()
                    .map_or(0, |focus| focus.longest_key_run(&entry)),
                score,
            );
            // Guards read the same context and may initialize expensive matchers.
            // Evaluate them only for a card that can replace the current answer.
            if best_score.is_none_or(|current| current < score)
                && entry.guard_passes(context)
                && let Some(reply) = entry.row.answer(context)
            {
                best = Some((entry, reply));
                best_score = Some(score);
            }
        }
        best
    }
}

fn typo_bucket(word: &str) -> Option<(u8, usize)> {
    let first = *word.as_bytes().first()?;
    word.bytes()
        .all(|byte| byte.is_ascii_lowercase())
        .then_some((first, word.len()))
}

fn pets_buying_shape(reading: &Reading<'_>) -> bool {
    static BUYING: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(
            r"\b(?:need|needs|needed|want|wants|get|gets|buy|buys|getting|buying)\b[\w\s,']{0,24}\b(?:for|before)\b",
        )
        .expect("source pet buying shape")
    });
    BUYING.is_match(&reading.line.words.join(" ").to_lowercase())
}

fn question_opener(word: &str) -> bool {
    matches!(
        word,
        "what"
            | "how"
            | "why"
            | "when"
            | "where"
            | "which"
            | "who"
            | "can"
            | "could"
            | "will"
            | "would"
            | "should"
            | "is"
            | "are"
            | "does"
            | "do"
            | "did"
            | "explain"
            | "describe"
            | "compare"
    )
}

fn intersects(left: &[Form], right: &[Form]) -> bool {
    left.iter().any(|form| right.contains(form))
}

fn without_plural_s(word: &str) -> &str {
    let stem = word.trim_end_matches(['s', 'S']);
    if stem.is_empty() { word } else { stem }
}

impl Entry<'_> {
    fn about_subject(&self, subject: &str) -> bool {
        let Some(head) = subject.split_whitespace().last().map(without_plural_s) else {
            return false;
        };
        self.row
            .key
            .iter()
            .flat_map(|group| group.iter())
            .flat_map(|phrase| phrase.split_whitespace())
            .any(|word| without_plural_s(word).eq_ignore_ascii_case(head))
    }

    fn guard_passes(&self, context: &guards::Context<'_>) -> bool {
        if matches!(
            self.row.callback,
            Callback::MealShortlist | Callback::MealClock
        ) && super::meal_shortlist::clock::indexed_request(
            self.row.domain,
            self.row.id,
            context.reading,
            context.memory,
        ) {
            return true;
        }
        if matches!(self.row.callback, Callback::MealClock) {
            return false;
        }
        if matches!(self.row.callback, Callback::MealShortlist) {
            return context.earlier.is_empty()
                && (super::meal_shortlist::is_open_dinner(context.reading)
                    || self.row.domain == "cooking"
                        && super::meal_shortlist::plant_ask(context.reading).is_some()
                        && context.memory.is_none_or(|memory| {
                            super::meal_shortlist::plant_ask(context.reading).is_some_and(|diet| {
                                !memory.pantry.blocks_plant_offer(memory, diet.vegan())
                            })
                        }));
        }
        if self.row.id == "plan-start" && dish_plan::tentative_recipe(context.reading) {
            return false;
        }
        if self.row.id == "plan-ideas" && dish_choices::free_time(context) {
            return false;
        }
        if self.row.domain == "travel"
            && self.row.id == "where-to-stay-there"
            && travel_stays::airport_stay_without_destination(context.reading)
        {
            // The current airport's lodging has its own key. A city carried from
            // the previous turn should not take that booking.
            return false;
        }
        !(matches!(
            self.row.callback,
            Callback::VegetableRoast | Callback::VegetableWatery
        ) && vegetables::defers_to_food_safety(context.reading))
            && self.row.guard.passes(
                context,
                self.row.id,
                self.row.key.first().copied().unwrap_or_default(),
            )
    }

    fn names_subject(&self, wanted: &[Vec<Form>]) -> bool {
        self.key
            .first()
            .is_some_and(|group| group.iter().any(|phrase| phrase.matches(wanted)))
    }

    fn fences_pass(&self, line: &[Vec<Form>]) -> bool {
        (self.ask.is_empty() || self.ask.iter().any(|phrase| phrase.occurs(line)))
            && !self.avoid.iter().any(|phrase| phrase.occurs(line))
    }

    fn definition_rank(
        &self,
        wanted: &[Vec<Form>],
        purpose: bool,
        line: &[Vec<Form>],
        database: &Database,
    ) -> Option<(bool, std::cmp::Reverse<usize>)> {
        if !self.fences_pass(line) {
            return None;
        }
        let names_subject = self.names_subject(wanted);
        if !(names_subject
            || self
                .key
                .iter()
                .flat_map(Group::iter)
                .any(|phrase| phrase.matches(wanted)))
        {
            return None;
        }
        let explicit = (names_subject
            && (self.row.explain || matches!(self.row.callback, Callback::DishDefinition)))
            || matches!(self.row.callback, Callback::Language) && self.row.id == "word-meaning";
        if !explicit {
            const DEFINING: &str = "is are was were means mean refers gives give costs cost covers cover keeps keep works work helps help makes make takes take needs need holds hold comes come runs run lets let stops stop protects protect replaces replace counts count serves serve";
            const NON_SUBJECT: &str = "of in on with at from your my our their its his her cup mug glass slice piece bit bowl plate packet tin can bottle spoon teaspoon tablespoon handful pinch taste sip bite";
            let contains = |word: &[Form], lexicon: &str| {
                lexicon.split_ascii_whitespace().any(|spelling| {
                    database
                        .forms
                        .get(spelling)
                        .is_some_and(|id| word.contains(id))
                })
            };
            let in_reply = self.opening;
            let openings = in_reply
                .len()
                .checked_sub(wanted.len())
                .map_or(0, |remaining| 1 + remaining.min(6));
            let defines = (0..openings).any(|start| {
                wanted
                    .iter()
                    .zip(in_reply.words().skip(start))
                    .all(|(word, reply)| intersects(word, reply))
                    && (purpose
                        || ((start == 0
                            || in_reply
                                .word(0)
                                .is_some_and(|word| contains(word, "a an the")))
                            && !in_reply
                                .words()
                                .take(start)
                                .any(|word| contains(word, "is are was were"))
                            && !in_reply
                                .words()
                                .take(start)
                                .next_back()
                                .is_some_and(|word| contains(word, NON_SUBJECT))
                            && in_reply
                                .word(start + wanted.len())
                                .is_some_and(|word| contains(word, DEFINING))))
            });
            if !defines {
                return None;
            }
        }
        let alternatives = self.key.iter().map(Group::len).sum();
        Some((explicit, std::cmp::Reverse(alternatives)))
    }

    fn score(
        &self,
        line: &[Vec<Form>],
        words: &[String],
        statement: bool,
        context: &guards::Context<'_>,
    ) -> Option<(i32, usize)> {
        if self.row.domain == "idioms" {
            // The authored rows repeat the same phrase in each of three parts.
            // Each part scores when the same multiword span has already matched.
            let matched = self
                .key
                .first()?
                .iter()
                .flat_map(|phrase| phrase.spans(line))
                .any(|(start, end)| end - start >= 2);
            return (matched && self.fences_pass(line))
                .then(|| (3 * self.key.len() as i32 + self.row.bias, self.key.len()));
        }
        let mut covered = vec![false; line.len()];
        let mut score: i32 = 0;
        let mut specificity = 0;
        let mut groups_hit = 0;
        let mut first_hit = false;
        let mut first_width = 0;
        let suppress_router_buy_frame =
            self.row.id == "router-buy" && technology_setup::asks_to_remove(words);
        let puncture_removal =
            self.row.id == "bike-puncture" && technology_setup::asks_to_remove(words);
        let kinds = matches!(self.row.guard, guards::Guard::Kinds);
        let stage = matches!(self.row.callback, Callback::Baby(babyfood::Reply::Stage));
        let compares =
            matches!(self.row.callback, Callback::Difference) || self.row.id == "spelling-pick-one";
        let weekend = self.row.domain == "entertainment" && self.row.id == "weekend-do";
        let leisure_plan = self.row.domain == "plans"
            && matches!(self.row.id, "free-time-plan" | "cheap-things-to-do");
        let discovery = self.row.domain == "science" && self.row.id == "who-discovered";
        let prior_history = self.row.domain == "history" && self.row.id == "the-past-follow-on";
        let walking_pace = self.row.domain == "fitness" && self.row.id == "walking-pace";
        let bicycle_brakes = self.row.id == "bike-brakes-rub";
        // The carried clothing-size card names the prior size with a pronoun.
        let refers_to_size = matches!(
            self.row.callback,
            Callback::ClothingWear(clothing_wear::Reply::Size)
        );
        // Source matching can reuse a covered position as the last choice for a later group.
        // These cards rely on that rule when the first key already named the later signal.
        let reuse_positions = matches!(self.row.callback, Callback::TravelStays)
            && matches!(
                self.row.id,
                "where-to-stay-there" | "where-to-stay-anywhere" | "is-it-good-for-families"
            )
            || walking_pace
            || bicycle_brakes
            || matches!(self.row.id, "meal-plan" | "habit-restart")
            || matches!(
                self.row.callback,
                Callback::PickyEater
                    | Callback::PresentationNerves
                    | Callback::MealShortlist
                    | Callback::NightWaking
            )
            || matches!(self.row.callback, Callback::Language) && self.row.id == "regular-past"
            || matches!(
                self.row.callback,
                Callback::Gift(gift_cards::Reply::Condition)
            ) && self.row.id == "gift-leaving-work"
            || matches!(
                self.row.id,
                "card-birthday" | "putting-things-off" | "follow-up-email" | "steps-in-a-distance"
            );
        for (index, group) in self.key.iter().enumerate() {
            let has_new_position = |start, end| covered[start..end].iter().any(|&hit| !hit);
            let matched = group
                .iter()
                .enumerate()
                .flat_map(|(order, phrase)| {
                    phrase
                        .spans(line)
                        .take(if bicycle_brakes { 1 } else { usize::MAX })
                        .map(move |(start, end)| (start, end, phrase.unique_in_domain, order))
                })
                .filter(|&(start, end, _, _)| reuse_positions || has_new_position(start, end))
                // In a removal frame, the buying row needs more than an isolated
                // subject and phrasing such as "what" or "should I get".
                .filter(|&(_, _, _, order)| {
                    if index == 0 || first_width >= 2 {
                        return true;
                    }
                    let key = self.row.key[index][order];
                    !(suppress_router_buy_frame && matches!(key, "what" | "which" | "should i get")
                        || puncture_removal && matches!(key, "how" | "get" | "got" | "getting"))
                })
                .filter(|&(start, end, _, _)| {
                    !self.row.grouped
                        || index != 0
                        || kinds
                        || stage
                        || compares
                        || refers_to_size
                        || weekend
                        || discovery
                        || prior_history
                        || words[start..end].iter().any(|word| content(word))
                })
                .max_by_key(|&(start, end, _, order)| {
                    let frame_word =
                        bicycle_brakes && matches!(self.row.key[index][order], "how" | "when");
                    (
                        has_new_position(start, end),
                        end - start,
                        !frame_word,
                        // The source ranks equal brake keys in authored order.
                        std::cmp::Reverse(if bicycle_brakes { order } else { start }),
                        // Variants that match the same span retain authored order.
                        // Their distinct authored words can have different rarity.
                        std::cmp::Reverse(order),
                    )
                });
            match matched {
                Some((start, end, unique_in_domain, _)) => {
                    if index == 0 {
                        first_hit = true;
                        first_width = end - start;
                    }
                    groups_hit += 1;
                    covered[start..end].fill(true);
                    specificity += unique_in_domain;
                    score += if end - start == 1 { 2 } else { 3 };
                }
                None if self.row.grouped && index == 0 => return None,
                None => {}
            }
        }
        let positions = covered.iter().filter(|&&bit| bit).count();
        let content_matches = words
            .iter()
            .zip(&covered)
            .filter(|(word, hit)| **hit && content(word))
            .count();
        let report = self.row.report && report_shape(words, &covered);
        let shaped = first_hit
            && ((matches!(self.row.guard, guards::Guard::FunQuiz)
                && context.reading.features.wants_a_quiz)
                || self
                    .row
                    .shape
                    .iter()
                    .any(|shape| shape.matches(context, self.row.key[0])));
        if statement
            && content_matches < 2
            // A bicycle report can reach either authored card with its part
            // and "when". The calendar callback still checks the timing.
            && !bicycle_brakes
            && self.row.id != "whenbuy-bike"
            && self.row.id != "five-a-day"
            && !report
            && !shaped
            && !kinds
            && !leisure_plan
            && !self.row.callback.answers_statements()
        {
            return None;
        }
        let normal_match = score >= 4
            && positions >= 2
            && content_matches >= 1
            && (!self.row.grouped || groups_hit >= 2);
        let tail_deduction = if weekend && normal_match {
            entertainment_time::weekend_rank_deduction(self.key, self.row.key, line, words)?
        } else {
            0
        };
        if leisure_plan {
            return (normal_match && self.fences_pass(line))
                .then(|| leisure::rank(self.row, self.key, line, words))
                .flatten()
                .map(|rank| (rank, self.key.len()));
        }
        ((normal_match || report || shaped) && self.fences_pass(line))
            .then(|| {
                let has_first_taste = self.row.id == "babyfood-first-taste"
                    && babyfood::introduces_a_food(context.baby_line());
                let explicit_implant_life = matches!(self.row.callback, Callback::JointImplant)
                    && covered.iter().zip(words).any(|(&hit, word)| {
                        hit && matches!(word.as_str(), "last" | "lasts" | "lifetime" | "revision")
                    });
                let score = score - tail_deduction
                    + if report
                        || shaped && !matches!(self.row.id, "card-birthday" | "card-something")
                        || has_first_taste
                    {
                        2
                    } else {
                        0
                    }
                    + i32::from(explicit_implant_life);
                // The source ranks distinctive words within the card's domain.
                let specificity = i32::try_from(specificity).unwrap_or(i32::MAX);
                let rank = if matches!(
                    self.row.callback,
                    Callback::Rice
                        | Callback::VegetableRoast
                        | Callback::VegetableWatery
                        | Callback::MealShortlist
                ) || matches!(self.row.id, "pick-rice" | "plan-start")
                {
                    authored_rank::rank(self, line, words)
                } else {
                    score
                        .saturating_add(self.row.bias)
                        .saturating_add(specificity)
                };
                (rank, self.key.len())
            })
            // The announcement and generic wording card can reach one point
            // on weak keys. The source excludes that score from selection.
            .filter(|(rank, _)| {
                !matches!(self.row.id, "plan-start" | "card-something") || *rank > 1
            })
    }
}

fn report_shape(words: &[String], covered: &[bool]) -> bool {
    fn normalized(word: &str) -> &str {
        match word {
            "i'm" => "im",
            "i've" => "ive",
            _ => word,
        }
    }
    let Some(first) = covered.iter().position(|&seen| seen) else {
        return false;
    };
    let last = covered
        .iter()
        .rposition(|&seen| seen)
        .expect("matched at least one word");
    let before = &words[..first];
    let lead = before
        .iter()
        .any(|word| REPORT_LEADS.contains(&normalized(word)));
    if lead
        && before
            .iter()
            .all(|word| REPORT_FILLER.contains(&normalized(word)))
    {
        return true;
    }
    let after = &words[last + 1..];
    !after.is_empty()
        && after.iter().all(|word| {
            let word = normalized(word);
            REPORT_HAPPENING_AGAIN.contains(&word) || REPORT_FILLER.contains(&word)
        })
}

fn has_ascii_phrase(text: &str, phrase: &str) -> bool {
    let text_bytes = text.as_bytes();
    text.match_indices(phrase).any(|(start, _)| {
        (start == 0 || !text_bytes[start - 1].is_ascii_alphanumeric())
            && text_bytes
                .get(start + phrase.len())
                .is_none_or(|value| !value.is_ascii_alphanumeric())
    })
}

// Some source callbacks look for a literal space before and after a phrase.
fn has_space_delimited_phrase(text: &str, phrase: &str) -> bool {
    let bytes = text.as_bytes();
    text.match_indices(phrase).any(|(start, _)| {
        (start == 0 || bytes[start - 1] == b' ')
            && bytes
                .get(start + phrase.len())
                .is_none_or(|byte| *byte == b' ')
    })
}

fn content(word: &str) -> bool {
    !matches!(
        word,
        "a" | "an"
            | "the"
            | "it"
            | "this"
            | "that"
            | "and"
            | "or"
            | "of"
            | "in"
            | "on"
            | "at"
            | "to"
            | "for"
            | "with"
            | "my"
            | "your"
            | "i"
            | "you"
            | "we"
            | "is"
            | "are"
            | "was"
            | "were"
            | "do"
            | "does"
            | "what"
            | "which"
            | "who"
            | "where"
            | "when"
            | "why"
            | "how"
            | "can"
            | "could"
            | "will"
            | "would"
            | "should"
            | "me"
            | "long"
            | "much"
            | "many"
            | "time"
            | "best"
            | "some"
            | "any"
    )
}

fn named_question_head(words: &[String]) -> Option<usize> {
    Some(match words.first()?.as_str() {
        "so" | "ok" | "okay" | "and" | "but" => 1,
        _ => 0,
    })
}

fn material_subject(words: &[String]) -> Option<(usize, usize, usize)> {
    let head = named_question_head(words)?;
    let mut start = match words.get(head)?.as_str() {
        "what" if matches!(words.get(head + 1)?.as_str(), "is" | "are" | "was" | "were") => {
            head + 2
        }
        "what's" | "whats" => head + 1,
        _ => return None,
    };
    // These composition phrases are authored in the pinned ORIGIN_ASK.
    let tail = words
        .iter()
        .rposition(|word| !matches!(word.as_str(), "exactly" | "please" | "again"))?
        + 1;
    let end = match words.get(start..tail)? {
        [.., made, preposition]
            if made == "made" && matches!(preposition.as_str(), "of" | "from" | "with") =>
        {
            tail - 2
        }
        [.., made, out, of] if made == "made" && out == "out" && of == "of" => tail - 3,
        _ => return None,
    };
    if start < end
        && matches!(
            words[start].as_str(),
            "a" | "an" | "the" | "my" | "your" | "our" | "their" | "his" | "her" | "its"
        )
    {
        start += 1;
    }
    (start < end).then_some((start, end, tail))
}

fn definition_subject(words: &[String]) -> Option<(usize, usize, bool)> {
    let mut start = named_question_head(words)?;
    let mut end = words.len();
    let mut purpose = false;
    match words.get(start)?.as_str() {
        "what" => {
            start += 2;
            match words.get(start - 1)?.as_str() {
                "is" | "are" | "was" | "were" => {}
                "does" | "do" if words.last()?.as_str() == "mean" => end -= 1,
                _ => return None,
            }
        }
        "what's" | "whats" | "define" => start += 1,
        _ => return None,
    }
    while start < end && matches!(words[start].as_str(), "a" | "an" | "the") {
        start += 1;
    }
    while start < end
        && matches!(
            words[end - 1].as_str(),
            "for" | "exactly" | "please" | "again"
        )
    {
        purpose |= words[end - 1] == "for";
        end -= 1;
    }
    if !(1..=3).contains(&end.saturating_sub(start))
        || matches!(words.get(start..end), Some([form, connector, ..])
            if matches!(form.as_str(), "plural" | "plurals") && matches!(connector.as_str(), "of" | "for"))
        || words[start..end].iter().any(|word| {
            matches!(
                word.as_str(),
                "it" | "that"
                    | "this"
                    | "he"
                    | "she"
                    | "they"
                    | "there"
                    | "wrong"
                    | "broken"
                    | "stuck"
                    | "pain"
                    | "hurting"
                    | "smells"
                    | "best"
            )
        })
    {
        return None;
    }
    Some((start, end, purpose))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_ids_match_btree_order_across_word_boundaries() {
        let high = 30_001;
        let indices = [high, 63, 64, 1, 64, 128, high, 0, 127, 63];
        let expected = indices.into_iter().collect::<BTreeSet<_>>();
        let candidates = CandidateIds::from_indices(high + 1, indices.into_iter());
        assert!(candidates.iter().eq(expected.iter().copied()));
        assert!(candidates.iter().eq(expected));
        assert!(
            CandidateIds::from_indices(high + 1, std::iter::empty())
                .iter()
                .next()
                .is_none()
        );
    }

    #[test]
    fn database_candidates_match_btree_postings() {
        for input in [
            "",
            "flompernackle",
            "how do i know what paint to buy",
            "paint paint paint brushes painted",
            "what are the brake pads made of",
        ] {
            let word_forms = tokens(input)
                .iter()
                .map(|word| DATABASE.indexed_forms(word))
                .collect::<Vec<_>>();
            let expected = word_forms
                .iter()
                .flatten()
                .flat_map(|&form| DATABASE.by_first_form.get(form))
                .copied()
                .map(Posting::index)
                .collect::<BTreeSet<_>>();
            assert!(
                DATABASE
                    .candidates(&word_forms, CandidateKeys::All)
                    .iter()
                    .eq(expected),
                "{input}"
            );
        }
    }

    #[test]
    fn equal_spans_use_the_first_authored_alternative_for_rarity() {
        let timing = lookup("Can you tell me whether my train is delayed right now?").unwrap();
        assert_eq!(
            (timing.domain, timing.id),
            ("consumer", "consumer-train-delay")
        );
        let delay = lookup("Is the train delayed?").unwrap();
        assert_eq!((delay.domain, delay.id), ("money", "train-delay"));
    }

    #[test]
    fn a_named_question_takes_precedence_over_the_scene() {
        let input = "We have a cat and a baby, what should we feed the dog?";
        let read = Reading::new(input);
        assert_eq!(read.context_fields.subject.as_deref(), Some("cat"));
        assert_eq!(
            read.context_fields
                .question_scene
                .and_then(|scene| scene.names(&read.entities)),
            Some(("dog", "cat"))
        );
        let card = lookup(input).unwrap();
        assert_eq!((card.domain, card.id), ("pets", "dog-feed-amount"));
        let turn = crate::Bot::new(Some("0")).unwrap().turn(input);
        assert_eq!(turn.module, Some("answers"));
        assert!(turn.reply.contains("an adult dog"), "{}", turn.reply);
    }

    #[test]
    fn genuine_comparisons_and_relevant_context_keep_their_cards() {
        for (input, expected) in [
            ("Should I get a cat or a dog?", ("opinions", "cats-dogs")),
            (
                "My wife is vegetarian, what should I cook for dinner?",
                ("cooking", "vegetarian-guest"),
            ),
            (
                "I have a cat, how can I stop my dog stealing the cat food?",
                ("pets", "one-pets-dinner"),
            ),
        ] {
            let card = lookup(input).unwrap_or_else(|| panic!("no card for {input}"));
            assert_eq!((card.domain, card.id), expected, "{input}");
        }
        let missing = Reading::new("I haven't got a blender, can I still make soup?");
        assert!(missing.context_fields.question_scene.is_none());
    }

    #[test]
    fn a_removal_word_does_not_make_a_puncture_from_the_question_frame() {
        for (ask, expected) in [
            (
                "How do I check my bike tyres before I set off?",
                "cardio-or-weights-first",
            ),
            ("How do I check my bike tyres?", "bike-puncture"),
            (
                "How do I change my bike tyres before I set off?",
                "bike-puncture",
            ),
        ] {
            assert_eq!(lookup(ask).map(|card| card.id), Some(expected), "{ask}");
        }
    }

    #[test]
    fn authored_static_scans_preserve_order_fallback_and_abstention() {
        for &(domain, id, input, expected) in static_table_selection::CASES {
            let reading = Reading::new(input);
            let context = guards::Context::new(&reading, &[]);
            let row = rows()
                .find(|row| row.domain == domain && row.id == id)
                .unwrap_or_else(|| panic!("missing {domain}/{id}"));
            assert!(matches!(row.callback, Callback::Text(_)));
            let response = row.answer(&context);
            assert_eq!(
                response.as_deref().filter(|reply| !reply.is_empty()),
                expected,
                "{domain}/{id}: {input}"
            );
        }
    }

    #[test]
    fn wording_and_word_forms_reach_authored_language_cards() {
        for (input, id, opening) in [
            (
                "what does ambiguous mean",
                "word-meaning",
                "Ambiguous means it can be read two ways",
            ),
            (
                "past tense of travel",
                "regular-past",
                "Travel -> travelled.",
            ),
            (
                "what is the plural of child",
                "plural-word",
                "Children is the plural of child.",
            ),
        ] {
            let answer = lookup(input).unwrap_or_else(|| panic!("no card for {input}"));
            assert_eq!((answer.domain, answer.id), ("language", id), "{input}");
            assert!(answer.reply.starts_with(opening), "{input}: {answer:?}");
        }
    }

    #[test]
    fn material_questions_use_the_authored_subject_and_ask() {
        for (input, id, detail) in [
            ("What is a scab made of?", "body-what-scab", "fibrin"),
            ("What’s a scab made from?", "body-what-scab", "fibrin"),
            (
                "What are tonsils made of?",
                "body-what-tonsils",
                "lymph tissue",
            ),
            (
                "What are tonsils made out of?",
                "body-what-tonsils",
                "lymph tissue",
            ),
            ("What is the Earth made of?", "earth-inside", "rocky crust"),
            (
                "What is electricity made from?",
                "electricity-made",
                "generator",
            ),
            ("What is glass made with?", "glass", "sand"),
            ("What is paper made of?", "paper", "wood"),
            ("What is the Sun made of?", "sun-made-of", "hydrogen"),
        ] {
            let card = lookup(input).unwrap_or_else(|| panic!("no card for {input}"));
            assert_eq!((card.domain, card.id), ("science", id), "{input}");
            let mut bot = crate::Bot::new(Some("0")).unwrap();
            let turn = bot.turn(input);
            assert_eq!(turn.module, Some("answers"), "{input}: {}", turn.reply);
            assert!(turn.reply.contains(detail), "{input}: {}", turn.reply);
        }
        assert!(lookup("What is a warranty made of?").is_none());
    }

    #[test]
    fn body_definitions_keep_authored_symptom_and_itch_questions_separate() {
        for (input, expected) in [
            ("What are tonsils?", ("science", "body-what-tonsils")),
            ("What do tonsils do?", ("science", "body-what-tonsils")),
            ("What is tonsillitis?", ("health", "tonsillitis")),
            (
                "My tonsils are swollen and it hurts to swallow",
                ("health", "tonsillitis"),
            ),
            ("Why does my scab itch?", ("science", "body-what-scab-itch")),
            ("My scab is red and weeping", ("health", "cut-infected")),
        ] {
            let card = lookup(input).unwrap_or_else(|| panic!("no card for {input}"));
            assert_eq!((card.domain, card.id), expected, "{input}");
        }
    }

    #[test]
    fn named_tonsillitis_questions_get_symptoms_without_taking_anatomy() {
        for input in [
            "What are the symptoms of tonsillitis?",
            "Tell me the common signs of tonsillitis",
            "Tonsillitis symptoms",
            "What symptoms does tonsillitis cause?",
            "Can tonsillitis cause difficulty breathing?",
        ] {
            let card = lookup(input).unwrap_or_else(|| panic!("no card for {input}"));
            assert_eq!((card.domain, card.id), ("health", "tonsillitis"), "{input}");
            let turn = crate::Bot::new(Some("0")).unwrap().turn(input);
            assert_eq!(turn.module, Some("answers"), "{input}: {}", turn.reply);
            assert!(
                turn.reply.contains("red, swollen tonsils"),
                "{input}: {}",
                turn.reply
            );
            assert!(turn.reply.contains("Call 999"), "{input}: {}", turn.reply);
        }
        for (input, text) in [
            ("What is tonsillitis?", "infection of the tonsils"),
            ("What are tonsils?", "lymph tissue"),
            ("What are tonsils made of?", "lymph tissue"),
        ] {
            let turn = crate::Bot::new(Some("0")).unwrap().turn(input);
            assert_eq!(turn.module, Some("answers"), "{input}: {}", turn.reply);
            assert!(turn.reply.contains(text), "{input}: {}", turn.reply);
        }
    }

    #[test]
    fn a_named_stitch_with_a_craft_qualifier_reaches_its_definition() {
        let matched = lookup("what is ssk in knitting").expect("knitting definition");
        assert_eq!(
            (matched.domain, matched.id),
            ("home", "read-a-craft-pattern")
        );
        assert!(
            matched
                .reply
                .starts_with("ssk means slip one stitch knitwise")
        );
        assert!(
            lookup("what is snk in knitting")
                .is_none_or(|matched| matched.id != "read-a-craft-pattern")
        );
    }

    #[test]
    fn timing_and_animal_costs_win_over_unrelated_cards() {
        for (input, domain, id, starts_with) in [
            (
                "how long does it take to cooking pasta",
                "pasta",
                "pasta-time",
                "Dried pasta is 8 to 12 minutes",
            ),
            (
                "how long does it take to learning to drive",
                "cars",
                "learn-to-drive-hours",
                "Most people need around 45 hours of lessons",
            ),
            (
                "how much would the dog cost",
                "pets",
                "pet-cost",
                "A puppy is about 800 to 3,000 pounds",
            ),
            (
                "how much does the cat cost",
                "pets",
                "pet-cost",
                "A kitten is about 50 to 150 pounds",
            ),
        ] {
            let answer = lookup(input).unwrap_or_else(|| panic!("no card for {input}"));
            assert_eq!((answer.domain, answer.id), (domain, id), "{input}");
            assert!(answer.reply.starts_with(starts_with), "{input}: {answer:?}");
        }
    }

    #[test]
    fn authored_pasta_cards_win_over_general_cooking_cards() {
        for (input, id) in [
            ("what are the different types of pasta", "pasta-kinds"),
            ("how long will spaghetti last in the fridge", "pasta-keep"),
        ] {
            assert_eq!(lookup(input).map(|card| card.id), Some(id), "{input}");
        }
    }

    #[test]
    fn normalized_question_forms_reach_the_same_acronym_card() {
        for input in [
            "what's that in tablespoons?",
            "what is that in tablespoons?",
            "what's tbsp?",
        ] {
            assert_eq!(
                lookup(input).map(|card| card.id),
                Some("explain-tbsp-name"),
                "{input}"
            );
        }
        assert_ne!(
            lookup("what about tablespoons?").map(|card| card.id),
            Some("explain-tbsp-name")
        );
    }

    #[test]
    fn bake_and_child_book_requests_select_the_authored_provider() {
        let bake = lookup("how do i make a loaf of bread").unwrap();
        assert_eq!(bake.id, "bakes");
        assert!(bake.reply.starts_with("500g strong white flour"));

        let book = lookup("a good book for my daughter aged 12").unwrap();
        assert_eq!(book.id, "book-for-a-child");
        assert!(book.reply.starts_with("Ten to twelve"));
    }

    #[test]
    fn raw_card_selection_can_count_an_entry_that_declines_to_reply() {
        let input = "what filling for cupcakes";
        assert_eq!(matched_id(&Reading::new(input)), Some("recipe-fairy-cakes"));
        assert_ne!(
            lookup(input).map(|card| card.id),
            Some("recipe-fairy-cakes")
        );
    }

    #[test]
    fn authored_dish_definitions_precede_storage_and_decline_equipment() {
        let card = lookup("what is halloumi").unwrap();
        assert_eq!(card.id, "what-is-halloumi");
        assert!(card.reply.starts_with("Halloumi is a Cypriot cheese"));

        let equipment = Reading::new("what is a pie dish");
        assert_eq!(matched_id(&equipment), Some("what-is-pie"));
        assert_ne!(
            lookup_reading(&equipment).map(|card| card.id),
            Some("what-is-pie")
        );
    }

    #[test]
    fn authored_drink_cards_expose_the_pinned_cake_guard_selection() {
        for (input, expected) in [
            (
                "what can i put between the layers of a coffee cake",
                "drink-what-it-is",
            ),
            ("what cream goes in a coffee cake", "drink-goes-with"),
            ("what filling for a coffee cake", "drink-what-it-is"),
        ] {
            assert_eq!(matched_id(&Reading::new(input)), Some(expected), "{input}");
        }
    }

    #[test]
    fn course_changes_need_a_typed_or_rewritten_ask() {
        for input in [
            "can i switch courses at uni in first year",
            "what about I changed the subject",
        ] {
            assert_eq!(
                lookup(input).map(|card| card.id),
                Some("change-subject-or-course"),
                "{input}"
            );
        }
        assert!(lookup("I changed the subject").is_none());
    }

    #[test]
    fn medicine_cards_retain_their_pinned_triggers_with_leaflet_or_pharmacist_replies() {
        for (input, domain, id, starts_with) in [
            (
                "How long should I wait between an antacid and my other tablets?",
                "health",
                "medicine-together-antacid-tablets",
                "Check both medicine leaflets, or ask a pharmacist",
            ),
            (
                "Can I take ibuprofen with sertraline?",
                "health",
                "medicine-together-antidepressant-ibuprofen",
                "Check the leaflets for both medicines and ask a pharmacist",
            ),
            (
                "I forgot my contraceptive pill last night. What should I do?",
                "science",
                "missed-contraceptive-pill",
                "Check the missed-pill instructions in the leaflet for your exact pill.",
            ),
        ] {
            let answer = lookup(input).unwrap_or_else(|| panic!("no card for {input}"));
            assert_eq!((answer.domain, answer.id), (domain, id), "{input}");
            assert!(answer.reply.starts_with(starts_with), "{input}: {answer:?}");
        }
    }

    #[test]
    fn codeine_abstention_removes_the_three_duplicate_source_replies() {
        for (domain, id) in [
            ("health", "medicine-in-pregnancy"),
            ("body", "painkillers-breastfeeding"),
            ("body", "what-can-i-take-pregnant-or-feeding"),
        ] {
            assert!(by_id(domain, id).is_none(), "{domain}/{id}");
        }
        assert!(by_id("body", "painkillers-pregnancy").is_some());
    }

    #[test]
    fn indexed_rows_answer_named_topics_and_require_a_shared_subject() {
        for (text, expected) in [
            ("what is a food bank", "food-bank"),
            ("what is a semicolon for", "semicolon"),
            ("what is photosynthesis?", "photosynthesis"),
            ("why do onions make you cry?", "onion-cry"),
            (
                "Explain the difference between affect and effect.",
                "affect-vs-effect",
            ),
            ("What does BTW mean?", "explain-btw-name"),
            ("It came out of the blue.", "idiom-out-of-the-blue"),
            ("How long does a house survey take?", "time-house-survey"),
            ("How long should I boil eggs?", "egg-boil"),
            (
                "my kettle is full of limescale, how do i get rid of it?",
                "descale-kettle",
            ),
            ("how do I descale a kettel?", "descale-kettle"),
        ] {
            assert_eq!(lookup(text).map(|entry| entry.id), Some(expected), "{text}");
        }
    }

    #[test]
    fn generated_guard_distinguishes_worn_clothing_from_a_painful_joint() {
        let row = rows()
            .find(|row| row.domain == "cleaning" && row.id == "mend-hole")
            .expect("generated garment repair card");
        assert!(matches!(row.guard, guards::Guard::Text(_)));
        for (text, applies) in [
            ("My jeans are worn through at the knee.", true),
            ("My coat has frayed cuffs.", true),
            ("My coat is tatty.", true),
            ("My knee has worn out.", false),
            ("My ankle hurts.", false),
        ] {
            let reading = Reading::new(text);
            let context = guards::Context::new(&reading, &[]);
            assert_eq!(
                row.guard.passes(&context, row.id, row.key[0]),
                applies,
                "{text}"
            );
        }
        assert_eq!(
            lookup("my coat is tatty").map(|card| card.id),
            Some("mend-hole")
        );
        assert!(lookup("the sofa is tatty").is_none_or(|card| card.id != "mend-hole"));
    }

    #[test]
    fn generated_reply_preserves_the_branch_and_default() {
        let row = rows()
            .find(|row| row.domain == "language" && row.id == "comma-rules")
            .expect("generated punctuation card");
        assert!(matches!(row.callback, Callback::Text(_)));
        for (text, start) in [
            (
                "Where does the comma go around 'however'?",
                "Put the comma after however",
            ),
            ("How do commas work?", "A comma marks a pause"),
        ] {
            let reading = Reading::new(text);
            let context = guards::Context::new(&reading, &[]);
            let answer = row.answer(&context).expect("punctuation explanation");
            assert!(matches!(answer, Cow::Borrowed(_)));
            assert!(answer.starts_with(start), "{answer}");
        }
    }

    #[test]
    fn authored_picky_eating_and_presentation_cards_reuse_overlapping_phrases() {
        let eating = lookup("she wont eat").expect("picky eating card");
        assert_eq!(eating.id, "picky-eater");
        assert!(eating.reply.starts_with("Keep it plain and separate"));
        assert!(
            lookup("my 5 month old won't eat anything").is_none_or(|card| card.id != "picky-eater")
        );
        assert!(
            lookup("my son won't eat onions, what can i cook?")
                .is_none_or(|card| card.id != "picky-eater")
        );
        let talk = lookup("How do I give a talk at work?").expect("presentation card");
        assert_eq!(talk.id, "presentation-nerves");
        assert!(talk.reply.starts_with("Write the first thirty seconds"));
    }

    #[test]
    fn shared_groups_keep_domain_rarity_and_posting_order() {
        const CARD: RawRow = RawRow {
            id: "first",
            domain: "one",
            reply: "Fixture opening.",
            key: &[&["zeptron"], &["repair"]],
            grouped: true,
            avoid: &["avoid this"],
            ask: &[&["how", "what"]],
            slips: &[],
            bias: 0,
            explain: false,
            report: false,
            fault: false,
            outcomes: &[],
            guard: guards::Guard::Pass,
            callback: Callback::Fixed,
            shape: &[],
            lead: leading::Lead::None,
        };
        static ROWS: &[RawRow] = &[
            CARD,
            RawRow {
                id: "second",
                ..CARD
            },
            RawRow {
                domain: "two",
                ..CARD
            },
        ];
        let database = Database::new(ROWS.iter());
        let rarity = |row| {
            database
                .entry(row)
                .key
                .first()
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .unique_in_domain
        };
        assert_eq!([rarity(0), rarity(1), rarity(2)], [0, 0, 1]);
        assert_eq!(database.entries[0].ask, database.entries[2].ask);
        assert_eq!(database.entries[0].avoid, database.entries[2].avoid);
        let indexed = |word| vec![database.indexed_forms(word)];
        assert!(
            database
                .candidates(&indexed("zeptron"), CandidateKeys::Scoring)
                .iter()
                .eq([0, 1, 2])
        );
        assert!(
            database
                .candidates(&indexed("repair"), CandidateKeys::All)
                .iter()
                .eq([0, 1, 2])
        );
        assert!(
            database
                .candidates(&indexed("repair"), CandidateKeys::Scoring)
                .iter()
                .next()
                .is_none()
        );
    }

    #[test]
    fn scoring_skips_guards_for_unmatched_cards_and_losing_ties() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static GUARD_CALLS: AtomicUsize = AtomicUsize::new(0);
        fn count_guard(_: &Reading<'_>) -> bool {
            GUARD_CALLS.fetch_add(1, Ordering::Relaxed);
            true
        }

        const CARD: RawRow = RawRow {
            id: "first",
            domain: "test",
            reply: "First reply.",
            key: &[&["zeptron"], &["repair"]],
            grouped: true,
            avoid: &[],
            ask: &[],
            slips: &[],
            bias: 0,
            explain: false,
            report: false,
            fault: false,
            outcomes: &[],
            guard: guards::Guard::Pass,
            callback: Callback::Fixed,
            shape: &[],
            lead: leading::Lead::None,
        };
        static ROWS: &[RawRow] = &[
            CARD,
            RawRow {
                id: "tied",
                guard: guards::Guard::Text(count_guard),
                ..CARD
            },
            RawRow {
                id: "unmatched",
                key: &[&["zeptron"], &["operate"]],
                guard: guards::Guard::Text(count_guard),
                ..CARD
            },
        ];
        let database = Database::new(ROWS.iter());
        let reading = Reading::new("How do I repair a zeptron?");
        let words = tokens(reading.raw);
        let forms = words
            .iter()
            .map(|word| database.indexed_forms(word))
            .collect::<Vec<_>>();
        let candidates = database.candidates(&forms, CandidateKeys::Scoring);
        let chosen = |excluded_id| {
            let mut context = guards::Context::new(&reading, &[]);
            context.excluded_id = excluded_id;
            database
                .best_scored(&candidates, &forms, &words, false, &context, |_| true)
                .map(|(entry, _)| entry.row.id)
        };
        assert_eq!(chosen(None), Some("first"));
        assert_eq!(GUARD_CALLS.load(Ordering::Relaxed), 0);

        assert_eq!(chosen(Some("first")), Some("tied"));
        assert_eq!(GUARD_CALLS.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn typo_rescue_leaves_two_plausible_word_families_unanswered() {
        static TEST_ROWS: &[RawRow] = &[RawRow {
            id: "testing",
            domain: "tests",
            reply: "Fixture reply",
            key: &[&["qaptel", "qartel", "zeptron"], &["repair"]],
            grouped: true,
            avoid: &[],
            ask: &[],
            slips: &[],
            bias: 0,
            explain: false,
            report: false,
            fault: false,
            outcomes: &[],
            guard: guards::Guard::Pass,
            callback: Callback::Fixed,
            shape: &[],
            lead: leading::Lead::None,
        }];
        let database = Database::new(TEST_ROWS.iter());
        let repair = vec![database.indexed_forms("repair")];
        assert_eq!(
            database
                .candidates(&repair, CandidateKeys::All)
                .iter()
                .next(),
            Some(0)
        );
        assert_eq!(
            database
                .candidates(&repair, CandidateKeys::Scoring)
                .iter()
                .next(),
            None
        );
        assert!(
            database
                .find(&Reading::new("How do I repair a qaptel?"), None, false)
                .is_some()
        );
        assert!(
            database
                .find(&Reading::new("How do I repair a qaktel?"), None, false)
                .is_none()
        );
        assert!(
            database
                .find(&Reading::new("How do I repair a zeprton?"), None, false)
                .is_some()
        );
    }

    #[test]
    fn a_shape_is_a_second_signal_for_the_row_that_names_its_subject() {
        for (text, expected) in [
            ("my phone is on 6%", true),
            ("my phone is on 80%", false),
            ("it's on 6%", false),
        ] {
            let matches_battery =
                lookup(text).is_some_and(|answer| answer.id == "phone-nearly-flat");
            assert_eq!(matches_battery, expected, "{text}");
        }
    }

    #[test]
    fn a_statement_can_name_the_fault_and_the_thing_that_has_it() {
        let phone = lookup("i dropped my phone in the sink").expect("a soaked phone card");
        assert_eq!(phone.id, "phone-water");
        assert!(!phone.outcomes.is_empty());

        let oven = lookup("my oven cooks unevenly").expect("an uneven oven card");
        assert_eq!(oven.id, "oven-bakes-unevenly");
        assert!(oven.fault);
    }

    #[test]
    fn recipe_callbacks_answer_a_dish_by_itself_and_decline_diet_requests() {
        let recipe = lookup("lasagne").expect("a dish named by itself asks for its method");
        assert_eq!(recipe.id, "recipe-lasagne");
        assert!(recipe.reply.contains("Layer"));

        assert!(
            !lookup("how do I make vegan lasagne?").is_some_and(|answer| answer.id == recipe.id)
        );
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/static_table_selection.rs"]
mod static_table_selection;

#[cfg(test)]
#[path = "../../../tests/fixtures/word_collections.rs"]
mod word_collection_fixtures;

#[cfg(test)]
mod word_collection_tests {
    use super::*;

    #[test]
    fn lowered_word_collections_match_pybot() {
        for &word_collection_fixtures::Case(domain, id, line, expected_guard, expected) in
            word_collection_fixtures::CASES
        {
            let row = rows()
                .find(|row| row.id == id && row.domain == domain)
                .unwrap();
            let reading = Reading::new(line);
            let context = guards::Context::new(&reading, &[]);
            assert_eq!(
                row.guard.passes(&context, id, &[]),
                expected_guard,
                "{domain}/{id}: {line}"
            );
            assert_eq!(
                row.answer(&context)
                    .as_deref()
                    .filter(|reply| !reply.is_empty()),
                expected,
                "{domain}/{id}: {line}"
            );
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/cooking_edges.rs"]
mod cooking_edge_fixtures;

#[cfg(test)]
#[test]
fn cooking_fault_and_quantity_callbacks_match_pybot() {
    use crate::bot::Exchange;
    let mut mismatches = Vec::new();
    for &cooking_edge_fixtures::Case(id, text, earlier, expected) in cooking_edge_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "cooking" && row.id == id)
            .unwrap();
        let earlier: Vec<_> = earlier
            .iter()
            .map(|(user, bot)| Exchange {
                user: (*user).to_owned(),
                bot: (*bot).to_owned(),
                module: "answers",
            })
            .collect();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &earlier);
        let actual = row.answer(&context);
        if actual.as_deref() != expected {
            mismatches.push(format!(
                "{id}: {text}\nsource {expected:?}\nnative {actual:?}"
            ));
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} mismatches\n{}",
        mismatches.len(),
        mismatches.join("\n\n")
    );
}

#[cfg(test)]
#[path = "../../../tests/fixtures/garden_more.rs"]
mod garden_more_fixtures;

#[cfg(test)]
#[test]
fn garden_callbacks_follow_the_authored_branch_order() {
    for &garden_more_fixtures::Case(id, text, expected) in garden_more_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "garden" && row.id == id)
            .unwrap();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        assert_eq!(row.answer(&context).as_deref(), expected, "{id}: {text}");
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/garden_guards.rs"]
mod garden_guard_fixtures;

#[cfg(test)]
#[test]
fn garden_guards_and_tree_care_match_the_source() {
    for &(id, text, expected_guard, expected_reply) in garden_guard_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "garden" && row.id == id)
            .unwrap();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        assert_eq!(
            row.guard.passes(&context, id, &[]),
            expected_guard,
            "{id}: {text}"
        );
        if let Some(expected) = expected_reply {
            assert_eq!(
                row.answer(&context).as_deref(),
                Some(expected),
                "{id}: {text}"
            );
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/timetable.rs"]
mod timetable_fixtures;

#[cfg(test)]
#[test]
fn reading_a_timetable_uses_the_original_repeat_aware_pool() {
    let row = rows()
        .find(|row| row.domain == "travel" && row.id == "read-a-timetable")
        .unwrap();
    let reading = Reading::new("how do i read a bus timetable");
    for &(earlier, expected) in timetable_fixtures::CASES {
        let memory = Memory {
            turns: earlier
                .iter()
                .map(|reply| crate::bot::Exchange {
                    user: "something i said".to_owned(),
                    bot: (*reply).to_owned(),
                    module: "answers",
                })
                .collect(),
            ..Memory::default()
        };
        let mut context = guards::Context::new(&reading, &memory.turns);
        context.memory = Some(&memory);
        assert_eq!(
            row.answer(&context).as_deref(),
            Some(expected),
            "{earlier:?}"
        );
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/landscape_materials.rs"]
mod landscape_fixtures;

#[cfg(test)]
#[test]
fn material_quantities_reuse_the_source_geometry_and_prior_three_turns() {
    let mut faults = Vec::new();
    for &landscape_fixtures::Case(id, text, earlier, guard, reply) in landscape_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "materials" && row.id == id)
            .unwrap();
        let turns: Vec<_> = earlier
            .iter()
            .map(|user| crate::bot::Exchange {
                user: (*user).to_owned(),
                bot: "Got it.".to_owned(),
                module: "answers",
            })
            .collect();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &turns);
        if row.guard.passes(&context, id, row.key[0]) != guard {
            faults.push(format!("{id}: wrong guard for {text} after {earlier:?}"));
        }
        let actual = row.answer(&context);
        if actual.as_deref() != Some(reply) {
            faults.push(format!(
                "{id}: {text} after {earlier:?}\nexpected {reply}\nactual {actual:?}"
            ));
        }
    }
    assert!(
        faults.is_empty(),
        "{} mismatches:\n{}",
        faults.len(),
        faults.join("\n\n")
    );
}

#[cfg(test)]
#[path = "../../../tests/fixtures/garden_faults.rs"]
mod garden_fault_fixtures;

#[cfg(test)]
#[test]
fn garden_faults_use_the_named_crop_and_the_last_three_user_lines() {
    for &garden_fault_fixtures::Case(id, text, previous, guard, expected) in
        garden_fault_fixtures::CASES
    {
        let row = rows()
            .find(|row| row.domain == "garden" && row.id == id)
            .unwrap();
        let turns: Vec<_> = previous
            .iter()
            .map(|turn| crate::bot::Exchange {
                user: (*turn).to_owned(),
                bot: "Got it.".to_owned(),
                module: "answers",
            })
            .collect();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &turns);
        assert_eq!(
            row.guard.passes(&context, id, row.key[0]),
            guard,
            "{id}: {text}"
        );
        if id != "split-a-log" {
            assert_eq!(
                row.answer(&context).as_deref(),
                expected,
                "{id}: {text} ({previous:?})"
            );
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/school_science.rs"]
mod school_science_fixtures;

#[cfg(test)]
#[test]
fn science_cards_follow_the_authored_subject_and_answer_order() {
    for &(id, text, passes, reply) in school_science_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "science" && row.id == id)
            .unwrap();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        assert_eq!(
            row.guard.passes(&context, id, row.key[0]),
            passes,
            "{id}: {text}"
        );
        if id != "what-water-is" {
            assert_eq!(row.answer(&context).as_deref(), reply, "{id}: {text}");
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/dish_texture.rs"]
mod dish_texture_fixtures;

#[cfg(test)]
#[test]
fn dish_texture_distinguishes_named_food_and_preserves_abstention() {
    for &(id, text, expected) in dish_texture_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "cooking" && row.id == id)
            .unwrap();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        let answer = row.answer(&context);
        assert_eq!(
            answer.as_deref().filter(|reply| !reply.is_empty()),
            expected,
            "{id}: {text}"
        );
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/science_counts.rs"]
mod science_counts_fixtures;

#[cfg(test)]
#[test]
fn science_counts_match_decimal_rounding_and_ordered_names() {
    for &(id, text, expected) in science_counts_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "science" && row.id == id)
            .unwrap();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        assert_eq!(row.answer(&context).as_deref(), expected, "{id}: {text}");
    }
}

#[cfg(test)]
#[test]
fn discoveries_and_repeated_count_keys_reach_the_authored_cards() {
    for (text, id) in [
        ("who discovered gravity", "who-discovered"),
        ("who worked out the structure of dna", "who-discovered"),
        ("walked 12000 steps today", "steps-in-a-distance"),
        ("how far is 10000 steps", "steps-in-a-distance"),
        ("how far is 10,000 steps", "steps-in-a-distance"),
        ("how fast do i walk 10000 steps", "walking-pace"),
        ("how fast should i walk a mile", "walking-pace"),
    ] {
        assert_eq!(lookup(text).map(|card| card.id), Some(id), "{text}");
    }
    for text in [
        "who discovered napoleon",
        "who discovered that i was pregnant",
    ] {
        assert_ne!(
            lookup(text).map(|card| card.id),
            Some("who-discovered"),
            "{text}"
        );
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/cooking_cards.rs"]
mod cooking_cards_fixtures;

#[cfg(test)]
#[test]
fn cooking_cards_preserve_named_terms_recipe_frames_and_vegetable_context() {
    for &(id, text, earlier, expected) in cooking_cards_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "cooking" && row.id == id)
            .unwrap();
        let earlier: Vec<_> = earlier
            .iter()
            .map(|(user, bot)| crate::bot::Exchange {
                user: (*user).to_owned(),
                bot: (*bot).to_owned(),
                module: "answers",
            })
            .collect();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &earlier);
        assert_eq!(row.answer(&context).as_deref(), expected, "{id}: {text}");
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/recipe_reading.rs"]
mod recipe_reading_fixtures;

#[cfg(test)]
#[test]
fn recipe_reading_preserves_instruction_priority_and_cup_abstention() {
    for &(id, text, passes, expected) in recipe_reading_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "cooking_gaps" && row.id == id)
            .unwrap();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        assert_eq!(
            row.guard.passes(&context, id, row.key[0]),
            passes,
            "{id}: {text}"
        );
        assert_eq!(row.answer(&context).as_deref(), expected, "{id}: {text}");
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/history_cards.rs"]
mod history_cards_fixtures;

#[cfg(test)]
#[test]
fn history_tables_follow_authored_names_and_recent_exchange() {
    for &(id, text, earlier, passes, expected) in history_cards_fixtures::CASES {
        let row = rows()
            .find(|row| row.domain == "history" && row.id == id)
            .unwrap();
        let earlier: Vec<_> = earlier
            .iter()
            .map(|(user, bot)| crate::bot::Exchange {
                user: (*user).to_owned(),
                bot: (*bot).to_owned(),
                module: "answers",
            })
            .collect();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &earlier);
        assert_eq!(
            row.guard.passes(&context, id, row.key[0]),
            passes,
            "{id}: {text}"
        );
        assert_eq!(row.answer(&context).as_deref(), expected, "{id}: {text}");
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/nature_cards.rs"]
mod nature_cards_fixtures;

#[cfg(test)]
#[test]
fn nature_cards_preserve_season_produce_and_previous_garden_details() {
    for &(id, text, earlier, passes, expected) in nature_cards_fixtures::CARDS {
        let row = rows()
            .find(|row| row.domain == "nature" && row.id == id)
            .unwrap();
        let earlier: Vec<_> = earlier
            .iter()
            .map(|(user, bot)| crate::bot::Exchange {
                user: (*user).to_owned(),
                bot: (*bot).to_owned(),
                module: "answers",
            })
            .collect();
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &earlier);
        assert_eq!(
            row.guard.passes(&context, id, row.key[0]),
            passes,
            "{id}: {text}"
        );
        assert_eq!(row.answer(&context).as_deref(), expected, "{id}: {text}");
    }
}

#[cfg(test)]
#[test]
fn full_moon_uses_the_authored_civil_time_and_day_counts() {
    for &(date, reply) in nature_cards_fixtures::MOON {
        assert_eq!(
            nature_cards::full_moon_at(date.parse().expect("fixture civil timestamp")).as_deref(),
            Some(reply),
            "{date}"
        );
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/leisure.rs"]
mod leisure_fixtures;

#[cfg(test)]
#[test]
fn the_cheap_day_guard_keeps_the_free_block_as_the_subject() {
    let row = rows()
        .find(|row| row.domain == "plans" && row.id == "cheap-things-to-do")
        .unwrap();
    let mut failures = Vec::new();
    for &(text, expected) in leisure_fixtures::CASES {
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        let actual = row.guard.passes(&context, row.id, row.key[0]);
        if actual != expected {
            failures.push(format!("{text:?}: actual={actual} expected={expected}, subject={:?} entities={:?} topic={:?}", reading.context_fields.subject, reading.entities, reading.own_topic));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[cfg(test)]
#[path = "../../../tests/fixtures/text_lowering.rs"]
mod text_lowering_fixtures;

#[cfg(test)]
#[test]
fn ordered_text_fields_keep_the_pinned_guards_and_replies() {
    use text_lowering_fixtures::Case;

    for &Case(id, text, guard, reply) in text_lowering_fixtures::CASES {
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        let row = rows().find(|row| row.id == id).expect(id);
        assert_eq!(
            row.guard.passes(&context, id, row.key[0]),
            guard,
            "{id}: {text:?}"
        );
        assert_eq!(row.answer(&context).as_deref(), reply, "{id}: {text:?}");
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/leisure_plan.rs"]
mod leisure_plan_fixtures;

#[cfg(test)]
#[test]
fn free_time_keeps_the_authored_subject_fences_and_plan_order() {
    let row = rows()
        .find(|row| row.domain == "plans" && row.id == "free-time-plan")
        .unwrap();
    let mut failures = Vec::new();
    for &(text, expected_guard, expected_reply) in leisure_plan_fixtures::CASES {
        let reading = Reading::new(text);
        let context = guards::Context::new(&reading, &[]);
        let guard = row.guard.passes(&context, row.id, row.key[0]);
        let reply = row.answer(&context);
        if (guard, reply.as_deref()) != (expected_guard, expected_reply) {
            failures.push(format!(
                "{text:?}: guard {guard} expected {expected_guard}, reply {:?} expected {:?}, clean {:?}, subject {:?}, topic {:?}",
                reply.as_deref().map(|part| part.chars().take(80).collect::<String>()),
                expected_reply.map(|part| part.chars().take(80).collect::<String>()),
                reading.line.clean,
                reading.context_fields.subject,
                reading.own_topic,
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
