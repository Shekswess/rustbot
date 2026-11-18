//! Direct source fixtures captured from PyBot 7d63c7f.
use super::*;
use crate::bot::understanding::context::History;

type Case = (
    Reply,
    &'static [&'static str],
    &'static str,
    Option<&'static str>,
    Option<bool>,
);

#[test]
fn clothing_care_callbacks_match_the_reference() {
    let cases: &[Case] = &[
        (
            Reply::NoIron,
            &[],
            "How do I iron a shirt without an iron?",
            Some(NO_IRON),
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "I don't have an iron for this shirt",
            Some(NO_IRON),
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "Ironing a blouse and my iron is broken",
            Some(NO_IRON),
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "I havent got an iron for my trousers",
            Some(NO_IRON),
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "How can I iron my shirt with no ironing board?",
            Some(NO_BOARD),
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "I dont have a board for my iron",
            Some(NO_BOARD),
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "Iron my blouse without the ironing board",
            Some(NO_BOARD),
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "I have no iron or ironing board",
            Some(NO_IRON),
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "How do I iron my new dress?",
            None,
            None,
        ),
        (
            Reply::NoIron,
            &[],
            "Can I press this shirt on my ironing board?",
            None,
            None,
        ),
        (Reply::NoIron, &[], "How do I wash my shirt?", None, None),
        (
            Reply::Dry,
            &[],
            "How do I dry my laundry indoors?",
            Some(RACK),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry this down jacket?",
            Some(DOWN),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry a puffer coat?",
            Some(DOWN),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry a wool coat?",
            Some(HANGER),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry a waterproof jacket?",
            Some(HANGER),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry a wool jumper?",
            Some(FLAT),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry my merino cardigan?",
            Some(FLAT),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry my towels?",
            Some(FLUFFY),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry a duvet?",
            Some(FLUFFY),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry my fleece indoors?",
            Some(HANGER),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry my coat down?",
            Some(DOWN),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "How can I dry my ordinary washing without a dryer?",
            Some(RACK),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "My tumble dryer leaves towels damp",
            Some(FLUFFY),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "My tumble dryer wont dry the clothes",
            Some(RACK),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "My dryer hasnt dried the washing",
            Some(RACK),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "My hair dryer doesn't dry my hair",
            Some(RACK),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "The house is damp inside",
            Some(RACK),
            Some(false),
        ),
        (
            Reply::Dry,
            &[],
            "My clothes are damp in the bathroom",
            Some(RACK),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "Why won't my raised bed soil dry?",
            Some(RACK),
            Some(false),
        ),
        (
            Reply::Dry,
            &[],
            "My bedroom is damp",
            Some(RACK),
            Some(false),
        ),
        (
            Reply::Dry,
            &[],
            "How do I dry a wool jumper in my room?",
            Some(FLAT),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "The washing is drying in the kitchen",
            Some(RACK),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "Can I dry this indoors?",
            Some(RACK),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "The wet walls and towels won't dry",
            Some(FLUFFY),
            Some(true),
        ),
        (
            Reply::WashTemperature,
            &[],
            "Is 90 degrees too hot for a wash?",
            Some(HOT),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "Can I wash towels at 95C?",
            Some(HOT),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "Is washing at 60 too hot?",
            Some(SIXTY),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "What temperature should I wash bed sheets at?",
            Some(BEDDING),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "What temperature for pillow cases?",
            Some(BEDDING),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "What temperature for towels?",
            Some(BEDDING),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "Is 40 enough for my towels?",
            Some(EVERYDAY),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "Can I wash sheets at 30?",
            Some(EVERYDAY),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "What's the everyday wash setting for shirts?",
            Some(EVERYDAY),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "Wash clothes at 60 or 90 degrees?",
            Some(HOT),
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "Does my heated towel rail need a bleed?",
            None,
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "Should I set the radiator for my towels to 60C?",
            None,
            None,
        ),
        (
            Reply::WashTemperature,
            &[],
            "I have 160 towels, what wash temperature?",
            Some(SIXTY),
            None,
        ),
        (
            Reply::WashTemperature,
            &["The washing machine smells of damp."],
            "I've tried a hot wash already.",
            None,
            None,
        ),
        (
            Reply::WashTemperature,
            &[
                "The pipes leaked last winter.",
                "I'm changing the subject.",
                "We ought to tidy up.",
                "Unrelated one now.",
            ],
            "What temperature should I wash the towels at?",
            None,
            None,
        ),
        (
            Reply::WashTemperature,
            &["I'm buying a coat."],
            "What temperature should I wash my jeans at?",
            Some(EVERYDAY),
            None,
        ),
        (
            Reply::WashTemperature,
            &["My hair dryer broke."],
            "Should I wash this at 60 degrees?",
            None,
            None,
        ),
        (
            Reply::WashTemperature,
            &["I'm moving soon."],
            "What temperature for bed sheets?",
            Some(BEDDING),
            None,
        ),
        (
            Reply::Dry,
            &[],
            "My tumble dryer isn't drying my clothes",
            Some(DRYER_COLD),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "The dryer leaves everything damp",
            Some(DRYER_SLOW),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "My dryer takes ages",
            Some(DRYER_SLOW),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "The tumble dryer isn't drying my down coat",
            Some(DRYER_COLD),
            Some(true),
        ),
        (
            Reply::Dry,
            &[],
            "My hairdryer isn't drying my hair",
            Some(RACK),
            Some(true),
        ),
    ];
    for &(kind, before, line, expected, guard) in cases {
        let mut runtime_history = History::default();
        let mut prior: Vec<Exchange> = Vec::new();
        for user in before {
            let mut reading = Reading::in_context(
                user,
                prior.iter().map(|turn| turn.user.as_str()),
                None,
                &runtime_history,
            );
            runtime_history.retain(&mut reading);
            prior.push(Exchange {
                user: (*user).into(),
                bot: "All right.".into(),
                module: "greeting",
            });
        }
        let reading = Reading::in_context(
            line,
            prior.iter().map(|turn| turn.user.as_str()),
            None,
            &runtime_history,
        );
        let actual = reply(kind, &reading, &prior);
        assert_eq!(
            actual.as_deref(),
            expected,
            "{kind:?} for {line:?} after {before:?}"
        );
        if let Some(expected) = guard {
            assert_eq!(drying_laundry(&reading), expected, "{line:?}");
        }
    }
}

#[test]
fn faulty_coat_guard_matches_the_reference() {
    for (line, expected) in [
        ("Is a waterproof jacket worth the money?", false),
        ("Which waterproof coat should I buy?", false),
        ("My jacket leaks in the rain", true),
        ("My coat got soaked, is it worth waterproofing it?", true),
        ("How can I reproof my Gore-Tex jacket?", true),
        ("Can I waterproof the seams on my new raincoat?", true),
        ("I have a coat", true),
        ("What kind of jacket is best for hiking?", false),
        ("My jacket is letting rain in and my zip broke", true),
        ("My jacket has stopped beading", true),
    ] {
        let read = Reading::new(line);
        assert_eq!(faulty_coat(&read), expected, "{line:?}");
    }
}

const NO_IRON: &str = "No iron needed: hang the shirt in the bathroom while you take a hot shower, or spritz it with water, smooth it flat with your hands and hang it somewhere warm. Ten minutes in a tumble dryer with a damp towel drops the creases too, and a damp collar pressed flat with your thumb and left to dry hangs right.";

const NO_BOARD: &str = "A folded bath towel on a table is the board: lay the shirt flat on it, damp, and press with the iron, working in small sections and keeping the towel under the whole of the front. A clean cotton sheet over the table works too, and the collar and cuffs want pressing on the reverse first so they keep their shape.";

const RACK: &str = "Hang the washing on a rack in the warmest room with a window cracked open, spaced out so air moves round it, and turn thicker things after a few hours. A dehumidifier dries a full load overnight indoors and stops the damp smell.";

const DOWN: &str = "Tumble a down jacket on low with two or three dryer balls and stop every twenty minutes to pull the clumps apart, until it is completely dry - hanging it wet leaves the down in lumps that never loft again. If it is dirty, wash it first on its own with a down wash and no fabric conditioner.";

const HANGER: &str = "Hang it on a wide hanger somewhere airy with the pockets empty and the hood out, and let it drip dry. Keep a waterproof away from a radiator and a tumble dryer, because the heat softens the coating that keeps the rain out.";

const FLAT: &str = "Dry wool flat, never on a hanger: press the water out in a towel, reshape it on a dry towel or a mesh rack and leave it somewhere airy, away from a radiator. Hanging it wet is what stretches a jumper out of shape.";

const FLUFFY: &str = "Shake it out, then hang it with space on both sides - a towel or a duvet bunched up stays damp in the middle. Finish it in a tumble dryer on low if you have one, and do not fold it away until it is dry to the touch, or it comes out musty.";

const HOT: &str = "90 is only for illness, nappies and bedding that has been through a bug: it shrinks cotton and fades colour, so nothing you want to keep should go in it. 60 does towels, bedding, underwear, socks and anything greasy or stained, and 30 or 40 covers shirts and jeans worn once.";

const SIXTY: &str = "60 is not too hot for towels, bedding, underwear, socks and anything greasy or stained - it is what those need, along with a full dose of detergent. Keep shirts and jeans worn once at 30 or 40, and save 90 for illness and nappies: heat is what shrinks and fades things.";

const BEDDING: &str = "60C for bedding, towels, underwear and socks: that is the heat that kills dust mites and the bacteria that make a towel smell, along with a full dose of detergent, and cotton bedding takes it well. Keep shirts, jeans and anything worn once at 30 or 40, and save 90 for illness and nappies.";

const EVERYDAY: &str = "Treat 30C as the everyday setting: it is about 10p a load cheaper than 40C and gets shirts, jeans and normally-worn clothes just as clean, as long as the drum is not crammed and you use the full dose of detergent. Save 40 for heavy soiling, 60 for towels, bedding, underwear, socks and anything greasy or stained, and 90 only for illness or nappies.";

const DRYER_COLD: &str = "Nearly always the air or the setting, not a part: check the dial first, since most machines finish on a cool-down phase that heats nothing, then empty the fluff filter and check the vent hose is not kinked. If the drum turns and the air still comes out cold, it is the thermal fuse on the element - an engineer's job, cheap part.";

const DRYER_SLOW: &str = "Slow drying is nearly always the air: empty the fluff filter, rinse the condenser if it has one, and check the vent hose is not kinked. A drum stuffed full will not dry either, so try half a load. If the air still comes out cold, it is the fuse on the element - an engineer's job, cheap part.";
