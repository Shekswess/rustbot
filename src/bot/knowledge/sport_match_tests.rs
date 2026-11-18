//! Match and equipment checks captured from PyBot 7d63c7f.
use super::*;

type Case = (Reply, &'static str, Option<&'static str>, Option<bool>);

#[test]
fn sport_match_callbacks_match_the_reference() {
    let cases: &[Case] = &[
        (
            Reply::Ground,
            "How big is a football pitch?",
            Some(FOOTBALL),
            None,
        ),
        (
            Reply::Ground,
            "What are the dimensions of a soccer pitch?",
            Some(FOOTBALL),
            None,
        ),
        (
            Reply::Ground,
            "How wide are tennis courts?",
            Some(TENNIS),
            None,
        ),
        (
            Reply::Ground,
            "How long is a tennis court?",
            Some(TENNIS),
            None,
        ),
        (
            Reply::Ground,
            "What is the size of the cricket square?",
            Some(CRICKET),
            None,
        ),
        (
            Reply::Ground,
            "How long is the cricket pitch?",
            Some(CRICKET),
            None,
        ),
        (
            Reply::Ground,
            "What are the netball court dimensions?",
            Some(NETBALL),
            None,
        ),
        (Reply::Ground, "How wide is a badminton court?", None, None),
        (Reply::Ground, "How big is the ground?", None, None),
        (
            Reply::Ground,
            "How far is a football ground from here?",
            Some(FOOTBALL),
            None,
        ),
        (
            Reply::Ground,
            "Which is bigger, a tennis court or a football pitch?",
            Some(FOOTBALL),
            None,
        ),
        (
            Reply::Ground,
            "How big is a FOOTBALL  PITCH?",
            Some(FOOTBALL),
            None,
        ),
        (Reply::Ground, "", Some(FOOTBALL), None),
        (
            Reply::Competition,
            "What's the difference between a league and a cup?",
            Some(LEAGUE_CUP),
            None,
        ),
        (
            Reply::Competition,
            "How does the league work?",
            Some(LEAGUE_CUP),
            None,
        ),
        (
            Reply::Competition,
            "How does the Champions League work?",
            Some(LEAGUE_CUP),
            None,
        ),
        (
            Reply::Competition,
            "What is the difference between the Premier League and the Champions League?",
            None,
            None,
        ),
        (
            Reply::Competition,
            "What's the difference between the FA Cup and the League Cup?",
            None,
            None,
        ),
        (Reply::Competition, "How does the FA Cup work?", None, None),
        (
            Reply::Competition,
            "How does the World Cup work?",
            None,
            None,
        ),
        (
            Reply::Competition,
            "How is the Premier League different from the FA Cup?",
            None,
            None,
        ),
        (
            Reply::Competition,
            "What is a league table?",
            Some(LEAGUE_CUP),
            None,
        ),
        (
            Reply::Competition,
            "What is the difference between a cup and a league table?",
            Some(LEAGUE_CUP),
            None,
        ),
        (
            Reply::Competition,
            "How does the Conference League compare to La Liga?",
            None,
            None,
        ),
        (
            Reply::Competition,
            "Is the playoff final a league or cup?",
            None,
            None,
        ),
        (
            Reply::Competition,
            "What does league mean?",
            Some(LEAGUE_CUP),
            None,
        ),
        (Reply::Competition, "", Some(LEAGUE_CUP), None),
        (
            Reply::WetKit,
            "What should I wear to a football match in the rain?",
            Some(WATCHING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "What should I wear to play football in the rain?",
            Some(PLAYING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "I'm playing football in wet weather, what should I wear?",
            Some(PLAYING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "What should I wear to watch my son play football in the rain?",
            Some(WATCHING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "My son has a rugby match in the cold, what should he wear?",
            Some(PLAYING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "My husband plays rugby. What should I wear to watch in the cold?",
            Some(WATCHING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "What should I take to my son's rugby match? It's freezing.",
            Some(WATCHING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "I'm in the team but will watch the second half. What should I wear in the rain?",
            Some(PLAYING),
            Some(false),
        ),
        (
            Reply::WetKit,
            "What should I pack for hockey if it's windy?",
            Some(WATCHING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "What should I wear to a match?",
            Some(WATCHING),
            Some(false),
        ),
        (
            Reply::WetKit,
            "It is cold. What should I wear?",
            Some(WATCHING),
            Some(false),
        ),
        (
            Reply::WetKit,
            "There's rain at the football match.",
            Some(WATCHING),
            Some(false),
        ),
        (
            Reply::WetKit,
            "Which boots should I buy for muddy football?",
            Some(WATCHING),
            Some(false),
        ),
        (
            Reply::WetKit,
            "How do I dry my football kit after a match in the rain?",
            Some(WATCHING),
            Some(false),
        ),
        (
            Reply::WetKit,
            "What should I wear at a football match in the rain to stay dry?",
            Some(WATCHING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "I don't wear a raincoat at rugby practice. What should I bring?",
            Some(WATCHING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "Did I make a mistake at the football match in the rain?",
            Some(WATCHING),
            Some(false),
        ),
        (
            Reply::WetKit,
            "I swear it was cold at the match.",
            Some(WATCHING),
            Some(false),
        ),
        (
            Reply::WetKit,
            "What can I wear to a windy match without spending much?",
            Some(WATCHING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "What should I wear when playing netball in drizzle?",
            Some(PLAYING),
            Some(true),
        ),
        (
            Reply::WetKit,
            "The football is freezing. What do I wear on the touchline?",
            Some(WATCHING),
            Some(true),
        ),
    ];
    for &(form, line, expected, guard) in cases {
        let reading = Reading::new(line);
        assert_eq!(
            reply(form, &reading).as_deref(),
            expected,
            "{form:?} for {line:?}"
        );
        if let Some(expected) = guard {
            assert_eq!(wet_match(&reading), expected, "{line:?}");
        }
    }
}

#[test]
fn sport_football_boots_generated_row_matches_the_reference() {
    use super::super::{guards::Context, rows};
    let card = rows()
        .find(|card| card.domain == "sport" && card.id == "football-boots")
        .expect("registered football-boots card");
    for (line, expected, allowed) in [
        ("What boots should I wear on grass?", DEFAULT_BOOTS, true),
        ("What studs do I need for 3G?", DEFAULT_BOOTS, true),
        (
            "I need football boots for indoor futsal",
            DEFAULT_BOOTS,
            true,
        ),
        ("Which boots grip well on soft ground?", DEFAULT_BOOTS, true),
        ("When should I buy football boots?", WHEN_TO_BUY, true),
        (
            "Which month is best to buy football boots?",
            WHEN_TO_BUY,
            true,
        ),
        ("When are cheap football boots on sale?", WHEN_TO_BUY, true),
        ("Where can I find cheap football boots?", WHERE_TO_BUY, true),
        (
            "Any football boots for my son's budget?",
            WHERE_TO_BUY,
            true,
        ),
        ("My football boots are too tight", FIT, true),
        ("What football boot size for a 10 year old?", FIT, true),
        ("How do I break in new boots?", BREAK_IN, true),
        ("How do I break in new boots that are too tight?", FIT, true),
        ("My boots have moulded studs", DEFAULT_BOOTS, true),
        ("My football boots rub and are too narrow", FIT, true),
        ("How do I remember what I study?", DEFAULT_BOOTS, false),
        ("Which studs should a student buy?", DEFAULT_BOOTS, false),
        ("When do I revise for mock exams?", WHEN_TO_BUY, false),
        ("Should I order boots online?", WHERE_TO_BUY, true),
        ("Football boots for which surface?", DEFAULT_BOOTS, true),
    ] {
        let reading = Reading::new(line);
        let context = Context::new(&reading, &[]);
        assert_eq!(card.answer(&context).as_deref(), Some(expected), "{line:?}");
        assert_eq!(
            card.guard.passes(&context, card.id, card.key[0]),
            allowed,
            "{line:?}"
        );
    }
}

#[test]
fn sport_bookings_require_a_sports_word() {
    for (line, expected) in [
        ("What does it mean when he gets booked?", false),
        ("Why did the ref book him?", true),
        ("What does booked mean in football?", true),
        ("How long does a red card ban last?", true),
        ("How many bookings does the restaurant have tonight?", true),
        ("How do I book a hotel?", false),
        ("I've booked a room", true),
        ("Is the book better than the film?", false),
        ("What book should I read?", false),
        ("How do I book a football ticket?", true),
        ("What does booking mean?", true),
        ("How do players get booked?", true),
        ("What is a sin bin?", false),
        ("I booked a pitch for the match", true),
        ("I booked a room for the rugby match", true),
        ("What does red mean?", true),
        ("Red?", false),
    ] {
        assert_eq!(booking_card(&Reading::new(line)), expected, "{line:?}");
    }
}

const FOOTBALL: &str = "A full-size football pitch is about 105 metres by 68 metres, which is 115 by 74 yards and a bit under two acres; the laws allow 90 to 120 metres long and 45 to 90 wide, which is why pitches differ. The goal is 7.32 metres wide and 2.44 metres high.";

const TENNIS: &str = "A tennis court is 23.77 metres long, 10.97 metres wide for doubles and 8.23 metres for singles, with room to run of 6.4 metres behind each baseline and 3.66 metres at the sides. The net is 1.07 metres at the centre and 0.91 at the posts.";

const CRICKET: &str = "A cricket pitch is 20.12 metres — 22 yards — from stump to stump and 3.05 metres wide, with the popping crease 1.22 metres in front of the stumps. The boundary round it is a different size at every ground, from about 60 metres to 75.";

const NETBALL: &str = "A netball court is 30.5 metres long and 15.25 metres wide, split into thirds, with a shooting circle of 4.9 metres round each goalpost.";

const LEAGUE_CUP: &str = "A league is decided over a season: every team plays the others home and away, and the table decides who wins, so one bad afternoon only costs you three points. A cup is knockout: you play once, and losing means you are out, which is why a lower team can beat a big one on the day. That is how the same club can be top of the league and out of the cup in the same week.";

const DEFAULT_BOOTS: &str = "The surface picks the boot: moulded studs (FG) on grass, screw-in studs (SG) when it is wet and soft, astro or turf trainers (TF) on 3G and 4G, flat indoor shoes for a hall. Astro boots on wet grass give almost no grip, which is how ankles turn. Try them on with match socks and leave a thumb's width at the toe.";

const WHEN_TO_BUY: &str = "Boots are cheapest at the end of a season, in April to June, and again when a new colourway comes out and last year's drop; for a child the club's kit sale just before the season starts is the moment, because that is when the out-grown pairs come in. The surface picks the boot: moulded studs (FG) on grass, screw-in studs (SG) when it is wet and soft, astro or turf trainers (TF) on 3G and 4G, flat indoor shoes for a hall. ";

const WHERE_TO_BUY: &str = "Buy them anywhere that lets you try them on: a sports shop, a supermarket's own brand, the big online retailers, or a club's second-hand kit sale, where out-grown pairs go for a few pounds. Do not spend much on a child's boots - feet grow a size a season - but do buy the right sole for the pitch, because a cheap boot in the wrong stud is worse than no stud at all. The surface picks the boot: moulded studs (FG) on grass, screw-in studs (SG) when it is wet and soft, astro or turf trainers (TF) on 3G and 4G, flat indoor shoes for a hall. Go by fit: try them on in the afternoon with match socks on, leave a thumb's width at the toe, and go half a size up for a child whose feet are still growing. ";

const FIT: &str = "Go by fit: try them on in the afternoon with match socks on, leave a thumb's width at the toe, and go half a size up for a child whose feet are still growing. The surface picks the boot: moulded studs (FG) on grass, screw-in studs (SG) when it is wet and soft, astro or turf trainers (TF) on 3G and 4G, flat indoor shoes for a hall. Astro boots on wet grass give almost no grip, which is how ankles turn.";

const BREAK_IN: &str = "Wear a new pair around the house an evening at a time and never in a match first; a leather boot softens within a week. The surface picks the boot: moulded studs (FG) on grass, screw-in studs (SG) when it is wet and soft, astro or turf trainers (TF) on 3G and 4G, flat indoor shoes for a hall. Astro boots on wet grass give almost no grip, which is how ankles turn.";

const WATCHING: &str = "A waterproof over everything and something to keep your legs dry: a coat with a hood, boots or wellies, and a dry top waiting in the car. Two hours standing in wind and rain is colder than the forecast sounds, so a hat and gloves even in spring.";

const PLAYING: &str = "Layers you can take off: a base layer and shorts under a thin waterproof top, gloves once it is under about 8C, and boots to suit the surface - moulded studs on grass, astro trainers on 3G, long studs only on a soft winter pitch. A towel and a dry top in the bag for after, because you cool fast once you stop.";
