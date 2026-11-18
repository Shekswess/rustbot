//! Follow-up on the person and the change these turns are about.

use crate::bot::{
    Memory, Reading,
    compose::{
        self, loss_reader,
        trouble::{advisor, reads_like_symptom, without_sick_figure},
    },
    knowledge::lookup,
    understanding::kinds::Kind,
};
use crate::text::is_space;
use fancy_regex::Regex;
use std::borrow::Cow;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "chat_someone_data.rs"]
mod data;
use data::*;

#[path = "chat_someone_feeling.rs"]
mod feeling;
#[path = "chat_someone_status.rs"]
mod status;
#[path = "chat_someone_travel_limit.rs"]
mod travel_limit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TroubleKind {
    Homeward,
    Hospital,
    Known,
    Care,
    Memory,
    Falls,
    Low,
    Sleep,
    Ill,
}

impl TroubleKind {
    fn label(self) -> &'static str {
        match self {
            Self::Homeward => "homeward",
            Self::Hospital => "hospital",
            Self::Known => "known",
            Self::Care => "care",
            Self::Memory => "memory",
            Self::Falls => "falls",
            Self::Low => "low",
            Self::Sleep => "sleep",
            Self::Ill => "ill",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PersonFocus {
    who: String,
    said: String,
    turn: usize,
    trouble: Option<(TroubleKind, usize)>,
}

fn split(line: &str) -> impl DoubleEndedIterator<Item = &str> + Clone {
    line.split(is_space).filter(|word| !word.is_empty())
}

fn flat(line: &str) -> String {
    split(&line.to_lowercase().replace('’', "'"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn mentions(pattern: &Regex, line: &str) -> bool {
    pattern.is_match(line).unwrap_or(false)
}

fn one_of<T: Copy>(table: &[(&str, T)], label: &str) -> Option<T> {
    table
        .binary_search_by_key(&label, |(key, _)| *key)
        .ok()
        .map(|i| table[i].1)
}

fn person(line: &str, focused: bool) -> Option<String> {
    let lower = flat(line);
    let words = WORD
        .find_iter(&lower)
        .flatten()
        .map(|word| {
            let word = word.as_str();
            word.strip_suffix("'s").unwrap_or(word).replace('\'', "")
        })
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if let Some(who) = WHO_SAID.captures(&words).ok().flatten() {
        return (!(focused
            && mentions(&A_BARE_PRONOUN_SUBJECT, &words)
            && mentions(&A_QUESTION_THEY_ASK, &words)))
        .then(|| who[1].to_owned());
    }
    if focused {
        return None;
    }
    WHO_SAID_THIRD
        .captures(&words)
        .ok()
        .flatten()
        .map(|matched| matched[1].to_owned())
}

fn trouble(line: &str) -> Option<TroubleKind> {
    let clean = flat(line);
    if mentions(&A_JOB_AT, &clean) && !mentions(&SOMETHING_WRONG, &clean) {
        return None;
    }
    PERSON_TROUBLE.iter().find_map(|&(kind, pattern)| {
        if !mentions(pattern, &clean) {
            return None;
        }
        let refused = match kind {
            TroubleKind::Known => !mentions(&CHANGE, &clean),
            TroubleKind::Memory => {
                mentions(&REPEATED_SAID, &clean)
                    && mentions(&A_CHILDS_PERSON, &clean)
                    && !mentions(&A_GROWN_AGE, &clean)
            }
            TroubleKind::Homeward => mentions(&A_WORRY_AFTER, &clean),
            TroubleKind::Falls => [&YOUNG_PERSON, &FALLS_FINE, &FELL_OUT]
                .iter()
                .any(|pattern| mentions(pattern, &clean)),
            _ => false,
        };
        (!refused).then_some(kind)
    })
}

fn statement_for_store(raw: &str) -> Option<Cow<'_, str>> {
    if mentions(&REMIND_QUESTION, raw) {
        return None;
    }
    let text = split(raw).collect::<Vec<_>>().join(" ");
    let Some(asked) = YES_NO_ASK.find(&text).ok().flatten() else {
        return Some(Cow::Borrowed(raw));
    };
    let statement = if let Ok(Some(frame)) = YES_NO_FRAME.find(&text) {
        text[frame.end()..].trim_matches(is_space)
    } else {
        let rest = &text[asked.end()..];
        let telling = TELLING_AFTER_ASK.find(rest).ok().flatten()?;
        rest[telling.start()..].trim_matches(is_space)
    };
    (!statement.is_empty()).then(|| Cow::Owned(statement.to_owned()))
}

/// Record a person's condition before any module chooses a reply.
pub(crate) fn observe(reading: &Reading<'_>, memory: &mut Memory) {
    let turn = memory.turns.len();
    let Some(typed) = statement_for_store(reading.raw) else {
        return;
    };
    let text = if reading.line.change_of_subject && !reading.line.clean.is_empty() {
        reading.line.clean.as_str()
    } else {
        typed.as_ref()
    };
    let focus = &mut memory.focused_person;
    if let Some(who) = person(text, focus.is_some()) {
        let said = text.trim_matches(is_space);
        if focus
            .as_ref()
            .is_none_or(|old| old.who != who || old.said != said)
        {
            *focus = Some(PersonFocus {
                who,
                said: said.to_owned(),
                turn,
                trouble: None,
            });
        }
    } else if let Some(record) = focus {
        record.turn = turn;
    }
    if let Some(record) = focus
        && let Some(kind) = trouble(text)
        && record.trouble.is_none_or(|(prior, _)| prior != kind)
    {
        record.trouble = Some((kind, turn));
    }
}

fn safety_thread(memory: &Memory) -> bool {
    memory
        .turns
        .iter()
        .rev()
        .take(3)
        .find(|turn| turn.module != "greeting" && !INSIDE_A_SAFETY_THREAD.contains(&turn.module))
        .is_some_and(|turn| turn.module == "safety")
}

fn said_wrong(clean: &str) -> bool {
    let owns_deed = [
        &PET_OR_PROFESSIONAL,
        &PHYSICAL_HARM,
        &GRIEF,
        &A_BABY,
        &REGRET_FIRST,
    ]
    .iter()
    .all(|pattern| !mentions(pattern, clean))
        && (REPAIR_LAPSES.iter().any(|pattern| mentions(pattern, clean))
            || !mentions(&NOT_A_DEED, clean)
                && !mentions(&NOBODY_TO_REPAIR, clean)
                && REPAIR_DEEDS.iter().any(|pattern| mentions(pattern, clean)));
    owns_deed
        || mentions(&SAID_WRONG, clean)
            && [
                &SAID_WRONG_NOT,
                &SAID_WRONG_LOST,
                &SAID_WRONG_TO_AN_ANIMAL,
                &SAID_WRONG_TO_A_PROFESSIONAL,
            ]
            .iter()
            .all(|pattern| !mentions(pattern, clean))
}

fn said_unkindly(clean: &str) -> bool {
    !said_wrong(clean)
        && !mentions(&I_SAID, clean)
        && SAID_UNKINDLY.captures_iter(clean).flatten().any(|capture| {
            let start = capture.get(0).unwrap().start();
            let word = capture.name("word").unwrap().start();
            if !mentions(&FEELING_CLAUSE, &clean[..word]) {
                return true;
            }
            let last_boundary = CLAUSE_BOUNDARY
                .find_iter(&clean[..start])
                .flatten()
                .map(|hit| hit.end())
                .last()
                .unwrap_or(0);
            let opening = &clean[last_boundary..start];
            !opening.trim_matches(is_space).is_empty() && !mentions(&FIRST_PERSON_DEED, opening)
        })
}

fn after_repair(memory: &Memory) -> bool {
    memory.turns.last().is_some_and(|turn| {
        let clean = flat(&turn.user);
        !clean.is_empty()
            && (said_wrong(&clean)
                || said_unkindly(&clean)
                || mentions(&AMENDS, &clean)
                    && !mentions(&PET_OR_PROFESSIONAL, &clean)
                    && !mentions(&PHYSICAL_HARM, &clean))
    })
}

fn after_plan(memory: &Memory, clean: &str) -> bool {
    let after_ask = memory
        .turns
        .iter()
        .rev()
        .map(|turn| flat(&turn.user))
        .find(|said| !said.is_empty() && said != clean)
        .is_some_and(|said| mentions(&PLAN_ASK, &said));
    after_ask
        || memory
            .turns
            .iter()
            .rev()
            .take(3)
            .any(|turn| mentions(&PLAN, &turn.user))
}

fn practical_ask(raw: &str) -> bool {
    pattern!(
        OTHER_ASK_WORD,
        r"\A\s*(?:who|who's|whos|whose|where|which)\b"
    );
    let normalized = flat(raw);
    let tight = split(&normalized)
        .map(|term| term.trim_matches(['.', ',', '!', '?', ';', ':', '"', '\'', '(', ')']))
        .filter(|term| FILLERS.binary_search(term).is_err())
        .collect::<Vec<_>>()
        .join(" ");
    let asking = raw.contains('?')
        || mentions(&ASK_FORM, &tight)
        || mentions(&OTHER_ASK_WORD, &tight)
        || [
            "what", "how", "should", "can", "do", "does", "is", "why", "when", "where",
        ]
        .iter()
        .any(|word| raw.starts_with(word));
    let later = mentions(&ASK_WE, &tight) || mentions(&ASK_TAIL, raw);
    let grief = mentions(&GRIEF_CUE, &tight);
    [&FUNERAL, &FUNERALS, &WAKE]
        .iter()
        .any(|pattern| mentions(pattern, &tight))
        && mentions(&A_PART_OF_THE_DAY, &tight)
        && !mentions(&SAY_THE_WORDS, &tight)
        || (asking || later) && mentions(&PET_ARRANGEMENT, &tight) && mentions(&PET, &tight)
        || later && mentions(&PAPER_ASK, &tight) && !grief
        || asking && mentions(&PRACTICAL, &tight) && !grief && !mentions(&ASK_WORDS, &tight)
}

fn bereavement_open(memory: &Memory) -> bool {
    let mut count = memory.turns.len();
    let mut practical = 0;
    while count > 0 && practical < THREAD_TURNS {
        let turn = &memory.turns[count - 1];
        if SAFETY_MODULES.contains(&turn.module) {
            count -= 1;
        } else if practical_ask(&turn.user) {
            count -= 1;
            practical += 1;
        } else {
            break;
        }
    }
    if count == 0 {
        return memory
            .turns
            .iter()
            .any(|turn| loss_reader::reads_as_loss(&turn.user));
    }
    let last = &memory.turns[count - 1];
    LOSS_REPLIES.binary_search(&last.bot.as_str()).is_ok()
        || !last.module.is_empty()
            && !FALLBACK_MODULES.contains(&last.module)
            && !SAFETY_MODULES.contains(&last.module)
            && loss_reader::reads_as_loss(&last.user)
}

fn grieving(memory: &Memory, clean: &str) -> bool {
    if loss_reader::reads_as_loss(clean)
        || bereavement_open(memory)
        || super::the_detail_the_thread_adds::heavy::sudden_loss(clean)
    {
        return true;
    }
    memory
        .turns
        .iter()
        .rev()
        .find(|turn| {
            let said = flat(&turn.user);
            !said.is_empty() && said != clean
        })
        .is_some_and(|turn| {
            loss_reader::reads_as_loss(&turn.user)
                || super::the_detail_the_thread_adds::heavy::sudden_loss(&turn.user)
                || LOSS_REPLIES.binary_search(&turn.bot.as_str()).is_ok()
                || SUDDEN_LOSS.contains(&turn.bot.as_str())
        })
}

fn normal_or_next(record: &PersonFocus, kind: TroubleKind, clean: &str) -> Option<String> {
    let normal = mentions(&SOMEONE_IS_IT_NORMAL, clean);
    let label = kind.label();
    if let Some(next) = one_of(SOMEONE_NEXT, label)
        && (normal || mentions(&SOMEONE_NEXT_STEP, clean))
        && !(kind == TroubleKind::Sleep && !normal && mentions(&YOUNG, &record.said))
    {
        let template = if normal {
            one_of(SOMEONE_NORMAL, label)?
        } else {
            next
        };
        return Some(template.replace("{who}", &record.who));
    }
    mentions(&SOMEONE_AGES_IT, clean).then(|| SOMEONE_NORMAL_ANY.to_owned())
}

fn follow_up(
    record: &PersonFocus,
    kind: TroubleKind,
    first: usize,
    clean: &str,
    now: usize,
) -> Option<String> {
    let count = split(clean).count();
    if count <= 9
        && let Some(reply) = normal_or_next(record, kind, clean)
    {
        return Some(reply);
    }
    if first < now && count <= 12 {
        let same_kind = PERSON_TROUBLE
            .iter()
            .rev()
            .find(|(prior, _)| *prior == kind)
            .is_some_and(|(_, pattern)| mentions(pattern, clean));
        if same_kind || count <= 8 && mentions(&SOMEONE_MORE, clean) {
            return one_of(SOMEONE_AGAIN, kind.label()).map(str::to_owned);
        }
    }
    None
}

fn about_someone(clean: &str, reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if safety_thread(memory) || grieving(memory, clean) {
        return None;
    }
    if after_repair(memory) && !mentions(&SOMEONE_TROUBLE_ANY, clean) {
        return None;
    }
    let record = memory.focused_person.as_ref();
    let count = split(clean).count();
    if record.is_none()
        && mentions(&SOMEONE_AGES_IT, clean)
        && reading.reference.followup
        && let Some(kind) = memory.turns.last().and_then(|turn| trouble(&turn.user))
    {
        return one_of(SOMEONE_NORMAL, kind.label()).map(str::to_owned);
    }
    if let Some(record) = record {
        if let Some((kind, first)) = record.trouble {
            let bare = clean.trim_end_matches(['?', '.', '!']);
            if kind == TroubleKind::Hospital
                && memory
                    .turns
                    .last()
                    .is_some_and(|turn| turn.module == "ward")
                && matches!(
                    bare,
                    "where does she work" | "where does he work" | "where do they work"
                )
                && let Some(pool) = one_of(SOMEONE_REPLY, kind.label())
            {
                return Some(compose::pick(pool, memory, "").replace("{who}", &record.who));
            }
            if let Some(reply) = follow_up(record, kind, first, clean, memory.turns.len()) {
                return Some(reply);
            }
        } else if count <= 9 && mentions(&SOMEONE_AGES_IT, clean) {
            return Some(SOMEONE_NORMAL_ANY.to_owned());
        }
    }
    if (mentions(&LIMIT, clean)
        || !clean.contains('?') && mentions(&A_LIMIT, clean) && !reads_like_symptom(clean))
        && after_plan(memory, clean)
    {
        return None;
    }
    if said_wrong(clean)
        || said_unkindly(clean)
        || !mentions(&MAYBE_SOMEONE, clean)
        || mentions(&LOSS, clean)
        || mentions(&END_OF_ROPE, clean)
    {
        return None;
    }
    let who = record
        .map(|r| r.who.clone())
        .or_else(|| person(clean, false))?;
    if mentions(&FEAR, clean)
        || mentions(&A_DATE_OF_OURS, clean) && !mentions(&A_WORRY_ABOUT_THEM, clean)
    {
        return None;
    }
    if let Some(kind) = trouble(clean) {
        let label = if kind == TroubleKind::Hospital && mentions(&SERIOUS_DIAGNOSIS, clean) {
            "diagnosed"
        } else {
            kind.label()
        };
        if let Some(pool) = one_of(SOMEONE_REPLY, label)
            && !(kind == TroubleKind::Ill && mentions(&SYMPTOM, clean))
            && !(matches!(kind, TroubleKind::Low | TroubleKind::Sleep) && mentions(&YOUNG, clean))
        {
            return Some(compose::pick(pool, memory, "").replace("{who}", &who));
        }
    }
    if reading.kind == Kind::Question
        || !mentions(
            &SOMEONE_TROUBLE_ANY,
            &THING_BEFORE_CANNOT.replace_all(clean, " "),
        )
        || mentions(&FIRST_PERSON, &WHO_SAID.replace_all(clean, " "))
        || lookup::lookup_in_context(reading, memory).is_some()
        || advisor::who_settles(reading).is_some()
    {
        return None;
    }
    Some(compose::pick(SOMEONE_ASK, memory, "").replace("{who}", &who))
}

pub(super) fn status_reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    status::reply(reading.raw, memory)
}

fn token_count(line: &str) -> usize {
    WORD.find_iter(&line.replace('\'', "")).take(23).count()
}

fn ask_what_to_bring(clean: &str, memory: &Memory) -> Option<String> {
    let person = memory.focused_person.as_ref()?;
    let (kind, _) = person.trouble?;
    if kind != TroubleKind::Hospital || !mentions(&SOMEONE_BRING_SOMEONE, clean) {
        return None;
    }
    if safety_thread(memory)
        || !memory
            .turns
            .iter()
            .rev()
            .take(3)
            .any(|turn| turn.user.trim_matches(is_space) == person.said)
    {
        return None;
    }
    let query = format!("{} {clean}", person.said);
    let found = lookup::lookup(&query)?;
    (found.id == "gift-hospital" && !memory.turns.iter().any(|turn| turn.bot == found.reply))
        .then(|| found.reply.into_owned())
}

fn on_line(clean: &str, said: &str, reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let tokens = token_count(clean);
    if tokens == 0 || tokens > 22 {
        return None;
    }
    let rendered = without_sick_figure(clean);
    let clean = rendered.as_ref();
    if mentions(&SOMEONE_BRING_SOMEONE, clean)
        && memory
            .focused_person
            .as_ref()
            .and_then(|person| person.trouble)
            .is_some_and(|(kind, _)| kind == TroubleKind::Hospital)
    {
        return ask_what_to_bring(clean, memory);
    }
    let care = about_someone(clean, reading, memory);
    if care.as_deref().is_some_and(|reply| {
        SOMEONE_NORMAL.iter().any(|&(_, normal)| normal == reply)
            && (mentions(&SOMEONE_IS_IT_NORMAL, clean) || mentions(&SOMEONE_NEXT_STEP, clean))
    }) {
        return care;
    }
    if tokens <= 16
        && !NEWS_WITH_A_NAME
            .iter()
            .any(|pattern| mentions(pattern, clean))
        && let Some(reply) = feeling::reply(clean, said, memory)
    {
        return Some(reply);
    }
    care
}

/// Propose the appropriate reply for a statement or follow-up about another person.
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(reply) = travel_limit::reply(reading.raw, memory) {
        return Some(reply);
    }
    let typed = flat(reading.raw);
    let said = &reading.line.plain;
    on_line(&typed, said, reading, memory).or_else(|| {
        [said, &reading.line.norm_text]
            .into_iter()
            .find_map(|retry| {
                (retry.trim_matches(is_space) != typed && token_count(retry) <= 8)
                    .then(|| on_line(retry, retry, reading, memory))
                    .flatten()
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn follows_admission_and_discharge_across_five_turns() {
        let mut bot = Bot::new(Some("0")).unwrap();
        for (line, expected) in [
            (
                "my mum is in hospital",
                "That is a worrying place for your mum to be. What has the hospital said?",
            ),
            (
                "still there",
                "No change yet, then. How has your mum been today?",
            ),
            (
                "what should i ask them",
                "Ask the ward what the plan is and who to ring if something changes at night; write it down, because it is hard to remember later.",
            ),
            (
                "what should i bring her",
                "Small things they can use lying in bed: a long phone cable and a charged power bank, earphones, a puzzle book or a magazine, a decent lip balm, a light blanket, or a printed photo of the family to put on the locker. Skip flowers on most wards and skip food unless you know the diet. Ask the ward what is allowed before you bring it in, and go when you can stay a while - the visit is the gift.",
            ),
            (
                "shes home from hospital now",
                "Good news. Keep the first week small: short visits, early nights, and let your mum set the pace rather than the other way round.",
            ),
        ] {
            let turn = bot.turn(line);
            assert_eq!(turn.module, Some("chat"), "{line}");
            assert_eq!(turn.reply, expected, "{line}");
        }
    }

    #[test]
    fn keeps_the_patient_through_an_indirect_question_and_updates_on_a_new_subject() {
        let mut memory = Memory::default();
        for line in ["my grandad has dementia", "he keeps asking where my nan is"] {
            observe(&Reading::new(line), &mut memory);
            memory.turns.push(Exchange {
                user: line.into(),
                bot: "Got it.".into(),
                module: "greeting",
            });
        }
        let follow = Reading::new("what do i say");
        observe(&follow, &mut memory);
        assert_eq!(memory.focused_person.as_ref().unwrap().who, "grandad");
        let line = Reading::new("my mums been in hospital a week");
        observe(&line, &mut memory);
        assert_eq!(memory.focused_person.as_ref().unwrap().who, "mum");
        assert!(respond(&line, &memory).is_some_and(|reply| reply.contains("your mum")));
    }

    #[test]
    fn a_yes_no_question_does_not_file_a_condition() {
        let mut memory = Memory::default();
        observe(&Reading::new("my mum is in hospital"), &mut memory);
        observe(
            &Reading::new("is my father getting forgetful?"),
            &mut memory,
        );
        let kept = memory.focused_person.as_ref().unwrap();
        assert_eq!(kept.who, "mum");
        assert_eq!(kept.trouble, Some((TroubleKind::Hospital, 0)));
    }

    #[test]
    fn a_safety_continuation_takes_the_ordinary_family_reply_off_the_table() {
        let mut memory = Memory::default();
        let previous = "my husband fell off a ladder";
        observe(&Reading::new(previous), &mut memory);
        memory.turns.push(Exchange {
            user: previous.into(),
            bot: "Keep him still while you wait for them.".into(),
            module: "safety",
        });
        let line = Reading::new("he is confused");
        observe(&line, &mut memory);
        assert!(respond(&line, &memory).is_none());
    }

    #[test]
    fn distinguishes_a_childs_feeling_from_your_own_and_an_adults() {
        let mut memory = Memory::default();
        for line in [
            "my baby hasnt been sleeping",
            "my daughter is 3 and is going to nursery",
        ] {
            let reading = Reading::new(line);
            crate::bot::memory::observe(&reading, &mut memory);
            observe(&reading, &mut memory);
            memory.turns.push(Exchange {
                user: line.into(),
                bot: "Got it.".into(),
                module: "greeting",
            });
        }
        let line = Reading::new("shes really upset");
        assert!(respond(&line, &memory).is_some_and(|reply| {
            ["small child", "little one", "Young children"]
                .iter()
                .any(|words| reply.contains(words))
        }));
        assert!(respond(&Reading::new("my wife is not tired"), &memory).is_none());
        assert!(respond(&Reading::new("my wife is tired of me"), &memory).is_none());
    }
}
