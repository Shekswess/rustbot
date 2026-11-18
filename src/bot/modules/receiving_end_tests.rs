//! Repeated-role and protected-thread replies from PyBot 7d63c7f.

use super::*;
use crate::bot::Exchange;

const TURNS: &[(&str, &str, &str)] = &[
    (
        "i chatted with my colleague",
        "I understand. Ask away.",
        "greeting",
    ),
    ("My father died last week.", "I'm here to listen.", "chat"),
    (
        "My aunt passed away in January.",
        "Thanks for letting me know.",
        "greeting",
    ),
    ("The laptop died this morning.", "What happened?", "chat"),
    (
        "That was a difficult week.",
        "I'm so sorry to hear that. It's a hard thing to carry — I'm here if you want to talk about it, or if anything practical would help.",
        "grief",
    ),
    (
        "Unrelated new subject now.",
        "There is always a choice to make.",
        "chat",
    ),
    (
        "I fell and injured my arm.",
        "The first step is to get help.",
        "chat",
    ),
    (
        "My arm has a bruise.",
        "Could you tell me a little more?",
        "chat",
    ),
    (
        "Urgent issue.",
        "Emergency services can help. Tell them you are outside the building.",
        "safety",
    ),
    ("Okay.", "Keep talking to the operator.", "greeting"),
    (
        "What about this?",
        "Emergency services will come to the building.",
        "answers",
    ),
    (
        "What about this?",
        "The building is closed today.",
        "answers",
    ),
    ("New topic now.", "", "chat"),
    (
        "That is a brush-off rather than an answer. Nothing to look up here, but the move that works is a short email: say what they said, what you want changed, and ask for a reply by a date. If the date passes, that email is what you take one rung above them, or to ACAS if it is a workplace.",
        "Noted.",
        "chat",
    ),
    (
        "There is a shape to these answers and it is worth naming: ask for the reason in writing, once, with the date you want it by. A recorded refusal is something you can challenge, and the complaints procedure is the route - it has to end somewhere, usually with the ombudsman.",
        "Noted.",
        "chat",
    ),
    (
        "That is a pattern rather than one bad day, so put it in writing rather than say it again: what you want changed, with dates. Nothing to look up here, but the complaints procedure is where this goes next - ask for a copy of it and follow the step it names.",
        "Noted.",
        "chat",
    ),
    ("An empty earlier response", "", "greeting"),
    ("Safety timing issue", "", "safety"),
    (
        "An unrelated thought.",
        "My reply is just a stub.",
        "safety_cards",
    ),
    ("A note about it.", "Nothing new today.", "carers_turn"),
];

const REPLIES: &[&str] = &[
    "Nothing in that to look up, but the step is the same as for any pattern like this: put it in writing once - the date, what happened, what you want - and keep a copy. If it does not come back in a week, it goes one rung above your boss, and ACAS gives free advice on the wording if it is a workplace.",
    "That is a pattern rather than one bad day, so put it in writing rather than say it again: what you want changed, with dates. Nothing to look up here, but the complaints procedure is where this goes next - ask for a copy of it and follow the step it names.",
    "There is a shape to these answers and it is worth naming: ask for the reason in writing, once, with the date you want it by. A recorded refusal is something you can challenge, and the complaints procedure is the route - it has to end somewhere, usually with the ombudsman.",
    "That is a brush-off rather than an answer. Nothing to look up here, but the move that works is a short email: say what they said, what you want changed, and ask for a reply by a date. If the date passes, that email is what you take one rung above them, or to ACAS if it is a workplace.",
    "When the answer is that is just how it is, ask once more in writing - what you want changed and by when - and keep the reply. That email is what the next rung reads: ACAS for a workplace, or the complaints procedure every school, council and surgery has to follow.",
    "Nothing in that to look up, but the step is the same as for any pattern like this: put it in writing once - the date, what happened, what you want - and keep a copy. If it does not come back in a week, it goes one rung above your supervisor, and ACAS gives free advice on the wording if it is a workplace.",
    "Nothing to look up here, but do not leave it spoken: put the request in writing with a date on it and keep a copy. If your landlord will not answer that, the next step is one rung above them, and after that the ombudsman, which is free to use and is where a case finally gets looked at.",
    "When it happens again and again the wording decides it: dates, what you asked for, what happened next. Send that once to your new agency and ask for an answer by a date. If it does not come, ask for the written complaints or grievance procedure and follow the step it names.",
    "When it happens again and again the wording decides it: dates, what you asked for, what happened next. Send that once to the jobcentre and ask for an answer by a date. If it does not come, ask for the written complaints or grievance procedure and follow the step it names.",
    "When it happens again and again the wording decides it: dates, what you asked for, what happened next. Send that once to the teacher and ask for an answer by a date. If it does not come, ask for the written complaints or grievance procedure and follow the step it names.",
    "When it happens again and again the wording decides it: dates, what you asked for, what happened next. Send that once to their teacher and ask for an answer by a date. If it does not come, ask for the written complaints or grievance procedure and follow the step it names.",
    "When it happens again and again the wording decides it: dates, what you asked for, what happened next. Send that once to your boss and ask for an answer by a date. If it does not come, ask for the written complaints or grievance procedure and follow the step it names.",
    "Keep a short record - the date, what happened, what you asked for. A record is what makes your landlord have to answer, and it is what the next rung reads. Every school, council and GP surgery has a complaints procedure with named steps, and asking for a copy is the first one.",
    "Nothing in that to look up, but the step is the same as for any pattern like this: put it in writing once - the date, what happened, what you want - and keep a copy. If it does not come back in a week, it goes one rung above the school, and ACAS gives free advice on the wording if it is a workplace.",
    "Nothing to look up here, but do not leave it spoken: put the request in writing with a date on it and keep a copy. If your boss will not answer that, the next step is one rung above them, and after that the ombudsman, which is free to use and is where a case finally gets looked at.",
    "Nothing in that to look up, but the step is the same as for any pattern like this: put it in writing once - the date, what happened, what you want - and keep a copy. If it does not come back in a week, it goes one rung above your manager, and ACAS gives free advice on the wording if it is a workplace.",
    "Keep a short record - the date, what happened, what you asked for. A record is what makes your boss have to answer, and it is what the next rung reads. Every school, council and GP surgery has a complaints procedure with named steps, and asking for a copy is the first one.",
    "Keep a short record - the date, what happened, what you asked for. A record is what makes your manager have to answer, and it is what the next rung reads. Every school, council and GP surgery has a complaints procedure with named steps, and asking for a copy is the first one.",
    "Nothing to look up here, but do not leave it spoken: put the request in writing with a date on it and keep a copy. If your line manager will not answer that, the next step is one rung above them, and after that the ombudsman, which is free to use and is where a case finally gets looked at.",
];

type Case = (&'static str, &'static [usize], Option<usize>);
const CASES: &[Case] = &[
    ("my boss keeps moving the goalposts", &[], Some(0)),
    ("my manager keeps changing things every week", &[], Some(1)),
    ("my boss is always taking credit for my work", &[], Some(1)),
    ("the school says thats life", &[], Some(2)),
    ("my landlord shrugs it off", &[], Some(3)),
    (
        "the landlord says everyone has to put up with it",
        &[],
        Some(2),
    ),
    ("my line manager keeps saying we'll see", &[], Some(2)),
    ("my manager says it's just how it is", &[], Some(4)),
    ("my supervisor always ignores me", &[], Some(5)),
    ("my old boss always gave me a lift home", &[], None),
    ("my colleague keeps bringing in cake", &[], None),
    ("my teacher always said I was good at maths", &[], None),
    ("my council wont reply", &[], None),
    ("my landlord won't even answer", &[], Some(6)),
    ("HR shrugs it off", &[], Some(4)),
    ("my HR shrugs it off", &[], Some(3)),
    ("the gp surgery keeps closing early", &[], Some(1)),
    ("GP surgery keeps ignoring me", &[], None),
    ("the letting agent keeps forgetting", &[], None),
    ("my new agency always forgets my shifts", &[], Some(7)),
    ("my team lead says we all have to keep working", &[], None),
    ("the jobcentre ignores me", &[], Some(8)),
    ("my headteacher keeps dismissing what I say", &[], None),
    ("my son's teacher keeps losing his work", &[], Some(9)),
    ("her teacher always ignores me", &[], Some(10)),
    ("his teacher always ignores me", &[], Some(1)),
    ("their council always forgets it", &[], Some(1)),
    ("my friend ignores me", &[], None),
    ("can my boss ignore me?", &[], Some(11)),
    ("call the manager", &[], None),
    ("my boss keeps talking to me about matters", &[], None),
    ("my landlord never listens", &[], Some(12)),
    ("the school never says anything", &[], Some(13)),
    ("my boss keeps changing after a great day", &[], Some(1)),
    ("my boss keeps my name hidden", &[], None),
    (
        "my landlord says we all have to put up with it",
        &[],
        Some(2),
    ),
    ("my manager always wanted me to go first", &[], None),
    ("my teacher was good and keeps losing books", &[], None),
    ("my supervisor constantly blames me", &[], Some(1)),
    ("the dentist ignores me", &[], None),
    ("doctors surgery ignores it", &[], Some(1)),
    ("my team leader says its just the way it is", &[], Some(3)),
    ("my boss shrugs and says it's fine", &[], Some(14)),
    ("my boss says to get on with it", &[], Some(2)),
    ("my boss says to just put up with it", &[], Some(14)),
    ("The new shop said hello.", &[], None),
    ("our manager never gets back to us", &[], Some(15)),
    ("my boss and my manager keep forgetting", &[], None),
    ("My Boss keeps moving the goalposts!", &[], Some(0)),
    ("My boss keeps  changing things", &[], Some(16)),
    (
        "My boss says xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx ignores me",
        &[],
        None,
    ),
    (
        "My boss says xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx ignores me",
        &[],
        None,
    ),
    (
        "My boss is very nice, but my teacher ignores me.",
        &[],
        Some(1),
    ),
    ("my landlady keeps refusing to answer", &[], None),
    ("my letting agent always jokes about the flat", &[], None),
    ("my boss keeps moving the goalposts", &[0], Some(16)),
    ("my boss keeps moving the goalposts", &[1], None),
    ("my boss keeps moving the goalposts", &[3], Some(16)),
    ("my boss keeps moving the goalposts", &[4], None),
    ("my boss keeps moving the goalposts", &[5], Some(16)),
    ("my boss keeps moving the goalposts", &[7], None),
    ("my boss keeps moving the goalposts", &[8], None),
    ("my boss keeps moving the goalposts", &[8, 9], None),
    ("my boss keeps moving the goalposts", &[8, 10], None),
    ("my boss keeps moving the goalposts", &[8, 11], Some(1)),
    ("my boss keeps moving the goalposts", &[13], None),
    ("my boss keeps moving the goalposts", &[14], None),
    ("my boss keeps moving the goalposts", &[15], None),
    ("my boss keeps moving the goalposts", &[17], Some(16)),
    ("my boss keeps moving the goalposts", &[18], Some(16)),
    ("my boss keeps moving the goalposts", &[8, 18], Some(1)),
    ("my boss keeps moving the goalposts", &[8, 19], Some(1)),
    ("my manager keeps changing things every week", &[0], Some(1)),
    ("my manager keeps changing things every week", &[1], None),
    ("my manager keeps changing things every week", &[3], Some(1)),
    ("my manager keeps changing things every week", &[4], None),
    ("my manager keeps changing things every week", &[5], Some(1)),
    ("my manager keeps changing things every week", &[7], None),
    ("my manager keeps changing things every week", &[8], None),
    ("my manager keeps changing things every week", &[8, 9], None),
    (
        "my manager keeps changing things every week",
        &[8, 10],
        None,
    ),
    (
        "my manager keeps changing things every week",
        &[8, 11],
        Some(17),
    ),
    ("my manager keeps changing things every week", &[13], None),
    ("my manager keeps changing things every week", &[14], None),
    ("my manager keeps changing things every week", &[15], None),
    (
        "my manager keeps changing things every week",
        &[17],
        Some(1),
    ),
    (
        "my manager keeps changing things every week",
        &[18],
        Some(1),
    ),
    (
        "my manager keeps changing things every week",
        &[8, 18],
        Some(17),
    ),
    (
        "my manager keeps changing things every week",
        &[8, 19],
        Some(17),
    ),
    (
        "my boss is always taking credit for my work",
        &[0],
        Some(16),
    ),
    ("my boss is always taking credit for my work", &[1], None),
    (
        "my boss is always taking credit for my work",
        &[3],
        Some(16),
    ),
    ("my boss is always taking credit for my work", &[4], None),
    (
        "my boss is always taking credit for my work",
        &[5],
        Some(16),
    ),
    ("my boss is always taking credit for my work", &[7], None),
    ("my boss is always taking credit for my work", &[8], None),
    ("my boss is always taking credit for my work", &[8, 9], None),
    (
        "my boss is always taking credit for my work",
        &[8, 10],
        None,
    ),
    (
        "my boss is always taking credit for my work",
        &[8, 11],
        Some(11),
    ),
    ("my boss is always taking credit for my work", &[13], None),
    ("my boss is always taking credit for my work", &[14], None),
    ("my boss is always taking credit for my work", &[15], None),
    (
        "my boss is always taking credit for my work",
        &[17],
        Some(16),
    ),
    (
        "my boss is always taking credit for my work",
        &[18],
        Some(16),
    ),
    (
        "my boss is always taking credit for my work",
        &[8, 18],
        Some(11),
    ),
    (
        "my boss is always taking credit for my work",
        &[8, 19],
        Some(11),
    ),
    ("the school says thats life", &[0], Some(2)),
    ("the school says thats life", &[1], None),
    ("the school says thats life", &[3], Some(2)),
    ("the school says thats life", &[4], None),
    ("the school says thats life", &[5], Some(2)),
    ("the school says thats life", &[7], None),
    ("the school says thats life", &[8], None),
    ("the school says thats life", &[8, 9], None),
    ("the school says thats life", &[8, 10], None),
    ("the school says thats life", &[8, 11], Some(2)),
    ("the school says thats life", &[13], None),
    ("the school says thats life", &[14], None),
    ("the school says thats life", &[15], None),
    ("the school says thats life", &[17], Some(2)),
    ("the school says thats life", &[18], Some(2)),
    ("the school says thats life", &[8, 18], Some(2)),
    ("the school says thats life", &[8, 19], Some(2)),
    ("my landlord shrugs it off", &[0], Some(2)),
    ("my landlord shrugs it off", &[1], None),
    ("my landlord shrugs it off", &[3], Some(2)),
    ("my landlord shrugs it off", &[4], None),
    ("my landlord shrugs it off", &[5], Some(2)),
    ("my landlord shrugs it off", &[7], None),
    ("my landlord shrugs it off", &[8], None),
    ("my landlord shrugs it off", &[8, 9], None),
    ("my landlord shrugs it off", &[8, 10], None),
    ("my landlord shrugs it off", &[8, 11], Some(4)),
    ("my landlord shrugs it off", &[13], None),
    ("my landlord shrugs it off", &[14], None),
    ("my landlord shrugs it off", &[15], None),
    ("my landlord shrugs it off", &[17], Some(2)),
    ("my landlord shrugs it off", &[18], Some(2)),
    ("my landlord shrugs it off", &[8, 18], Some(4)),
    ("my landlord shrugs it off", &[8, 19], Some(4)),
    (
        "the landlord says everyone has to put up with it",
        &[0],
        Some(2),
    ),
    (
        "the landlord says everyone has to put up with it",
        &[1],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[3],
        Some(2),
    ),
    (
        "the landlord says everyone has to put up with it",
        &[4],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[5],
        Some(2),
    ),
    (
        "the landlord says everyone has to put up with it",
        &[7],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[8],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[8, 9],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[8, 10],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[8, 11],
        Some(2),
    ),
    (
        "the landlord says everyone has to put up with it",
        &[13],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[14],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[15],
        None,
    ),
    (
        "the landlord says everyone has to put up with it",
        &[17],
        Some(2),
    ),
    (
        "the landlord says everyone has to put up with it",
        &[18],
        Some(2),
    ),
    (
        "the landlord says everyone has to put up with it",
        &[8, 18],
        Some(2),
    ),
    (
        "the landlord says everyone has to put up with it",
        &[8, 19],
        Some(2),
    ),
    ("my line manager keeps saying we'll see", &[0], Some(3)),
    ("my line manager keeps saying we'll see", &[1], None),
    ("my line manager keeps saying we'll see", &[3], Some(3)),
    ("my line manager keeps saying we'll see", &[4], None),
    ("my line manager keeps saying we'll see", &[5], Some(3)),
    ("my line manager keeps saying we'll see", &[7], None),
    ("my line manager keeps saying we'll see", &[8], None),
    ("my line manager keeps saying we'll see", &[8, 9], None),
    ("my line manager keeps saying we'll see", &[8, 10], None),
    ("my line manager keeps saying we'll see", &[8, 11], Some(18)),
    ("my line manager keeps saying we'll see", &[13], None),
    ("my line manager keeps saying we'll see", &[14], None),
    ("my line manager keeps saying we'll see", &[15], None),
    ("my line manager keeps saying we'll see", &[17], Some(3)),
    ("my line manager keeps saying we'll see", &[18], Some(3)),
    ("my line manager keeps saying we'll see", &[8, 18], Some(18)),
    ("my line manager keeps saying we'll see", &[8, 19], Some(18)),
    ("my manager says it's just how it is", &[0], Some(3)),
    ("my manager says it's just how it is", &[1], None),
    ("my manager says it's just how it is", &[3], Some(3)),
    ("my manager says it's just how it is", &[4], None),
    ("my manager says it's just how it is", &[5], Some(3)),
    ("my manager says it's just how it is", &[7], None),
    ("my manager says it's just how it is", &[8], None),
    ("my manager says it's just how it is", &[8, 9], None),
    ("my manager says it's just how it is", &[8, 10], None),
    ("my manager says it's just how it is", &[8, 11], Some(4)),
    ("my manager says it's just how it is", &[13], None),
    ("my manager says it's just how it is", &[14], None),
    ("my manager says it's just how it is", &[15], None),
    ("my manager says it's just how it is", &[17], Some(3)),
    ("my manager says it's just how it is", &[18], Some(3)),
    ("my manager says it's just how it is", &[8, 18], Some(4)),
    ("my manager says it's just how it is", &[8, 19], Some(4)),
    (
        "my boss keeps moving the goalposts",
        &[8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
        None,
    ),
    (
        "my boss keeps moving the goalposts",
        &[8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
        Some(16),
    ),
    ("my boss keeps moving the goalposts", &[4, 12], None),
    ("my boss keeps moving the goalposts", &[4, 16], None),
    ("my boss keeps moving the goalposts", &[7, 16, 5], Some(1)),
    ("my boss keeps moving the goalposts", &[1, 16, 5, 16], None),
    (
        "my boss keeps moving the goalposts",
        &[1, 16, 5, 16, 5],
        Some(16),
    ),
];

#[test]
fn named_roles_keep_the_source_wording_and_protected_context_fences() {
    let mut differences = Vec::new();
    for &(line, prior, index) in CASES {
        let memory = Memory {
            turns: prior
                .iter()
                .map(|&index| {
                    let (user, bot, module) = TURNS[index];
                    Exchange {
                        user: user.to_owned(),
                        bot: bot.to_owned(),
                        module,
                    }
                })
                .collect(),
            ..Memory::default()
        };
        let reading = Reading::understand(
            line,
            memory.turns.iter().map(|turn| turn.user.as_str()),
            None,
            &memory.prior_readings,
        );
        let actual = respond(&reading, &memory);
        let expected = index.map(|index| REPLIES[index]);
        if actual.as_deref() != expected {
            differences.push((line, prior, expected, actual));
        }
    }
    assert!(differences.is_empty(), "{differences:#?}");
}

#[test]
fn the_wording_seed_keeps_the_source_apostrophe() {
    for (line, reply) in [
        (
            "my landlord won’t even answer",
            Some(
                "When the answer is that is just how it is, ask once more in writing - what you want changed and by when - and keep the reply. That email is what the next rung reads: ACAS for a workplace, or the complaints procedure every school, council and surgery has to follow.",
            ),
        ),
        (
            "My LANDLORD won’t even answer!",
            Some(
                "When the answer is that is just how it is, ask once more in writing - what you want changed and by when - and keep the reply. That email is what the next rung reads: ACAS for a workplace, or the complaints procedure every school, council and surgery has to follow.",
            ),
        ),
        (
            "my landlord won't even answer",
            Some(
                "Nothing to look up here, but do not leave it spoken: put the request in writing with a date on it and keep a copy. If your landlord will not answer that, the next step is one rung above them, and after that the ombudsman, which is free to use and is where a case finally gets looked at.",
            ),
        ),
        (
            "my boss keeps saying we’ll see",
            Some(
                "There is a shape to these answers and it is worth naming: ask for the reason in writing, once, with the date you want it by. A recorded refusal is something you can challenge, and the complaints procedure is the route - it has to end somewhere, usually with the ombudsman.",
            ),
        ),
    ] {
        assert_eq!(
            respond(&Reading::new(line), &Memory::default()).as_deref(),
            reply,
            "{line}"
        );
    }
}
