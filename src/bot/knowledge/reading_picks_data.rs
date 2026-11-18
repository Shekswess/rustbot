//! Authored source data from PyBot 7d63c7f, bot/knowledge/entertainment.py.

pub(super) const BOOK_ANCHOR: &str = r#"\b(?:about|on|covering|teach(?:es)?|for beginners? (?:at|in|to)|beginners? (?:guide to|book (?:on|about))|for (?:someone|somebody|anyone|anybody|people|a person|my (?:mum|mom|dad|mother|father|brother|sister|son|daughter|wife|husband|partner|friend|nan|gran|grandad|grandma|grandpa)) (?:who|that) (?:likes?|loves?|enjoys?|is into|is a fan of|is interested in|is keen on)|(?:who|that) (?:likes?|loves?|enjoys?|is into|is a fan of|is interested in|is keen on)|(?:someone|somebody|anyone|anybody|people|a person) (?:interested in|keen on|into)|(?:if|when) (?:i|you|they|he|she|im|i'm) (?:like|love|enjoy|am into|are into|is into)|a fan of|interested in|keen on|(?:to|for) learn(?:ing)?)\b\s+(.{0,40})"#;

pub(super) const BOOK_SUBJECT_BEFORE: &str = r#"([a-z' ]{0,40})\bbooks?\b"#;

pub(super) const BOOK_VERB_BEFORE: &str = r#"\b(?:should|shall|can|could|must|might|would|will|do|does|did|need|needs|needed|worth|to|lets|let's)\s+(?:we\s+|i\s+|you\s+|they\s+|he\s+|she\s+|it\s+)?book(?:ing|ed)?\b"#;

pub(super) const BOOK_VERB_AFTER: &str = r#"\bbook(?:ing|ed)?\s+(?:first|now|ahead|early|before|soon|online|today|tonight|tickets?|in advance|up)\b"#;

pub(super) const A_DETERMINER_JUST_BEFORE: &str = r#"\b(?:a|an|the|this|that|my|our|your|his|her|their|its|any|some|no|each|every|another|one|same|next|other|good|great|new|second|third)\s+(?:\w+\s+)?$"#;

pub(super) const A_BOOK_I_AM_NOT_ENJOYING: &str = r#"\b(?:boring|bored\s+(?:of|with)|a\s+slog|so\s+slow|really\s+slow|not\s+enjoying|can'?t\s+get\s+into|cant\s+get\s+into|gave\s+up\s+on|rubbish|terrible|awful|dull)\b"#;

pub(super) const ASKS_FOR_A_PICK: &str = r#"\b(?:recommend|suggest|suggestion|any\s+good|something|what\s+should\s+i\s+read|what\s+to\s+read|what\s+can\s+i\s+read|a\s+book\s+(?:that|to|for|about)|what\s+book|which\s+book|any\s+ideas|ideas\s+for)\b"#;

pub(super) const SUITABILITY_ASK: &str = r#"\b(?:is|are|was|were|would)\b[^?.!]{0,40}?\b(?:ok|okay|suitable|appropriate|fine|alright|all right)\b[^?.!]{0,20}?\bfor\b|\b(?:what|which) age\b[^?.!]{0,40}\b(?:suitable|appropriate|ok|okay|right)\b|\b(?:too old|too young|right age|age range|age rating)\b|\bfor (?:a|an) \d{1,2} ?(?:year|yr)s? ?old\b[^?.!]{0,20}\b(?:ok|okay|suitable)\b"#;

pub(super) const COMIC_MEANING_ASK: &str = r#"\bwhat(?:'s| is| are)\s+(?:a|an|the)?\s*(?:graphic novels?|mangas?|comic books?|comics?|webtoons?)\s*[?.!]*\s*$|\bdifference between\b"#;

pub(super) const SUBJECT_UNCOVERED_ORDER: &[&str] = &[
    "keeping chickens",
    "second world war",
    "public speaking",
    "allotmenteering",
    "first world war",
    "family history",
    "wild swimming",
    "horse riding",
    "snowboarding",
    "martial arts",
    "home brewing",
    "beer brewing",
    "board games",
    "calligraphy",
    "beekeeping",
    "meditation",
    "embroidery",
    "rug making",
    "genealogy",
    "astrology",
    "mushrooms",
    "world war",
    "ceramics",
    "swimming",
    "quilting",
    "foraging",
    "mycology",
    "chickens",
    "pottery",
    "singing",
    "dancing",
    "surfing",
    "snooker",
    "origami",
    "bridge",
    "tennis",
    "skiing",
    "boxing",
    "poker",
    "darts",
    "tarot",
    "golf",
    "bees",
    "wars",
    "ww2",
    "ww1",
    "war",
];

pub(super) const RESERVING_PHRASES: &[&str] = &[
    "book a",
    "book an",
    "book the",
    "book my",
    "book our",
    "book some",
    "to book",
    "booking a",
    "booking the",
    "booked a",
    "booked the",
    "book it",
    "reserve a",
    "reserve the",
    "reserved a",
    "booked for",
    "booked in",
    "booked up",
    "booked with",
    "is booked",
    "are booked",
    "was booked",
    "were booked",
    "be booked",
    "got booked",
];

pub(super) const JUDGE_WORDS: &[&str] = &[
    "good",
    "worth",
    "worthwhile",
    "like",
    "boring",
    "rubbish",
    "any",
    "better",
    "watch",
    "watching",
    "read",
    "reading",
    "see",
    "start",
    "best",
    "should",
];

pub(super) const TEEN_WORDS: &[&str] = &[
    "eighteen",
    "fifteen",
    "fourteen",
    "seventeen",
    "sixteen",
    "teen",
    "teenaged",
    "teenager",
    "teenagers",
    "teens",
    "thirteen",
];

pub(super) const CHILD_OF_THEIRS: &[&str] = &[
    "boy",
    "boys",
    "child",
    "children",
    "daughter",
    "girl",
    "girls",
    "goddaughter",
    "godson",
    "grandchild",
    "grandchildren",
    "kid",
    "kids",
    "nephew",
    "niece",
    "son",
    "teen",
    "teenager",
    "tween",
    "tweens",
    "youngster",
    "youngsters",
];

pub(super) const LIKING: &[&str] = &[
    "adores", "devours", "enjoy", "enjoys", "into", "like", "likes", "love", "loves", "obsessed",
    "read", "reads",
];

pub(super) const ASKS_FOR_A_PICK_WORDS: &[&str] = &[
    "any",
    "best",
    "for",
    "good",
    "idea",
    "ideas",
    "next",
    "recommend",
    "should",
    "start",
    "suggest",
    "what",
    "which",
];

pub(super) const KIND_ASKS: &[&str] = &[
    "any",
    "beginner",
    "beginners",
    "best",
    "first",
    "good",
    "into",
    "next",
    "read",
    "reading",
    "recommend",
    "recommendation",
    "recommendations",
    "start",
    "started",
    "suggest",
    "try",
    "what",
    "which",
    "worth",
];

pub(super) const BEGINNER_WORDS: &[&str] = &["beginner", "beginners", "newbie", "newcomer"];

pub(super) const COMIC_SUITABILITY: &str = "Judge it by the age rating on the back - 8+, 12+ and 16+ - and by what the child already reads, then read or skim the first few pages yourself for the two things that matter, sex and violence. *Dog Man* and *Hilda* are safe from about seven, *Heartstopper* and *Fullmetal Alchemist* from about twelve, and *Watchmen* and *Saga* are the ones to leave until they are older - *Watchmen* has sexual violence in it. A librarian will tell you what is in a title in about a minute, and will not mind being asked.";

pub(super) const COMIC_FOR_A_CHILD: &str = "For a child, *Dog Man* by Dav Pilkey is the one that gets read to bits, and *Hilda* by Luke Pearson is the warmer, stranger alternative. *Naruto* or *One Piece* if they want manga to live in. A library will lend all of these, and a finished single story is easier to hand over than a long superhero run.";

pub(super) const COMIC_FOR_A_TEEN: &str = "*Heartstopper* by Alice Oseman is the warmest place to start: a school story that reads fast and is kind about being a teenager. *One Piece* if they want a long one to live in, *Fullmetal Alchemist* if they would rather finish something, and *Persepolis* if they like real life. A library will lend all of these.";

pub(super) const COMIC_FOR_A_GROWN_UP: &str = "*Saga* by Brian K. Vaughan is the usual first pick for an adult: a space story with a family at its centre and art that carries the plot. *Watchmen* if you want the one that changed what comics could do, *Persepolis* by Marjane Satrapi for a memoir told this way, and *Heartstopper* by Alice Oseman for a warm school romance. For manga, *Death Note* or *One Piece* is where most people start. Begin with a complete story rather than a forty-year superhero run, and a library will lend all of these.";

pub(super) const COMIC_BEGINNER: &str = "*Bone* by Jeff Smith is the easiest place to start: one long story, funny, and it looks like a children's book until you are deep in it. *Persepolis* by Marjane Satrapi if you want something true, *Saga* if you want a space story. A library will have all three, and one complete story is a better first comic than a run of forty issues.";

pub(super) const COMIC_MANGA: &str = "*Death Note* (a notebook that kills, and the duel that follows) is the usual first manga: twelve volumes, a clean ending, and nothing to know beforehand. *One Piece* if they want a long one to live in, or *Fullmetal Alchemist* if they would rather have something they can finish. Read right to left, back to front - the pages are printed the Japanese way.";

pub(super) const COMIC_MARVEL: &str = "*Daredevil: Born Again* by Frank Miller is the one to start with: short, brutal and complete in itself. *Hawkeye* by Matt Fraction if you want funny, *Ms Marvel* by G. Willow Wilson if you want a teenager finding her feet. Skip the crossover events - they assume forty years of reading behind them.";

pub(super) const COMIC_DC: &str = "*Batman: Year One* by Frank Miller is the cleanest way in: one short story, no homework. *All-Star Superman* by Grant Morrison if you want the character at his best, *Swamp Thing* by Alan Moore if you want something strange and grown-up. *Watchmen* is the other DC one worth the time.";

pub(super) const COMIC_MEANING: &str = "A graphic novel is one whole story told in comic panels and published as a single book rather than a monthly issue - *Persepolis* and *Watchmen* are the classic examples. Manga is Japanese comics, usually printed right to left, and *Death Note* is an easy first one.";

pub(super) const JUDGED_WORKS: &[&str] = &[
    "the wire",
    "breaking bad",
    "better call saul",
    "the sopranos",
    "succession",
    "fleabag",
    "happy valley",
    "line of duty",
    "peaky blinders",
    "chernobyl",
    "the bear",
    "ted lasso",
    "schitts creek",
    "derry girls",
    "game of thrones",
    "the crown",
    "stranger things",
    "sherlock",
    "black mirror",
    "killing eve",
    "mad men",
    "downton abbey",
    "only murders in the building",
    "the traitors",
    "doctor who",
    "pride and prejudice",
    "1984",
    "to kill a mockingbird",
    "dune",
    "the office",
    "oppenheimer",
    "the godfather",
    "lord of the rings",
    "harry potter",
    "gone girl",
    "my neighbour totoro",
    "spirited away",
    "parasite",
    "paddington 2",
    "everything everywhere all at once",
    "squid game",
    "the last of us",
    "shogun",
    "slow horses",
    "severance",
    "the boys",
    "money heist",
    "bridgerton",
    "the witcher",
    "gilmore girls",
    "sex education",
    "normal people",
    "bluey",
    "avatar the last airbender",
    "the matrix",
    "inception",
    "interstellar",
    "barbie",
    "wonka",
    "the holdovers",
    "past lives",
    "where the crawdads sing",
    "the midnight library",
    "lessons in chemistry",
    "the seven husbands of evelyn hugo",
    "tomorrow and tomorrow and tomorrow",
    "a little life",
    "klara and the sun",
    "the song of achilles",
    "circe",
    "piranesi",
    "project hail mary",
    "the martian",
    "sapiens",
];

pub(super) const SUBJECT_BOOK_NAMES: &[(&str, &str)] = &[
    ("stocks and shares", "investing"),
    ("vegetable garden", "gardening"),
    ("training a puppy", "dog training"),
    ("personal finance", "money"),
    ("servicing my car", "car"),
    ("human behaviour", "psychology"),
    ("vegetable patch", "gardening"),
    ("car maintenance", "car"),
    ("human behavior", "psychology"),
    ("climate change", "climate"),
    ("puppy training", "dog training"),
    ("grow your own", "gardening"),
    ("bird watching", "birdwatching"),
    ("neuroscience", "psychology"),
    ("philosophers", "philosophy"),
    ("the universe", "science"),
    ("watercolours", "painting"),
    ("dog training", "dog training"),
    ("birdwatching", "birdwatching"),
    ("home repairs", "diy"),
    ("the economy", "economics"),
    ("the climate", "climate"),
    ("mathematics", "maths"),
    ("linguistics", "language"),
    ("dressmaking", "sewing"),
    ("watercolour", "painting"),
    ("photography", "photography"),
    ("breadmaking", "baking"),
    ("programming", "coding"),
    ("index funds", "investing"),
    ("woodworking", "woodworking"),
    ("fly fishing", "fishing"),
    ("sea fishing", "fishing"),
    ("psychology", "psychology"),
    ("philosophy", "philosophy"),
    ("government", "politics"),
    ("allotments", "gardening"),
    ("watercolor", "painting"),
    ("stargazing", "astronomy"),
    ("javascript", "coding"),
    ("web design", "coding"),
    ("investment", "investing"),
    ("bouldering", "climbing"),
    ("the brain", "psychology"),
    ("economics", "economics"),
    ("chemistry", "science"),
    ("evolution", "science"),
    ("gardening", "gardening"),
    ("allotment", "gardening"),
    ("veg patch", "gardening"),
    ("sketching", "drawing"),
    ("astronomy", "astronomy"),
    ("the stars", "astronomy"),
    ("investing", "investing"),
    ("budgeting", "money"),
    ("carpentry", "woodworking"),
    ("the mind", "psychology"),
    ("politics", "politics"),
    ("language", "language"),
    ("knitting", "knitting"),
    ("painting", "painting"),
    ("acrylics", "painting"),
    ("keyboard", "piano"),
    ("woodwork", "woodworking"),
    ("climbing", "climbing"),
    ("plumbing", "diy"),
    ("science", "science"),
    ("physics", "science"),
    ("biology", "science"),
    ("climate", "climate"),
    ("numbers", "maths"),
    ("grammar", "language"),
    ("crochet", "crochet"),
    ("drawing", "drawing"),
    ("cooking", "cooking"),
    ("running", "running"),
    ("jogging", "running"),
    ("history", "history"),
    ("fishing", "fishing"),
    ("poetry", "poetry"),
    ("garden", "gardening"),
    ("sewing", "sewing"),
    ("sketch", "drawing"),
    ("guitar", "guitar"),
    ("baking", "baking"),
    ("coding", "coding"),
    ("python", "coding"),
    ("tiling", "diy"),
    ("maths", "maths"),
    ("poems", "poetry"),
    ("chess", "chess"),
    ("paint", "painting"),
    ("piano", "piano"),
    ("bread", "baking"),
    ("birds", "birdwatching"),
    ("money", "money"),
    ("knit", "knitting"),
    ("draw", "drawing"),
    ("bake", "baking"),
    ("cook", "cooking"),
    ("yoga", "yoga"),
    ("code", "coding"),
    ("wine", "wine"),
    ("sew", "sewing"),
    ("run", "running"),
    ("diy", "diy"),
    ("car", "car"),
];

pub(super) const SUBJECT_BOOKS: &[(&str, &str)] = &[
    (
        "psychology",
        "*Thinking, Fast and Slow* by Daniel Kahneman is the one that changes how you see your own decisions, though it is a long read; *The Man Who Mistook His Wife for a Hat* by Oliver Sacks is the shorter, stranger way in.",
    ),
    (
        "philosophy",
        "*Think* by Simon Blackburn is written for someone who has never read any: a chapter a question, no jargon. *Sophie's World* by Jostein Gaarder tells the whole history of it as a novel if you want the story instead.",
    ),
    (
        "economics",
        "*The Undercover Economist* by Tim Harford explains prices, markets and why things cost what they do without a single equation. *The Armchair Economist* by Steven Landsburg is the more argumentative one.",
    ),
    (
        "science",
        "*A Short History of Nearly Everything* by Bill Bryson is the one to start with: how we know what we know, and funny throughout. *The Selfish Gene* by Richard Dawkins if it is evolution you want.",
    ),
    (
        "climate",
        "*How to Avoid a Climate Disaster* by Bill Gates is the plainest account of where the emissions actually come from and what would fix each one, without the gloom. *The Uninhabitable Earth* by David Wallace-Wells is the bleak one.",
    ),
    (
        "maths",
        "*Alex's Adventures in Numberland* by Alex Bellos is maths as stories about people and puzzles, so nothing feels like homework. *How Not to Be Wrong* by Jordan Ellenberg is the one about using it to think.",
    ),
    (
        "politics",
        "*How Westminster Works... and Why It Doesn't* by Ian Dunt is the short angry guide to how British government actually runs. *The Prince* by Machiavelli if you want the old one people still quote.",
    ),
    (
        "poetry",
        "*Staying Alive* edited by Neil Astley is the anthology to start with: modern poems in English, sorted by what they are about rather than by poet. Read one a night and skip anything that does not land.",
    ),
    (
        "language",
        "*The Language Instinct* by Steven Pinker is the readable one on how language works in the head, and *Mother Tongue* by Bill Bryson is the fun history of English if that is the half you want.",
    ),
    (
        "gardening",
        "*How to Garden* (DK) is the beginner's standard: a photograph for every job, month by month, and no assumed knowledge. *The Allotment Book* by Andi Clevely if it is a plot rather than a border.",
    ),
    (
        "chess",
        "*Bobby Fischer Teaches Chess* is all positions and no theory, so you learn by doing, and free games online afterwards make it stick. *Chess for Dummies* if you want the openings named as well.",
    ),
    (
        "knitting",
        "*Stitch 'n Bitch* by Debbie Stoller: cast on, knit, purl, then patterns you can actually finish. A pair of 4mm needles and a ball of aran wool is the whole kit and it costs about ten pounds.",
    ),
    (
        "crochet",
        "*The Happy Hooker* by Debbie Stoller is the crochet one from the same series: the stitches step by step, then small things worth making. One 4mm hook and a ball of aran wool to start.",
    ),
    (
        "sewing",
        "*The Sewing Book* by Alison Smith (DK) photographs every technique from a hem to a zip, so it stays useful for years. Practise on an old pillowcase rather than good fabric.",
    ),
    (
        "drawing",
        "*Drawing on the Right Side of the Brain* by Betty Edwards, because beginners draw what they think is there instead of what is, which is the thing the exercises fix. A 2B pencil and cheap paper is the kit.",
    ),
    (
        "painting",
        "*Watercolour for the Absolute Beginner* by Matthew Palmer: three washes and you have painted something, with nothing assumed. Cold-pressed 300gsm paper decides more than the paints do, and acrylics are the easier way in.",
    ),
    (
        "photography",
        "*Understanding Exposure* by Bryan Peterson, since exposure is what separates a snapshot from a photograph and the book works on any camera, phone included.",
    ),
    (
        "guitar",
        "*Hal Leonard Guitar Method, Book 1*: chords and songs from the first pages, with the audio included. Any cheap acoustic plays well enough to learn on.",
    ),
    (
        "piano",
        "*Alfred's Basic Adult Piano Course, Book 1* starts at the very beginning and has both hands together within a few pages. A 61-key keyboard is enough to learn on.",
    ),
    (
        "baking",
        "*Flour Water Salt Yeast* by Ken Forkish for bread: four ingredients, a schedule, and a loaf that tastes like a bakery's. *The Great British Bake Off: How to Bake* if it is cakes and pastry you want.",
    ),
    (
        "cooking",
        "*Salt, Fat, Acid, Heat* by Samin Nosrat teaches why a dish works, so you stop following recipes. *Delia's Complete Cookery Course* if you would rather be told exactly what to do.",
    ),
    (
        "running",
        "*Run Walk Run* by Jeff Galloway: walking breaks get a beginner to 5k without the injuries that stop people. Get shoes fitted at a running shop rather than bought from a supermarket.",
    ),
    (
        "yoga",
        "*Light on Yoga* by B.K.S. Iyengar is the classic reference, but go to two or three classes first, because a book cannot see your back.",
    ),
    (
        "dog training",
        "*Perfect Puppy* by Gwen Bailey covers the first weeks, house training and the five things to teach first. *The Dog Listener* by Jan Fennell if the dog is out of puppyhood already.",
    ),
    (
        "birdwatching",
        "*The RSPB Pocket Guide to British Birds* names what you are looking at, and a pair of 8x42 binoculars is the one purchase: second hand is fine, and the garden birds are the place to start.",
    ),
    (
        "astronomy",
        "*Turn Left at Orion* by Guy Consolmagno: what to look for first from a back garden, and where it is. Binoculars find most of it, so no telescope yet.",
    ),
    (
        "coding",
        "*Python Crash Course* by Eric Matthes, which has you build a game and a small web app instead of reading about variables. Free tutorials afterwards are fine.",
    ),
    (
        "investing",
        "*Smarter Investing* by Tim Hale: the boring index-fund argument, with the numbers and the costs. Nothing else is needed until that makes sense.",
    ),
    (
        "money",
        "*Money: A User's Guide* by Laura Whateley takes bills, debt, pensions and savings in the order they matter for somebody in the UK.",
    ),
    (
        "history",
        "*A Little History of the World* by E. H. Gombrich: short chapters, written for a child but never childish, and it gives you the map everything else hangs on.",
    ),
    (
        "wine",
        "*Wine Folly* by Madeline Puckette shows the grapes and regions as pictures, which is how they stick, and the tasting words come after.",
    ),
    (
        "woodworking",
        "*Woodworking Basics* by Peter Korn takes one joint at a time with hand tools, which is the right order. A hardpoint saw, a chisel, a square and a bench hook do more than a garage of machines.",
    ),
    (
        "fishing",
        "*Fishing for Dummies* covers the rig, the bait and the knots for coarse fishing, and a day ticket at a club water is the way to try it. Rod licence first if you are in England or Wales.",
    ),
    (
        "climbing",
        "*How to Rock Climb!* by John Long is the standard for the basics, ropes and knots. Go to an indoor wall with hired shoes and try bouldering first, since it needs no partner.",
    ),
    (
        "diy",
        "*The Reader's Digest Complete DIY Manual* covers the jobs in a house from a shelf to a radiator, and a Haynes manual for your own model if it is the car.",
    ),
    (
        "car",
        "A Haynes manual for your exact model, since the engine has to match, plus a socket set. That covers servicing, brakes and most of an MOT.",
    ),
];

pub(super) const CHILD_BOOK_WORDS: &[&str] = &[
    "daughter",
    "son",
    "child",
    "kid",
    "year old",
    "grandson",
    "granddaughter",
    "niece",
    "nephew",
    "teenager",
];

pub(super) const BABY_BOOK_WORDS: &[&str] = &[
    "baby",
    "babies",
    "toddler",
    "one year old",
    "two year old",
    "18 months",
    "six months",
    "nine months",
    "newborn",
];

pub(super) const CLASS_BOOK_WORDS: &[&str] = &[
    "my class",
    "the class",
    "assembly",
    "story time",
    "storytime",
    "reception class",
    "nursery",
];

pub(super) const BOOK_COMPLAINT: &str = "Put it down - finishing a book you are not enjoying is not a virtue, and 50 pages is a fair trial before you give up on it. If you want another: *The Thursday Murder Club* by Richard Osman is funny with short chapters, *The Silent Patient* if you want a twist, and *The Midnight Library* by Matt Haig if you want something to think about. What did the last one you loved have that this one has not?";

pub(super) const BOOK_FOR_BABY: &str = "For a baby, board books that survive being chewed and pulled: *The Very Hungry Caterpillar* by Eric Carle, *Dear Zoo* by Rod Campbell, *Where's Spot?* by Eric Hill, or any of the *That's Not My...* touchy-feely books. Read the same three or four over and over - the repetition is the point at that age - and let them turn the pages, even if it slows you down.";

pub(super) const BOOK_FOR_CLASS: &str = "For reading aloud to a group: *The Gruffalo* by Julia Donaldson for the rhymes, *The Tiger Who Came to Tea* by Judith Kerr for the pause at the doorbell, and *Oi Frog!* by Kes Gray for the joke on every page. *The Gruffalo* is the safest one for a mixed group: the rhythm carries it and there is a repeated line they can join in with.";

pub(super) const BOOK_FOR_CHILD: &str = "For a younger reader: *The Bolds* by Julian Clary (funny on every page), *The Worst Witch* (short chapters), *The Explorer* by Katherine Rundell for animals and adventure, or *Dog Man* if comics are easier. Start with *The Bolds* or *Dog Man*: jokes first, and no long descriptions to get through.";

pub(super) const BOOK_GENERAL: &str = "*The Thursday Murder Club* by Richard Osman is the safest pick: funny, short chapters, easy to pick up. For something darker, *The Silent Patient*; for a younger reader, *Wonder* by R. J. Palacio or *The Explorer* by Katherine Rundell. Tell me what you last enjoyed and I'll narrow it down.";

pub(super) const KIND_WORDS: &[(&str, &str)] = &[
    ("manga", COMIC_MANGA),
    ("marvel", COMIC_MARVEL),
    ("dc", COMIC_DC),
    ("batman", COMIC_DC),
    ("superman", COMIC_DC),
];
