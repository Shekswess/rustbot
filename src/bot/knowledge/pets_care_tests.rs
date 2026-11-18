//! Direct callback fixtures captured from PyBot 7d63c7f.
use super::*;
use crate::bot::understanding::context::History;

type Case = (
    Option<Reply>,
    &'static [&'static str],
    &'static str,
    Option<&'static str>,
    Option<bool>,
);

#[test]
fn pets_care_callbacks_match_the_reference() {
    let cases: &[Case] = &[
        (
            Some(Reply::KittenKit),
            &[],
            "What should I buy for my new kitten?",
            Some(KITTEN_SUPPLIES),
            None,
        ),
        (
            Some(Reply::KittenKit),
            &[],
            "What should I buy for my new cat?",
            Some(CAT_SUPPLIES),
            None,
        ),
        (
            Some(Reply::KittenKit),
            &[],
            "Should I get a cat?",
            None,
            None,
        ),
        (
            Some(Reply::KittenKit),
            &[],
            "I am thinking of getting a cat",
            None,
            None,
        ),
        (
            Some(Reply::KittenKit),
            &[],
            "I am thinking of getting two kittens",
            Some(KITTEN_SUPPLIES),
            None,
        ),
        (
            Some(Reply::KittenKit),
            &[],
            "Thinking about getting a cat, what supplies do I need?",
            Some(CAT_SUPPLIES),
            None,
        ),
        (
            Some(Reply::Rescue),
            &[],
            "Should I get a rescue cat or kitten?",
            Some(RESCUE_CAT),
            None,
        ),
        (
            Some(Reply::Rescue),
            &[],
            "Should I get a rescue dog or puppy?",
            Some(RESCUE_DOG),
            None,
        ),
        (
            Some(Reply::Rescue),
            &[],
            "An adult kitty or a new baby?",
            Some(RESCUE_CAT),
            None,
        ),
        (
            Some(Reply::Rescue),
            &[],
            "Should I get a kitten or a puppy?",
            Some(RESCUE_CAT),
            None,
        ),
        (
            Some(Reply::Bed),
            &[],
            "Where should my puppy sleep?",
            Some(DOG_BED),
            None,
        ),
        (
            Some(Reply::Bed),
            &[],
            "Where should my cat sleep?",
            Some(CAT_BED),
            None,
        ),
        (
            Some(Reply::Bed),
            &[],
            "Where should my beagle sleep?",
            Some(DOG_BED),
            None,
        ),
        (
            Some(Reply::Bed),
            &[],
            "Where should my ragdoll sleep?",
            Some(CAT_BED),
            None,
        ),
        (
            Some(Reply::Bed),
            &[],
            "Where should my dog and cat sleep?",
            Some(BOTH_BEDS),
            None,
        ),
        (
            Some(Reply::Bed),
            &[],
            "Where should it sleep?",
            Some(BOTH_BEDS),
            None,
        ),
        (
            Some(Reply::Bed),
            &["We're getting a dog next week."],
            "Where should it sleep?",
            Some(DOG_BED),
            None,
        ),
        (
            Some(Reply::Bed),
            &["We're getting a cat next week.", "Thanks", "Hello"],
            "Where should it sleep?",
            Some(CAT_BED),
            None,
        ),
        (
            Some(Reply::Bed),
            &["We're getting a dog next week.", "We also have a cat."],
            "Where should it sleep?",
            Some(DOG_BED),
            None,
        ),
        (
            Some(Reply::Bed),
            &[
                "We're getting a dog next week.",
                "My cat is getting a new cat flap.",
            ],
            "Where should it sleep?",
            Some(BOTH_BEDS),
            None,
        ),
        (
            Some(Reply::Bed),
            &["We're getting a cat next week."],
            "Where should my puppy sleep?",
            Some(DOG_BED),
            None,
        ),
        (
            Some(Reply::Litter),
            &[],
            "How often do I change the cat's litter tray?",
            Some(CAT_TRAY),
            None,
        ),
        (
            Some(Reply::Litter),
            &[],
            "How often do I clean the bunny's litter tray?",
            Some(RABBIT_TRAY),
            None,
        ),
        (
            Some(Reply::Litter),
            &[],
            "How often should I clean my guinea pig's tray?",
            Some(GUINEA_PIG_TRAY),
            None,
        ),
        (
            Some(Reply::Litter),
            &[],
            "How should I handle my hamster's litter tray?",
            Some(HAMSTER_TRAY),
            None,
        ),
        (
            Some(Reply::Litter),
            &[],
            "How should I handle my dog's litter tray?",
            Some(CAT_TRAY),
            None,
        ),
        (
            Some(Reply::Litter),
            &[],
            "How should I clean the litter tray?",
            Some(CAT_TRAY),
            None,
        ),
        (
            Some(Reply::Litter),
            &["We're getting a guinea pig next week."],
            "How often do I clean the tray?",
            Some(GUINEA_PIG_TRAY),
            None,
        ),
        (
            Some(Reply::Litter),
            &[
                "We're getting a guinea pig.",
                "It will get here soon.",
                "It will be blue.",
                "We have the carrier.",
            ],
            "How often do I clean the tray?",
            Some(CAT_TRAY),
            None,
        ),
        (
            Some(Reply::Litter),
            &["My sister has a rabbit.", "I have a hamster."],
            "How often do I clean the tray?",
            Some(HAMSTER_TRAY),
            None,
        ),
        (
            Some(Reply::CatLitter),
            &[],
            "My cat peed on the bed",
            Some(SPOT_CLEAN),
            None,
        ),
        (
            Some(Reply::CatLitter),
            &[],
            "My cat is going outside the litter tray",
            Some(CAT_LITTER),
            None,
        ),
        (
            Some(Reply::CatLitter),
            &[],
            "My cat peed on a bath mat",
            Some(SPOT_CLEAN),
            None,
        ),
        (
            Some(Reply::CatLitter),
            &[],
            "My cat's tray smells really bad",
            Some(CAT_LITTER),
            None,
        ),
        (
            Some(Reply::Setup),
            &[],
            "What do I need for a rabbit?",
            Some(RABBIT_SETUP),
            None,
        ),
        (
            Some(Reply::Setup),
            &[],
            "How often should I clean the rabbit hutch?",
            Some(RABBIT_CLEANING),
            None,
        ),
        (
            Some(Reply::Setup),
            &[],
            "What do I need for a cavy?",
            Some(GUINEA_PIG_SETUP),
            None,
        ),
        (
            Some(Reply::Setup),
            &[],
            "How often do I wash my guinea pig's cage?",
            Some(GUINEA_PIG_CLEANING),
            None,
        ),
        (
            Some(Reply::Setup),
            &[],
            "What do I need for a hamster?",
            Some(HAMSTER_SETUP),
            None,
        ),
        (
            Some(Reply::Setup),
            &[],
            "What do I need for a budgie?",
            None,
            None,
        ),
        (
            Some(Reply::Setup),
            &[],
            "What do I need?",
            Some(RABBIT_SETUP),
            None,
        ),
        (
            Some(Reply::Setup),
            &["We're getting a guinea pig. "],
            "How often do I clean it?",
            Some(GUINEA_PIG_CLEANING),
            None,
        ),
        (
            Some(Reply::Setup),
            &["We have a goldfish."],
            "What do I need?",
            None,
            None,
        ),
        (
            Some(Reply::Hamster),
            &[],
            "How do I set up a hamster cage?",
            Some(HAMSTER_SETUP),
            None,
        ),
        (
            Some(Reply::Hamster),
            &[],
            "How often do I wash my hamster's cage?",
            Some(HAMSTER_CLEANING),
            None,
        ),
        (
            Some(Reply::Hamster),
            &[],
            "How often do I wash a rabbit hutch?",
            Some(HAMSTER_CLEANING),
            None,
        ),
        (
            Some(Reply::Alone),
            &[],
            "Can I keep just one rabbit?",
            Some(RABBIT_COMPANY),
            None,
        ),
        (
            Some(Reply::Alone),
            &[],
            "Do guinea pigs need a friend?",
            Some(GUINEA_PIG_COMPANY),
            None,
        ),
        (
            Some(Reply::Alone),
            &[],
            "Can I leave my hamster alone?",
            None,
            None,
        ),
        (
            Some(Reply::Alone),
            &["We're getting a guinea pig."],
            "Can I keep just one?",
            Some(GUINEA_PIG_COMPANY),
            None,
        ),
        (
            Some(Reply::Alone),
            &["Our budgie is very loud."],
            "Can I keep just one?",
            None,
            None,
        ),
        (
            Some(Reply::Alone),
            &[],
            "Can I keep just one?",
            Some(RABBIT_COMPANY),
            None,
        ),
        (
            Some(Reply::Handling),
            &[],
            "My guinea pig squeals when I lift her",
            Some(GUINEA_PIG_HANDLING),
            Some(true),
        ),
        (
            Some(Reply::Handling),
            &[],
            "My hamster squeaks when I pick him up",
            Some(RODENT_HANDLING),
            Some(true),
        ),
        (
            Some(Reply::Handling),
            &[],
            "How do I pick up my ferret?",
            Some(FERRET_HANDLING),
            Some(true),
        ),
        (
            Some(Reply::Handling),
            &[],
            "How should I hold my rabbit?",
            Some(RABBIT_HANDLING),
            Some(true),
        ),
        (
            Some(Reply::Handling),
            &[],
            "How do I hold a hamster?",
            Some(RODENT_HANDLING),
            Some(false),
        ),
        (
            Some(Reply::Handling),
            &[],
            "How should I hold the guinea pig?",
            Some(GUINEA_PIG_HANDLING),
            Some(false),
        ),
        (
            Some(Reply::Handling),
            &[],
            "My hamster won't be held",
            Some(RODENT_HANDLING),
            Some(false),
        ),
        (
            Some(Reply::Handling),
            &[],
            "My hamster is nervous when held",
            Some(RODENT_HANDLING),
            Some(true),
        ),
        (
            Some(Reply::Handling),
            &[],
            "How do I hold it?",
            Some(RABBIT_HANDLING),
            Some(true),
        ),
        (
            Some(Reply::Handling),
            &[],
            "My rabbit and hamster are frightened",
            Some(RODENT_HANDLING),
            Some(true),
        ),
        (None, &[], "We just got a kitten", None, Some(false)),
        (
            None,
            &[],
            "Any tips on looking after a kitten?",
            None,
            Some(true),
        ),
        (
            None,
            &[],
            "How should I help a kitten settle?",
            None,
            Some(true),
        ),
        (
            None,
            &[],
            "I just got a kitten what do I need?",
            None,
            Some(false),
        ),
        (
            None,
            &[],
            "What do I need to know about a kitten?",
            None,
            Some(false),
        ),
        (None, &[], "What should I do?", None, Some(false)),
        (
            None,
            &[],
            "I brought home a kitten yesterday",
            None,
            Some(false),
        ),
        (
            None,
            &["We got a kitten."],
            "How old is he?",
            None,
            Some(false),
        ),
        (
            None,
            &["We got a kitten."],
            "Any tips on helping it settle?",
            None,
            Some(false),
        ),
    ];
    for &(kind, before, line, expected, fence) in cases {
        let mut history = History::default();
        let mut prior: Vec<Exchange> = Vec::new();
        let mut wants = None;
        for user in before {
            let mut reading = Reading::understand(
                user,
                prior.iter().map(|turn| turn.user.as_str()),
                wants,
                &history,
            );
            wants = reading.wants;
            history.retain(&mut reading);
            prior.push(Exchange {
                user: (*user).into(),
                bot: "All right.".into(),
                module: "greeting",
            });
        }
        let reading = Reading::understand(
            line,
            prior.iter().map(|turn| turn.user.as_str()),
            wants,
            &history,
        );
        let offered = kind.and_then(|kind| reply(kind, &reading, &prior));
        assert_eq!(
            offered.as_deref(),
            expected,
            "{kind:?} for {line:?} after {before:?}"
        );
        if let Some(fence) = fence {
            let actual = match kind {
                Some(Reply::Handling) => handling_context(&reading),
                None => new_kitten_context(&reading),
                other => panic!("unexpected guarded reply {other:?}"),
            };
            assert_eq!(
                actual, fence,
                "{kind:?} guard for {line:?} after {before:?}"
            );
        }
    }
}

const KITTEN_SUPPLIES: &str = "Litter tray and litter, two bowls, a scratching post, a carrier, a soft bed and kitten food, plus a vet visit booked for the first week. A second litter tray and a few toys save a lot of trouble, and nothing stringy or small enough to swallow.";

const CAT_SUPPLIES: &str = "Litter tray and litter, two bowls, a scratching post, a carrier, a soft bed and food for its age, plus a vet visit booked for the first week. A second litter tray and a few toys save a lot of trouble, and nothing stringy or small enough to swallow.";

const RESCUE_CAT: &str = "A rescue cat comes with a known character, is past the kitten climbing and biting, and settles in a week or two; a kitten is months of work but you shape it from the start. Out at work all day, or a first cat: the adult rescue is the kinder start.";

const RESCUE_DOG: &str = "A rescue adult usually comes with a known character, is past the chewing, and settles in a week or two; a puppy is a year of work but you shape everything from the start. First dog, or a busy household: the rescue is the kinder start.";

const DOG_BED: &str = "Put it in the quiet corner of the room he already settles in, out of the walkway and away from the radiator he will overheat beside: a dog rests best where he can see the door without lying across the path everyone takes, so the middle of a busy room stays empty. Warm it with a worn jumper of yours for the first week, and if he will not use it, move it to where he already lies down rather than buy another one.";

const CAT_BED: &str = "Put it where she already sleeps - the top of a wardrobe, a windowsill, the end of the sofa - and away from the draught between the door and the radiator: a cat picks height and a view over a soft bed on the floor, so a bed in the middle of a busy room stays empty. Warm it with a worn jumper of yours for the first week, and if she ignores it, move it rather than buy another one.";

const BOTH_BEDS: &str = "Put it where she already sleeps - the top of a wardrobe, a windowsill, the end of the sofa - and away from the draught between the door and the radiator: a cat picks height and a view over a soft bed on the floor, so a bed in the middle of a busy room stays empty. Warm it with a worn jumper of yours for the first week, and if she ignores it, move it rather than buy another one. Put it in the quiet corner of the room he already settles in, out of the walkway and away from the radiator he will overheat beside: a dog rests best where he can see the door without lying across the path everyone takes, so the middle of a busy room stays empty. Warm it with a worn jumper of yours for the first week, and if he will not use it, move it to where he already lies down rather than buy another one.";

const CAT_TRAY: &str = "Scoop the solids out twice a day and change the whole tray of clumping litter once a week - a tray that is cleaned that often does not need disinfecting every time, just hot water and a drop of unscented washing-up liquid. More trays than cats (three for two cats) is what stops accidents, and an unwashed tray is the commonest reason a cat starts going elsewhere. Never mix bleach with cat urine: it gives off a gas that hurts their lungs and yours.";

const RABBIT_TRAY: &str = "A rabbit litter-trains easily: a tray in the corner it already uses, lined with paper or wood pellets - never clumping cat litter, which sets hard if a rabbit eats it. Lift the wet litter out every day, change the whole tray once or twice a week, and give a pair two trays, because a rabbit likes to eat hay and go in the same spot. Under a hay rack is where it will use it most.";

const GUINEA_PIG_TRAY: &str = "A guinea pig picks a corner of the cage to use, so a tray there lined with paper or wood pellets catches most of it - never clumping cat litter, which sets hard if it is eaten. Lift the wet litter out every day and change the tray twice a week, and put the hay rack over it, because a guinea pig toilets where it eats.";

const HAMSTER_TRAY: &str = "A hamster picks one corner of the cage to pee in, so a small tray there with sand or paper pellets catches it: empty it every day or two and rinse it with hot water. Change the whole cage bedding every few weeks, not every week, because a hamster needs its nest to keep smelling of itself.";

const SPOT_CLEAN: &str = "Clean the spot properly first, because the smell is what takes her back to it: soak it with an enzyme cleaner, leave it to work and let it dry, and skip bleach and ammonia, because both smell like wee to a cat. Then the tray - one per cat plus one, scooped daily, unscented litter, in a quiet corner away from the food - and whatever changed in the house: a move, a new pet or a baby, building work, a door that now stays shut. A cat who was always clean and has suddenly started needs the vet first, because a urine infection or a bladder problem sits behind a lot of it.";

const CAT_LITTER: &str = "One tray per cat plus one, in quiet places, scooped daily and washed weekly with unscented soap. If it still smells after that, the plastic has picked it up: replace the tray and the litter, which is cheaper than fighting it. If a litter-trained cat suddenly goes outside the tray, a vet check comes first, because it is often a urine infection.";

const RABBIT_SETUP: &str = "A rabbit needs a hutch with a run attached, or a big shed, plus hay all day, a water bottle, a litter tray, bedding, a carrier for the vet and a friend of its own kind. A small cage with an hour out is not enough for a rabbit, and hay is most of what it eats: it wears the teeth down and keeps the gut moving.";

const RABBIT_CLEANING: &str = "Spot-clean the wet corner and the litter tray every day, change the bedding once a week and give the hutch a deep clean once a month: rabbit urine soaks into wood and the ammonia it gives off damages their lungs. Use a pet-safe disinfectant or a 1:10 white vinegar solution, rinse and dry it before the rabbit goes back in, and skip the phenolic ones like Jeyes fluid - they are toxic to small animals. Vinegar is also what takes the chalky white urine marks off the floor.";

const GUINEA_PIG_SETUP: &str = "A guinea pig needs a hutch with a big run attached, or a shed with floor space, hay all day and a handful of nuggets with vitamin C in them, since they cannot make their own. Keep two rather than one - a guinea pig on its own is a lonely animal - and skip the small cage.";

const GUINEA_PIG_CLEANING: &str = "Spot-clean the wet corner and the litter tray every day, change the bedding once a week and give the cage a deep clean once a month: guinea-pig urine soaks into wood and the ammonia it gives off damages their lungs. Use a pet-safe disinfectant or a 1:10 white vinegar solution, rinse and dry it before the guinea pigs go back in, and skip phenolic ones like Jeyes fluid. Keep a handful of the old bedding to put back, because a cage that smells completely new makes them nervous.";

const HAMSTER_SETUP: &str = "A hamster needs a deep bed of paper or wood-shaving bedding, a solid wheel and a sand bath, in a cage at least 80cm by 40cm on one level rather than tubes. Spot clean the wet corner daily, change the bedding every few weeks, and give a water bottle rather than a bowl. They are awake at night, so keep the cage out of the bedroom.";

const HAMSTER_CLEANING: &str = "Spot-clean the wet corner every day and change the bedding every few weeks, with a full scrub of the cage once a month: hamster urine is concentrated and the ammonia builds up fast in a small cage. Hot soapy water or a pet-safe cleaner, rinsed and dried right through before the bedding goes back, because a damp cage gives a hamster a chill and a chest infection. Put a handful of the old bedding back in the nest so the cage still smells of home.";

const RABBIT_COMPANY: &str = "Rabbits are herd animals, so one on its own is the exception: the best setup is a neutered pair in a hutch with a run attached, and a lone rabbit gets withdrawn, over-grooms and chews the bars. A few hours alone in the day is fine, a whole day and night with nobody in is not. Bonding needs neutral ground, two of everything - bowls, trays, hidey holes - and short meetings that build over a week or two, never one rabbit dropped into the other's hutch. Neuter both first, or they will fight and then breed.";

const GUINEA_PIG_COMPANY: &str = "Guinea pigs are herd animals, so a single one is the exception: two sows, or a neutered boar with a sow, in a big cage or a hutch with a run, is the normal setup, and a guinea pig kept alone gets withdrawn and can stop eating. A few hours alone in the day is fine, a whole day and night with nobody in is not. Bond on neutral ground with two of everything and short meetings, and never drop a new one into the other's cage. A rabbit is not a substitute: they eat different food and a rabbit can injure a guinea pig.";

const GUINEA_PIG_HANDLING: &str = "A guinea pig is scooped up in both hands from underneath, one hand under the hindquarters and the other across the shoulders, and held low over your lap - never by the scruff, the ears or a leg, and never up at chest height, because a guinea pig that is dropped or that jumps from a height breaks its back. Sit on the floor with him on your lap for a few minutes a day and a bit of food in your hand, and put him down before he starts to struggle, so being picked up stops being a surprise. A pet that squeals, flinches or struggles at being handled when he did not before needs a vet - a sore tooth, an injured back or mites is the usual reason.";

const RODENT_HANDLING: &str = "A hamster, gerbil, rat or mouse is cupped in both hands, one under the chest and one over the back, and never picked up by the tail or the scruff. Sit on the floor with him on your lap for a few minutes a day and a bit of food in your hand, and put him down before he starts to struggle, so being picked up stops being a surprise. A pet that squeals, flinches or struggles at being handled when he did not before needs a vet - a sore tooth, an injured back or mites is the usual reason.";

const FERRET_HANDLING: &str = "A ferret is picked up round the shoulders with the hindquarters supported in your other hand, and never by the tail or the ears. Sit on the floor with him on your lap for a few minutes a day and a bit of food in your hand, and put him down before he starts to struggle, so being picked up stops being a surprise. A pet that squeals, flinches or struggles at being handled when he did not before needs a vet - a sore tooth, an injured back or mites is the usual reason.";

const RABBIT_HANDLING: &str = "A rabbit is picked up with one hand under the chest and the other under the hindquarters, held against your body so the back legs cannot kick out - never by the ears, the scruff or the back of the neck, because a rabbit that kicks while nothing is holding its back end can break its own spine. Sit on the floor with him on your lap for a few minutes a day and a bit of food in your hand, and put him down before he starts to struggle, so being picked up stops being a surprise. A pet that squeals, flinches or struggles at being handled when he did not before needs a vet - a sore tooth, an injured back or mites is the usual reason and a rabbit that stops eating after it is worth ringing the vet the same day.";

#[test]
fn cat_urinary_reports_get_prompt_or_immediate_veterinary_advice() {
    for (input, expected) in [
        ("My cat is weeing blood.", super::CAT_URINE_PROMPT),
        ("My cat has a urine infection.", super::CAT_URINE_PROMPT),
        ("My cat is straining to pee.", super::CAT_URINE_NOW),
        ("My cat is trying to pee.", super::CAT_URINE_NOW),
        (
            "My cat is spraying and only a few drops come out.",
            super::CAT_URINE_NOW,
        ),
        (
            "My cat keeps going to the litter tray but cannot pee.",
            super::CAT_URINE_NOW,
        ),
        (
            "My cat is going in and out of the litter tray and no urine is coming out.",
            super::CAT_URINE_NOW,
        ),
    ] {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        let turn = bot.turn(input);
        assert_eq!(turn.module, Some("answers"), "{input}: {turn:?}");
        assert_eq!(turn.reply, expected, "{input}");
    }
    for input in [
        "My cat keeps spraying the wall.",
        "Where should I put the cat litter tray?",
        "Is the flea spray safe around my cat?",
        "My cat is straining to poop in the litter tray.",
        "My cat is fine and I can't pee after cleaning the litter tray.",
    ] {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        let turn = bot.turn(input);
        assert_ne!(turn.reply, super::CAT_URINE_NOW, "{input}");
        assert_ne!(turn.reply, super::CAT_URINE_PROMPT, "{input}");
    }
}

#[test]
fn lost_pet_advice_follows_animal_and_stands_aside_for_prevention() {
    let dog = lost(&Reading::new("My dog is missing")).unwrap();
    assert!(dog.contains("your council's dog warden"));
    let cat = lost(&Reading::new("I cannot find my kitten")).unwrap();
    assert!(cat.contains("your cat is microchipped"));
    assert!(!cat.contains("warden"));
    assert!(lost(&Reading::new("How do I stop my dog escaping the garden?")).is_none());
}

#[test]
fn baby_and_booster_replies_follow_the_previous_subject() {
    let earlier = [Exchange {
        user: "I'm pregnant".into(),
        bot: String::new(),
        module: "answers",
    }];
    let cat_at_home = Reading::new("Is a cat okay with a new baby?");
    assert!(baby(&cat_at_home, &earlier).starts_with("During pregnancy"));
    assert!(baby(&cat_at_home, &[]).starts_with("Yes, a cat and a baby"));

    let earlier = [Exchange {
        user: "I have a pet rabbit".into(),
        bot: String::new(),
        module: "answers",
    }];
    let check = Reading::new("How often does she need a vet check?");
    assert!(checkup(&check, &earlier).contains("myxomatosis"));
    let dog = Reading::new("Does my dog need yearly jabs?");
    assert!(checkup(&dog, &earlier).contains("Booster intervals vary"));
}

#[test]
fn seasonal_cat_and_soaking_guards() {
    use super::super::pets_training::not_a_cat_name;

    assert!(!not_a_cat_name(&Reading::new("My cat is called Tom")));
    assert!(not_a_cat_name(&Reading::new(
        "My cat is called Tom. Is he in season?"
    )));
    assert!(got_wet(&Reading::new("My guinea pig fell in the sink")));
    assert!(!got_wet(&Reading::new(
        "My dog shakes when the washing machine goes on"
    )));
}
