//! Native port of bot/compose/a_first_time.py.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;
static A_FIRST_TIME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:first time|first one|first ever|first go|first race|first shift|first day|never done (?:one|this|it|that) before|never done (?:one|this|it|that)|never (?:done|run|given|taught|led|organised|organized|played|performed)\b[^?]*\bbefore|never had one|havent done (?:one|this|it) before|haven't done (?:one|this|it) before|(?:its|it's|this is) my first|my first time|ive never|i've never|not done one before)\b"#).unwrap()
});
static A_FIRST_FENCED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:died|death|dying|dead|loss|lost|grief|bereave\w*|funeral|miscarriage|stillborn|cancer|hospice|hospital|operation|surgery|diagnos\w*|chemo|baby|babies|pregnan\w*|birth|numb|passed away|put to sleep|euthanis\w*)\b"#).unwrap()
});
static A_A_TASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:talk|speech|presentation|pitch|demo|interview|viva|defence|defense|panel|race|marathon|half marathon|10k|5k|parkrun|triathlon|swim|match|game|final|heat|test|exam|driving test|theory test|practical|audition|performance|gig|show|shift|first day|day one|lesson|class|course|lecture|seminar|reading|sermon|wedding speech|best man|maid of honour|toast|eulogy|chair|chairing|meeting|appointment|scan|procedure|surgery|flight|driving lesson|lesson)\b"#).unwrap()
});
static A_FIRST_NAMES_ITS_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:had|eaten|tried|cooked|made|bought|seen|visited|flown|driven|used|caught|owned|worn|grown|sold|wrote|read|played|kept|baked|brewed)\s+(?:a|an|the|any|some)\s+\w+"#).unwrap()
});
const FIRST_TIME: &[&str] = &[
    "The first one is the one you over-prepare for, and that is the right mistake to make. Say it out loud once, all the way through, to a person or the wall - that is the step people skip and the one that takes the nerves down. What is the part that worries you most?",
    "Nobody is calm for their first one, and the nerves go in the first two minutes once you are actually in it. Walk it through out loud once beforehand rather than reading it over again in your head. What is it you are facing?",
    "Everybody's first is the worst one, and almost nobody does it as badly as they think. Practise it once, out loud and timed, and know that the beginning is the part to have ready. What would go wrong if it went wrong?",
];

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let low = reading.folded();
    if low.ends_with('?') || !A_FIRST_TIME.is_match(low) || A_FIRST_FENCED.is_match(low) {
        return None;
    }
    let task = A_A_TASK.is_match(low)
        || (!A_FIRST_NAMES_ITS_THING.is_match(low)
            && memory
                .turns
                .iter()
                .rev()
                .take(4)
                .any(|turn| A_A_TASK.is_match(&turn.user)));
    task.then(|| super::choose_recent(FIRST_TIME, memory, low).to_owned())
}
