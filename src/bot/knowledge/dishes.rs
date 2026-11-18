//! Definitions, aliases and disambiguation from bot/knowledge/dishes.py.
#[path = "announced.rs"]
mod announced;
pub(super) use announced::followup;
pub(crate) use announced::{doneness_reply::respond as observed_doneness, part as announced_part};
pub(super) use announced::{
    named_as_excluded, named_dish_has_heat, named_recipe_phrase, names_dish, oven_setting,
    plan_label, plan_label_in_chat, practical_plan,
};

pub(super) fn same_word(first: &str, second: &str) -> bool {
    first == second
        || [(first, second), (second, first)]
            .into_iter()
            .any(|(longer, shorter)| {
                longer.chars().nth(3).is_some()
                    && (["s", "es", "ing", "ed"]
                        .into_iter()
                        .any(|suffix| longer.strip_suffix(suffix) == Some(shorter))
                        || longer
                            .strip_suffix("ing")
                            .is_some_and(|stem| shorter.strip_suffix('e') == Some(stem)))
            })
}

pub(super) fn names(words: &[String], forms: &[Vec<String>]) -> bool {
    forms
        .iter()
        .filter(|phrase| !phrase.is_empty())
        .any(|phrase| {
            words.windows(phrase.len()).any(|window| {
                window
                    .iter()
                    .zip(phrase)
                    .all(|(actual, expected)| same_word(actual, expected))
            })
        })
}

struct Dish {
    names: &'static [&'static str],
    longest: &'static str,
    description: &'static str,
}

const ASKS: &[&str] = &[
    "what is",
    "what's",
    "whats",
    "what are",
    "what was",
    "mean",
    "means",
    "meaning",
    "definition",
    "explain",
    "tell me about",
    "what do you mean",
    "is a",
    "is an",
    "is it a",
    "are they",
    "what kind of",
    "what sort of",
    "where does",
    "where do",
    "what does",
];
const LEAD: &[&str] = &[
    "quick one",
    "one thing",
    "actually",
    "please",
    "sorry",
    "quick",
    "hello",
    "right",
    "okay",
    "then",
    "also",
    "yeah",
    "hiya",
    "just",
    "well",
    "and",
    "but",
    "yes",
    "so",
    "ok",
    "hi",
    "oh",
    "um",
    "er",
    "",
];
const AVOID: &[&str] = &[
    "how do i",
    "how to",
    "how should i",
    "best way",
    "recipe",
    "recipes",
    "goes with",
    "go with",
    "go nicely",
    "serve with",
    "serving with",
    "side for",
    "sides for",
    "side dish",
    "with what",
    "alongside",
    "to serve",
    "for dinner",
    "for tea",
    "for lunch",
    "for a crowd",
    "for two",
    "for four",
    "what about",
    "how about",
    "what if",
    "same for",
    "favourite",
    "favorite",
    "favourites",
    "favorites",
    "your favourite",
    "your favorite",
    "you like",
    "do you like",
    "would you",
    "prefer",
    "your pick",
    "pick one",
    "which one",
    "what would you",
    "how long",
    "how much",
    "make it",
    "make them",
    "cook it",
    "cook them",
    "should i",
    "can i",
    "calories",
    "good for you",
    "healthy",
    "what is a good",
    "what is the best",
    "best way",
    "for dinner",
    "ideas",
    "suggest",
    "recommend",
    "which is better",
    "casserole dish",
    "hash brown",
    "hash browns",
    "tap",
    "taps",
    "washer",
    "plumbing",
    "pipe",
    "pipes",
    "paint",
    "radiator",
    "boiler",
    "called",
    "named",
    "puppy",
    "puppies",
    "dog",
    "dogs",
    "kitten",
    "kittens",
    "cat",
    "cats",
    "hamster",
    "rabbit",
    "lol",
    "haha",
    "ha",
    "thanks",
    "ta",
    "cheers",
    "bye",
    "gotta",
    "swap",
    "swaps",
    "swap it",
    "ceiling",
    "ceilings",
    "roof",
    "wall",
    "walls",
    "floor",
    "floors",
    "water",
    "damp",
    "fridge",
    "freezer",
    "window",
    "windows",
    "door",
    "doors",
    "tag",
    "tags",
    "hashtag",
    "hashtags",
    "symbol",
    "key",
    "code",
    "sign",
    "taste like",
    "tastes like",
    "smell like",
    "look like",
    "sound like",
    "dance",
    "dances",
    "dancing",
    "dancers",
];
const BARE: &[&str] = &["pie", "pies", "a pie", "pudding", "puddings", "a pudding"];
const DISHES: &[Dish] = &[
    Dish {
        names: &["steak and kidney pudding", "steak and kidney puddings"],
        longest: "steak and kidney puddings",
        description: "Steak and kidney pudding is diced beef and kidney with gravy, sealed under a suet pastry crust and steamed for three or four hours rather than baked, then turned out onto the plate. The suet crust is what makes it a pudding: it stays light and soaks up the gravy, where a shortcrust would go heavy. If you want it brown on top, the pastry was baked and that is a pie.",
    },
    Dish {
        names: &[
            "yorkshire pudding",
            "yorkshire puddings",
            "yorkies",
            "yorkie puds",
            "yorkshire puds",
        ],
        longest: "yorkshire puddings",
        description: "A Yorkshire pudding is a batter of plain flour, eggs and milk baked in a thin layer of smoking-hot beef dripping or oil, so it puffs up crisp at the edges and soft in the middle. The fat has to be almost smoking before the batter goes in and the oven door stays shut for the first twenty minutes, or it will not rise. It is served with roast beef and gravy, and the batter makes toad in the hole around sausages.",
    },
    Dish {
        names: &["fricassee", "fricassée", "fricasseed chicken"],
        longest: "fricasseed chicken",
        description: "A fricassee is chicken, rabbit or veal cut into pieces, browned lightly and then stewed in stock and finished with a white sauce of cream or egg yolk - a French method, and a mild, pale dish rather than a brown stew. It is what to cook when someone wants something gentle. Take it off the heat before the egg yolk goes in, or it curdles.",
    },
    Dish {
        names: &["coronation chicken"],
        longest: "coronation chicken",
        description: "Coronation chicken is cold cooked chicken in a mild curry and mayonnaise sauce with apricot, mango chutney or sultanas - a dish invented for the Queen's coronation in 1953, so it is meant to be gentle rather than hot. It goes in sandwiches, jacket potatoes or on a salad. Curry powder is better cooked in a little butter and cooled before it meets the mayonnaise.",
    },
    Dish {
        names: &["christmas pudding", "christmas puddings", "plum pudding"],
        longest: "christmas puddings",
        description: "Christmas pudding is a steamed suet pudding of dried fruit, breadcrumbs, brown sugar, spice and ale or brandy, made weeks ahead so it matures and then reheated on the day - Stir-up Sunday, five weeks before, is the traditional day to make it. It is steamed for hours, never baked, and served with brandy butter or custard. Wrapped and kept cool it lasts a year.",
    },
    Dish {
        names: &["cottage pie", "cottage pies", "beef pie with mash"],
        longest: "beef pie with mash",
        description: "Cottage pie is minced beef with onion and gravy under a mash crust, baked until the top is crisp - the beef version of shepherd's pie. Brown the mince properly first and let the gravy reduce, or the mash slides off in the serving. Fork the mash up before it goes in the oven so the ridges brown.",
    },
    Dish {
        names: &["bubble and squeak", "bubble n squeak"],
        longest: "bubble and squeak",
        description: "Bubble and squeak is leftover potatoes and cabbage or sprouts crushed together and fried in a hot pan until the underside is crusty brown - the name is the noise it made in the pan. Cold potatoes work better than fresh, because they have dried out. A fried egg on top makes it breakfast.",
    },
    Dish {
        names: &[
            "bap",
            "baps",
            "barm",
            "barm cake",
            "cob",
            "roll with a crust",
        ],
        longest: "roll with a crust",
        description: "A bap is a soft white bread roll, usually with a floury top and a little milk or butter in the dough, and the same thing is a barm cake in Lancashire and a cob in the Midlands - it is the roll a bacon butty or a burger goes in. Softness is the point, so bake it hot and briefly and cover the rolls with a cloth as they cool. A bap is not a crusty roll, and the two are not swapped.",
    },
    Dish {
        names: &["hollandaise", "hollandaise sauce"],
        longest: "hollandaise sauce",
        description: "Hollandaise is egg yolks whisked over gentle heat until thick, then warm clarified butter whisked in drop by drop to make an emulsion, sharpened with lemon and salt. It is the sauce on eggs benedict. Keep it warm rather than hot, because boiling splits it, and a teaspoon of cold water whisked in often brings a split one back.",
    },
    Dish {
        names: &["aioli", "garlic mayonnaise"],
        longest: "garlic mayonnaise",
        description: "Aioli is garlic pounded to a paste with salt, then olive oil whisked in to make a thick emulsion - in Provence that is all it is, and the version with egg yolk is mayonnaise with garlic. It is served cold with fish, boiled vegetables and bread. Pounding the garlic with a pinch of salt first stops it turning bitter.",
    },
    Dish {
        names: &["clootie dumpling", "clootie dumplings"],
        longest: "clootie dumplings",
        description: "A clootie dumpling is a Scottish steamed fruit pudding, like a christmas pudding but rounder and softer, tied in a floured cloth - a cloot - and boiled for hours rather than steamed in a basin. The cloth gives it its wrinkled skin. Serve it with custard, or sliced and fried with a cooked breakfast.",
    },
    Dish {
        names: &["silverside", "salt silverside", "salted silverside"],
        longest: "salted silverside",
        description: "Silverside is a boned cut from the back of the hindquarter, usually salted and simmered rather than roasted, then served hot with mustard sauce or cold in sandwiches. Soak a very salty one overnight before cooking, then bring it to a bare simmer and hold it there - a rolling boil makes it stringy. Brined and tinned, the same cut is sold as corned beef.",
    },
    Dish {
        names: &["toad in the hole", "toad in the hole"],
        longest: "toad in the hole",
        description: "Toad in the hole is sausages baked in a Yorkshire pudding batter, in one roasting tin, so the batter rises around them and the sausages flavour it. Brown the sausages first in the tin with a little oil, get that fat hot, then pour the batter in and bake at 220C (200C fan) for 25 to 30 minutes. Onion gravy over the top is the usual thing.",
    },
    Dish {
        names: &["bara brith", "bara brith bread"],
        longest: "bara brith bread",
        description: "Bara brith means speckled bread in Welsh: a fruit loaf of dried fruit, brown sugar and mixed spice, usually with the fruit soaked overnight in strong tea first. It is baked in a loaf tin like a cake, left a day to settle, then sliced and buttered. It keeps a week in a tin and is better on the second day.",
    },
    Dish {
        names: &["parkin", "parkin cake", "yorkshire parkin"],
        longest: "yorkshire parkin",
        description: "Parkin is a sticky ginger cake from Yorkshire and Lancashire, made with oatmeal, black treacle, golden syrup and ground ginger - oats in a cake is what makes it chewy rather than spongy. It comes out soft and improves for three or four days in a tin, so it is made for bonfire night and eaten the week after. Cut it into squares, not slices.",
    },
    Dish {
        names: &["hash", "corned beef hash", "hash dinner"],
        longest: "corned beef hash",
        description: "Hash is cooked meat and potatoes chopped up and fried together in a hot pan until the edges crisp - corned beef hash is the version most people mean, with onion and a little stock or gravy to keep it moist. It is a way to use up a Sunday roast, so the potatoes want to be cold and cooked. A fried egg or brown sauce on top is not optional for most people.",
    },
    Dish {
        names: &[
            "suet pudding",
            "suet puddings",
            "steamed pudding",
            "steamed puddings",
        ],
        longest: "steamed puddings",
        description: "A suet pudding is a pudding made with shredded beef suet instead of butter - either a suet pastry around a filling, like steak and kidney pudding, or a suet sponge with dried fruit, like spotted dick. The suet melts as it steams, so it comes out light where a butter sponge would sit heavy. It is always steamed rather than baked, for one and a half hours and up, in a basin covered with paper.",
    },
    Dish {
        names: &["halloumi", "halloumi cheese", "grilled halloumi"],
        longest: "grilled halloumi",
        description: "Halloumi is a Cypriot cheese of sheep's and goat's milk that is heated and pressed as it is made, so it holds its shape and squeaks instead of melting. Slice it, dry the slices on kitchen paper, then fry or grill hard for a minute a side until the outside is dark and crisp. It is very salty, so it wants lemon, chilli or honey rather than more salt.",
    },
    Dish {
        names: &["polenta", "polenta grain", "cornmeal polenta"],
        longest: "cornmeal polenta",
        description: "Polenta is coarsely ground cornmeal simmered in water or stock until it thickens and pulls away from the side of the pan: about 40 minutes of stirring for the coarse kind, five for the instant. Served soft with butter and parmesan, or spread in a tray, cooled firm and cut into slices to fry. The fried leftovers are the best part.",
    },
    Dish {
        names: &["couscous", "cous cous", "instant couscous"],
        longest: "instant couscous",
        description: "Couscous is semolina rolled into tiny grains by hand and steamed over the stew rather than boiled; the boxed instant kind has already been steamed, so it only needs boiling water poured over it. Use the same volume of water as grain, cover it for five minutes, then fluff it with a fork and stir in butter or oil. It is the bed the stew sits on.",
    },
    Dish {
        names: &[
            "coriander",
            "cilantro",
            "coriander leaves",
            "ground coriander",
            "coriander seeds",
        ],
        longest: "coriander leaves",
        description: "Coriander is two different things from one plant: the leaves and stalks, which taste bright and faintly soapy and go in raw at the end, and the ground seed, which is warm and citrusy and goes in at the start. Toast the seeds to wake them up before grinding. In a curry paste the root is the most flavoursome part.",
    },
    Dish {
        names: &["samosa", "samosas", "vegetable samosa", "lamb samosa"],
        longest: "vegetable samosa",
        description: "A samosa is a pastry triangle of spiced potato and peas, or minced lamb, wrapped in a thin flour pastry and deep-fried until it blisters - or baked, which is lighter but not the same. The filling has to be cool and dry before it is wrapped or the pastry goes soft. Serve them with a yoghurt dip or a tamarind chutney.",
    },
    Dish {
        names: &[
            "gherkin",
            "gherkins",
            "cornichon",
            "cornichons",
            "pickled cucumber",
        ],
        longest: "pickled cucumber",
        description: "A gherkin is a small cucumber pickled in vinegar or brine, eaten whole as a snack or sliced into a burger; cornichons are the tiny sour French ones. A fresh gherkin is simply a small cucumber, and only the pickled one keeps for months in the jar. Rinse them if you want less salt.",
    },
    Dish {
        names: &["ragu", "ragù", "ragus", "bolognese sauce", "bolognese"],
        longest: "bolognese sauce",
        description: "A ragu is a slow-cooked Italian meat sauce: onion, carrot and celery softened first, then mince or a whole piece of meat, wine, a little milk and tomatoes, simmered for two hours or more - bolognese is one of them. The long cook and the milk are what make it rich rather than sharp. In Bologna it is served with tagliatelle, because the ragù clings to the wide strands.",
    },
    Dish {
        names: &["beef wellington", "wellington beef"],
        longest: "beef wellington",
        description: "Beef Wellington is a fillet of beef seared, coated in mushroom duxelles and sometimes pâté, wrapped in puff pastry and baked, then rested and carved in thick slices. The trick is chilling it after wrapping so the pastry does not go soggy, and a hot oven, 200C (180C fan), for about 25 minutes for medium rare. It is a dish for a celebration: the fillet is the expensive cut and it does not want cooking through.",
    },
    Dish {
        names: &[
            "scotch pancake",
            "scotch pancakes",
            "drop scone",
            "drop scones",
        ],
        longest: "scotch pancakes",
        description: "A Scotch pancake is a small thick pancake from a thick batter poured in spoonfuls onto a griddle - a drop scone - and it is eaten cold with butter or jam rather than hot with lemon. Self-raising flour and a rest of ten minutes before cooking give it the height, and the pan wants to be dry and only just hot enough. They freeze well and toast from frozen.",
    },
    Dish {
        names: &[
            "fruit fool",
            "gooseberry fool",
            "raspberry fool",
            "rhubarb fool",
        ],
        longest: "gooseberry fool",
        description: "A fruit fool is stewed fruit folded into whipped cream and chilled - gooseberry is the classic, and raspberry and rhubarb work the same way. The fruit has to be cold and thick before it is folded in or it loosens the cream. It is one of the oldest British puddings and one of the quickest.",
    },
    Dish {
        names: &["brisket", "beef brisket", "brisket of beef"],
        longest: "brisket of beef",
        description: "Brisket is the boned chest of the beef, a tough cut full of connective tissue, so it wants long, slow, moist heat - four or five hours at 150C - rather than a quick roast. Cooked like that it falls into soft slices; pot-roasted with stock and onions it makes its own gravy. Always slice it across the grain.",
    },
    Dish {
        names: &["turmeric", "tumeric", "ground turmeric", "turmeric powder"],
        longest: "ground turmeric",
        description: "Turmeric is a bright yellow root, used dried and ground, with an earthy, slightly bitter taste and no real heat - it is in curry powder mostly for colour. A little goes a long way, and it stains boards, cloth and fingers yellow, so keep it off anything pale. Fresh root is milder and can be grated into rice or a dressing.",
    },
    Dish {
        names: &[
            "shepherd's pie",
            "shepherds pie",
            "shepherd pie",
            "shepherd's pies",
        ],
        longest: "shepherd's pies",
        description: "Shepherd's pie is minced lamb cooked with onion and gravy in a dish, with mashed potato spread over the top and baked until the ridges catch and brown. Lamb is what makes it a shepherd's pie - made with beef it is a cottage pie. Let the mince cool a little before the mash goes on, or it sinks in.",
    },
    Dish {
        names: &["black pudding", "black puddings"],
        longest: "black puddings",
        description: "Black pudding is a sausage of pig's blood, pork fat, oatmeal and seasoning, sliced thick and fried until the outside crisps - it is a blood sausage, not a dessert. Stornoway and Bury are the ones people argue about. It is eaten with a full breakfast, or crumbled into scallops and apple.",
    },
    Dish {
        names: &["bechamel", "bechamel sauce", "white sauce"],
        longest: "bechamel sauce",
        description: "Bechamel is milk thickened with a roux of equal weights of butter and plain flour, cooked out for a minute so it stops tasting of raw flour, then seasoned with salt, pepper and a grating of nutmeg. It is the white sauce under lasagne and cauliflower cheese, and stirring in cheese turns it into a Mornay. Warm the milk first and whisk from the bottom of the pan to keep it smooth.",
    },
    Dish {
        names: &["raita", "cucumber raita", "raitas"],
        longest: "cucumber raita",
        description: "Raita is an Indian side of plain yoghurt with grated cucumber, salt and roasted cumin, sometimes mint or tomato, served cold beside a hot curry. Whisk the yoghurt smooth, stir in cucumber that has been salted and squeezed, and dust the cumin on just before it goes to the table. It is there to cool the mouth, so keep it simple.",
    },
    Dish {
        names: &["kipper", "kippers", "smoked herring"],
        longest: "smoked herring",
        description: "A kipper is a herring that has been split, salted and smoked, so it is already cooked and needs only heating: grill it or poach it in a little water for three or four minutes. It comes as a whole flattened fish with a lot of fine bones. Serve it with buttered bread and a squeeze of lemon, and eat it for breakfast if you are in Scotland.",
    },
    Dish {
        names: &["plantain", "plantains", "cooking banana", "green plantain"],
        longest: "cooking banana",
        description: "Plantain is a cooking banana: bigger, starchier and meant to be cooked, never eaten raw. Green ones are savoury and firm, so slice them thin and fry them crisp; left until the skin is black they are sweet, and thick coins fried in butter go soft and caramelised. Either way they take salt, chilli or a bean stew.",
    },
    Dish {
        names: &["parsnip", "parsnips", "roast parsnips"],
        longest: "roast parsnips",
        description: "A parsnip is a long cream root with a sweet, peppery taste that gets sweeter after a frost because the cold turns its starch to sugar. Roast it in chunks around the meat or mash it, and cut the woody core out of a big one first. Peel it thinly, since most of the flavour sits just under the skin.",
    },
    Dish {
        names: &["shallot", "shallots", "banana shallot"],
        longest: "banana shallot",
        description: "A shallot is a small elongated onion of its own variety, milder and sweeter than an onion, and it splits into two or three cloves under the skin. It is the one to use raw in a vinaigrette or pickled whole, and it softens and caramelises faster than an onion does. The banana shallot is the long one and is easiest to peel.",
    },
    Dish {
        names: &["paprika", "smoked paprika", "sweet paprika", "hot paprika"],
        longest: "smoked paprika",
        description: "Paprika is ground dried peppers - sweet, hot or smoked depending on which chilli was used, and the Hungarian sweet one is what people usually mean. Sweet paprika cannot take high heat: stir it in near the end, or bloom it in warm oil for a few seconds, or it turns bitter. Smoked paprika is for depth, in chorizo, beans and roast chicken.",
    },
    Dish {
        names: &[
            "samphire",
            "marsh samphire",
            "sea samphire",
            "rock samphire",
        ],
        longest: "marsh samphire",
        description: "Samphire is a salty green that grows on mudflats and beaches, looking like fleshy joined-up stems, sold fresh in summer - that one is marsh samphire. Wash it well, steam or boil it for about three minutes and eat it with butter and lemon, no salt. Rock samphire is the rarer, stronger-tasting one.",
    },
    Dish {
        names: &["cobbler", "fruit cobbler", "peach cobbler"],
        longest: "fruit cobbler",
        description: "A cobbler is fruit baked under rough spoonfuls of scone dough, so the topping comes out cobbled rather than rolled flat - it is American, and it is not a pie or a crumble. The fruit should be bubbling at the edges and the topping golden, and the dough wants a spoonful of sugar and a little salt in it. Cream or custard over the top, hot.",
    },
    Dish {
        names: &["cornish pasty", "pasty", "pasty's", "pasties"],
        longest: "cornish pasty",
        description: "A Cornish pasty is a half-moon of shortcrust pastry crimped along one edge, filled with beef skirt, potato, swede and onion, and baked without a tin - the raw filling cooks inside the sealed pastry, which is why it is juicy. It is eaten by hand, and the thick crimp was the handle for a miner's dirty fingers. Season it well inside: the pastry keeps the steam in, so a bland filling stays bland.",
    },
    Dish {
        names: &["dripping", "beef dripping"],
        longest: "beef dripping",
        description: "Dripping is the fat that renders out of a beef roast and sets to a pale, grainy solid in the fridge - it is the best fat there is for roast potatoes and Yorkshire puddings, because it takes a much higher heat than oil without burning. Pour it off the roasting tin through a sieve into a jar, keep it in the fridge for weeks, and spoon out what you need. The dark jelly under it is the roasting juices and it goes in the gravy.",
    },
    Dish {
        names: &["veloute", "veloute sauce"],
        longest: "veloute sauce",
        description: "A veloute is a roux loosened with white stock instead of milk, simmered until it coats a spoon and finished with cream or egg yolk. It is the base of a lot of classic French sauces. Use a stock you would drink on its own, because there is nothing in a veloute to hide a weak one.",
    },
    Dish {
        names: &["salsa", "salsa sauce", "pico de gallo", "tomato salsa"],
        longest: "pico de gallo",
        description: "Salsa is a Mexican sauce, most often raw tomato, onion, chilli, lime and coriander chopped together - that one is pico de gallo, and it is a relish rather than a dip. Cooked and blended it is a smoother, smokier salsa for chips or for baking under. The dance of the same name is a different question.",
    },
    Dish {
        names: &["paneer", "paneer cheese", "indian cheese"],
        longest: "paneer cheese",
        description: "Paneer is an Indian cheese made by curdling hot milk with lemon juice or vinegar and pressing the curds under a weight, so it is firm, mild and does not melt - it browns in the pan instead. Fry the cubes first so they keep their shape, then add them to the sauce at the end. It takes on whatever spice is around it.",
    },
    Dish {
        names: &[
            "bulgur",
            "burghul",
            "bulghur",
            "bulgur wheat",
            "cracked wheat",
        ],
        longest: "cracked wheat",
        description: "Bulgur is wheat that has been parboiled, dried and cracked, so it needs only soaking or a few minutes of simmering rather than half an hour. Coarse bulgur is for pilafs and salads, fine bulgur for kibbeh and tabbouleh. It swells to twice its size and keeps a firm bite where rice would go soft.",
    },
    Dish {
        names: &["caper", "capers", "caper berries", "caperberries"],
        longest: "caper berries",
        description: "Capers are the unopened flower buds of a Mediterranean shrub, pickled in brine or packed in salt, and used for a sour, salty hit in tartare sauce, with salmon, and in puttanesca. Rinse the salted ones before use, and chop rather than crush them. The bigger caperberries are the open fruit of the same plant and are eaten whole.",
    },
    Dish {
        names: &["salsify", "black salsify", "oyster plant", "scorzonera"],
        longest: "black salsify",
        description: "Salsify is a long thin root that tastes faintly of oyster: the black-skinned one is salsify and the white one is scorzonera. Peel it and drop it straight into water with lemon juice, or it browns in minutes, then roast or fry it in butter. It is a winter vegetable and it is best simply cooked.",
    },
    Dish {
        names: &["fennel", "fennel bulb", "fennel bulbs", "fennel fronds"],
        longest: "fennel fronds",
        description: "Fennel is the vegetable with the fat white bulb and feathery green fronds, tasting of aniseed; the seed is the same plant's spice. Cut the bulb through the core so the wedges hold together, brown it hard in a pan, then braise or roast it, and use the fronds raw as a herb. Raw shaved thin with lemon it is a salad on its own.",
    },
    Dish {
        names: &["chive", "chives", "garlic chives"],
        longest: "garlic chives",
        description: "Chives are the thin green stems of a small allium with a mild onion taste, used raw as a garnish - snipped over eggs, potatoes, soup or a baked potato. Heat kills the flavour, so add them at the end rather than in the pan. They grow back after cutting, which is why a pot on the windowsill keeps going.",
    },
    Dish {
        names: &["pudding", "puddings", "a pudding", "sweet pudding"],
        longest: "sweet pudding",
        description: "A pudding is the sweet course at the end of a British meal, and also a family of dishes steamed or baked under suet pastry or a sponge - sticky toffee, treacle, rice, Christmas pudding. The word means either the course or the dish, so which one it is depends on the sentence. Steamed ones cook covered in a water bath; baked ones go straight in the oven.",
    },
    Dish {
        names: &["compote", "fruit compote"],
        longest: "fruit compote",
        description: "A compote is whole or large pieces of fruit poached gently in sugar syrup, sometimes with vanilla, lemon peel or spice, until they are soft but still in shape. It keeps in the fridge for a few days and is served with cream, yoghurt or porridge. Use less syrup than you think, because the fruit gives up its own juice.",
    },
    Dish {
        names: &["coulis", "fruit coulis"],
        longest: "fruit coulis",
        description: "A coulis is a smooth sauce of raw or lightly cooked fruit blended and then pushed through a sieve, spooned over ice cream, cheesecake or pancakes - raspberry and mango are the usual ones. Sieving is what makes it a coulis rather than a purée, and a squeeze of lemon keeps the colour bright. A spoonful of icing sugar if the fruit is sharp.",
    },
    Dish {
        names: &["spotted dick"],
        longest: "spotted dick",
        description: "Spotted dick is a steamed suet pudding with currants and raisins in it - the spots are the dried fruit - served with custard. It is made in a pudding basin, covered and steamed for an hour and a half rather than baked, which is why it is light rather than heavy. A spoon of golden syrup in the bottom of the basin is the usual trick.",
    },
    Dish {
        names: &["banoffee pie", "banoffi pie"],
        longest: "banoffee pie",
        description: "Banoffee pie is a biscuit-and-butter base, a thick layer of toffee made by boiling a tin of condensed milk, sliced bananas and whipped cream on top, and none of it is baked. The toffee is the work: three hours in a pan of simmering water, and let the tin cool before opening it. Chill the pie for a few hours or it slumps when you cut it.",
    },
    Dish {
        names: &["stottie", "stottie cake", "stotties"],
        longest: "stottie cake",
        description: "A stottie is a big flat white loaf from Tyneside, baked slowly so it stays soft and a bit stodgy inside, then split and filled - ham and pease pudding in a stottie is the classic. The name comes from stott, to bounce, because the dough is thrown down to knock it back.",
    },
    Dish {
        names: &["tahini", "tahina", "sesame paste"],
        longest: "sesame paste",
        description: "Tahini is ground sesame seeds and nothing else - a thick, faintly bitter paste used in hummus, in dressings, and on its own beaten with lemon and water as a sauce. It seizes up when liquid goes in at first and turns creamy with more beating, so keep going. Stir the jar before you use it, keep it in the cupboard, and it lasts months.",
    },
    Dish {
        names: &["jus", "a jus", "red wine jus", "meat jus"],
        longest: "red wine jus",
        description: "A jus is the thin gravy that comes from a roast: the juices left in the tin, deglazed with wine or stock and reduced until it coats the back of a spoon. Nothing thickens it, which is what makes it lighter and glossier than gravy. Pour the fat off first and strain it if you want it clear.",
    },
    Dish {
        names: &["hake", "hake fillet", "hake fillets"],
        longest: "hake fillets",
        description: "Hake is a white sea fish in the cod family, soft-fleshed and sold as fillets or as steaks with a bone through the middle. It flakes more easily than cod, so cook it briefly - skin-side down in a hot pan, or 12 to 15 minutes in a hot oven. It takes strong flavours well: garlic, paprika, tomato and olive oil.",
    },
    Dish {
        names: &["quinoa", "quinoa grain", "red quinoa"],
        longest: "quinoa grain",
        description: "Quinoa is a seed from a South American plant, cooked like rice but faster, with a nutty taste and a slight crunch - it is not a grain. Rinse it before cooking to wash off the bitter coating, then simmer it in twice its volume of water for about 15 minutes until the little rings come away. It works hot as a side and cold in salads.",
    },
    Dish {
        names: &["cumin", "cumin seeds", "ground cumin", "jeera"],
        longest: "ground cumin",
        description: "Cumin is a warm, earthy spice sold as whole seeds or ground, and the seeds keep their flavour far longer than the powder. Toast the seeds for a minute in a dry pan, or fry them in the oil at the start of a curry, then crush them. It is the base note of most curry powders, of chilli and of a lot of Mexican cooking.",
    },
    Dish {
        names: &["bhaji", "bhajis", "bhajee", "onion bhaji", "onion bhajis"],
        longest: "onion bhajis",
        description: "A bhaji is a fritter of sliced vegetables in a spiced gram flour batter, dropped in spoonfuls into hot oil: onion is the usual one, and cauliflower and spinach work the same way. Gram flour, not wheat, is what makes it a bhaji, and the batter should just hold together. Eat them hot with a yoghurt or tamarind dip.",
    },
    Dish {
        names: &["scotch egg", "scotch eggs"],
        longest: "scotch eggs",
        description: "A Scotch egg is a hard-boiled egg wrapped in sausage meat, rolled in breadcrumbs and deep-fried, so the shell of meat keeps a little juice in and the yolk stays soft if it is boiled for six minutes first. The ones from a good butcher are a different thing from the chilled ones in a supermarket. Baked at 200C for 25 minutes works if you would rather not fry.",
    },
    Dish {
        names: &["pesto", "pesto sauce"],
        longest: "pesto sauce",
        description: "Pesto is basil, pine nuts, garlic and parmesan pounded with olive oil into a coarse green paste - pounded in a mortar rather than blended, because a blade heats it and turns it bitter. Stir it into hot pasta off the heat with a spoonful of the cooking water to loosen it. Walnuts or almonds stand in for pine nuts.",
    },
    Dish {
        names: &["vinaigrette"],
        longest: "vinaigrette",
        description: "A vinaigrette is oil and vinegar shaken together with salt, usually three parts oil to one of vinegar, with a little mustard to hold the two together. It is the dressing for a green salad and for cold vegetables. Make it in a jar and shake it again just before it goes on, because it separates as it stands.",
    },
    Dish {
        names: &["soda bread", "soda breads"],
        longest: "soda breads",
        description: "Soda bread is flour, bicarbonate of soda, salt and buttermilk with no yeast - the soda and the acid in the buttermilk make it rise as it bakes, so it is mixed and in the oven in minutes and eaten the same day. Handle the dough as little as possible and cut a cross in the top. It stales fast, so toast or freeze what is left.",
    },
    Dish {
        names: &["brioche", "brioche bun"],
        longest: "brioche bun",
        description: "Brioche is an enriched bread of flour, eggs and a lot of butter with a little sugar, so it is soft, yellow and slightly sweet. The butter goes in cold and a little at a time, and the dough wants a long cold prove. It makes the best burger bun and the worst sandwich for a wet filling.",
    },
    Dish {
        names: &["pitta", "pita", "pitta bread", "pita bread"],
        longest: "pitta bread",
        description: "Pitta is a plain flatbread of flour, water and yeast baked very hot and fast, so the steam inside separates the two layers and leaves a pocket. That is why it opens for filling instead of needing to be cut. Keep the pack wrapped so it stays soft enough to open.",
    },
    Dish {
        names: &["cheesecake", "cheesecakes"],
        longest: "cheesecakes",
        description: "Cheesecake is crushed biscuit and butter pressed into a tin with a soft cheese filling on top - baked with eggs and cream for the New York kind, or just chilled for the no-bake one. A baked cheesecake cracks when the oven is too hot, so bake it low and let it cool in the oven with the door ajar. Rest it in the fridge overnight.",
    },
    Dish {
        names: &["pie", "pies", "a pie", "savoury pie"],
        longest: "savoury pie",
        description: "A pie has pastry under the filling and a lid over it, or just a lid, where a tart is open and a cobbler has scones on top. Shortcrust is the plain one, puff the laminated one that rises in layers, and hot-water crust is for a raised pork pie. Fillings run from steak and ale to apple.",
    },
    Dish {
        names: &["casserole", "casseroles", "hotpot", "hot pot"],
        longest: "casseroles",
        description: "A casserole is meat and vegetables cooked slowly in liquid in one lidded pot, in the oven or on the hob - the point is a cheap cut and a long cook, an hour and a half to two hours at 160C, so shin, shoulder and thighs come out tender. Brown the meat first and keep the liquid to about a third of the way up, or it turns into soup. Dumplings go on top for the last twenty minutes; a casserole dish is the pot it is cooked in, not the food.",
    },
    Dish {
        names: &["mayonnaise"],
        longest: "mayonnaise",
        description: "Mayonnaise is egg yolk whisked with mustard, salt and vinegar, with oil added a little at a time until it thickens into an emulsion. The yolk is what holds the oil and the vinegar together. Eggs at room temperature and a slow drizzle at the start are what stop it splitting.",
    },
    Dish {
        names: &["naan", "naan bread", "naans"],
        longest: "naan bread",
        description: "Naan is a soft flatbread of white flour, yoghurt and yeast, slapped onto the wall of a hot tandoor or cooked in a very hot dry pan so it puffs and blisters in patches. It gets brushed with butter or ghee as it comes off the heat. The yoghurt is what keeps it soft the next day.",
    },
    Dish {
        names: &["hummus", "houmous", "humous", "houmus", "hummus dip"],
        longest: "hummus dip",
        description: "Hummus is a dip of cooked chickpeas blitzed with tahini, lemon juice, garlic and olive oil, loosened with a little cold water until it is smooth and pale. Tinned chickpeas work fine, and a pinch of bicarbonate of soda in the water while they simmer makes them softer. Spread it in a bowl with a swirl and a pool of olive oil, and eat it with flatbread or raw vegetables.",
    },
    Dish {
        names: &["leek", "leeks", "baby leeks"],
        longest: "baby leeks",
        description: "A leek is a long green and white allium, milder and sweeter than an onion, that softens and sweetens long before it browns. Slit it lengthwise and rinse between the layers, because grit grows down inside it. It is the base of leek and potato soup and it braises well in butter with a little stock.",
    },
    Dish {
        names: &["colcannon"],
        longest: "colcannon",
        description: "Colcannon is mashed potato with cooked cabbage or kale folded through it and plenty of butter, and it is a Halloween dish in Ireland - a ring or a coin used to be hidden in the pan. Champ is the same idea made with spring onions instead of cabbage. Warm milk and butter go in last, off the heat, or the mash goes gluey.",
    },
    Dish {
        names: &["frittata", "frittatas"],
        longest: "frittatas",
        description: "A frittata is an Italian open omelette of eggs and whatever is in the fridge, cooked slowly in a pan and then finished under the grill or in the oven so the top sets without the bottom burning. It is thicker than an omelette and cut in wedges, and it is as good cold the next day. Cook the vegetables first and drain anything wet, or it will weep.",
    },
    Dish {
        names: &["gratin", "gratins", "au gratin"],
        longest: "au gratin",
        description: "A gratin is anything topped with cheese, butter and breadcrumbs and browned under the grill or in a hot oven until the top is crisp - gratin dauphinois is potato and cream cooked for two hours, and a gratin of pasta or cauliflower works the same way. The browned top is the point, so give it the last five minutes uncovered. Breadcrumbs on their own brown more evenly than cheese.",
    },
    Dish {
        names: &["suet", "beef suet"],
        longest: "beef suet",
        description: "Suet is the hard white fat from around a cow's or sheep's kidneys, sold shredded and mixed with a little flour, and it is what makes a steamed pudding light and a dumpling fluffy - it melts slowly inside the dough and leaves air behind. Shortcrust and dumplings use it where a softer fat would go heavy. A vegetarian version made from palm or rice bran fat works the same way.",
    },
    Dish {
        names: &["marinade", "marinades"],
        longest: "marinades",
        description: "A marinade is a liquid meat, fish or vegetables sit in before cooking: something acidic such as wine, vinegar, lemon or yoghurt, plus oil, salt and spice. It seasons the surface and tenderises a little, and the salt and the time matter more than the flavourings. Half an hour for fish, a few hours for chicken, overnight for a tough cut, always in the fridge.",
    },
    Dish {
        names: &["dry rub", "spice rub"],
        longest: "spice rub",
        description: "A dry rub is salt, sugar and spices patted onto meat with no liquid, so the salt seasons it right through and the sugar helps the crust colour. Leave it on for half an hour at least, or overnight uncovered in the fridge for crisper skin. Pat off the loose spice before cooking so it does not burn.",
    },
    Dish {
        names: &["eton mess"],
        longest: "eton mess",
        description: "Eton mess is broken meringue, whipped cream and strawberries folded together, deliberately messy rather than layered. Assemble it just before it goes to the table or the meringue loses its crunch. It is said to come from a picnic at Eton where a strawberry and meringue pudding was dropped, and the name is the whole point.",
    },
    Dish {
        names: &["cranachan"],
        longest: "cranachan",
        description: "Cranachan is a Scottish pudding of whipped cream, toasted oats, whisky and honey with raspberries folded in. Toast the oats slowly with a little sugar so they stay crisp against the cream. It is served in layers with a dram on the side.",
    },
    Dish {
        names: &["kohlrabi", "kohl rabi"],
        longest: "kohl rabi",
        description: "Kohlrabi is a swollen stem rather than a root - pale green or purple, with a taste between cabbage and turnip and the crunch of a crisp apple. Peel the tough outer skin off, then slice it raw into salad or roast it in wedges until the edges caramelise. The leaves cook like cabbage.",
    },
    Dish {
        names: &["haggis", "haggises"],
        longest: "haggises",
        description: "Haggis is a Scottish sausage of sheep's heart, liver and lungs minced with oatmeal, suet, onion and pepper, and boiled in a casing - it is a savoury pudding, traditionally served with neeps and tatties. The oatmeal and the pepper are what make it crumbly and warming. A vegetarian one is made with lentils and oats, and the good ones are as peppery as the meat.",
    },
    Dish {
        names: &["biscuit", "biscuits"],
        longest: "biscuits",
        description: "In Britain a biscuit is a small crisp baked thing - a digestive, a custard cream, a shortbread - while in America a biscuit is a soft soda-raised bread roll eaten with gravy. The British kind gets its snap from sugar, fat and a short bake: take them out while they are still slightly soft and they crisp as they cool. A soft biscuit has usually been stored with moisture, so keep them in a tin, not the fridge.",
    },
    Dish {
        names: &["chutney", "chutneys"],
        longest: "chutneys",
        description: "Chutney is fruit or vegetables cooked down with vinegar, sugar and spice until thick and jammy, so it keeps for months in jars. The vinegar and sugar are what preserve it, so do not cut them down. An Indian chutney can also be a fresh uncooked relish, but the jarred British kind is the cooked one.",
    },
    Dish {
        names: &["focaccia"],
        longest: "focaccia",
        description: "Focaccia is a flat bread of soft wet dough stretched into an oiled tin, dimpled with the fingers and baked with olive oil, coarse salt and often rosemary. The dimples hold the oil, which is what makes the top crisp and the crumb light. It is meant to be wet, so resist adding flour to make it easier to handle.",
    },
    Dish {
        names: &["ciabatta"],
        longest: "ciabatta",
        description: "Ciabatta is a very wet Italian white bread, around three parts water to four of flour, which is where the big irregular holes and the chewy crust come from. The dough is too sticky to knead by hand, so it is stretched and folded instead. Prove it slowly and handle it gently or the air comes out.",
    },
    Dish {
        names: &["bannock", "bannocks"],
        longest: "bannocks",
        description: "Bannock is a flat quick bread from Scotland and the north of England, of flour, oatmeal or barley with bicarbonate of soda, salt and buttermilk, cooked on a hot girdle or in a pan rather than baked. It is ready in minutes and eaten warm with butter. What you buy is a dense round flat bread for toasting.",
    },
    Dish {
        names: &["crumpet", "crumpets"],
        longest: "crumpets",
        description: "A crumpet is a soft spongy griddle cake of batter cooked on one side only, so the top sets full of holes, then toasted until the holes fill with butter. The batter has to be bubbly before it goes into the rings. A pikelet is the thinner version from Wales and the Midlands.",
    },
    Dish {
        names: &["syllabub"],
        longest: "syllabub",
        description: "Syllabub is cream whipped with white wine, lemon juice and sugar until it holds its shape, sometimes with the wine poured in from a height so it froths on top. It sets because the acid from the wine and lemon firms the cream. Serve it in glasses with a biscuit, cold and the same day.",
    },
    Dish {
        names: &["tiramisu"],
        longest: "tiramisu",
        description: "Tiramisu is savoiardi biscuits dipped in strong coffee and marsala, layered with mascarpone beaten with egg yolks and sugar, and dusted with cocoa. The coffee has to be cold and the dip quick, or the biscuits turn to mush. It needs at least four hours in the fridge, so make it the day before.",
    },
    Dish {
        names: &["tzatziki", "tsatsiki", "tatziki"],
        longest: "tzatziki",
        description: "Tzatziki is a Greek dip of thick yoghurt, grated cucumber, garlic, olive oil and a little vinegar or lemon, with dill or mint stirred through. Salt the grated cucumber and squeeze it dry first, or the dip turns watery within the hour. It goes with grilled meat, souvlaki and flatbread, and cold from the fridge it cools a hot plate down.",
    },
    Dish {
        names: &["swede", "swedes", "rutabaga", "neeps"],
        longest: "rutabaga",
        description: "Swede is a round root with purple skin and yellow flesh - a bigger, sweeter relative of the turnip, and called rutabaga in America and neeps in Scotland. It roasts in chunks around a joint, mashes with butter and pepper, or goes into a pasty. The smaller ones are sweeter, and it takes longer to cook than a potato.",
    },
    Dish {
        names: &["falafel", "felafel", "falafels"],
        longest: "falafels",
        description: "Falafel is soaked raw chickpeas ground with onion, garlic, herbs and cumin, shaped into balls and fried until dark and crisp - made from cooked chickpeas the mixture falls apart in the oil. The chickpeas need an overnight soak, not a simmer. Serve them in flatbread with salad, pickles and tahini.",
    },
    Dish {
        names: &["stovies", "stovie"],
        longest: "stovies",
        description: "Stovies is a Scottish one-pot of potatoes, onions and leftover roast meat cooked slowly in dripping or stock until the potatoes break down - the name comes from stoving, cooking slowly in a closed pot. It is made from what is left of a Sunday roast, so it is thrifty by design. Serve it with oatcakes and a pickled beetroot on the side.",
    },
    Dish {
        names: &["tagine", "tagines", "tajine"],
        longest: "tagines",
        description: "A tagine is a Moroccan stew named after the conical clay pot it is cooked in: meat, usually lamb or chicken, with spices, dried fruit and often preserved lemon or olives, cooked low and slow so it comes out sweet and savoury at once. A heavy pan with a lid does the same job in an oven at 160C. Couscous or flatbread to mop it up.",
    },
    Dish {
        names: &["brine", "brining"],
        longest: "brining",
        description: "Brine is salt water, about five per cent salt, that meat sits in before cooking so it takes up water and stays juicy. An hour for chops, four to twelve hours for a chicken or a turkey. Pat it dry before it goes in the pan or it will steam instead of browning.",
    },
    Dish {
        names: &["trifle", "trifles"],
        longest: "trifles",
        description: "A trifle is sponge or stale cake soaked with sherry or fruit juice, then fruit, then custard, then whipped cream, and the classic has raspberries and ratafia biscuits. Make the custard with egg yolks or cornflour rather than custard powder, or it tastes thin. Build it the day before so the sponge soaks.",
    },
    Dish {
        names: &["granita"],
        longest: "granita",
        description: "Granita is fruit juice or coffee with sugar and water frozen in a tray and forked through every half hour, so it sets into coarse ice crystals rather than a smooth cream. It needs no churning or special kit. More sugar means softer crystals, and alcohol keeps it from freezing hard at all.",
    },
    Dish {
        names: &["sorbet", "sorbets"],
        longest: "sorbets",
        description: "Sorbet is frozen fruit puree or juice with sugar syrup and nothing else - no dairy, which is what makes it different from ice cream. It needs more sugar than ice cream to stop it freezing solid, and a squeeze of lemon keeps the fruit tasting fresh. Frozen still and scraped with a fork it is a granita.",
    },
    Dish {
        names: &["pilaf", "pilau", "pilaff", "pilafs"],
        longest: "pilaff",
        description: "A pilaf is rice fried in a little fat with onion first, then cooked in measured stock with the lid on and left alone, so every grain stays separate - where a risotto is stirred and a plain rice is boiled and drained. One part rice to one and a half parts stock, twelve minutes, then ten minutes off the heat. Dried fruit, nuts or cumin go in at the frying stage.",
    },
    Dish {
        names: &["glaze", "glazes"],
        longest: "glazes",
        description: "A glaze is a thin coating brushed onto food so it cooks to a shine: reduced stock, honey and soy, jam loosened with water, or sugar and butter. Brush it on towards the end, because the sugar in it burns before the food is cooked through. Brush again every few minutes so it builds in layers.",
    },
    Dish {
        names: &["bagel", "bagels"],
        longest: "bagels",
        description: "A bagel is a bread ring boiled for a minute or two before it is baked, which sets the crust and gives it the chew. It is dense on purpose, so it takes a lot of filling without falling apart. New York bagels are boiled in malted water, which is where the shine and the colour come from.",
    },
    Dish {
        names: &["gravy"],
        longest: "gravy",
        description: "Gravy is the juices left in the roasting tin loosened with stock or wine, thickened a little with flour and simmered until it tastes of the meat. The flavour is in the browned bits stuck to the tin, so splash in hot stock and scrape them up. Stir in the juices from the rested joint at the end so they stay in the gravy.",
    },
];

fn opens(said: &str) -> bool {
    let said = said.trim_matches([' ', '.', '!', '?', ',']);
    let stripped = LEAD
        .iter()
        .filter(|lead| !lead.is_empty())
        .find_map(|lead| {
            let after = said.strip_prefix(lead)?;
            (after.is_empty() || after.starts_with(' ')).then(|| after.trim_start())
        })
        .unwrap_or(said);
    let head = stripped
        .split_whitespace()
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");
    ASKS.iter().any(|ask| head.starts_with(ask))
}

fn qualified_bare(dish: &Dish, said: &str) -> bool {
    let names = dish
        .names
        .iter()
        .copied()
        .filter(|name| said.contains(name));
    if names.clone().any(|name| !BARE.contains(&name)) {
        return false;
    }
    for name in names {
        let noun = name.split_whitespace().next_back().unwrap_or(name);
        let delim = format!(" {noun} ");
        let (head, tail) = said.split_once(&delim).unwrap_or((said, ""));
        if head.split_whitespace().next_back().is_some_and(|word| {
            !matches!(
                word,
                "a" | "an"
                    | "the"
                    | "and"
                    | "or"
                    | "this"
                    | "that"
                    | "my"
                    | "is"
                    | "are"
                    | "was"
                    | "were"
            )
        }) {
            return true;
        }
        if tail
            .split_whitespace()
            .next()
            .is_some_and(|word| !matches!(word, "for" | "is" | "are" | "and" | "or" | "with" | "s"))
        {
            return true;
        }
    }
    false
}

fn phrase(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(i, _)| {
        (i == 0 || text.as_bytes()[i - 1] == b' ')
            && (i + name.len() == text.len() || text.as_bytes()[i + name.len()] == b' ')
    })
}

pub(crate) fn definition(clean: &str) -> Option<String> {
    if !opens(clean) {
        return None;
    }
    let words = clean.trim_matches([' ', '.', '!', '?', ',']);
    if AVOID.iter().any(|word| phrase(words, word)) {
        return None;
    }
    let said = format!(" {clean} ");
    let mut rows = DISHES.iter().filter(|row| {
        row.names.iter().any(|name| phrase(&said, name)) && !qualified_bare(row, &said)
    });
    let first = rows.next()?;
    let other = rows.find(|row| !row.names.iter().any(|name| phrase(first.longest, name)));
    Some(other.map_or_else(
        || first.description.to_owned(),
        |row| {
            format!(
                "{} Or the other way round: {}",
                first.description, row.description
            )
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_dish_in_a_longer_name_does_not_win() {
        let pie = definition("what is a shepherd's pie").unwrap();
        assert!(!pie.contains("Or the other way round"));
        assert_eq!(definition("what is a pie dish?"), None);
        assert_eq!(definition("how do i make a yorkshire pudding?"), None);
    }
}
