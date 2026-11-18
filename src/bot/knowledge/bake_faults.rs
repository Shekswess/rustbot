//! Baking faults and fixes from bot/knowledge/bake_faults.py.
pub(crate) fn fault(words: &[String]) -> Option<&'static str> {
    let said = words.join(" ");
    let has = |wanted: &[&str]| {
        words
            .iter()
            .any(|word| wanted.iter().any(|want| word.starts_with(want)))
    };
    let says = |wanted: &[&str]| wanted.iter().any(|needle| said.contains(needle));
    if has(&["cheesecake", "cheesecakes"]) {
        if has(&["grainy", "gritty", "curdled"]) {
            return Some(
                "A grainy cheesecake has curdled: the oven was too hot, or it was beaten too long after the eggs went in. Bake it at 150C (130C fan) in a water bath, mix only until each thing is just in, and let it cool in the oven with the door ajar so it sets slowly. It is still good to eat - a grainy one is a texture, not a fault to throw away.",
            );
        }
        if has(&["watery", "soggy", "set"]) {
            return Some(
                "A watery cheesecake was underbaked or it went into the fridge too soon: it needs to set in the oven until only the middle wobbles, then cool in the tin an hour before it sees the cold. Bake it in a water bath at 150C (130C fan), because the steam is what makes it dense and creamy rather than grainy, and drain the tin well before it is chilled.",
            );
        }
        return Some(
            "A cheesecake cracks when it is baked too hot or cooled too fast: bake it at 150C (130C fan) in a water bath, and when only the middle still wobbles turn the oven off and leave it in there with the door ajar for an hour. A crack is cosmetic - a cold cheesecake and a spoon of sour cream over the top covers it - and the next one will not crack if it cools slowly.",
        );
    }
    if has(&["ganache"]) {
        return Some(
            "A split ganache got too hot, or cold cream went into warm chocolate: warm it gently over barely simmering water, off the heat, and stir in a tablespoon of warm milk or cream until it comes back together. Next time pour hot cream over chopped chocolate, wait a minute, then stir from the middle outwards, and keep every spoon and bowl dry.",
        );
    }
    if has(&["meringue", "meringues"]) {
        let weeping = "A meringue weeps when the sugar was not fully dissolved or it was underbaked: rub a little between your fingers and it should feel smooth, not gritty, before it goes in. Bake it low and slow, 100C (80C fan) for an hour and a half, then leave it in the oven with the door ajar to cool, and make it on a dry day.";
        if has(&[
            "weeping", "weeped", "weeps", "wept", "beading", "beaded", "beads", "gritty", "grainy",
        ]) {
            return Some(weeping);
        }
        if has(&[
            "soft",
            "softened",
            "soggy",
            "sticky",
            "chewy",
            "floppy",
            "limp",
            "collapsed",
        ]) {
            return Some(
                "A meringue goes soft when it has not dried out enough, or it was stored where the air is damp: bake it at 100C (80C fan) for an hour and a half and lift one off the paper to check it comes away clean, then cool it in the oven with the door ajar and keep it in an airtight tin - never the fridge, which is what makes it collapse. A sticky one wants 15 minutes more at the same heat, and a chewy one wants the sugar whisked in more slowly and a longer, slower bake.",
            );
        }
        return Some(weeping);
    }
    if has(&["macaron", "macarons"]) {
        if has(&["hollow"]) {
            return Some(
                "A hollow macaron was underbaked, or the whites were whipped too stiff and the shell set before the inside could come up: whip to soft to medium peaks, fold the batter until it ribbons and settles back in about 20 seconds, and bake at 150C (130C fan) for a minute or two longer, resting the shells only until a skin forms - much longer than that and they go hollow instead of getting feet.",
            );
        }
        if has(&[
            "soft",
            "softened",
            "softening",
            "soggy",
            "sticky",
            "chewy",
            "limp",
            "floppy",
            "collapsed",
            "sank",
        ]) {
            return Some(
                "A macaron that goes soft a day after it is filled is the filling's moisture working into the shell, which is what a macaron is meant to do: keep them in an airtight box and eat them within two or three days, and take them out of the fridge 20 minutes before you do, because cold makes a shell chewy. One that comes out of the oven soft was underbaked or the shells were not left long enough to form a skin: rest them 30 to 60 minutes until they are dry to the touch, then bake at 150C (130C fan) for 2 minutes longer than you did and check one at the back of the tray.",
            );
        }
        return Some(
            "Macarons crack when the oven is too hot or the piped shells were not rested: tap the tray, leave them 30 to 60 minutes until a skin forms, then bake at 150C (130C fan) with the door propped a crack. Fold the batter until it ribbons slowly off the spatula - overmixed and they spread, undermixed and they crack - and a hollow one usually wants 5C less and two minutes more.",
        );
    }
    if has(&["choux", "profiteroles", "profiterole", "eclairs", "eclair"]) {
        return Some(
            "Choux does not puff when the dough is too wet or the oven door is opened: dry the pan over the heat for a minute once the flour is in, beat in the eggs until it just falls off the spoon in a V, and keep the door shut for the first 20 minutes at 200C (180C fan). A wet dough steams instead of setting, and a cool oven lets it slide.",
        );
    }
    if has(&["brownie", "brownies"]) {
        return Some(
            "Brownies that will not set are underbaked: 20 to 25 minutes at 180C (160C fan), until a skewer comes out with a few moist crumbs and no wet batter, then cool them completely in the tin - an hour in the fridge if you are in a hurry - because they firm up as they cool. A runny middle is warm butter and sugar, so give them 5 more minutes rather than more flour.",
        );
    }
    if has(&["pastry", "tart", "tarts", "pie"]) || says(&["pie crust", "pie shell", "pastry case"])
    {
        if has(&[
            "soft", "softened", "soggy", "sticky", "greasy", "wet", "limp",
        ]) {
            return Some(
                "Soft pastry is too much fat or water, or the fat got warm: use about half the weight of butter to flour, cut it in cold until it looks like breadcrumbs, and add iced water a spoon at a time until it only just comes together. Handle it as little as you can, rest it 20 minutes in the fridge, and roll it on a floured surface rather than pressing it - warm butter is what makes it limp, so cool the bowl and your hands if the kitchen is hot.",
            );
        }
        if has(&["crack", "cracked", "cracks"]) && !has(&["shrank", "shrunk", "shrinking"]) {
            return None;
        }
        return Some(
            "Pastry shrinks when the gluten is worked too much or it is stretched into the tin: rest it 20 minutes in the fridge before rolling, ease it in rather than pulling it, and let it hang over the edge instead of pressing it up the sides. Chill the lined tin 15 minutes before it bakes and it will hold its shape.",
        );
    }
    if has(&["cake mixture", "cake batter", "sponge mixture"])
        || says(&["cake mixture", "cake batter"])
    {
        return Some(
            "A curdled cake mixture is cold eggs going into warm butter too fast, and it comes back easily: beat in a tablespoon of the weighed flour, and stand the bowl in warm water for a minute. Next time use eggs at room temperature and add them a little at a time, beating well after each one, and it will not split.",
        );
    }
    None
}
const FAMILIES: &[(&[&str], &str)] = &[
    (
        &[
            "dry",
            "dried out",
            "overcooked",
            "over-cooked",
            "hard",
            "tough",
        ],
        "It is nearly always the time in the oven, or flour scooped rather than weighed. Next time: take it out 5 minutes before the recipe says, weigh the flour instead of scooping it, and brush the top with milk or syrup while it is still warm.",
    ),
    (
        &[
            "crumbly",
            "crumbs",
            "crumbles",
            "falling apart",
            "fell apart",
            "sandy",
            "dry and crumbly",
        ],
        "Too little fat or egg to hold it together, or it was cut while it was still warm. Next time: weigh the butter and sugar, add an extra yolk, and leave it 10 minutes in the tin before turning it out.",
    ),
    (
        &["sank", "sunk", "sunken", "sinking", "sink", "collapsed"],
        "It is underbaked, or the oven door was opened in the first 20 minutes, or there is too much raising agent in it, and all three make a bake rise fast and then drop. Next time: bake it at 180C (160C fan) without opening the door, fill the tin two-thirds full, and check it with a skewer before it comes out.",
    ),
    (
        &["flat", "didnt rise", "didn't rise", "not risen", "no rise"],
        "The baking powder is past its date, the flour was beaten in too hard, or the oven was not up to heat. Next time: use a teaspoon of baking powder to 150g of plain flour, fold the flour in with a spoon, and let the oven come fully up to heat first.",
    ),
    (
        &[
            "gummy",
            "clammy",
            "wet in the middle",
            "soggy in the middle",
            "stodgy",
            "doughy",
            "raw in the middle",
            "soggy",
            "wet inside",
            "squidgy",
            "not cooked in the middle",
        ],
        "It needed longer, not a hotter oven: the outside set while the middle was still wet, and a mixture weighed by eye goes in wetter than the recipe meant. Next time: give it 5 to 10 more minutes at the same heat with foil over the top, weigh the milk, butter and flour, and use a skewer in the centre not at the edge.",
    ),
    (
        &[
            "pale",
            "palest",
            "no colour",
            "white on top",
            "pale on top",
            "blond",
            "not brown",
        ],
        "It is short of heat or short of sugar: a pale bake was on too low a shelf or the oven was under its temperature, and a recipe cut down on sugar browns slowly. Next time: bake it one shelf up at the heat the recipe says, and check the oven with a thermometer - an oven that reads 160C when it says 180C gives a pale bake every time.",
    ),
    (
        &[
            "bicarbonate",
            "bicarb",
            "soda",
            "soapy",
            "tastes of soap",
            "tastes of baking soda",
            "tastes of bicarbonate",
            "chemical taste",
            "metallic",
        ],
        "There is too much bicarbonate in it, or nothing acid for it to react with, and both leave that soapy taste behind. Next time: use a level teaspoon to 150g of plain flour, keep it for recipes with buttermilk, yoghurt, lemon or cocoa in them, and use baking powder when nothing in the mix is acid.",
    ),
    (
        &["stuck", "sticking", "sticks", "broke", "broken"],
        "The tin was not greased all over, or it was turned out while it was still hot. Next time: grease the tin and cut a strip of baking paper across the base and up two sides, then run a knife round the edge and wait 10 minutes before turning it out.",
    ),
];

pub(crate) fn recurring(text: &str, words: &[String]) -> Option<&'static str> {
    let both = format!("{} {}", words.join(" "), text.to_lowercase());
    FAMILIES.iter().find_map(|(words, answer)| {
        words
            .iter()
            .any(|needle| both.contains(needle))
            .then_some(*answer)
    })
}
