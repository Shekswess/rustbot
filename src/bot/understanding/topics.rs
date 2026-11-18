//! Weighted topic vocabulary and contextual evidence from the pinned topic reader.
#[path = "topic_data.rs"]
mod data;
use super::{
    entities::{self, Entity},
    line::Line,
    numbers::Quantity,
};
use data::*;

type Weights = &'static [(&'static str, i32)];
struct Lexeme {
    text: &'static str,
    order: usize,
    weights: Weights,
}
struct Cue {
    reach: usize,
    words: &'static [&'static str],
}
fn has(set: &[&str], s: &str) -> bool {
    set.binary_search(&s).is_ok()
}
fn any(words: &[String], set: &[&str]) -> bool {
    words.iter().any(|w| has(set, w))
}
fn get<'a, T>(table: &'a [(&str, T)], s: &str) -> Option<&'a T> {
    table
        .binary_search_by_key(&s, |e| e.0)
        .ok()
        .map(|i| &table[i].1)
}
fn lexeme(s: &str) -> Option<&'static Lexeme> {
    TABLE
        .binary_search_by_key(&s, |row| row.text)
        .ok()
        .map(|i| &TABLE[i])
}
fn family(s: &str) -> Option<&'static Lexeme> {
    for end in DERIVED_ENDINGS {
        if let Some(stem) = s.strip_suffix(end)
            && stem.chars().count() >= 5
            && let Some(hit) = lexeme(stem).or_else(|| lexeme(&format!("{stem}e")))
        {
            return Some(hit);
        }
        if s.chars().count() >= 5
            && !has(STOPWORDS, s)
            && let Some(hit) = lexeme(&format!("{s}{end}"))
        {
            return Some(hit);
        }
    }
    for &(a, b) in FAMILY_PAIRS {
        for (start, end) in [(a, b), (b, a)] {
            if let Some(stem) = s.strip_suffix(start)
                && let Some(hit) = lexeme(&format!("{stem}{end}"))
            {
                return Some(hit);
            }
        }
    }
    None
}
fn lookup(s: &str) -> impl Iterator<Item = (&'static str, i32)> {
    let mut direct = lexeme(s);
    if direct.is_none() {
        direct = [
            ("ies", "y"),
            ("es", ""),
            ("s", ""),
            ("ing", ""),
            ("ing", "e"),
            ("ed", ""),
            ("ed", "e"),
        ]
        .iter()
        .find_map(|(suffix, end)| {
            let stem = s.strip_suffix(suffix)?;
            if stem.chars().count() <= 2 {
                return None;
            }
            if end.is_empty() {
                lexeme(stem)
            } else {
                lexeme(&format!("{stem}{end}"))
            }
        });
    }
    let derived = direct.is_none();
    direct
        .or_else(|| family(s))
        .map_or(&[][..], |entry| entry.weights)
        .iter()
        .copied()
        .filter(move |(t, _)| !derived || *t != "about-the-bot")
}
fn cued(norm: &[String], at: usize, tiers: &[Cue]) -> bool {
    tiers.iter().any(|c| {
        any(
            &norm[at.saturating_sub(c.reach)..(at + c.reach + 1).min(norm.len())],
            c.words,
        )
    })
}
pub(super) fn background(norm: &[String]) -> (Vec<bool>, std::ops::Range<usize>) {
    let stop = |i: usize| {
        (i + 1..norm.len())
            .find(|&p| has(CLAUSE_BREAKS, &norm[p]) || has(WH_QUESTIONS, &norm[p]))
            .unwrap_or(norm.len())
    };
    let since = (1..norm.len())
        .find(|&i| has(REASON_STARTS, &norm[i]))
        .map_or(0..0, |i| i..stop(i));
    let mut spans = Vec::new();
    if matches!(norm.get(..2),Some([a,b])if a=="it"&&has(START_VERBS,b)) {
        spans.push((0, stop(1), true))
    }
    if let Some(i) =
        (1..norm.len()).find(|&i| has(BACKGROUND_STARTS, &norm[i]) && !has(REPORTING, &norm[i - 1]))
    {
        spans.push((i, stop(i), false))
    }
    let mut skip = vec![false; norm.len()];
    for (mut i, end, anywhere) in spans {
        while i < end {
            if !has(PLACE_PREPS, &norm[i])
                || !anywhere && i > 0 && !skip[i - 1] && !has(PLACE_LEADS, &norm[i - 1])
            {
                i += 1;
                continue;
            }
            i += 1;
            let limit = (i + 3).min(end);
            while i < limit
                && !has(CLAUSE_BREAKS, &norm[i])
                && !has(PLACE_STOP, &norm[i])
                && !has(PLACE_PREPS, &norm[i])
            {
                skip[i] = true;
                i += 1;
            }
        }
    }
    (skip, since)
}

/// The normalized words whose clause only describes where something happened.
pub(crate) fn background_words(norm: &[String]) -> impl Iterator<Item = &str> + Clone {
    let (skip, _) = background(norm);
    norm.iter()
        .zip(skip)
        .filter(|(_, marked)| *marked)
        .map(|(word, _)| word.as_str())
}

fn sums(norm: &[String]) -> Vec<usize> {
    let training = any(norm, EXERCISE_NOUNS);
    norm.windows(2).enumerate().filter_map(|(i,p)|{
        if !matches!(p[0].as_str(),"work"|"works"|"working"|"worked")||p[1]!="out"{return None}
        let next=&norm[i+2..(i+8).min(norm.len())];
        if matches!(next.get(..3),Some([a,t,b])if has(NUMBER_WORDS,a)&&t=="times"&&has(NUMBER_WORDS,b)){return Some(i)}
        (!training&&!any(&next[..3.min(next.len())],EXERCISE_AFTER)&&(any(next,SUMS_AFTER)||any(next,NUMBER_WORDS)||next.iter().any(|word|!word.is_empty()&&word.chars().all(char::is_numeric)))).then_some(i)
    }).collect()
}
pub(crate) fn warmth(norm: &[String]) -> Option<&str> {
    for (i, w) in norm.iter().enumerate() {
        if !has(WARMTH_WORDS, w) {
            continue;
        }
        let mut at = i;
        while at > 0 && has(WARMTH_DEGREE, &norm[at - 1]) {
            at -= 1
        }
        if at < 2 || !has(WARMTH_ARE, &norm[at - 1]) || any(&norm[at..i], WARMTH_HABITUAL) {
            continue;
        }
        let who = &norm[at - 2];
        if !has(COLD_PEOPLE, who)
            && !has(PERSON_WORDS, who)
            && !who.strip_suffix('s').is_some_and(|s| has(PERSON_WORDS, s))
        {
            continue;
        }
        if norm[i + 1..]
            .iter()
            .find(|w| !has(WARMTH_THEN, w))
            .is_none_or(|w| matches!(w.as_str(), "." | "!" | "?" | ","))
        {
            return Some(w);
        }
    }
    None
}
pub(crate) fn elsewhere(norm: &[String], i: usize) -> bool {
    let w = &norm[i];
    if w == "teething" && any(norm, TEETHING_IS_A_FAULT) && !any(norm, TEETHING_AND_A_CHILD) {
        return true;
    }
    get(ELSEWHERE, w).is_some_and(|tiers| {
        i > 0 && has(BODY_TWO_LIFE, w) && has(OWNING_WORDS, &norm[i - 1]) || cued(norm, i, tiers)
    })
}
fn portion(norm: &[String], i: usize) -> bool {
    if !has(PORTION_WORDS, &norm[i]) {
        return false;
    }
    let next = norm.get(i + 1).map(String::as_str);
    let then = norm.get(i + 2);
    if matches!(next, Some("a" | "an" | "the")) {
        return then.is_none_or(|w| !has(PORTION_UNITS, w));
    }
    next == Some("of") && then.is_some_and(|w| matches!(w.as_str(), "a" | "an" | "the"))
}
fn body_verb(norm: &[String], i: usize) -> bool {
    if norm.get(i + 1).is_some_and(|w| has(DETERMINERS, w)) {
        let mut at = i + 2;
        if let Some(first) = norm.get(at)
            && (!has(DEGREE_NOUNS, first) || norm.get(at + 1).is_some_and(|s| s == "of"))
        {
            let mut noun = first;
            while norm.get(at + 1).is_some_and(|w| w == "of") {
                at += 2;
                if norm.get(at).is_some_and(|w| has(DETERMINERS, w)) {
                    at += 1;
                }
                if let Some(w) = norm.get(at) {
                    noun = w
                }
            }
            return has(BODY_OR_SELF, noun);
        }
    }
    norm.get(i + 1)
        .is_none_or(|w| has(BODY_OR_SELF, w) || has(CUE_CARRIERS, w))
        || !norm.iter().enumerate().any(|(j, w)| {
            j != i
                && !has(POLY_FORMS, w)
                && lookup(w).any(|(t, n)| n >= 2 && !matches!(t, "health" | "small-talk"))
        })
}
fn mute(topic: &str, norm: &[String], at: usize, len: usize, study: bool) -> bool {
    let span = &norm[at..at + len];
    if has(WATCHING, &span[len - 1]) && norm.get(at + len).is_some_and(|w| has(WATCH_OUT_AFTER, w))
    {
        return true;
    }
    if topic == "family-and-parenting"
        && len > 1
        && span.join(" ").contains("health visitor")
        && norm
            .join(" ")
            .replace("health visitor", " ")
            .replace("health visitors", " ")
            .split_whitespace()
            .any(|w| lookup(w).any(|(t, n)| t == "health" && n >= 2))
    {
        return true;
    }
    if len == 1 && has(BODY_CUES, &norm[at]) {
        return !body_verb(norm, at);
    }
    !study
        && get(VERB_CUES, topic)
            .and_then(|verbs| span.iter().position(|w| has(verbs, w)))
            .is_some_and(|i| norm.get(at + i + 1).is_some_and(|w| has(DETERMINERS, w)))
}
#[derive(Default)]
struct Scores(Vec<(&'static str, i32, i32, usize, bool)>);
impl Scores {
    fn note(&mut self, topic: &'static str, points: i32, pos: usize, own: bool) {
        if let Some(row) = self.0.iter_mut().find(|r| r.0 == topic) {
            row.1 += points;
            row.2 = row.2.max(points);
            row.4 |= own
        } else {
            self.0.push((topic, points, points, pos, own))
        }
    }
    fn strong_besides(&self, topic: &str, n: i32) -> bool {
        self.0.iter().any(|r| r.0 != topic && r.1 >= n)
    }
}
fn naked_season(norm: &[String], phrases: &[(usize, &Lexeme, usize, usize)]) -> bool {
    let mut spots = norm
        .iter()
        .enumerate()
        .filter(|(_, w)| has(SEASONS, w))
        .map(|(i, _)| i)
        .peekable();
    spots.peek().is_some_and(|&i| i > 2)
        && !spots.any(|i| has(WEATHER_BEFORE, &norm[i - 1]))
        && !any(norm, WEATHER_ITSELF)
        && !norm.iter().any(|w| {
            !has(SEASONS, w) && lookup(w).any(|(t, n)| t == "nature-and-weather" && n >= 2)
        })
        && !phrases.iter().any(|(_, row, i, len)| {
            !any(&norm[*i..*i + *len], SEASONS)
                && row.weights.iter().any(|(t, _)| *t == "nature-and-weather")
        })
}
fn phrases(norm: &[String]) -> Vec<(usize, &'static Lexeme, usize, usize)> {
    let mut spans = Vec::new();
    for start in 0..norm.len() {
        for len in 2..=7.min(norm.len() - start) {
            if let Some(row) = lexeme(&norm[start..start + len].join(" ")) {
                spans.push((row.order, row, start, len));
            }
        }
    }
    spans.sort_by_key(|s| s.0);
    spans
}
fn score(line: &Line, phrases: &[(usize, &Lexeme, usize, usize)], bg: &[bool]) -> Scores {
    let norm = &line.norm;
    let sums = sums(norm);
    let study = any(norm, STUDY_OBJECTS);
    let mut found = Scores::default();
    let warm = warmth(norm).is_some();
    let mut idioms = vec![false; norm.len()];
    for phrase in IDIOMS {
        let len = phrase.split_whitespace().count();
        for (j, p) in norm.windows(len).enumerate() {
            if p.iter().map(String::as_str).eq(phrase.split_whitespace()) {
                idioms[j..j + len].fill(true)
            }
        }
    }
    for (i, p) in norm.windows(2).enumerate() {
        if !has(DRIVES, &p[0]) || !has(DRIVEN_PERSON, &p[1]) {
            continue;
        }
        let end = (i + 7).min(norm.len());
        let tail = &norm[i + 2..end];
        if any(tail, DRIVEN_MOOD) || any(tail, DRIVEN_TAIL) && any(tail, DRIVEN_JOINERS) {
            idioms[i] = true;
            idioms[i + 1] = true;
            for j in i + 2..end {
                if [DRIVEN_MOOD, DRIVEN_TAIL, DRIVEN_JOINERS]
                    .iter()
                    .any(|s| has(s, &norm[j]))
                {
                    idioms[j] = true
                }
            }
        }
    }
    let mut sense_at = vec![false; norm.len()];
    let mut sense_topics = Vec::new();
    for &(forms, topic, tiers) in SENSES {
        let mut first = None;
        let mut present = false;
        for (i, w) in norm.iter().enumerate().filter(|(_, w)| has(forms, w)) {
            let _ = w;
            first.get_or_insert(i);
            if cued(norm, i, tiers) {
                sense_at[i] = true;
                present = true
            }
        }
        if let Some(at) = first
            && present
        {
            sense_topics.push((topic, at))
        }
    }
    for (i, w) in norm.iter().enumerate() {
        if bg[i]
            || idioms[i]
            || sense_at[i]
            || sums.contains(&i)
            || entities::verdict(norm, i)
            || has(RUN_OUT_BEFORE, w) && norm.get(i + 1).is_some_and(|t| t == "out")
            || has(COMPLAINT_OF, w) && norm.get(i + 1).is_some_and(|t| has(COMPLAINT_OF_NEXT, t))
            || entities::is_person_place(norm, i)
            || elsewhere(norm, i)
            || warm && has(WARMTH_WORDS, w)
        {
            continue;
        }
        for (topic, weight) in lookup(w) {
            if !mute(topic, norm, i, 1, study) {
                found.note(topic, weight, i, !has(POLY_FORMS, w) && !portion(norm, i));
            }
        }
    }
    for (topic, at) in sense_topics {
        found.note(topic, 6, at, true);
    }
    let ages = entities::child_age_positions(norm);
    if let Some(&p) = ages.first()
        && (found
            .0
            .iter()
            .any(|r| r.0 == "family-and-parenting" && r.1 >= 2)
            || !found
                .0
                .iter()
                .any(|r| !matches!(r.0, "family-and-parenting" | "feelings") && r.1 >= 2))
    {
        found.note("family-and-parenting", 2, p, true)
    }
    if let Some((i, _)) = norm.windows(2).enumerate().find(|(i, p)| {
        if !has(BOOK_WORDS, &p[0]) || p[1] != "for" {
            return false;
        }
        let tail = &norm[i + 2..(i + 7).min(norm.len())];
        any(tail, A_CHILD_TO_READ_FOR)
            || ages.iter().any(|a| *a >= i + 2 && *a <= i + 5)
            || tail.iter().any(|w| w == "year") && tail.iter().any(|w| w == "old" || w == "olds")
    }) {
        found.note("entertainment", 3, i, true)
    }
    if let Some(&i) = sums.first()
        && !found.strong_besides("", 2)
    {
        found.note("maths-and-numbers", 2, i, true)
    }
    if let Some(i) = output_position(norm) {
        found.note("health", 2, i, true)
    }
    if let Some(i) = stone(norm) {
        found.note("fitness", 3, i, true)
    }
    if let Some(i) = tight_money(norm, &line.text)
        && !found.strong_besides("money", 2)
    {
        found.note("money", 3, i, true)
    }
    if let Some(&i) = sums.first()
        && any(norm, SUM_MONEY)
    {
        found.note("money", 3, i, true)
    }
    let named = norm
        .iter()
        .any(|w| !has(ADDRESS_WORDS, w) && lookup(w).any(|(t, n)| n >= 2 && t != "small-talk"));
    let mut last = None;
    for &(id, row, start, len) in phrases {
        if last == Some(id)
            || !line.norm_text.contains(row.text)
            || bg[start]
            || !sums.is_empty() && row.text == "work out"
            || named && row.weights.iter().all(|(t, _)| *t == "small-talk")
        {
            continue;
        }
        last = Some(id);
        for &(topic, n) in row.weights {
            if !mute(topic, norm, start, len, study) {
                found.note(topic, n, start, true)
            }
        }
    }
    let text = format!(" {} ", line.norm_text);
    if !found.strong_besides("", 3)
        && let Ok(Some(m)) = JUDGEMENT_OF_RECORD.find(&text)
    {
        found.note(
            "feelings",
            3,
            text[..m.start()].bytes().filter(|b| *b == b' ').count(),
            true,
        )
    }
    if !found.strong_besides("", 3) {
        for m in NOT_CUT_OUT.find_iter(&text).flatten() {
            let back: Vec<_> = text[..m.start()].split_whitespace().rev().take(6).collect();
            if back.iter().any(|w| has(NOT_WORDS, w))
                && back
                    .iter()
                    .find(|w| has(OWN_SELF, w) || has(OTHER_PERSON, w))
                    .is_some_and(|w| has(OWN_SELF, w))
            {
                found.note(
                    "feelings",
                    3,
                    text[..m.start()].bytes().filter(|b| *b == b' ').count(),
                    true,
                );
                break;
            }
        }
    }
    if naked_season(norm, phrases)
        && found.strong_besides("nature-and-weather", 2)
        && let Some(r) = found.0.iter_mut().find(|r| r.0 == "nature-and-weather")
    {
        r.4 = false
    }
    for (pat, topic) in CANCELS {
        if !found.strong_besides(topic, 2) {
            continue;
        }
        if let Some(r) = found.0.iter_mut().find(|r| r.0 == *topic)
            && let Ok(Some(m)) = pat.find(&text)
            && text[m.end()..]
                .split_whitespace()
                .next()
                .is_none_or(|w| has(CANCEL_AFTER, w) || matches!(w, "." | "," | ";" | ":"))
        {
            r.1 = (r.1 - 2).max(0)
        }
    }
    found
        .0
        .sort_by_key(|r| (std::cmp::Reverse(r.1), std::cmp::Reverse(r.2), r.3));
    for r in &mut found.0 {
        if !r.4 {
            r.1 = r.1.min(1);
            r.2 = r.2.min(1)
        }
    }
    found
}

pub(super) fn determiner(w: &str) -> bool {
    has(DETERMINERS, w)
}
pub(crate) fn gift(norm: &[String]) -> bool {
    let includes = |set| any(norm, set);
    let says = |s| norm.iter().any(|w| w == s);
    let occasion = includes(GIFT_OCCASION);
    let asking = includes(GIFT_ASKING) || occasion;
    let negative = says("nothing") || says("anything");
    let wanting = includes(GIFT_WANTING);
    if includes(GIFT_ALWAYS)
        || includes(GIFT_WORDS) && (asking || includes(GIVE_VERB) || says("for"))
        || wanting && includes(GIVE_TO) && says("for") && negative
    {
        return true;
    }
    for (i, w) in norm.iter().enumerate() {
        let rest = &norm[i + 1..];
        if occasion && has(GIVE_TO, w) && any(&rest[..2.min(rest.len())], GIVE_THINGS) {
            return true;
        }
        if !has(GIVE_VERB, w) || rest.is_empty() {
            continue;
        }
        let head = &rest[0];
        if asking && (has(GIVE_THING, head) || has(GIFT_WORDS, head) || has(GIFT_OCCASION, head)) {
            return true;
        }
        let Some(start) = rest.iter().take(2).enumerate().find_map(|(j, w)| {
            if has(GIVE_TO, w) {
                Some(j + 1)
            } else if has(GIVE_DETERMINERS, w) && rest.get(j + 1).is_some_and(|n| has(GIVE_TO, n)) {
                Some(j + 2)
            } else {
                None
            }
        }) else {
            continue;
        };
        let tail = &rest[start..];
        if asking
            && (tail.first().is_none_or(|w| has(GIVE_RELATIVE, w))
                || any(&tail[..3.min(tail.len())], GIFTS_OR_OCCASIONS)
                || tail.first().is_some_and(|w| w == "for")
                    && tail
                        .iter()
                        .skip(1)
                        .take(3)
                        .any(|w| w.chars().any(char::is_numeric)))
        {
            return true;
        }
        if wanting
            && tail.first().is_some_and(|w| {
                has(GIVE_THING, w)
                    || has(GIFT_WORDS, w)
                    || any(&tail[..3.min(tail.len())], GIFT_OCCASION)
                    || negative && tail.iter().any(|w| w == "for")
            })
        {
            return true;
        }
    }
    false
}
fn named_condition(norm: &[String], skip: &std::ops::Range<usize>) -> bool {
    if any(norm, PETS) && !any(norm, HUMAN) {
        return false;
    }
    norm.iter().enumerate().any(|(i, w)| {
        if !is_condition(w)
            || skip.contains(&i)
            || get(CONDITION_NOT_NEXT, w)
                .is_some_and(|s| norm.get(i + 1).is_some_and(|n| has(s, n)))
            || get(CONDITION_OTHER_LIFE, w).is_some_and(|s| any(norm, s))
        {
            return false;
        }
        let near = |set| {
            norm.iter()
                .enumerate()
                .any(|(j, n)| !skip.contains(&j) && i.abs_diff(j) <= CONDITION_REACH && has(set, n))
        };
        (!has(CONDITION_NEEDS_PART, w) || near(BODY))
            && get(CONDITION_NEEDS_WORD, w).is_none_or(|s| near(*s))
    })
}
pub(crate) fn rank(
    line: &Line,
    entities: &[Entity],
    quantities: &[Quantity],
    inherited: Option<&'static str>,
) -> (Vec<(&'static str, i32)>, Option<&'static str>) {
    rank_for_ask(line, entities, quantities, inherited, None)
}
pub(crate) fn rank_for_ask(
    line: &Line,
    entities: &[Entity],
    quantities: &[Quantity],
    inherited: Option<&'static str>,
    asks: Option<bool>,
) -> (Vec<(&'static str, i32)>, Option<&'static str>) {
    let norm = &line.norm;
    let (bg, reason) = background(norm);
    let phrases = phrases(norm);
    let mut found = score(line, &phrases, &bg);
    let subject = found.0.iter().find(|r| r.2 >= 2).map(|r| r.0);
    let includes = |set| any(norm, set);
    let has_entity = |kind: &str| entities.iter().any(|e| e.kind == kind);
    if has_entity("place") {
        let strong = found
            .0
            .iter()
            .any(|r| !matches!(r.0, "geography-and-travel" | "small-talk") && r.1 >= 2);
        let ask = asks.unwrap_or_else(|| {
            line.text.contains('?') || norm.first().is_none_or(|w| has(ASK_WORDS, w))
        });
        let points = if includes(TRAVEL_STRONG) {
            4
        } else if !has_entity("person") && !strong && ask {
            3
        } else {
            0
        };
        if points > 0 {
            found.note("geography-and-travel", points, 0, true)
        }
    }
    let ends = |entity: &Entity, set: &[&str]| {
        entity
            .text
            .split_whitespace()
            .last()
            .is_some_and(|w| has(set, &w.to_lowercase()))
    };
    if entities
        .iter()
        .any(|e| e.kind == "place" && ends(e, PLACE_HOMOGRAPHS))
    {
        found.note("geography-and-travel", COUNTRY_BOOST, 0, true)
    }
    if entities.iter().any(|e| ends(e, PETS)) {
        found.note("pets", PET_BOOST, 0, true)
    }
    let named = |set| {
        norm.iter()
            .enumerate()
            .any(|(i, w)| !reason.contains(&i) && has(set, w))
    };
    if named(BODY) && named(HURTS) {
        found.note("health", HURT_BOOST, 0, true)
    }
    if norm.iter().enumerate().any(|(i, w)| {
        has(HEAD_WORDS, w)
            && !has(HEAD_FOG, w)
            && any(
                &norm[i + 1..(i + 1 + HEAD_FOG_REACH).min(norm.len())],
                HEAD_FOG,
            )
    }) {
        found.note("health", HEAD_FOG_BOOST, 0, true)
    }
    let people = found.0.iter().any(|r| {
        matches!(
            r.0,
            "family-and-parenting" | "friends-and-relationships" | "pets"
        ) && r.1 >= 2
    });
    if !people
        && norm.iter().enumerate().any(|(i, w)| {
            has(SKIN_PARTS, w)
                && !reason.contains(&i)
                && norm.iter().enumerate().any(|(j, n)| {
                    !reason.contains(&j) && has(SKIN_STATES, n) && i.abs_diff(j) <= SKIN_REACH
                })
        })
    {
        found.note("health", SKIN_BOOST, 0, true)
    }
    if named_condition(norm, &reason) {
        found.note("health", CONDITION_BOOST, 0, true)
    }
    if norm.iter().enumerate().any(|(at, w)| {
        w == "up"
            && at > 0
            && !reason.contains(&at)
            && (has(PLAYS_UP_BEFORE, &norm[at - 1])
                || at >= 2
                    && matches!(norm[at - 1].as_str(), "me" | "my")
                    && has(PLAYS_UP_BEFORE, &norm[at - 2]))
            && (at.saturating_sub(PLAY_UP_REACH)..at)
                .any(|i| !reason.contains(&i) && has(BODY, &norm[i]))
    }) {
        found.note("health", PART_UP_BOOST, 0, true)
    }
    let text = format!(" {} ", norm.join(" "));
    if unwell(norm) {
        found.note("health", UNWELL_BOOST, 0, true)
    }
    if NOSE_BLOCKED.is_match(&text).unwrap_or(false) {
        found.note("health", NOSE_BLOCKED_BOOST, 0, true)
    }
    let owns = norm
        .windows(2)
        .any(|p| has(OWNERS, &p[0]) && has(ANIMALS, &p[1]));
    if owns {
        found.note("pets", PET_BOOST, 0, true)
    }
    if norm
        .windows(2)
        .any(|p| has(OWNED_BY, &p[0]) && has(FRIEND_WORDS, &p[1]))
    {
        found.note("friends-and-relationships", FRIEND_BOOST, 0, true)
    }
    if found.0.iter().any(|r| has(PEOPLE_TOPICS, r.0) && r.1 >= 2)
        && let Some(topic) = someone_wants(norm)
    {
        let points = found
            .0
            .iter()
            .find(|r| r.0 == topic)
            .map_or(2, |r| r.1.max(2))
            + 1;
        found.note(topic, points, 0, true);
    }
    if !includes(CROP_KITCHEN)
        && CROP_STATES.iter().any(|s| text.contains(s))
        && (includes(GROWING_PLACES)
            || norm
                .windows(2)
                .any(|p| has(OWNERS, &p[0]) && has(CROPS, &p[1])))
    {
        found.note("garden-and-plants", CROP_BOOST, 0, true)
    }
    if includes(GIVEN_UP_FOOD) && GIVING_UP.iter().any(|w| text.contains(w)) {
        found.note("food-and-drink", GIVING_UP_BOOST, 0, true)
    }
    let worn = norm
        .iter()
        .enumerate()
        .any(|(i, w)| has(WORN_OUT, w) && any(&norm[i.saturating_sub(3)..i], WORN_OUT_CUE))
        || WORN_OUT_SAID.iter().any(|p| {
            text.find(&format!(" {p} ")).is_some_and(|i| {
                text[..i]
                    .split_whitespace()
                    .rev()
                    .take(3)
                    .any(|w| has(WORN_OUT_CUE, w))
            })
        });
    if worn && !returns_the_question(norm) {
        found.note("sleep", WORN_OUT_BOOST, 0, true)
    }
    if includes(WORLD)
        && !includes(WORLD_NOT)
        && !includes(NATURE_WORDS)
        && !includes(HURTS)
        && !owns
        && norm.first().is_some_and(|w| {
            has(MECHANISM, w) && (w != "how" || norm.get(1).is_some_and(|w| has(MECHANISM_HOW, w)))
        })
    {
        found.note("science", SCIENCE_BOOST, 0, true)
    }
    for number in quantities {
        if number.unit == Some("percent") {
            found.note("maths-and-numbers", 3, 0, true)
        }
    }
    if gift(norm) {
        found.note("shopping", GIFT_BOOST, 0, true)
    }
    if found.0.iter().any(|r| r.0 == "health" && r.1 >= 3)
        && !found
            .0
            .iter()
            .any(|r| r.1 >= 2 && !matches!(r.0, "health" | "small-talk"))
    {
        found.0.retain(|r| r.0 != "small-talk")
    }
    if let Some(inherited) = inherited {
        for row in &mut found.0 {
            if row.0 != inherited && row.2 < 2 {
                row.1 -= 1
            }
        }
        found.note(inherited, CARRY, 0, true);
    }
    if naked_season(norm, &phrases)
        && found.strong_besides("nature-and-weather", 2)
        && let Some(r) = found.0.iter_mut().find(|r| r.0 == "nature-and-weather")
    {
        r.1 = r.1.min(1)
    }
    let mut ranked: Vec<_> = found
        .0
        .into_iter()
        .filter(|r| r.1 >= 2)
        .map(|r| (r.0, r.1))
        .collect();
    ranked.sort_by_key(|&(topic, n)| {
        (
            std::cmp::Reverse(n),
            std::cmp::Reverse(Some(topic) == inherited),
        )
    });
    (ranked, subject)
}
pub(crate) fn unsure<'a>(norm: &'a [String], topic: Option<&str>) -> Vec<&'a str> {
    if topic.is_none() {
        return Vec::new();
    }
    let mut uncertain: Vec<_> = norm
        .iter()
        .map(String::as_str)
        .filter(|w| get(AMBIGUITIES, w).is_some_and(|cues| !any(norm, cues)))
        .collect();
    uncertain.sort_unstable();
    uncertain.dedup();
    uncertain
}
fn returns_the_question(norm: &[String]) -> bool {
    let len = norm.len();
    if len < 2 || !has(BACK_AT, &norm[len - 1]) {
        return false;
    }
    if len == 2 && has(BACK_JOIN, &norm[0])
        || len == 3 && has(BACK_ASKING, &norm[0]) && has(BACK_JOIN, &norm[1])
    {
        return true;
    }
    if len > 2 && has(BACK_NOT_A_SUBJECT, &norm[len - 3])
        || norm[..len - 1]
            .iter()
            .any(|w| has(BACK_AT, w) || matches!(w.as_str(), "your" | "youre"))
    {
        return false;
    }
    let join = &norm[len - 2];
    if join == "about" && !any(&norm[..len - 2], BACK_ASKING) {
        return false;
    }
    has(BACK_JOIN, join) || has(BACK_AFTER, join)
}
pub(super) fn is_condition(word: &str) -> bool {
    has(CONDITIONS, word)
}

pub(crate) fn output_position(norm: &[String]) -> Option<usize> {
    if !any(norm, BODILY_OUTPUT) || any(norm, SAMPLE_WORDS) || any(norm, ANIMALS) || any(norm, PETS)
    {
        return None;
    }
    if any(norm, ONLY_A_STOOL)
        && !norm
            .iter()
            .any(|w| has(BODILY_OUTPUT, w) && !has(ONLY_A_STOOL, w))
        && (any(norm, STOOL_AS_FURNITURE) || !A_GUT_ONLY.is_match(&norm.join(" ")).unwrap_or(false))
    {
        return None;
    }
    let person = norm.iter().any(|w| {
        has(HUMAN, w) && !has(ANIMALS, w)
            || matches!(w.as_str(), "i" | "im" | "ive" | "we" | "weve")
    });
    norm.iter().enumerate().find_map(|(at, w)| {
        (has(BODILY_OUTPUT, w)
            && if has(BODILY_OUTPUT_NOUN, w) {
                at > 0 && has(OWNERS, &norm[at - 1])
            } else {
                person
            })
        .then_some(at)
    })
}
fn unwell(norm: &[String]) -> bool {
    let warm = warmth(norm);
    let words: Vec<_> = norm
        .iter()
        .map(String::as_str)
        .filter(|w| Some(*w) != warm)
        .collect();
    if !words.iter().any(|w| has(THE_PEOPLE, w))
        || UNWELL_IDIOMS.iter().any(|p| words.join(" ").contains(p))
    {
        return false;
    }
    for (at, &w) in words.iter().enumerate() {
        let after = words.get(at + 1).copied().unwrap_or("");
        if has(UNWELL_SIGNS, w) {
            if has(BEHAVIOUR, after)
                || has(SIGN_NOT_AFTER, after)
                || has(NOUNY_SIGNS, w)
                    && !after.is_empty()
                    && !has(SIGN_FOLLOWS, after)
                    && !after.chars().all(char::is_numeric)
                || w == "broke" && !words.iter().any(|w| has(BODY, w))
            {
                continue;
            }
            return true;
        }
        if at > 0 && words[at - 1] == "not" && has(UNWELL_NOT, w) {
            if w == "responding" {
                for &next in words.iter().skip(at + 1).take(3) {
                    if has(COMMS, next) {
                        return false;
                    }
                    if has(HEARERS, next) {
                        return true;
                    }
                }
            }
            return true;
        }
    }
    false
}
pub(super) fn purse(norm: &[String], text: &str) -> Option<usize> {
    let wait = |words: &[&str]| {
        words.iter().take(3).enumerate().any(|(i, w)| {
            has(A_WAIT, w)
                && words
                    .iter()
                    .skip(i + 1)
                    .take(2)
                    .find(|&&s| !matches!(s, "the" | "a" | "i"))
                    .is_some_and(|s| has(A_WAIT_TO, s))
        })
    };
    let list: Vec<_> = norm.iter().map(String::as_str).collect();
    (1..list.len())
        .find_map(|i| {
            let w = list[i];
            if !has(AMOUNT_NOUN, w) && !has(A_MONEY_WORD, w) {
                return None;
            }
            if !has(AMOUNT_NOUN, w) {
                let previous = list[i - 1];
                if !has(NUMBER_WORDS, previous)
                    && !previous.trim_matches('.').chars().all(char::is_numeric)
                    && previous.parse::<f64>().is_err()
                {
                    return None;
                }
                let mut amount = previous
                    .parse::<f64>()
                    .ok()
                    .or_else(|| super::numbers::word_number(previous))
                    .unwrap_or(0.0);
                if i >= 2
                    && matches!(previous, "hundred" | "thousand")
                    && let Some(other) = super::numbers::word_number(list[i - 2])
                {
                    amount *= other
                }
                if amount >= 500. {
                    return None;
                }
            }
            wait(&list[i + 1..]).then_some(i - 1)
        })
        .or_else(|| {
            static WORD: std::sync::LazyLock<regex::Regex> =
                std::sync::LazyLock::new(|| regex::Regex::new("[a-z']+").unwrap());
            let lower = text.to_lowercase();
            A_AMOUNT_SYMBOL
                .captures_iter(&lower)
                .flatten()
                .find_map(|m| {
                    let val = if let Some(p) = m.get(1) {
                        p.as_str().replace(',', "").parse::<f64>().ok()?
                    } else {
                        m.get(2)?.as_str().parse::<f64>().ok()? / 100.
                    };
                    let tail: Vec<_> = WORD
                        .find_iter(&lower[m.get(0)?.end()..])
                        .map(|m| m.as_str())
                        .collect();
                    (val < 500. && wait(&tail)).then_some(0)
                })
        })
}
pub(super) fn tight_money(norm: &[String], text: &str) -> Option<usize> {
    let list: Vec<_> = norm.iter().map(String::as_str).collect();
    let mut pos = list.iter().enumerate().find_map(|(i, w)| {
        (*w == "broke"
            && list[..i].iter().all(|w| has(BROKE_AS_SUBJECT, w))
            && list[i + 1..].iter().all(|w| has(BROKE_AS_TIME, w)))
        .then_some(i)
    });
    let purse = purse(norm, text);
    if let Some(i) = purse {
        pos = Some(pos.map_or(i, |p| p.min(i)))
    }
    let joined = format!(" {} ", list.join(" "));
    for phrase in MONEY_TIGHT {
        if let Some(at) = joined.find(&format!(" {phrase} ")) {
            let i = joined[..at].split_whitespace().count();
            pos = Some(pos.map_or(i, |p| p.min(i)))
        }
    }
    pos.filter(|_| {
        !list
            .iter()
            .any(|w| lookup(w).any(|(t, n)| n >= 2 && !matches!(t, "money" | "small-talk")))
    })
}
fn stone(norm: &[String]) -> Option<usize> {
    if !norm
        .iter()
        .any(|w| matches!(w.as_str(), "stone" | "stones"))
    {
        return None;
    }
    let mut cues: Vec<_> = norm
        .iter()
        .enumerate()
        .filter_map(|(i, w)| has(STONE_VERBS, w).then_some(i))
        .collect();
    let text = norm.join(" ");
    for p in STONE_COMPOUNDS {
        cues.extend(
            text.match_indices(p)
                .map(|(i, _)| text[..i].split_whitespace().count()),
        );
    }
    norm.iter().enumerate().find_map(|(i, w)| {
        (matches!(w.as_str(), "stone" | "stones") && cues.iter().any(|p| i.abs_diff(*p) <= 4))
            .then_some(i)
    })
}
fn someone_wants(norm: &[String]) -> Option<&'static str> {
    for (at, w) in norm.iter().enumerate() {
        if !has(WANT_VERBS, w) || !any(&norm[at.saturating_sub(3)..at], A_PERSON_WHO_WANTS) {
            continue;
        }
        for pos in at + 1..(at + 5).min(norm.len()) {
            if has(WANTED_BEFORE_A_STOP, &norm[pos]) {
                break;
            }
            if has(A_THING_WANTED, &norm[pos]) {
                for candidate in norm
                    .iter()
                    .skip(pos + 1)
                    .take(4)
                    .filter(|w| !has(A_THING_WANTED, w))
                {
                    if let Some((topic, _)) = lookup(candidate)
                        .find(|(t, n)| *n >= 2 && *t != "small-talk" && !has(PEOPLE_TOPICS, t))
                    {
                        return Some(topic);
                    }
                }
                break;
            }
        }
    }
    None
}

/// Source words and phrase weights supporting one ranked topic.
pub(super) fn evidence(line: &Line, topic: Option<&str>) -> Vec<(String, i32)> {
    let Some(topic) = topic else {
        return Vec::new();
    };
    let norm = &line.norm;
    let mut covered = std::collections::HashSet::new();
    let mut found = Vec::new();
    for size in 2..=4 {
        for window in norm.windows(size) {
            if let Some(row) = lexeme(&window.join(" ")) {
                for &(name, weight) in row.weights {
                    if name == topic {
                        found.push((row.text.to_owned(), weight));
                        covered.extend(window.iter().map(String::as_str));
                    }
                }
            }
        }
    }
    for word in norm {
        if !covered.contains(word.as_str()) {
            found.extend(
                lookup(word)
                    .filter(|(name, _)| *name == topic)
                    .map(|(_, weight)| (word.clone(), weight)),
            );
        }
    }
    found.sort_by(|(a, wa), (b, wb)| wb.cmp(wa).then_with(|| a.cmp(b)));
    found
}
#[path = "subject_data.rs"]
mod subject_data;
#[derive(Default, Clone, Copy)]
pub(super) struct PriorCues {
    bank: bool,
    seal: bool,
}
impl PriorCues {
    pub(super) fn from_line(norm: &[String]) -> Self {
        Self {
            bank: any(norm, subject_data::BANK_CUES),
            seal: any(norm, subject_data::SEAL_CUES),
        }
    }
}
pub(super) fn unclear_subject(
    norm: &[String],
    inherited: Option<&str>,
    previous: PriorCues,
) -> Option<&'static str> {
    use subject_data::*;
    let mut content = norm
        .iter()
        .enumerate()
        .filter(|(_, word)| !has(NOT_THE_SUBJECT, word));
    let (at, word) = content.next()?;
    let (word, cues, topics, prior) = match word.as_str() {
        "bank" => ("bank", BANK_CUES, BANK_TOPICS, previous.bank),
        "banks" => ("banks", BANK_CUES, BANK_TOPICS, previous.bank),
        "seal" => ("seal", SEAL_CUES, SEAL_TOPICS, previous.seal),
        "seals" => ("seals", SEAL_CUES, SEAL_TOPICS, previous.seal),
        _ => return None,
    };
    if prior
        || has(THE_OWNER_SETTLES, word) && at > 0 && has(POSSESSIVE, &norm[at - 1])
        || inherited.is_some_and(|i| has(topics, i))
    {
        return None;
    }
    if !content.take(8).any(|(_, w)| has(A_STATE_VERB, w))
        || any(&norm[at.saturating_sub(99)..norm.len().min(at + 100)], cues)
    {
        return None;
    }
    Some(word)
}

#[cfg(test)]
mod evidence_tests {
    use super::*;
    mod cases {
        type EvidenceCase = (&'static str, &'static str, &'static [(&'static str, i32)]);
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/topic_evidence.rs"
        ));
    }
    #[test]
    fn preserves_order_and_repeated_topic_evidence() {
        for &(text, topic, expected) in cases::EVIDENCE {
            let line = super::super::line::normalise(text);
            let evidence = evidence(&line, Some(topic));
            assert!(
                evidence
                    .iter()
                    .map(|(w, n)| (w.as_str(), *n))
                    .eq(expected.iter().copied()),
                "{text:?} / {topic} {evidence:?} {expected:?}"
            );
        }
    }
    #[test]
    fn resolves_ambiguous_subjects_against_the_last_turn() {
        use std::collections::HashMap;
        let mut lines = HashMap::new();
        let mut cues = HashMap::new();
        for &(text, topic, previous, expected) in cases::AMBIGUITY {
            let line = lines
                .entry(text)
                .or_insert_with(|| super::super::line::normalise(text));
            let prior = cues.entry(previous).or_insert_with(|| {
                PriorCues::from_line(&super::super::line::normalise(previous).norm)
            });
            assert_eq!(
                unclear_subject(&line.norm, (!topic.is_empty()).then_some(topic), *prior),
                expected,
                "{text:?} after {previous:?} in {topic}"
            );
        }
    }
}
