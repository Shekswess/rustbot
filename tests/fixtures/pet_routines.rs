// Authored callbacks captured from pinned PyBot 7d63c7f.
pub struct Case(
    pub &'static str,
    pub &'static [&'static str],
    pub &'static str,
    pub bool,
    pub Option<&'static str>,
);
pub const CASES: &[Case] = &[
    Case(
        "dog-pulling",
        &[],
        "How do I stop my dog pulling the lead?",
        true,
        Some(
            "Train the lead as a stop-and-go game: the moment the lead goes tight, stand still until it slackens, then walk on, and reward him at your side with a treat. A front-clip harness or head halter helps while he learns, and a treat held at your knee keeps him where you want him.",
        ),
    ),
    Case(
        "dog-pulling",
        &[],
        "My dog slipped his collar.",
        true,
        Some(
            "Check the fit before the training: a collar should take two fingers and no more, and a dog who backs out of a lead does it at the widest part of his head, so a sighthound or a narrow-headed dog needs a harness or a collar that tightens a little as he pulls. Practise putting it on and off with a treat so the lead is not the thing he braces against, and check the clip and stitching for wear.",
        ),
    ),
    Case(
        "dog-pulling",
        &[],
        "My dog's collar is too tight.",
        true,
        Some(
            "Train the lead as a stop-and-go game: the moment the lead goes tight, stand still until it slackens, then walk on, and reward him at your side with a treat. A front-clip harness or head halter helps while he learns, and a treat held at your knee keeps him where you want him.",
        ),
    ),
    Case(
        "dog-pulling",
        &[],
        "How should I hold the lead on walks?",
        true,
        Some(
            "Train the lead as a stop-and-go game: the moment the lead goes tight, stand still until it slackens, then walk on, and reward him at your side with a treat. A front-clip harness or head halter helps while he learns, and a treat held at your knee keeps him where you want him.",
        ),
    ),
    Case(
        "dog-pulling",
        &[],
        "My razor keeps pulling.",
        false,
        Some(
            "Train the lead as a stop-and-go game: the moment the lead goes tight, stand still until it slackens, then walk on, and reward him at your side with a treat. A front-clip harness or head halter helps while he learns, and a treat held at your knee keeps him where you want him.",
        ),
    ),
    Case(
        "dog-pulling",
        &["My dog pulls on walks."],
        "What about the garden?",
        false,
        Some(
            "Train the lead as a stop-and-go game: the moment the lead goes tight, stand still until it slackens, then walk on, and reward him at your side with a treat. A front-clip harness or head halter helps while he learns, and a treat held at your knee keeps him where you want him.",
        ),
    ),
    Case(
        "dog-pulling-dog",
        &[],
        "My dog keeps pulling when we go out.",
        true,
        Some(
            "Train the lead as a stop-and-go game: the moment the lead goes tight, stand still until it slackens, then walk on, and reward him at your side with a treat. A front-clip harness or head halter helps while he learns, and a treat held at your knee keeps him where you want him.",
        ),
    ),
    Case(
        "dog-pulling-dog",
        &[],
        "My puppy drags me on the lead.",
        true,
        Some(
            "Train the lead as a stop-and-go game: the moment the lead goes tight, stand still until it slackens, then walk on, and reward him at your side with a treat. A front-clip harness or head halter helps while he learns, and a treat held at your knee keeps him where you want him.",
        ),
    ),
    Case(
        "dog-pulling-dog",
        &[],
        "My dog slips his harness.",
        true,
        Some(
            "Check the fit before the training: a collar should take two fingers and no more, and a dog who backs out of a lead does it at the widest part of his head, so a sighthound or a narrow-headed dog needs a harness or a collar that tightens a little as he pulls. Practise putting it on and off with a treat so the lead is not the thing he braces against, and check the clip and stitching for wear.",
        ),
    ),
    Case(
        "cat-biting",
        &[],
        "How can I stop my cat biting my hands during play?",
        true,
        Some(
            "Hands are not the toy: play with a rod toy on a string and put it away after ten minutes, twice a day, because an under-exercised cat bites. Watch for the warning - tail flicking, ears back, skin twitching along the back - and stop stroking before it, since most bites are a cat asking for a break. A bite that comes out of nowhere in a calm cat is worth a vet visit, because pain or a bad tooth is often behind it.",
        ),
    ),
    Case(
        "cat-biting",
        &[],
        "My cat hisses, growls and then bites when I stroke her.",
        true,
        Some(
            "Take the warning as a no and give her the space: put her down or step back, stop what you were doing, and let her come to you rather than picking her up to show her it is fine. Never tell a cat off for hissing or growling - a cat punished for the warning skips it and goes straight to the bite - and give her somewhere high to get away to, because most of this is a cat with no escape route. Watch for the tail flick, the ears going back and the skin twitching along the back, and stop stroking before those. A cat who suddenly hisses or growls at someone she knows, or when she is picked up, needs a vet: pain, a bad tooth or arthritis is the usual reason, and this is often the first sign of it.",
        ),
    ),
    Case(
        "cat-biting",
        &[],
        "My kitten nips my hands.",
        true,
        Some(
            "Hands are not the toy: play with a rod toy on a string and put it away after ten minutes, twice a day, because an under-exercised cat bites. Watch for the warning - tail flicking, ears back, skin twitching along the back - and stop stroking before it, since most bites are a cat asking for a break. A bite that comes out of nowhere in a calm cat is worth a vet visit, because pain or a bad tooth is often behind it.",
        ),
    ),
    Case(
        "cat-biting",
        &[],
        "My cat gave me a deep bite and it is swollen.",
        true,
        Some(
            "Hands are not the toy: play with a rod toy on a string and put it away after ten minutes, twice a day, because an under-exercised cat bites. Watch for the warning - tail flicking, ears back, skin twitching along the back - and stop stroking before it, since most bites are a cat asking for a break. A bite that comes out of nowhere in a calm cat is worth a vet visit, because pain or a bad tooth is often behind it.",
        ),
    ),
    Case(
        "cat-and-children",
        &[],
        "Is it safe to have a cat around my children?",
        true,
        Some(
            "A cat and a child get on fine with three rules: never leave a baby or a toddler alone in a room with the cat, give the cat somewhere up high and a room it can leave by, and show the child one finger down the back and to leave the cat alone while it is eating or in its basket. The old story that a cat smothers a baby is not true - a cat goes to warmth, so keep it out of the cot, and shut the bedroom and the pram against it while the baby sleeps. A swat is nearly always fear rather than nastiness: a cat that swats a child it used to accept should see the vet, because pain is often behind a change. Wash any scratch with soap and water, and a deep bite on a hand wants looking at.",
        ),
    ),
    Case(
        "cat-and-children",
        &[],
        "We have two kids and a cat.",
        true,
        None,
    ),
    Case(
        "cat-and-children",
        &[],
        "I worry how my cat will get on with my child.",
        true,
        Some(
            "A cat and a child get on fine with three rules: never leave a baby or a toddler alone in a room with the cat, give the cat somewhere up high and a room it can leave by, and show the child one finger down the back and to leave the cat alone while it is eating or in its basket. The old story that a cat smothers a baby is not true - a cat goes to warmth, so keep it out of the cot, and shut the bedroom and the pram against it while the baby sleeps. A swat is nearly always fear rather than nastiness: a cat that swats a child it used to accept should see the vet, because pain is often behind a change. Wash any scratch with soap and water, and a deep bite on a hand wants looking at.",
        ),
    ),
    Case(
        "cat-and-children",
        &[],
        "Can my toddler play with our cat?",
        true,
        Some(
            "A cat and a child get on fine with three rules: never leave a baby or a toddler alone in a room with the cat, give the cat somewhere up high and a room it can leave by, and show the child one finger down the back and to leave the cat alone while it is eating or in its basket. The old story that a cat smothers a baby is not true - a cat goes to warmth, so keep it out of the cot, and shut the bedroom and the pram against it while the baby sleeps. A swat is nearly always fear rather than nastiness: a cat that swats a child it used to accept should see the vet, because pain is often behind a change. Wash any scratch with soap and water, and a deep bite on a hand wants looking at.",
        ),
    ),
    Case(
        "pet-moult",
        &[],
        "My dog is moulting a lot.",
        true,
        Some(
            "A dog blowing its coat twice a year is normal: brush it daily while it lasts, more if the coat is long, and a rubber glove or a damp hand takes the loose hair off the furniture. Bare skin, bald patches or constant scratching is a skin question for the vet rather than the moult.",
        ),
    ),
    Case(
        "pet-moult",
        &[],
        "My rabbit is moulting.",
        true,
        Some(
            "A rabbit in moult needs brushing every day, and twice a day at the peak, because the fur it swallows is what can block the gut: keep the hay coming and watch the droppings. Bare patches, or skin that is sore or scurfy, are the vet rather than the moult.",
        ),
    ),
    Case(
        "pet-moult",
        &[],
        "My rabbit is bringing in a new coat.",
        true,
        Some(
            "A rabbit in moult needs brushing every day, and twice a day at the peak, because the fur it swallows is what can block the gut: keep the hay coming and watch the droppings. Bare patches, or skin that is sore or scurfy, are the vet rather than the moult.",
        ),
    ),
    Case(
        "pet-moult",
        &[],
        "My puppy is blowing his coat.",
        true,
        Some(
            "A dog blowing its coat twice a year is normal: brush it daily while it lasts, more if the coat is long, and a rubber glove or a damp hand takes the loose hair off the furniture. Bare skin, bald patches or constant scratching is a skin question for the vet rather than the moult.",
        ),
    ),
    Case(
        "pet-moult",
        &[],
        "My cat is moulting.",
        true,
        Some(
            "A dog blowing its coat twice a year is normal: brush it daily while it lasts, more if the coat is long, and a rubber glove or a damp hand takes the loose hair off the furniture. Bare skin, bald patches or constant scratching is a skin question for the vet rather than the moult.",
        ),
    ),
    Case(
        "pet-scratching",
        &[],
        "How do I stop my cat scratching the door?",
        true,
        Some(
            "A pet that scratches the door wants through, so the scratching is the way it has learned to ask. Do not open it while the paw is on the door: wait for a pause of a few seconds, then open it, and it learns the pause is what works. Hanging a bell on the door and teaching a touch of the nose to it gives a cat a quieter way to ask, and a dog with a shut door between him and you usually wants company more than the room: put his bed on the side of the door you are on, and leave the door open once he is settled.",
        ),
    ),
    Case(
        "pet-scratching",
        &[],
        "My dog scratches the carpet before bed.",
        true,
        Some(
            "Scratching the floor or the bedding before lying down is normal: dogs and cats turn and scrape to flatten a spot and leave their scent, and it is not a fault. It is worth answering if it is new or constant: give him a soft bed of his own so he has something to turn round in, check the room is not too warm, and if a dog is scraping hard at hard flooring his claws may be catching, so have them clipped. A cat that scrapes at the carpet by the door is marking, and a mat or a vertical post in that spot takes the behaviour.",
        ),
    ),
    Case(
        "pet-scratching",
        &[],
        "My cat is scratching the armchair.",
        true,
        Some(
            "Put a tall scratching post right beside the corner she uses and make the sofa less appealing with a throw or double-sided tape for a few weeks: cats scratch to mark territory and to shed the outer claw, so the post has to be tall enough to stretch up and steady enough not to wobble. A dog scratching at the sofa or the cushions is usually digging up a nest, so give him a blanket to scrabble at and a walk before the evening, and if it is one spot in particular put something solid in front of it.",
        ),
    ),
    Case(
        "pet-scratching",
        &[],
        "My dog scratches the sofa.",
        true,
        Some(
            "A dog scrabbling at the sofa or the cushions is usually digging itself a nest, so give him a blanket or an old duvet of his own to scrabble at and a walk before the evening. If it is one spot he keeps going at, put something solid in front of it for a fortnight and praise him when he uses his own bed instead. Chewing the furniture is a different problem with a different answer, so say if that is what is happening.",
        ),
    ),
    Case(
        "pet-scratching",
        &[],
        "My pet is scratching herself constantly.",
        true,
        Some(
            "A pet scratching itself sore is a skin question, not a behaviour one: fleas are the first thing to rule out, then a food or pollen allergy, and a cat over-grooming a bald patch can be mites or stress. Have the vet look before the skin is broken, because an open scratch gets infected. In the meantime a flea treatment from the vet rather than the supermarket, no shampoo unless the vet names one, and a cone if he will not leave a wound alone.",
        ),
    ),
    Case(
        "pet-scratching",
        &[],
        "My cat has been scratching its ears.",
        true,
        Some(
            "A pet scratching itself sore is a skin question, not a behaviour one: fleas are the first thing to rule out, then a food or pollen allergy, and a cat over-grooming a bald patch can be mites or stress. Have the vet look before the skin is broken, because an open scratch gets infected. In the meantime a flea treatment from the vet rather than the supermarket, no shampoo unless the vet names one, and a cone if he will not leave a wound alone.",
        ),
    ),
    Case(
        "pet-scratching",
        &[],
        "How do I remove cat scratch marks from wood?",
        true,
        Some(
            "Give them something that is theirs to scratch and put it where they already do it: a tall, steady post for a cat, and a blanket or a bit of matting for a dog. Cats scratch to mark and to shed claw, so a post beside the sofa stops the sofa being the spot, and a dog scraping at the floor or the bedding before lying down is just making a nest. Clipped claws and a walk before the evening take the rest of it, and scratching that has suddenly become constant, or is leaving the skin raw, is the vet's.",
        ),
    ),
    Case(
        "pet-loss",
        &[],
        "My dog died last night.",
        true,
        Some(
            "I'm sorry, that's one of the hardest days. Nothing has to be decided today: the vet can arrange cremation or a home burial, and you can ask for a paw print or a lock of fur to keep. Be gentle with yourself for a while; it does get easier.",
        ),
    ),
    Case(
        "pet-loss",
        &[],
        "We had to put our cat to sleep.",
        true,
        Some(
            "I'm sorry, that's one of the hardest days. Nothing has to be decided today: the vet can arrange cremation or a home burial, and you can ask for a paw print or a lock of fur to keep. Be gentle with yourself for a while; it does get easier.",
        ),
    ),
    Case("pet-loss", &[], "My rabbit ate something dead.", true, None),
    Case("pet-loss", &[], "My dog is asleep.", true, None),
    Case(
        "pet-loss",
        &["My cat keeps bringing in mice."],
        "They were dead.",
        true,
        Some(
            "I'm sorry, that's one of the hardest days. Nothing has to be decided today: the vet can arrange cremation or a home burial, and you can ask for a paw print or a lock of fur to keep. Be gentle with yourself for a while; it does get easier.",
        ),
    ),
    Case(
        "pet-loss",
        &["My cat caught three mice this week."],
        "They are all dead.",
        true,
        Some(
            "That is the hunting instinct rather than anything wrong with the animal. Clear what it brings in away in gloves, bin or bury it where nothing else can get at it, and wash your hands after. Hunters pick up worms from what they catch, so keep the worming up to date, and a quick-release collar with a bell plus keeping it in from dusk until morning cuts how often it happens.",
        ),
    ),
    Case(
        "pet-loss",
        &["My cat caught a mouse."],
        "My cat died.",
        true,
        Some(
            "I'm sorry, that's one of the hardest days. Nothing has to be decided today: the vet can arrange cremation or a home burial, and you can ask for a paw print or a lock of fur to keep. Be gentle with yourself for a while; it does get easier.",
        ),
    ),
];
