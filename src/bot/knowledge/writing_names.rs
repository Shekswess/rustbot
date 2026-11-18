//! Authored story ideas and names from PyBot's entertainment module.
use super::Form;
use regex::Regex;
use std::sync::LazyLock;

const CHARACTER_NAME_RULES: &[&str] = &[
    "Pick a plain first name and a surname that says what they do or where they are from: Nell Harrow, Amos Pike, Ivy Sanderson. Write down three and the story usually picks one itself - and say the full name out loud twice, because a name you stumble over is one the reader will too.",
    "A name that is easy to say and slightly odd is the one that sticks: Ruth Okonkwo, Samuel Beck, Ada Pryce. Give the villain the smoother name and the hero the plainer one, and skip the apostrophes - nobody remembers how to spell them.",
    "Old-fashioned first names worn by young people: Walter, Mabel, Agnes, Ernest. It does the work of a description in one word, because the reader fills in the corners, and it fits every kind of story from a farm to a space station.",
];
const NICK_FIRST: &[&str] = &[
    "Captain", "Big", "Tiny", "Old", "Sir", "Lady", "Doctor", "Saint",
];
const NICK_LAST: &[&str] = &[
    "Chaos", "Thunder", "Biscuit", "Trouble", "Sunshine", "Nonsense", "Legend", "Saturday",
    "Pockets", "Rocket",
];
const STORY_IDEAS: &[&str] = &[
    "A person finds a door in their own house that they have never opened, and the key is in their coat pocket.",
    "Two neighbours share a wall and a secret, and only one of them knows it is shared.",
    "A letter arrives forty years late, and the person it was meant for is still at the same address.",
    "Someone who has never left their village has to cross a city in one night.",
    "A family inherits a shop that comes with one customer who will not go away.",
    "A rescue dog remembers the person who lost it, and finds them in a supermarket queue.",
    "The last person on a night shift starts getting phone calls from the empty office upstairs.",
    "A cook loses their sense of taste the week they are cooking for the person they want to impress.",
    "A child's imaginary friend turns up at the school gate, and only the child can see them.",
    "A retired teacher keeps the exercise books of one pupil, and finally works out why.",
    "A train stops at a station that is not on the map, and one passenger gets off.",
    "Two strangers are given the same locker key at a swimming pool.",
    "A man who fixes everything in the street cannot fix the one thing in his own house.",
    "A woman starts receiving post addressed to her in her own handwriting, dated next year.",
    "A wedding photographer finds someone in the background of every photo who was never invited.",
    "A town agrees to forget one thing together, and one person refuses.",
];
const QUIZ_NAMES: &[&str] = &[
    "Quizteam Aguilera",
    "Les Quizerables",
    "Norfolk 'n' Chance",
    "The Quizzly Bears",
    "Sultans of Quiz",
    "Trivia Newton-John",
    "University of Trivia",
    "Agatha Quiztie",
    "Let's Get Quizzical",
    "The Also Rans",
];
const TEAM_NAMES: &[&str] = &[
    "The Long Shots",
    "Second Wind",
    "The Underdogs",
    "The Night Owls",
    "The Wild Cards",
    "The Try Hards",
    "The Comeback Club",
    "The Early Birds",
];

static CHARACTER_ROLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:villain|villainess|baddie|antagonist|hero|heroine)\b").unwrap()
});
static QUIZ: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(?:quiz|trivia)\b").unwrap());

pub(super) fn reply(form: Form, line: &str) -> Option<String> {
    let seed = line.chars().map(|letter| letter as usize).sum::<usize>();
    match form {
        Form::CharacterName => {
            let index = if CHARACTER_ROLE.is_match(line) {
                1
            } else {
                seed % CHARACTER_NAME_RULES.len()
            };
            Some(CHARACTER_NAME_RULES[index].into())
        }
        Form::Nickname => {
            let [first, second, third] = [0, 1, 2].map(|step| {
                let start = NICK_FIRST[(seed + step * 3) % NICK_FIRST.len()];
                let end = NICK_LAST[(seed / 7 + step * 5) % NICK_LAST.len()];
                format!("{start} {end}")
            });
            Some(format!(
                "Start with *{first}*; *{second}* and *{third}* if it sticks. A nickname works best when it comes from something only you two know."
            ))
        }
        Form::StoryIdea => {
            let seed = seed.max(1);
            let [first, second] = [0, 7].map(|step| STORY_IDEAS[(seed + step) % STORY_IDEAS.len()]);
            Some(format!(
                "Two to start with:\n- {first}\n- {second}\nWhich one sounds like yours? I can help you find the first scene, or the thing that goes wrong."
            ))
        }
        Form::QuizName => {
            let seed = seed.max(1);
            let pool = if QUIZ.is_match(line) {
                QUIZ_NAMES
            } else {
                TEAM_NAMES
            };
            let [first, second, third] = [0, 3, 6].map(|step| pool[(seed + step) % pool.len()]);
            Some(format!(
                "For the sheet: *{first}*. Others that go down well: *{second}* and *{third}*."
            ))
        }
        _ => None,
    }
}
