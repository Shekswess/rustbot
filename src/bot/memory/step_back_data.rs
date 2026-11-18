//! Step-reading gates and templates from PyBot's memory/repeat.py at 7d63c7f.
pub(super) const WORDING: &str = r####"(?i)\bhow (?:do|can|should|could|would|might) (?:i|we)\s+(?:(?:word|write|say|phrase|put|explain|draft|reply|tell|ask|answer)\b|(?:bring|raise|mention|broach|approach)\s+(?:it|that|this|them|the (?:subject|issue|matter|problem))\b)"####;
pub(super) const QUALIFIER: &str = r####"(?i)\b(?:because|so that|which|if|unless|before|after|until|while|when|once|in case|as long as|rather than)\b"####;
pub(super) const LIST_LEAD: &str = r####"(?i)\b(?:works?|working|helps?|helped|things to do|to do|steps?|rules?|habits?|ways?|tips?|start|begin|try|keep|stick to|do these|do this)\b"####;
pub(super) const NOT_A_TAIL: &str = r####"(?i)\b(?:things|rest|following|list|these|those|steps|rules|ways|tips|habits|ideas|examples|reasons)\b"####;
pub(super) const NOT_A_DOING: &str = r####"(?i)\b(?:you|your|i|we|they|he|she|it|its|their)\b|\b(?:is|are|was|were|has|have|had|means|meant)\b"####;
pub(super) const A_QUESTION: &str = r####"(?i)\W*(?:how|what|which|when|where|who|why|do|did|does|can|could|should|would|will|shall|is|are|was|were|have|has)\b"####;
pub(super) const DECLARATIVE: &str = r####"(?i)^(?:[a-z']+\s+)(?:is|are|was|were|has|have|had|can|cannot|will|would|should|takes?|gives?|means?|costs?|comes?|goes?|does|did|gets?|keeps?)\b"####;
pub(super) const QUESTIONS_THAT_ARE_NOT_STEPS: &str =
    r####"(?i)^(?:what|which|how|when|where|who|why|tell me|let me know)\b"####;
pub(super) const PIECE_END: &str = r####":\s|:|(?<!\d),(?!\d)| - |;| \u2014 "####;
pub(super) const NOT_A_STEP: &str =
    r####"(?i)^(?:tell|let|send|give|show|ask|name)\s+(?:me|us)\b"####;
pub(super) const A_MENU: &str =
    r####"(?i)\b(?:whichever|whatever|whenever|either|any of them)\b"####;
pub(super) const CONDITION_CLAUSE: &str =
    r####"(?i)^\s*(?:if|unless|when|whenever|once|in case|should|as long as)\b"####;
pub(super) const CHANGE_ASK: &str = r####"(?i)\b(?:change|changed|fix|tackle|improve)\b"####;
pub(super) const STEP_SAYS_FIRST: &str = r####"(?i)^(?:start|begin|first)\b"####;
pub(super) const LEAD_SAYS_FIRST: &str = r####"(?i)\b(?:start|begin|first)\b"####;
pub(super) const NEEDS_COMPLEMENT: &[&str] = &[
    r####"apply"####,
    r####"cover"####,
    r####"dip"####,
    r####"fill"####,
    r####"hang"####,
    r####"keep"####,
    r####"lay"####,
    r####"leave"####,
    r####"place"####,
    r####"pour"####,
    r####"put"####,
    r####"seal"####,
    r####"set"####,
    r####"soak"####,
    r####"spray"####,
    r####"store"####,
    r####"wrap"####,
];
pub(super) const LEADING_PHRASE: &[&str] = &[
    r####"about"####,
    r####"after"####,
    r####"afternoon"####,
    r####"also"####,
    r####"at"####,
    r####"before"####,
    r####"carefully"####,
    r####"day"####,
    r####"days"####,
    r####"during"####,
    r####"each"####,
    r####"easily"####,
    r####"evening"####,
    r####"evenings"####,
    r####"every"####,
    r####"five"####,
    r####"for"####,
    r####"four"####,
    r####"gently"####,
    r####"half"####,
    r####"hour"####,
    r####"hours"####,
    r####"in"####,
    r####"just"####,
    r####"lightly"####,
    r####"minute"####,
    r####"minutes"####,
    r####"month"####,
    r####"months"####,
    r####"morning"####,
    r####"mornings"####,
    r####"night"####,
    r####"nights"####,
    r####"on"####,
    r####"once"####,
    r####"one"####,
    r####"only"####,
    r####"per"####,
    r####"quarter"####,
    r####"quickly"####,
    r####"really"####,
    r####"slowly"####,
    r####"three"####,
    r####"times"####,
    r####"twice"####,
    r####"two"####,
    r####"week"####,
    r####"weeks"####,
    r####"year"####,
    r####"years"####,
];
pub(super) const AN_ARTICLE: &[&str] = &[r####"a"####, r####"an"####, r####"the"####];
pub(super) const OPENERS: &[&str] = &[
    r####"at"####,
    r####"by"####,
    r####"for"####,
    r####"from"####,
    r####"in"####,
    r####"inside"####,
    r####"into"####,
    r####"of"####,
    r####"on"####,
    r####"onto"####,
    r####"over"####,
    r####"through"####,
    r####"to"####,
    r####"under"####,
    r####"with"####,
];
pub(super) const DANGLING: &[&str] = &[
    r####"a"####,
    r####"am"####,
    r####"an"####,
    r####"and"####,
    r####"are"####,
    r####"as"####,
    r####"at"####,
    r####"be"####,
    r####"because"####,
    r####"been"####,
    r####"being"####,
    r####"but"####,
    r####"by"####,
    r####"even"####,
    r####"for"####,
    r####"from"####,
    r####"her"####,
    r####"here"####,
    r####"his"####,
    r####"how"####,
    r####"if"####,
    r####"in"####,
    r####"is"####,
    r####"it"####,
    r####"its"####,
    r####"my"####,
    r####"of"####,
    r####"on"####,
    r####"or"####,
    r####"our"####,
    r####"plus"####,
    r####"so"####,
    r####"than"####,
    r####"that"####,
    r####"the"####,
    r####"their"####,
    r####"then"####,
    r####"there"####,
    r####"these"####,
    r####"this"####,
    r####"those"####,
    r####"to"####,
    r####"unless"####,
    r####"until"####,
    r####"was"####,
    r####"were"####,
    r####"what"####,
    r####"when"####,
    r####"which"####,
    r####"while"####,
    r####"who"####,
    r####"whom"####,
    r####"whose"####,
    r####"why"####,
    r####"with"####,
    r####"yet"####,
    r####"your"####,
];
pub(super) const MOVE_LEADS: &[&str] = &[
    r####"Start with this: {step}."####,
    r####"The thing to do first: {step}."####,
    r####"I would start here: {step}."####,
    r####"The first thing to do: {step}."####,
];
pub(super) const CHANGE_LEADS: &[&str] = &[
    r####"The first thing I would change: {step}."####,
    r####"What I would change first: {step}."####,
    r####"Change this first: {step}."####,
];
pub(super) const NOW_LEADS: &[&str] = &[
    r####"The one to do now: {step}."####,
    r####"The one I would pick: {step}."####,
    r####"Do this one: {step}."####,
];
pub(super) const STEP_WORDS: usize = 20;
pub(super) const STEP_WITH_ITS_METHOD: usize = 30;
pub(super) const CUT_WORDS: usize = 12;
pub(super) const SHORT_STEP: usize = 6;
pub(super) const LEADING_PHRASE_MAX: usize = 3;
