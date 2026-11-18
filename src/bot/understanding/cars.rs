//! Car faults, their negation, and when the conversation switches to driving lessons.
use regex::Regex;
use std::sync::LazyLock;
static CAR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:car|cars|van|vans|mot|mot'd|garage|garages|mechanic|mechanics|tyre|tyres|tire|tires|engine|gearbox|clutch|brakes|brake|exhaust|radiator|battery|alternator|starter|windscreen|bonnet|suspension|shock|absorber|wheel|wheels|hub|bearing|bearing's|jump\s?start|service|serviced|motoring|audi|ford|vauxhall|toyota|honda|nissan|renault|peugeot|citroen|skoda|hyundai|kia|mazda|volvo|suzuki|bmw|mercedes|volkswagen|vw|fiat|tesla|dacia|chevrolet|chrysler|dodge|subaru|mitsubishi|lexus|saab|alfa|jeep|bentley|porsche|ferrari|lamborghini|aston|lada|daewoo|infiniti|acura|punto|corsa|clio|astra|mondeo|passat|micra|yaris|fabia|octavia|ibiza|qashqai|juke|captur|kadjar|sandero|duster|sprinter|picanto|zafira|insignia|vectra|laguna|megane|berlingo|picasso|cupra|leon|toledo|kamiq|karoq|kodiaq|tucson|sportage|camaro|impreza|forester|hilux|auris|avensis|prius|celica|supra|aygo|navara)\b"#).unwrap()
});
static FAULT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:noise|noises|noisy|rattle|rattles|rattling|click|clicking|clicks|knock|knocking|grinding|squeal|squealing|squeak|squeaking|whining|humming|judder|juddering|wont start|won't start|wouldnt start|wouldn't start|not starting|cutting out|cuts out|stalls|stalled|warning light|engine light|light on|leak|leaks|leaking|leaked|smoke|smoking|overheating|overheated|misfire|misfiring|broken|broke|faulty|gone wrong|failed|failing|fail|puncture|punctured|flat|bald|locked up|seized|clunk|clunking|vibration|vibrating|pulling|sticking|jammed|rust|rusted|corroded|seized up|garage (?:says|said|thinks|reckons|quoted|quotes|wants|has had)|they quoted|needs? (?:a )?(?:new|replacing)|worn|knackered|shot)\b"#).unwrap()
});
static LEARNING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:lesson|lessons|learner|learners|instructor|instructors|provisional|theory|driving test|test booked|passed|pass|learn|learning|beginners?|tuition)\b"#).unwrap()
});
static CLAUSE_BREAK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:but|and|though|although|while|whereas|so|because|then|except)\b|[;,.!?]"#)
        .unwrap()
});
static NOT_A_FAULT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:not|no|never|isnt|isn't|arent|wasnt|wasn't|hasnt|hasn't|havent|haven't|dont|don't|cant|can't|fine|ok|okay|sorted|fixed|repaired|stopped|gone|better)\b"#).unwrap()
});
static NOT_THE_MACHINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:bike|cycle|bicycle|motorbike|motorcycle|scooter|mower|lawnmower|tractor|lorry|bus|train|plane|caravan|trailer)s?\b"#).unwrap()
});

fn before(text: &str, at: usize, chars: usize) -> &str {
    let prefix = &text[..at];
    let start = prefix
        .char_indices()
        .rev()
        .nth(chars.saturating_sub(1))
        .map_or(0, |(start, _)| start);
    &prefix[start..]
}

fn broken_car(text: &str) -> bool {
    CLAUSE_BREAK.split(text).any(|clause| {
        CAR.find_iter(clause)
            .any(|part| !NOT_THE_MACHINE.is_match(before(clause, part.start(), 20)))
            && FAULT.find_iter(clause).any(|fault| {
                if ["not ", "wont", "won't", "wouldnt", "wouldn't"]
                    .iter()
                    .any(|w| fault.as_str().starts_with(w))
                {
                    return true;
                }
                let after = &clause[fault.end()..];
                let end = after
                    .char_indices()
                    .nth(24)
                    .map_or(after.len(), |(at, _)| at);
                !NOT_A_FAULT.is_match(before(clause, fault.start(), 24))
                    && !NOT_A_FAULT.is_match(&after[..end])
            })
    })
}

pub(crate) fn is_subject<'a>(
    text: &str,
    earlier: impl DoubleEndedIterator<Item = &'a str>,
) -> bool {
    let lower = text.to_lowercase();
    !LEARNING.is_match(&lower)
        && (broken_car(&lower)
            || earlier
                .rev()
                .take(3)
                .any(|line| broken_car(&line.to_lowercase())))
}
