//! Pinned glossary, scaling, and serving callbacks.

pub(super) type Case = (
    &'static str,
    &'static str,
    &'static [(&'static str, &'static str)],
    Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    (
        "cook-words",
        "what does reduce mean",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work.",
        ),
    ),
    (
        "cook-words",
        "what does sear mean",
        &[],
        Some(
            "Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself.",
        ),
    ),
    (
        "cook-words",
        "what does blanch mean",
        &[],
        Some(
            "Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins.",
        ),
    ),
    (
        "cook-words",
        "what does sweat mean",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes.",
        ),
    ),
    (
        "cook-words",
        "what does al dente mean",
        &[],
        Some(
            "Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing.",
        ),
    ),
    (
        "cook-words",
        "what does fold mean",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    (
        "cook-words",
        "what does deglaze mean",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between reduce and sear",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work. Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between reduce and blanch",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work. Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between reduce and sweat",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work. Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between reduce and al dente",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work. Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between reduce and fold",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work. Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between reduce and deglaze",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work. Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sear and reduce",
        &[],
        Some(
            "Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself. Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sear and blanch",
        &[],
        Some(
            "Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself. Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sear and sweat",
        &[],
        Some(
            "Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself. Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sear and al dente",
        &[],
        Some(
            "Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself. Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sear and fold",
        &[],
        Some(
            "Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself. Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sear and deglaze",
        &[],
        Some(
            "Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself. Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between blanch and reduce",
        &[],
        Some(
            "Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins. Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between blanch and sear",
        &[],
        Some(
            "Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins. Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between blanch and sweat",
        &[],
        Some(
            "Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins. Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between blanch and al dente",
        &[],
        Some(
            "Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins. Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between blanch and fold",
        &[],
        Some(
            "Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins. Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between blanch and deglaze",
        &[],
        Some(
            "Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins. Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sweat and reduce",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes. Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sweat and sear",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes. Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sweat and blanch",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes. Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sweat and al dente",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes. Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sweat and fold",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes. Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between sweat and deglaze",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes. Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between al dente and reduce",
        &[],
        Some(
            "Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing. Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between al dente and sear",
        &[],
        Some(
            "Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing. Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between al dente and blanch",
        &[],
        Some(
            "Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing. Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between al dente and sweat",
        &[],
        Some(
            "Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing. Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between al dente and fold",
        &[],
        Some(
            "Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing. Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between al dente and deglaze",
        &[],
        Some(
            "Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing. Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between fold and reduce",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream. Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between fold and sear",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream. Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between fold and blanch",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream. Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between fold and sweat",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream. Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between fold and al dente",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream. Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between fold and deglaze",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream. Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between deglaze and reduce",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is. Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between deglaze and sear",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is. Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between deglaze and blanch",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is. Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between deglaze and sweat",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is. Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between deglaze and al dente",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is. Al dente means pasta with a little bite left in the middle: take a minute off the packet time, taste it, and keep testing.",
        ),
    ),
    (
        "cook-words",
        "what's the difference between deglaze and fold",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is. Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    (
        "cook-words",
        "what is deglazing",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what does blanching mean",
        &[],
        Some(
            "Blanching is a minute or two in boiling water, then straight into cold water: it sets the colour of green vegetables and loosens tomato and peach skins.",
        ),
    ),
    (
        "cook-words",
        "what does folding mean",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    (
        "cook-words",
        "how do i fold in the flour",
        &[],
        Some(
            "Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    ("cook-words", "how do i fold my sheet", &[], None),
    (
        "cook-words",
        "how do i reduce the sauce",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work.",
        ),
    ),
    ("cook-words", "how do i reduce the brightness", &[], None),
    (
        "cook-words",
        "how do i sear it",
        &[],
        Some(
            "Searing is browning meat hard in a hot pan, for flavour and colour, not for cooking it through. Pat it dry first and leave it alone until it releases by itself.",
        ),
    ),
    (
        "cook-words",
        "how do i sweat the onions",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes.",
        ),
    ),
    ("cook-words", "how do i sweat", &[], None),
    (
        "cook-words",
        "reduce it and fold the whites",
        &[],
        Some(
            "Reducing is simmering a sauce uncovered until water boils off and it thickens and tastes stronger. Watch it at the end - the last two minutes do most of the work. Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    ("cook-words", "what does resolve mean", &[], None),
    (
        "cook-words",
        "this deglazed dish was good",
        &[],
        Some(
            "Deglazing is pouring wine, stock or water into a hot pan after frying and scraping the brown bits up: that is where the flavour is.",
        ),
    ),
    (
        "cook-words",
        "what does sweat fold sear mean",
        &[],
        Some(
            "Sweating is softening onion or vegetables slowly in a little fat with the lid on, without colouring them. Low heat, five to ten minutes. Folding is mixing gently with a big spoon, turning the bowl and cutting through, so you keep the air in whipped egg white or cream.",
        ),
    ),
    ("cook-words", "é sear", &[], None),
    (
        "pudding-ideas",
        "pudding ideas",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues.",
        ),
    ),
    (
        "pudding-ideas",
        "what pudding for a crowd of 8",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues. For 8 people make two dishes of it and put the custard out in a jug.",
        ),
    ),
    (
        "pudding-ideas",
        "dessert for 7 people",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues.",
        ),
    ),
    (
        "pudding-ideas",
        "dessert for 9 people",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues. For 9 people make two dishes of it and put the custard out in a jug.",
        ),
    ),
    (
        "pudding-ideas",
        "one dessert for 10",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues. For 10 people make two dishes of it and put the custard out in a jug.",
        ),
    ),
    (
        "pudding-ideas",
        "dessert for 12 or 15",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues. For 12 people make two dishes of it and put the custard out in a jug.",
        ),
    ),
    (
        "pudding-ideas",
        "dessert for 99",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues. For 99 people make two dishes of it and put the custard out in a jug.",
        ),
    ),
    (
        "pudding-ideas",
        "dessert for 100",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues.",
        ),
    ),
    (
        "pudding-ideas",
        "pudding after 12:30",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues. For 12 people make two dishes of it and put the custard out in a jug.",
        ),
    ),
    (
        "pudding-ideas",
        "dessert for 1,012 people",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues.",
        ),
    ),
    (
        "pudding-ideas",
        "dessert for １２ people",
        &[],
        Some(
            "An apple and blackberry crumble is the easiest crowd-pleaser: fruit in a dish, a crumble of butter, flour and sugar on top, 35 minutes at 180C, and custard. Baked rice pudding and bread and butter pudding are just as easy and soft enough for anyone who finds chewing hard; Eton mess takes five minutes if you buy the meringues. For 12 people make two dishes of it and put the custard out in a jug.",
        ),
    ),
    (
        "halve-a-recipe",
        "can i halve a recipe",
        &[],
        Some(
            "Halve every ingredient and leave the oven temperature alone: the tin goes one size down if the batter looks thin in it, and start checking five minutes early. Cooking time does not halve - a smaller cake or loaf takes about two thirds as long - so test it with a skewer sooner than you think. Halve the raising agent exactly, and if the sum gives you half an egg, beat one and use half of it.",
        ),
    ),
    (
        "halve-a-recipe",
        "can i double the crumble",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "what if i doubled it",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "should i scale my recipe up",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "should i scale a recipe down",
        &[],
        Some(
            "Halve every ingredient and leave the oven temperature alone: the tin goes one size down if the batter looks thin in it, and start checking five minutes early. Cooking time does not halve - a smaller cake or loaf takes about two thirds as long - so test it with a skewer sooner than you think. Halve the raising agent exactly, and if the sum gives you half an egg, beat one and use half of it.",
        ),
    ),
    (
        "halve-a-recipe",
        "how do i scale a recipe down for twenty",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "how do i make enough for a crowd",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "how do i make a bigger batch",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "help me make half a batch",
        &[],
        Some(
            "Halve every ingredient and leave the oven temperature alone: the tin goes one size down if the batter looks thin in it, and start checking five minutes early. Cooking time does not halve - a smaller cake or loaf takes about two thirds as long - so test it with a skewer sooner than you think. Halve the raising agent exactly, and if the sum gives you half an egg, beat one and use half of it.",
        ),
    ),
    (
        "halve-a-recipe",
        "how do i make three times the recipe",
        &[],
        Some(
            "Halve every ingredient and leave the oven temperature alone: the tin goes one size down if the batter looks thin in it, and start checking five minutes early. Cooking time does not halve - a smaller cake or loaf takes about two thirds as long - so test it with a skewer sooner than you think. Halve the raising agent exactly, and if the sum gives you half an egg, beat one and use half of it.",
        ),
    ),
    (
        "halve-a-recipe",
        "how to double-up a recipe",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "how do i cut this recipe in half",
        &[],
        Some(
            "Halve every ingredient and leave the oven temperature alone: the tin goes one size down if the batter looks thin in it, and start checking five minutes early. Cooking time does not halve - a smaller cake or loaf takes about two thirds as long - so test it with a skewer sooner than you think. Halve the raising agent exactly, and if the sum gives you half an egg, beat one and use half of it.",
        ),
    ),
    (
        "halve-a-recipe",
        "how do i adapt for 201 people",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "can i double’s a loaf",
        &[],
        Some(
            "Double every ingredient and use a bigger tin rather than a deeper one, so the middle cooks at the same speed. It takes longer than twice the time is worth assuming: start checking at the original time and give it another ten or fifteen minutes if the skewer comes out wet. Salt, spice and raising agent go up by less than double, to taste, and a big batch is better split between two tins than crammed into one.",
        ),
    ),
    (
        "halve-a-recipe",
        "can i double's a loaf",
        &[],
        Some(
            "Halve every ingredient and leave the oven temperature alone: the tin goes one size down if the batter looks thin in it, and start checking five minutes early. Cooking time does not halve - a smaller cake or loaf takes about two thirds as long - so test it with a skewer sooner than you think. Halve the raising agent exactly, and if the sum gives you half an egg, beat one and use half of it.",
        ),
    ),
    (
        "kids-vegetables",
        "how do i get my son to eat vegetables",
        &[],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "kids-vegetables",
        "my kid refuses peas",
        &[],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "kids-vegetables",
        "how should i eat more vegetables",
        &[],
        None,
    ),
    (
        "kids-vegetables",
        "how can i blend veg into sauce",
        &[("my boy is fussy", "")],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "kids-vegetables",
        "how can i blend veg into sauce",
        &[("my boy is hungry", "")],
        None,
    ),
    (
        "kids-vegetables",
        "can i hide veg in sauce",
        &[
            ("my son is fussy", ""),
            ("he eats pasta", ""),
            ("he likes sauce", ""),
        ],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "kids-vegetables",
        "can i hide veg in sauce",
        &[
            ("my son is fussy", ""),
            ("he eats pasta", ""),
            ("he likes sauce", ""),
            ("a red sauce", ""),
        ],
        None,
    ),
    (
        "kids-vegetables",
        "my daughter wont eat broccoli what can i do",
        &[(
            "she likes crisps",
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        )],
        None,
    ),
    (
        "kids-vegetables",
        "my daughter wont eat broccoli what can i do",
        &[
            (
                "she likes crisps",
                "KEEP SERVING ONE SMALL PIECE OF VEG NEXT TO SOMETHING THEY ALREADY LIKE, WITHOUT A FIGHT OR A REWARD: IT CAN TAKE TEN TRIES BEFORE A TODDLER ACCEPTS A NEW ONE. ROASTING CARROTS OR BROCCOLI IN A LITTLE OIL MAKES THEM SWEETER, AND A DIP TO DUNK TURNS IT INTO A GAME. GRATED OR BLENDED INTO WHAT THEY ALREADY EAT - CARROT IN THE BOLOGNESE, COURGETTE IN A MUFFIN, SPINACH IN A SOUP - GETS THE VEG IN WITHOUT A STANDOFF.",
            ),
            ("she does not like peas", ""),
        ],
        None,
    ),
    (
        "kids-vegetables",
        "my daughter wont eat broccoli what can i do",
        &[
            (
                "she likes crisps",
                "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
            ),
            ("she does not like peas", ""),
            ("she likes carrots", ""),
        ],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "kids-vegetables",
        "how can i eat more veg",
        &[("My Son Likes Peas", "")],
        None,
    ),
    (
        "kids-vegetables",
        "how can i eat more veg",
        &[(
            "my son likes peas",
            "For an idea: Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        )],
        None,
    ),
    (
        "more-vegetables",
        "how do i get my son to eat vegetables",
        &[],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "more-vegetables",
        "my kid refuses peas",
        &[],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "more-vegetables",
        "how should i eat more vegetables",
        &[],
        Some(
            "Put them where you already eat rather than on a separate pile: grated carrot or courgette in the mince, a handful of spinach stirred into pasta at the end, frozen peas straight into rice, beans or lentils bulking out a chilli. Frozen and tinned count, and so does a bag of mixed veg thrown into a tray bake. Aim for one extra portion at a meal you already have rather than a new meal you have to keep up.",
        ),
    ),
    (
        "more-vegetables",
        "how can i blend veg into sauce",
        &[("my boy is fussy", "")],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "more-vegetables",
        "how can i blend veg into sauce",
        &[("my boy is hungry", "")],
        Some(
            "Put them where you already eat rather than on a separate pile: grated carrot or courgette in the mince, a handful of spinach stirred into pasta at the end, frozen peas straight into rice, beans or lentils bulking out a chilli. Frozen and tinned count, and so does a bag of mixed veg thrown into a tray bake. Aim for one extra portion at a meal you already have rather than a new meal you have to keep up.",
        ),
    ),
    (
        "more-vegetables",
        "can i hide veg in sauce",
        &[
            ("my son is fussy", ""),
            ("he eats pasta", ""),
            ("he likes sauce", ""),
        ],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "more-vegetables",
        "can i hide veg in sauce",
        &[
            ("my son is fussy", ""),
            ("he eats pasta", ""),
            ("he likes sauce", ""),
            ("a red sauce", ""),
        ],
        Some(
            "Put them where you already eat rather than on a separate pile: grated carrot or courgette in the mince, a handful of spinach stirred into pasta at the end, frozen peas straight into rice, beans or lentils bulking out a chilli. Frozen and tinned count, and so does a bag of mixed veg thrown into a tray bake. Aim for one extra portion at a meal you already have rather than a new meal you have to keep up.",
        ),
    ),
    (
        "more-vegetables",
        "my daughter wont eat broccoli what can i do",
        &[(
            "she likes crisps",
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        )],
        None,
    ),
    (
        "more-vegetables",
        "my daughter wont eat broccoli what can i do",
        &[
            (
                "she likes crisps",
                "KEEP SERVING ONE SMALL PIECE OF VEG NEXT TO SOMETHING THEY ALREADY LIKE, WITHOUT A FIGHT OR A REWARD: IT CAN TAKE TEN TRIES BEFORE A TODDLER ACCEPTS A NEW ONE. ROASTING CARROTS OR BROCCOLI IN A LITTLE OIL MAKES THEM SWEETER, AND A DIP TO DUNK TURNS IT INTO A GAME. GRATED OR BLENDED INTO WHAT THEY ALREADY EAT - CARROT IN THE BOLOGNESE, COURGETTE IN A MUFFIN, SPINACH IN A SOUP - GETS THE VEG IN WITHOUT A STANDOFF.",
            ),
            ("she does not like peas", ""),
        ],
        None,
    ),
    (
        "more-vegetables",
        "my daughter wont eat broccoli what can i do",
        &[
            (
                "she likes crisps",
                "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
            ),
            ("she does not like peas", ""),
            ("she likes carrots", ""),
        ],
        Some(
            "Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        ),
    ),
    (
        "more-vegetables",
        "how can i eat more veg",
        &[("My Son Likes Peas", "")],
        Some(
            "Put them where you already eat rather than on a separate pile: grated carrot or courgette in the mince, a handful of spinach stirred into pasta at the end, frozen peas straight into rice, beans or lentils bulking out a chilli. Frozen and tinned count, and so does a bag of mixed veg thrown into a tray bake. Aim for one extra portion at a meal you already have rather than a new meal you have to keep up.",
        ),
    ),
    (
        "more-vegetables",
        "how can i eat more veg",
        &[(
            "my son likes peas",
            "For an idea: Keep serving one small piece of veg next to something they already like, without a fight or a reward: it can take ten tries before a toddler accepts a new one. Roasting carrots or broccoli in a little oil makes them sweeter, and a dip to dunk turns it into a game. Grated or blended into what they already eat - carrot in the bolognese, courgette in a muffin, spinach in a soup - gets the veg in without a standoff.",
        )],
        None,
    ),
];
