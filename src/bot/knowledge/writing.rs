//! Authored writing forms and their follow-ups from PyBot's entertainment rows.
#[path = "writing_data.rs"]
mod data;
#[path = "writing_names.rs"]
mod names;
#[cfg(test)]
#[path = "writing_tests.rs"]
mod tests;

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Form {
    Haiku,
    Limerick,
    Poem,
    Rhyme,
    Story,
    Rap,
    Song,
    Another,
    StoryIdea,
    CharacterName,
    Nickname,
    QuizName,
}

impl Form {
    fn name(self) -> &'static str {
        match self {
            Self::Haiku => "haiku",
            Self::Limerick => "limerick",
            Self::Poem => "poem",
            Self::Rhyme => "rhyme",
            Self::Story => "story",
            Self::Rap => "rap",
            Self::Song => "song",
            Self::Another => "another",
            Self::StoryIdea => "story idea",
            Self::CharacterName => "character name",
            Self::Nickname => "nickname",
            Self::QuizName => "quiz name",
        }
    }

    fn read(name: &str) -> Option<Self> {
        [
            Self::Haiku,
            Self::Limerick,
            Self::Poem,
            Self::Rhyme,
            Self::Story,
            Self::Rap,
            Self::Song,
        ]
        .into_iter()
        .find(|item| item.name().eq_ignore_ascii_case(name))
    }

    fn table(self) -> &'static [(&'static str, &'static [&'static str])] {
        match self {
            Self::Haiku => data::HAIKU,
            Self::Limerick => data::LIMERICKS,
            Self::Poem => data::POEMS,
            _ => &[],
        }
    }
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    A_VERSE_LEAD,
    A_LISTENER_IS_NAMED,
    ASKS_ABOUT_A_STORY,
    ASKS_FOR_A_STORY,
    A_STORY_ALREADY_TOLD,
    A_NEW_ONE_ASKED_FOR,
    ASKS_THE_BOT,
    A_STORY_ALREADY_HEARD,
    SECOND_STORY,
    A_STORY_WORD,
    REPORTS_ON_A_FORM,
    ASKS_FOR_ONE,
    A_REPEATED_REPORT,
    A_READING_ROUTINE,
    A_MAKING_UP,
    A_QUESTION_BESIDE_THE_REPORT,
    A_SECOND_ONE,
    NOT_A_VERSE_ASK,
);

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:about|of|on|regarding|describing|for|named|called)\s+(.+)$").unwrap()
});
static CHILD_SUBJECT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:about|of|on|regarding)\s+(\S.*)$").unwrap());
static AGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(\d+|one|two|three|four|five|six|seven|eight|nine|ten)\b").unwrap()
});
static HOW_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:how do i|how to|how can i|how should i|tips for|advice on|writes|wrote|writing|written|reads|reading)\b").unwrap()
});
static RHYMES_WITH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"rhyme\w*\s+(?:with|for|to)\s+([a-z']+)").unwrap());
static BRIGHTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"happ|bright|cheer|funny|nicer|lighter|lift").unwrap());
static QUIETER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"calm|quiet|sooth|gentl|soft|slower|easier").unwrap());
static REPORT_FORM: LazyLock<Vec<(&str, Regex)>> = LazyLock::new(|| {
    data::FORM_REPORT_SHAPES
        .iter()
        .map(|(name, pattern)| (*name, Regex::new(pattern).unwrap()))
        .collect()
});
static ENDINGS: LazyLock<Vec<(&str, &[&str])>> = LazyLock::new(|| {
    let mut endings = data::RHYMES_BY_ENDING.to_vec();
    endings.sort_by_key(|(ending, _)| std::cmp::Reverse(ending.len()));
    endings
});

fn get<T: Copy>(table: &[(&str, T)], key: &str) -> Option<T> {
    table
        .iter()
        .find_map(|(name, value)| (*name == key).then_some(*value))
}

fn key(text: &str, count: usize) -> usize {
    text.chars().map(|letter| letter as usize).sum::<usize>() % count
}

fn lines(lead: &str, body: &[&str]) -> String {
    format!("{lead}\n{}", body.join("\n"))
}

fn first_upper(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

fn subject_words(text: &str) -> Vec<&str> {
    let mut words: Vec<_> = WORD
        .find_iter(text)
        .map(|found| found.as_str())
        .take_while(|word| !data::SUBJECT_END.contains(word))
        .enumerate()
        .take_while(|(i, word)| *i == 0 || !data::CONNECTORS.contains(word))
        .take(3)
        .map(|(_, word)| word)
        .collect();
    while words
        .last()
        .is_some_and(|last| data::SUBJECT_TAIL.contains(last) || data::CONNECTORS.contains(last))
    {
        words.pop();
    }
    if let Some((last, other)) = words.split_last()
        && last.len() > 4
        && last.ends_with("ing")
        && !data::ING_NOUNS.contains(last)
        && other
            .iter()
            .any(|word| !data::SUBJECT_STOP.contains(word) && !data::EMPTY_SUBJECT.contains(word))
    {
        words.pop();
    }
    words
}

fn named(clean: &str) -> Vec<&str> {
    SUBJECT
        .captures(clean)
        .and_then(|captures| captures.get(1))
        .map_or_else(Vec::new, |tail| subject_words(tail.as_str()))
}

fn subject(words: &[&str]) -> String {
    if words.iter().all(|word| data::EMPTY_SUBJECT.contains(word)) {
        String::new()
    } else {
        words.join(" ")
    }
}

fn thing(words: &[&str]) -> String {
    let Some((&last, rest)) = words.split_last() else {
        return String::new();
    };
    let bare = |word: &str| word.strip_suffix("'s").unwrap_or(word).to_owned();
    let mut noun = bare(last);
    for word in rest.iter().rev() {
        if !data::SUBJECT_STOP.contains(&noun.as_str())
            && !data::CONNECTORS.contains(&noun.as_str())
        {
            break;
        }
        noun = bare(word);
    }
    if noun.len() > 4 && noun.ends_with("ies") {
        noun.truncate(noun.len() - 3);
        noun.push('y');
    } else if noun.len() > 3
        && noun.ends_with('s')
        && !["ss", "is", "us"].iter().any(|end| noun.ends_with(end))
        && !data::S_FINAL_SINGULARS.contains(&noun.as_str())
    {
        noun.pop();
    }
    noun
}

fn offered(form: Form, subject: &str) -> String {
    let name = if form == Form::Rhyme {
        "poem"
    } else {
        form.name()
    };
    format!(
        "A {name} about {subject} needs one real thing in it: give me a sound, a smell or a day from it and I'll build the {name} round that."
    )
}

fn verse(form: Form, noun: &str, subject: &str) -> Option<String> {
    let authored = get(form.table(), noun);
    let known_poem = get(data::POEMS, noun);
    let person = data::PEOPLE.contains(&noun);
    let last = subject.split_whitespace().next_back();
    if authored.is_none()
        && known_poem.is_none()
        && !person
        && (data::ABSTRACT.contains(&noun)
            || last.is_some_and(|word| data::ABSTRACT.contains(&word)))
    {
        return Some(offered(form, subject));
    }
    let lead = match form {
        Form::Haiku => format!("A haiku about {subject}, 5-7-5:"),
        Form::Limerick => format!("A limerick about {subject}:"),
        Form::Poem => format!("A short poem about {subject}:"),
        _ => format!("A rhyme about {subject}:"),
    };
    if form == Form::Haiku {
        return authored.map(|verses| lines(&lead, verses));
    }
    if let Some(verses) = authored.or_else(|| {
        (!matches!(form, Form::Poem | Form::Limerick))
            .then_some(known_poem)
            .flatten()
    }) {
        return Some(lines(&lead, verses));
    }
    if form != Form::Limerick && person {
        return Some(lines(&lead, data::WARM_POEM));
    }
    let template = if form == Form::Limerick {
        data::GENERIC_LIMERICK
    } else {
        data::GENERIC_POEM
    };
    Some(format!(
        "{lead}\n{}",
        template.join("\n").replace("{thing}", noun)
    ))
}

fn poems_without_subject(form: Form, clean: &str, words: &[&str], rhyme: bool) -> Option<String> {
    let forms = get(data::VERSE_FORM_WORDS, form.name()).unwrap_or_default();
    if HOW_TO.is_match(clean)
        || WORD.find_iter(clean).map(|word| word.as_str()).any(|word| {
            !forms.contains(&word)
                && (data::OTHER_FORM_WORDS.contains(&word)
                    || data::VERSE_FORM_WORDS
                        .iter()
                        .any(|(name, terms)| *name != form.name() && terms.contains(&word)))
        })
        || (!words.is_empty() && !(words.len() == 1 && data::WHOM_IT_IS_FOR.contains(&words[0])))
    {
        return None;
    }
    let lead = match form {
        Form::Haiku => "A haiku, then:",
        Form::Limerick => "A limerick, then:",
        _ if rhyme => "A rhyme, then:",
        _ => "A short poem, then:",
    };
    Some(lines(lead, get(data::NO_SUBJECT_VERSE, form.name())?))
}

fn rhyme(clean: &str) -> Option<String> {
    let found = RHYMES_WITH.captures(clean)?;
    let word = &found[1];
    if let Some(exception) = get(data::NO_RHYME, word) {
        return Some(exception.into());
    }
    let bare = word
        .strip_suffix('e')
        .filter(|_| word.len() > 3)
        .unwrap_or(word);
    for &(end, pool) in ENDINGS.iter() {
        if !word.ends_with(end) && !bare.ends_with(end) {
            continue;
        }
        let picks: Vec<_> = pool
            .iter()
            .copied()
            .filter(|pick| *pick != word)
            .take(5)
            .collect();
        if let Some((last, start)) = picks.split_last() {
            let list = if start.is_empty() {
                (*last).to_owned()
            } else {
                format!("{} and {last}", start.join(", "))
            };
            return Some(format!(
                "Words that rhyme with *{word}*: {list}. For a poem, the easiest thing is to put *{word}* at the end of a line and pick the rhyme that says what you mean."
            ));
        }
    }
    None
}

fn reports(clean: &str) -> bool {
    REPORTS_ON_A_FORM.is_match(clean) && !ASKS_FOR_ONE.is_match(clean)
}

fn report(clean: &str) -> Option<String> {
    if A_QUESTION_BESIDE_THE_REPORT.is_match(clean)
        || (A_READING_ROUTINE.is_match(clean) && !A_MAKING_UP.is_match(clean))
    {
        return None;
    }
    let form = REPORT_FORM
        .iter()
        .find_map(|(name, regex)| regex.is_match(clean).then_some(*name))
        .unwrap_or("story");
    if !A_REPEATED_REPORT.is_match(clean) {
        if let Some(pool) = get(data::REPORT_ONCE_BACK, form) {
            return Some(pool[key(clean, pool.len())].into());
        }
        let pool = data::REPORT_ONCE_GENERIC;
        return Some(pool[key(clean, pool.len())].replace("%s", form));
    }
    let pool = get(data::REPORT_BACK, form)?;
    Some(pool[key(clean, pool.len())].into())
}

fn synthetic(form: Form, topic: Option<&str>) -> String {
    topic.map_or_else(
        || format!("a {}", form.name()),
        |topic| format!("a {} about {topic}", form.name()),
    )
}

fn an_extra(reading: &Reading<'_>) -> Option<Option<String>> {
    [reading.raw, &reading.line.plain, &reading.line.clean]
        .into_iter()
        .find_map(|line| {
            let flat = line.split_whitespace().collect::<Vec<_>>().join(" ");
            if NOT_A_VERSE_ASK.is_match(&flat) {
                return None;
            }
            let found = A_SECOND_ONE.captures(&flat)?;
            let topic = found
                .get(1)
                .map(|group| group.as_str().trim_matches([' ', '.', ',', '!', '?']))
                .filter(|topic| !topic.is_empty())
                .map(str::to_owned);
            Some(topic)
        })
}

fn last_form(earlier: &[Exchange]) -> Option<Form> {
    for turn in earlier.iter().rev().take(2) {
        if let Some(found) = A_VERSE_LEAD.captures(&turn.bot)
            && let Some(form) = Form::read(&found[1])
        {
            return Some(form);
        }
        let said = format!("{} {}", turn.user.to_lowercase(), turn.bot.to_lowercase());
        if let Some(form) = [Form::Haiku, Form::Limerick, Form::Poem]
            .into_iter()
            .find(|form| said.contains(form.name()))
        {
            return Some(form);
        }
    }
    None
}

fn placeholder(subject: &str) -> bool {
    subject.split_whitespace().next().is_none_or(|head| {
        data::PLACEHOLDER_SUBJECT
            .contains(&head.trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"']))
    })
}

fn render(template: &str, noun: &str, clean: &str) -> String {
    let said = clean.to_lowercase();
    let ownership = data::POSSESSIVE.iter().find_map(|&owner| {
        said.match_indices(owner).find_map(|(at, word)| {
            let left = &said[at + word.len()..];
            let end = left.find(noun)?;
            (left[..end].split_whitespace().count() <= 2).then(|| {
                said[at..at + word.len() + end + noun.len()]
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
        })
    });
    let a = ownership.clone().unwrap_or_else(|| format!("a {noun}"));
    let the = ownership.unwrap_or_else(|| format!("the {noun}"));
    template
        .replace("{a_thing}", &a)
        .replace("{the_thing}", &the)
        .replace("{The_thing}", &first_upper(&the))
        .replace("{thing}", noun)
}

fn bedtime(clean: &str) -> String {
    let listener = AGE.captures(clean).map_or_else(
        || "A short one for a child".to_owned(),
        |age| format!("A short one for a {} year old", &age[1]),
    );
    if let Some(found) = CHILD_SUBJECT.captures(clean)
        && !placeholder(&found[1])
    {
        let noun = thing(&named(&found[0]));
        if !noun.is_empty() && !placeholder(&noun) {
            let pool = data::CHILD_STORY_OPENINGS;
            let opening =
                pool[key(&format!("{clean}{noun}"), pool.len())].replace("{thing}", &noun);
            return format!("{listener}: {opening}");
        }
    }
    let pool = data::CHILD_STORIES;
    format!("{listener}: {}", pool[key(clean, pool.len())])
}

fn story(clean: &str, earlier: &[Exchange], words: &[&str]) -> Option<String> {
    if earlier
        .iter()
        .rev()
        .find(|turn| !turn.user.is_empty() && !turn.bot.is_empty())
        .is_some_and(|turn| reports(&turn.user))
        && !ASKS_FOR_ONE.is_match(clean)
        && !SECOND_STORY.is_match(clean.trim())
    {
        return None;
    }
    if !A_STORY_WORD.is_match(clean)
        && (!SECOND_STORY.is_match(clean.trim())
            || !earlier
                .iter()
                .rev()
                .find(|turn| !turn.bot.is_empty())
                .is_some_and(|turn| A_STORY_WORD.is_match(&turn.bot)))
    {
        return None;
    }
    if ASKS_ABOUT_A_STORY.is_match(clean) || !ASKS_FOR_A_STORY.is_match(clean) {
        return None;
    }
    if A_STORY_ALREADY_HEARD.is_match(clean) {
        return Some(data::HEARD_IT_ALREADY[key(clean, data::HEARD_IT_ALREADY.len())].into());
    }
    if !ASKS_THE_BOT.is_match(clean)
        && !A_NEW_ONE_ASKED_FOR.is_match(clean)
        && A_STORY_ALREADY_TOLD.is_match(clean)
    {
        return None;
    }
    if A_LISTENER_IS_NAMED.is_match(clean) {
        return Some(bedtime(clean));
    }
    let named = thing(words);
    let noun = if placeholder(&named) {
        data::GENERIC_STORY_THING
    } else {
        &named
    };
    let subject = subject(words);
    let lead = if placeholder(&subject) {
        get(data::NO_SUBJECT_LEADING, "story")?.to_owned()
    } else if data::ABSTRACT.contains(&noun) {
        return Some(offered(Form::Story, &subject));
    } else {
        get(data::STORY_LEADING, "story")?.replace("{subject}", &subject)
    };
    let pool = data::STORY_OPENINGS;
    let start = key(noun, pool.len());
    let opening = (0..pool.len())
        .map(|step| render(pool[(start + step) % pool.len()], noun, clean))
        .find(|opening| {
            !earlier
                .iter()
                .any(|turn| turn.bot.contains(opening.lines().next().unwrap_or("")))
        })
        .unwrap_or_else(|| render(pool[start], noun, clean));
    Some(format!("{lead}\n{opening}"))
}

fn music(form: Form, clean: &str, words: &[&str]) -> Option<String> {
    if !words.is_empty() && words.iter().all(|word| data::EMPTY_SUBJECT.contains(word)) {
        return None;
    }
    let name = form.name();
    let named = thing(words);
    let noun = if named.is_empty() {
        get(data::NO_THING_IN_A_VERSE, name)?
    } else {
        &named
    };
    let subject = subject(words);
    if data::ABSTRACT.contains(&noun) {
        return Some(offered(
            form,
            if subject.is_empty() { noun } else { &subject },
        ));
    }
    let lead = if subject.is_empty() {
        get(data::NO_SUBJECT_LEADING, name)?.to_owned()
    } else {
        get(data::STORY_LEADING, name)?.replace("{subject}", &subject)
    };
    let body = if form == Form::Rap {
        render(&data::GENERIC_RAP.join("\n"), noun, clean)
    } else {
        render(data::GENERIC_SONG, noun, clean)
    };
    Some(format!("{lead}\n{body}"))
}

fn core(form: Form, clean: &str, earlier: &[Exchange]) -> Option<String> {
    if form == Form::Rhyme {
        return rhyme(clean);
    }
    if reports(clean) {
        return report(clean);
    }
    let words = named(clean);
    let mut reply = match form {
        Form::Haiku | Form::Limerick | Form::Poem => {
            let subject = subject(&words);
            let rhymed = form == Form::Poem && clean.contains("rhyme");
            if subject.is_empty() {
                poems_without_subject(form, clean, &words, rhymed)
            } else {
                verse(
                    if rhymed { Form::Rhyme } else { form },
                    &thing(&words),
                    &subject,
                )
            }
        }
        Form::Story => story(clean, earlier, &words),
        Form::Rap | Form::Song => music(form, clean, &words),
        _ => None,
    }?;
    if data::OFFERED.contains(&thing(&words).as_str())
        || words
            .last()
            .is_some_and(|word| data::OFFERED.contains(word))
    {
        reply.push_str(&data::OFFER_TAIL.replace("{form}", form.name()));
    }
    Some(reply)
}

fn another(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<String> {
    let form = last_form(earlier)?;
    if let Some(topic) = an_extra(reading)
        && let Some(out) = core(form, &synthetic(form, topic.as_deref()), earlier)
    {
        return Some(out);
    }
    let clean = &reading.line.clean;
    let words = named(clean);
    let noun = thing(&words);
    if !noun.is_empty() {
        return verse(form, &noun, &subject(&words));
    }
    let brighter = BRIGHTER.is_match(clean);
    if !brighter && !QUIETER.is_match(clean) {
        return None;
    }
    let subjects = if brighter { data::BRIGHT } else { data::CALM };
    let phrase = if brighter {
        "Something brighter, then."
    } else {
        "Something quieter, then."
    };
    let verses = subjects.iter().find_map(|noun| get(form.table(), noun))?;
    Some(lines(phrase, verses))
}

pub(super) fn reply(
    form: Form,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    if matches!(
        form,
        Form::StoryIdea | Form::CharacterName | Form::Nickname | Form::QuizName
    ) {
        return names::reply(form, &reading.line.clean).map(Cow::Owned);
    }
    if form == Form::Another {
        return another(reading, earlier).map(Cow::Owned);
    }
    let clean = &reading.line.clean;
    if let Some(topic) = an_extra(reading) {
        let chosen = last_form(earlier).unwrap_or(form);
        let answer = core(chosen, &synthetic(chosen, topic.as_deref()), earlier);
        if answer.is_some() || chosen != form || topic.is_none() {
            return answer.map(Cow::Owned);
        }
    }
    if !reading.attempts.asks_for_one_more
        && get(data::THE_FORM_NAMED, form.name()).is_some_and(|forms| {
            reading
                .attempts
                .tried
                .iter()
                .any(|word| forms.contains(&word.as_str()))
        })
        && form != Form::Rhyme
    {
        return None;
    }
    core(form, clean, earlier).map(Cow::Owned)
}
