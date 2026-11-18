//! Guard and callback outcomes from the pinned planning cards.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub bool,
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "move-house-plan",
        "when we move what do we do first",
        true,
        None,
    ),
    Case(
        "move-house-plan",
        "what should i do first when moving",
        true,
        None,
    ),
    Case(
        "move-house-plan",
        "how do we plan a move abroad",
        false,
        None,
    ),
    Case(
        "move-house-plan",
        "how can i plan my move with the team",
        false,
        None,
    ),
    Case(
        "move-house-plan",
        "can i plan our move to spain",
        false,
        None,
    ),
    Case("move-house-plan", "give me tips before my move", true, None),
    Case(
        "move-house-plan",
        "before we move flat, what should we do",
        true,
        None,
    ),
    Case("move-house-plan", "when we move our office", false, None),
    Case("move-house-plan", "help me move a chess piece", false, None),
    Case(
        "move-house-plan",
        "how do i prepare before moving in together",
        false,
        None,
    ),
    Case(
        "move-house-plan",
        "what should i do before we move house to Canada",
        true,
        None,
    ),
    Case("move-house-plan", "house move schedule", true, None),
    Case("move-house-plan", "our move prep", true, None),
    Case("move-house-plan", "move in checklist", true, None),
    Case("move-house-plan", "what is the next move", false, None),
    Case("declutter-plan", "how do i get rid of", true, None),
    Case("declutter-plan", "how do i get rid of?", true, None),
    Case(
        "declutter-plan",
        "how do i get rid of a verruca",
        false,
        None,
    ),
    Case(
        "declutter-plan",
        "how do i get rid of limescale",
        false,
        None,
    ),
    Case("declutter-plan", "fed up with clutter", true, None),
    Case(
        "declutter-plan",
        "where should i start with the loft",
        true,
        None,
    ),
    Case(
        "declutter-plan",
        "how do i get rid of papers from the desk",
        true,
        None,
    ),
    Case("declutter-plan", "too much rubbish", true, None),
    Case("declutter-plan", "too much work", false, None),
    Case("declutter-plan", "how should i sort the house", true, None),
    Case("declutter-plan", "i need to declutter", true, None),
    Case(
        "declutter-plan",
        "i have too much in the filing cabinet",
        true,
        None,
    ),
    Case(
        "declutter-plan",
        "the kitchenware is in my way",
        false,
        None,
    ),
    Case(
        "declutter-plan",
        "how do i sort\t  the house out",
        true,
        None,
    ),
    Case("saving-plan", "how do i start saving", true, None),
    Case(
        "saving-plan",
        "what shall we do for a rainy day",
        false,
        None,
    ),
    Case(
        "saving-plan",
        "how do i put some money aside for a rainy day",
        true,
        None,
    ),
    Case("saving-plan", "can i save it for a rainy day", true, None),
    Case("saving-plan", "ideas for a rainy day indoors", false, None),
    Case("saving-plan", "how to invest for a rainy day", true, None),
    Case("saving-plan", "any ideas for a rainy weekend", true, None),
    Case(
        "saving-plan",
        "how much emergency fund do i need",
        true,
        None,
    ),
    Case("saving-plan", "i want to save some money", true, None),
    Case("saving-plan", "the rainy day plan", true, None),
    Case("party-kids", "how many kids do i have", false, None),
    Case("party-kids", "how many children do we have", false, None),
    Case("party-kids", "how many kids have i got", false, None),
    Case(
        "party-kids",
        "how many little kids did we have",
        false,
        None,
    ),
    Case("party-kids", "how many kids to invite", true, None),
    Case(
        "party-kids",
        "how many children should i invite",
        true,
        None,
    ),
    Case("party-kids", "how many brownies do we have", false, None),
    Case(
        "party-kids",
        "how many other little preschool children do we have",
        true,
        None,
    ),
    Case("party-kids", "party for a five year old", true, None),
    Case("party-kids", "how many friends does she have", true, None),
    Case("party-kids", "HOW MANY KIDS HAVE WE GOT", false, None),
    Case(
        "party-milestone",
        "what should i do my mum is 70 next week",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "my son is 7 next week what should we do",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-milestone",
        "my mum is 17 next week what can we do",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "what do we do when my car is 17",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "my boiler is 70 next week what can we do",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "what should we do my dad will be 61 on friday",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "what should i get my mum who is 70 next week",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "present ideas for my dad who turns 60 on sunday",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "how should we celebrate when she turns 50 in two weeks",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "my dad turns 60 what can i do",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "my niece is turning 8 this weekend, any ideas",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-milestone",
        "when my granny becomes 99 soon what do i do",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "my daughter is 4 next week and her brother is 14 next week, how can we celebrate",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-milestone",
        "she will be ٢٠ soon, what can i do",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-milestone",
        "next saturday my kid is 12 years old what can we do",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-milestone",
        "my husband becomes 100 soon",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "how do i plan my mum's birthday party",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "what should we do for my son",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-grownup",
        "party for my sons 40th",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "when is her birthday",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "when should i send invitations to my mother's birthday",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "what do i get my mum for her birthday",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "present ideas for my dad",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "what should i do for the 13th",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "the party for 12 and 40 people",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-grownup",
        "party for 0 kids",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-grownup",
        "party for 00 guests",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "i forgot my mums birthday what do i do",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "i didnt get my mum anything for her birthday",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "how do i make it up to my mum",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "how do i make my mums birthday up to my mum",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "how do i make it up to my doctor",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "how do i make amends to my dad",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "how can i put things right with my sister",
        false,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "our cake for a birthday party",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "i lay out the party table for 100 guests",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "a party in year one",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-grownup",
        "a reception class party",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
    Case(
        "party-grownup",
        "can we celebrate the ٢٠th birthday",
        true,
        Some(
            "Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top.",
        ),
    ),
    Case(
        "party-grownup",
        "WHAT DO I DO FOR MY SONS 12TH",
        true,
        Some(
            "Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both.",
        ),
    ),
];
