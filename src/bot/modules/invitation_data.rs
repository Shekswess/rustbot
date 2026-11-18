//! Authored invitation readers and reply pools from PyBot's composer.

pub(super) const INVITED: &str = r#"\b(?:do|would|will|can)\s+you\s+(?:want|fancy|feel\s+like|be\s+up\s+for|come|join)\b|\b(?:do|would|will)\s+you\s+(?:like\s+to\s+)?(?:come|join)\b|\bfancy\b|\bup\s+for\b|\bwanna\b|\bshall\s+we\b|\bwe\s+could\b|\bwant\s+to\s+(?:go|get|grab|have|see|watch|come|join)\b|\bcare\s+to\s+(?:come|join)\b"#;
pub(super) const INVITED_AT_THE_START: &str =
    r#"^(?:fancy|wanna|up\s+for|shall\s+we|we\s+could|want\s+to)\b"#;
pub(super) const NOT_THEIRS: &str = r#"^(?:i|we|he|she|they|my|our|her|his|their)\b"#;
pub(super) const NOT_WANTED: &str = r#"\b(?:know|knew|learn|ask|asked|tell|say|hear|check|explain|guess|help|try)\b|\b(?:see|check|find\s+out)\s+if\b|\bwhether\b|\bhow\s+to\b"#;
pub(super) const NOT_AN_INVITE: &str = r#"\b(?:anything|something|a\s+hand|a\s+lift|a\s+receipt|a\s+piece|me\s+come|her|him|them|die|dying|kill)\b|\b(?:want|like|need|fancy)\s+(?:me|us)\s+to\b"#;
pub(super) const PRESENCE: &str = r#"\b(?:can|could|would|will)\s+(?:you|u)\s+(?:please\s+|possibly\s+)?(?:come|pop|stop|drop|visit|join|meet|attend|be)\b|\bare\s+(?:you|u)\s+(?:coming|free|around|about)\b|\b(?:do|would|will)\s+(?:you|u)\s+(?:be\s+)?(?:coming|there|around)\b|\b(?:you|u)\s+(?:coming|joining\s+(?:me|us))\b|\bwhy\s+don'?t\s+(?:you|u)\s+(?:come|join)\b|\b(?:come|join)\s+(?:me|us)\s+(?:for|to)\b"#;
pub(super) const NOT_PRESENCE: &str = r#"\bcome\s+(?:up|back|across|down\s+to|off)\b|\b(?:be|being)\s+there\s+for\s+(?:me|us|him|her|them)\b|\bmeet\s+(?:the|a|an|my|your|our)\b|\b(?:be|is|are|was)\s+(?:there|around)\s+(?:when|if|to\s+(?:help|talk|listen))\b"#;
pub(super) const ASKING_IF_IM_HERE: &str = r#"^(?:hi|hey|hello|ok|okay|so|right|and|thanks|ta|sorry|please)?\s*(?:are\s+(?:you|u)|you|r\s+u)\s+(?:there|here|about|around)\s*[?.!]*$"#;

pub(super) const OCCASIONS: &[(&str, &[&str])] = &[
    (
        r#"\b(?:coffee|cuppa|brew|pint|beer|wine|drink|juice|tea\s+out|pub|bar|cafe|coffee\s+shop)\b"#,
        &[
            r#"I can't come along, but that sounds like a good idea. Where do you go for it?"#,
            r#"Wish I could join you. Somewhere near you, or the place you always go?"#,
        ],
    ),
    (
        r#"\b(?:lunch|dinner|brunch|breakfast|takeaway|take\s+away|curry|pizza|chinese|indian|bite|meal|restaurant|nando'?s|kebab)\b"#,
        &[
            r#"I can't make it, but that sounds good. Is it a treat, or an ordinary weeknight?"#,
            r#"Wish I could be there. Somewhere new, or the place you always end up in?"#,
        ],
    ),
    (
        r#"\b(?:walk|stroll|hike|wander|run|jog|cycle|bike\s+ride|swim|gym|park\s+run)\b"#,
        &[
            r#"I can't come with you, but that sounds like a good idea. Is there a route you like, or somewhere new to try?"#,
            r#"Wish I could join you. It clears the head more than people expect - where do you go?"#,
        ],
    ),
    (
        r#"\b(?:film|movie|show|play|gig|concert|match|telly|cinema|theatre|festival)\b"#,
        &[
            r#"I can't come along, but that is a good shout. What is on?"#,
            r#"Wish I could be there. Something you have been meaning to see, or picking on the night?"#,
        ],
    ),
    (
        r#"\b(?:shopping|shops|market|city\s+centre|high\s+street|boot\s+sale)\b"#,
        &[
            r#"I can't come along, but that sounds like a good afternoon. Is it a browse, or are you after something in particular?"#,
            r#"Wish I could be there. Do you know what you want, or will it find you?"#,
        ],
    ),
    (
        r#"\b(?:day\s+out|weekend\s+away|trip|beach|countryside|seaside|coast|museum|stately\s+home|picnic)\b"#,
        &[
            r#"I can't come along, but that sounds like a good day out. Where are you thinking?"#,
            r#"Wish I could be there. Have you been before, or would it be the first time?"#,
        ],
    ),
    (
        r#"\b(?:party|bbq|barbecue|barbeque|wedding|birthday|christening|housewarming|house-warming|quiz|karaoke|reunion|funeral|wake|service|graduation|engagement|anniversary|get-together|get\s+together|celebration|naming\s+ceremony)\b|\b(?:a|the|this|that|my|his|her|their|big|fancy|work|office|leaving|christmas|charity|fundraising|retirement|birthday)\s+do\b"#,
        &[
            r#"I can't be there - I'm words on a screen - but it sounds a good do. What do you need help with for it: what to take, what to wear, or what to say to the host?"#,
            r#"No legs on this end, so I can't make it, sorry. Tell me how many are going and what you've got to sort out, and I'll do the planning side of it."#,
        ],
    ),
];

pub(super) const PRESENCE_REPLIES: &[&str] = &[
    r#"I can't be there in person - nothing here leaves the chat - but tell me the plan and I'll do the useful half: what to take, what to cook, how to get there."#,
    r#"No body on this end, so coming along isn't mine, sorry. Say what the day looks like and I'll help with the bits that are words - the list, the timing, what to say."#,
    r#"I'd be there if I could. I'm not, so let me help from here: how many are coming, and what have you got left to sort?"#,
];
