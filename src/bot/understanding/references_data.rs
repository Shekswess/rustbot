//! Vocabulary and matching expressions from the pinned PyBot reference reader.
pub(super) const FOLLOWUP_STARTS: &[&str] = &[
    "also",
    "and",
    "and then",
    "and what",
    "another",
    "anything else",
    "but",
    "can it",
    "can that",
    "does it",
    "does that",
    "else",
    "even if",
    "how about",
    "how long",
    "how many",
    "how much",
    "how often",
    "instead",
    "is it",
    "is that",
    "more",
    "ok and",
    "ok so",
    "other",
    "same",
    "should it",
    "so",
    "so that",
    "then",
    "what about",
    "what else",
    "what if",
    "when",
    "where",
    "which",
    "why",
    "will it",
    "will that",
    "would it",
];
pub(super) const POINTING_BACK: &[&str] = &[
    "about", "across", "around", "at", "behind", "beside", "by", "for", "from", "in", "inside",
    "into", "near", "off", "on", "onto", "outside", "over", "through", "to", "under", "with",
];
pub(super) const ASK_OPENERS: &[&str] = &[
    "can", "could", "did", "do", "does", "may", "might", "must", "shall", "should", "will", "would",
];
pub(super) const CLAUSE_SUBJECTS: &[&str] = &[
    "cant", "dont", "he", "i", "im", "it", "its", "ive", "she", "thats", "there", "they", "we",
    "you",
];
pub(super) const COORDINATORS: &[&str] = &["and", "but", "so", "then"];
pub(super) const POSSESSIVE_PRONOUNS: &[&str] = &["its", "their"];
pub(super) const PRONOUN_WORDS: &[&str] = &[
    "he", "her", "hers", "herself", "him", "himself", "his", "it", "its", "itself", "one", "ones",
    "she", "that", "their", "theirs", "them", "there", "these", "they", "this", "those",
];
pub(super) const OWN_PERSONS: &[&str] = &["name", "person", "title"];
pub(super) const ONE_PERSON: &[&str] = &["he", "her", "hers", "him", "his", "she"];
pub(super) const PLACEHOLDER_IT: &str = r#"^(?:is|was|would|will|does|did|can|could|should)(?:n'?t)?\s+it\s+(?:[a-z']+\s+){0,3}?to\s+[a-z]+"#;
pub(super) const NAMES_A_THING: &str =
    r#"(?i)\b(?:a|an|the|some|any|my|your|our|their|his|her|its|another)\s+[a-z]"#;
pub(super) const STANDS_FOR_SOMETHING: &[&str] = &[
    "her", "him", "it", "its", "one", "ones", "that", "them", "there", "these", "they", "this",
    "those",
];
pub(super) const LEAVES_SOMETHING_OUT: &[&str] = &[
    "about", "after", "and", "at", "before", "but", "during", "for", "from", "if", "in", "instead",
    "on", "or", "since", "that", "then", "till", "to", "until", "when", "while", "with", "without",
];
pub(super) const ANIMALS_IN_HAND: &str = r#"(?i)\b(dogs?|pupp(?:y|ies)|cats?|kittens?|rabbits?|bunn(?:y|ies)|hamsters?|guinea\s+pigs?|parrots?|budgies?|goldfish|fish|tortoises?|turtles?|snakes?|lizards?|geckos?|horses?|pon(?:y|ies))\b"#;
pub(super) const OWN_SUBJECT: &[&str] = &["i", "id", "ill", "im", "ive", "we", "wed", "weve"];
pub(super) const ANIMAL_NAMES: &[&str] = &[
    "cat", "cats", "dog", "dogs", "kitten", "kittens", "puppies", "puppy",
];
pub(super) const THIRD_PERSON: &[&str] = &[
    "he", "her", "hers", "herself", "him", "himself", "his", "it", "its", "itself", "one", "ones",
    "she", "that", "their", "theirs", "them", "there", "these", "they", "this", "those",
];
pub(super) const MINE: &[&str] = &["mine", "my", "our", "ours"];
pub(super) const IT_WORDS: &[&str] = &["it", "its", "itself"];
pub(super) const THAT_WORDS: &[&str] = &["one", "ones", "that", "these", "this", "those"];
pub(super) const THEY_WORDS: &[&str] = &["their", "theirs", "them", "they"];
pub(super) const THEY_APOSTROPHES: &[&str] = &["theyd", "theyll", "theyre"];
pub(super) const THERE_WORDS: &[&str] = &["there"];
pub(super) const HE_WORDS: &[&str] = &["he", "him", "himself", "his"];
pub(super) const HE_APOSTROPHES: &[&str] = &["hed", "hes", "hisself"];
pub(super) const SHE_WORDS: &[&str] = &["her", "hers", "herself", "she"];
pub(super) const SHE_APOSTROPHES: &[&str] = &["shed", "shes"];
pub(super) const DETERMINERS: &[&str] = &[
    "a", "an", "another", "any", "each", "every", "her", "his", "its", "my", "our", "some", "that",
    "the", "their", "these", "this", "those", "your",
];
