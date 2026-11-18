// Patterns, vocabularies and pools from the pinned setback reader.

pattern!(
    OWNER,
    r#"\b(my|our|the|this|that|his|her|their)\s+(?:whole\s+|entire\s+)?([a-z][a-z'-]{1,18})(?:\s+([a-z][a-z'-]{1,18}))?"#
);
pub(super) const MODIFIER: &[&str] = &[
    "back",
    "big",
    "birthday",
    "cheap",
    "christmas",
    "driving",
    "expensive",
    "favorite",
    "favourite",
    "first",
    "front",
    "huge",
    "last",
    "little",
    "new",
    "next",
    "old",
    "other",
    "own",
    "previous",
    "running",
    "second",
    "spare",
    "third",
    "washing",
];
pub(super) const JOINERS: &[&str] = &[
    "a",
    "about",
    "absolutely",
    "after",
    "again",
    "all",
    "am",
    "an",
    "and",
    "are",
    "as",
    "at",
    "be",
    "because",
    "been",
    "before",
    "being",
    "but",
    "by",
    "came",
    "comes",
    "completely",
    "for",
    "from",
    "goes",
    "going",
    "gone",
    "had",
    "has",
    "have",
    "in",
    "into",
    "is",
    "just",
    "keeps",
    "kept",
    "last",
    "made",
    "makes",
    "next",
    "now",
    "of",
    "on",
    "onto",
    "or",
    "properly",
    "really",
    "since",
    "so",
    "still",
    "that",
    "the",
    "then",
    "this",
    "to",
    "today",
    "too",
    "totally",
    "turned",
    "turns",
    "was",
    "went",
    "were",
    "when",
    "which",
    "while",
    "who",
    "with",
    "yesterday",
];
pub(super) const NOT_A_THING: &[&str] = &[
    "afternoon",
    "all",
    "anything",
    "begin",
    "bit",
    "day",
    "end",
    "evening",
    "everything",
    "hour",
    "it",
    "life",
    "lot",
    "minute",
    "month",
    "morning",
    "night",
    "start",
    "thing",
    "time",
    "way",
    "week",
    "weekend",
    "whole",
    "world",
    "year",
];
pub(super) const YOURS: &[&str] = &["my", "our"];
pub(super) const PEOPLE: &[&str] = &[
    "aunt",
    "baby",
    "boss",
    "boy",
    "boyfriend",
    "brother",
    "carer",
    "cat",
    "child",
    "colleague",
    "cousin",
    "dad",
    "daddy",
    "daughter",
    "dog",
    "father",
    "fiance",
    "fiancee",
    "flatmate",
    "friend",
    "girl",
    "girlfriend",
    "gran",
    "grandad",
    "grandma",
    "grandpa",
    "grandparents",
    "granny",
    "he",
    "her",
    "hers",
    "him",
    "his",
    "housemate",
    "husband",
    "kid",
    "kids",
    "kitten",
    "landlady",
    "landlord",
    "mam",
    "manager",
    "mate",
    "mom",
    "mother",
    "mum",
    "mummy",
    "nan",
    "nana",
    "neighbor",
    "neighbour",
    "nephew",
    "niece",
    "nurse",
    "parent",
    "partner",
    "pet",
    "puppy",
    "she",
    "sibling",
    "sister",
    "son",
    "teenager",
    "tenant",
    "their",
    "them",
    "they",
    "toddler",
    "uncle",
    "vet",
    "wife",
];
pattern!(
    VAGUE,
    r#"(?x)\b(?:
        went\s+wrong | gone\s+wrong
      | (?:was|were|has\s+been|have\s+been|turned\s+out)\s+(?:a\s+|an\s+)?
        (?:(?:bit\s+of\s+a|total|complete|real|proper|right|absolute|utter|huge|massive|
             big)\s+)?
        (?:disaster|catastrophe|flop|washout|shambles|nightmare)
      | went\s+badly | went\s+pear[\s-]?shaped | came\s+out\s+wrong
    )\b"#
);
pattern!(
    NOW,
    r#"(?x)\b(?:is|are|'s|'re)\s+(?:really\s+|properly\s+|still\s+|just\s+)?
    (?:
        killing\s+me | doing\s+my\s+(?:head|nut)\s+in
      | driving\s+me\s+(?:mad|crazy|nuts|insane|up\s+the\s+wall|round\s+the\s+bend)
    )
  | \b(?:is|are|'s|'re)\s+(?:really\s+|still\s+|all\s+|just\s+)?going\s+wrong\b
  # The same thing said as a gradual decline rather than a fault: "the training is going
  # badly", "the lessons are not going well", "its going badly". Read as a fact to file, each of
  # these was answered "Got it - the training is going badly", which is the coldest reply in
  # STYLE.md on a line that has just handed over something going wrong.
  | \b(?:isn['\u2019]?t|aren['\u2019]?t|is\s+not|are\s+not|not)
    \s+going\s+(?:well|great|smoothly|to\s+plan|right)\b
  | \b(?:is|are|'s|'re|was|were)\s+(?:really\s+|still\s+|all\s+|just\s+)*
    going\s+(?:badly|awfully|terribly|wrong)\b
  | \bits\s+(?:all\s+|really\s+|still\s+|just\s+)?
    (?:not\s+going\s+(?:well|great|smoothly|to\s+plan|right)|going\s+(?:badly|awfully|terribly|wrong))\b
  | \b(?:is|are|'s|'re)\s+(?:a\s+|an\s+)?
    (?:(?:bit\s+of\s+a|total|complete|real|proper|right|absolute|utter|huge|massive|big)\s+)?
    (?:disaster|catastrophe|nightmare|shambles)\b
  # A thing of theirs wearing them down over time: "work has been getting me down lately",
  # "the commute is getting me down", "my job gets me down". The auxiliary is optional because
  # people leave it out ("work gets me down"), and "has been"/"have been" is how the same
  # thing is said when it has gone on a while. A person is not a thing here - "my boss gets me
  # down" names somebody, and `_refused` leaves those to bot/compose/harm.py.
  | \b(?:(?:is|are|was|were|'s|'re)\s+)?(?:(?:has|have|had)\s+been\s+|been\s+)?
    (?:getting|gets|got|wearing|wears|wore)\s+(?:me|us)\s+down\b
"#
);
pattern!(
    BLAME,
    r#"(?x)\b(?:i|we)\s+(?:really\s+|totally\s+|completely\s+|properly\s+|just\s+)*
    (?:messed|screwed|balls(?:ed)?|buggered|wrecked)\s+(?:it\s+)?up\b
    |\b(?:i|we)\s+(?:really\s+|totally\s+|completely\s+)?(?:ruined|wrecked)\b"#
);
pattern!(
    HELPLESS,
    r#"(?x)\s*(?:,|\band\b|\bbut\b|\bso\b)\s*
    (?:\w+\s+){0,3}?
    (?:i\s+)?(?:don['\u2019]?t|do\s+not|dont|can['\u2019]?t|cannot|cant)\s+
    (?:really\s+|even\s+|honestly\s+)?
    (?:know\s+what\s+to\s+do|know\s+where\s+to\s+start|cope|keep\s+going|take\s+it\s+any\s+more)
  | \s*(?:,|\band\b|\bbut\b)\s*
    (?:\w+\s+){0,3}?i\s*(?:['\u2019]m|m\b|am)\s*(?:at\s+my\s+wits['\u2019]?\s*end|at\s+the\s+end\s+of\s+my\s+tether)
"#
);
pub(super) const MORE_THAN: usize = 4;
pub(super) const GOES_ON: &[&str] = &[
    "after", "and", "because", "before", "but", "cause", "cos", "so", "then", "when", "which",
    "while",
];
pub(super) const WHOLE_NOW: &[&str] = &[
    "Ugh, that sounds relentless. What's the worst bit of it?",
    "That's a lot at the moment. What's the hardest part?",
    "Ah no. What is it that's wearing you down?",
];
pub(super) const WHOLE: &[&str] = &[
    "Sounds like one of those days. What went wrong?",
    "Oh no — what happened?",
    "Ouch. What went wrong?",
    "Ah, everything at once? What happened?",
];
pub(super) const ONGOING: &[&str] = &[
    "Ugh — {thing} sounds relentless. What's the worst bit of it?",
    "That's a lot to be carrying. What's the hardest part of {thing}?",
    "Ah no. What's the worst of {thing} at the moment?",
    "I bet. What is it about {thing} that's wearing you down?",
];
pub(super) const NOT_A_DOER: &[&str] = &[
    "application",
    "appointment",
    "audit",
    "birthday",
    "class",
    "date",
    "day",
    "essay",
    "exam",
    "holiday",
    "interview",
    "lesson",
    "meeting",
    "party",
    "presentation",
    "review",
    "shift",
    "test",
    "trial",
    "wedding",
];
pub(super) const ABOUT_OCCASION: &[&str] = &[
    "Oh no — what happened with {thing}?",
    "That's rotten — what happened with {thing}?",
    "Oh no. What went wrong with {thing}?",
];
pub(super) const ABOUT_THING: &[&str] = &[
    "Oh no — what happened with {thing}?",
    "Ah, that's a shame. What went wrong with {thing}?",
    "That's rotten — what happened with {thing}?",
    "Ah no. What went wrong with {thing}?",
    "Oh no. How did {thing} go wrong?",
];
