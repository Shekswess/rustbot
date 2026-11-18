//! Pinned direct game, hobby and chess cases from PyBot 7d63c7f.
use super::*;

#[test]
fn games_and_leisure_callbacks_match_the_reference() {
    let cases = [
        (
            Some(Reply::Party),
            "games for a toddler party",
            None,
            Some(
                "Keep each game to five minutes and let everyone win: pass the parcel with a sweet in every layer, musical statues where you freeze in a silly shape, sleeping lions, and a treasure hunt of pictures taped at their height round one room. Cake and food first, games after, is easier at that age.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Party),
            "indoor birthday games for a 6 year old",
            None,
            Some(
                "Ten minutes a game is plenty at that age: musical statues, pass the parcel with a sweet in every layer, pin the tail on the donkey, and a treasure hunt of picture clues round the house. Sleeping lions calms them down before food, and Simon Says fills the gap while the cake comes out.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Party),
            "my daughter is turning 8 what games for the party",
            None,
            Some(
                "At that age they want to win something: a treasure hunt with written clues, a relay or an obstacle race in the garden, and musical chairs with music they picked. Have two prizes a game so nobody is out early for long, and keep one quiet game for the end, when the sugar has worn off.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Party),
            "party games for children",
            None,
            Some(
                "Ten minutes a game is plenty: musical statues, pass the parcel with a sweet in every layer, pin the tail on the donkey, and a treasure hunt of picture clues round the garden. Sleeping lions calms them down before food, and Simon Says fills the gap while the cake comes out. Tell me the age and I'll pitch it right.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Party),
            "6 year old wants games for grown ups",
            None,
            None,
            Some(true),
        ),
        (
            Some(Reply::Party),
            "birthday party games for adults",
            None,
            None,
            Some(true),
        ),
        (
            Some(Reply::Party),
            "what can we do at my daughters 5th birthday in our flat",
            None,
            Some(
                "Ten minutes a game is plenty at that age: musical statues, pass the parcel with a sweet in every layer, pin the tail on the donkey, and a treasure hunt of picture clues round the house. Sleeping lions calms them down before food, and Simon Says fills the gap while the cake comes out.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Party),
            "i missed my daughters school play",
            None,
            Some(
                "Ten minutes a game is plenty: musical statues, pass the parcel with a sweet in every layer, pin the tail on the donkey, and a treasure hunt of picture clues round the garden. Sleeping lions calms them down before food, and Simon Says fills the gap while the cake comes out. Tell me the age and I'll pitch it right.",
            ),
            Some(false),
        ),
        (
            Some(Reply::Card),
            "what are the rules of uno",
            None,
            Some(
                "Deal seven cards each and turn one over as the pile. On your turn put down a card matching the top one in colour or number, or use the action card you are holding: skip, reverse, and the +2 make the next player miss or draw. You win by getting rid of every card, and you must call 'uno' when you are down to one - that is the rule everyone forgets.",
            ),
            None,
        ),
        (
            Some(Reply::Card),
            "how do you play pontoon",
            None,
            Some(
                "The aim is to get closer to 21 than the dealer without going over: number cards count their number, picture cards ten, and an ace is 1 or 11, whichever helps. You are dealt two cards and choose hit or stand; the dealer must draw to 16 and stop on 17. Over 21 is a bust and you lose at once, which is why 16 is a bad hand to be brave on.",
            ),
            None,
        ),
        (
            Some(Reply::Card),
            "how do you play patience",
            None,
            Some(
                "Deal seven columns of one to seven cards, only the top card of each face up, and the rest of the pack face down as the stock. Build the columns downwards in alternating colours and the four piles upwards from ace in one suit, moving cards between columns as they turn over. Turn the stock one card at a time - that is the version most people play; the win is all four suits stacked to king.",
            ),
            None,
        ),
        (
            Some(Reply::Card),
            "how do i play snap",
            None,
            Some(
                "Deal the whole pack out between you and turn cards face up into the middle one at a time. When two cards of the same number land on top of each other, the first person to shout 'snap' and put a hand flat on the pile takes it. Whoever ends up with every card wins, and small children are much better at this than adults.",
            ),
            None,
        ),
        (
            Some(Reply::Card),
            "how to play go fish",
            None,
            Some(
                "Deal five cards each and spread the rest face down in the middle. On your turn ask one player for a number you already hold, and if they have it you take the card and ask again; if not they say 'go fish' and you take one from the middle. Lay down every pair you complete, and the most pairs at the end wins.",
            ),
            None,
        ),
        (Some(Reply::Card), "how do i play bridge", None, None, None),
        (None, "what is a castle", None, None, Some(false)),
        (None, "how do I castle", None, None, Some(true)),
        (None, "when should i castle", None, None, Some(true)),
        (None, "when did the castle fall", None, None, Some(true)),
        (None, "what is castling in chess", None, None, Some(true)),
        (
            None,
            "what are the long and short moves",
            None,
            None,
            Some(true),
        ),
        (
            Some(Reply::Activity),
            "how do i start photography",
            None,
            Some(
                "Use the phone you already have for a month and finish one project - a colour, a street, one tree every day - before you buy a camera. Then a second-hand body with a single 35mm or 50mm lens beats a new camera with two bad ones. Learn three things first: light behind you, get closer than feels natural, and take ten shots of one thing instead of one shot of ten things.",
            ),
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i join a choir",
            None,
            Some(
                "Find a choir rather than a teacher first: most towns have a community choir that takes people who have never sung, no audition, about five pounds a session. Singing with others teaches pitch faster than singing alone, and the confidence is most of it. Take a teacher later if you want to sing on your own.",
            ),
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i learn to sew",
            None,
            Some(
                "A basic machine with a straight stitch and a zigzag is all anyone needs, and a second-hand one from a repair shop costs less than a new cheap one. Start on scraps, not clothes: stitch straight lines, then a cushion cover, then take in a hem on something you do not mind ruining. A local sewing shop's two-hour beginner class is worth more than a video, because someone sets the tension for you.",
            ),
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i start baking",
            None,
            Some(
                "Weigh 500g strong white flour, 7g dried yeast, 10g salt and 350ml warm-not-hot water: that is the whole recipe, and a scale is the only kit you need. Knead ten minutes, leave it two hours somewhere warm until it has doubled, shape it and bake it hot. Then bake that same loaf five times rather than five different things - the first is usually dense and the fifth is usually good, so the only way to fail is to stop at the first.",
            ),
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i bake a potato",
            None,
            None,
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i make fish pie",
            None,
            None,
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i fish on a river",
            None,
            Some(
                "In England and Wales you need a rod licence first - about seven pounds for a day, from the Environment Agency online - and then a day ticket at a local club water and a beginner's float rod set for about forty pounds. Go with somebody who fishes if you can, and learn one knot (the improved clinch), one rig and one bait before buying anything else. Go in the morning in summer: fishing is mostly about being somewhere quiet, so the first trip is a success if you enjoy it.",
            ),
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i stop fish falling apart",
            None,
            None,
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i start painting",
            None,
            Some(
                "Start with acrylics: they dry in minutes, clean up with water, and a starter set is under twenty pounds. If it is watercolour you want, the paper decides everything - cold-pressed 300gsm and no cheaper - and you work light to dark because you cannot paint white back in. Either way, paint the same simple thing - one mug, one window - five times instead of five different things, and work bigger than feels comfortable with one brush that holds a point.",
            ),
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i paint my shed",
            None,
            None,
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i choose a paint colour",
            None,
            None,
            None,
        ),
        (
            Some(Reply::Activity),
            "where do i take old paint",
            None,
            None,
            None,
        ),
        (
            Some(Reply::Activity),
            "how do i start watercolours",
            None,
            Some(
                "Start with acrylics: they dry in minutes, clean up with water, and a starter set is under twenty pounds. If it is watercolour you want, the paper decides everything - cold-pressed 300gsm and no cheaper - and you work light to dark because you cannot paint white back in. Either way, paint the same simple thing - one mug, one window - five times instead of five different things, and work bigger than feels comfortable with one brush that holds a point.",
            ),
            None,
        ),
        (
            Some(Reply::GamePick),
            "recommend some board games",
            None,
            Some(
                "On a table: *Ticket to Ride* (trains, simple, satisfying), *Codenames* (two teams, one word grid) and *Azul* (tiles, quick, sharp). *Ticket to Ride* is the safest box for a household.",
            ),
            Some(true),
        ),
        (
            Some(Reply::GamePick),
            "what should i play on my switch",
            None,
            Some(
                "On a Switch: *Animal Crossing: New Horizons* to wind down, *Mario Kart 8 Deluxe* for people in the room, and *The Legend of Zelda: Breath of the Wild* for a big adventure.",
            ),
            Some(true),
        ),
        (
            Some(Reply::GamePick),
            "any games for two of us",
            None,
            Some(
                "*Hades* is the one I'd hand you first: fast, short runs, and it never wastes your time. *Stardew Valley* if you want calm, and *The Last of Us* if you want a story to sit through.",
            ),
            Some(true),
        ),
        (
            Some(Reply::GamePick),
            "a calm game to play when im stressed",
            None,
            Some(
                "*Stardew Valley* is the one for that: farm, fish, befriend the village, nothing is timed against you. *Animal Crossing* on a phone or a Switch, or *Unpacking* if you want something short and gentle.",
            ),
            Some(true),
        ),
        (
            Some(Reply::GamePick),
            "what is a good video game for a story",
            None,
            Some(
                "For a story: *The Last of Us* (grief and a road trip), *Red Dead Redemption 2* (a slow western with a lot of heart), or *Disco Elysium* if you would rather read. *The Last of Us* is the one people finish and then sit still for a while.",
            ),
            Some(true),
        ),
        (
            Some(Reply::GamePick),
            "good video game for one sitting",
            None,
            Some(
                "Short ones: *Unpacking* (about three hours, and it says more than it sounds like it would), *Portal* (also about three), or *Firewatch* (an evening, and it stays with you).",
            ),
            Some(true),
        ),
        (
            Some(Reply::GamePick),
            "do i need a coop for my chickens",
            None,
            None,
            Some(true),
        ),
        (
            Some(Reply::GamePick),
            "is monopoly a good game",
            None,
            Some(
                "*Hades* is the one I'd hand you first: fast, short runs, and it never wastes your time. *Stardew Valley* if you want calm, and *The Last of Us* if you want a story to sit through.",
            ),
            Some(false),
        ),
        (
            Some(Reply::GamePick),
            "should i buy a ps5 or an xbox",
            None,
            Some(
                "On a console: *Hades* for short, fast runs, *The Last of Us* if you want a story to sit through, and *Elden Ring* if you want a huge, hard world. *Hades* is the one I'd start with.",
            ),
            Some(false),
        ),
        (
            Some(Reply::GamePick),
            "how do i get better at video games",
            None,
            Some(
                "*Hades* is the one I'd hand you first: fast, short runs, and it never wastes your time. *Stardew Valley* if you want calm, and *The Last of Us* if you want a story to sit through.",
            ),
            Some(false),
        ),
        (
            Some(Reply::Solo),
            "what can i play on my own",
            None,
            Some(
                "*Stardew Valley* is the one: farm, fish, befriend the village, nothing timed against you, and it runs on almost anything. *Hades* if you want ten-minute runs with a fight in them, or *Unpacking* for an evening of something quiet.",
            ),
            None,
        ),
        (
            Some(Reply::GamePick),
            "recommend some games",
            Some("my boy is 9 years old and into dinosaurs"),
            Some(
                "*Dobble*, *UNO* and *Jenga* work from about five up, and *Catan Junior* if they are ready for a proper board game. A treasure hunt round the house costs nothing and takes the longest.",
            ),
            Some(false),
        ),
        (
            Some(Reply::Solo),
            "what can i play on my own",
            Some("my boy is 9 years old and into dinosaurs"),
            Some(
                "*Dobble* is fine with one player as a speed game, and *UNO* works solo if you play the house down. *Catan Junior* really needs two, though.",
            ),
            None,
        ),
        (
            Some(Reply::Solo),
            "what can i play on my own",
            Some("my boy is 15 years old and into dinosaurs"),
            Some(
                "*Stardew Valley* is the one: farm, fish, befriend the village, nothing timed against you, and it runs on almost anything. *Hades* if you want ten-minute runs with a fight in them, or *Unpacking* for an evening of something quiet.",
            ),
            None,
        ),
    ];
    let mut differences = Vec::new();
    for (kind, line, prior, expected, guard) in cases {
        let earlier = prior.map(|user| Exchange {
            user: user.into(),
            bot: String::new(),
            module: "greeting",
        });
        let reading = Reading::new(line);
        let result = kind.and_then(|kind| reply(kind, &reading, earlier.as_slice()));
        if result.as_deref() != expected {
            differences.push(format!(
                "{kind:?} {line}: {result:?}, reference {expected:?}"
            ));
        }
        if let Some(expected) = guard {
            let actual = match kind {
                Some(Reply::Party) => party_context(&reading),
                Some(Reply::GamePick) => pick_context(&reading, earlier.as_slice()),
                _ => chess_context(&reading),
            };
            if actual != expected {
                differences.push(format!(
                    "{kind:?} {line}: guard {actual}, reference {expected}"
                ));
            }
        }
    }
    assert!(
        differences.is_empty(),
        "mismatches\n{}",
        differences.join("\n")
    );
}
