//! Records of occupations used when a later turn asks about the same work.

use super::{workplace::words, workplace_data as data};
use fancy_regex::Regex as Lookaround;
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

pub(in crate::bot) fn is_job_word(word: &str) -> bool {
    data::JOB_WORDS.contains(&word)
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    JOB_AFTER,
    JOB_BECOME,
    JOB_WORK,
    JOB_APPOSITIVE,
    JOB_AS_A,
    JOB_WANTED,
    CLAUSE_EDGE,
    POSSESSION,
    JOB_GOT_POSSESSIVE,
    JOB_OWN_VOICE,
    JOB_MINE,
    A_SHIFT_NOT_A_TRADE,
    CHANGED_WORK,
    WORK_NOW
);
macro_rules! lookarounds {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Lookaround> =
            LazyLock::new(|| Lookaround::new(data::$name).expect(stringify!($name)));)+
    };
}
lookarounds!(
    JOB_WAS,
    JOB_GOT_AT,
    OTHER_JOB,
    OTHER_JOB_AS,
    OTHER_JOB_NAMED
);

static SPLIT_OWNERS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"(?i)\b(?:his|her|their|our|my)\s+(?:[a-z]+\s+)?(?:mum|mom|dad|father|mother|wife|husband|son|daughter|sister|brother|friend|partner|kids?|children)\b",
        r"(?i)\b(?:he|she|they)\s+(?:worked|works|got|began|started|is|was|were)\b",
    ]).expect("whose job the clause names")
});
static PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:my|our|his|her|their)\s+([a-z][a-z'-]*)")
        .expect("explicit household relations")
});
static ASCII_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());

#[derive(Default)]
pub(crate) struct Work {
    mine: Vec<Own>,
    others: Vec<Other>,
    recent_woman: Option<String>,
    recent_man: Option<String>,
}

struct Own {
    title: String,
    line: String,
    weight: u8,
}

struct Other {
    relation: String,
    title: String,
}

impl Work {
    pub(super) fn observe(&mut self, line: &str, plain: &str) {
        if let Some((title, weight)) = own(line).or_else(|| own(plain)) {
            match self.mine.last_mut() {
                Some(previous) if previous.title == title => previous.weight = weight,
                Some(previous) if previous.weight > weight => (),
                _ => self.mine.push(Own {
                    title,
                    line: same_line_key(line),
                    weight,
                }),
            }
        }
        for found in PERSON.captures_iter(line) {
            let low = found[1].to_lowercase();
            let role = relation(&low);
            if pronoun_fits("she", role) {
                self.recent_woman = Some(role.to_owned());
            }
            if pronoun_fits("he", role) {
                self.recent_man = Some(role.to_owned());
            }
        }
        let mut people = self.person_jobs(line);
        for person in self.person_jobs(plain) {
            if !people.iter().any(|held| held.relation == person.relation) {
                people.push(person);
            }
        }
        for person in people {
            // The duration reader consults the original logged fact. A subsequent update to
            // the same person's live record does not add another log entry.
            if !self
                .others
                .iter()
                .any(|held| held.relation == person.relation)
            {
                self.others.push(person);
            }
        }
    }

    fn person_jobs(&self, line: &str) -> Vec<Other> {
        let mut people = Vec::new();
        for (pattern, place) in [
            (&*OTHER_JOB_AS, true),
            (&*OTHER_JOB, false),
            (&*OTHER_JOB_NAMED, false),
        ] {
            for found in pattern.captures_iter(line).map_while(Result::ok) {
                let (Some(subject), Some(job)) = (found.name("subj"), found.name("job")) else {
                    continue;
                };
                let before = &line[..job.start()];
                if JOB_WANTED.is_match(CLAUSE_EDGE.split(before).last().unwrap_or("")) {
                    continue;
                }
                let Some(title) = job_title(job.as_str(), place, false, false) else {
                    continue;
                };
                if data::SOBER_WORDS.contains(&title.as_str()) {
                    continue;
                }
                let subject = subject.as_str().to_lowercase();
                let role = match subject.split_once(' ') {
                    Some((_, role)) => relation(role),
                    None if subject == "she" => self.recent_woman.as_deref().unwrap_or(""),
                    None if subject == "he" => self.recent_man.as_deref().unwrap_or(""),
                    _ => continue,
                };
                if (pronoun_fits("she", role) || pronoun_fits("he", role))
                    && !people.iter().any(|person: &Other| person.relation == role)
                {
                    people.push(Other {
                        relation: role.to_owned(),
                        title,
                    });
                }
            }
        }
        people
    }

    pub(super) fn earlier_job(&self, current: &str) -> Option<&str> {
        let cleaned = same_line_key(current);
        self.mine
            .iter()
            .rev()
            .find(|prior| prior.line != cleaned && !A_SHIFT_NOT_A_TRADE.is_match(&prior.title))
            .map(|prior| prior.title.as_str())
    }

    pub(super) fn own_job(&self) -> Option<&str> {
        self.mine.last().map(|prior| prior.title.as_str())
    }

    pub(super) fn their_job(&self, pronoun: &str) -> Option<(&str, &str)> {
        self.others
            .iter()
            .rev()
            .find(|record| pronoun_fits(pronoun, &record.relation))
            .map(|record| (record.relation.as_str(), record.title.as_str()))
    }
}

fn relation(role: &str) -> &str {
    data::SPOKEN_RELATION
        .binary_search_by_key(&role, |&(short, _)| short)
        .map_or(role, |index| data::SPOKEN_RELATION[index].1)
}

fn pronoun_fits(pronoun: &str, relation: &str) -> bool {
    match pronoun {
        "she" => data::SHE.contains(&relation),
        "he" => data::HE.contains(&relation),
        _ => data::SHE.contains(&relation) || data::HE.contains(&relation),
    }
}

fn same_line_key(text: &str) -> String {
    static JUNK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9 ]+").unwrap());
    words(&JUNK.replace_all(&text.to_lowercase(), " "))
        .collect::<Vec<_>>()
        .join(" ")
}

struct Candidate<'a> {
    value: &'a str,
    frame: &'a str,
    article: bool,
    past: bool,
    employer: bool,
    role_from_job: bool,
    end: usize,
}

fn current_jobs(text: &str) -> Vec<Candidate<'_>> {
    for regex in [&*JOB_AFTER, &*JOB_BECOME, &*JOB_WORK] {
        let work = std::ptr::eq(regex, &*JOB_WORK);
        let candidates: Vec<_> = regex
            .captures_iter(text)
            .filter_map(|match_| {
                let value = match_.get(if work { 3 } else { 1 })?;
                Some(Candidate {
                    value: value.as_str(),
                    frame: if work {
                        match_.get(1).map_or("", |part| part.as_str())
                    } else {
                        ""
                    },
                    article: work && match_.get(2).is_some(),
                    past: false,
                    employer: false,
                    role_from_job: false,
                    end: value.end(),
                })
            })
            .collect();
        if !candidates.is_empty() {
            return candidates;
        }
    }
    let past: Vec<_> = JOB_WAS
        .captures_iter(text)
        .map_while(Result::ok)
        .filter_map(|match_| {
            let value = match_.get(3)?;
            Some(Candidate {
                value: value.as_str(),
                frame: "",
                article: match_.get(2).is_some(),
                past: true,
                employer: false,
                role_from_job: false,
                end: value.end(),
            })
        })
        .collect();
    if !past.is_empty() {
        return past;
    }
    let named: Vec<_> = JOB_AS_A
        .captures_iter(text)
        .filter_map(|match_| {
            let clause = CLAUSE_EDGE
                .split(&text[..match_.get(0)?.start()])
                .last()
                .unwrap_or("");
            if JOB_WANTED.is_match(clause)
                || SPLIT_OWNERS.is_match(clause)
                || !(JOB_OWN_VOICE.is_match(clause) || JOB_MINE.is_match(clause))
            {
                return None;
            }
            let value = match_.get(1)?;
            Some(Candidate {
                value: value.as_str(),
                frame: "as",
                article: false,
                past: false,
                employer: false,
                role_from_job: true,
                end: value.end(),
            })
        })
        .collect();
    if !named.is_empty() {
        return named;
    }
    let employer: Vec<_> = JOB_GOT_AT
        .captures_iter(text)
        .map_while(Result::ok)
        .filter_map(|match_| {
            let value = match_.get(3)?;
            Some(Candidate {
                value: value.as_str(),
                frame: match_.get(1)?.as_str(),
                article: match_.get(2).is_some(),
                past: false,
                employer: true,
                role_from_job: false,
                end: value.end(),
            })
        })
        .collect();
    if !employer.is_empty() {
        return employer;
    }
    // A bare comma and noun is only an introduction when the preceding words name the speaker.
    if POSSESSION.is_match(text) || !JOB_OWN_VOICE.is_match(text) {
        return vec![];
    }
    JOB_APPOSITIVE
        .captures_iter(text)
        .filter_map(|match_| {
            let value = match_.get(1)?;
            Some(Candidate {
                value: value.as_str(),
                frame: "",
                article: true,
                past: false,
                employer: false,
                role_from_job: false,
                end: value.end(),
            })
        })
        .collect()
}

fn own(text: &str) -> Option<(String, u8)> {
    for candidate in current_jobs(text) {
        let frame = candidate.frame.to_lowercase();
        let place = matches!(frame.as_str(), "in" | "at" | "for" | "on") || candidate.employer;
        let lower = candidate.value.to_lowercase();
        let line = lower.trim_matches([' ', '.', '!', '?', ',']);
        if candidate.employer
            && ((frame == "in" && !candidate.article)
                || JOB_GOT_POSSESSIVE.is_match(line)
                || words(line).next().is_some_and(|first| {
                    data::JOB_GOT_NOT_A_PLACE.contains(&first)
                        && !data::JOB_GOT_AN_EMPLOYER.contains(&first)
                }))
        {
            continue;
        }
        if text[candidate.end..]
            .trim_start_matches(crate::text::is_space)
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_digit())
        {
            continue;
        }
        if let Some(title) = job_title(
            candidate.value,
            if candidate.role_from_job {
                false
            } else {
                place
            },
            candidate.past,
            candidate.article,
        ) {
            let weight = work_weight(&title, &frame);
            return Some((title, weight));
        }
    }
    if CHANGED_WORK.is_match(text)
        && let Some(work) = WORK_NOW.captures(text)
    {
        let place = work
            .get(3)?
            .as_str()
            .trim_matches(crate::text::is_space)
            .trim_matches(['.', ',', '?', '!', ':', '\'', '"', ';'])
            .to_lowercase();
        if !place.is_empty()
            && !data::JOB_STOP.contains(&place.as_str())
            && words(&place).take(4).count() <= 3
        {
            return Some((place, 2));
        }
    }
    None
}

fn job_title(raw: &str, place: bool, past: bool, article: bool) -> Option<String> {
    let tail = |parts: &[&str]| parts.iter().filter_map(|part| raw.find(part)).min();
    let cut = tail(data::JOB_TAIL).or_else(|| place.then(|| tail(data::JOB_TAIL_PLACE)).flatten());
    let phrase = &raw[..cut.unwrap_or(raw.len())];
    let mut work: Vec<_> = words(phrase).collect();
    while let Some(span) = (1..=work.len().min(3)).rev().find(|&span| {
        let end = work[work.len() - span..].join(" ").to_lowercase();
        data::THE_MOMENT.contains(&end.trim_matches(['.', ',', '?', '!', ':', '\'', '"', ';']))
    }) {
        work.truncate(work.len() - span);
    }
    let given = *work.first()?;
    let words: Vec<_> = work.iter().map(|word| word.to_lowercase()).collect();
    let first = words.first()?.as_str();
    if [
        data::JOB_STOP,
        data::NOT_NAMES,
        data::NOT_NAMES_EXTRA,
        data::SELF_JUDGEMENT,
        &[
            "bit", "little", "lot", "couple", "few", "my", "our", "his", "her", "their", "your",
        ],
    ]
    .iter()
    .any(|list| list.contains(&first))
        || (data::JUDGEMENT_NOUNS.contains(&first)
            && words
                .get(1)
                .is_none_or(|word| data::JUDGEMENT_AFTER.contains(&word.as_str())))
        || (work.len() == 1 && given.chars().next().is_some_and(char::is_uppercase))
        || (past && (data::WAS_NOT_A_JOB.contains(&first) || !was_role(&words, article)))
    {
        return None;
    }
    let mut keep = &words[..words.len().min(3)];
    while keep
        .last()
        .is_some_and(|end| data::JOB_DANGLING.contains(&end.trim_matches(['.', ',', '!', '?'])))
    {
        keep = &keep[..keep.len() - 1];
    }
    (!keep.is_empty()).then(|| keep.join(" "))
}

fn was_role(words: &[String], article: bool) -> bool {
    let body: Vec<_> = words
        .iter()
        .map(String::as_str)
        .skip_while(|word| {
            [
                "both", "all", "just", "only", "still", "now", "then", "also",
            ]
            .contains(word)
        })
        .take(3)
        .collect();
    let Some(last) = body.last() else {
        return false;
    };
    article
        || (!body
            .iter()
            .any(|word| word.len() > 4 && (word.ends_with("ing") || word.ends_with("ed")))
            && ![data::STATE_WORDS, data::PAST_STATES, data::PREGNANCY_STATES]
                .iter()
                .any(|set| set.contains(last))
            && (body.iter().any(|word| {
                data::JOB_WORDS.contains(word)
                    || word
                        .strip_suffix('s')
                        .is_some_and(|one| data::JOB_WORDS.contains(&one))
            }) || (last.len() > 4
                && data::ROLE_ENDING
                    .iter()
                    .any(|ending| last.ends_with(ending)))))
}

fn work_weight(job: &str, frame: &str) -> u8 {
    if matches!(frame, "in" | "at" | "on") {
        return 2;
    }
    if ASCII_WORD
        .find_iter(job)
        .map(|word| word.as_str())
        .filter(|word| !data::WORK_DETAIL_FILLER.contains(word))
        .all(|word| matches!(word, "job" | "jobs") || data::WORK_DETAIL.contains(&word))
    {
        1
    } else {
        3
    }
}
