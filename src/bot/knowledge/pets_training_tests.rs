//! Direct callback fixtures captured from PyBot 7d63c7f.
use super::*;
use crate::bot::understanding::context::History;

type Case = (
    Reply,
    &'static [&'static str],
    &'static str,
    Option<&'static str>,
);

#[test]
fn warning_and_senior_guards_distinguish_play_pain_and_age() {
    for (prompt, expected) in [
        ("My dog growls at me when I touch his back.", true),
        ("My dog yelps when I touch his back.", false),
        ("My puppy snaps at my hands.", false),
        ("My dog showed his teeth.", true),
    ] {
        assert_eq!(
            warning_sound_said(&Reading::new(prompt)),
            expected,
            "{prompt}"
        );
    }
    for (prompt, expected) in [
        ("My older dog is sleepy.", true),
        ("My dog is seven years old.", true),
        ("My two year old dog is sleepy.", false),
        ("My three year old dog has stiff hips.", true),
    ] {
        assert_eq!(dog_not_young(&Reading::new(prompt)), expected, "{prompt}");
    }
    let card = crate::lookup_knowledge("My dog growls at me when I touch his back.").unwrap();
    assert_eq!((card.domain, card.id), ("pets", "dog-warning"));
    assert!(card.reply.starts_with("A growl is a warning"));
}

#[test]
fn pets_training_callbacks_match_the_reference() {
    let cases: &[Case] = &[
        (
            Reply::Recall,
            &[],
            "How do I teach my dog to come when called?",
            Some(RECALL),
        ),
        (
            Reply::Recall,
            &[],
            "My puppy won't come when I call",
            Some(RECALL),
        ),
        (Reply::Recall, &[], "What's my dog called?", None),
        (Reply::Recall, &[], "What should I call my new puppy?", None),
        (Reply::Recall, &[], "My dog is called Biscuit", None),
        (
            Reply::Recall,
            &[],
            "My dog called Rex won't come back",
            None,
        ),
        (
            Reply::Recall,
            &[],
            "When I called him he looked up",
            Some(RECALL),
        ),
        (
            Reply::Recall,
            &[],
            "What should I do when I call him?",
            Some(RECALL),
        ),
        (
            Reply::Recall,
            &[],
            "We call our dog back with a whistle",
            None,
        ),
        (Reply::Recall, &[], "I called for my puppy", Some(RECALL)),
        (
            Reply::Chewing,
            &[],
            "My puppy chews everything",
            Some(YOUNG_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My dog chewed the arm of the sofa",
            Some(ADULT_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My 20 month old puppy keeps chewing the sofa",
            Some(ADULT_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My two year old puppy chews my coat",
            Some(ADULT_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My dog is five months old and chewing everything",
            Some(YOUNG_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My dog is twenty weeks of age and chews every shoe",
            Some(YOUNG_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My dog is half a year old and teething",
            Some(YOUNG_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My dog is 52 weeks old and teething",
            Some(ADULT_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My six-year-old collie chewed the sofa",
            Some(ADULT_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "Young dog keeps chewing",
            Some(YOUNG_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My dog is teething",
            Some(YOUNG_CHEWING),
        ),
        (
            Reply::Chewing,
            &[],
            "My dog is half a month old and chewing",
            Some(YOUNG_CHEWING),
        ),
        (
            Reply::Barking,
            &[],
            "My dog barks at the door",
            Some(BARKING_PLAN),
        ),
        (
            Reply::Barking,
            &[],
            "Why does my dog bark at the door?",
            Some(BARKING_REASON),
        ),
        (
            Reply::Barking,
            &[],
            "What makes my dog bark at the door?",
            Some(BARKING_REASON),
        ),
        (
            Reply::Barking,
            &[],
            "How do I stop my dog barking?",
            Some(BARKING_PLAN),
        ),
        (
            Reply::Barking,
            &[],
            "My dog is barking, why?",
            Some(BARKING_REASON),
        ),
        (
            Reply::Barking,
            &[],
            "I want to know why my dog is barking",
            Some(BARKING_REASON),
        ),
        (
            Reply::Wary,
            &[],
            "My dog is scared of my husband",
            Some(WARY_HUSBAND),
        ),
        (
            Reply::Wary,
            &[],
            "Why is my dog scared of men?",
            Some(WHY_WARY_OF_MEN),
        ),
        (
            Reply::Wary,
            &[],
            "My dog hides from strangers",
            Some(WARY_STRANGERS),
        ),
        (
            Reply::Wary,
            &[],
            "My dog is scared of my dad and my partner",
            Some(WARY_PARTNER),
        ),
        (
            Reply::Wary,
            &[],
            "My dog is scared of my mother",
            Some(WARY_MUM),
        ),
        (
            Reply::Wary,
            &[],
            "My dog hides from my neighbor",
            Some(WARY_NEIGHBOUR),
        ),
        (
            Reply::Wary,
            &[],
            "My dog is afraid of visitors",
            Some(WARY_VISITORS),
        ),
        (
            Reply::Wary,
            &[],
            "Why is my puppy wary of that guest?",
            Some(WHY_WARY_OF_GUEST),
        ),
        (
            Reply::Wary,
            &[],
            "My dog is afraid of someone",
            Some(WARY_SOMEONE),
        ),
        (
            Reply::Wary,
            &[],
            "My dog is afraid of the tall person",
            Some(WARY_UNNAMED),
        ),
        (
            Reply::Wary,
            &[],
            "Why is my dog afraid of my grandma?",
            Some(WHY_WARY_OF_GRAN),
        ),
        (
            Reply::Noise,
            &[],
            "My cat is afraid of the hoover",
            Some(CAT_NOISE),
        ),
        (
            Reply::Noise,
            &[],
            "My rabbit hides from the vacuum",
            Some(RABBIT_NOISE),
        ),
        (
            Reply::Noise,
            &[],
            "My puppy is scared of the mixer",
            Some(DOG_NOISE),
        ),
        (
            Reply::Noise,
            &[],
            "My hamster is scared of the balloon",
            Some(DOG_NOISE),
        ),
        (
            Reply::Noise,
            &[],
            "My cat and dog both hate the vacuum",
            Some(CAT_NOISE),
        ),
        (
            Reply::Noise,
            &["My cat is twelve years old."],
            "She's frightened of the hoover.",
            Some(CAT_NOISE),
        ),
        (
            Reply::Noise,
            &["I have a rabbit."],
            "He's scared of the blender.",
            Some(RABBIT_NOISE),
        ),
        (
            Reply::Night,
            &[],
            "My dog keeps waking at 9 every night",
            Some(ADULT_NIGHT),
        ),
        (
            Reply::Night,
            &[],
            "My senior dog paces at night",
            Some(SENIOR_NIGHT),
        ),
        (
            Reply::Night,
            &[],
            "My 9 week old puppy cries at night",
            Some(PUPPY_NIGHT),
        ),
        (
            Reply::Night,
            &[],
            "My 5 month old dog cries at night",
            Some(SENIOR_NIGHT),
        ),
        (
            Reply::Night,
            &[],
            "My dog is 3 and still wakes",
            Some(SENIOR_NIGHT),
        ),
        (
            Reply::Night,
            &[],
            "My dog turns 9 in May and is waking up",
            Some(ADULT_NIGHT),
        ),
        (
            Reply::Night,
            &[],
            "My dog is eight years old and wakes at night",
            Some(SENIOR_NIGHT),
        ),
        (
            Reply::Night,
            &[],
            "My 10 year old puppy cries at night",
            Some(PUPPY_NIGHT),
        ),
        (
            Reply::Night,
            &[],
            "My dog is about 4 and keeps waking up",
            Some(SENIOR_NIGHT),
        ),
        (
            Reply::Night,
            &["We brought the puppy home."],
            "How long until he sleeps through the night?",
            Some(PUPPY_NIGHT),
        ),
        (
            Reply::Night,
            &["We brought the puppy home."],
            "My dog is 11 and wakes at night",
            Some(SENIOR_NIGHT),
        ),
        (
            Reply::Night,
            &["We brought the pup home."],
            "How long until he sleeps through the night?",
            Some(PUPPY_NIGHT),
        ),
        (
            Reply::Night,
            &[
                "We brought the puppy home.",
                "That's good.",
                "I got a carrier.",
            ],
            "How long until he sleeps through the night?",
            Some(ADULT_NIGHT),
        ),
        (
            Reply::Bored,
            &[],
            "My cat is bored, any toys?",
            Some(CAT_BORED),
        ),
        (
            Reply::Bored,
            &[],
            "I think my rabbit is bored",
            Some(RABBIT_BORED),
        ),
        (
            Reply::Bored,
            &[],
            "My dog gets bored when I'm at work",
            Some(DOG_BORED_AT_WORK),
        ),
        (Reply::Bored, &[], "My dog is bored", Some(DOG_BORED)),
        (Reply::Bored, &[], "Is my guinea pig bored?", None),
        (Reply::Bored, &[], "My cat and hamster need toys", None),
        (Reply::Bored, &[], "My budgie is bored", None),
        (
            Reply::Bored,
            &["My cat is bored."],
            "What toys should I get her?",
            Some(DOG_BORED),
        ),
        (
            Reply::Bored,
            &["My rabbit has been restless."],
            "How can I keep him occupied?",
            Some(DOG_BORED),
        ),
        (
            Reply::Name,
            &[],
            "Does my cat know her name?",
            Some(CAT_NAME),
        ),
        (
            Reply::Name,
            &[],
            "Can a rabbit learn its name?",
            Some(RABBIT_NAME),
        ),
        (
            Reply::Name,
            &[],
            "Does my dog know my voice?",
            Some(DOG_NAME),
        ),
        (Reply::Name, &[], "Does my guinea pig know his name?", None),
        (
            Reply::Name,
            &[],
            "Does my cat or rabbit know its name?",
            Some(CAT_NAME),
        ),
        (
            Reply::Name,
            &["My cat is called Esme."],
            "Does she know her name?",
            Some(DOG_NAME),
        ),
        (
            Reply::Name,
            &["I have a rabbit."],
            "Will he learn his name?",
            Some(RABBIT_NAME),
        ),
    ];
    for &(kind, before, line, expected) in cases {
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
        let offered = reply(kind, &reading, &prior);
        assert_eq!(
            offered.as_deref(),
            expected,
            "{kind:?} for {line:?} after {before:?}"
        );
    }
}

const RECALL: &str = "Practise coming back somewhere boring, on a long line, saying his name once and rewarding with something better than the treats he gets otherwise: cheese or sausage. Never call him to something he dislikes, like a bath or the end of a walk, or the word stops working.";

const YOUNG_CHEWING: &str = "Chewing is normal while teeth change, so redirect it: plenty of chew toys, a frozen carrot or stuffed Kong, and pick everything else up out of reach. Puppies usually grow out of it by seven or eight months.";

const ADULT_CHEWING: &str = "An adult dog chews for a reason and teeth are not it: boredom, a walk that is too short, or being left alone longer than it can take. Give it a proper sniffy walk before you go out, leave a stuffed Kong or a long-lasting chew, and block off the sofa for a fortnight while the habit breaks. If it has just started in a dog that never did it, get its teeth and gums checked, because a sore mouth makes a dog chew to soothe it.";

const BARKING_PLAN: &str = "Find what starts it and block that first: a curtain over the window he watches, or the post put through a box away from the door. Then reward the quiet - a treat the moment he stops - and don't shout, because to a dog shouting sounds like you joining in. If he barks for hours when he is left, that is the being-left problem rather than the barking one.";

const BARKING_REASON: &str = "Barking is usually the alarm going off, so find what starts it and block that first: a curtain over the window he watches, or the post put through a box away from the door. Then reward the quiet - a treat the moment he stops - and don't shout, because to a dog shouting sounds like you joining in. If he barks for hours when he is left, that is the being-left problem rather than the barking one.";

const WARY_HUSBAND: &str = "Let him come to your husband on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while your husband sits nearby and ignores the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WHY_WARY_OF_MEN: &str = "Wary dogs are usually more frightened than bold, so let him come to men on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while they sit nearby and ignore the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WARY_STRANGERS: &str = "Let him come to strangers on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while they sit nearby and ignore the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WARY_PARTNER: &str = "Let him come to your partner on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while your partner sits nearby and ignores the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WARY_MUM: &str = "Let him come to your mum on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while your mum sits nearby and ignores the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WARY_NEIGHBOUR: &str = "Let him come to your neighbour on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while your neighbour sits nearby and ignores the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WARY_VISITORS: &str = "Let him come to visitors on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while they sit nearby and ignore the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WHY_WARY_OF_GUEST: &str = "Wary dogs are usually more frightened than bold, so let him come to that guest on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while they sit nearby and ignore the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WARY_SOMEONE: &str = "Let him come to that person on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while that person sits nearby and ignores the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WARY_UNNAMED: &str = "Let him come to the person he is wary of on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while that person sits nearby and ignores the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const WHY_WARY_OF_GRAN: &str = "Wary dogs are usually more frightened than bold, so let him come to your gran on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while your gran sits nearby and ignores the dog. Being boring and calm is what builds trust, and it usually takes weeks rather than days.";

const CAT_NOISE: &str = "Let a cat leave the room and shut the door - walking away is what settles them - then switch it on for a few seconds at a time with the food bowl down, stopping before they bolt. Don't hold a cat still in front of it: a frightened animal that can't get away gets worse, not braver. If they shake for hours or won't eat afterwards, ask the vet.";

const RABBIT_NOISE: &str = "Let a rabbit leave the room and shut the door - walking away is what settles them - then switch it on for a few seconds at a time with the food bowl down, stopping before they bolt. Don't hold a rabbit still in front of it: a frightened animal that can't get away gets worse, not braver. If they shake for hours or won't eat afterwards, ask the vet.";

const DOG_NOISE: &str = "Let a dog leave the room and shut the door - walking away is what settles them - then switch it on for a few seconds at a time with the food bowl down, stopping before they bolt. Don't hold a dog still in front of it: a frightened animal that can't get away gets worse, not braver. If they shake for hours or won't eat afterwards, ask the vet.";

const ADULT_NIGHT: &str = "Keep the night dull and the day busy: a last toilet trip on the lead, no lights, no chat and no play when he wakes, and a long walk plus some training in the afternoon so he is tired at bedtime. If he is sleeping a lot in the day, wake him gently for a short walk. A dog who starts waking at night after years of sleeping through is worth a vet check, especially if he is drinking more or seems stiff.";

const SENIOR_NIGHT: &str = "An older dog who starts waking at night is usually in discomfort rather than being naughty: arthritis makes lying still painful, and a dog who is stiff, pacing, panting or confused in the evening needs the vet rather than a training plan. Book a check, ask about pain relief, put a soft bed in a warm room with a night light, and take him out once last thing; sudden confusion in an old dog is worth a same-day call.";

const PUPPY_NIGHT: &str = "A puppy cannot hold it all night: at eight weeks expect to get up once, around 2 or 3am, and take him out just before you go to bed. Keep the last meal early and the bed beside yours so he can smell you, and answer the crying quietly and briefly with no lights and no play, so night stays boring. Most puppies sleep seven hours by about four months.";

const CAT_BORED: &str = "Play with a wand toy for five or ten minutes, two or three times a day, and let her stalk and pounce before you let her catch it, then give her a meal so the hunt has an end. The rest of the day wants things rather than toys: a food puzzle, a cardboard box, a paper bag, somewhere high to sit and a window to watch. Rotate the toys and put half of them away, because a basket she has seen for months is furniture.";

const RABBIT_BORED: &str = "A rabbit is bored when it has nothing to chew and nowhere to dig: give it a run of a few hours a day outside a hutch, a box of hay to root through, willow or apple sticks to chew and something to hide under. A lone rabbit wants a rabbit friend more than it wants toys, and the two are happiest neutered and introduced slowly.";

const DOG_BORED_AT_WORK: &str = "Four hours at a time is about the usual comfortable limit for an adult dog, so build the time up in short chunks and have someone look in at lunchtime for a puppy. Then fill the day: half his dinner scattered on the lawn or in a snuffle mat, something legal to chew, the radio on, and a walk where he is allowed to sniff when you get back. A dog left with nothing to do chews the furniture and paces, and that is boredom rather than badness.";

const DOG_BORED: &str = "A dog is bored because the day is short on sniffing and chewing, not because he needs more toys. One walk where he is allowed to stop and sniff does more than a long march, and a scatter of his dinner on the lawn or in a snuffle mat takes ten minutes off the clock. Add five minutes of training a day, something legal to chew and a game of tug, and break the day up rather than saving it all for the evening.";

const CAT_NAME: &str = "Yes - a cat can pick her own name out of a list of similar words, so she knows it about as well as a dog does; what she does not do is come running when she hears it. Say it before the food or the wand toy rather than after, and she will start looking up when you use it. A cat that stops answering her name may have gone deaf, which a vet can check, or may just have decided the sofa is better.";

const RABBIT_NAME: &str = "A rabbit learns its name as a sound that means food, not the way a dog does: say it just before you put the bowl down or open the run, and it will start turning towards you. They are prey animals and will not usually come when called, and a rabbit that ignores its name has not forgotten it.";

const DOG_NAME: &str = "Yes - most family dogs understand a few dozen words, and the best of them well over a hundred, and their own name is the first. Say the name once and then the cue - 'Biscuit, sit' - and reward, so the name becomes the thing to look at before something good happens. Never use it to tell him off: a name that means trouble is a name he learns to ignore.";
