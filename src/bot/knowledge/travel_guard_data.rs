//! Authored trip and household word patterns from PyBot's travel rules.

pub(super) const THE_TRIP_IN_THE_LINE: &str = r#"(?i)\b(?:holiday|holidays|holidaymaker|trip|trips|travel|travelling|traveling|flight|flights|fly|flying|hotel|hotels|abroad|away|resort|resorts|villa|cruise|airport|city\s+break|beach|abroad|all\s+inclusive|package|packages|tour|tours|itinerary|hotel\s+and\s+flight|flights\s+and\s+hotel|transfer|transfers|operator|operators|atol|booking)\b"#;
pub(super) const TRAINS_FOR: &str = r#"\b(to|need to|have to|got to|going to|how long to|weeks to|time to|how to|back to)\s+train\b|\btrain(?:ing|ed)?\s+(?:for|hard|harder|again|towards|up for)\b|\btrain(?:ing|ed)\s+(?:twice|three|four|every|each|most|more)\b"#;
pub(super) const FOOD_TO_CARRY: &[&str] = &[
    "food",
    "foods",
    "snacks",
    "snack",
    "lunch",
    "dinner",
    "breakfast",
    "meal",
    "meals",
    "picnic",
    "dish",
    "dishes",
    "something to eat",
    "anything to eat",
    "cook",
    "cooking",
    "bake",
    "baking",
    "recipe",
    "recipes",
    "drinks",
];
pub(super) const A_WALK_IS_SAID: &str = r#"(?i)\b(walk|walks|walking|walked|stroll|strolls|strolling|path|paths|route|routes|rambl\w*|ramble|ramble)\b"#;
pub(super) const FLAT_YOU_LIVE_IN: &str = r#"(?i)\b(?:my|the|our|this|that|their|his|her|a|one)\s+(?:tiny\s+|small\s+|little\s+|new\s+|old\s+|top\s+floor\s+|ground\s+floor\s+|second\s+floor\s+|studio\s+)?flat\b|\b(?:move|moves|moved|moving|rent|rents|rented|renting|let|letting|buy|buys|bought|buying|sell|sells|selling|sold|view|viewing|sort|sorting|sorted|decorate|decorating|refurbish|refurbishing)\s+(?:(?:a|an|the|my|our|this|that|his|her)\s+)?(?:tiny\s+|small\s+|little\s+|new\s+|old\s+)?flat\b|\b(?:heat|heating|clean|cleaning)\s+(?:a|an|the|my|our|this|that)\s+(?:tiny\s+|small\s+|little\s+)?flat\b|\bflats\b|\bflat\s?mates?\b|\bbedsit\b"#;
pub(super) const A_BAKE_OR_A_PAN: &str = r#"(?i)\b(?:bread|breads|loaf|loaves|cake|cakes|sponge|scones?|pastry|pastries|tarts?|buns?|rolls?|pizza|pizzas|pancakes?|omelettes?|omelets?|yorkshires?|puddings?|meringues?|souffles?|biscuits?|cookies?|brownies?|crumpets?|dough|naan|pittas?|pita|ciabatta|focaccia|baguette|batter|sourdough|flatbread|chapatis?|tortillas?|waffles?|loaf tin)\b"#;
pub(super) const CAME_OUT_FLAT: &str = r#"(?i)\b(?:came|come|comes|coming|turned|turns|turning|went|gone|goes|going)\s+(?:out\s+)?(?:a\s+bit\s+|a\s+little\s+|really\s+|so\s+|rather\s+|too\s+)?flat\b"#;
pub(super) const THE_GROUND_UNDERFOOT: &str = r#"(?i)\b(?:walk|walks|walking|walked|stroll|strolls|hike|hikes|route|routes|path|paths|towpath|trail|trails|terrain|ground|fields?|park|parks|promenade|seafront|beach|miles?|kilometres?|outing|day out|somewhere to go|wheelchair|buggy|pram|pushchair)\b"#;
