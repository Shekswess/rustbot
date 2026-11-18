//! Turns follow the upstream observer → proposer → arbiter → history contract.
mod compose;
pub(crate) mod knowledge;
mod memory;
mod modules;
mod transcript;
pub(crate) mod understanding;

use mt19937::MT19937;
use serde::Serialize;
use std::cell::RefCell;
use transcript::Transcript;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Candidate {
    pub module: &'static str,
    pub priority: f64,
    pub reply: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Turn {
    pub reply: String,
    pub module: Option<&'static str>,
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize)]
pub struct ModuleInfo {
    pub name: &'static str,
    pub priority: f64,
    pub hooks: &'static [&'static str],
}

#[derive(Debug, Serialize)]
pub struct Inventory {
    pub modules: Vec<ModuleInfo>,
    pub domains: Vec<String>,
    pub knowledge_entries: usize,
}

pub(crate) use understanding::Reading;

#[derive(Debug)]
pub(crate) struct Exchange {
    pub user: String,
    pub bot: String,
    pub module: &'static str,
}

#[derive(Clone, Copy)]
struct AnswerAnchor {
    turn: usize,
    card: Option<(&'static str, &'static str)>,
}

#[derive(Default)]
pub(crate) struct Memory {
    urgent_medical: Option<modules::safety::Urgency>,
    frequency_reply: bool,
    row: Option<modules::chat::Row>,
    loss: Option<modules::grief::Loss>,
    family_counts: memory::kidcount::Facts,
    group_ages: memory::groupages::Facts,
    household: memory::household::Facts,
    steps: Option<memory::plan::Cursor>,
    focused_person: Option<modules::chat_someone::PersonFocus>,
    ack_topics: memory::ack::Topics,
    entries: memory::will_it_work::EntryAnswers,
    last_answer_anchor: Option<AnswerAnchor>,
    last_knowledge: Option<(&'static str, &'static str)>,
    asleep_said: Option<String>,
    personal: memory::Personal,
    birth_facts: memory::a_year_of_birth::Facts,
    possessions: memory::possessions::Facts,
    decided: memory::decided::Decision,
    occupations: memory::occupations::Work,
    stated_plans: memory::stated_plans::Facts,
    trip_date_reply: Option<String>,
    doneness_topics: memory::done_topic::Topics,
    active_outcomes: knowledge::outcomes::Outcomes,
    meal_plan: Option<knowledge::meal_shortlist::MealPlan>,
    pantry: knowledge::lookup::PantryIntake,
    turns: Transcript,
    name: Option<String>,
    previous_name: Option<String>,
    denied_name: Option<(String, String)>,
    rng: RefCell<Random>,
    introduced_now: bool,
    intent_wants: Option<&'static str>,
    prior_readings: understanding::context::History,
}

struct Random(MT19937);
impl Default for Random {
    fn default() -> Self {
        let mut bytes = [0u8; 32];
        if getrandom::fill(&mut bytes).is_err() {
            return Self(MT19937::new_with_slice_seed(&[0]));
        }
        let words: [u32; 8] = std::array::from_fn(|i| {
            u32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().expect("four bytes"))
        });
        Self(MT19937::new_with_slice_seed(&words))
    }
}

fn seed_words(seed: &str) -> Result<Vec<u32>, String> {
    let digits = seed.strip_prefix(['+', '-']).unwrap_or(seed);
    let mut words = vec![0u32];
    for part in digits.split('_') {
        if part.is_empty() || !part.bytes().all(|c| c.is_ascii_digit()) {
            return Err("the seed must be a decimal integer".into());
        }
        for digit in part.bytes() {
            let mut carry = (digit - b'0') as u64;
            for word in &mut words {
                let n = (*word as u64) * 10 + carry;
                *word = n as u32;
                carry = n >> 32;
            }
            if carry != 0 {
                words.push(carry as u32);
            }
        }
    }
    Ok(words)
}

type Respond = fn(&Reading<'_>, &Memory) -> Option<String>;

enum Responder {
    Text(Respond),
    Answers,
    LeadIn,
}

struct Hook {
    name: &'static str,
    priority: f64,
    respond: Responder,
}

const HANDLERS: &[Hook] = &[
    Hook {
        name: "a_count_of_packets",
        priority: 9000.0,
        respond: Responder::Text(modules::a_count_of_packets::respond),
    },
    Hook {
        name: "safety",
        priority: 9000.0,
        respond: Responder::Text(modules::safety::respond),
    },
    Hook {
        name: "the_month_they_have",
        priority: 8000.0,
        respond: Responder::Text(modules::the_month_they_have::respond),
    },
    Hook {
        name: "since_it_began",
        priority: 7500.0,
        respond: Responder::Text(modules::since_it_began::respond),
    },
    Hook {
        name: "keeping",
        priority: 7400.0,
        respond: Responder::Text(modules::keeping::respond),
    },
    Hook {
        name: "a_question_about_somebody",
        priority: 7300.0,
        respond: Responder::Text(modules::a_question_about_somebody::respond),
    },
    Hook {
        name: "an_age_ahead",
        priority: 7150.0,
        respond: Responder::Text(modules::an_age_ahead::respond),
    },
    Hook {
        name: "steps",
        priority: 7100.0,
        respond: Responder::Text(modules::steps::respond),
    },
    Hook {
        name: "what_they_said",
        priority: 7060.0,
        respond: Responder::Text(modules::what_they_said::respond),
    },
    Hook {
        name: "a_fact_asked_again",
        priority: 7050.0,
        respond: Responder::Text(modules::a_fact_asked_again::respond),
    },
    Hook {
        name: "a_return_brings_back_the_advice",
        priority: 7050.0,
        respond: Responder::Text(memory::return_advice::respond),
    },
    Hook {
        name: "group_ages",
        priority: 7050.0,
        respond: Responder::Text(modules::group_ages::respond),
    },
    Hook {
        name: "household",
        priority: 7050.0,
        respond: Responder::Text(modules::household::respond),
    },
    Hook {
        name: "kid_tally",
        priority: 7050.0,
        respond: Responder::Text(modules::kid_tally::respond),
    },
    Hook {
        name: "said_fact_back",
        priority: 7050.0,
        respond: Responder::Text(modules::said_fact_back::respond),
    },
    Hook {
        name: "live_times",
        priority: 7001.0,
        respond: Responder::Text(modules::live_times::respond),
    },
    Hook {
        name: "a_count_said",
        priority: 7000.0,
        respond: Responder::Text(modules::a_count_said::respond),
    },
    Hook {
        name: "favourites",
        priority: 7000.0,
        respond: Responder::Text(memory::favourite),
    },
    Hook {
        name: "memory_store",
        priority: 7000.0,
        respond: Responder::Text(memory::respond),
    },
    Hook {
        name: "recap",
        priority: 7000.0,
        respond: Responder::Text(modules::recap::respond),
    },
    Hook {
        name: "attribute_said_again",
        priority: 6900.0,
        respond: Responder::Text(memory::attribute_said_again::respond),
    },
    Hook {
        name: "thing_i_did_with",
        priority: 6900.0,
        respond: Responder::Text(modules::thing_i_did_with::respond),
    },
    Hook {
        name: "name_taken_back",
        priority: 6600.0,
        respond: Responder::Text(modules::name_taken_back::respond),
    },
    Hook {
        name: "dreams",
        priority: 3000.0,
        respond: Responder::Text(modules::dreams::respond),
    },
    Hook {
        name: "grief",
        priority: 2100.0,
        respond: Responder::Text(modules::grief::respond),
    },
    Hook {
        name: "loss",
        priority: 2000.0,
        respond: Responder::Text(compose::loss::respond),
    },
    Hook {
        name: "who_to_see",
        priority: 1500.0,
        respond: Responder::Text(modules::who_to_see::respond),
    },
    Hook {
        name: "pace",
        priority: 1300.0,
        respond: Responder::Text(modules::pace::respond),
    },
    Hook {
        name: "a_cake_between_its_layers",
        priority: 1200.0,
        respond: Responder::Text(modules::a_cake_between_its_layers::respond),
    },
    Hook {
        name: "words",
        priority: 1200.0,
        respond: Responder::Text(modules::words::respond),
    },
    Hook {
        name: "my_work",
        priority: 1150.0,
        respond: Responder::Text(modules::my_work::respond),
    },
    Hook {
        name: "years_in",
        priority: 1150.0,
        respond: Responder::Text(modules::years_in::respond),
    },
    Hook {
        name: "aboutachild",
        priority: 1100.0,
        respond: Responder::Text(modules::aboutachild::respond),
    },
    Hook {
        name: "familyask",
        priority: 1100.0,
        respond: Responder::Text(modules::familyask::respond),
    },
    Hook {
        name: "night",
        priority: 1100.0,
        respond: Responder::Text(modules::night::respond),
    },
    Hook {
        name: "saving_followup",
        priority: 1100.0,
        respond: Responder::Text(modules::saving_followup::respond),
    },
    Hook {
        name: "how_long_a_stay",
        priority: 1050.0,
        respond: Responder::Text(modules::how_long_a_stay::respond),
    },
    Hook {
        name: "no_advice",
        priority: 1050.0,
        respond: Responder::Text(modules::no_advice::respond),
    },
    Hook {
        name: "a_figure_beside_the_state",
        priority: 1000.0,
        respond: Responder::Text(modules::a_figure_beside_the_state::respond),
    },
    Hook {
        name: "a_measure_asked_again",
        priority: 1000.0,
        respond: Responder::Text(modules::a_measure_asked_again::respond),
    },
    Hook {
        name: "a_shock_at_a_bill",
        priority: 1000.0,
        respond: Responder::Text(modules::a_shock_at_a_bill::respond),
    },
    Hook {
        name: "a_will_it_work",
        priority: 1000.0,
        respond: Responder::Text(modules::a_will_it_work::respond),
    },
    Hook {
        name: "figure_asked_again",
        priority: 1000.0,
        respond: Responder::Text(memory::stand_figure::respond),
    },
    Hook {
        name: "taking_asked_again",
        priority: 1000.0,
        respond: Responder::Text(memory::taking_again::respond),
    },
    Hook {
        name: "a_limit_over_the_answer",
        priority: 500.0,
        respond: Responder::Text(modules::a_limit_over_the_answer::respond),
    },
    Hook {
        name: "already",
        priority: 400.0,
        respond: Responder::Text(memory::already::respond),
    },
    Hook {
        name: "doubt",
        priority: 400.0,
        respond: Responder::Text(modules::doubt::respond),
    },
    Hook {
        name: "colour_on_the_body",
        priority: 350.0,
        respond: Responder::Text(modules::colour_on_the_body::respond),
    },
    Hook {
        name: "said_back",
        priority: 350.0,
        respond: Responder::Text(memory::lineback::respond),
    },
    Hook {
        name: "after_a_lead_in",
        priority: 320.0,
        respond: Responder::LeadIn,
    },
    Hook {
        name: "chat",
        priority: 300.0,
        respond: Responder::Text(modules::chat::respond),
    },
    Hook {
        name: "callback_about",
        priority: 260.0,
        respond: Responder::Text(modules::callback_about::respond),
    },
    Hook {
        name: "their_pastime",
        priority: 300.0,
        respond: Responder::Text(modules::their_pastime::respond),
    },
    Hook {
        name: "theirperson",
        priority: 250.0,
        respond: Responder::Text(modules::theirperson::respond),
    },
    Hook {
        name: "their_life",
        priority: 100.0,
        respond: Responder::Text(modules::their_life::respond),
    },
    Hook {
        name: "answers",
        priority: 0.0,
        respond: Responder::Answers,
    },
    Hook {
        name: "ruled_out",
        priority: -450.0,
        respond: Responder::Text(compose::ruledout::respond),
    },
    Hook {
        name: "a_debt_said_flat",
        priority: -500.0,
        respond: Responder::Text(modules::a_debt_said_flat::respond),
    },
    Hook {
        name: "invitation",
        priority: -500.0,
        respond: Responder::Text(modules::invitation::respond),
    },
    Hook {
        name: "judge_ask",
        priority: -500.0,
        respond: Responder::Text(modules::judge_ask::respond),
    },
    Hook {
        name: "record_due",
        priority: -500.0,
        respond: Responder::Text(modules::record_due::respond),
    },
    Hook {
        name: "talkto",
        priority: -500.0,
        respond: Responder::Text(modules::talkto::respond),
    },
    Hook {
        name: "a_letter_about_money",
        priority: -600.0,
        respond: Responder::Text(modules::a_letter_about_money::respond),
    },
    Hook {
        name: "confusing",
        priority: -900.0,
        respond: Responder::Text(modules::confusing::respond),
    },
    Hook {
        name: "urgent",
        priority: -900.0,
        respond: Responder::Text(modules::urgent::respond),
    },
    Hook {
        name: "nudge",
        priority: -950.0,
        respond: Responder::Text(compose::still::respond),
    },
    Hook {
        name: "their_taste",
        priority: -960.0,
        respond: Responder::Text(modules::their_taste::respond),
    },
    Hook {
        name: "hardpast",
        priority: -980.0,
        respond: Responder::Text(modules::hardpast::respond),
    },
    Hook {
        name: "receiving_end",
        priority: -980.0,
        respond: Responder::Text(modules::receiving_end::respond),
    },
    Hook {
        name: "ward",
        priority: -985.0,
        respond: Responder::Text(compose::ward::respond),
    },
    Hook {
        name: "near_me",
        priority: -995.0,
        respond: Responder::Text(compose::nearby_one::respond),
    },
    Hook {
        name: "a_first_time",
        priority: -996.0,
        respond: Responder::Text(compose::a_first_time::respond),
    },
    Hook {
        name: "a_figure_of_a_dish",
        priority: -997.0,
        respond: Responder::Text(modules::a_figure_of_a_dish::respond),
    },
    Hook {
        name: "chasing_a_body",
        priority: -997.0,
        respond: Responder::Text(modules::chasing_a_body::respond),
    },
    Hook {
        name: "detail_question",
        priority: -997.0,
        respond: Responder::Text(modules::detail_question::respond),
    },
    Hook {
        name: "the_trouble_named",
        priority: -997.0,
        respond: Responder::Text(modules::the_trouble_named::respond),
    },
    Hook {
        name: "carers_turn",
        priority: -998.0,
        respond: Responder::Text(modules::carers_turn::respond),
    },
    Hook {
        name: "live_source",
        priority: -998.0,
        respond: Responder::Text(modules::live_source::respond),
    },
    Hook {
        name: "onsubject",
        priority: -998.0,
        respond: Responder::Text(modules::onsubject::respond),
    },
    Hook {
        name: "what_decides_it",
        priority: -998.0,
        respond: Responder::Text(compose::what_decides_it::respond),
    },
    Hook {
        name: "a_check_with_you",
        priority: -999.0,
        respond: Responder::Text(compose::check_with_you::respond),
    },
    Hook {
        name: "a_day_somebody_else_keeps",
        priority: -999.0,
        respond: Responder::Text(modules::a_day_somebody_else_keeps::respond),
    },
    Hook {
        name: "a_habit_is_raised",
        priority: -999.0,
        respond: Responder::Text(compose::habit::respond),
    },
    Hook {
        name: "a_job_that_is_not_a_diy_job",
        priority: -999.0,
        respond: Responder::Text(modules::diy_job::respond),
    },
    Hook {
        name: "a_number_the_bot_knows",
        priority: -999.0,
        respond: Responder::Text(modules::a_number_the_bot_knows::respond),
    },
    Hook {
        name: "a_plan_the_thread_holds",
        priority: -999.0,
        respond: Responder::Text(modules::a_plan_the_thread_holds::respond),
    },
    Hook {
        name: "a_thing_being_weighed",
        priority: -999.0,
        respond: Responder::Text(compose::a_thing_being_weighed::respond),
    },
    Hook {
        name: "a_thing_wants_doing",
        priority: -999.0,
        respond: Responder::Text(compose::a_thing_wants_doing::respond),
    },
    Hook {
        name: "bulky",
        priority: -999.0,
        respond: Responder::Text(modules::bulky::respond),
    },
    Hook {
        name: "the_detail_the_thread_adds",
        priority: -999.0,
        respond: Responder::Text(modules::the_detail_the_thread_adds::respond),
    },
    Hook {
        name: "the_kind_one_line_up",
        priority: -999.0,
        respond: Responder::Text(modules::the_kind_one_line_up::respond),
    },
    Hook {
        name: "the_trouble_it_has",
        priority: -999.0,
        respond: Responder::Text(modules::the_trouble_it_has::respond),
    },
    Hook {
        name: "the_wait_about_a_body",
        priority: -999.0,
        respond: Responder::Text(modules::the_wait_about_a_body::respond),
    },
    Hook {
        name: "where_it_answers",
        priority: -999.0,
        respond: Responder::Text(compose::routes::respond),
    },
    Hook {
        name: "first_line_open",
        priority: -999.5,
        respond: Responder::Text(modules::first_line_open),
    },
    Hook {
        name: "the_shape_of_the_answer",
        priority: -999.5,
        respond: Responder::Text(compose::the_shape_without_the_fact::respond),
    },
    Hook {
        name: "greeting",
        priority: -1000.0,
        respond: Responder::Text(modules::greeting::respond),
    },
];

/// One conversation with in-memory history and an immutable set of local rules.
#[derive(Default)]
pub struct Bot {
    memory: Memory,
    seed: Option<Vec<u32>>,
}

impl Bot {
    /// Begin an empty conversation, optionally seeding rules that use shared randomness.
    pub fn new(seed: Option<&str>) -> Result<Self, String> {
        let seed = seed.map(seed_words).transpose()?;
        let mut bot = Self {
            seed,
            ..Self::default()
        };
        bot.apply_seed();
        Ok(bot)
    }

    fn apply_seed(&mut self) {
        if let Some(seed) = &self.seed {
            *self.memory.rng.get_mut() = Random(MT19937::new_with_slice_seed(seed));
        }
    }

    /// Run one turn and append the selected reply to the session.
    pub fn turn(&mut self, text: &str) -> Turn {
        let mut reading = Reading::understand(
            text,
            self.memory.turns.iter().map(|t| t.user.as_str()),
            self.memory.intent_wants,
            &self.memory.prior_readings,
        );
        self.memory.intent_wants = reading.wants;
        modules::said_fact_back::observe(&reading, &mut self.memory);
        modules::a_debt_said_flat::observe(&reading, &mut self.memory);
        modules::a_count_said::observe(&reading, &mut self.memory);
        memory::observe(&reading, &mut self.memory);
        modules::done_ask::observe(&mut reading, &mut self.memory);
        modules::trip_date_said_again::observe(text, &mut self.memory);
        modules::name_taken_back::observe(&reading, &mut self.memory);
        modules::chat_someone::observe(&reading, &mut self.memory);
        modules::a_limit_over_the_answer::observe(&reading, &mut self.memory);
        modules::grief::observe(text, &mut self.memory);
        modules::chat::observe(&reading, &mut self.memory);
        modules::answers::observe(&reading, &mut self.memory);
        let mut proposed_outcomes: knowledge::outcomes::Outcomes = &[];
        let mut proposed_entry = None;
        let mut proposed_card = None;
        let mut proposed_meal = None;
        let mut score_carried_subject = false;
        let withheld = knowledge::withheld::applies(text, Some(&self.memory));
        let forgets_name = memory::asks_to_forget_name(&reading.clean);
        let medical_urgency = modules::safety::has_medical_urgency(&reading, &self.memory)
            && !modules::the_detail_the_thread_adds::heavy::only_a_dream(&reading.clean);
        let mut candidates: Vec<_> = HANDLERS
            .iter()
            .filter(|hook| !withheld || hook.priority >= 9000.0)
            .filter(|hook| !medical_urgency || !matches!(hook.name, "answers" | "chat"))
            .filter(|hook| !forgets_name || hook.name != "answers")
            .filter_map(|hook| {
                let reply = match hook.respond {
                    Responder::Text(respond) => respond(&reading, &self.memory)?,
                    Responder::LeadIn => {
                        modules::after_a_lead_in::respond(&reading, &mut self.memory)?
                    }
                    Responder::Answers => {
                        let (answer, meal) =
                            modules::answers::offer_for_turn(&reading, &self.memory)?;
                        proposed_outcomes = answer.outcomes;
                        proposed_entry = answer.entry;
                        proposed_card = answer.selected_card;
                        proposed_meal = meal;
                        score_carried_subject = answer.score_carried_subject;
                        answer.reply
                    }
                };
                let reply = if matches!(
                    hook.name,
                    "answers" | "safety" | "greeting" | "chat" | "saving_followup"
                ) && modules::safety::selected_medical_urgency(&reply).is_none()
                {
                    memory::frame_reply(&reply, text, &self.memory).unwrap_or(reply)
                } else {
                    reply
                };
                let safety_card = hook.name == "answers"
                    && proposed_card.is_some_and(|(domain, card)| {
                        (self.memory.pantry.current_spoilage()
                            && matches!(domain, "still_ok" | "freshness"))
                            || (domain, card) == ("cooking", "nut-allergy")
                    });
                let blocked_plan = hook.name == "answers"
                    && proposed_meal.as_ref().is_some_and(|plan| {
                        if let Some(diet) = plan.diet {
                            self.memory
                                .pantry
                                .blocks_plant_offer(&self.memory, diet.vegan())
                        } else {
                            self.memory.pantry.blocks_typed_offer(&self.memory)
                        }
                    });
                if matches!(hook.name, "answers" | "chat")
                    && !safety_card
                    && (blocked_plan
                        || self
                            .memory
                            .pantry
                            .blocks_reply(&reading, &reply, &self.memory))
                {
                    if hook.name == "answers" {
                        proposed_meal = None;
                        proposed_entry = None;
                        proposed_card = None;
                        proposed_outcomes = &[];
                        score_carried_subject = false;
                    }
                    return None;
                }
                (!reply.trim().is_empty()).then_some(Candidate {
                    module: hook.name,
                    priority: hook.priority,
                    reply,
                })
            })
            .collect();

        if candidates.iter().any(|candidate| {
            candidate.module == "answers"
                && modules::a_shock_at_a_bill::yields_to_money_answer(&reading, &candidate.reply)
        }) {
            candidates.retain(|candidate| candidate.module != "a_shock_at_a_bill");
        }

        let picked = modules::arbiter::choose(
            &reading,
            &self.memory,
            &candidates,
            score_carried_subject,
            proposed_card,
        );
        let fallback = if withheld {
            knowledge::withheld::REPLY
        } else {
            "Hello!"
        };
        let (reply, module) =
            picked.map_or((fallback, None), |p| (p.reply.as_str(), Some(p.module)));
        if module == Some("household") {
            memory::household::record_selected(&reading, &mut self.memory);
        }
        if module == Some("steps") {
            memory::plan::record_selected(&reading, &mut self.memory);
        }
        if module == Some("chat") {
            memory::record_carried_chat_reply(&reading, reply, &mut self.memory);
            modules::chat::record_selected(&reading, reply, &mut self.memory);
        }
        if module == Some("night") && knowledge::lookup::sleep_aid(text).is_none() {
            self.memory.asleep_said = Some(reply.chars().take(120).collect());
        }
        self.memory.active_outcomes = if module == Some("answers") {
            if let Some(plan) = proposed_meal {
                self.memory.meal_plan = Some(plan);
            }
            if proposed_card.is_some() {
                self.memory.last_knowledge = proposed_card;
            }
            if let Some(entry) = proposed_entry {
                self.memory.entries.remember(entry);
                self.memory.last_answer_anchor = Some(AnswerAnchor {
                    turn: self.memory.turns.len(),
                    card: proposed_card,
                });
            }
            proposed_outcomes
        } else {
            &[]
        };
        self.memory.urgent_medical = (module == Some("safety"))
            .then(|| modules::safety::selected_medical_urgency(reply))
            .flatten();
        self.memory.ack_topics.note(&reading, &self.memory.turns);
        memory::repair::after(text, &mut self.memory);
        modules::chat::after(text, reply, &mut self.memory);
        self.memory.prior_readings.retain(&mut reading);
        self.memory.turns.record(Exchange {
            user: text.to_owned(),
            bot: reply.to_owned(),
            module: module.unwrap_or_default(),
        });
        Turn {
            reply: reply.to_owned(),
            module,
            candidates,
        }
    }

    /// Forget the complete conversation history.
    pub fn reset(&mut self) -> Result<(), String> {
        self.memory = Memory::default();
        self.apply_seed();
        Ok(())
    }

    /// Report the active native hook modules and indexed knowledge domains.
    pub fn inventory(&self) -> Result<Inventory, String> {
        let mut modules: Vec<_> = HANDLERS
            .iter()
            .map(|h| ModuleInfo {
                name: h.name,
                priority: h.priority,
                hooks: if matches!(h.name, "chat" | "memory_store") {
                    &["observe", "respond", "after"]
                } else if matches!(
                    h.name,
                    "a_count_said"
                        | "a_debt_said_flat"
                        | "a_limit_over_the_answer"
                        | "answers"
                        | "favourites"
                        | "grief"
                        | "name_taken_back"
                        | "said_fact_back"
                        | "group_ages"
                        | "household"
                ) {
                    &["observe", "respond"]
                } else {
                    &["respond"]
                },
            })
            .collect();
        modules.push(ModuleInfo {
            name: "intent",
            priority: 9500.0,
            hooks: &["observe"],
        });
        modules.push(ModuleInfo {
            name: "understand",
            priority: 10000.0,
            hooks: &["observe"],
        });
        modules.push(ModuleInfo {
            name: "done_ask",
            priority: 6900.0,
            hooks: &["observe"],
        });
        modules.push(ModuleInfo {
            name: "trip_date_said_again",
            priority: 6900.0,
            hooks: &["observe"],
        });
        modules.push(ModuleInfo {
            name: "arbiter",
            priority: 0.0,
            hooks: &["choose"],
        });
        modules.sort_by(|a, b| {
            b.priority
                .total_cmp(&a.priority)
                .then_with(|| a.name.cmp(b.name))
        });
        Ok(Inventory {
            modules,
            domains: knowledge::lookup::domains(),
            knowledge_entries: knowledge::lookup::entry_count(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_matching_knowledge_card_keeps_the_fallback_proposal_below_it() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("Why do onions make you cry?");
        assert_eq!(turn.module, Some("answers"));
        assert!(
            turn.candidates
                .iter()
                .map(|candidate| candidate.module)
                .eq(["answers", "the_shape_of_the_answer", "greeting",])
        );
    }

    #[test]
    fn immediate_detail_questions_quote_the_selected_knowledge_card() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("How long should I boil an egg?");
        let egg = bot.turn("And if I want the yolk soft?");
        assert_eq!(egg.module, Some("answers"));
        assert_eq!(egg.reply, "6 minutes for a runny yolk, 7 for jammy.");

        bot.turn("Why does my phone only charge when I hold the cable at an angle?");
        let cable = bot.turn("Should I try a different cable?");
        assert_eq!(cable.module, Some("answers"));
        assert_eq!(cable.reply, "Try another cable and plug first.");

        bot.turn("I dropped my phone in the sink.");
        let wet = bot.turn("Should I try a different cable?");
        assert_ne!(wet.reply, cable.reply);
    }

    #[test]
    fn explicit_name_denial_and_replacement() {
        let mut bot = Bot::new(Some("0")).unwrap();
        for (input, module, reply) in [
            (
                "my name is kay",
                "chat",
                "Nice to meet you, Kay. What can I help with?",
            ),
            (
                "im not kay",
                "name_taken_back",
                "My mistake on Kay. What should I call you?",
            ),
            (
                "dave",
                "chat",
                "Hi Dave — good to meet you. What's on your mind?",
            ),
            ("whats my name", "memory_store", "You're Dave."),
        ] {
            let turn = bot.turn(input);
            assert_eq!(
                (turn.module, turn.reply.as_str()),
                (Some(module), reply),
                "{input}"
            );
        }
        assert!(!memory::names::takes_back("im not hungry", "Dave"));
        assert!(memory::names::takes_back(
            "no, don't call me Dave by the way",
            "Dave"
        ));
    }

    #[test]
    fn reset_discards_names_and_history() {
        let mut bot = Bot::default();
        let turn = bot.turn("hi, i'm Sam");
        assert!(turn.reply.contains("Sam"), "{turn:?}");
        let turn = bot.turn("what's my name?");
        assert_eq!(turn.module, Some("memory_store"));
        assert!(turn.reply.contains("Sam"));
        bot.reset().unwrap();
        assert!(!bot.turn("what's my name?").reply.contains("Sam"));
    }

    #[test]
    fn a_fresh_meal_stores_the_selected_candidates_plan() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let prompt = "what should i cook tonight";
        let proposed =
            modules::answers::offer_for_turn(&Reading::new(prompt), &bot.memory).unwrap();
        assert!(proposed.1.is_some());
        assert!(bot.memory.meal_plan.is_none());
        let turn = bot.turn(prompt);
        assert_eq!(turn.module, Some("answers"));
        assert_eq!(
            turn.reply,
            "Here's my pick: pea and mint soup in 15 minutes, or tomato pasta in 20, or a potato curry in 30. Tell me what is in the fridge and I will narrow it down."
        );
        let plan = bot.memory.meal_plan.as_ref().unwrap();
        assert_eq!(plan.turn, 0);
        assert_eq!(
            plan.offered(),
            [
                "a potato curry",
                "tuna pasta",
                "a frittata",
                "vegetable soup"
            ]
        );

        bot.reset().unwrap();
        let turn = bot.turn("what can i eat for dinner if i have a nut allergy");
        assert!(bot.memory.meal_plan.is_none(), "{turn:?}");
    }
}
