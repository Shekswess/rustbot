//! Recipe rows from `bot/knowledge/meals.py` at `7d63c7fbebb44c7e466e5c8b5c1f341ab767929e`.

use super::{DISHES, Recipe};

#[rustfmt::skip]
pub(super) const RECIPES: &[Recipe] = &[
    Recipe { dish: &DISHES[2], uses: &["egg", "rice", "pea"], how: "fry cold cooked rice in hot oil for 5 minutes, push it to one side, scramble two eggs in the gap, then stir it all together with soy sauce and peas", gives_cook_time: true },
    Recipe { dish: &DISHES[7], uses: &["pasta", "tuna"], how: "stir a tin of tuna through hot pasta with olive oil, lemon and capers if you have them", gives_cook_time: false },
    Recipe { dish: &DISHES[14], uses: &["chickpea", "tomato", "onion"], how: "fry an onion with curry powder, add a tin of chickpeas and a tin of tomatoes and simmer for 15 minutes", gives_cook_time: true },
    Recipe { dish: &DISHES[15], uses: &["lentil", "onion", "garlic"], how: "simmer red lentils with onion, garlic and cumin in plenty of water for 25 minutes, until soft, then salt it", gives_cook_time: true },
    Recipe { dish: &DISHES[17], uses: &["mince", "bean", "tomato", "onion"], how: "brown the mince, add onion, a tin of beans and a tin of tomatoes with chilli and cumin, and simmer it 25 minutes", gives_cook_time: true },
    Recipe { dish: &DISHES[26], uses: &["tuna", "tomato", "cucumber"], how: "drain a tin of tuna into a bowl with chopped tomato and cucumber, dress it with oil, lemon and salt, and add a handful of leaves or cold pasta if you want it filling", gives_cook_time: false },
    Recipe { dish: &DISHES[27], uses: &["chickpea", "tomato", "cucumber", "lemon"], how: "toss a drained tin of chickpeas with chopped tomato and cucumber, dress it with lemon juice, olive oil and salt, and crumble cheese over it if you eat it", gives_cook_time: false },
    Recipe { dish: &DISHES[39], uses: &["lentil", "carrot", "onion"], how: "fry onion and carrot, add lentils and stock and simmer for 25 minutes", gives_cook_time: true },
    Recipe { dish: &DISHES[40], uses: &["pea", "onion"], how: "simmer frozen peas and a chopped onion in stock for 10 minutes with a handful of mint, then blend it", gives_cook_time: true },
    Recipe { dish: &DISHES[41], uses: &["spinach", "chickpea", "onion", "tomato"], how: "fry an onion with curry powder, add a tin of chickpeas and a tin of tomatoes and simmer 15 minutes, then stir the spinach in at the end until it wilts", gives_cook_time: true },
    Recipe { dish: &DISHES[44], uses: &["mince", "tomato", "onion", "pasta"], how: "brown the mince with a chopped onion, add a tin of tomatoes and a stock cube and simmer it 20 minutes while the spaghetti cooks", gives_cook_time: true },
    Recipe { dish: &DISHES[49], uses: &["mince", "cabbage", "onion"], how: "brown the mince with a chopped onion, add shredded cabbage and a splash of water and cook it 8 minutes with a lid on", gives_cook_time: true },
    Recipe { dish: &DISHES[53], uses: &["sweet potato", "chickpea", "tomato", "onion"], how: "fry onion with curry powder, add cubed sweet potato and a tin of tomatoes, simmer 20 minutes and stir in a tin of chickpeas", gives_cook_time: true },
    Recipe { dish: &DISHES[55], uses: &["squash", "chickpea", "onion"], how: "toss squash chunks, onion wedges and cumin with oil at 200C (180C fan) for 25 minutes, then stir in a tin of chickpeas and a squeeze of lemon", gives_cook_time: true },
];

#[rustfmt::skip]
pub(super) const STAPLES: &[&str] = &["bean", "bread", "butter", "carrot", "cheese", "egg", "flour", "garlic", "herb", "lemon", "milk", "noodle", "oil", "onion", "pasta", "pea", "pepper", "potato", "rice", "salt", "spice", "stock", "tomato", "tortilla"];
