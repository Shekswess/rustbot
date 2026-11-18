//! Pinned source admission and callbacks for planning unallocated time.

pub(super) const CASES: &[(&str, bool, Option<&str>)] = &[
    (
        "What should I do this weekend?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    (
        "What can I do on Saturday?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    (
        "What should I do this afternoon?",
        true,
        Some(
            "Put two things in the afternoon and give each a time: one that gets you out (a walk you have not done, a park, the library) and one you keep putting off that takes an hour. A free afternoon with no time on it is what dissolves into the phone.",
        ),
    ),
    (
        "What should I do with a free afternoon?",
        true,
        Some(
            "Put two things in the afternoon and give each a time: one that gets you out (a walk you have not done, a park, the library) and one you keep putting off that takes an hour. A free afternoon with no time on it is what dissolves into the phone.",
        ),
    ),
    (
        "Any ideas for an afternoon off?",
        true,
        Some(
            "Put two things in the afternoon and give each a time: one that gets you out (a walk you have not done, a park, the library) and one you keep putting off that takes an hour. A free afternoon with no time on it is what dissolves into the phone.",
        ),
    ),
    (
        "What should I do with a day off?",
        true,
        Some(
            "Treat it as a small Saturday: one thing out of the house (a walk you have not done, a market, a museum or gallery - most are free), one job you keep putting off that takes an hour, and one person to see. Put times on the first two the night before; a free day goes in small pieces when nothing is decided.",
        ),
    ),
    (
        "I have a free day tomorrow any ideas?",
        true,
        Some(
            "Treat it as a small Saturday: one thing out of the house (a walk you have not done, a market, a museum or gallery - most are free), one job you keep putting off that takes an hour, and one person to see. Put times on the first two the night before; a free day goes in small pieces when nothing is decided.",
        ),
    ),
    (
        "Any ideas for a free evening?",
        true,
        Some(
            "Give the evening a shape rather than waiting for an idea: one thing you will actually do at 8pm (a film you have been meaning to watch, a walk before it gets dark, a call to someone you like), tea that takes a bit of effort, and the phone in another room for the first hour. A free evening feels long when nothing is decided and short when one thing is.",
        ),
    ),
    (
        "What should I do after work?",
        true,
        Some(
            "Give the evening a shape rather than waiting for an idea: one thing you will actually do at 8pm (a film you have been meaning to watch, a walk before it gets dark, a call to someone you like), tea that takes a bit of effort, and the phone in another room for the first hour. A free evening feels long when nothing is decided and short when one thing is.",
        ),
    ),
    (
        "What can I do when I get home?",
        true,
        Some(
            "Give the evening a shape rather than waiting for an idea: one thing you will actually do at 8pm (a film you have been meaning to watch, a walk before it gets dark, a call to someone you like), tea that takes a bit of effort, and the phone in another room for the first hour. A free evening feels long when nothing is decided and short when one thing is.",
        ),
    ),
    (
        "What should I do this evening?",
        true,
        Some(
            "Give the evening a shape rather than waiting for an idea: one thing you will actually do at 8pm (a film you have been meaning to watch, a walk before it gets dark, a call to someone you like), tea that takes a bit of effort, and the phone in another room for the first hour. A free evening feels long when nothing is decided and short when one thing is.",
        ),
    ),
    ("What should I do before work?", false, None),
    (
        "Any tips for a few days off?",
        true,
        Some(
            "Treat it as a small Saturday: one thing out of the house (a walk you have not done, a market, a museum or gallery - most are free), one job you keep putting off that takes an hour, and one person to see. Put times on the first two the night before; a free day goes in small pieces when nothing is decided.",
        ),
    ),
    (
        "I have 2 days off next week what can I do?",
        true,
        Some(
            "Treat it as a small Saturday: one thing out of the house (a walk you have not done, a market, a museum or gallery - most are free), one job you keep putting off that takes an hour, and one person to see. Put times on the first two the night before; a free day goes in small pieces when nothing is decided.",
        ),
    ),
    (
        "What can I do with two days off?",
        true,
        Some(
            "Treat it as a small Saturday: one thing out of the house (a walk you have not done, a market, a museum or gallery - most are free), one job you keep putting off that takes an hour, and one person to see. Put times on the first two the night before; a free day goes in small pieces when nothing is decided.",
        ),
    ),
    (
        "What can I do with a couple of days to myself?",
        true,
        Some(
            "Put two things in the afternoon and give each a time: one that gets you out (a walk you have not done, a park, the library) and one you keep putting off that takes an hour. A free afternoon with no time on it is what dissolves into the phone.",
        ),
    ),
    (
        "I have three days free with no plans",
        true,
        Some(
            "Put two things in the afternoon and give each a time: one that gets you out (a walk you have not done, a park, the library) and one you keep putting off that takes an hour. A free afternoon with no time on it is what dissolves into the phone.",
        ),
    ),
    (
        "What should I do with a week off work?",
        true,
        Some(
            "Put a couple of nights away in the middle of the week and keep the first and last days free: somewhere two or three hours away by train is enough, and going Tuesday to Thursday costs less than the weekend. Fill the other days a day at a time - one thing out of the house, one job you keep putting off - and a week off stops slipping past unnoticed.",
        ),
    ),
    (
        "How do I plan my week off?",
        true,
        Some(
            "Put a couple of nights away in the middle of the week and keep the first and last days free: somewhere two or three hours away by train is enough, and going Tuesday to Thursday costs less than the weekend. Fill the other days a day at a time - one thing out of the house, one job you keep putting off - and a week off stops slipping past unnoticed.",
        ),
    ),
    (
        "How do I use my week off in the summer?",
        true,
        Some(
            "Put a couple of nights away in the middle of the week and keep the first and last days free: somewhere two or three hours away by train is enough, and going Tuesday to Thursday costs less than the weekend. Fill the other days a day at a time - one thing out of the house, one job you keep putting off - and a week off stops slipping past unnoticed. In summer, put the outdoors before ten or after five and keep the middle of the day for the job you keep putting off, so the heat sets the day instead of you.",
        ),
    ),
    (
        "How should I spend my week off in winter?",
        true,
        Some(
            "Put a couple of nights away in the middle of the week and keep the first and last days free: somewhere two or three hours away by train is enough, and going Tuesday to Thursday costs less than the weekend. Fill the other days a day at a time - one thing out of the house, one job you keep putting off - and a week off stops slipping past unnoticed. In winter, plan round the dark: the walk or the market at midday, and the cinema, the museum and the long cook for the evenings.",
        ),
    ),
    (
        "What can I do in a week off with no money?",
        true,
        Some(
            "Start with the days that cost nothing: the walk you have not done, the library, a market, a park with a flask and a sandwich. Put the one paid thing in the middle of the week, because a cheap train day trip on a Tuesday off-peak is the best value going, and give each day one job out of the house so the week does not go by in small pieces. None of that has to cost anything: the walk and the library are free either way, and a flask beats a cafe.",
        ),
    ),
    (
        "What should I do on my week off on a budget?",
        true,
        Some(
            "Start with the days that cost nothing: the walk you have not done, the library, a market, a park with a flask and a sandwich. Put the one paid thing in the middle of the week, because a cheap train day trip on a Tuesday off-peak is the best value going, and give each day one job out of the house so the week does not go by in small pieces. None of that has to cost anything: the walk and the library are free either way, and a flask beats a cafe.",
        ),
    ),
    (
        "I have a week off and im skint any ideas?",
        true,
        Some(
            "Start with the days that cost nothing: the walk you have not done, the library, a market, a park with a flask and a sandwich. Put the one paid thing in the middle of the week, because a cheap train day trip on a Tuesday off-peak is the best value going, and give each day one job out of the house so the week does not go by in small pieces. None of that has to cost anything: the walk and the library are free either way, and a flask beats a cafe.",
        ),
    ),
    (
        "What to do this weekend on a tight budget?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened. None of that has to cost anything: the walk and the library are free either way, and a flask beats a cafe.",
        ),
    ),
    (
        "Something to do on Saturday without spending much?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    (
        "Suggest something to do in the garden this weekend",
        false,
        None,
    ),
    (
        "Suggest something for the garden at the weekend",
        false,
        None,
    ),
    (
        "What should I do with the weekend to myself?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    (
        "I have nothing planned this weekend and it is cold",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    ("I have a cold this weekend what should I do?", false, None),
    (
        "I have toothache at the weekend what can I do?",
        false,
        None,
    ),
    ("What should i do tonight im exhausted?", true, None),
    (
        "I have 200 quid and a few days off where can I go?",
        true,
        None,
    ),
    ("Where should we go on a week off in October?", true, None),
    ("I want to plan a trip with a week off", false, None),
    ("What should I do in Lisbon for a weekend?", false, None),
    ("What can I do this weekend in Paris?", false, None),
    ("What should i do in Bath this weekend?", false, None),
    ("I have a week off in May any ideas?", true, None),
    ("Any ideas for a film tonight?", false, None),
    ("Any Netflix suggestions for tonight?", false, None),
    (
        "What can I do with a bag of apples this weekend?",
        false,
        None,
    ),
    ("I am thinking of doing a roast on Sunday", true, None),
    ("Fancy a spag bol tonight", false, None),
    ("Im gonna make a spag bol tnite any tips?", false, None),
    ("Any tips about curry for Sunday?", false, None),
    ("I feel like doing a jigsaw tonight", false, None),
    ("What can I do with my evenings if I hate TV?", true, None),
    (
        "What can I do this weekend with friends?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    ("Any ideas for a rainy day?", true, None),
    (
        "Any ideas for an evening in without TV?",
        true,
        Some(
            "Give the evening a shape rather than waiting for an idea: one thing you will actually do at 8pm (a film you have been meaning to watch, a walk before it gets dark, a call to someone you like), tea that takes a bit of effort, and the phone in another room for the first hour. A free evening feels long when nothing is decided and short when one thing is.",
        ),
    ),
    ("Give me three ideas for a cheap date", false, None),
    ("Any tips for a bank holiday?", true, None),
    (
        "I think I should hire a skip on Saturday",
        false,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    ("Maybe I should go hiking this weekend", true, None),
    ("I want to book a day off next week", true, None),
    (
        "What should i write in my request for a day off?",
        true,
        None,
    ),
    ("Its parents evening tonight", false, None),
    (
        "I have a week off with a newborn any ideas for the baby?",
        false,
        Some(
            "With a newborn the week plans itself, so plan the small things instead: take the nights in shifts so one of you gets a four-hour stretch, get out once a day even if it is only round the block, and say yes to anyone who offers to hold the baby while you sleep. Register the birth inside 42 days, keep the health visitor's number to hand, and book nothing that has a start time - a week off now is recovery, not a holiday.",
        ),
    ),
    (
        "I have a week off and my baby is due any ideas?",
        true,
        Some(
            "With a newborn the week plans itself, so plan the small things instead: take the nights in shifts so one of you gets a four-hour stretch, get out once a day even if it is only round the block, and say yes to anyone who offers to hold the baby while you sleep. Register the birth inside 42 days, keep the health visitor's number to hand, and book nothing that has a start time - a week off now is recovery, not a holiday.",
        ),
    ),
    (
        "What can I do on my week of paternity leave?",
        true,
        Some(
            "With a newborn the week plans itself, so plan the small things instead: take the nights in shifts so one of you gets a four-hour stretch, get out once a day even if it is only round the block, and say yes to anyone who offers to hold the baby while you sleep. Register the birth inside 42 days, keep the health visitor's number to hand, and book nothing that has a start time - a week off now is recovery, not a holiday.",
        ),
    ),
    ("What to do on a week off in the sunshine?", false, None),
    (
        "How do i use a free week in the autumn?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    (
        "I have no energy this afternoon what can I do?",
        false,
        None,
    ),
    (
        "I have the day off tomorrow what about work?",
        true,
        Some(
            "Treat it as a small Saturday: one thing out of the house (a walk you have not done, a market, a museum or gallery - most are free), one job you keep putting off that takes an hour, and one person to see. Put times on the first two the night before; a free day goes in small pieces when nothing is decided.",
        ),
    ),
    (
        "What can I do this weekend without leaving home?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    (
        "I have the weekend free and want ideas for a present",
        true,
        None,
    ),
    (
        "Can I take a week off and what can I do with it?",
        true,
        None,
    ),
    ("What do I do on Saturday if the shop is shut?", false, None),
    ("What should I do with two days away?", true, None),
    (
        "Saturday?",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    (
        "I have a weekend",
        true,
        Some(
            "Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened.",
        ),
    ),
    ("I have 3 days free to revise any ideas?", true, None),
    (
        "What to do on a few days off in the house?",
        true,
        Some(
            "Treat it as a small Saturday: one thing out of the house (a walk you have not done, a market, a museum or gallery - most are free), one job you keep putting off that takes an hour, and one person to see. Put times on the first two the night before; a free day goes in small pieces when nothing is decided.",
        ),
    ),
    ("Any ideas for 2 days off at home and no spend?", true, None),
];
