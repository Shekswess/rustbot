//! Film suggestions and query shapes from the pinned entertainment rules.

pub(super) const FRESH_LIST: &str = r#"\b(?:any|some|more|which|what)\s+(?:good\s+|other\s+|different\s+)?(?:films?|movies?|books?|novels?|series|shows?|programmes?|programs?|podcasts?)\b|\b(?:films?|movies?|books?|novels?|series|shows?|programmes?|programs?|podcasts?)\s+(?:for|to watch|to read|to listen|on the|for the)\b"#;
pub(super) const FILM_ASK: &str = r#"\b(?:want|wants|need|needs|fancy|looking for|after|recommend|recommendation|suggest|should i|shall i|can you|could you|give me|got any|any|what|which|good|best|decent|mood for|in the mood)\b"#;
pub(super) const GENTLE_LIST: &str = "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.";
pub(super) const EASY_FILM_LIST: &str = "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.";
pub(super) const ASK_BACK: &str = "Tell me what you liked about the last film you enjoyed: the story, the jokes, or how tense it was. Name one and I'll suggest something close.";
pub(super) const FILM_ASK_WORDS: &[&str] = &[
    "boxset", "episode", "episodes", "film", "films", "iplayer", "movie", "movies", "netflix",
    "series", "telly", "tv", "watch", "watching",
];
pub(super) const FILM_MOODS: &[(&str, &str)] = &[
    (
        "sad",
        "For a good cry: *Manchester by the Sea* is quiet and devastating, *Atonement* lands its ending like a brick, and *Up* does it in the first ten minutes. Have tissues ready.",
    ),
    (
        "funny",
        "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
    ),
    (
        "think",
        "*Arrival* (would you want to know the future?), *12 Angry Men* (one room, all talk, gripping) or *Ex Machina* (quiet, unsettling science fiction). All three stay with you afterwards.",
    ),
    (
        "uplifting",
        "*Hunt for the Wilderpeople* (funny and warm), *The Shawshank Redemption* (hope you have to earn) or *Sing Street* (music, first love and a big grin at the end).",
    ),
    (
        "gripping",
        "*Prisoners* is tense from the first scene, *Gone Girl* keeps you guessing to the end, and *Sicario* turns the screw for two hours straight.",
    ),
    (
        "short",
        "*Before Sunset* runs 80 minutes, *Locke* is 85 with one man in a car, and *Toy Story* is 81. All three are short and none of them asks much of a tired evening.",
    ),
    (
        "true",
        "*Zodiac* (the investigation, not the killings), *Catch Me If You Can* (light and a lot of fun) or *127 Hours* if you want to be on the edge of your seat.",
    ),
    (
        "twist",
        "*The Sixth Sense*, *The Prestige* and *Gone Girl* all turn on a twist you will not see coming. Start with *The Prestige*.",
    ),
    (
        "romance",
        "*Before Sunrise* (two people talking all night), *When Harry Met Sally* (the funny one) or *Pride & Prejudice* (2005) for the longing and the costumes.",
    ),
    (
        "light",
        "*Paddington 2*, *Chef* or *The Detectorists* (a series): warm, low-stakes and easy to watch, and nothing bad happens in any of them.",
    ),
];
