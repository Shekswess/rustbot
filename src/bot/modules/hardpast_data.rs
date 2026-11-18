//! Authored fences and replies for hard events told as past news.

pub(super) const A_HARD_PAST: &str = r#"\b(?:alcoholic|alcoholics|drinker|drinkers|addict(?:s|ed|ion)?|overdos(?:e|ed)|rehab|prison|jail|divorce[ds]?|custody|abusive|abuser|widow|widowed|widower|estrang\w*|homeless|bankrupt\w*|gambling|dementia|stillborn|stillbirth|miscarriage|miscarried|car crash|in a crash|had a crash|bad crash|fatal crash|marriage ended|marriage broke|relationship ended|walked out|left us)\b"#;
pub(super) const A_THE_PAST: &str = r#"\b(?:was|were|had been|used to|grew up|growing up|as a kid|when i was|went to|walked out|ended|ago|for a bit|for a while|for years|back then|in the end|at the time|last (?:year|summer|winter|decade)|years back|after that)\b"#;
pub(super) const A_NEWS_ARRIVING: &str = r#"\b(?:getting|going through|just told|just said|just heard|asked for|wants|wanting|is leaving|has asked|separating|divorcing|this morning|tonight|right now|at the moment|today)\b"#;
pub(super) const A_CLOSED: &str = r#"\b(?:fine|okay|ok|good|better|clean|dry|sober|sorted|happy|healthy|stable|working|employed|a job|job|a house|a flat|a home|back on my feet|still friends|still together|paid off|all paid)\b[^.!?]{0,24}?\b(?:now|since|these days|any ?more)\b|\b(?:now|since|these days)\b[^.!?]{0,24}?\b(?:fine|okay|ok|good|better|clean|dry|sober|sorted|happy|healthy|stable|working|employed|a job|job|a house|a flat|a home|back on my feet|still friends|still together|paid off|all paid)\b|\b(?:dry|clean|sober|straight|off it)\b[^.!?]{0,16}?\b\d+\s*(?:years?|months?|decades?)\b|\b(?:never looked back|put it behind me|put that behind me|moved on from it|turned (?:it|things|my life) around|came out the other side|over it now|past it now|in a better place|still friends|no longer)\b"#;
pub(super) const A_LOSS: &str = r#"\b(?:died|dies|dying|death|dead|passed away|funeral|buried|miscarriage|miscarried|stillborn|stillbirth|widow|widows|widowed|widower|bereaved|bereavement)\b"#;

pub(super) const A_OWNED: &str = r#"
my our his her their your mum mums mum's dad dads dad's mother father brother sister son daughter
husband wife partner ex nan gran granny grandma grandad grandpa uncle aunt cousin parents family
stepdad stepmum
"#;

pub(super) const A_BETWEEN: &str = r#"
i we ive weve been was were am had have has got did do done went never once really badly eventually
finally long time terrible bad hard worst back then ago after before so that this it there with from
at to a an the in into through out on off for of and but also both years year months weeks days
month week twice again now still just
"#;

pub(super) const A_HARD_PAST_SAID: &[&str] = &[
    r#"That is a heavy thing to have lived with. Say more about it if you want to."#,
    r#"That is not a small thing to carry. I'm listening if you want to say more."#,
    r#"That one leaves a mark. Go on if you want to."#,
    r#"That is a lot to have been through. Tell me as much or as little as you like."#,
];
