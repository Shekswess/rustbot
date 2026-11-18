//! Context fences for the three guarded music knowledge cards in PyBot.
use crate::bot::Reading;

const MUSIC_CUES: &[&str] = &[
    "music",
    "practice",
    "practise",
    "practising",
    "practicing",
    "playing",
    "play",
    "guitar",
    "piano",
    "violin",
    "drums",
    "singing",
    "sing",
    "band",
    "instrument",
    "chords",
    "rehearsal",
    "rehearsing",
    "trumpet",
    "flute",
    "saxophone",
    "cello",
    "ukulele",
    "bass",
    "clarinet",
];
const INFECTION_CUES: &[&str] = &[
    "red",
    "swollen",
    "swelling",
    "pus",
    "infection",
    "infected",
    "whitlow",
    "abscess",
    "throbbing",
    "hot",
    "fever",
    "septic",
    "nail fold",
];
const MUSIC_AROUND_STAFF: &[&str] = &[
    "bar",
    "bars",
    "bass",
    "chord",
    "chords",
    "clef",
    "clefs",
    "dot",
    "dots",
    "instrument",
    "learn",
    "learning",
    "line",
    "lines",
    "music",
    "musical",
    "note",
    "notes",
    "octave",
    "piano",
    "read",
    "reading",
    "score",
    "sheet",
    "sight",
    "sing",
    "song",
    "space",
    "spaces",
    "stave",
    "theory",
    "treble",
    "violin",
];
const MUSIC_PHRASES: &[&str] = &[
    "middle c",
    "note lengths",
    "sheet music",
    "sight read",
    "treble clef",
    "bass clef",
];
const STAFF_MEANS_PEOPLE: &[&str] = &[
    "my staff",
    "our staff",
    "two staff",
    "three staff",
    "four staff",
    "staff who",
    "staff member",
    "staff members",
    "members of staff",
    "staff are",
    "staff is",
    "staff not",
    "staff dont",
    "staff won't",
    "staff wont",
    "staff keep",
    "staff always",
    "staff never",
    "staff argue",
    "staff meeting",
    "staff rota",
    "staff turnover",
    "short staffed",
    "understaffed",
    "of staff",
];
const STAFF_TAILS: &[&str] = &["", "ed", "er", "ers", "ing", "s"];

pub(super) fn making_music(reading: &Reading<'_>) -> bool {
    reading
        .line
        .clean
        .split_whitespace()
        .any(|word| MUSIC_CUES.contains(&word))
}

pub(super) fn not_an_infection(reading: &Reading<'_>) -> bool {
    !INFECTION_CUES
        .iter()
        .any(|word| reading.line.clean.contains(word))
}

pub(super) fn staff_is_music(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    let flat = clean.replace('\'', "");
    let says_staff = flat.split_whitespace().any(|word| {
        word.strip_prefix("staff")
            .is_some_and(|tail| STAFF_TAILS.contains(&tail))
    });
    !says_staff
        || MUSIC_PHRASES.iter().any(|phrase| clean.contains(phrase))
        || (!STAFF_MEANS_PEOPLE
            .iter()
            .any(|phrase| clean.contains(phrase))
            && flat
                .split_whitespace()
                .any(|word| MUSIC_AROUND_STAFF.contains(&word)))
}

#[cfg(test)]
mod tests {
    use super::*;
    type Guard = fn(&Reading<'_>) -> bool;

    #[test]
    fn music_context_matches_pinned_guard_decisions() {
        let cases: &[(Guard, &str, bool)] = &[
            (making_music, "my neighbours can hear me practising", true),
            (making_music, "neighbours can hear my guitar", true),
            (making_music, "can my neighbours hear my dog", false),
            (making_music, "my neighbours can hear my TV", false),
            (making_music, "can my neighbours hear the piano?", false),
            (making_music, "when I sing in the dark", true),
            (
                not_an_infection,
                "my fingers are sore after playing guitar",
                true,
            ),
            (not_an_infection, "my finger is red after guitar", false),
            (not_an_infection, "my fingertips are hot and swollen", false),
            (not_an_infection, "my fingertips are painful", true),
            (not_an_infection, "my fingertips throb", true),
            (
                not_an_infection,
                "playing guitar gave me another blister",
                true,
            ),
            (
                staff_is_music,
                "how do i improve at sight reading sheet music",
                true,
            ),
            (
                staff_is_music,
                "my team is short staffed and i keep covering",
                false,
            ),
            (staff_is_music, "how do i handle two staff who argue", false),
            (
                staff_is_music,
                "what are the five lines of a staff in music",
                true,
            ),
            (staff_is_music, "how do i read notes on the staff", true),
            (staff_is_music, "what is a staff?", true),
        ];
        for &(guard, line, expected) in cases {
            assert_eq!(guard(&Reading::new(line)), expected, "{line}");
        }
    }
}
