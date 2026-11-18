//! Rules from the pinned quotation module and its history reader (Apache-2.0).

pub(super) const A_THIRD_PARTY_ASK: &str = r#"(?i)\bwhat\s+did\s+(?:they|he|she|them|it|the\s+[a-z]+|(?!(?:i|we|you|my|our)\b)[a-z]+)\s+(?:say|said|tell\s+(?:you|me)|told\s+(?:you|me)|quote|quoted|charge|charged|offer|offered|reckon|reckoned|suggest|suggested|text|texted|come\s+back\s+with|wants?)\b|\bwhat\s+(?:did|have|has)\s+(?:they|he|she)\s+(?:said|told\s+(?:you|me))\b|\bremind\s+me\s+what\s+(?:they|he|she)\s+(?:said|say|told\s+you|quoted|wanted)\b"#;

pub(super) const SOMEBODY_SPOKE: &str = r#"(?i)\b(?!i\b|we\b)(?:they|he|she|the\s+[a-z]+|[a-z]+)\s+(?:said|says|say|told|tells|quoted|asked|charges?|offer(?:s|ed)?|reckons|thinks|texted|called|wants)\b"#;

pub(super) const A_FIGURE: &str = r#"(?i)(?:£|\$|€)\s?\d[\d,.]*\s?(?:k\b|m\b)?|\b\d[\d,.]*\s?(?:k\b|grand\b|thousand\b|quid\b|pounds?\b|p\b)|\b\d[\d,.]*\b"#;

pub(super) const QUESTION: &str = r#"(?i)^\s*(?:what|which|who|when|where|why|how|should|shall|can|could|would|do|does|did|is|are|was|were|will|have|has|am)\b"#;

pub(super) const REPORTING_VERB: &str = r#"(?i)\b(?P<who>(?:[a-z]+\s+)?[a-z]+)\s+(?:said|says|telling\s+me|told\s+me|tells\s+me|reckoned|texted|quoted|charged|offered|asked|wants|wanted|asks|needs|went)\b\s+(?:that\s+)?(.+)$"#;

pub(super) const A_MONEY: &str =
    r#"(?i)[£$€]|\b\d[\d,.]*\s?(?:k|m|grand|thousand|quid|pounds?|p)\b"#;

pub(super) const A_UNIT_AFTER: &str = r#"(?i)^\s*(?:years?|yrs?|months?|weeks?|days?|hours?|hrs?|minutes?|mins?|miles?|km|kilometres?|kilograms?|kg|stone|feet|foot|inches|times?|percent|%|o'?clock)\b"#;

pub(super) const NOT_A_DETAIL: &[&str] = &[
    r#"a"#,
    r#"about"#,
    r#"again"#,
    r#"already"#,
    r#"an"#,
    r#"and"#,
    r#"any"#,
    r#"are"#,
    r#"as"#,
    r#"at"#,
    r#"be"#,
    r#"been"#,
    r#"being"#,
    r#"but"#,
    r#"by"#,
    r#"called"#,
    r#"can"#,
    r#"cannot"#,
    r#"cant"#,
    r#"charge"#,
    r#"charged"#,
    r#"could"#,
    r#"did"#,
    r#"do"#,
    r#"does"#,
    r#"done"#,
    r#"exact"#,
    r#"exactly"#,
    r#"for"#,
    r#"he"#,
    r#"her"#,
    r#"hers"#,
    r#"him"#,
    r#"his"#,
    r#"how"#,
    r#"i"#,
    r#"id"#,
    r#"if"#,
    r#"in"#,
    r#"is"#,
    r#"it"#,
    r#"itll"#,
    r#"its"#,
    r#"just"#,
    r#"kind"#,
    r#"may"#,
    r#"me"#,
    r#"mention"#,
    r#"mentioned"#,
    r#"might"#,
    r#"must"#,
    r#"my"#,
    r#"name"#,
    r#"not"#,
    r#"of"#,
    r#"offer"#,
    r#"offered"#,
    r#"on"#,
    r#"one"#,
    r#"ones"#,
    r#"or"#,
    r#"our"#,
    r#"please"#,
    r#"quote"#,
    r#"quoted"#,
    r#"quotes"#,
    r#"really"#,
    r#"reckon"#,
    r#"remind"#,
    r#"said"#,
    r#"say"#,
    r#"saying"#,
    r#"says"#,
    r#"shall"#,
    r#"she"#,
    r#"should"#,
    r#"so"#,
    r#"some"#,
    r#"sort"#,
    r#"suggested"#,
    r#"tell"#,
    r#"telling"#,
    r#"text"#,
    r#"texted"#,
    r#"that"#,
    r#"the"#,
    r#"their"#,
    r#"theirs"#,
    r#"them"#,
    r#"these"#,
    r#"they"#,
    r#"thing"#,
    r#"things"#,
    r#"this"#,
    r#"those"#,
    r#"to"#,
    r#"told"#,
    r#"us"#,
    r#"was"#,
    r#"we"#,
    r#"were"#,
    r#"what"#,
    r#"when"#,
    r#"where"#,
    r#"which"#,
    r#"who"#,
    r#"whom"#,
    r#"whose"#,
    r#"why"#,
    r#"will"#,
    r#"with"#,
    r#"would"#,
    r#"you"#,
    r#"your"#,
];

pub(super) const A_KIND: &[&str] = &[
    r#"charge"#,
    r#"charged"#,
    r#"cost"#,
    r#"costs"#,
    r#"many"#,
    r#"much"#,
    r#"number"#,
    r#"offer"#,
    r#"offered"#,
    r#"paid"#,
    r#"pay"#,
    r#"price"#,
    r#"quote"#,
    r#"quoted"#,
];

pub(super) const THE_PERSON: &[&str] = &[
    r#"i"#, r#"id"#, r#"im"#, r#"ive"#, r#"me"#, r#"us"#, r#"we"#,
];

pub(super) const ASKS_BACK: &str = r#"(?i)\b(?:what|which|who|when|where|how\s+\w+)\b[^?!]{0,40}?\bdid\s+i\s+(?:say|said|tell\s+you|told\s+you|mention|mentioned)\b|\b(?:what|which)\s+(?:[a-z']+\s+){1,3}?(?:was|is|were)\s+it\b|\bwhat(?:'s|\s+is|\s+was)\s+my\s+(?:[a-z']+\s+){1,3}?again\b|\bremind\s+me\s+(?:what|which|when|where|who|how\s+many|how\s+much)\b|\bwhat\s+did\s+i\s+say\b|\bwhat\s+(?:was|were|am|is)\s+(?:i|we)\s+going\s+to\s+\w+|\bwhat\s+(?:was|is)\s+it\s+(?:he|she|they|we|i|you)\b|\bhow\s+(?:much|many)\b[^?!]{0,25}?\bdid\s+i\s+(?:say|said|need|want|ask)\b"#;

pub(super) const ASKS_BACK_A_KIND: &str = r#"(?i)\b(?:what|which)\s+(?:birthday|cost|date|day|dose|height|number|paid|pay|price|size|spend|spent|time|weight)\b[^?!]{0,22}?\bagain\b|\bthe\s+(?:birthday|cost|date|day|dose|height|number|paid|pay|price|size|spend|spent|time|weight)\s+(?:(?:that\s+)?(?:i|we)\s+(?:said|gave|told\s+you|mentioned)|again)\b|\b(?:did|have)\s+(?:i|we)\b[^?!]{0,30}?\b(?:birthday|cost|date|day|dose|height|number|paid|pay|price|size|spend|spent|time|weight)\b|\bhow\s+(?:much|many)\b[^?!]{0,25}?\bagain\b"#;

pub(super) const A_TELLING: &str =
    r#"(?i)^\s*(?:was|is|were|are|should\s+be|came\s+in|goes|needs|will\s+be)\b"#;

pub(super) const A_PAST_ACT: &str = r#"(?i)\b(?:did|have)\s+(?:i|we)\b"#;

pub(super) const A_KIND_FOR_WORD: &[&str] = &[
    r#"birthday"#,
    r#"cost"#,
    r#"date"#,
    r#"day"#,
    r#"dose"#,
    r#"height"#,
    r#"number"#,
    r#"paid"#,
    r#"pay"#,
    r#"price"#,
    r#"size"#,
    r#"spend"#,
    r#"spent"#,
    r#"time"#,
    r#"weight"#,
];
