//! Direct callback replies captured from PyBot 7d63c7f.
use super::*;

struct Case {
    form: Form,
    line: &'static str,
    earlier: &'static [(&'static str, &'static str)],
    reply: Option<&'static str>,
}

const CASES: &[Case] = &[
    Case {
        form: Form::StoryIdea,
        line: "give me a story idea",
        earlier: &[],
        reply: Some(
            "Two to start with:\n- A letter arrives forty years late, and the person it was meant for is still at the same address.\n- A retired teacher keeps the exercise books of one pupil, and finally works out why.\nWhich one sounds like yours? I can help you find the first scene, or the thing that goes wrong.",
        ),
    },
    Case {
        form: Form::StoryIdea,
        line: "i need a plot idea",
        earlier: &[],
        reply: Some(
            "Two to start with:\n- A child's imaginary friend turns up at the school gate, and only the child can see them.\n- A town agrees to forget one thing together, and one person refuses.\nWhich one sounds like yours? I can help you find the first scene, or the thing that goes wrong.",
        ),
    },
    Case {
        form: Form::CharacterName,
        line: "give me a character name",
        earlier: &[],
        reply: Some(
            "Pick a plain first name and a surname that says what they do or where they are from: Nell Harrow, Amos Pike, Ivy Sanderson. Write down three and the story usually picks one itself - and say the full name out loud twice, because a name you stumble over is one the reader will too.",
        ),
    },
    Case {
        form: Form::CharacterName,
        line: "name my character the villain",
        earlier: &[],
        reply: Some(
            "A name that is easy to say and slightly odd is the one that sticks: Ruth Okonkwo, Samuel Beck, Ada Pryce. Give the villain the smoother name and the hero the plainer one, and skip the apostrophes - nobody remembers how to spell them.",
        ),
    },
    Case {
        form: Form::CharacterName,
        line: "i need a name for my protagonist",
        earlier: &[],
        reply: Some(
            "Old-fashioned first names worn by young people: Walter, Mabel, Agnes, Ernest. It does the work of a description in one word, because the reader fills in the corners, and it fits every kind of story from a farm to a space station.",
        ),
    },
    Case {
        form: Form::Nickname,
        line: "make up a rap name",
        earlier: &[],
        reply: Some(
            "Start with *Captain Rocket*; *Old Sunshine* and *Doctor Rocket* if it sticks. A nickname works best when it comes from something only you two know.",
        ),
    },
    Case {
        form: Form::Nickname,
        line: "give me a nickname for mark",
        earlier: &[],
        reply: Some(
            "Start with *Doctor Nonsense*; *Big Chaos* and *Sir Nonsense* if it sticks. A nickname works best when it comes from something only you two know.",
        ),
    },
    Case {
        form: Form::QuizName,
        line: "what should we call our pub quiz team",
        earlier: &[],
        reply: Some(
            "For the sheet: *University of Trivia*. Others that go down well: *The Also Rans* and *Norfolk 'n' Chance*.",
        ),
    },
    Case {
        form: Form::QuizName,
        line: "name ideas for our netball team",
        earlier: &[],
        reply: Some(
            "For the sheet: *The Try Hards*. Others that go down well: *The Long Shots* and *The Night Owls*.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "write me a haiku",
        earlier: &[],
        reply: Some(
            "A haiku, then:\nCold tap in the yard,\nthe kettle ticks itself warm,\nmorning starts again.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "give me a haiku about the sea",
        earlier: &[],
        reply: Some(
            "A haiku about the sea, 5-7-5:\nThe sea keeps its own\nslow time, and the tide turns twice\nbefore anyone looks.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "write a haiku about cupboards",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Haiku,
        line: "write a haiku about grief",
        earlier: &[],
        reply: Some(
            "A haiku about grief needs one real thing in it: give me a sound, a smell or a day from it and I'll build the haiku round that.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "write me a haiku for me",
        earlier: &[],
        reply: Some(
            "A haiku, then:\nCold tap in the yard,\nthe kettle ticks itself warm,\nmorning starts again.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "write a haiku about my",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Haiku,
        line: "how do I write a haiku",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Haiku,
        line: "write me a sonnet",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Haiku,
        line: "write a haiku about my cat",
        earlier: &[],
        reply: Some(
            "A haiku about my cat, 5-7-5:\nCat on the warm sill,\none eye on the garden birds,\nboth ears fast asleep.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "A haiku about the rain?",
        earlier: &[],
        reply: Some(
            "A haiku about the rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        ),
    },
    Case {
        form: Form::Limerick,
        line: "give me a limerick",
        earlier: &[],
        reply: Some(
            "A limerick, then:\nThere once was a fellow called Stan,\nwho swapped his old bike for a van.\nIt rattles, it groans,\non cobbles and stones,\nand it still beats the bus, said his gran.",
        ),
    },
    Case {
        form: Form::Limerick,
        line: "write a limerick about a man from spain",
        earlier: &[],
        reply: Some(
            "A limerick about a man:\nThere once was a man I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        ),
    },
    Case {
        form: Form::Limerick,
        line: "a limerick about my three ducks",
        earlier: &[],
        reply: Some(
            "A limerick about my three ducks:\nThere once was a duck I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        ),
    },
    Case {
        form: Form::Limerick,
        line: "write a limerick about school",
        earlier: &[],
        reply: Some(
            "A limerick about school:\nThere once was a school I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?\nGive me one real thing from it - a sound, a smell or a day - and I will write the limerick round that.",
        ),
    },
    Case {
        form: Form::Limerick,
        line: "write a limerick for my",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Limerick,
        line: "write a limerick about housework",
        earlier: &[],
        reply: Some(
            "A limerick about housework:\nThere once was a housework I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        ),
    },
    Case {
        form: Form::Poem,
        line: "write me a poem",
        earlier: &[],
        reply: Some(
            "A short poem, then:\nSome days are quiet, some are loud,\nand some you spend inside a cloud.\nThe kettle helps, the window too,\nand so, quite honestly, do you.",
        ),
    },
    Case {
        form: Form::Poem,
        line: "write me a poem about autumn for my daughter's card",
        earlier: &[],
        reply: Some(
            "A short poem about autumn:\nThe leaves let go without a sound\nand lie in gold across the ground.\nThe trees stand bare and do not mind:\nthere is another spring behind.",
        ),
    },
    Case {
        form: Form::Poem,
        line: "a poem about my husband",
        earlier: &[],
        reply: Some(
            "A short poem about my husband:\nYou are the one I tell things to,\nthe dull ones and the hard ones too;\nthe day can take whatever's spare,\nand still I'm glad you're there.",
        ),
    },
    Case {
        form: Form::Poem,
        line: "write a rhyme about christmas",
        earlier: &[],
        reply: Some(
            "A rhyme about christmas:\nI keep a little christmas in mind,\nthe sort of christmas that's hard to find.\nIt does not shout, it does not boast,\nand that's the christmas I like the most.",
        ),
    },
    Case {
        form: Form::Poem,
        line: "write a poem about my favourite thing",
        earlier: &[],
        reply: Some(
            "A short poem about my favourite thing:\nI keep a little thing in mind,\nthe sort of thing that's hard to find.\nIt does not shout, it does not boast,\nand that's the thing I like the most.",
        ),
    },
    Case {
        form: Form::Poem,
        line: "write a poem about the dog learning tricks",
        earlier: &[],
        reply: Some(
            "A short poem about the dog:\nI keep a little dog in mind,\nthe sort of dog that's hard to find.\nIt does not shout, it does not boast,\nand that's the dog I like the most.",
        ),
    },
    Case {
        form: Form::Poem,
        line: "write a poem about life",
        earlier: &[],
        reply: Some(
            "A short poem about life:\nI keep a little life in mind,\nthe sort of life that's hard to find.\nIt does not shout, it does not boast,\nand that's the life I like the most.\nGive me one real thing from it - a sound, a smell or a day - and I will write the poem round that.",
        ),
    },
    Case {
        form: Form::Poem,
        line: "write a poem about lockdown",
        earlier: &[],
        reply: Some(
            "A poem about lockdown needs one real thing in it: give me a sound, a smell or a day from it and I'll build the poem round that.",
        ),
    },
    Case {
        form: Form::Poem,
        line: "write a poem about my",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Poem,
        line: "i am writing my own poem",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Poem,
        line: "write a poem about paper planes",
        earlier: &[],
        reply: Some(
            "A short poem about paper planes:\nI keep a little plane in mind,\nthe sort of plane that's hard to find.\nIt does not shout, it does not boast,\nand that's the plane I like the most.",
        ),
    },
    Case {
        form: Form::Rhyme,
        line: "what rhymes with orange",
        earlier: &[],
        reply: Some(
            "Nothing rhymes exactly with *orange*. The near ones are *sporange* (a spore case on a fern), *Blorenge* (a hill in Wales) and the two-word *door hinge*. If you need it for a poem, put *orange* at the end of a line that does not have to rhyme, and rhyme the line before it with something else.",
        ),
    },
    Case {
        form: Form::Rhyme,
        line: "what rhymes with light",
        earlier: &[],
        reply: Some(
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        ),
    },
    Case {
        form: Form::Rhyme,
        line: "any words that rhyme with silver",
        earlier: &[],
        reply: Some(
            "Nothing rhymes exactly with *silver*. *Chilver* (a ewe lamb) is the near miss, and most poets rhyme it with *river* or *deliver* and let the vowel do the work.",
        ),
    },
    Case {
        form: Form::Rhyme,
        line: "what rhymes with the",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Rhyme,
        line: "something rhyming with love",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Rhyme,
        line: "give me a rhyme",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Story,
        line: "tell me a story",
        earlier: &[],
        reply: Some(
            "Here is one, made up on the spot:\nIt started with a small thing nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        ),
    },
    Case {
        form: Form::Story,
        line: "tell me a story about anything",
        earlier: &[],
        reply: Some(
            "Here is one, made up on the spot:\nIt started with a small thing nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        ),
    },
    Case {
        form: Form::Story,
        line: "tell me a story about a lighthouse",
        earlier: &[],
        reply: Some(
            "A short story about a lighthouse:\nThe lighthouse arrived in a box with no sender and no note.\nIt sat in the hall for a week, and then it started to hum.\nShe unscrewed one corner, looked inside, and sat down on the floor.\n\"Well,\" she said to nobody at all, \"that changes the week.\"",
        ),
    },
    Case {
        form: Form::Story,
        line: "a bedtime story for a five year old",
        earlier: &[],
        reply: Some(
            "A short one for a five year old: A little girl put a pea in her pocket and it grew all the way up to the ceiling of her bedroom, where it turned into a beanstalk with a door in it. She did not go in, because it was bedtime, and that is the end. A story for a five year old wants one idea, one small problem, and a safe last line rather than a moral.",
        ),
    },
    Case {
        form: Form::Story,
        line: "a bedtime story for my son about a dinosaur",
        earlier: &[],
        reply: Some(
            "A short one for a child: There was a dinosaur in the garden one Tuesday, and nobody in the house had seen one before.\nThe dog said hello first, then everybody did, and by bedtime it had a name and a place by the door.",
        ),
    },
    Case {
        form: Form::Story,
        line: "tell me a bedtime story about anything",
        earlier: &[],
        reply: Some(
            "A short one for a child: A little girl put a pea in her pocket and it grew all the way up to the ceiling of her bedroom, where it turned into a beanstalk with a door in it. She did not go in, because it was bedtime, and that is the end. A story for a five year old wants one idea, one small problem, and a safe last line rather than a moral.",
        ),
    },
    Case {
        form: Form::Story,
        line: "did you hear the story about the dog",
        earlier: &[],
        reply: Some(
            "I haven't heard that one - go on, tell me. You'll tell it better than I could make it up.",
        ),
    },
    Case {
        form: Form::Story,
        line: "my son keeps telling the same story about school",
        earlier: &[],
        reply: Some(
            "A story they ask for again and again is the good kind - they know it by heart by now. What's it about?",
        ),
    },
    Case {
        form: Form::Story,
        line: "what makes a good story",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Story,
        line: "we tried a bath n a story",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Story,
        line: "tell me a story about my little brother",
        earlier: &[],
        reply: Some(
            "A short story about my little brother:\nHe had carried my little brother from one flat to another for eleven years.\nThe morning he finally opened it, the kettle boiled and the phone rang at the same time.\nHe let the kettle win, and the phone stopped.\nThen he sat at the table with my little brother and read the whole thing twice.",
        ),
    },
    Case {
        form: Form::Story,
        line: "tell me a story about anxiety",
        earlier: &[],
        reply: Some(
            "A story about anxiety needs one real thing in it: give me a sound, a smell or a day from it and I'll build the story round that.",
        ),
    },
    Case {
        form: Form::Rap,
        line: "write me a rap",
        earlier: &[],
        reply: Some(
            "A rap, then:\nYo, the beat is back and it is here to stay,\nI have been up since six and I am out all day.\nThey said it was nothing, I said it was mine,\nso the beat and me, we are doing just fine.",
        ),
    },
    Case {
        form: Form::Rap,
        line: "write a rap about my nan",
        earlier: &[],
        reply: Some(
            "A rap about my nan:\nYo, my nan is back and it is here to stay,\nI have been up since six and I am out all day.\nThey said it was nothing, I said it was mine,\nso my nan and me, we are doing just fine.",
        ),
    },
    Case {
        form: Form::Rap,
        line: "rap about a bicycle",
        earlier: &[],
        reply: Some(
            "A rap about a bicycle:\nYo, the bicycle is back and it is here to stay,\nI have been up since six and I am out all day.\nThey said it was nothing, I said it was mine,\nso the bicycle and me, we are doing just fine.",
        ),
    },
    Case {
        form: Form::Rap,
        line: "write a rap for my card",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Rap,
        line: "he keeps making up raps",
        earlier: &[],
        reply: Some(
            "Making up raps is how a lot of people find their voice. Does any of it stick, or is it all gone the moment it's said?",
        ),
    },
    Case {
        form: Form::Rap,
        line: "i tried a rap, can you write one about cycling",
        earlier: &[],
        reply: Some(
            "A rap about cycling:\nYo, the cycling is back and it is here to stay,\nI have been up since six and I am out all day.\nThey said it was nothing, I said it was mine,\nso the cycling and me, we are doing just fine.",
        ),
    },
    Case {
        form: Form::Song,
        line: "write me a song",
        earlier: &[],
        reply: Some(
            "A song, then:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the tune and I walked away.\nChorus:\nOh, the tune, the tune,\nit comes back around,\nthe tune comes back around.",
        ),
    },
    Case {
        form: Form::Song,
        line: "write a song about the stars",
        earlier: &[],
        reply: Some(
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        ),
    },
    Case {
        form: Form::Song,
        line: "write a song about my little brother",
        earlier: &[],
        reply: Some(
            "A song about my little brother:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about my little brother and I walked away.\nChorus:\nOh, my little brother, my little brother,\nit comes back around,\nmy little brother comes back around.",
        ),
    },
    Case {
        form: Form::Song,
        line: "write a song about my",
        earlier: &[],
        reply: None,
    },
    Case {
        form: Form::Song,
        line: "write a song about work",
        earlier: &[],
        reply: Some(
            "A song about work:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the work and I walked away.\nChorus:\nOh, the work, the work,\nit comes back around,\nthe work comes back around.\nGive me one real thing from it - a sound, a smell or a day - and I will write the song round that.",
        ),
    },
    Case {
        form: Form::Song,
        line: "they keep singing the same song",
        earlier: &[],
        reply: Some(
            "A song sung the same way every time has properly stuck. Do you join in, or is it theirs alone?",
        ),
    },
    Case {
        form: Form::Story,
        line: "one more about a dog",
        earlier: &[(
            "tell me a story about a cat",
            "A short story about a cat:\nIt started with a cat nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        )],
        reply: Some(
            "A short story about a dog:\nHe had carried the dog from one flat to another for eleven years.\nThe morning he finally opened it, the kettle boiled and the phone rang at the same time.\nHe let the kettle win, and the phone stopped.\nThen he sat at the table with the dog and read the whole thing twice.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more about a dog",
        earlier: &[(
            "tell me a story about a cat",
            "A short story about a cat:\nIt started with a cat nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        )],
        reply: Some(
            "A short story about a dog:\nHe had carried the dog from one flat to another for eleven years.\nThe morning he finally opened it, the kettle boiled and the phone rang at the same time.\nHe let the kettle win, and the phone stopped.\nThen he sat at the table with the dog and read the whole thing twice.",
        ),
    },
    Case {
        form: Form::Story,
        line: "one more",
        earlier: &[(
            "tell me a story about a cat",
            "A short story about a cat:\nIt started with a cat nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        )],
        reply: None,
    },
    Case {
        form: Form::Another,
        line: "one more",
        earlier: &[(
            "tell me a story about a cat",
            "A short story about a cat:\nIt started with a cat nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        )],
        reply: None,
    },
    Case {
        form: Form::Story,
        line: "something happier",
        earlier: &[(
            "tell me a story about a cat",
            "A short story about a cat:\nIt started with a cat nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        )],
        reply: None,
    },
    Case {
        form: Form::Another,
        line: "something happier",
        earlier: &[(
            "tell me a story about a cat",
            "A short story about a cat:\nIt started with a cat nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        )],
        reply: None,
    },
    Case {
        form: Form::Story,
        line: "one more beer please",
        earlier: &[(
            "tell me a story about a cat",
            "A short story about a cat:\nIt started with a cat nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        )],
        reply: None,
    },
    Case {
        form: Form::Another,
        line: "one more beer please",
        earlier: &[(
            "tell me a story about a cat",
            "A short story about a cat:\nIt started with a cat nobody in the house had planned for.\nThe Tuesday came and went, and it was still there on the Wednesday.\nBy Friday the whole street had an opinion, and no two of them matched.\nIn the end they kept it, and it kept them.",
        )],
        reply: None,
    },
    Case {
        form: Form::Haiku,
        line: "one more about a dog",
        earlier: &[(
            "write a haiku about rain",
            "A haiku about rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        )],
        reply: Some(
            "A haiku about a dog, 5-7-5:\nWet nose at the door,\nthe whole day arrives at once,\ntail first, always first.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more about a dog",
        earlier: &[(
            "write a haiku about rain",
            "A haiku about rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        )],
        reply: Some(
            "A haiku about a dog, 5-7-5:\nWet nose at the door,\nthe whole day arrives at once,\ntail first, always first.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "one more",
        earlier: &[(
            "write a haiku about rain",
            "A haiku about rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        )],
        reply: Some(
            "A haiku, then:\nCold tap in the yard,\nthe kettle ticks itself warm,\nmorning starts again.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more",
        earlier: &[(
            "write a haiku about rain",
            "A haiku about rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        )],
        reply: Some(
            "A haiku, then:\nCold tap in the yard,\nthe kettle ticks itself warm,\nmorning starts again.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "something happier",
        earlier: &[(
            "write a haiku about rain",
            "A haiku about rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        )],
        reply: Some(
            "A haiku, then:\nCold tap in the yard,\nthe kettle ticks itself warm,\nmorning starts again.",
        ),
    },
    Case {
        form: Form::Another,
        line: "something happier",
        earlier: &[(
            "write a haiku about rain",
            "A haiku about rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        )],
        reply: Some(
            "Something brighter, then.\nThe sun on my back,\nand a chair that is too warm\nto do anything.",
        ),
    },
    Case {
        form: Form::Haiku,
        line: "one more beer please",
        earlier: &[(
            "write a haiku about rain",
            "A haiku about rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        )],
        reply: Some(
            "A haiku, then:\nCold tap in the yard,\nthe kettle ticks itself warm,\nmorning starts again.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more beer please",
        earlier: &[(
            "write a haiku about rain",
            "A haiku about rain, 5-7-5:\nRain on the window,\na hundred small taps, all night,\nand the garden drinks.",
        )],
        reply: None,
    },
    Case {
        form: Form::Limerick,
        line: "one more about a dog",
        earlier: &[(
            "write a limerick about my brother",
            "A limerick about my brother:\nThere once was a brother I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        )],
        reply: Some(
            "A limerick about a dog:\nThere once was a dog called Ned,\nwho slept at the end of the bed.\nHe dreamt of a cat,\nof a bone, and of that\nhalf a sausage he thought he'd been fed.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more about a dog",
        earlier: &[(
            "write a limerick about my brother",
            "A limerick about my brother:\nThere once was a brother I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        )],
        reply: Some(
            "A limerick about a dog:\nThere once was a dog called Ned,\nwho slept at the end of the bed.\nHe dreamt of a cat,\nof a bone, and of that\nhalf a sausage he thought he'd been fed.",
        ),
    },
    Case {
        form: Form::Limerick,
        line: "one more",
        earlier: &[(
            "write a limerick about my brother",
            "A limerick about my brother:\nThere once was a brother I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        )],
        reply: Some(
            "A limerick, then:\nThere once was a fellow called Stan,\nwho swapped his old bike for a van.\nIt rattles, it groans,\non cobbles and stones,\nand it still beats the bus, said his gran.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more",
        earlier: &[(
            "write a limerick about my brother",
            "A limerick about my brother:\nThere once was a brother I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        )],
        reply: Some(
            "A limerick, then:\nThere once was a fellow called Stan,\nwho swapped his old bike for a van.\nIt rattles, it groans,\non cobbles and stones,\nand it still beats the bus, said his gran.",
        ),
    },
    Case {
        form: Form::Limerick,
        line: "something happier",
        earlier: &[(
            "write a limerick about my brother",
            "A limerick about my brother:\nThere once was a brother I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        )],
        reply: Some(
            "A limerick, then:\nThere once was a fellow called Stan,\nwho swapped his old bike for a van.\nIt rattles, it groans,\non cobbles and stones,\nand it still beats the bus, said his gran.",
        ),
    },
    Case {
        form: Form::Another,
        line: "something happier",
        earlier: &[(
            "write a limerick about my brother",
            "A limerick about my brother:\nThere once was a brother I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        )],
        reply: None,
    },
    Case {
        form: Form::Limerick,
        line: "one more beer please",
        earlier: &[(
            "write a limerick about my brother",
            "A limerick about my brother:\nThere once was a brother I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        )],
        reply: Some(
            "A limerick, then:\nThere once was a fellow called Stan,\nwho swapped his old bike for a van.\nIt rattles, it groans,\non cobbles and stones,\nand it still beats the bus, said his gran.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more beer please",
        earlier: &[(
            "write a limerick about my brother",
            "A limerick about my brother:\nThere once was a brother I knew,\nwhose stories were never quite true.\nThey wandered the day,\nand took their own way,\nand honestly, what can you do?",
        )],
        reply: None,
    },
    Case {
        form: Form::Song,
        line: "one more about a dog",
        earlier: &[(
            "write a song about the stars",
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        )],
        reply: Some(
            "A song about a dog:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the dog and I walked away.\nChorus:\nOh, the dog, the dog,\nit comes back around,\nthe dog comes back around.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more about a dog",
        earlier: &[(
            "write a song about the stars",
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        )],
        reply: Some(
            "A song about a dog:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the dog and I walked away.\nChorus:\nOh, the dog, the dog,\nit comes back around,\nthe dog comes back around.",
        ),
    },
    Case {
        form: Form::Song,
        line: "one more",
        earlier: &[(
            "write a song about the stars",
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        )],
        reply: Some(
            "A song, then:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the tune and I walked away.\nChorus:\nOh, the tune, the tune,\nit comes back around,\nthe tune comes back around.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more",
        earlier: &[(
            "write a song about the stars",
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        )],
        reply: Some(
            "A song, then:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the tune and I walked away.\nChorus:\nOh, the tune, the tune,\nit comes back around,\nthe tune comes back around.",
        ),
    },
    Case {
        form: Form::Song,
        line: "something happier",
        earlier: &[(
            "write a song about the stars",
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        )],
        reply: Some(
            "A song, then:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the tune and I walked away.\nChorus:\nOh, the tune, the tune,\nit comes back around,\nthe tune comes back around.",
        ),
    },
    Case {
        form: Form::Another,
        line: "something happier",
        earlier: &[(
            "write a song about the stars",
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        )],
        reply: None,
    },
    Case {
        form: Form::Song,
        line: "one more beer please",
        earlier: &[(
            "write a song about the stars",
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        )],
        reply: Some(
            "A song, then:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the tune and I walked away.\nChorus:\nOh, the tune, the tune,\nit comes back around,\nthe tune comes back around.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more beer please",
        earlier: &[(
            "write a song about the stars",
            "A song about the stars:\nVerse:\nIt was a Tuesday, nothing much to say,\nI thought about the star and I walked away.\nChorus:\nOh, the star, the star,\nit comes back around,\nthe star comes back around.",
        )],
        reply: None,
    },
    Case {
        form: Form::Rhyme,
        line: "one more about a dog",
        earlier: &[(
            "what rhymes with light",
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        )],
        reply: Some(
            "A short poem about a dog:\nI keep a little dog in mind,\nthe sort of dog that's hard to find.\nIt does not shout, it does not boast,\nand that's the dog I like the most.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more about a dog",
        earlier: &[(
            "what rhymes with light",
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        )],
        reply: Some(
            "A short poem about a dog:\nI keep a little dog in mind,\nthe sort of dog that's hard to find.\nIt does not shout, it does not boast,\nand that's the dog I like the most.",
        ),
    },
    Case {
        form: Form::Rhyme,
        line: "one more",
        earlier: &[(
            "what rhymes with light",
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        )],
        reply: Some(
            "A short poem, then:\nSome days are quiet, some are loud,\nand some you spend inside a cloud.\nThe kettle helps, the window too,\nand so, quite honestly, do you.",
        ),
    },
    Case {
        form: Form::Another,
        line: "one more",
        earlier: &[(
            "what rhymes with light",
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        )],
        reply: Some(
            "A short poem, then:\nSome days are quiet, some are loud,\nand some you spend inside a cloud.\nThe kettle helps, the window too,\nand so, quite honestly, do you.",
        ),
    },
    Case {
        form: Form::Rhyme,
        line: "something happier",
        earlier: &[(
            "what rhymes with light",
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        )],
        reply: None,
    },
    Case {
        form: Form::Another,
        line: "something happier",
        earlier: &[(
            "what rhymes with light",
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        )],
        reply: Some(
            "Something brighter, then.\nAnother year, and here you are,\nwith all the miles and all the scars.\nThe cake is small, the wish is too:\nmore of the good ones, sent to you.",
        ),
    },
    Case {
        form: Form::Rhyme,
        line: "one more beer please",
        earlier: &[(
            "what rhymes with light",
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        )],
        reply: None,
    },
    Case {
        form: Form::Another,
        line: "one more beer please",
        earlier: &[(
            "what rhymes with light",
            "Words that rhyme with *light*: night, bright, kite, fright and tight. For a poem, the easiest thing is to put *light* at the end of a line and pick the rhyme that says what you mean.",
        )],
        reply: None,
    },
];

#[test]
fn writing_callbacks_match_the_pinned_engine() {
    let mut differences = Vec::new();
    for case in CASES {
        let earlier: Vec<_> = case
            .earlier
            .iter()
            .map(|&(user, bot)| Exchange {
                user: user.into(),
                bot: bot.into(),
                module: "answers",
            })
            .collect();
        let reading = Reading::new(case.line);
        let response = reply(case.form, &reading, &earlier);
        if response.as_deref() != case.reply {
            differences.push(format!(
                "{:?}: {:?} after {:?}: native {:?}, reference {:?}",
                case.form,
                case.line,
                earlier.last().map(|turn| &turn.user),
                response,
                case.reply
            ));
        }
    }
    assert!(
        differences.is_empty(),
        "mismatches {}/{}\n{}",
        differences.len(),
        CASES.len(),
        differences.join("\n")
    );
}
