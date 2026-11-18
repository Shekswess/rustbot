//! Authored language explanations and selectors from the pinned PyBot source.

pub(super) const A_DASH_OF_SOMETHING: &str = r#"(?i)\bdash(?:es)? of\b"#;
pub(super) const WRITING_BRACKETS: &str = r#"\bsentences?\b|\bwriting\b|\bwrote\b|\bgrammar\b|\bpunctuat\w*\b|\bessays?\b|\bparagraphs?\b|\bcomma\b|\bsemicolon\b|\bquote\b|\bquotes\b|\bspell\w*\b|\btext\b|\bword\b|\bwords\b|\bmarks?\b|\bmean\b|\bmeans\b|\benglish\b|\bwriter\b|\bdraft\b|\bcopy\b|\bedit\w*\b|\bproofread\w*\b|\bstyle\b"#;
pub(super) const NOT_A_PUNCTUATION_BRACKET: &str = r#"\b(wall|walls|shelf|shelves|shelving|tv|telly|television|tvs|monitor|monitors|speaker|speakers|soundbar|radiator|radiators|curtain|curtains|rail|rails|blind|blinds|pipe|pipes|plasterboard|stud|studs|drill|screws?|screwdriver|fixings?|rawl|mount|mounts|mounted|mounting|cistern|sink|boiler|cabinet|cabinets|wardrobe|mirror|aerial|satellite|dish|hinges?|fence|post|posts|beam|joists?|trellis|gutter|guttering|bathroom|kitchen)\b"#;
pub(super) const BUYING_BRACKETS: &str =
    r#"\bneed(s|ed)?\b|\bshould i (?:get|buy|use)\b|\bbuy\b|\bwhat size\b|\bwhich one\b"#;
pub(super) const NOT_MATHS_BRACKETS: &str = r#"\b(expand|expanding|simplify|simplifying|multiply|multiplying|factorise|factorize|factorising|solve|solving|remove|removing)\b|\d|\bx\b|\bxy\b|\bequals\b|\btax\b|\bincome\b|\bsalary\b|\bband\b|\bprice\b|\bpay\b|\bage\b|\bearn\b|\bcup\b|\bround\b|\btournament\b|\bleague\b|\bcompetition\b|\balgebra\b|\balgebraic\b|\bmaths\b|\bmath\b|\bequations?\b|\bsimplif\w*\b|\bpowers?\b|\bterms?\b|\bcoefficients?\b|\bnegatives?\b|\bdouble\b|\btriple\b|\bunderstand\b|\bmultiply out\b|\bsums?\b|\bcalculate\b|\bexpressions?\b|\bworking out\b|\bwork out\b|\bteam\b|\bteams\b|\bplayer\b|\bplayers\b|\bknockout\b|\bqualify\b|\bfixture\b|\bmatch\b|\bmatches\b|\bdraw\b|\bgroup\b|\bengland\b|\bscotland\b|\bwales\b|\bireland\b|\bfinal\b|\bshel(?:f|ves)\b|\bwall\b|\btv\b|\btelevision\b|\bscrews?\b|\bdrill\b|\bfixings?\b|\bcurtains?\b|\bblinds?\b|\bmonitor\b|\bspeakers?\b|\baerial\b|\bcupboard\b|\bwardrobe\b|\bmount\b|\bmounted\b|\bhinge\b|\bgate\b|\bradiators?\b|\bboiler\b|\btoilet\b|\bsink\b|\bcarrier\b|\btowel rail\b"#;
pub(super) const SOMEBODY_FORGETTING: &str = r#"\b(?:keeps?\s+forgetting|forgets|forgetting|forgot|repeats?\s+(?:himself|herself|themselves|itself)|cant\s+remember|can't\s+remember|mid\s+sentence|cant\s+find\s+(?:his|her|their)\s+words|can't\s+find\s+(?:his|her|their)\s+words)\b"#;
pub(super) const VERSE_INSTANCE_LEAD: &str = r#"^(?:please |can you |could you |will you |would you |ok |okay |so |and )*(?:give|write|make|compose|pen|say|tell|do|recite|sing)\b"#;
pub(super) const EVERY_MINUTE: &str = r#"\b(?:minutes|minute|mins|min)\b"#;
pub(super) const A_COUNTED_MINUTE: &str = r#"\b(?:\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|couple|few|half)(?:\s+(?:of|more|whole|short|quick|extra|good))?\s+(?:minutes|minute|mins|min)\b"#;
pub(super) const FIGURE_DEFAULT: &str = "Irony says the opposite of what is meant (a fire station burning down); onomatopoeia is a word that sounds like the thing (splash, buzz); alliteration repeats the opening sound (slippery slope); hyperbole exaggerates on purpose (a million times). Ask me about one and I'll give you its example.";
pub(super) const BOTH_FIGURES_DEFAULT: &str = "A simile compares with like or as (her laugh was like a kettle), and a metaphor says it is the thing (her laugh was a kettle). Both make a picture; the metaphor just does it without the 'like'.";
pub(super) const DASH_DEFAULT: &str = "A hyphen joins words into one (well-known, two-year-old) and has no spaces. A dash separates a phrase for emphasis — like this — and British style usually spaces an en dash. Word processors can turn two hyphens into a dash for you.";
pub(super) const GRAMMAR_DEFAULT: &str = "The parts of a sentence, in the order they turn up: a noun names something, a verb is what it does or is, an adjective describes it, an adverb says how the doing happened, and a pronoun stands in for the noun. 'The tired dog slept badly' - dog is the noun, tired the adjective, slept the verb, badly the adverb.";
pub(super) const POEM_DEFAULT: &str = "A haiku is three lines of five, seven and five sounds; a limerick is five lines rhyming aabba with a joke in it; a sonnet is fourteen lines in iambic pentameter. Ask about one and I'll give you its shape.";
pub(super) const SPEAK_DEFAULT: &str = "Slow to about two-thirds of your usual speed and finish the ends of words, which is most of what 'clear' means. Pause at the full stops instead of running the sentences together, drop your voice a little at the end of a sentence rather than letting it rise, and if you are on a call, stand up - it makes the voice carry.";
pub(super) const MARK_DEFAULT: &str = "Tell me which key it is on and I will name it: the tall line on the left of the keyboard is the pipe (|), the mark over 2 is the at sign (@), over 3 the hash (#), and the wavy one on the left is a tilde (~). If you can paste it into a line, I can read it too.";
pub(super) const FIGURES: &[(&[&str], &str)] = &[
    (
        &["irony", "ironic", "ironically"],
        "Irony says the opposite of what is meant and lets the situation do the work: a fire station burning down, or a no-smoking sign behind the smokers. Sarcasm is the spoken, pointed version of it.",
    ),
    (
        &["onomatopoeia", "onomatopoeic"],
        "Onomatopoeia is a word that sounds like the thing it names: splash, buzz, hiss, cuckoo, sizzle. Comics lean on it, and so does most children's poetry.",
    ),
    (
        &["alliteration"],
        "Alliteration repeats the opening sound of nearby words: slippery slope, sweet sixteen, the wild wind. It is about the sound, not the letter - \"know nothing\" is alliteration and \"cute cat\" is not.",
    ),
    (
        &["personification"],
        "Personification gives a thing or an idea human qualities: the wind whispered, the deadline stalked me, the kettle sulked. It turns an abstract thing into something you can picture.",
    ),
    (
        &["hyperbole"],
        "Hyperbole is exaggeration on purpose, not meant literally: I've told you a million times, this bag weighs a ton. It works because both people know it is not true.",
    ),
    (
        &["euphemism"],
        "A euphemism is a soft word for a hard one: passed away for died, let go for sacked, between jobs for unemployed. It spares feelings, and it can hide what happened.",
    ),
];
pub(super) const DASH_LOOKS: &[(&[&str], &str)] = &[
    (
        &["em dash", "em-dash", "long dash", "longest"],
        "An em dash is the longest dash — like this — and it marks a break or an aside where a comma would be too quiet. On Windows hold Alt and type 0151; on a Mac it is Shift and Option and the hyphen key.",
    ),
    (
        &["en dash", "en-dash", "minus", "range"],
        "An en dash is the middle-length dash, and its job is a range or a pair: 10–20 people, the London–Paris train. On a Mac it is Option and the hyphen key; in Word two hyphens turn into one as you type.",
    ),
    (
        &[
            "type",
            "keyboard",
            "how do i get",
            "how do i make",
            "shortcut",
        ],
        "For a dash you can just write two hyphens and let the word processor turn them into one; in Word and Google Docs that happens as you type. On a Mac use Option and the hyphen key, and on Windows Alt and 0151.",
    ),
];
pub(super) const POEM_FORMS: &[(&[&str], &str)] = &[
    (
        &["sonnet", "sonnets"],
        "A sonnet is fourteen lines in iambic pentameter - ten syllables a line, stress on every second one. Shakespeare's kind rhymes in three quatrains and a closing couplet (abab cdcd efef gg) and turns at line nine; the Italian kind splits six and eight.",
    ),
    (
        &["stanza", "stanzas", "verse paragraph"],
        "A stanza is a group of lines set apart in a poem, the way a paragraph is in prose. Two lines are a couplet, four a quatrain, and the pattern of rhymes across them is the rhyme scheme.",
    ),
    (
        &["free verse", "free-verse"],
        "Free verse is poetry with no fixed metre and no rhyme: the line breaks and the rhythm of the sentences do the work instead. Most modern poetry is written this way.",
    ),
    (
        &["iambic pentameter", "iambic", "pentameter", "blank verse"],
        "Iambic pentameter is five beats of da-DUM to the line - ten syllables with the stress on every second one: 'Shall I compare thee to a summer's day'. Blank verse is that without rhyme, which is what most of Shakespeare's plays are in.",
    ),
    (
        &["couplet", "couplets"],
        "A couplet is two lines that rhyme and usually stand together: 'Good night, good night! Parting is such sweet sorrow / That I shall say good night till it be morrow.'",
    ),
    (
        &["ballad", "ballads"],
        "A ballad is a story told in short verses, meant to be sung or recited: four-line stanzas that rhyme abab or abcb, with a refrain that comes back. The old folk ballads are the kind that run to a dozen verses, and pop still uses the name for a slow song about one feeling.",
    ),
];
pub(super) const MARK_LOOKS: &[(&[&str], &str)] = &[
    (
        &["wavy", "wave", "squiggle", "squiggly", "tilde", "squiggles"],
        "That is a tilde, ~. It means 'about' (~20 people came), and on most keyboards it is Shift and the hash key.",
    ),
    (
        &[
            "low line",
            "low dash",
            "underline",
            "under the letter",
            "under a letter",
            "between words",
        ],
        "That is an underscore, _. It joins words in file names and usernames (my_notes.txt); a hyphen is the shorter mark at half height.",
    ),
    (
        &["upright", "vertical", "tall line", "straight line"],
        "That is a pipe, |, also called the vertical bar. It separates options in a computer command; in writing a slash does the job better.",
    ),
    (
        &["star", "little star", "star shaped"],
        "That is an asterisk, *. It marks a footnote, and on a phone keyboard it hides behind the ?123 key.",
    ),
    (
        &["wedge", "little hat", "roof", "pointing up", "arrow head"],
        "That is a caret, ^. It marks where a word is to be slipped in, and in maths it means to the power of - 2^3 is eight.",
    ),
    (
        &["backwards", "other way", "wrong way"],
        "The one leaning the other way is a backslash, \\. It separates folders in a Windows file path like C:\\Users\\me, and it almost never belongs in writing.",
    ),
    (
        &["over the 2", "above the 2", "at sign", "a with a circle"],
        "That is the at sign, @ - Shift and 2 on a UK keyboard, and it is in every email address.",
    ),
    (
        &["over the 3", "above the 3", "hash", "pound", "number sign"],
        "That is the hash, # - Shift and 3 on a UK keyboard. On a phone it starts a hashtag, and in the US it labels a number.",
    ),
    (
        &["dot", "dots", "three dots", "full stop", "period"],
        "Three dots in a row are an ellipsis (...), and it means something has been left out or the thought trails off. A single dot is a full stop.",
    ),
    (
        &["squarish", "square", "box"],
        "Square brackets are [ ], and they hold your own words inside a quotation or a gap in a transcript.",
    ),
    (
        &["curly", "squiggly brackets", "brace", "braces"],
        "Curly brackets are { }, and they group things in maths and in code. Round brackets ( ) take an aside the sentence could do without.",
    ),
    (
        &["sideways v", "pointy", "arrow brackets"],
        "Angle brackets are < >. They mark a tag in code and stand in for a word to fill in later, as in <your name>.",
    ),
];
pub(super) const PHOTO_DETAIL: &[(&[&str], &str)] = &[
    (
        &[
            "dog",
            "dogs",
            "puppy",
            "pup",
            "cat",
            "cats",
            "kitten",
            "rabbit",
            "bunny",
            "horse",
            "pony",
            "parrot",
            "hamster",
            "guinea pig",
            "tortoise",
            "pets",
            "pet",
        ],
        "and that look it gives you just before it decides not to listen",
    ),
    (
        &[
            "cake",
            "cakes",
            "bread",
            "loaf",
            "dinner",
            "lunch",
            "breakfast",
            "soup",
            "pie",
            "biscuits",
            "cookies",
            "curry",
            "roast",
            "pancakes",
            "sourdough",
            "pizza",
            "pasta",
            "barbecue",
            "bbq",
            "coffee",
            "chocolate",
        ],
        "and it did not last the afternoon",
    ),
    (
        &[
            "sea",
            "beach",
            "mountain",
            "mountains",
            "hill",
            "hills",
            "sunset",
            "sunrise",
            "sky",
            "lake",
            "river",
            "city",
            "street",
            "forest",
            "snow",
            "holiday",
            "view",
            "garden",
        ],
        "and nobody else was awake yet",
    ),
];
pub(super) const GRAMMAR_TERMS: &[(&str, &str, &str)] = &[
    (
        "noun",
        "nouns",
        "A noun names a person, place or thing - Jess, Leeds, table, kindness. If you can put 'the' in front of it, it is a noun.",
    ),
    (
        "verb",
        "verbs",
        "A verb is the doing or being word: run, eat, seems, is. Every sentence needs one, and the verb changes with the time - I walk today, I walked yesterday.",
    ),
    (
        "adjective",
        "adjectives",
        "An adjective describes a noun: a red bus, a tired dog, an honest answer. It sits before the noun or after 'is' - the bus is red.",
    ),
    (
        "adverb",
        "adverbs",
        "An adverb says how, when or where something happened: she sang loudly, we left early, he waited outside. Many end in -ly, but not all do (soon, well, fast).",
    ),
    (
        "pronoun",
        "pronouns",
        "A pronoun stands in for a noun so you are not repeating it: I, you, he, she, it, we, they, me, him, her, them. 'Sam lost Sam's keys' is why we have them.",
    ),
    (
        "preposition",
        "prepositions",
        "A preposition says where or when in relation to something: in, on, under, between, before, after, with. The cat sat on the mat - 'on' is the preposition.",
    ),
    (
        "conjunction",
        "conjunctions",
        "A conjunction joins words or clauses: and, but, because, although, so. 'I was late because the bus broke down' joins two ideas with 'because'.",
    ),
    (
        "tense",
        "tenses",
        "Tense is when the verb happens: past (I cooked), present (I cook, I am cooking), future (I will cook). Pick one and keep to it within a sentence.",
    ),
    (
        "clause",
        "clauses",
        "A clause is a group of words with its own verb and subject: 'when the bus came' and 'we got on'. A sentence is one or more clauses; a phrase has no verb at all.",
    ),
    (
        "sentence",
        "sentences",
        "A sentence needs a subject and a verb and makes one complete point, starting with a capital and ending with a full stop: 'The bus was late.'",
    ),
    (
        "fronted adverbial",
        "fronted adverbials",
        "A fronted adverbial is the phrase that opens a sentence and says when, where or how: 'After lunch, we walked.' 'Slowly, the door opened.' Put a comma after it, and make sure the thing it describes is the subject of the sentence - 'Walking home, the rain started' says the rain was walking.",
    ),
    (
        "split infinitive",
        "split infinitives",
        "A split infinitive puts a word between to and the verb: 'to boldly go'. It is not an error, whatever you were taught - split it when it reads better and move the word when it gets clumsy: 'to go boldly'.",
    ),
    (
        "dangling modifier",
        "dangling modifiers",
        "A dangling modifier is an opening phrase that describes the wrong thing: 'Walking to work, the rain soaked me' says the rain was walking. Name the person straight after the comma - 'Walking to work, I was soaked by the rain' - or turn the phrase into a clause.",
    ),
    (
        "whom",
        "whoms",
        "Use whom after a preposition and where it is the one being acted on: 'to whom it may concern', 'the person whom I met'. If you can say he or she, write who; if you would say him or her, write whom. In speech who nearly always sounds right.",
    ),
    (
        "adverbial",
        "adverbials",
        "An adverbial is any phrase that says when, where, how or why: 'at six', 'in the kitchen', 'very slowly', 'because it rained'. It is not always an adverb - it can be a whole phrase or clause.",
    ),
    (
        "apostrophe",
        "apostrophes",
        "An apostrophe does two jobs: it marks something missing (do not becomes don't) or shows belonging (Jess's coat). It never makes a plain plural - two cats, not two cat's.",
    ),
    (
        "hashtag",
        "hashtags",
        "A hashtag is the # sign used as a label: #slowcooking groups posts about one thing. On a UK keyboard it is Shift and 3, and on a phone it is on the symbols page.",
    ),
    (
        "number sign",
        "number signs",
        "The number sign is # - Shift and 3 on a UK keyboard. In the US it is how you write a number (No. 5) and online it starts a hashtag.",
    ),
    (
        "ampersand",
        "ampersands",
        "An ampersand is the & sign and it means 'and'. Write and out in a sentence; the sign is for names and headings, like Marks & Spencer or R&D.",
    ),
    (
        "at sign",
        "at signs",
        "The at sign is @ - Shift and 2 on a UK keyboard - and it is in every email address: name@example.com.",
    ),
    (
        "asterisk",
        "asterisks",
        "An asterisk is the * sign, a small star. It marks a footnote or shows anything goes (*), and on a phone keyboard it is usually behind the ?123 key.",
    ),
    (
        "slash",
        "slashs",
        "A slash is the / sign, and it mostly means 'or' (and/or) or separates the parts of a date (12/05/2024). A backslash \\ is the other way over and turns up in computer paths rather than in writing.",
    ),
    (
        "backslash",
        "backslashs",
        "A backslash is the \\ sign, the mirror of a slash. It almost never belongs in writing - it separates folders in a Windows file path, like C:\\Users\\me.",
    ),
    (
        "underscore",
        "underscores",
        "An underscore is the _ sign, the low line under a letter. It joins words in file names and usernames (my_notes.txt); a hyphen is the shorter mark at half height.",
    ),
    (
        "tilde",
        "tildes",
        "A tilde is the ~ sign, the small wavy line. It means 'about' (~20 people came), and on most keyboards it is Shift and the hash key.",
    ),
    (
        "pipe",
        "pipes",
        "A pipe is the | sign, the upright bar. Most people call it the pipe or the vertical bar, and it separates options in a computer command; in writing a slash does the job better.",
    ),
    (
        "caret",
        "carets",
        "A caret is the ^ sign, a small wedge above the line. In proofreading it marks where a word is to be slipped in, and in maths it means to the power of - 2^3 is eight.",
    ),
    (
        "bullet point",
        "bullet points",
        "A bullet point is the dot that starts each item in a list. Keep every item to a line, start each one the same way, and leave the full stops off short items.",
    ),
    (
        "curly bracket",
        "curly brackets",
        "Curly brackets { } group things in maths and code, and square brackets [ ] hold your own words inside someone else's quotation. Round brackets ( ) take an aside the sentence could do without.",
    ),
    (
        "square bracket",
        "square brackets",
        "Square brackets [ ] hold your own words inside a quotation, and they mark a missing word in a transcript: 'she said it was [unclear] on the tape'.",
    ),
    (
        "angle bracket",
        "angle brackets",
        "Angle brackets < > mark a tag in code and stand in for a word to fill in later, as in <your name>. The plain brackets ( ) are the ones for writing.",
    ),
];
pub(super) const METAPHOR: &str = "A metaphor says a thing is another thing (her laugh was a kettle, the office is a zoo), so the picture does the work without a 'like' in front of it. A simile does put one in: her laugh was like a kettle. Both paint a picture; the metaphor just claims it outright.";
pub(super) const SIMILE: &str = "A simile compares one thing to another with like or as (her laugh was like a kettle, as busy as a bee). A metaphor skips those two words and says the thing is the other one (her laugh was a kettle). Both paint a picture; the simile keeps the comparison out in the open.";
pub(super) const SPEAK_WORDS_0: &[&str] = &[
    "um", "ums", "erm", "er", "filler", "fillers", "like", "y'know",
];
pub(super) const SPEAK_REPLY_0: &str = "At the moment you feel an 'um' coming, close your mouth and breathe instead: a half-second of silence sounds calm and no one notices it, where a filler sounds like doubt. Record thirty seconds of yourself on your phone and count the fillers; most people halve them in a week just by hearing them once.";
pub(super) const SPEAK_WORDS_1: &[&str] = &[
    "confident",
    "confidence",
    "confidently",
    "nervous",
    "nerves",
    "shaking",
    "shaky",
    "wobble",
    "wobbly",
    "wavers",
];
pub(super) const SPEAK_REPLY_1: &str = "Slow down and pause at the full stops rather than at the commas, and look at one person for a whole sentence instead of sweeping the room - both read as confidence even when you are nervous. Write the first thirty seconds word for word and say it aloud five times, because getting started is the part nerves spoil.";
pub(super) const VERSE_METHOD_WORDS: &[&str] = &[
    "advice",
    "beats",
    "begin",
    "beginning",
    "better",
    "difference",
    "example",
    "examples",
    "explain",
    "explained",
    "form",
    "forms",
    "guide",
    "how",
    "howto",
    "improve",
    "learn",
    "lines",
    "method",
    "rhyme",
    "rhymes",
    "rhythm",
    "rule",
    "rules",
    "start",
    "structure",
    "syllables",
    "teach",
    "tip",
    "tips",
    "what",
    "whats",
    "why",
    "written",
];
pub(super) const PEOPLE_PHOTO: &[&str] = &[
    "me",
    "us",
    "my mum",
    "my mom",
    "my dad",
    "my sister",
    "my brother",
    "my friend",
    "my friends",
    "family",
    "the kids",
    "my son",
    "my daughter",
    "my wife",
    "my husband",
    "my partner",
    "my nan",
    "my grandma",
    "my grandad",
];
pub(super) const PHOTO_DETAIL_PEOPLE: &str = "and the hour before anybody started posing";
pub(super) const PHOTO_DETAIL_ANY: &str = "and the part of the day nobody photographs";
pub(super) const CAPTION_DEFAULT: &str = "Say what is in the picture and one thing the picture cannot show, then stop: \"Third attempt at the sourdough - this one actually rose.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.";
pub(super) const CAPTION_BEFORE: &str =
    "Say what is in the picture and one thing the picture cannot show, then stop: \"";
pub(super) const CAPTION_AFTER: &str = ".\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.";
