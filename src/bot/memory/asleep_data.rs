//! Source patterns and wording from bot/memory/asleep.py.
pattern!(
    SOMEONE_ELSE,
    r##"(?i)\b(?:my|our|her|his|their|the)\s+(?:son|daughter|baby|boy|girl|child|children|kid|kids|toddler|teenager|teen|mum|mom|mother|dad|father|partner|husband|wife|boyfriend|girlfriend|dog|cat|puppy|kitten|pet|flatmate|housemate|roommate)\b"##
);
pattern!(THEY_NOT_ME, r##"(?i)^\s*(?:he|she|they|it)\b"##);
pattern!(
    NIGHT_WAKING,
    r##"(?i)\b(?:keep|keeps|keeping|always|still|constantly|often)\s+wak(?:e|es|ing)\b|\bwak(?:e|es|ing)\s+up\s+(?:in the night|during the night|several times|a few times)\b|\b(?:in|during) the night\b|\bget back to sleep\b|\bcant sleep\b|\bcan't sleep\b|\binsomnia\b"##
);
pattern!(
    CARRIES_ON,
    r##"(?i)^\s*(?:and|also|then|but|plus|oh and|,\s*and)\b"##
);
pattern!(
    HAS_A_SUBJECT,
    r##"(?i)\b(?:i|we|you|he|she|they|it)\b|'m\b|\bim\b|\bmy\b|\bour\b"##
);
pattern!(
    I_WAKE,
    r##"(?i)^\s*(?:i|im|i'm|ive|i've|we|we're|were|weve|we've)\s+(?:usually\s+|normally\s+|often\s+|just\s+|then\s+|still\s+)?(?:wake|wakes|waking|woke|woken|waked|waken|get\s+up|gets\s+up|got\s+up|getting\s+up|get|got|am|up)\b"##
);
pattern!(
    BED_MARKER,
    r##"(?i)\b(?:go(?:es|ing)?\s+(?:off\s+)?to\s+(?:bed|sleep)|went\s+to\s+(?:bed|sleep)|to\s+bed|in\s+bed|bedtime|bed|asleep|sleep(?:s|ing)?|slept)\b"##
);
pattern!(
    WAKE_MARKER,
    r##"(?i)\b(?:wake|wakes|waking|woke)(?:\s+up)?\b|\b(?:get|gets|getting|got)\s+up\b|\b(?:am|are|be|im|i'm)\s+up\b|\bup\b(?=\s*(?:at|around|about|by)\b)"##
);
pattern!(
    ASKS_A_BEDTIME,
    r##"(?i)\b(?:what\s+time|when|what\s+hour|how\s+long\s+before|what\s+sort\s+of\s+time)\b[^.!?]{0,40}?\b(?:go|going|be|get|getting)\s+(?:off\s+)?to\s+(?:bed|sleep)\b|\b(?:go|going)\s+to\s+bed\s+(?:to|if|so\s+that|before)\b|\bshould\s+i\s+(?:go\s+to\s+bed|be\s+in\s+bed|be\s+going\s+to\s+bed)\b"##
);
pattern!(
    ATOM,
    r##"(?i)(?:(?:at|around|about|by|till|until)\s+)?(?P<atom>midnight|noon|half\s+(?P<half>one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)|(?P<clock>[01]?\d|2[0-3])(?:[:.](?P<minutes>[0-5]\d))?\s*(?P<when>am|pm|a\.m\.|p\.m\.)?|(?P<word>one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b)"##
);
pub(super) const SHORT: &[&str] = &[
    "{length} between bed and the alarm is under the seven to nine most adults need, so the end worth moving is the bedtime.",
    "{length} is short of the seven to nine most adults do best on. The going to bed is the end that gives, because the wake time is what sets the clock.",
];
pub(super) const FINE: &[&str] = &[
    "{length} is inside the seven to nine most adults need, so it is the quality of the night rather than the length that is worth looking at.",
    "{length} sits in the seven to nine range, so the length is not the problem - a warm room, a drink, or a phone in bed are what break a night that long.",
];
pub(super) const LONG: &[&str] = &[
    "{length} is more than the seven to nine most adults need, and sleeping past that often leaves a person groggy rather than rested.",
    "{length} is past the seven to nine range. A slightly earlier alarm usually leaves somebody more awake, not less.",
];
