//! Pinned replies and guards from original film recommendation rules.
pub(super) struct Case {
    pub id: &'static str,
    pub prompt: &'static str,
    pub topics: &'static [&'static str],
    pub last: Option<&'static str>,
    pub guard: bool,
    pub reply: Option<&'static str>,
}
pub(super) const CASES: &[Case] = &[
    Case {
        id: "film-like",
        prompt: "can you recommend a film like inception",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Inception* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for a flight?",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "something funny to watch",
        topics: &[],
        last: None,
        guard: true,
        reply: None,
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "i want a film like mission impossible",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Mission Impossible* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a movie like foobar's part-two and not long",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Foobar'S Part-Two* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like the bear",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "After *The Bear*: *Boiling Point* (the film, one unbroken take), *Burnt* for a lighter kitchen, or *The Bear*'s closest cousin *Shameless* (US) for a family that shouts. All short episodes.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like the beard",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *The Beard* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "could you suggest a film i liked",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "Tell me what you liked about the last film you enjoyed: the story, the jokes, or how tense it was. Name one and I'll suggest something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "what did she like about it",
        topics: &[],
        last: None,
        guard: true,
        reply: None,
    },
    Case {
        id: "film-like",
        prompt: "i loved the film with the hotel in it",
        topics: &[],
        last: None,
        guard: true,
        reply: None,
    },
    Case {
        id: "film-like",
        prompt: "any good films",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "they liked the one with the hotel in it",
        topics: &[],
        last: None,
        guard: true,
        reply: None,
    },
    Case {
        id: "film-like",
        prompt: "movie like paddington 2",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "If you liked *Paddington*, try *The Mitchells vs the Machines* or *Matilda*: warm, funny and nothing scary. For a clever, warm film, *Knives Out* or *The Grand Budapest Hotel* are the next step.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "something like the office",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "After *The Office*: the UK original if you have only seen the US one, then *Parks and Recreation*, *Brooklyn Nine-Nine* or *Superstore*. All the same mockumentary warmth.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any movies for the night",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "i liked that",
        topics: &[],
        last: None,
        guard: true,
        reply: None,
    },
    Case {
        id: "film-like",
        prompt: "a movie similar to oranges: part 2",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Oranges: Part 2* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like a dog and her toy",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *A Dog* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like the wire",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "After *The Wire*: *The Shield* and *Generation Kill* (same creator), *Bosch* for the police work, or *Oz* for the same prison-city sweep.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like the wireless",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *The Wireless* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "the office at work is large",
        topics: &[],
        last: None,
        guard: true,
        reply: None,
    },
    Case {
        id: "film-like",
        prompt: "i recommend a film like saint-john's moon, please",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Saint-John'S Moon* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "can you recommend a movie?",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "Tell me what you liked about the last film you enjoyed: the story, the jokes, or how tense it was. Name one and I'll suggest something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "what films are good?",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "films to watch after a long day",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "i dont want to watch a scary film",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "what else is good",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "and a series",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "any other funny films",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "any good comedies",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "what scary movie should i watch",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "anything but scary please",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "no books what about a scary film",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "no plans tonight, something scary",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "no gore in a film",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "the scary ones are not for me",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "not a scary one",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "anything a bit lighter",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "it turns off",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "my dog is scared",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "i dislike horror",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "what should i read",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "i dont want a war film",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "a war film please",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "a film about war i can watch with my dad",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "what's a good wartime film that's short",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "i hate comedies but i want a war film",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "i dont want to watch a film about war",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "a wartime film",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "wartime films are not for me",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "a war film but not a comedy",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "watch some war films",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "roses are not a documentary",
        topics: &[],
        last: None,
        guard: true,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "i want war films except war",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-war",
        prompt: "i dont want war, actually something about war",
        topics: &[],
        last: None,
        guard: false,
        reply: Some(
            "*Paths of Glory* is the one to start with: Kubrick, the WWI trenches, 88 minutes. *Went the Day Well?* (92 minutes) is a British village invasion story from 1942, and *Journey's End* (107 minutes) stays in one dugout and never shows the battle. All three fit inside two hours, and *Full Metal Jacket* is 116 if you want the training half too.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight",
        topics: &["something funny to watch"],
        last: None,
        guard: true,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight",
        topics: &["id like a romance", "i dont want something funny"],
        last: None,
        guard: true,
        reply: Some(
            "*Before Sunrise* (two people talking all night), *When Harry Met Sally* (the funny one) or *Pride & Prejudice* (2005) for the longing and the costumes.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight",
        topics: &["a sad book", "need a think about it"],
        last: None,
        guard: true,
        reply: Some(
            "For a good cry: *Manchester by the Sea* is quiet and devastating, *Atonement* lands its ending like a brick, and *Up* does it in the first ten minutes. Have tissues ready.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight",
        topics: &["a sunny day"],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "what films are good",
        topics: &["something funny to watch"],
        last: None,
        guard: true,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "what films are good",
        topics: &["id like a romance", "i dont want something funny"],
        last: None,
        guard: true,
        reply: Some(
            "*Before Sunrise* (two people talking all night), *When Harry Met Sally* (the funny one) or *Pride & Prejudice* (2005) for the longing and the costumes.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "what films are good",
        topics: &["a sad book", "need a think about it"],
        last: None,
        guard: true,
        reply: Some(
            "For a good cry: *Manchester by the Sea* is quiet and devastating, *Atonement* lands its ending like a brick, and *Up* does it in the first ten minutes. Have tissues ready.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "what films are good",
        topics: &["a sunny day"],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "more films for tonight",
        topics: &["something funny to watch"],
        last: None,
        guard: true,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "more films for tonight",
        topics: &["id like a romance", "i dont want something funny"],
        last: None,
        guard: true,
        reply: Some(
            "*Before Sunrise* (two people talking all night), *When Harry Met Sally* (the funny one) or *Pride & Prejudice* (2005) for the longing and the costumes.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "more films for tonight",
        topics: &["a sad book", "need a think about it"],
        last: None,
        guard: true,
        reply: Some(
            "For a good cry: *Manchester by the Sea* is quiet and devastating, *Atonement* lands its ending like a brick, and *Up* does it in the first ten minutes. Have tissues ready.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "more films for tonight",
        topics: &["a sunny day"],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight under 95 minutes",
        topics: &["something funny to watch"],
        last: None,
        guard: true,
        reply: Some(
            "For laughs, under 95 minutes: *What We Do in the Shadows* (86 minutes). The other two run longer, so say if you would rather have one of those.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight under 95 minutes",
        topics: &["id like a romance", "i dont want something funny"],
        last: None,
        guard: true,
        reply: Some(
            "Nothing in that list comes in under 95 minutes - the shortest is *When Harry Met Sally* at 96. Say another mood and I will look again.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight under 95 minutes",
        topics: &["a sad book", "need a think about it"],
        last: None,
        guard: true,
        reply: Some(
            "For a good cry, nothing there comes in under 95 minutes - the shortest is *Up* at 96. Say another mood and I will look again.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "any films for the flight under 95 minutes",
        topics: &["a sunny day"],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "films to watch when it rains",
        topics: &["something funny to watch"],
        last: None,
        guard: true,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "films to watch when it rains",
        topics: &["id like a romance", "i dont want something funny"],
        last: None,
        guard: true,
        reply: Some(
            "*Before Sunrise* (two people talking all night), *When Harry Met Sally* (the funny one) or *Pride & Prejudice* (2005) for the longing and the costumes.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "films to watch when it rains",
        topics: &["a sad book", "need a think about it"],
        last: None,
        guard: true,
        reply: Some(
            "For a good cry: *Manchester by the Sea* is quiet and devastating, *Atonement* lands its ending like a brick, and *Up* does it in the first ten minutes. Have tissues ready.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "films to watch when it rains",
        topics: &["a sunny day"],
        last: None,
        guard: true,
        reply: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a movie similar to inception",
        topics: &["something funny to watch"],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Inception* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a movie similar to inception",
        topics: &["id like a romance", "i dont want something funny"],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Inception* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a movie similar to inception",
        topics: &["a sad book", "need a think about it"],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Inception* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a movie similar to inception",
        topics: &["a sunny day"],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *Inception* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like she said",
        topics: &["something funny to watch"],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *She Said* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like she said",
        topics: &["id like a romance", "i dont want something funny"],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *She Said* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like she said",
        topics: &["a sad book", "need a think about it"],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *She Said* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-like",
        prompt: "a film like she said",
        topics: &["a sunny day"],
        last: None,
        guard: true,
        reply: Some(
            "I don't know *She Said* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "what else is good",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: false,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "what else is good",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "something not scary",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: true,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "something not scary",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "and a series",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "and a series",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "can you suggest a podcast",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "can you suggest a podcast",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "a book instead",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "a book instead",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "what books",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "what books",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "a different film",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: true,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "a different film",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "a show please",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "a show please",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "a film and a book",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: true,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "a film and a book",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: true,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "",
        topics: &[],
        last: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
        guard: false,
        reply: Some(
            "For laughs: *What We Do in the Shadows* (a daft vampire mockumentary), *The Nice Guys* (funny and twisty) or *Paddington 2*, which makes almost everyone laugh.",
        ),
    },
    Case {
        id: "film-nothing-scary",
        prompt: "",
        topics: &[],
        last: Some(
            "If you want something easy and good tonight: *Groundhog Day* for comedy, *Knives Out* for a clever whodunnit, or *Arrival* if you'd rather think. All three work on a sofa with no planning.",
        ),
        guard: false,
        reply: Some(
            "*Knives Out*, *The Grand Budapest Hotel* and *Paddington 2* are warm, funny and tense in the right places, and nothing that will keep you up. For a series, *The Detectorists* is gentle and very good, *Ghosts* is daft and kind, and *Happy Valley* is the tense British crime one.",
        ),
    },
];
