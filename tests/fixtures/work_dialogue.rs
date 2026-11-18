//! Exact reply and guard outcomes for the pinned work register.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub Option<bool>,
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "two-people-who-clash",
        "did the world war affect the factory",
        Some(false),
        None,
    ),
    Case(
        "two-people-who-clash",
        "what caused the cold war",
        Some(false),
        None,
    ),
    Case(
        "two-people-who-clash",
        "do two managers get into a war of words",
        Some(true),
        None,
    ),
    Case(
        "two-people-who-clash",
        "why is there a war of nerves",
        Some(true),
        None,
    ),
    Case(
        "two-people-who-clash",
        "the war on price rises",
        Some(false),
        None,
    ),
    Case(
        "two-people-who-clash",
        "does it clash with Vietnam war",
        Some(false),
        None,
    ),
    Case(
        "two-people-who-clash",
        "who won the battle of britain",
        Some(false),
        None,
    ),
    Case(
        "two-people-who-clash",
        "can two team members work through a disagreement",
        Some(true),
        None,
    ),
    Case(
        "two-people-who-clash",
        "can my coworkers stop warring",
        Some(true),
        None,
    ),
    Case("crying-at-work", "i cried at work", Some(true), None),
    Case(
        "crying-at-work",
        "my colleague cried at work",
        Some(false),
        None,
    ),
    Case("crying-at-work", "someone at work cried", Some(false), None),
    Case(
        "crying-at-work",
        "nobody cried at the office",
        Some(true),
        None,
    ),
    Case("crying-at-work", "my boss made me cry", Some(true), None),
    Case(
        "crying-at-work",
        "my boss reduced me to tears",
        Some(false),
        None,
    ),
    Case(
        "crying-at-work",
        "i made my colleague cry",
        Some(false),
        None,
    ),
    Case(
        "crying-at-work",
        "i was crying and my manager helped",
        Some(true),
        None,
    ),
    Case("crying-at-work", "my friend choked up", Some(false), None),
    Case(
        "crying-at-work",
        "she was upset and broke down",
        Some(false),
        None,
    ),
    Case("crying-at-work", "we all cried", Some(true), None),
    Case(
        "crying-at-work",
        "i broke down in the meeting",
        Some(true),
        None,
    ),
    Case("crying-at-work", "i'm in tears", Some(true), None),
    Case("someone-crying", "i cried at work", Some(false), None),
    Case(
        "someone-crying",
        "my colleague cried at work",
        Some(true),
        None,
    ),
    Case("someone-crying", "someone at work cried", Some(true), None),
    Case(
        "someone-crying",
        "nobody cried at the office",
        Some(false),
        None,
    ),
    Case("someone-crying", "my boss made me cry", Some(false), None),
    Case(
        "someone-crying",
        "my boss reduced me to tears",
        Some(true),
        None,
    ),
    Case(
        "someone-crying",
        "i made my colleague cry",
        Some(true),
        None,
    ),
    Case(
        "someone-crying",
        "i was crying and my manager helped",
        Some(false),
        None,
    ),
    Case("someone-crying", "my friend choked up", Some(true), None),
    Case(
        "someone-crying",
        "she was upset and broke down",
        Some(true),
        None,
    ),
    Case("someone-crying", "we all cried", Some(true), None),
    Case(
        "someone-crying",
        "i broke down in the meeting",
        Some(false),
        None,
    ),
    Case("someone-crying", "i'm in tears", Some(false), None),
    Case(
        "commute-long",
        "my commute is 2 hours each way",
        Some(true),
        None,
    ),
    Case(
        "commute-long",
        "commute is only 30 minutes each way",
        Some(true),
        None,
    ),
    Case(
        "commute-long",
        "it costs 6 pounds each way",
        Some(false),
        None,
    ),
    Case(
        "commute-long",
        "commute is 40 miles each way",
        Some(false),
        None,
    ),
    Case("commute-long", "it's 50 miles to work", Some(true), None),
    Case("commute-long", "50 minutes every morning", Some(true), None),
    Case(
        "commute-long",
        "the regular 30 min each way commute exhausts me",
        Some(true),
        None,
    ),
    Case(
        "commute-long",
        "cycle for 20mins each way",
        Some(false),
        None,
    ),
    Case(
        "dress-for-work",
        "what should i wear to the interview",
        Some(true),
        None,
    ),
    Case(
        "dress-for-work",
        "the dress code is smart casual",
        Some(false),
        None,
    ),
    Case(
        "dress-for-work",
        "what should i wear to the wedding",
        Some(false),
        None,
    ),
    Case(
        "dress-for-work",
        "what's the dress code at work?",
        Some(true),
        None,
    ),
    Case(
        "dress-for-work",
        "what should i wear smart but not too formal at a funeral",
        Some(false),
        None,
    ),
    Case("dress-for-work", "a dinner with my boss", Some(true), None),
    Case(
        "dress-for-work",
        "smart clothes for a new school",
        Some(false),
        None,
    ),
    Case(
        "dress-for-work",
        "smart clothes for a trial shift",
        Some(true),
        None,
    ),
    Case("dress-for-work", "which shirt is right", Some(true), None),
    Case("wrong-job", "i don't know what i want", Some(false), None),
    Case(
        "wrong-job",
        "i have no idea what job i want",
        Some(true),
        None,
    ),
    Case(
        "wrong-job",
        "what kind of trousers would suit me",
        Some(false),
        None,
    ),
    Case("wrong-job", "which career would suit me", Some(true), None),
    Case(
        "wrong-job",
        "what jobs can i do as a 16 year old",
        Some(false),
        None,
    ),
    Case(
        "wrong-job",
        "what jobs can a seventeen-years-old do",
        Some(false),
        None,
    ),
    Case(
        "wrong-job",
        "just left school what job do i get",
        Some(false),
        None,
    ),
    Case("wrong-job", "I want to change my job", Some(true), None),
    Case("wrong-job", "what would suit me", Some(false), None),
    Case(
        "wrong-job",
        "am i too late to change role",
        Some(true),
        None,
    ),
    Case(
        "write-a-reference",
        "how do i write a reference for a colleague",
        Some(true),
        None,
    ),
    Case(
        "write-a-reference",
        "who should i use as a reference",
        Some(false),
        None,
    ),
    Case(
        "write-a-reference",
        "do i have to give a reference for a leaver",
        Some(true),
        None,
    ),
    Case(
        "write-a-reference",
        "wording for a former colleague",
        Some(true),
        None,
    ),
    Case(
        "write-a-reference",
        "I was asked to write a reference",
        Some(true),
        None,
    ),
    Case(
        "write-a-reference",
        "is a brief letter enough for a reference",
        Some(true),
        None,
    ),
    Case(
        "write-a-reference",
        "what do they check in a reference",
        Some(false),
        None,
    ),
    Case(
        "interview-prep",
        "how can i prepare for a job interview",
        None,
        Some(
            "Prepare four stories in situation-task-action-result shape: a week you organised, a difficult customer or colleague, something you fixed, and a time you got it wrong and changed. Then write three questions for them about the team, the first six months and what good looks like. Say the answers out loud once - saying it is different from thinking it.",
        ),
    ),
    Case(
        "interview-prep",
        "what happens at a job interview",
        None,
        Some(
            "Nearly always the same shape: someone meets you at reception, a few minutes of small talk on the way in, then twenty to forty minutes of questions about what you have actually done - a time you fixed something, a time you got it wrong - then five or ten minutes for your questions, and a word about next steps or a tour. Answer with examples rather than adjectives, keep one question ready about the team and the first six months, and ask when you will hear back before you leave.",
        ),
    ),
    Case(
        "interview-prep",
        "i'm nervous about my interview",
        None,
        Some(
            "Nerves before an interview are normal, and they shrink once you have something ready. Prepare four stories in situation-task-action-result shape: a week you organised, a difficult customer or colleague, something you fixed, and a time you got it wrong and changed. Then write three questions for them about the team, the first six months and what good looks like. Say the answers out loud once - saying it is different from thinking it.",
        ),
    ),
    Case(
        "interview-prep",
        "what happens if i panic at the job interview",
        None,
        Some(
            "Nearly always the same shape: someone meets you at reception, a few minutes of small talk on the way in, then twenty to forty minutes of questions about what you have actually done - a time you fixed something, a time you got it wrong - then five or ten minutes for your questions, and a word about next steps or a tour. Answer with examples rather than adjectives, keep one question ready about the team and the first six months, and ask when you will hear back before you leave.",
        ),
    ),
    Case(
        "interview-prep",
        "is it normal to be anxious at an interview",
        None,
        Some(
            "Nerves before an interview are normal, and they shrink once you have something ready. Prepare four stories in situation-task-action-result shape: a week you organised, a difficult customer or colleague, something you fixed, and a time you got it wrong and changed. Then write three questions for them about the team, the first six months and what good looks like. Say the answers out loud once - saying it is different from thinking it.",
        ),
    ),
    Case(
        "interview-prep",
        "what is the interview like?",
        None,
        Some(
            "Nearly always the same shape: someone meets you at reception, a few minutes of small talk on the way in, then twenty to forty minutes of questions about what you have actually done - a time you fixed something, a time you got it wrong - then five or ten minutes for your questions, and a word about next steps or a tour. Answer with examples rather than adjectives, keep one question ready about the team and the first six months, and ask when you will hear back before you leave.",
        ),
    ),
    Case(
        "interview-prep",
        "how do i get ready for the interview when i have no experience?",
        None,
        Some(
            "Prepare four stories in situation-task-action-result shape: a week you organised, a difficult customer or colleague, something you fixed, and a time you got it wrong and changed. Then write three questions for them about the team, the first six months and what good looks like. Say the answers out loud once - saying it is different from thinking it.",
        ),
    ),
    Case(
        "say-no-politely",
        "how do i say no politely",
        None,
        Some(
            "Keep it to one short sentence and stop: \"I can't take that on this week.\" Offer what you can do instead - \"I could do Monday morning\" - and if they push back, repeat the same sentence in the same words. One sentence is a boundary; three of apology are an invitation to talk you out of it.",
        ),
    ),
    Case(
        "say-no-politely",
        "how do i say no to a shift politely",
        None,
        Some(
            "Keep it to one short sentence and stop: \"I can't take that on this week.\" Offer what you can do instead - \"I could do Monday morning\" - and if they push back, repeat the same sentence in the same words. One sentence is a boundary; three of apology are an invitation to talk you out of it.",
        ),
    ),
    Case(
        "say-no-politely",
        "How can I refuse some wine politely?",
        None,
        Some(
            "Keep it to four words and take the decision out of it: \"None for me, thank you\", then ask them something so the attention moves off you. A reason at a table is an invitation to be talked round, and one no is enough.",
        ),
    ),
    Case(
        "say-no-politely",
        "is it okay to turn down a second helping",
        None,
        Some(
            "Keep it to four words and take the decision out of it: \"None for me, thank you\", then ask them something so the attention moves off you. A reason at a table is an invitation to be talked round, and one no is enough.",
        ),
    ),
    Case(
        "say-no-politely",
        "I do not like beer but want to say no nicely",
        None,
        Some(
            "Keep it to four words and take the decision out of it: \"None for me, thank you\", then ask them something so the attention moves off you. A reason at a table is an invitation to be talked round, and one no is enough.",
        ),
    ),
    Case(
        "say-no-politely",
        "how do i politely refuse to drink",
        None,
        Some(
            "Keep it to four words and take the decision out of it: \"None for me, thank you\", then ask them something so the attention moves off you. A reason at a table is an invitation to be talked round, and one no is enough.",
        ),
    ),
    Case(
        "say-no-politely",
        "how should i say no to a concert invite?",
        None,
        Some(
            "Keep it to one short sentence and stop: \"I can't take that on this week.\" Offer what you can do instead - \"I could do Monday morning\" - and if they push back, repeat the same sentence in the same words. One sentence is a boundary; three of apology are an invitation to talk you out of it.",
        ),
    ),
    Case(
        "say-no-politely",
        "how do i say no on a foodless outing",
        None,
        Some(
            "Keep it to one short sentence and stop: \"I can't take that on this week.\" Offer what you can do instead - \"I could do Monday morning\" - and if they push back, repeat the same sentence in the same words. One sentence is a boundary; three of apology are an invitation to talk you out of it.",
        ),
    ),
    Case(
        "first-day",
        "what do i bring for my first day at a new job",
        None,
        Some(
            "Take a notebook, a pen, your ID and the paperwork they asked for, and dress one notch smarter than you think you need to. Your only jobs on day one are names, where the kettle and toilet are, and writing down what you are told. Asking questions is not a weakness in week one - it is the whole job.",
        ),
    ),
    Case(
        "first-day",
        "what questions do i ask on my first day",
        None,
        Some(
            "Ask four things: who does what on the team, what a good first month looks like, who checks your work before it goes out, and what you should read or learn first. Write the answers down as you hear them, because four answers said out loud on day one are gone by Friday.",
        ),
    ),
    Case(
        "first-day",
        "i got the job, what should i bring on the first day",
        None,
        Some(
            "Congratulations - the first week is mostly names and questions, so go easy on yourself. Take a notebook, a pen, your ID and the paperwork they asked for, and dress one notch smarter than you think you need to. Your only jobs on day one are names, where the kettle and toilet are, and writing down what you are told. Asking questions is not a weakness in week one - it is the whole job.",
        ),
    ),
    Case(
        "first-day",
        "i got the job, what questions do i ask?",
        None,
        Some(
            "Congratulations. Ask four things: who does what on the team, what a good first month looks like, who checks your work before it goes out, and what you should read or learn first. Write the answers down as you hear them, because four answers said out loud on day one are gone by Friday.",
        ),
    ),
    Case(
        "first-day",
        "hes nervous about starting his new job",
        None,
        None,
    ),
    Case(
        "first-day",
        "my sister is nervous about her first day, what should we pack",
        None,
        Some(
            "Take a notebook, a pen, your ID and the paperwork they asked for, and dress one notch smarter than you think you need to. Your only jobs on day one are names, where the kettle and toilet are, and writing down what you are told. Asking questions is not a weakness in week one - it is the whole job.",
        ),
    ),
    Case(
        "first-day",
        "i feel lost in my new job",
        None,
        Some(
            "Write down every question you have for a week, then take the list to your manager at the end of it: \"These are the things I keep guessing at - can we go through them?\" Ask two people if you can sit with them for an hour and watch how the work is really done, and say yes to the tea round, because the small talk is where the useful information is. Being new is a licence to ask that you only get once, so use it in the first month rather than saving it.",
        ),
    ),
    Case(
        "first-day",
        "started last week and nobody talks to me",
        None,
        Some(
            "Write down every question you have for a week, then take the list to your manager at the end of it: \"These are the things I keep guessing at - can we go through them?\" Ask two people if you can sit with them for an hour and watch how the work is really done, and say yes to the tea round, because the small talk is where the useful information is. Being new is a licence to ask that you only get once, so use it in the first month rather than saving it.",
        ),
    ),
    Case(
        "first-day",
        "am i expected to talk about myself on my first day?",
        None,
        Some(
            "Take a notebook, a pen, your ID and the paperwork they asked for, and dress one notch smarter than you think you need to. Your only jobs on day one are names, where the kettle and toilet are, and writing down what you are told. Asking questions is not a weakness in week one - it is the whole job.",
        ),
    ),
    Case(
        "first-day",
        "I am excited about the first day",
        None,
        Some(
            "Take a notebook, a pen, your ID and the paperwork they asked for, and dress one notch smarter than you think you need to. Your only jobs on day one are names, where the kettle and toilet are, and writing down what you are told. Asking questions is not a weakness in week one - it is the whole job.",
        ),
    ),
];
pub(super) type Conversation = (
    &'static str,
    &'static str,
    &'static [&'static str],
    Option<bool>,
    Option<&'static str>,
);
pub(super) const EXTRA: &[Conversation] = &[
    (
        "defensive-talk",
        "how do i explain without sounding defensive",
        &[],
        None,
        None,
    ),
    (
        "defensive-talk",
        "how should i explain that i missed the deadline without sounding defensive",
        &[],
        None,
        Some(
            "Lead with the fact, then what you did about it, then what you learned: \"The project slipped by six weeks; I flagged it in May, we re-planned, and I now build in a review at the halfway point.\" Talking about the fix is what stops it sounding defensive, so don't explain the cause twice.",
        ),
    ),
    (
        "defensive-talk",
        "what can i say without sounding defensive",
        &["I missed my deadline."],
        None,
        Some(
            "Lead with the fact, then what you did about it, then what you learned: \"The project slipped by six weeks; I flagged it in May, we re-planned, and I now build in a review at the halfway point.\" Talking about the fix is what stops it sounding defensive, so don't explain the cause twice.",
        ),
    ),
    (
        "defensive-talk",
        "what can i say without sounding defensive",
        &[
            "I missed my deadline.",
            "I told my boss.",
            "I feel awkward.",
        ],
        None,
        Some(
            "Lead with the fact, then what you did about it, then what you learned: \"The project slipped by six weeks; I flagged it in May, we re-planned, and I now build in a review at the halfway point.\" Talking about the fix is what stops it sounding defensive, so don't explain the cause twice.",
        ),
    ),
    (
        "defensive-talk",
        "what can i say without sounding defensive",
        &[
            "I missed my deadline.",
            "I told my boss.",
            "I feel awkward.",
            "I went home.",
        ],
        None,
        None,
    ),
    (
        "defensive-talk",
        "can i say something",
        &[
            "I submitted the report",
            "late to the meeting",
            "I feel weird.",
        ],
        None,
        Some(
            "Lead with the fact, then what you did about it, then what you learned: \"The project slipped by six weeks; I flagged it in May, we re-planned, and I now build in a review at the halfway point.\" Talking about the fix is what stops it sounding defensive, so don't explain the cause twice.",
        ),
    ),
    (
        "defensive-talk",
        "can i say something",
        &["my boss said no", "I feel awful", "I will try again"],
        None,
        None,
    ),
    (
        "colleague-smell",
        "how do i tell my colleague they smell",
        &[],
        None,
        Some(
            "Say it once, in private, about the effect and not the person: \"This is awkward, and I'd want to know - you had a smell about you in the meeting and I don't think you meant to.\" One sentence, then move on to something practical: a spare shirt in the locker, deodorant in the bathroom, washing at the gym before the shift. Never in front of anyone else and never by email. If it is your manager, or you cannot face it, ask HR to have the word - that is what they are for, and you do not have to name the reason in writing.",
        ),
    ),
    (
        "colleague-smell",
        "how do i tell my boss he smells of smoke",
        &[],
        None,
        Some(
            "Say it once, in private, about the effect and not the person: \"This is awkward, and I'd want to know - you smelled of smoke in the meeting and I don't think you meant to.\" One sentence, then move on to something practical: a spare shirt in the locker, deodorant in the bathroom, washing at the gym before the shift. Never in front of anyone else and never by email. If it is your manager, or you cannot face it, ask HR to have the word - that is what they are for, and you do not have to name the reason in writing.",
        ),
    ),
    (
        "colleague-smell",
        "my coworker has bad breath how to mention it",
        &[],
        None,
        Some(
            "Say it once, in private, about the effect and not the person: \"This is awkward, and I'd want to know - your breath across the desk was hard to miss and I don't think you meant to.\" One sentence, then move on to something practical: a spare shirt in the locker, deodorant in the bathroom, washing at the gym before the shift. Never in front of anyone else and never by email. If it is your manager, or you cannot face it, ask HR to have the word - that is what they are for, and you do not have to name the reason in writing.",
        ),
    ),
    (
        "colleague-smell",
        "my colleague's feet smell",
        &[],
        None,
        Some(
            "Say it once, in private, about the effect and not the person: \"This is awkward, and I'd want to know - your shoes under the desk were hard to miss and I don't think you meant to.\" One sentence, then move on to something practical: a spare shirt in the locker, deodorant in the bathroom, washing at the gym before the shift. Never in front of anyone else and never by email. If it is your manager, or you cannot face it, ask HR to have the word - that is what they are for, and you do not have to name the reason in writing.",
        ),
    ),
    (
        "colleague-smell",
        "my wife smells of smoke",
        &[],
        None,
        Some(
            "Say it the same day, at home, about the thing rather than the person: \"your jacket still smells of smoke - have you noticed?\" is easier to hear than anything about you. One sentence, then something that helps - a wash at sixty, a new deodorant, the window open - and no audience and no joke about it.",
        ),
    ),
    (
        "colleague-smell",
        "my housemate has bad breath",
        &[],
        None,
        Some(
            "Say it the same day, at home, about the thing rather than the person: \"your breath in the car was hard to miss - have you noticed?\" is easier to hear than anything about you. One sentence, then something that helps - a wash at sixty, a new deodorant, the window open - and no audience and no joke about it.",
        ),
    ),
    (
        "colleague-smell",
        "my partner's socks stink",
        &[],
        None,
        Some(
            "Say it the same day, at home, about the thing rather than the person: \"your trainers by the door were hard to miss - have you noticed?\" is easier to hear than anything about you. One sentence, then something that helps - a wash at sixty, a new deodorant, the window open - and no audience and no joke about it.",
        ),
    ),
    (
        "colleague-smell",
        "my brother smells",
        &[],
        None,
        Some(
            "Say it the same day, at home, about the thing rather than the person: \"your shirts still smell after the wash - have you noticed?\" is easier to hear than anything about you. One sentence, then something that helps - a wash at sixty, a new deodorant, the window open - and no audience and no joke about it.",
        ),
    ),
    (
        "colleague-smell",
        "tell my worker who smells of cigarette and garlic",
        &[],
        None,
        Some(
            "Say it once, in private, about the effect and not the person: \"This is awkward, and I'd want to know - you smelled of smoke in the meeting and I don't think you meant to.\" One sentence, then move on to something practical: a spare shirt in the locker, deodorant in the bathroom, washing at the gym before the shift. Never in front of anyone else and never by email. If it is your manager, or you cannot face it, ask HR to have the word - that is what they are for, and you do not have to name the reason in writing.",
        ),
    ),
    (
        "noisy-office",
        "how do I get a quiet hour with a newborn",
        &[],
        Some(true),
        Some(
            "Take the hour when someone else has them instead of trying to find one: one stretch a day handed over to a partner, a grandparent or a friend is worth more than any trick. If you are on your own, go out with the pram or a sling at the time they sleep, because a walk is the one hour that comes free, and put the cot or the basket in the room you are in rather than chasing sleep in another room. Rest when they rest and leave the tidying: the hour is for you, and a baby group or a home-start volunteer will give you another one.",
        ),
    ),
    (
        "noisy-office",
        "how do i get a quiet hour at work",
        &[],
        Some(true),
        Some(
            "Ask for a specific seat, not for silence: the back row away from the door, the printer and the kitchen, and say what it buys - \"I lose an hour a day to the corridor outside my desk, so I'd like to move to the far wall.\" Book a meeting room for the two hours of deep work a day and treat it as a real appointment, and use headphones as the sign that you are in one. If the noise is machinery, an alarm or something that makes your ears ring, that is a health-and-safety report, not a seating request.",
        ),
    ),
    (
        "noisy-office",
        "a colleague plays radio all day",
        &[],
        Some(true),
        Some(
            "Have one friendly word before you change anything else, because most people have no idea how far their voice carries: \"I can't hear myself think with that on - any chance of headphones?\" Ask for the one change you want rather than for quiet in general, and ask it at their desk rather than in front of the team. Then do your half: your own headphones, or a seat away from them if the desk layout allows, and the deep-work hour booked in the calendar. If it carries on, that is a thing to raise with your manager once, with the times written down - it is their job to sort out where people sit, not yours.",
        ),
    ),
    (
        "noisy-office",
        "someone beside me is on the phone",
        &[],
        Some(true),
        Some(
            "Have one friendly word before you change anything else, because most people have no idea how far their voice carries: \"I can't hear myself think on my own calls - would you mind taking that off speaker?\" Ask for the one change you want rather than for quiet in general, and ask it at their desk rather than in front of the team. Then do your half: your own headphones, or a seat away from them if the desk layout allows, and the deep-work hour booked in the calendar. If it carries on, that is a thing to raise with your manager once, with the times written down - it is their job to sort out where people sit, not yours.",
        ),
    ),
    (
        "noisy-office",
        "my colleague is loud",
        &[],
        Some(true),
        Some(
            "Have one friendly word before you change anything else, because most people have no idea how far their voice carries: \"Sorry to be that person - I can't hear myself think where I'm sitting. Any chance of keeping that down?\" Ask for the one change you want rather than for quiet in general, and ask it at their desk rather than in front of the team. Then do your half: your own headphones, or a seat away from them if the desk layout allows, and the deep-work hour booked in the calendar. If it carries on, that is a thing to raise with your manager once, with the times written down - it is their job to sort out where people sit, not yours.",
        ),
    ),
    (
        "noisy-office",
        "my colleague is quiet today",
        &[],
        Some(false),
        Some(
            "Have one friendly word before you change anything else, because most people have no idea how far their voice carries: \"Sorry to be that person - I can't hear myself think where I'm sitting. Any chance of keeping that down?\" Ask for the one change you want rather than for quiet in general, and ask it at their desk rather than in front of the team. Then do your half: your own headphones, or a seat away from them if the desk layout allows, and the deep-work hour booked in the calendar. If it carries on, that is a thing to raise with your manager once, with the times written down - it is their job to sort out where people sit, not yours.",
        ),
    ),
    (
        "noisy-office",
        "how do i speak to my colleague about bad hygiene",
        &[],
        Some(false),
        Some(
            "Have one friendly word before you change anything else, because most people have no idea how far their voice carries: \"Sorry to be that person - I can't hear myself think where I'm sitting. Any chance of keeping that down?\" Ask for the one change you want rather than for quiet in general, and ask it at their desk rather than in front of the team. Then do your half: your own headphones, or a seat away from them if the desk layout allows, and the deep-work hour booked in the calendar. If it carries on, that is a thing to raise with your manager once, with the times written down - it is their job to sort out where people sit, not yours.",
        ),
    ),
    (
        "noisy-office",
        "how do i work in an open plan office",
        &[],
        Some(true),
        Some(
            "Ask for a specific seat, not for silence: the back row away from the door, the printer and the kitchen, and say what it buys - \"I lose an hour a day to the corridor outside my desk, so I'd like to move to the far wall.\" Book a meeting room for the two hours of deep work a day and treat it as a real appointment, and use headphones as the sign that you are in one. If the noise is machinery, an alarm or something that makes your ears ring, that is a health-and-safety report, not a seating request.",
        ),
    ),
    (
        "noisy-office",
        "my colleagues talk on the phone with the radio on",
        &[],
        Some(true),
        Some(
            "Have one friendly word before you change anything else, because most people have no idea how far their voice carries: \"I can't hear myself think with that on - any chance of headphones?\" Ask for the one change you want rather than for quiet in general, and ask it at their desk rather than in front of the team. Then do your half: your own headphones, or a seat away from them if the desk layout allows, and the deep-work hour booked in the calendar. If it carries on, that is a thing to raise with your manager once, with the times written down - it is their job to sort out where people sit, not yours.",
        ),
    ),
];
