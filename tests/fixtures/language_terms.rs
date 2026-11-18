//! Original PyBot language guards and explanations at the pinned revision.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub Option<bool>,
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "brackets-punctuation",
        "when should i use brackets in a sentence",
        Some(true),
        None,
    ),
    Case(
        "brackets-punctuation",
        "what are angle brackets",
        Some(true),
        None,
    ),
    Case(
        "brackets-punctuation",
        "how to multiply numbers in brackets",
        Some(false),
        None,
    ),
    Case(
        "brackets-punctuation",
        "what tax bracket am i in",
        Some(false),
        None,
    ),
    Case(
        "brackets-punctuation",
        "have you seen metal wall brackets",
        Some(false),
        None,
    ),
    Case(
        "brackets-punctuation",
        "using punctuation around a number in brackets",
        Some(true),
        None,
    ),
    Case(
        "dash-hyphen",
        "what is an em dash",
        Some(true),
        Some(
            "An em dash is the longest dash — like this — and it marks a break or an aside where a comma would be too quiet. On Windows hold Alt and type 0151; on a Mac it is Shift and Option and the hyphen key.",
        ),
    ),
    Case(
        "dash-hyphen",
        "how do you use an en dash",
        Some(true),
        Some(
            "An en dash is the middle-length dash, and its job is a range or a pair: 10–20 people, the London–Paris train. On a Mac it is Option and the hyphen key; in Word two hyphens turn into one as you type.",
        ),
    ),
    Case(
        "dash-hyphen",
        "a dash of milk",
        Some(false),
        Some(
            "A hyphen joins words into one (well-known, two-year-old) and has no spaces. A dash separates a phrase for emphasis — like this — and British style usually spaces an en dash. Word processors can turn two hyphens into a dash for you.",
        ),
    ),
    Case(
        "dash-hyphen",
        "a dash of something",
        Some(false),
        Some(
            "A hyphen joins words into one (well-known, two-year-old) and has no spaces. A dash separates a phrase for emphasis — like this — and British style usually spaces an en dash. Word processors can turn two hyphens into a dash for you.",
        ),
    ),
    Case(
        "dash-hyphen",
        "what shape is the long dash",
        Some(true),
        Some(
            "An em dash is the longest dash — like this — and it marks a break or an aside where a comma would be too quiet. On Windows hold Alt and type 0151; on a Mac it is Shift and Option and the hyphen key.",
        ),
    ),
    Case(
        "dash-hyphen",
        "when do i need hyphens",
        Some(true),
        Some(
            "A hyphen joins words into one (well-known, two-year-old) and has no spaces. A dash separates a phrase for emphasis — like this — and British style usually spaces an en dash. Word processors can turn two hyphens into a dash for you.",
        ),
    ),
    Case(
        "dash-hyphen",
        "what is a hyphen in a sentence",
        Some(true),
        Some(
            "A hyphen joins words into one (well-known, two-year-old) and has no spaces. A dash separates a phrase for emphasis — like this — and British style usually spaces an en dash. Word processors can turn two hyphens into a dash for you.",
        ),
    ),
    Case(
        "which-plural-of-a-us-word",
        "octopuses or octopi",
        Some(true),
        None,
    ),
    Case(
        "which-plural-of-a-us-word",
        "which is the plural of octopus",
        Some(false),
        None,
    ),
    Case(
        "which-plural-of-a-us-word",
        "is it virii or viruses",
        Some(true),
        None,
    ),
    Case(
        "which-plural-of-a-us-word",
        "these are octopuses",
        Some(false),
        None,
    ),
    Case(
        "meeting-minutes",
        "how to write up the minutes",
        Some(true),
        None,
    ),
    Case(
        "meeting-minutes",
        "the meeting runs for 90 minutes",
        Some(false),
        None,
    ),
    Case(
        "meeting-minutes",
        "it ran for ten minutes how do i write the minutes",
        Some(true),
        None,
    ),
    Case(
        "meeting-minutes",
        "my meeting is ten more minutes",
        Some(false),
        None,
    ),
    Case(
        "meeting-minutes",
        "fifteen mins in total",
        Some(false),
        None,
    ),
    Case("write-a-limerick", "write me a limerick", Some(false), None),
    Case(
        "write-a-limerick",
        "how to write a limerick",
        Some(true),
        None,
    ),
    Case(
        "write-a-limerick",
        "please write a limerick",
        Some(false),
        None,
    ),
    Case(
        "write-a-limerick",
        "help me with the rhythm of a limerick",
        Some(true),
        None,
    ),
    Case(
        "write-a-limerick",
        "can you give me a limerick",
        Some(false),
        None,
    ),
    Case(
        "write-a-limerick",
        "explain the rules of a limerick",
        Some(true),
        None,
    ),
    Case("write-a-limerick", "", Some(true), None),
    Case(
        "metaphor-simile",
        "what is a metaphor",
        None,
        Some(
            "A metaphor says a thing is another thing (her laugh was a kettle, the office is a zoo), so the picture does the work without a 'like' in front of it. A simile does put one in: her laugh was like a kettle. Both paint a picture; the metaphor just claims it outright.",
        ),
    ),
    Case(
        "metaphor-simile",
        "what is a simile",
        None,
        Some(
            "A simile compares one thing to another with like or as (her laugh was like a kettle, as busy as a bee). A metaphor skips those two words and says the thing is the other one (her laugh was a kettle). Both paint a picture; the simile keeps the comparison out in the open.",
        ),
    ),
    Case(
        "metaphor-simile",
        "similes",
        None,
        Some(
            "A simile compares one thing to another with like or as (her laugh was like a kettle, as busy as a bee). A metaphor skips those two words and says the thing is the other one (her laugh was a kettle). Both paint a picture; the simile keeps the comparison out in the open.",
        ),
    ),
    Case(
        "metaphor-simile",
        "what is the difference between a simile and a metaphor",
        None,
        Some(
            "A simile compares with like or as (her laugh was like a kettle), and a metaphor says it is the thing (her laugh was a kettle). Both make a picture; the metaphor just does it without the 'like'.",
        ),
    ),
    Case(
        "metaphor-simile",
        "how to compare",
        None,
        Some(
            "A simile compares with like or as (her laugh was like a kettle), and a metaphor says it is the thing (her laugh was a kettle). Both make a picture; the metaphor just does it without the 'like'.",
        ),
    ),
    Case(
        "figure-of-speech",
        "what is a irony",
        None,
        Some(
            "Irony says the opposite of what is meant and lets the situation do the work: a fire station burning down, or a no-smoking sign behind the smokers. Sarcasm is the spoken, pointed version of it.",
        ),
    ),
    Case(
        "figure-of-speech",
        "what is a onomatopoeia",
        None,
        Some(
            "Onomatopoeia is a word that sounds like the thing it names: splash, buzz, hiss, cuckoo, sizzle. Comics lean on it, and so does most children's poetry.",
        ),
    ),
    Case(
        "figure-of-speech",
        "what is a alliteration",
        None,
        Some(
            "Alliteration repeats the opening sound of nearby words: slippery slope, sweet sixteen, the wild wind. It is about the sound, not the letter - \"know nothing\" is alliteration and \"cute cat\" is not.",
        ),
    ),
    Case(
        "figure-of-speech",
        "what is a personification",
        None,
        Some(
            "Personification gives a thing or an idea human qualities: the wind whispered, the deadline stalked me, the kettle sulked. It turns an abstract thing into something you can picture.",
        ),
    ),
    Case(
        "figure-of-speech",
        "what is a hyperbole",
        None,
        Some(
            "Hyperbole is exaggeration on purpose, not meant literally: I've told you a million times, this bag weighs a ton. It works because both people know it is not true.",
        ),
    ),
    Case(
        "figure-of-speech",
        "what is a euphemism",
        None,
        Some(
            "A euphemism is a soft word for a hard one: passed away for died, let go for sacked, between jobs for unemployed. It spares feelings, and it can hide what happened.",
        ),
    ),
    Case(
        "figure-of-speech",
        "what is a figure of speech",
        None,
        Some(
            "Irony says the opposite of what is meant (a fire station burning down); onomatopoeia is a word that sounds like the thing (splash, buzz); alliteration repeats the opening sound (slippery slope); hyperbole exaggerates on purpose (a million times). Ask me about one and I'll give you its example.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a noun",
        Some(true),
        Some(
            "A noun names a person, place or thing - Jess, Leeds, table, kindness. If you can put 'the' in front of it, it is a noun.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a verb",
        Some(true),
        Some(
            "A verb is the doing or being word: run, eat, seems, is. Every sentence needs one, and the verb changes with the time - I walk today, I walked yesterday.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a adjective",
        Some(true),
        Some(
            "An adjective describes a noun: a red bus, a tired dog, an honest answer. It sits before the noun or after 'is' - the bus is red.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a adverb",
        Some(true),
        Some(
            "An adverb says how, when or where something happened: she sang loudly, we left early, he waited outside. Many end in -ly, but not all do (soon, well, fast).",
        ),
    ),
    Case(
        "grammar-term",
        "what is a pronoun",
        Some(true),
        Some(
            "A pronoun stands in for a noun so you are not repeating it: I, you, he, she, it, we, they, me, him, her, them. 'Sam lost Sam's keys' is why we have them.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a preposition",
        Some(true),
        Some(
            "A preposition says where or when in relation to something: in, on, under, between, before, after, with. The cat sat on the mat - 'on' is the preposition.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a conjunction",
        Some(true),
        Some(
            "A conjunction joins words or clauses: and, but, because, although, so. 'I was late because the bus broke down' joins two ideas with 'because'.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a tense",
        Some(true),
        Some(
            "Tense is when the verb happens: past (I cooked), present (I cook, I am cooking), future (I will cook). Pick one and keep to it within a sentence.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a clause",
        Some(true),
        Some(
            "A clause is a group of words with its own verb and subject: 'when the bus came' and 'we got on'. A sentence is one or more clauses; a phrase has no verb at all.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a sentence",
        Some(true),
        Some(
            "A sentence needs a subject and a verb and makes one complete point, starting with a capital and ending with a full stop: 'The bus was late.'",
        ),
    ),
    Case(
        "grammar-term",
        "what is a fronted adverbial",
        Some(true),
        Some(
            "A fronted adverbial is the phrase that opens a sentence and says when, where or how: 'After lunch, we walked.' 'Slowly, the door opened.' Put a comma after it, and make sure the thing it describes is the subject of the sentence - 'Walking home, the rain started' says the rain was walking.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a split infinitive",
        Some(true),
        Some(
            "A split infinitive puts a word between to and the verb: 'to boldly go'. It is not an error, whatever you were taught - split it when it reads better and move the word when it gets clumsy: 'to go boldly'.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a dangling modifier",
        Some(true),
        Some(
            "A dangling modifier is an opening phrase that describes the wrong thing: 'Walking to work, the rain soaked me' says the rain was walking. Name the person straight after the comma - 'Walking to work, I was soaked by the rain' - or turn the phrase into a clause.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a whom",
        Some(true),
        Some(
            "Use whom after a preposition and where it is the one being acted on: 'to whom it may concern', 'the person whom I met'. If you can say he or she, write who; if you would say him or her, write whom. In speech who nearly always sounds right.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a adverbial",
        Some(true),
        Some(
            "An adverbial is any phrase that says when, where, how or why: 'at six', 'in the kitchen', 'very slowly', 'because it rained'. It is not always an adverb - it can be a whole phrase or clause.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a apostrophe",
        Some(true),
        Some(
            "An apostrophe does two jobs: it marks something missing (do not becomes don't) or shows belonging (Jess's coat). It never makes a plain plural - two cats, not two cat's.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a hashtag",
        Some(true),
        Some(
            "A hashtag is the # sign used as a label: #slowcooking groups posts about one thing. On a UK keyboard it is Shift and 3, and on a phone it is on the symbols page.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a number sign",
        Some(true),
        Some(
            "The number sign is # - Shift and 3 on a UK keyboard. In the US it is how you write a number (No. 5) and online it starts a hashtag.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a ampersand",
        Some(true),
        Some(
            "An ampersand is the & sign and it means 'and'. Write and out in a sentence; the sign is for names and headings, like Marks & Spencer or R&D.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a at sign",
        Some(true),
        Some(
            "The at sign is @ - Shift and 2 on a UK keyboard - and it is in every email address: name@example.com.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a asterisk",
        Some(true),
        Some(
            "An asterisk is the * sign, a small star. It marks a footnote or shows anything goes (*), and on a phone keyboard it is usually behind the ?123 key.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a slash",
        Some(true),
        Some(
            "A slash is the / sign, and it mostly means 'or' (and/or) or separates the parts of a date (12/05/2024). A backslash \\ is the other way over and turns up in computer paths rather than in writing.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a backslash",
        Some(true),
        Some(
            "A backslash is the \\ sign, the mirror of a slash. It almost never belongs in writing - it separates folders in a Windows file path, like C:\\Users\\me.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a underscore",
        Some(true),
        Some(
            "An underscore is the _ sign, the low line under a letter. It joins words in file names and usernames (my_notes.txt); a hyphen is the shorter mark at half height.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a tilde",
        Some(true),
        Some(
            "A tilde is the ~ sign, the small wavy line. It means 'about' (~20 people came), and on most keyboards it is Shift and the hash key.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a pipe",
        Some(true),
        Some(
            "A pipe is the | sign, the upright bar. Most people call it the pipe or the vertical bar, and it separates options in a computer command; in writing a slash does the job better.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a caret",
        Some(true),
        Some(
            "A caret is the ^ sign, a small wedge above the line. In proofreading it marks where a word is to be slipped in, and in maths it means to the power of - 2^3 is eight.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a bullet point",
        Some(true),
        Some(
            "A bullet point is the dot that starts each item in a list. Keep every item to a line, start each one the same way, and leave the full stops off short items.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a curly bracket",
        Some(true),
        Some(
            "Curly brackets { } group things in maths and code, and square brackets [ ] hold your own words inside someone else's quotation. Round brackets ( ) take an aside the sentence could do without.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a square bracket",
        Some(true),
        Some(
            "Square brackets [ ] hold your own words inside a quotation, and they mark a missing word in a transcript: 'she said it was [unclear] on the tape'.",
        ),
    ),
    Case(
        "grammar-term",
        "what is a angle bracket",
        Some(true),
        Some(
            "Angle brackets < > mark a tag in code and stand in for a word to fill in later, as in <your name>. The plain brackets ( ) are the ones for writing.",
        ),
    ),
    Case(
        "grammar-term",
        "what is an adverb",
        Some(true),
        Some(
            "An adverb says how, when or where something happened: she sang loudly, we left early, he waited outside. Many end in -ly, but not all do (soon, well, fast).",
        ),
    ),
    Case(
        "grammar-term",
        "what are nouns",
        Some(true),
        Some(
            "A noun names a person, place or thing - Jess, Leeds, table, kindness. If you can put 'the' in front of it, it is a noun.",
        ),
    ),
    Case(
        "grammar-term",
        "why am i forgetting easy words",
        Some(false),
        Some(
            "The parts of a sentence, in the order they turn up: a noun names something, a verb is what it does or is, an adjective describes it, an adverb says how the doing happened, and a pronoun stands in for the noun. 'The tired dog slept badly' - dog is the noun, tired the adjective, slept the verb, badly the adverb.",
        ),
    ),
    Case(
        "grammar-term",
        "what is grammar",
        Some(true),
        Some(
            "The parts of a sentence, in the order they turn up: a noun names something, a verb is what it does or is, an adjective describes it, an adverb says how the doing happened, and a pronoun stands in for the noun. 'The tired dog slept badly' - dog is the noun, tired the adjective, slept the verb, badly the adverb.",
        ),
    ),
    Case(
        "grammar-term",
        "what does pronouns mean",
        Some(true),
        Some(
            "A pronoun stands in for a noun so you are not repeating it: I, you, he, she, it, we, they, me, him, her, them. 'Sam lost Sam's keys' is why we have them.",
        ),
    ),
    Case(
        "poem-form",
        "what is a sonnet",
        None,
        Some(
            "A sonnet is fourteen lines in iambic pentameter - ten syllables a line, stress on every second one. Shakespeare's kind rhymes in three quatrains and a closing couplet (abab cdcd efef gg) and turns at line nine; the Italian kind splits six and eight.",
        ),
    ),
    Case(
        "poem-form",
        "what is a stanza",
        None,
        Some(
            "A stanza is a group of lines set apart in a poem, the way a paragraph is in prose. Two lines are a couplet, four a quatrain, and the pattern of rhymes across them is the rhyme scheme.",
        ),
    ),
    Case(
        "poem-form",
        "what is a free verse",
        None,
        Some(
            "Free verse is poetry with no fixed metre and no rhyme: the line breaks and the rhythm of the sentences do the work instead. Most modern poetry is written this way.",
        ),
    ),
    Case(
        "poem-form",
        "what is a iambic pentameter",
        None,
        Some(
            "Iambic pentameter is five beats of da-DUM to the line - ten syllables with the stress on every second one: 'Shall I compare thee to a summer's day'. Blank verse is that without rhyme, which is what most of Shakespeare's plays are in.",
        ),
    ),
    Case(
        "poem-form",
        "what is a couplet",
        None,
        Some(
            "A couplet is two lines that rhyme and usually stand together: 'Good night, good night! Parting is such sweet sorrow / That I shall say good night till it be morrow.'",
        ),
    ),
    Case(
        "poem-form",
        "what is a ballad",
        None,
        Some(
            "A ballad is a story told in short verses, meant to be sung or recited: four-line stanzas that rhyme abab or abcb, with a refrain that comes back. The old folk ballads are the kind that run to a dozen verses, and pop still uses the name for a slow song about one feeling.",
        ),
    ),
    Case(
        "poem-form",
        "tell me the form of a poem",
        None,
        Some(
            "A haiku is three lines of five, seven and five sounds; a limerick is five lines rhyming aabba with a joke in it; a sonnet is fourteen lines in iambic pentameter. Ask about one and I'll give you its shape.",
        ),
    ),
    Case(
        "poem-form",
        "does a haiku have to rhyme",
        None,
        Some(
            "A haiku is three lines of five, seven and five sounds; a limerick is five lines rhyming aabba with a joke in it; a sonnet is fourteen lines in iambic pentameter. Ask about one and I'll give you its shape.",
        ),
    ),
    Case(
        "speak-clearly",
        "how do i speak more clearly",
        None,
        Some(
            "Slow to about two-thirds of your usual speed and finish the ends of words, which is most of what 'clear' means. Pause at the full stops instead of running the sentences together, drop your voice a little at the end of a sentence rather than letting it rise, and if you are on a call, stand up - it makes the voice carry.",
        ),
    ),
    Case(
        "speak-clearly",
        "how do i stop saying um",
        None,
        Some(
            "At the moment you feel an 'um' coming, close your mouth and breathe instead: a half-second of silence sounds calm and no one notices it, where a filler sounds like doubt. Record thirty seconds of yourself on your phone and count the fillers; most people halve them in a week just by hearing them once.",
        ),
    ),
    Case(
        "speak-clearly",
        "how do i speak more confidently",
        None,
        Some(
            "Slow down and pause at the full stops rather than at the commas, and look at one person for a whole sentence instead of sweeping the room - both read as confidence even when you are nervous. Write the first thirty seconds word for word and say it aloud five times, because getting started is the part nerves spoil.",
        ),
    ),
    Case(
        "speak-clearly",
        "my voice sounds nervous",
        None,
        Some(
            "Slow down and pause at the full stops rather than at the commas, and look at one person for a whole sentence instead of sweeping the room - both read as confidence even when you are nervous. Write the first thirty seconds word for word and say it aloud five times, because getting started is the part nerves spoil.",
        ),
    ),
    Case(
        "speak-clearly",
        "speaking slowly",
        None,
        Some(
            "Slow to about two-thirds of your usual speed and finish the ends of words, which is most of what 'clear' means. Pause at the full stops instead of running the sentences together, drop your voice a little at the end of a sentence rather than letting it rise, and if you are on a call, stand up - it makes the voice carry.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the wavy",
        None,
        Some(
            "That is a tilde, ~. It means 'about' (~20 people came), and on most keyboards it is Shift and the hash key.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the low line",
        None,
        Some(
            "That is an underscore, _. It joins words in file names and usernames (my_notes.txt); a hyphen is the shorter mark at half height.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the upright",
        None,
        Some(
            "That is a pipe, |, also called the vertical bar. It separates options in a computer command; in writing a slash does the job better.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the star",
        None,
        Some(
            "That is an asterisk, *. It marks a footnote, and on a phone keyboard it hides behind the ?123 key.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the wedge",
        None,
        Some(
            "That is a caret, ^. It marks where a word is to be slipped in, and in maths it means to the power of - 2^3 is eight.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the backwards",
        None,
        Some(
            "The one leaning the other way is a backslash, \\. It separates folders in a Windows file path like C:\\Users\\me, and it almost never belongs in writing.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the over the 2",
        None,
        Some(
            "That is the at sign, @ - Shift and 2 on a UK keyboard, and it is in every email address.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the over the 3",
        None,
        Some(
            "That is the hash, # - Shift and 3 on a UK keyboard. On a phone it starts a hashtag, and in the US it labels a number.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the dot",
        None,
        Some(
            "Three dots in a row are an ellipsis (...), and it means something has been left out or the thought trails off. A single dot is a full stop.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the squarish",
        None,
        Some(
            "Square brackets are [ ], and they hold your own words inside a quotation or a gap in a transcript.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the curly",
        None,
        Some(
            "Curly brackets are { }, and they group things in maths and in code. Round brackets ( ) take an aside the sentence could do without.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is the sideways v",
        None,
        Some(
            "Angle brackets are < >. They mark a tag in code and stand in for a word to fill in later, as in <your name>.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what's this #",
        None,
        Some(
            "A hashtag is the # sign used as a label: #slowcooking groups posts about one thing. On a UK keyboard it is Shift and 3, and on a phone it is on the symbols page.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is @",
        None,
        Some(
            "The at sign is @ - Shift and 2 on a UK keyboard - and it is in every email address: name@example.com.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is a pipe",
        None,
        Some(
            "A pipe is the | sign, the upright bar. Most people call it the pipe or the vertical bar, and it separates options in a computer command; in writing a slash does the job better.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is this *",
        None,
        Some(
            "An asterisk is the * sign, a small star. It marks a footnote or shows anything goes (*), and on a phone keyboard it is usually behind the ?123 key.",
        ),
    ),
    Case(
        "mark-unnamed",
        "what is this symbol",
        None,
        Some(
            "Tell me which key it is on and I will name it: the tall line on the left of the keyboard is the pipe (|), the mark over 2 is the at sign (@), over 3 the hash (#), and the wavy one on the left is a tilde (~). If you can paste it into a line, I can read it too.",
        ),
    ),
    Case(
        "write-a-caption",
        "write a caption",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"Third attempt at the sourdough - this one actually rose.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "write a caption for a photo of my dog in the snow",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"My dog in the snow, and that look it gives you just before it decides not to listen.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "a caption for a picture of the cake i made",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"The cake I made, and it did not last the afternoon.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "my family at home",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"Third attempt at the sourdough - this one actually rose.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "a caption for a shot of my sister",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"My sister, and the hour before anybody started posing.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "caption for an image showing sunrise",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"Sunrise, and nobody else was awake yet.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "photo of my bike",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"My bike, and the part of the day nobody photographs.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "photo of the weird long tail part of a toy dinosaur",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"Third attempt at the sourdough - this one actually rose.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "photo of us from the reunion",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"Us, and the hour before anybody started posing.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "write-a-caption",
        "a caption for a photo of that",
        None,
        Some(
            "Say what is in the picture and one thing the picture cannot show, then stop: \"Third attempt at the sourdough - this one actually rose.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of walk",
        Some(true),
        Some(
            "Walk -> walked. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of hope",
        Some(true),
        Some(
            "Hope -> hoped. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of carry",
        Some(true),
        Some(
            "Carry -> carried. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of play",
        Some(true),
        Some(
            "Play -> played. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of stop",
        Some(true),
        Some(
            "Stop -> stopped. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of visit",
        Some(true),
        Some(
            "Visit -> visited. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of open",
        Some(true),
        Some(
            "Open -> opened. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of travel",
        Some(true),
        Some(
            "Travel -> travelled. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of cancel",
        Some(true),
        Some(
            "Cancel -> cancelled. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of stir",
        Some(true),
        Some(
            "Stir -> stirred. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of box",
        Some(true),
        Some(
            "Box -> boxed. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of lie",
        Some(true),
        Some(
            "Lie is two verbs. Lying down is lie, lay, lain: 'I lay on the bed', 'I have lain there'. Telling a lie is lie, lied, lied: 'I lied to him'. The one people get wrong is the reclining one - 'I laid on the bed' should be 'I lay on the bed', because laid is the past of lay, meaning to put something down.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of lay",
        Some(true),
        Some(
            "Lay is laid, laid: 'I laid the table', 'I have laid it out'. The trap is that lay is also the past of lie, the lying-down verb - so 'I lay on the bed' is correct for reclining, and 'I laid the baby on the bed' is correct for putting her down. If you can replace it with put, use laid.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of buy",
        Some(false),
        Some(
            "The past tense of buy is bought, and the past participle is the same: 'I bought it yesterday, I have bought it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of run",
        Some(false),
        Some(
            "The past tense of run is ran - 'I ran yesterday' - and the past participle is run, the one that goes with have: 'I have run'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "regular-past",
        "what is the past tense of flibbertigibbet",
        Some(true),
        Some(
            "I don't know a verb 'flibbertigibbet'. If it is a name or a made-up word the -ed rule still gives flibbertigibbeted, and if it is a typo the nearest real verb is the one to take.",
        ),
    ),
    Case("regular-past", "help with past tense", Some(false), None),
    Case(
        "regular-past",
        "what is the past participle of accept",
        Some(true),
        Some(
            "Accept -> accepted. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "regular-past",
        "past form of dine",
        Some(true),
        Some(
            "Dine -> dined. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of be",
        Some(true),
        Some(
            "The past tense of be is was - 'I was yesterday' - and the past participle is been, the one that goes with have: 'I have been'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of become",
        Some(true),
        Some(
            "The past tense of become is became - 'I became yesterday' - and the past participle is become, the one that goes with have: 'I have become'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of begin",
        Some(true),
        Some(
            "The past tense of begin is began - 'I began yesterday' - and the past participle is begun, the one that goes with have: 'I have begun'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of break",
        Some(true),
        Some(
            "The past tense of break is broke - 'I broke yesterday' - and the past participle is broken, the one that goes with have: 'I have broken'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of bring",
        Some(true),
        Some(
            "The past tense of bring is brought, and the past participle is the same: 'I brought it yesterday, I have brought it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of build",
        Some(true),
        Some(
            "The past tense of build is built, and the past participle is the same: 'I built it yesterday, I have built it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of buy",
        Some(true),
        Some(
            "The past tense of buy is bought, and the past participle is the same: 'I bought it yesterday, I have bought it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of catch",
        Some(true),
        Some(
            "The past tense of catch is caught, and the past participle is the same: 'I caught it yesterday, I have caught it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of choose",
        Some(true),
        Some(
            "The past tense of choose is chose - 'I chose yesterday' - and the past participle is chosen, the one that goes with have: 'I have chosen'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of come",
        Some(true),
        Some(
            "The past tense of come is came - 'I came yesterday' - and the past participle is come, the one that goes with have: 'I have come'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of do",
        Some(true),
        Some(
            "The past tense of do is did - 'I did yesterday' - and the past participle is done, the one that goes with have: 'I have done'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of drink",
        Some(true),
        Some(
            "The past tense of drink is drank - 'I drank yesterday' - and the past participle is drunk, the one that goes with have: 'I have drunk'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of drive",
        Some(true),
        Some(
            "The past tense of drive is drove - 'I drove yesterday' - and the past participle is driven, the one that goes with have: 'I have driven'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of eat",
        Some(true),
        Some(
            "The past tense of eat is ate - 'I ate yesterday' - and the past participle is eaten, the one that goes with have: 'I have eaten'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of fall",
        Some(true),
        Some(
            "The past tense of fall is fell - 'I fell yesterday' - and the past participle is fallen, the one that goes with have: 'I have fallen'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of feel",
        Some(true),
        Some(
            "The past tense of feel is felt, and the past participle is the same: 'I felt it yesterday, I have felt it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of fight",
        Some(true),
        Some(
            "The past tense of fight is fought, and the past participle is the same: 'I fought it yesterday, I have fought it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of find",
        Some(true),
        Some(
            "The past tense of find is found, and the past participle is the same: 'I found it yesterday, I have found it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of flee",
        Some(true),
        Some(
            "The past tense of flee is fled, and the past participle is the same: 'I fled it yesterday, I have fled it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of fly",
        Some(true),
        Some(
            "The past tense of fly is flew - 'I flew yesterday' - and the past participle is flown, the one that goes with have: 'I have flown'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of forget",
        Some(true),
        Some(
            "The past tense of forget is forgot - 'I forgot yesterday' - and the past participle is forgotten, the one that goes with have: 'I have forgotten'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of get",
        Some(true),
        Some(
            "The past tense of get is got, and the past participle is the same: 'I got it yesterday, I have got it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of give",
        Some(true),
        Some(
            "The past tense of give is gave - 'I gave yesterday' - and the past participle is given, the one that goes with have: 'I have given'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of go",
        Some(true),
        Some(
            "The past tense of go is went - 'I went yesterday' - and the past participle is gone, the one that goes with have: 'I have gone'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of grow",
        Some(true),
        Some(
            "The past tense of grow is grew - 'I grew yesterday' - and the past participle is grown, the one that goes with have: 'I have grown'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of have",
        Some(true),
        Some(
            "The past tense of have is had, and the past participle is the same: 'I had it yesterday, I have had it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of hear",
        Some(true),
        Some(
            "The past tense of hear is heard, and the past participle is the same: 'I heard it yesterday, I have heard it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of hide",
        Some(true),
        Some(
            "The past tense of hide is hid - 'I hid yesterday' - and the past participle is hidden, the one that goes with have: 'I have hidden'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of hold",
        Some(true),
        Some(
            "The past tense of hold is held, and the past participle is the same: 'I held it yesterday, I have held it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of keep",
        Some(true),
        Some(
            "The past tense of keep is kept, and the past participle is the same: 'I kept it yesterday, I have kept it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of know",
        Some(true),
        Some(
            "The past tense of know is knew - 'I knew yesterday' - and the past participle is known, the one that goes with have: 'I have known'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of leave",
        Some(true),
        Some(
            "The past tense of leave is left, and the past participle is the same: 'I left it yesterday, I have left it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of lose",
        Some(true),
        Some(
            "The past tense of lose is lost, and the past participle is the same: 'I lost it yesterday, I have lost it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of make",
        Some(true),
        Some(
            "The past tense of make is made, and the past participle is the same: 'I made it yesterday, I have made it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of meet",
        Some(true),
        Some(
            "The past tense of meet is met, and the past participle is the same: 'I met it yesterday, I have met it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of pay",
        Some(true),
        Some(
            "The past tense of pay is paid, and the past participle is the same: 'I paid it yesterday, I have paid it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of put",
        Some(true),
        Some(
            "The past tense of put is put, and the past participle is the same: 'I put it yesterday, I have put it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of read",
        Some(true),
        Some(
            "The past tense of read is read, and the past participle is the same: 'I read it yesterday, I have read it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of ride",
        Some(true),
        Some(
            "The past tense of ride is rode - 'I rode yesterday' - and the past participle is ridden, the one that goes with have: 'I have ridden'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of ring",
        Some(true),
        Some(
            "The past tense of ring is rang - 'I rang yesterday' - and the past participle is rung, the one that goes with have: 'I have rung'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of rise",
        Some(true),
        Some(
            "The past tense of rise is rose - 'I rose yesterday' - and the past participle is risen, the one that goes with have: 'I have risen'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of run",
        Some(true),
        Some(
            "The past tense of run is ran - 'I ran yesterday' - and the past participle is run, the one that goes with have: 'I have run'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of say",
        Some(true),
        Some(
            "The past tense of say is said, and the past participle is the same: 'I said it yesterday, I have said it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of see",
        Some(true),
        Some(
            "The past tense of see is saw - 'I saw yesterday' - and the past participle is seen, the one that goes with have: 'I have seen'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of sell",
        Some(true),
        Some(
            "The past tense of sell is sold, and the past participle is the same: 'I sold it yesterday, I have sold it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of send",
        Some(true),
        Some(
            "The past tense of send is sent, and the past participle is the same: 'I sent it yesterday, I have sent it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of sing",
        Some(true),
        Some(
            "The past tense of sing is sang - 'I sang yesterday' - and the past participle is sung, the one that goes with have: 'I have sung'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of sit",
        Some(true),
        Some(
            "The past tense of sit is sat, and the past participle is the same: 'I sat it yesterday, I have sat it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of sleep",
        Some(true),
        Some(
            "The past tense of sleep is slept, and the past participle is the same: 'I slept it yesterday, I have slept it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of speak",
        Some(true),
        Some(
            "The past tense of speak is spoke - 'I spoke yesterday' - and the past participle is spoken, the one that goes with have: 'I have spoken'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of spend",
        Some(true),
        Some(
            "The past tense of spend is spent, and the past participle is the same: 'I spent it yesterday, I have spent it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of stand",
        Some(true),
        Some(
            "The past tense of stand is stood, and the past participle is the same: 'I stood it yesterday, I have stood it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of steal",
        Some(true),
        Some(
            "The past tense of steal is stole - 'I stole yesterday' - and the past participle is stolen, the one that goes with have: 'I have stolen'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of swim",
        Some(true),
        Some(
            "The past tense of swim is swam - 'I swam yesterday' - and the past participle is swum, the one that goes with have: 'I have swum'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of take",
        Some(true),
        Some(
            "The past tense of take is took - 'I took yesterday' - and the past participle is taken, the one that goes with have: 'I have taken'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of teach",
        Some(true),
        Some(
            "The past tense of teach is taught, and the past participle is the same: 'I taught it yesterday, I have taught it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of tell",
        Some(true),
        Some(
            "The past tense of tell is told, and the past participle is the same: 'I told it yesterday, I have told it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of think",
        Some(true),
        Some(
            "The past tense of think is thought, and the past participle is the same: 'I thought it yesterday, I have thought it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of throw",
        Some(true),
        Some(
            "The past tense of throw is threw - 'I threw yesterday' - and the past participle is thrown, the one that goes with have: 'I have thrown'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of understand",
        Some(true),
        Some(
            "The past tense of understand is understood, and the past participle is the same: 'I understood it yesterday, I have understood it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of wake",
        Some(true),
        Some(
            "The past tense of wake is woke - 'I woke yesterday' - and the past participle is woken, the one that goes with have: 'I have woken'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of wear",
        Some(true),
        Some(
            "The past tense of wear is wore - 'I wore yesterday' - and the past participle is worn, the one that goes with have: 'I have worn'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of win",
        Some(true),
        Some(
            "The past tense of win is won, and the past participle is the same: 'I won it yesterday, I have won it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of write",
        Some(true),
        Some(
            "The past tense of write is wrote - 'I wrote yesterday' - and the past participle is written, the one that goes with have: 'I have written'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of bet",
        Some(true),
        Some(
            "The past tense of bet is bet, and the past participle is the same: 'I bet it yesterday, I have bet it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of burst",
        Some(true),
        Some(
            "The past tense of burst is burst, and the past participle is the same: 'I burst it yesterday, I have burst it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of cast",
        Some(true),
        Some(
            "The past tense of cast is cast, and the past participle is the same: 'I cast it yesterday, I have cast it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of cost",
        Some(true),
        Some(
            "The past tense of cost is cost, and the past participle is the same: 'I cost it yesterday, I have cost it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of cut",
        Some(true),
        Some(
            "The past tense of cut is cut, and the past participle is the same: 'I cut it yesterday, I have cut it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of hit",
        Some(true),
        Some(
            "The past tense of hit is hit, and the past participle is the same: 'I hit it yesterday, I have hit it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of hurt",
        Some(true),
        Some(
            "The past tense of hurt is hurt, and the past participle is the same: 'I hurt it yesterday, I have hurt it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of let",
        Some(true),
        Some(
            "The past tense of let is let, and the past participle is the same: 'I let it yesterday, I have let it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of quit",
        Some(true),
        Some(
            "The past tense of quit is quit, and the past participle is the same: 'I quit it yesterday, I have quit it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of set",
        Some(true),
        Some(
            "The past tense of set is set, and the past participle is the same: 'I set it yesterday, I have set it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of shut",
        Some(true),
        Some(
            "The past tense of shut is shut, and the past participle is the same: 'I shut it yesterday, I have shut it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of split",
        Some(true),
        Some(
            "The past tense of split is split, and the past participle is the same: 'I split it yesterday, I have split it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "what is the past tense of spread",
        Some(true),
        Some(
            "The past tense of spread is spread, and the past participle is the same: 'I spread it yesterday, I have spread it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "can you tell me the past tense of swim",
        Some(true),
        Some(
            "The past tense of swim is swam - 'I swam yesterday' - and the past participle is swum, the one that goes with have: 'I have swum'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.",
        ),
    ),
    Case(
        "irregular-past",
        "do you know the past form of stop",
        Some(false),
        None,
    ),
    Case(
        "plural-word",
        "what is the plural of crisis",
        Some(true),
        Some(
            "Crises is the plural of crisis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of analysis",
        Some(true),
        Some(
            "Analyses is the plural of analysis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of thesis",
        Some(true),
        Some(
            "Theses is the plural of thesis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of basis",
        Some(true),
        Some(
            "Bases is the plural of basis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of axis",
        Some(true),
        Some(
            "Axes is the plural of axis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of oasis",
        Some(true),
        Some(
            "Oases is the plural of oasis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of diagnosis",
        Some(true),
        Some(
            "Diagnoses is the plural of diagnosis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of prognosis",
        Some(true),
        Some(
            "Prognoses is the plural of prognosis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hypothesis",
        Some(true),
        Some(
            "Hypotheses is the plural of hypothesis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of synopsis",
        Some(true),
        Some(
            "Synopses is the plural of synopsis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of emphasis",
        Some(true),
        Some(
            "Emphases is the plural of emphasis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of parenthesis",
        Some(true),
        Some(
            "Parentheses is the plural of parenthesis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of paralysis",
        Some(true),
        Some(
            "Paralyses is the plural of paralysis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of ellipsis",
        Some(true),
        Some(
            "Ellipses is the plural of ellipsis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of neurosis",
        Some(true),
        Some(
            "Neuroses is the plural of neurosis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of metamorphosis",
        Some(true),
        Some(
            "Metamorphoses is the plural of metamorphosis. The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of child",
        Some(true),
        Some(
            "Children is the plural of child. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of person",
        Some(true),
        Some(
            "People is the plural of person. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of man",
        Some(true),
        Some(
            "Men is the plural of man. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of woman",
        Some(true),
        Some(
            "Women is the plural of woman. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of tooth",
        Some(true),
        Some(
            "Teeth is the plural of tooth. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of foot",
        Some(true),
        Some(
            "Feet is the plural of foot. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mouse",
        Some(true),
        Some(
            "Mice is the plural of mouse. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of goose",
        Some(true),
        Some(
            "Geese is the plural of goose. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of ox",
        Some(true),
        Some(
            "Oxen is the plural of ox. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of louse",
        Some(true),
        Some(
            "Lice is the plural of louse. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of policeman",
        Some(true),
        Some(
            "Policemen is the plural of policeman. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of fireman",
        Some(true),
        Some(
            "Firemen is the plural of fireman. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of spokesman",
        Some(true),
        Some(
            "Spokesmen is the plural of spokesman. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of chairman",
        Some(true),
        Some(
            "Chairmen is the plural of chairman. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sheep",
        Some(true),
        Some(
            "Sheep is its own plural - the same word for one and for many: one sheep, two sheep. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of deer",
        Some(true),
        Some(
            "Deer is its own plural - the same word for one and for many: one deer, two deer. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of fish",
        Some(true),
        Some(
            "Fish is its own plural - the same word for one and for many: one fish, two fish. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of trout",
        Some(true),
        Some(
            "Trout is its own plural - the same word for one and for many: one trout, two trout. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of salmon",
        Some(true),
        Some(
            "Salmon is its own plural - the same word for one and for many: one salmon, two salmon. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of aircraft",
        Some(true),
        Some(
            "Aircraft is its own plural - the same word for one and for many: one aircraft, two aircraft. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of series",
        Some(true),
        Some(
            "Series is its own plural - the same word for one and for many: one series, two series. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of species",
        Some(true),
        Some(
            "Species is its own plural - the same word for one and for many: one species, two species. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of offspring",
        Some(true),
        Some(
            "Offspring is its own plural - the same word for one and for many: one offspring, two offspring. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of headquarters",
        Some(true),
        Some(
            "Headquarters is its own plural - the same word for one and for many: one headquarters, two headquarters. Deer, fish, trout, aircraft and series work the same way: the word does not change at all.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of knife",
        Some(true),
        Some(
            "Knives is the plural of knife. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of wife",
        Some(true),
        Some(
            "Wives is the plural of wife. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of life",
        Some(true),
        Some(
            "Lives is the plural of life. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of leaf",
        Some(true),
        Some(
            "Leaves is the plural of leaf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of half",
        Some(true),
        Some(
            "Halves is the plural of half. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of loaf",
        Some(true),
        Some(
            "Loaves is the plural of loaf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of shelf",
        Some(true),
        Some(
            "Shelves is the plural of shelf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of thief",
        Some(true),
        Some(
            "Thieves is the plural of thief. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of wolf",
        Some(true),
        Some(
            "Wolves is the plural of wolf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of calf",
        Some(true),
        Some(
            "Calves is the plural of calf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of scarf",
        Some(true),
        Some(
            "Scarves is the plural of scarf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hoof",
        Some(true),
        Some(
            "Hooves is the plural of hoof. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of self",
        Some(true),
        Some(
            "Selves is the plural of self. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of elf",
        Some(true),
        Some(
            "Elves is the plural of elf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of wharf",
        Some(true),
        Some(
            "Wharves is the plural of wharf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sheaf",
        Some(true),
        Some(
            "Sheaves is the plural of sheaf. The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of potato",
        Some(true),
        Some(
            "Potatoes is the plural of potato. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of tomato",
        Some(true),
        Some(
            "Tomatoes is the plural of tomato. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hero",
        Some(true),
        Some(
            "Heroes is the plural of hero. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of echo",
        Some(true),
        Some(
            "Echoes is the plural of echo. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of veto",
        Some(true),
        Some(
            "Vetoes is the plural of veto. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of torpedo",
        Some(true),
        Some(
            "Torpedoes is the plural of torpedo. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of volcano",
        Some(true),
        Some(
            "Volcanoes is the plural of volcano. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mosquito",
        Some(true),
        Some(
            "Mosquitoes is the plural of mosquito. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of buffalo",
        Some(true),
        Some(
            "Buffaloes is the plural of buffalo. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of domino",
        Some(true),
        Some(
            "Dominoes is the plural of domino. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of cargo",
        Some(true),
        Some(
            "Cargoes is the plural of cargo. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mango",
        Some(true),
        Some(
            "Mangoes is the plural of mango. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bus",
        Some(true),
        Some(
            "Buses is the plural of bus. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of box",
        Some(true),
        Some(
            "Boxes is the plural of box. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of church",
        Some(true),
        Some(
            "Churches is the plural of church. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of dish",
        Some(true),
        Some(
            "Dishes is the plural of dish. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of glass",
        Some(true),
        Some(
            "Glasses is the plural of glass. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of class",
        Some(true),
        Some(
            "Classes is the plural of class. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of watch",
        Some(true),
        Some(
            "Watches is the plural of watch. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of brush",
        Some(true),
        Some(
            "Brushes is the plural of brush. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bench",
        Some(true),
        Some(
            "Benches is the plural of bench. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of branch",
        Some(true),
        Some(
            "Branches is the plural of branch. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of match",
        Some(true),
        Some(
            "Matches is the plural of match. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of quiz",
        Some(true),
        Some(
            "Quizzes is the plural of quiz. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of fox",
        Some(true),
        Some(
            "Foxes is the plural of fox. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of dress",
        Some(true),
        Some(
            "Dresses is the plural of dress. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of wish",
        Some(true),
        Some(
            "Wishes is the plural of wish. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of kiss",
        Some(true),
        Some(
            "Kisses is the plural of kiss. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bunch",
        Some(true),
        Some(
            "Bunches is the plural of bunch. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of boss",
        Some(true),
        Some(
            "Bosses is the plural of boss. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of city",
        Some(true),
        Some(
            "Cities is the plural of city. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of baby",
        Some(true),
        Some(
            "Babies is the plural of baby. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of story",
        Some(true),
        Some(
            "Stories is the plural of story. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of country",
        Some(true),
        Some(
            "Countries is the plural of country. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of party",
        Some(true),
        Some(
            "Parties is the plural of party. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of family",
        Some(true),
        Some(
            "Families is the plural of family. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of company",
        Some(true),
        Some(
            "Companies is the plural of company. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of lady",
        Some(true),
        Some(
            "Ladies is the plural of lady. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of puppy",
        Some(true),
        Some(
            "Puppies is the plural of puppy. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of enemy",
        Some(true),
        Some(
            "Enemies is the plural of enemy. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hobby",
        Some(true),
        Some(
            "Hobbies is the plural of hobby. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of diary",
        Some(true),
        Some(
            "Diaries is the plural of diary. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of memory",
        Some(true),
        Some(
            "Memories is the plural of memory. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of library",
        Some(true),
        Some(
            "Libraries is the plural of library. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of army",
        Some(true),
        Some(
            "Armies is the plural of army. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of factory",
        Some(true),
        Some(
            "Factories is the plural of factory. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of century",
        Some(true),
        Some(
            "Centuries is the plural of century. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of salary",
        Some(true),
        Some(
            "Salaries is the plural of salary. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of berry",
        Some(true),
        Some(
            "Berries is the plural of berry. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of supply",
        Some(true),
        Some(
            "Supplies is the plural of supply. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sky",
        Some(true),
        Some(
            "Skies is the plural of sky. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of fly",
        Some(true),
        Some(
            "Flies is the plural of fly. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of penny",
        Some(true),
        Some(
            "Pennies is the plural of penny. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of trolley",
        Some(true),
        Some(
            "Trolleys is the plural of trolley. A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of roof",
        Some(true),
        Some(
            "Roofs is the plural of roof. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of chief",
        Some(true),
        Some(
            "Chiefs is the plural of chief. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of chef",
        Some(true),
        Some(
            "Chefs is the plural of chef. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of belief",
        Some(true),
        Some(
            "Beliefs is the plural of belief. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of cliff",
        Some(true),
        Some(
            "Cliffs is the plural of cliff. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of safe",
        Some(true),
        Some(
            "Safes is the plural of safe. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of gulf",
        Some(true),
        Some(
            "Gulfs is the plural of gulf. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of handkerchief",
        Some(true),
        Some(
            "Handkerchiefs is the plural of handkerchief. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of reef",
        Some(true),
        Some(
            "Reefs is the plural of reef. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of proof",
        Some(true),
        Some(
            "Proofs is the plural of proof. Not every -f word changes: roof and roofs, chief and chiefs, safe and safes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of datum",
        Some(true),
        Some(
            "Data is the plural of datum. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of medium",
        Some(true),
        Some(
            "Media is the plural of medium. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bacterium",
        Some(true),
        Some(
            "Bacteria is the plural of bacterium. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of criterion",
        Some(true),
        Some(
            "Criteria is the plural of criterion. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of phenomenon",
        Some(true),
        Some(
            "Phenomena is the plural of phenomenon. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of curriculum",
        Some(true),
        Some(
            "Curricula is the plural of curriculum. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of memorandum",
        Some(true),
        Some(
            "Memoranda is the plural of memorandum. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of index",
        Some(true),
        Some(
            "Indices is the plural of index. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of appendix",
        Some(true),
        Some(
            "Appendices is the plural of appendix. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of matrix",
        Some(true),
        Some(
            "Matrices is the plural of matrix. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of vertex",
        Some(true),
        Some(
            "Vertices is the plural of vertex. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of formula",
        Some(true),
        Some(
            "Formulae is the plural of formula. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of larva",
        Some(true),
        Some(
            "Larvae is the plural of larva. These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of answer",
        Some(true),
        Some(
            "Answers is the plural of answer. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of animal",
        Some(true),
        Some(
            "Animals is the plural of animal. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of apple",
        Some(true),
        Some(
            "Apples is the plural of apple. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bag",
        Some(true),
        Some(
            "Bags is the plural of bag. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of ball",
        Some(true),
        Some(
            "Balls is the plural of ball. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of banana",
        Some(true),
        Some(
            "Bananas is the plural of banana. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of basket",
        Some(true),
        Some(
            "Baskets is the plural of basket. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bath",
        Some(true),
        Some(
            "Baths is the plural of bath. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bean",
        Some(true),
        Some(
            "Beans is the plural of bean. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bed",
        Some(true),
        Some(
            "Beds is the plural of bed. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bee",
        Some(true),
        Some(
            "Bees is the plural of bee. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bike",
        Some(true),
        Some(
            "Bikes is the plural of bike. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bird",
        Some(true),
        Some(
            "Birds is the plural of bird. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of birthday",
        Some(true),
        Some(
            "Birthdays is the plural of birthday. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of biscuit",
        Some(true),
        Some(
            "Biscuits is the plural of biscuit. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of boat",
        Some(true),
        Some(
            "Boats is the plural of boat. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of body",
        Some(true),
        Some(
            "Bodys is the plural of body. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bone",
        Some(true),
        Some(
            "Bones is the plural of bone. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bottle",
        Some(true),
        Some(
            "Bottles is the plural of bottle. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bowl",
        Some(true),
        Some(
            "Bowls is the plural of bowl. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of brother",
        Some(true),
        Some(
            "Brothers is the plural of brother. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of building",
        Some(true),
        Some(
            "Buildings is the plural of building. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of burger",
        Some(true),
        Some(
            "Burgers is the plural of burger. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of cake",
        Some(true),
        Some(
            "Cakes is the plural of cake. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of camera",
        Some(true),
        Some(
            "Cameras is the plural of camera. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of candle",
        Some(true),
        Some(
            "Candles is the plural of candle. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of card",
        Some(true),
        Some(
            "Cards is the plural of card. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of carpet",
        Some(true),
        Some(
            "Carpets is the plural of carpet. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of carrot",
        Some(true),
        Some(
            "Carrots is the plural of carrot. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of cat",
        Some(true),
        Some(
            "Cats is the plural of cat. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of chair",
        Some(true),
        Some(
            "Chairs is the plural of chair. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of chicken",
        Some(true),
        Some(
            "Chickens is the plural of chicken. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of chip",
        Some(true),
        Some(
            "Chips is the plural of chip. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of clock",
        Some(true),
        Some(
            "Clocks is the plural of clock. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of coat",
        Some(true),
        Some(
            "Coats is the plural of coat. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of coin",
        Some(true),
        Some(
            "Coins is the plural of coin. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of computer",
        Some(true),
        Some(
            "Computers is the plural of computer. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of cook",
        Some(true),
        Some(
            "Cooks is the plural of cook. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of corner",
        Some(true),
        Some(
            "Corners is the plural of corner. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of cow",
        Some(true),
        Some(
            "Cows is the plural of cow. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of cup",
        Some(true),
        Some(
            "Cups is the plural of cup. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of curtain",
        Some(true),
        Some(
            "Curtains is the plural of curtain. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of customer",
        Some(true),
        Some(
            "Customers is the plural of customer. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of daughter",
        Some(true),
        Some(
            "Daughters is the plural of daughter. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of day",
        Some(true),
        Some(
            "Days is the plural of day. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of desk",
        Some(true),
        Some(
            "Desks is the plural of desk. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of dinner",
        Some(true),
        Some(
            "Dinners is the plural of dinner. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of doctor",
        Some(true),
        Some(
            "Doctors is the plural of doctor. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of dog",
        Some(true),
        Some(
            "Dogs is the plural of dog. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of door",
        Some(true),
        Some(
            "Doors is the plural of door. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of driver",
        Some(true),
        Some(
            "Drivers is the plural of driver. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of ear",
        Some(true),
        Some(
            "Ears is the plural of ear. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of egg",
        Some(true),
        Some(
            "Eggs is the plural of egg. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of engineer",
        Some(true),
        Some(
            "Engineers is the plural of engineer. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of evening",
        Some(true),
        Some(
            "Evenings is the plural of evening. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of event",
        Some(true),
        Some(
            "Events is the plural of event. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of exam",
        Some(true),
        Some(
            "Exams is the plural of exam. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of eye",
        Some(true),
        Some(
            "Eyes is the plural of eye. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of face",
        Some(true),
        Some(
            "Faces is the plural of face. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of fact",
        Some(true),
        Some(
            "Facts is the plural of fact. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of farm",
        Some(true),
        Some(
            "Farms is the plural of farm. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of farmer",
        Some(true),
        Some(
            "Farmers is the plural of farmer. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of father",
        Some(true),
        Some(
            "Fathers is the plural of father. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of festival",
        Some(true),
        Some(
            "Festivals is the plural of festival. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of film",
        Some(true),
        Some(
            "Films is the plural of film. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of finger",
        Some(true),
        Some(
            "Fingers is the plural of finger. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of floor",
        Some(true),
        Some(
            "Floors is the plural of floor. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of flower",
        Some(true),
        Some(
            "Flowers is the plural of flower. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of fork",
        Some(true),
        Some(
            "Forks is the plural of fork. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of friend",
        Some(true),
        Some(
            "Friends is the plural of friend. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of game",
        Some(true),
        Some(
            "Games is the plural of game. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of garden",
        Some(true),
        Some(
            "Gardens is the plural of garden. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of gate",
        Some(true),
        Some(
            "Gates is the plural of gate. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of girl",
        Some(true),
        Some(
            "Girls is the plural of girl. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of goal",
        Some(true),
        Some(
            "Goals is the plural of goal. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of group",
        Some(true),
        Some(
            "Groups is the plural of group. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of guest",
        Some(true),
        Some(
            "Guests is the plural of guest. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hand",
        Some(true),
        Some(
            "Hands is the plural of hand. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hat",
        Some(true),
        Some(
            "Hats is the plural of hat. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of head",
        Some(true),
        Some(
            "Heads is the plural of head. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hill",
        Some(true),
        Some(
            "Hills is the plural of hill. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of holiday",
        Some(true),
        Some(
            "Holidays is the plural of holiday. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of home",
        Some(true),
        Some(
            "Homes is the plural of home. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of horse",
        Some(true),
        Some(
            "Horses is the plural of horse. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hospital",
        Some(true),
        Some(
            "Hospitals is the plural of hospital. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hotel",
        Some(true),
        Some(
            "Hotels is the plural of hotel. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hour",
        Some(true),
        Some(
            "Hours is the plural of hour. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of house",
        Some(true),
        Some(
            "Houses is the plural of house. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of idea",
        Some(true),
        Some(
            "Ideas is the plural of idea. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of island",
        Some(true),
        Some(
            "Islands is the plural of island. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of jacket",
        Some(true),
        Some(
            "Jackets is the plural of jacket. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of job",
        Some(true),
        Some(
            "Jobs is the plural of job. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of journey",
        Some(true),
        Some(
            "Journeys is the plural of journey. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of kettle",
        Some(true),
        Some(
            "Kettles is the plural of kettle. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of key",
        Some(true),
        Some(
            "Keys is the plural of key. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of kid",
        Some(true),
        Some(
            "Kids is the plural of kid. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of kitchen",
        Some(true),
        Some(
            "Kitchens is the plural of kitchen. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of lake",
        Some(true),
        Some(
            "Lakes is the plural of lake. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of lamp",
        Some(true),
        Some(
            "Lamps is the plural of lamp. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of laptop",
        Some(true),
        Some(
            "Laptops is the plural of laptop. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of leg",
        Some(true),
        Some(
            "Legs is the plural of leg. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of lesson",
        Some(true),
        Some(
            "Lessons is the plural of lesson. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of letter",
        Some(true),
        Some(
            "Letters is the plural of letter. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of light",
        Some(true),
        Some(
            "Lights is the plural of light. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of line",
        Some(true),
        Some(
            "Lines is the plural of line. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of list",
        Some(true),
        Some(
            "Lists is the plural of list. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of lunch",
        Some(true),
        Some(
            "Lunchs is the plural of lunch. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of machine",
        Some(true),
        Some(
            "Machines is the plural of machine. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of map",
        Some(true),
        Some(
            "Maps is the plural of map. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of market",
        Some(true),
        Some(
            "Markets is the plural of market. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of meal",
        Some(true),
        Some(
            "Meals is the plural of meal. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of meeting",
        Some(true),
        Some(
            "Meetings is the plural of meeting. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of message",
        Some(true),
        Some(
            "Messages is the plural of message. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of minute",
        Some(true),
        Some(
            "Minutes is the plural of minute. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mirror",
        Some(true),
        Some(
            "Mirrors is the plural of mirror. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mistake",
        Some(true),
        Some(
            "Mistakes is the plural of mistake. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of month",
        Some(true),
        Some(
            "Months is the plural of month. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of morning",
        Some(true),
        Some(
            "Mornings is the plural of morning. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mother",
        Some(true),
        Some(
            "Mothers is the plural of mother. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mountain",
        Some(true),
        Some(
            "Mountains is the plural of mountain. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mouth",
        Some(true),
        Some(
            "Mouths is the plural of mouth. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of movie",
        Some(true),
        Some(
            "Movies is the plural of movie. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of museum",
        Some(true),
        Some(
            "Museums is the plural of museum. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of name",
        Some(true),
        Some(
            "Names is the plural of name. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of neighbour",
        Some(true),
        Some(
            "Neighbours is the plural of neighbour. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of night",
        Some(true),
        Some(
            "Nights is the plural of night. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of noise",
        Some(true),
        Some(
            "Noises is the plural of noise. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of nose",
        Some(true),
        Some(
            "Noses is the plural of nose. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of note",
        Some(true),
        Some(
            "Notes is the plural of note. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of nurse",
        Some(true),
        Some(
            "Nurses is the plural of nurse. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of office",
        Some(true),
        Some(
            "Offices is the plural of office. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of onion",
        Some(true),
        Some(
            "Onions is the plural of onion. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of orange",
        Some(true),
        Some(
            "Oranges is the plural of orange. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of page",
        Some(true),
        Some(
            "Pages is the plural of page. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of paper",
        Some(true),
        Some(
            "Papers is the plural of paper. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of parent",
        Some(true),
        Some(
            "Parents is the plural of parent. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of park",
        Some(true),
        Some(
            "Parks is the plural of park. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of passenger",
        Some(true),
        Some(
            "Passengers is the plural of passenger. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of pasta",
        Some(true),
        Some(
            "Pastas is the plural of pasta. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of path",
        Some(true),
        Some(
            "Paths is the plural of path. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of pen",
        Some(true),
        Some(
            "Pens is the plural of pen. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of pencil",
        Some(true),
        Some(
            "Pencils is the plural of pencil. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of phone",
        Some(true),
        Some(
            "Phones is the plural of phone. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of photo",
        Some(true),
        Some(
            "Photos is the plural of photo. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of piano",
        Some(true),
        Some(
            "Pianos is the plural of piano. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of picture",
        Some(true),
        Some(
            "Pictures is the plural of picture. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of piece",
        Some(true),
        Some(
            "Pieces is the plural of piece. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of plane",
        Some(true),
        Some(
            "Planes is the plural of plane. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of plate",
        Some(true),
        Some(
            "Plates is the plural of plate. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of pocket",
        Some(true),
        Some(
            "Pockets is the plural of pocket. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of point",
        Some(true),
        Some(
            "Points is the plural of point. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of question",
        Some(true),
        Some(
            "Questions is the plural of question. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of radio",
        Some(true),
        Some(
            "Radios is the plural of radio. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of receipt",
        Some(true),
        Some(
            "Receipts is the plural of receipt. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of restaurant",
        Some(true),
        Some(
            "Restaurants is the plural of restaurant. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of ring",
        Some(true),
        Some(
            "Rings is the plural of ring. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of river",
        Some(true),
        Some(
            "Rivers is the plural of river. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of road",
        Some(true),
        Some(
            "Roads is the plural of road. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of room",
        Some(true),
        Some(
            "Rooms is the plural of room. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of salad",
        Some(true),
        Some(
            "Salads is the plural of salad. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sandwich",
        Some(true),
        Some(
            "Sandwichs is the plural of sandwich. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of school",
        Some(true),
        Some(
            "Schools is the plural of school. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of scientist",
        Some(true),
        Some(
            "Scientists is the plural of scientist. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sea",
        Some(true),
        Some(
            "Seas is the plural of sea. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of season",
        Some(true),
        Some(
            "Seasons is the plural of season. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of seat",
        Some(true),
        Some(
            "Seats is the plural of seat. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sentence",
        Some(true),
        Some(
            "Sentences is the plural of sentence. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of shop",
        Some(true),
        Some(
            "Shops is the plural of shop. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of shoulder",
        Some(true),
        Some(
            "Shoulders is the plural of shoulder. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sister",
        Some(true),
        Some(
            "Sisters is the plural of sister. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of site",
        Some(true),
        Some(
            "Sites is the plural of site. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of slice",
        Some(true),
        Some(
            "Slices is the plural of slice. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sofa",
        Some(true),
        Some(
            "Sofas is the plural of sofa. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of son",
        Some(true),
        Some(
            "Sons is the plural of son. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of song",
        Some(true),
        Some(
            "Songs is the plural of song. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of spoon",
        Some(true),
        Some(
            "Spoons is the plural of spoon. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sport",
        Some(true),
        Some(
            "Sports is the plural of sport. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of spring",
        Some(true),
        Some(
            "Springs is the plural of spring. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of stair",
        Some(true),
        Some(
            "Stairs is the plural of stair. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of star",
        Some(true),
        Some(
            "Stars is the plural of star. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of station",
        Some(true),
        Some(
            "Stations is the plural of station. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of street",
        Some(true),
        Some(
            "Streets is the plural of street. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of student",
        Some(true),
        Some(
            "Students is the plural of student. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of summer",
        Some(true),
        Some(
            "Summers is the plural of summer. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sun",
        Some(true),
        Some(
            "Suns is the plural of sun. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of supermarket",
        Some(true),
        Some(
            "Supermarkets is the plural of supermarket. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of surprise",
        Some(true),
        Some(
            "Surprises is the plural of surprise. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of sweet",
        Some(true),
        Some(
            "Sweets is the plural of sweet. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of table",
        Some(true),
        Some(
            "Tables is the plural of table. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of taxi",
        Some(true),
        Some(
            "Taxis is the plural of taxi. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of teacher",
        Some(true),
        Some(
            "Teachers is the plural of teacher. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of team",
        Some(true),
        Some(
            "Teams is the plural of team. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of teenager",
        Some(true),
        Some(
            "Teenagers is the plural of teenager. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of telephone",
        Some(true),
        Some(
            "Telephones is the plural of telephone. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of television",
        Some(true),
        Some(
            "Televisions is the plural of television. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of thing",
        Some(true),
        Some(
            "Things is the plural of thing. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of ticket",
        Some(true),
        Some(
            "Tickets is the plural of ticket. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of town",
        Some(true),
        Some(
            "Towns is the plural of town. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of train",
        Some(true),
        Some(
            "Trains is the plural of train. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of tree",
        Some(true),
        Some(
            "Trees is the plural of tree. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of trip",
        Some(true),
        Some(
            "Trips is the plural of trip. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of truck",
        Some(true),
        Some(
            "Trucks is the plural of truck. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of uncle",
        Some(true),
        Some(
            "Uncles is the plural of uncle. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of vegetable",
        Some(true),
        Some(
            "Vegetables is the plural of vegetable. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of village",
        Some(true),
        Some(
            "Villages is the plural of village. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of visit",
        Some(true),
        Some(
            "Visits is the plural of visit. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of voice",
        Some(true),
        Some(
            "Voices is the plural of voice. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of wall",
        Some(true),
        Some(
            "Walls is the plural of wall. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of week",
        Some(true),
        Some(
            "Weeks is the plural of week. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of weekend",
        Some(true),
        Some(
            "Weekends is the plural of weekend. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of window",
        Some(true),
        Some(
            "Windows is the plural of window. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of winter",
        Some(true),
        Some(
            "Winters is the plural of winter. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of word",
        Some(true),
        Some(
            "Most plurals just add s (cat, cats). Words ending in s, x, z, ch or sh take -es (bus, buses), a consonant then y becomes -ies (baby, babies), and -f or -fe turns into -ves (knife, knives).",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of worker",
        Some(true),
        Some(
            "Workers is the plural of worker. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of world",
        Some(true),
        Some(
            "Worlds is the plural of world. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of year",
        Some(true),
        Some(
            "Years is the plural of year. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of zoo",
        Some(true),
        Some(
            "Zoos is the plural of zoo. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of ant",
        Some(true),
        Some(
            "Ants is the plural of ant. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of bear",
        Some(true),
        Some(
            "Bears is the plural of bear. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of camel",
        Some(true),
        Some(
            "Camels is the plural of camel. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of dolphin",
        Some(true),
        Some(
            "Dolphins is the plural of dolphin. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of duck",
        Some(true),
        Some(
            "Ducks is the plural of duck. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of elephant",
        Some(true),
        Some(
            "Elephants is the plural of elephant. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of frog",
        Some(true),
        Some(
            "Frogs is the plural of frog. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of giraffe",
        Some(true),
        Some(
            "Giraffes is the plural of giraffe. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of insect",
        Some(true),
        Some(
            "Insects is the plural of insect. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of kangaroo",
        Some(true),
        Some(
            "Kangaroos is the plural of kangaroo. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of lion",
        Some(true),
        Some(
            "Lions is the plural of lion. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of monkey",
        Some(true),
        Some(
            "Monkeys is the plural of monkey. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of parrot",
        Some(true),
        Some(
            "Parrots is the plural of parrot. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of penguin",
        Some(true),
        Some(
            "Penguins is the plural of penguin. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of rabbit",
        Some(true),
        Some(
            "Rabbits is the plural of rabbit. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of shark",
        Some(true),
        Some(
            "Sharks is the plural of shark. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of snake",
        Some(true),
        Some(
            "Snakes is the plural of snake. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of spider",
        Some(true),
        Some(
            "Spiders is the plural of spider. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of tiger",
        Some(true),
        Some(
            "Tigers is the plural of tiger. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of whale",
        Some(true),
        Some(
            "Whales is the plural of whale. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of zebra",
        Some(true),
        Some(
            "Zebras is the plural of zebra. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of children",
        Some(true),
        Some(
            "Children is already plural - one child, two children. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of mice",
        Some(true),
        Some(
            "Mice is already plural - one mouse, two mice. These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of rice",
        Some(true),
        Some(
            "Rice has no plural in the everyday sense: you would say some rice, or a bit of rice, not rices.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of lettuce",
        Some(true),
        Some(
            "Lettuces is the plural of lettuce. Most words just add s: cat and cats, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of happiness",
        Some(true),
        Some(
            "Happiness has no plural: it is an amount, not a count, so you would say some happiness, or a bit of happiness.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of burrito",
        Some(true),
        Some(
            "Burritos is the plural of burrito. A short -o word takes -es (potato and potatoes, echo and echoes); a -o word that takes a plain s does too, so it is burritos, avocados and pianos.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of quartz",
        Some(true),
        Some(
            "Quartzzes is the plural of quartz. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of fuzz",
        Some(true),
        Some(
            "Fuzzes is the plural of fuzz. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of frizz",
        Some(true),
        Some(
            "Frizzes is the plural of frizz. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of studio",
        Some(true),
        Some(
            "Studios is the plural of studio. A short -o word takes -es (potato and potatoes, echo and echoes); a -o word that takes a plain s does too, so it is burritos, avocados and pianos.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of spacecraft",
        Some(true),
        Some(
            "Spacecraft is its own plural - the same word for one and for many: one spacecraft, two spacecraft.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of seahorse",
        Some(true),
        Some(
            "Seahorses is the plural of seahorse. Most words just add s: cat and cats, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of embassy",
        Some(true),
        Some(
            "Embassies is the plural of embassy. A consonant then y becomes -ies: baby and babies, city and cities.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of hero",
        Some(true),
        Some(
            "Heroes is the plural of hero. A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of octopus",
        Some(false),
        None,
    ),
    Case(
        "plural-word",
        "what is the plural of tax",
        Some(true),
        Some(
            "Taxes is the plural of tax. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of waltz",
        Some(true),
        Some(
            "Waltzzes is the plural of waltz. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of été",
        Some(true),
        Some("Etes is the plural of ete. Most words just add s: cat and cats, house and houses."),
    ),
    Case(
        "plural-word",
        "what is the plural of mother in law",
        Some(true),
        Some(
            "Mothers is the plural of mother. Most words just add s: cat and cats, plan and plans, house and houses.",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of loft insulation",
        Some(true),
        Some(
            "Most plurals just add s (cat, cats). Words ending in s, x, z, ch or sh take -es (bus, buses), a consonant then y becomes -ies (baby, babies), and -f or -fe turns into -ves (knife, knives).",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of that",
        Some(false),
        None,
    ),
    Case(
        "plural-word",
        "how do plurals of words work",
        Some(true),
        Some(
            "Most plurals just add s (cat, cats). Words ending in s, x, z, ch or sh take -es (bus, buses), a consonant then y becomes -ies (baby, babies), and -f or -fe turns into -ves (knife, knives).",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of has",
        Some(true),
        Some(
            "Most plurals just add s (cat, cats). Words ending in s, x, z, ch or sh take -es (bus, buses), a consonant then y becomes -ies (baby, babies), and -f or -fe turns into -ves (knife, knives).",
        ),
    ),
    Case(
        "plural-word",
        "what is the plural of verbs",
        Some(true),
        Some(
            "Verbses is the plural of verbs. Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes.",
        ),
    ),
    Case(
        "word-meaning",
        "what is ambiguous",
        None,
        Some(
            "Ambiguous means it can be read two ways and you cannot tell which was meant: 'the meeting is at some point tomorrow' is ambiguous about the time.",
        ),
    ),
    Case(
        "word-meaning",
        "what is bittersweet",
        None,
        Some(
            "Bittersweet is happy and sad in the same moment: the last day of a job you loved, or a child's first day at school.",
        ),
    ),
    Case(
        "word-meaning",
        "what is cynical",
        None,
        Some(
            "Cynical means expecting the worst of people's reasons - assuming a charity drive is really about publicity. It is different from sceptical, which just wants proof.",
        ),
    ),
    Case(
        "word-meaning",
        "what is mundane",
        None,
        Some(
            "Mundane means ordinary and a bit dull: the mundane jobs like the washing up and the school run. It is not an insult to a person.",
        ),
    ),
    Case(
        "word-meaning",
        "what is serendipity",
        None,
        Some(
            "Serendipity is a good thing found while looking for something else: finding a twenty in an old coat, or meeting your best friend by taking the wrong bus.",
        ),
    ),
    Case(
        "word-meaning",
        "what is procrastinate",
        None,
        Some(
            "To procrastinate is to put off what you meant to do today, usually by doing something easier instead. It is about the delay, not about being lazy.",
        ),
    ),
    Case(
        "word-meaning",
        "what is gaslighting",
        None,
        Some(
            "Gaslighting is making someone doubt their own memory of events: 'that never happened, you're imagining it'. The point is to leave them unsure of themselves.",
        ),
    ),
    Case(
        "word-meaning",
        "what is condescending",
        None,
        Some(
            "Condescending means talking down to someone, as if they were slower than you: explaining the job to the person who has done it for ten years.",
        ),
    ),
    Case(
        "word-meaning",
        "what is patronising",
        None,
        Some(
            "Patronising is the same talking-down, often with false kindness: 'well done you!' to a competent adult. Condescending is the colder word for it.",
        ),
    ),
    Case(
        "word-meaning",
        "what is pretentious",
        None,
        Some(
            "Pretentious means claiming more importance or taste than the thing deserves: a menu that calls chips 'triple-cooked potato batons'.",
        ),
    ),
    Case(
        "word-meaning",
        "what is no worries",
        None,
        Some(
            "'No worries' means it is fine and nothing is owed - an answer to an apology or a thank you, not a warning. In Australia and New Zealand it is also the everyday reply to 'thank you'.",
        ),
    ),
    Case(
        "word-meaning",
        "what is no hard feelings",
        None,
        Some(
            "'No hard feelings' means you are not holding a grudge: 'you got the job and I didn't, but no hard feelings'. It closes a disappointment without pretending it did not sting.",
        ),
    ),
    Case(
        "word-meaning",
        "what is no offence",
        None,
        Some(
            "'No offence' is a warning that the next thing is blunt, as in 'no offence, but the kitchen is a mess'. It does not make an insult kind, so only say it if you would say the same thing to their face.",
        ),
    ),
    Case(
        "word-meaning",
        "what is all good",
        None,
        Some(
            "'All good' means nothing is wrong and there is nothing to apologise for - the relaxed answer to 'sorry I'm late' or 'thanks'. It is casual, so keep it out of a formal email.",
        ),
    ),
    Case(
        "word-meaning",
        "what is fair enough",
        None,
        Some(
            "'Fair enough' means you accept the answer even if you would rather not - it closes a small disagreement rather than starting one.",
        ),
    ),
    Case(
        "word-meaning",
        "what is ballpark figure",
        None,
        Some(
            "A ballpark figure is a rough number, close enough to plan with: 'about two thousand, give or take'. Ballpark on its own means roughly right.",
        ),
    ),
    Case(
        "word-meaning",
        "what is touch base",
        None,
        Some(
            "To touch base is to check in briefly with someone: 'I'll touch base on Friday about the timetable'. It is a short catch-up, not a meeting.",
        ),
    ),
    Case(
        "word-meaning",
        "what is in two minds",
        None,
        Some(
            "To be in two minds is to be unable to choose: 'I'm in two minds about the job'. The American version is 'on the fence'.",
        ),
    ),
    Case(
        "word-meaning",
        "what is on the fence",
        None,
        Some(
            "To be on the fence is to have not decided yet: 'the council is still on the fence about the bins'. Sitting on the fence means avoiding the choice.",
        ),
    ),
    Case(
        "word-meaning",
        "what is out of the loop",
        None,
        Some(
            "To be out of the loop is to stop being told what is happening: 'I was on leave, so I'm out of the loop on the merger'.",
        ),
    ),
    Case(
        "word-meaning",
        "what is red flag",
        None,
        Some(
            "A red flag is a warning sign worth taking seriously: a deposit asked for before a viewing, or a new job with no contract.",
        ),
    ),
    Case(
        "word-meaning",
        "what is grey area",
        None,
        Some(
            "A grey area is a case the rules do not clearly cover either way: taking a sick day for a sick child. Grey in Britain, gray in America.",
        ),
    ),
    Case(
        "word-meaning",
        "what is catch 22",
        None,
        Some(
            "A catch-22 is a rule you cannot win either way: you need experience to get the job, and you need the job to get the experience.",
        ),
    ),
    Case(
        "word-meaning",
        "what is silver lining",
        None,
        Some(
            "A silver lining is the small good thing inside a bad one: the flight was cancelled, but you got an extra day with your family.",
        ),
    ),
    Case(
        "word-meaning",
        "what is cold feet",
        None,
        Some(
            "To get cold feet is to lose your nerve just before a big step: the wedding is booked and now you are panicking. It means doubt, not that you are ill.",
        ),
    ),
    Case(
        "word-meaning",
        "what is under the bus",
        None,
        Some(
            "To throw someone under the bus is to blame them to save yourself: the manager took the credit and then blamed the team when it failed.",
        ),
    ),
    Case(
        "word-meaning",
        "what is hindsight",
        None,
        Some(
            "Hindsight is seeing what you should have done once it is too late: 'in hindsight we should have booked earlier'. Hindsight is always 20/20 means it looks obvious afterwards.",
        ),
    ),
    Case(
        "word-meaning",
        "what is cliche",
        None,
        Some(
            "A cliche is a phrase so overused it has stopped meaning anything: 'thinking outside the box', 'at the end of the day'. Clichéd describes the phrase, not you.",
        ),
    ),
    Case(
        "word-meaning",
        "what is food for thought",
        None,
        Some(
            "Food for thought is something worth thinking about: 'her talk on sleep gave me food for thought'. Nothing to do with eating.",
        ),
    ),
    Case(
        "word-meaning",
        "what is the big picture",
        None,
        Some(
            "The big picture is the whole situation rather than the detail: stepping back to see whether the plan still makes sense.",
        ),
    ),
    Case(
        "word-meaning",
        "what is bonjour",
        None,
        Some(
            "Bonjour is French for hello - literally 'good day'. You say it from morning until the evening, and bonsoir (good evening) after that.",
        ),
    ),
    Case(
        "word-meaning",
        "what is bonsoir",
        None,
        Some(
            "Bonsoir is French for good evening. It takes over from bonjour once it gets dark, and it works both arriving and leaving.",
        ),
    ),
    Case(
        "word-meaning",
        "what is au revoir",
        None,
        Some(
            "Au revoir is French for goodbye - literally 'until we see each other again'. Bonjour when you arrive, au revoir when you go.",
        ),
    ),
    Case(
        "word-meaning",
        "what is merci",
        None,
        Some(
            "Merci is French for thank you. Merci beaucoup is thank you very much, and the reply is je vous en prie or, informally, pas de problème.",
        ),
    ),
    Case(
        "word-meaning",
        "what is merci beaucoup",
        None,
        Some(
            "Merci is French for thank you. Merci beaucoup is thank you very much, and the reply is je vous en prie or, informally, pas de problème.",
        ),
    ),
    Case(
        "word-meaning",
        "what is sil vous plait",
        None,
        Some(
            "S'il vous plaît is French for please - literally 'if it pleases you'. Say it when you ask for something, and merci when you get it.",
        ),
    ),
    Case(
        "word-meaning",
        "what is bon appetit",
        None,
        Some(
            "Bon appétit is French for enjoy your meal - literally 'good appetite'. Said before eating, the way people say 'enjoy' in English.",
        ),
    ),
    Case(
        "word-meaning",
        "what is ciao",
        None,
        Some(
            "Ciao is Italian for hello and goodbye, both - you say it arriving and leaving. Arrivederci is the more formal goodbye.",
        ),
    ),
    Case(
        "word-meaning",
        "what is arrivederci",
        None,
        Some(
            "Arrivederci is Italian for goodbye - literally 'until we see each other again'. Ciao does the same job between friends.",
        ),
    ),
    Case(
        "word-meaning",
        "what is grazie",
        None,
        Some(
            "Grazie is Italian for thank you. Grazie mille (a thousand thanks) is the warmer version, and prego is the reply - you're welcome.",
        ),
    ),
    Case(
        "word-meaning",
        "what is prego",
        None,
        Some(
            "Prego is Italian for you're welcome - the answer to grazie. It also means 'please, go ahead' when someone waits for you to pass.",
        ),
    ),
    Case(
        "word-meaning",
        "what is hola",
        None,
        Some(
            "Hola is Spanish for hello. It works all day, and buenos días (good morning), buenas tardes (afternoon) and buenas noches (evening) are the politer versions.",
        ),
    ),
    Case(
        "word-meaning",
        "what is adios",
        None,
        Some(
            "Adiós is Spanish for goodbye. Hasta luego (see you later) is the softer way out of a conversation.",
        ),
    ),
    Case(
        "word-meaning",
        "what is gracias",
        None,
        Some(
            "Gracias is Spanish for thank you. Muchas gracias is thanks a lot, and de nada is the reply - you're welcome.",
        ),
    ),
    Case(
        "word-meaning",
        "what is por favor",
        None,
        Some(
            "Por favor is Spanish for please - literally 'by favour'. Gracias is the thank you that goes with it.",
        ),
    ),
    Case(
        "word-meaning",
        "what is danke",
        None,
        Some(
            "Danke is German for thank you. Danke schön is thanks a lot, and bitte is the reply - you're welcome, or please, depending on how it is used.",
        ),
    ),
    Case(
        "word-meaning",
        "what is bitte",
        None,
        Some(
            "Bitte is German for please and for you're welcome, depending on the moment: bitte when you ask, bitte when you hand something over. Danke is thanks.",
        ),
    ),
    Case(
        "word-meaning",
        "what is auf wiedersehen",
        None,
        Some(
            "Auf Wiedersehen is German for goodbye - literally 'until we see each other again'. Tschüss is the casual version between friends.",
        ),
    ),
    Case(
        "word-meaning",
        "what is guten tag",
        None,
        Some(
            "Guten Tag is German for good day - hello, in the middle of the day. Guten Morgen is good morning and Guten Abend good evening.",
        ),
    ),
    Case(
        "word-meaning",
        "what is salud",
        None,
        Some(
            "Salud is Spanish for health. It is the toast - the same job as 'cheers' - and also what you say after a sneeze.",
        ),
    ),
    Case(
        "word-meaning",
        "what is namaste",
        None,
        Some(
            "Namaste is Hindi for hello and goodbye, said with the palms together. It means roughly 'I bow to you'.",
        ),
    ),
    Case(
        "word-meaning",
        "what is sayonara",
        None,
        Some("Sayonara is Japanese for goodbye. Konnichiwa is hello, and arigato is thank you."),
    ),
    Case(
        "word-meaning",
        "what is konnichiwa",
        None,
        Some(
            "Konnichiwa is Japanese for hello - strictly good afternoon, though it works as a general hello. Arigato is thank you.",
        ),
    ),
    Case(
        "word-meaning",
        "what is arigato",
        None,
        Some("Arigato is Japanese for thank you. Arigato gozaimasu is the politer, fuller form."),
    ),
    Case(
        "word-meaning",
        "what is ni hao",
        None,
        Some(
            "Nǐ hǎo is Mandarin for hello - literally 'you good'. Xièxie (thank you) is the pair you will use most.",
        ),
    ),
    Case(
        "word-meaning",
        "what is buenos dias",
        None,
        Some(
            "Buenos días is Spanish for good morning. Hola does for hello all day, and buenas noches is good evening or good night.",
        ),
    ),
    Case(
        "word-meaning",
        "what is buenas noches",
        None,
        Some(
            "Buenas noches is Spanish for good evening, and also good night when you go to bed. Hasta mañana is see you tomorrow.",
        ),
    ),
    Case(
        "word-meaning",
        "what is hasta luego",
        None,
        Some(
            "I can take a single word or saying apart if you give me the one you are stuck on - the meaning, then an example of it in a sentence.",
        ),
    ),
    Case(
        "word-meaning",
        "what is mucho gusto",
        None,
        Some(
            "Mucho gusto is Spanish for pleased to meet you - literally 'much pleasure'. The reply is igualmente, likewise.",
        ),
    ),
    Case(
        "word-meaning",
        "what is de nada",
        None,
        Some(
            "De nada is Spanish for you're welcome - literally 'of nothing'. It answers gracias, the way 'no problem' does here.",
        ),
    ),
    Case(
        "word-meaning",
        "what is como estas",
        None,
        Some(
            "Cómo estás is Spanish for how are you. The answer people expect is bien, gracias (well, thanks).",
        ),
    ),
    Case(
        "word-meaning",
        "what is que pasa",
        None,
        Some(
            "Qué pasa is Spanish for what's happening - a casual 'what's up'. Qué tal is the softer version.",
        ),
    ),
    Case(
        "word-meaning",
        "what is xiexie",
        None,
        Some(
            "Xièxie is Mandarin for thank you. Bù kèqì (you're welcome) is the reply, and nǐ hǎo is hello.",
        ),
    ),
    Case(
        "word-meaning",
        "what is oui",
        None,
        Some(
            "Oui is French for yes, and non is no. Oui, merci is yes please, which is the pair you will use most.",
        ),
    ),
    Case(
        "word-meaning",
        "what is d accord",
        None,
        Some(
            "D'accord is French for okay or agreed - the sound of 'that's fine'. Say it to agree to a plan.",
        ),
    ),
    Case(
        "word-meaning",
        "what is daccord",
        None,
        Some(
            "D'accord is French for okay or agreed - the sound of 'that's fine'. Say it to agree to a plan.",
        ),
    ),
    Case(
        "word-meaning",
        "what is ca va",
        None,
        Some(
            "Ça va is French for how are you, and also 'it's fine' as an answer. Ça va bien is 'going well'.",
        ),
    ),
    Case(
        "word-meaning",
        "what is pas de probleme",
        None,
        Some("Pas de problème is French for no problem - both 'you're welcome' and 'that's fine'."),
    ),
    Case(
        "word-meaning",
        "what is je vous en prie",
        None,
        Some(
            "Je vous en prie is French for you're welcome, and also 'please do' when you hold a door. It answers merci.",
        ),
    ),
    Case(
        "word-meaning",
        "what is tschuss",
        None,
        Some(
            "Tschüss is German for bye - the casual form between friends. Auf Wiedersehen is the formal goodbye.",
        ),
    ),
    Case(
        "word-meaning",
        "what is servus",
        None,
        Some(
            "Servus is a greeting in southern Germany and Austria: hello, and also bye, depending on the moment.",
        ),
    ),
    Case(
        "word-meaning",
        "what is scheisse",
        None,
        Some(
            "Scheiße is German for shit - the everyday swear word, used far more casually than in English, so hearing it is not as strong as it sounds.",
        ),
    ),
    Case(
        "word-meaning",
        "what is molto bene",
        None,
        Some("Molto bene is Italian for very good or very well - molto is very, bene is good."),
    ),
    Case(
        "word-meaning",
        "define boundary",
        None,
        Some(
            "A boundary is a limit you set on what you will do or accept: 'I don't answer work messages after eight'. It is a rule for yourself, not a demand on someone else - which is why saying it out loud is the hard part.",
        ),
    ),
    Case(
        "word-meaning",
        "meaning of gaslighting",
        None,
        Some(
            "Gaslighting is making someone doubt their own memory of events: 'that never happened, you're imagining it'. The point is to leave them unsure of themselves.",
        ),
    ),
    Case(
        "word-meaning",
        "search word zzz",
        None,
        Some(
            "I can take a single word or saying apart if you give me the one you are stuck on - the meaning, then an example of it in a sentence.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is gutted",
        Some(true),
        Some(
            "Gutted means deeply disappointed: 'gutted I missed it'. It is the everyday British word for it - stronger than annoyed, milder than devastated.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is chuffed",
        Some(true),
        Some(
            "Chuffed means pleased: 'chuffed with the result'. Chuffed to bits is very pleased indeed, and not chuffed is put out.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is knackered",
        Some(true),
        Some(
            "Knackered means exhausted: 'knackered after the move'. It also means broken, as in 'the washing machine is knackered' - the context tells you which.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is peckish",
        Some(true),
        Some(
            "Peckish means a bit hungry rather than starving: 'I'm peckish' is an invitation to a snack, not to a meal.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is dodgy",
        Some(true),
        Some(
            "Dodgy means unreliable or not to be trusted: a dodgy knee, dodgy wifi, a dodgy deal. It is a mild word, and it hedges - dodgy suggests you are not certain.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is cheeky",
        Some(true),
        Some(
            "Cheeky means pleasantly rude or a bit forward: a cheeky joke, a cheeky pint. Brits use it warmly as often as critically, so 'cheeky' is not an insult on its own.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is gobsmacked",
        Some(true),
        Some(
            "Gobsmacked means so surprised you cannot speak: 'gobsmacked when he turned up'. Gob is slang for mouth, and you are smacked about it.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is faff",
        Some(true),
        Some(
            "A faff is a job with more fiddly steps than it is worth: 'the paperwork is such a faff'. To faff about is to spend time on small things instead of the real one.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is kerfuffle",
        Some(true),
        Some(
            "A kerfuffle is a fuss about something small: 'a right kerfuffle over the seating'. It sounds more serious than it is.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is naff",
        Some(true),
        Some(
            "Naff means unfashionable or a bit cheap: naff souvenirs, a naff joke. It is a gentle dig rather than a real insult.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is nicked",
        Some(true),
        Some(
            "Nicked means stolen: 'someone nicked my bike'. It is also a narrow win - 'they nicked it 1-0' - and, in police talk, arrested.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is buzzing",
        Some(true),
        Some(
            "Buzzing means excited and delighted: 'buzzing for the trip'. It is the happy twin of dreading.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is niggling",
        Some(true),
        Some(
            "Niggling means small but persistent and hard to ignore: a niggling worry, a niggling pain. The size is not the point - the persistence is.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is miffed",
        Some(true),
        Some(
            "Miffed means mildly annoyed or put out: 'a bit miffed she wasn't asked'. Milder than angry, and usually about something small.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is cheesed off",
        Some(true),
        Some(
            "Cheesed off means fed up: 'cheesed off with the weather'. Brassed off and browned off mean the same thing.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is cringe",
        Some(true),
        Some(
            "Cringe is the wince you feel at someone else's embarrassment: 'watching him sing was pure cringe'. As a verb, the memory makes you cringe.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is salty",
        Some(true),
        Some(
            "Salty means bitter and sulky about losing or being left out: 'still salty about the game'. The food sense is the plain one - too much salt - so the joke is that the person is tasting tears.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is sus",
        Some(true),
        Some(
            "Sus means suspicious or not quite right, of a thing or a person: 'that email looks sus', 'he was acting sus all night'. It is short for suspicious.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is no cap",
        Some(true),
        Some(
            "'No cap' means no lie: 'it's the best one, no cap'. Cap is the lie and capping is exaggerating, so no cap is the promise that you mean it.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is vibes",
        Some(true),
        Some(
            "Vibes are the feeling a place or person gives off: 'good vibes', 'a bad vibe'. Not something you can measure, which is the point of the word.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is ghosting",
        Some(true),
        Some(
            "Ghosting is disappearing on someone without a word: no reply, no explanation, just silence. It is common enough in dating to have its own word.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is catfish",
        Some(true),
        Some(
            "To catfish is to invent an online identity to draw someone into a relationship: the person you have been talking to is not who the photos showed.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is doomscrolling",
        Some(true),
        Some(
            "Doomscrolling is scrolling bad news long after you have stopped taking it in. The feed carries on; the mood does not recover.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is fomo",
        Some(true),
        Some(
            "FOMO is fear of missing out - the tug you feel seeing everyone else's weekend. It is said as a word, 'fomo', not spelled out.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is clickbait",
        Some(true),
        Some(
            "Clickbait is a headline written to get the click rather than to tell you anything: 'you won't believe what happened next'.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is troll",
        Some(true),
        Some(
            "A troll is someone who posts to wind people up rather than to argue in good faith. Answering them is the contact they wanted.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is meme",
        Some(true),
        Some(
            "A meme is a joke or an idea copied and varied across the internet: one picture with a caption that people keep reworking.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is empathy",
        Some(true),
        Some(
            "Empathy is feeling with someone - you can imagine how it is from inside their side. Sympathy is feeling sorry for them, which is kind but stands further away.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is sympathy",
        Some(true),
        Some(
            "Sympathy is feeling sorry for someone's trouble and saying so: 'you have my sympathy'. Empathy is the one where you feel it with them.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is apathy",
        Some(true),
        Some(
            "Apathy is not caring when caring would be reasonable - the flat 'what's the point'. It is not the same as being calm or easy-going.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is ambivalent",
        Some(true),
        Some(
            "Ambivalent means holding both feelings at once: wanting the job and dreading it. It does not mean you do not care.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is indifferent",
        Some(true),
        Some(
            "Indifferent means it genuinely makes no difference to you either way. Being indifferent is flatter than disliking something.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is boundaries",
        Some(true),
        Some(
            "A boundary is a limit you set on what you will do or accept: 'I don't answer work messages after eight'. It is a rule for yourself, not a demand on someone else - which is why saying it out loud is the hard part.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is closure",
        Some(true),
        Some(
            "Closure is the sense of an ending that lets you stop going over something: an explanation, an apology, a last conversation. Plenty of people never get it, and you can still move on without it.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is narcissist",
        Some(true),
        Some(
            "A narcissist is someone with an inflated sense of their own importance and little room for other people's feelings. The word is used loosely in ordinary talk; as a diagnosis it is a specific personality disorder, and only a professional can make it.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is validation",
        Some(true),
        Some(
            "Validation is being told that what you feel or did makes sense: 'that's reasonable'. People want it because being disbelieved is a sting of its own.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is redundancy",
        Some(true),
        Some(
            "Redundancy is losing your job because the role is going, not because of anything you did: the work is cut, moved or merged. It usually means notice plus a statutory payment based on your age and length of service.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is furlough",
        Some(true),
        Some(
            "Furlough is a temporary lay-off where your employer keeps your job but stops paying you - or the state pays part of your wage through the employer.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is appraisal",
        Some(true),
        Some(
            "An appraisal is the regular sit-down where your manager goes over your work: what went well, what to change, and sometimes pay. Bring your own list of what you did.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is paradox",
        Some(true),
        Some(
            "A paradox is a statement that seems to contradict itself but may still be true: 'the more you learn, the less you know'.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is hypothetical",
        Some(true),
        Some(
            "Hypothetical means imagined rather than real: 'hypothetically, if I left'. People use it to ask a question without owning it.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is arbitrary",
        Some(true),
        Some(
            "Arbitrary means decided without a rule or reason: an arbitrary deadline, an arbitrary number. It is a criticism of how the choice was made.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is mitigate",
        Some(true),
        Some(
            "To mitigate is to make something less bad: extra drainage mitigates flooding. Do not confuse it with 'militate', which means to weigh against.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is inertia",
        Some(true),
        Some(
            "Inertia is the tendency to carry on as you were: in physics, an object's resistance to a change of movement; at home, the reason a routine outlasts its point.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is reiterate",
        Some(true),
        Some(
            "To reiterate is to say something again, usually to be sure it was heard: 'let me reiterate the deadline'.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is satire",
        Some(true),
        Some(
            "Satire uses humour to mock something in order to change it: a sketch that makes a habit ridiculous. It is aimed at the powerful, which is what separates it from a joke at someone's expense.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is sarcasm",
        Some(true),
        Some(
            "Sarcasm is saying the opposite of what you mean with a bite to it: 'great, another meeting'. Irony is the wider pattern; sarcasm is aimed at a person.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is stereotype",
        Some(true),
        Some(
            "A stereotype is a fixed, over-simple picture of a group, applied to individuals who may not fit it at all.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is prejudice",
        Some(true),
        Some(
            "Prejudice is judging before you know: a set view about a group of people, held regardless of what they are actually like.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is discrimination",
        Some(true),
        Some(
            "Discrimination is treating someone worse because of who they are - their sex, race, age, disability, religion and so on - rather than what they do.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is consent",
        Some(true),
        Some(
            "Consent is a clear yes, given freely, and it can be taken back at any point. Someone asleep, too drunk to follow what is happening, or being pressured cannot give it.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is placebo",
        Some(true),
        Some(
            "A placebo is a treatment with no active ingredient in it. People often improve anyway, which is the placebo effect, and it is why trials compare against one.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is kudos",
        Some(true),
        Some(
            "Kudos is praise and credit for something done: 'kudos for getting through that'. It is uncountable - you get kudos, not a kudo.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is schadenfreude",
        Some(true),
        Some(
            "Schadenfreude is pleasure at someone else's bad luck - German for 'harm-joy', and borrowed because it names the feeling exactly.",
        ),
    ),
    Case(
        "everyday-word-meaning",
        "what is zeitgeist",
        Some(true),
        Some(
            "Zeitgeist is the mood of a period: the ideas and tastes that feel typical of the time. German for 'time-spirit'.",
        ),
    ),
    Case("everyday-word-meaning", "define zzz", Some(true), None),
    Case(
        "everyday-word-meaning",
        "some of this furniture is old",
        Some(false),
        None,
    ),
    Case(
        "everyday-word-meaning",
        "explain the best way to clean a sofa",
        Some(false),
        None,
    ),
    Case(
        "everyday-word-meaning",
        "what does gut mean",
        Some(true),
        None,
    ),
    Case(
        "word-synonyms",
        "another word for happy",
        None,
        Some(
            "Glad, cheerful, pleased, content, delighted. Glad is the everyday one, content means quietly satisfied, and delighted is a big thing.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for sad",
        None,
        Some(
            "Unhappy, down, miserable, low, gutted. Down and low are for a flat week, miserable and gutted for something that has actually happened.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for big",
        None,
        Some(
            "Large, huge, enormous, vast, sizeable. Large is neutral and fits writing; huge and enormous are the spoken ones, and vast is for space rather than a thing.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for small",
        None,
        Some(
            "Little, tiny, slight, compact, minor. Tiny is smaller than little, and minor is for importance rather than size.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for good",
        None,
        Some(
            "Fine, solid, strong, decent, excellent. Fine is faint praise, decent is a real compliment in speech, and excellent is for something you would single out.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for bad",
        None,
        Some(
            "Poor, weak, awful, dreadful, rubbish. Poor and weak are the calm written ones; awful and rubbish are how people say it out loud.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for said",
        None,
        Some(
            "Said, asked, replied, muttered, shouted, whispered. Said is invisible and usually the right word - save the others for the two or three places where how it was said matters.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for nice",
        None,
        Some(
            "Kind, friendly, thoughtful, pleasant, lovely. Kind and thoughtful are about what someone did; pleasant and lovely are about how something felt.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for tired",
        None,
        Some(
            "Weary, worn out, exhausted, drained, sleepy. Sleepy means ready for bed, exhausted means past it, and drained is the one after a hard week.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for angry",
        None,
        Some(
            "Cross, annoyed, furious, irritated, livid. Cross is the mild everyday word, annoyed and irritated are about a thing, and furious and livid are the top end.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for scared",
        None,
        Some(
            "Frightened, afraid, nervous, anxious, terrified. Nervous is before, frightened is now, and terrified is the extreme of either.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for beautiful",
        None,
        Some(
            "Lovely, stunning, gorgeous, striking, handsome. Beautiful is the safe one, stunning is a reaction, and handsome works for a building as well as a face.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for interesting",
        None,
        Some(
            "Intriguing, absorbing, fascinating, notable, worth a look. Absorbing means it held your attention, fascinating is stronger, and notable is the neutral written one.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for important",
        None,
        Some(
            "Key, vital, crucial, significant, essential. Significant is the calm report word; crucial and essential mean it cannot be left out.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for difficult",
        None,
        Some(
            "Hard, tricky, demanding, awkward, challenging. Tricky is a puzzle, demanding is effort, and awkward is a person or a corner rather than a task.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for easy",
        None,
        Some(
            "Simple, straightforward, effortless, quick, painless. Simple is about the steps, straightforward is about the instructions, and effortless is how it felt.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for funny",
        None,
        Some(
            "Amusing, comical, hilarious, witty, odd. Witty is clever, hilarious is loud, and odd is the other meaning of funny.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for hungry",
        None,
        Some(
            "Starving, peckish, famished, ravenous. Peckish is a bit, starving is an exaggeration everyone uses, and ravenous is the real thing.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for thank you",
        None,
        Some(
            "Thanks, many thanks, thank you so much, I really appreciate it, much appreciated. Thanks is fine in a message and the longer ones belong in a card or after a real favour.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for sorry",
        None,
        Some(
            "I apologise, my apologies, I'm sorry about that, excuse me. I apologise is the written one for a mistake, and sorry is warmer when you mean it.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for no",
        None,
        Some(
            "Not this time, I can't, I'd rather not, that doesn't work for me. I'd rather not is the kindest firm one, and adding a reason of one clause is plenty.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for good luck",
        None,
        Some(
            "Best of luck, all the best, fingers crossed, break a leg. Fingers crossed is the friendly one, and break a leg is only for performers.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for congratulations",
        None,
        Some(
            "Well done, nice one, many congratulations, thrilled for you. Well done is for effort, congratulations for the news itself.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for delicious",
        None,
        Some(
            "Tasty, mouthwatering, lovely, moreish, scrumptious. Tasty is the everyday one, mouthwatering is for how it looks, and moreish means you want more of it.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for clever",
        None,
        Some(
            "Bright, sharp, smart, inventive, quick. Bright and smart are the everyday ones, sharp is fast on the uptake, and inventive is about ideas.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for brave",
        None,
        Some(
            "Courageous, bold, gutsy, plucky, fearless. Courageous is the formal one, bold is about a decision, and gutsy is doing it while scared.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for busy",
        None,
        Some(
            "Flat out, rushed, swamped, hectic, tied up. Flat out and swamped are the spoken ones, hectic is about the day, and tied up is the polite one.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for great",
        None,
        Some(
            "Excellent, brilliant, superb, terrific, first-rate. Brilliant is the British spoken one, superb is the calm written one, and terrific is louder.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for quick",
        None,
        Some(
            "Fast, swift, speedy, brisk, prompt. Fast is about speed, brisk is a pace you set, and prompt means on time.",
        ),
    ),
    Case(
        "word-synonyms",
        "another word for i dont know",
        None,
        Some(
            "I'm not sure, I'm not certain, I couldn't say, I'd have to check. I'd have to check is the one that keeps you honest without sounding helpless.",
        ),
    ),
    Case(
        "word-synonyms",
        "another way to say happy birthday in a card",
        None,
        None,
    ),
    Case(
        "word-synonyms",
        "a better word for sad please",
        None,
        Some(
            "Unhappy, down, miserable, low, gutted. Down and low are for a flat week, miserable and gutted for something that has actually happened.",
        ),
    ),
    Case(
        "word-synonyms",
        "what is another word for wobbling furniture",
        None,
        None,
    ),
    Case("word-synonyms", "whats a good word for a dog", None, None),
    Case(
        "a-kind-word",
        "what should i say to my landlord about the rent rise",
        Some(false),
        Some(
            "Say the one thing you actually noticed and stop: \"You always remember how everyone takes their tea.\" Praise that names a thing they did is a compliment; praise with no subject in it is noise.",
        ),
    ),
    Case(
        "a-kind-word",
        "a kind word about my sister",
        Some(true),
        Some(
            "Name the thing they do that nobody remarks on and say it straight: \"You're the one who keeps this group actually going.\" The specific thing is the compliment; \"you're great\" is a placeholder and they can tell.",
        ),
    ),
    Case(
        "a-kind-word",
        "compliment her new haircut",
        Some(true),
        Some(
            "Praise the detail you can see, not the thing itself: \"That cut makes your jaw look sharp,\" not \"nice haircut.\" Say it once, plainly, and let it stand - the detail is the proof you looked.",
        ),
    ),
    Case(
        "a-kind-word",
        "say something nice about that dog",
        Some(true),
        Some(
            "Go on the animal and not the owner: \"He doesn't just ignore you, he audibly ignores you.\" One true thing you have watched it do beats the word cute every time, because it proves you were paying attention.",
        ),
    ),
    Case(
        "a-kind-word",
        "a nice word for a stranger",
        Some(true),
        Some(
            "Pick something they chose rather than something they were born with - the coat, the dog, the way they handled the queue - and say it in five words as you go: \"That coat is a very good coat.\" No follow-up needed, so it does not turn into a conversation they did not ask for.",
        ),
    ),
    Case(
        "a-kind-word",
        "a compliment about her cake",
        Some(true),
        Some(
            "Praise the thing they made rather than the person making it: \"That crust has a proper crackle to it.\" They know how much work went into it, so the detail is what lands; \"it's lovely\" is what people say when they did not look.",
        ),
    ),
    Case(
        "a-kind-word",
        "a kind thing to say about their garden",
        Some(true),
        Some(
            "Praise the work you can see rather than the whole thing: \"That border is doing a lot of quiet work.\" The one detail you noticed is the compliment; \"it looks lovely\" is a placeholder and they can tell.",
        ),
    ),
    Case(
        "a-kind-word",
        "a nice thing to tell someone",
        Some(true),
        Some(
            "Name the thing they do that nobody remarks on and say it straight: \"You're the one who keeps this group actually going.\" The specific thing is the compliment; \"you're great\" is a placeholder and they can tell.",
        ),
    ),
    Case(
        "a-kind-word",
        "what do i say to him about the mess",
        Some(false),
        Some(
            "Say the one thing you actually noticed and stop: \"You always remember how everyone takes their tea.\" Praise that names a thing they did is a compliment; praise with no subject in it is noise.",
        ),
    ),
    Case(
        "write-an-email",
        "how do i write an email to my plumber",
        None,
        Some(
            "Put the ask in the subject line and in the first sentence: \"Leak under the kitchen sink - when can you come?\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\"",
        ),
    ),
    Case(
        "write-an-email",
        "how do i email my teacher",
        None,
        Some(
            "Put the ask in the subject line and in the first sentence: \"Year 6 trip - a question about the form\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\"",
        ),
    ),
    Case(
        "write-an-email",
        "subject line for email to the doctor",
        None,
        Some(
            "Put the ask in the subject line and in the first sentence: \"Repeat prescription - could you check this for me\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\"",
        ),
    ),
    Case(
        "write-an-email",
        "how do i email the landlord",
        None,
        Some(
            "Put the ask in the subject line and in the first sentence: \"Boiler still not working - flat 3\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\"",
        ),
    ),
    Case(
        "write-an-email",
        "how do i email my boss",
        None,
        Some(
            "Put the ask in the subject line and in the first sentence: \"A quick question about Thursday\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\"",
        ),
    ),
    Case(
        "write-an-email",
        "how do i email about this invoice",
        None,
        Some(
            "Put the ask in the subject line and in the first sentence: \"Invoice 2144 - payment due\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\"",
        ),
    ),
    Case(
        "write-an-email",
        "how should i write an email",
        None,
        Some(
            "Put the ask in the subject line and in the first sentence: \"Invoice 2144 - payment due\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\"",
        ),
    ),
    Case(
        "write-an-email",
        "how do i email the landlord about a boiler leak",
        None,
        Some(
            "Put the ask in the subject line and in the first sentence: \"Leak under the kitchen sink - when can you come?\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\"",
        ),
    ),
    Case(
        "thank-you-note",
        "how do i write a thank you note for flowers",
        None,
        Some(
            "Say where the flowers are and what they have done for the room: \"Thank you for the flowers - they are on the kitchen table and the whole week looks better.\" Two or three lines, say when you will see them next, and stop.",
        ),
    ),
    Case(
        "thank-you-note",
        "thank you note for the present",
        None,
        Some(
            "Name the thing and what you are doing with it, rather than thanking them for it: \"Thank you for the cafetiere - it is on my desk and I use it every morning.\" Two or three lines, say when you will see them next, and stop; a present does not need a page.",
        ),
    ),
    Case(
        "thank-you-note",
        "write a thank you note for dinner",
        None,
        Some(
            "Name the thing they did and what it changed for you, then stop: \"Thank you for the meal on Saturday - I came home full and happy.\" One detail from the evening that only you would know beats a paragraph of adjectives, and anything you are actually going to do, say when.",
        ),
    ),
    Case(
        "thank-you-note",
        "how should i thank them for the weekend",
        None,
        Some(
            "Pick the one thing they did that you noticed - the spare bed, the early start, the extra trip to the station - and say what it saved you: \"Thank you for having me for the weekend - the lift on Sunday saved me the whole morning.\" Then say when you will have them back, if you mean it.",
        ),
    ),
    Case(
        "thank-you-note",
        "a thank you note for the lift",
        None,
        Some(
            "Name the trip and what it saved you: \"Thank you for the lift to the station on Friday - it saved me an hour and a soaking.\" Then one line about what it meant to you, and stop.",
        ),
    ),
    Case(
        "thank-you-note",
        "how can i thank them for looking after my cat",
        None,
        Some(
            "Name the job they did and what it saved you, then stop: \"Thank you for looking after the cat while we were away - coming back to a happy animal was the whole holiday.\" One line about what it meant to you beats a paragraph, and say what you will do in return if you mean it.",
        ),
    ),
    Case(
        "thank-you-note",
        "thank you note to my teacher",
        None,
        Some(
            "Say what they did and what it was worth to you: \"Thank you for the time you gave me over the summer - I still use the list you wrote.\" One sentence of detail, one of what it meant, then stop; people keep the short ones.",
        ),
    ),
    Case(
        "thank-you-note",
        "how to write a thank you note",
        None,
        Some(
            "Name the thing they did, not the day in general: \"Thank you for the lemon cake and for staying to help with the chairs, it made the afternoon easy.\" One line about a specific detail beats a paragraph of adjectives, and send it the same day, because thanks the next morning reads as an afterthought.",
        ),
    ),
    Case(
        "thanks-for-hospitality",
        "how do i thank someone for a meal",
        None,
        Some(
            "Name the thing they did and what it changed for you, then stop: \"Thank you for the meal on Saturday - I came home full and happy.\" One detail from the evening that only you would know beats a paragraph of adjectives, and anything you are actually going to do, say when.",
        ),
    ),
    Case(
        "thanks-for-hospitality",
        "thank you letter for a present",
        None,
        Some(
            "Name the thing and what you are doing with it, rather than thanking them for it: \"Thank you for the cafetiere - it is on my desk and I use it every morning.\" Two or three lines, say when you will see them next, and stop; a present does not need a page.",
        ),
    ),
    Case(
        "thanks-for-hospitality",
        "a note thanking my host",
        None,
        Some(
            "Pick the one thing they did that you noticed - the spare bed, the early start, the extra trip to the station - and say what it saved you: \"Thank you for having me for the weekend - the lift on Sunday saved me the whole morning.\" Then say when you will have them back, if you mean it.",
        ),
    ),
    Case(
        "thanks-for-hospitality",
        "how do i thank them for the lift",
        None,
        Some(
            "Name the trip and what it saved you: \"Thank you for the lift to the station on Friday - it saved me an hour and a soaking.\" Then one line about what it meant to you, and stop.",
        ),
    ),
    Case(
        "thanks-for-hospitality",
        "write thanks for a weekend away",
        None,
        Some(
            "Pick the one thing they did that you noticed - the spare bed, the early start, the extra trip to the station - and say what it saved you: \"Thank you for having me for the weekend - the lift on Sunday saved me the whole morning.\" Then say when you will have them back, if you mean it.",
        ),
    ),
    Case(
        "thanks-for-hospitality",
        "what do i say to thank them for flowers",
        None,
        Some(
            "Say where the flowers are and what they have done for the room: \"Thank you for the flowers - they are on the kitchen table and the whole week looks better.\" Two or three lines, say when you will see them next, and stop.",
        ),
    ),
];
