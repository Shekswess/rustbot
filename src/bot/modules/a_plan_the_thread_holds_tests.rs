// Pinned plans and fallback gates from PyBot 7d63c7f.
use super::*;
use crate::bot::Exchange;

fn conversation(lines: &[&str]) -> Memory {
    let turns = lines
        .iter()
        .map(|&line| Exchange {
            user: line.into(),
            module: "greeting",
            bot: "Say more.".into(),
        })
        .collect();
    Memory {
        turns,
        ..Memory::default()
    }
}

type PlanCase = (
    &'static str,
    &'static [&'static str],
    Option<[&'static str; 3]>,
);

#[test]
fn matches_pinned_day_plan_pools_and_abstentions() {
    let cases: &[PlanCase] = &[
        (
            "so whats the plan for the day then",
            &[
                "were going to a gig on saturday",
                "three of us",
                "eating first",
                "cheap and cheerful",
            ],
            Some([
                "You have told me \"were going to a gig on saturday\", \"eating first\" and \"cheap and cheerful\", so the order is the hour that cannot move, then the meal. Book the table tonight: the rest is only times.",
                "I have it all: \"were going to a gig on saturday\", \"eating first\" and \"cheap and cheerful\". The order is the hour that cannot move, then the meal, and the one thing left to find is the doors time - the venue's own page has that.",
                "Nothing here needs looking up - you have told me \"were going to a gig on saturday\", \"eating first\" and \"cheap and cheerful\", and that is a day out with the order already in it: the hour that cannot move, then the meal. Get the table settled and the day runs itself.",
            ]),
        ),
        (
            "what is the plan",
            &[
                "a train on tuesday",
                "three of us",
                "go back that afternoon",
            ],
            Some([
                "You have told me \"a train on tuesday\", \"three of us\" and \"go back that afternoon\", so the order is the hour that cannot move. Book the two bookings tonight: the rest is only times.",
                "I have it all: \"a train on tuesday\", \"three of us\" and \"go back that afternoon\". The order is the hour that cannot move, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"a train on tuesday\", \"three of us\" and \"go back that afternoon\", and that is a day out with the order already in it: the hour that cannot move. Get the two bookings settled and the day runs itself.",
            ]),
        ),
        (
            "whats the order",
            &[
                "going for a show",
                "stay over",
                "four of us",
                "we got a table",
            ],
            Some([
                "You have told me \"going for a show\", \"four of us\" and \"we got a table\", so the order is the hour that cannot move, then the meal, then the bed. The one thing that runs out is done already, so the hours are all that is left.",
                "I have it all: \"going for a show\", \"four of us\" and \"we got a table\". The order is the hour that cannot move, then the meal, then the bed, and the one thing left to find is the doors time - the venue's own page has that.",
                "Nothing here needs looking up - you have told me \"going for a show\", \"four of us\" and \"we got a table\", and that is a day out with the order already in it: the hour that cannot move, then the meal, then the bed. Fix the hours round it and the day runs itself.",
            ]),
        ),
        (
            "what should we do first",
            &[
                "going to a concert",
                "three of us",
                "eat first",
                "got a table",
            ],
            Some([
                "You have told me \"going to a concert\", \"eat first\" and \"got a table\", so the order is the hour that cannot move, then the meal. The one thing that runs out is done already, so the hours are all that is left.",
                "I have it all: \"going to a concert\", \"eat first\" and \"got a table\". The order is the hour that cannot move, then the meal, and the one thing left to find is the doors time - the venue's own page has that.",
                "Nothing here needs looking up - you have told me \"going to a concert\", \"eat first\" and \"got a table\", and that is a day out with the order already in it: the hour that cannot move, then the meal. Fix the hours round it and the day runs itself.",
            ]),
        ),
        (
            "where do we start",
            &[
                "a night out for us",
                "we booked a table for four",
                "we are staying in a hotel",
            ],
            Some([
                "You have told me \"a night out for us\", \"we booked a table for four\" and \"we are staying in a hotel\", so the order is the hour that cannot move, then the meal, then the bed. The one thing that runs out is done already, so the hours are all that is left.",
                "I have it all: \"a night out for us\", \"we booked a table for four\" and \"we are staying in a hotel\". The order is the hour that cannot move, then the meal, then the bed, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"a night out for us\", \"we booked a table for four\" and \"we are staying in a hotel\", and that is a day out with the order already in it: the hour that cannot move, then the meal, then the bed. Fix the hours round it and the day runs itself.",
            ]),
        ),
        (
            "sort out the plan",
            &[
                "were going on a day out",
                "five of us",
                "five of us",
                "five of us",
            ],
            Some([
                "You have told me \"were going on a day out\" and \"five of us\", so the order is the hour that cannot move. Book the two bookings tonight: the rest is only times.",
                "I have it all: \"were going on a day out\" and \"five of us\". The order is the hour that cannot move, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"were going on a day out\" and \"five of us\", and that is a day out with the order already in it: the hour that cannot move. Get the two bookings settled and the day runs itself.",
            ]),
        ),
        (
            "sort out the plan",
            &[
                "were going on a day out",
                "five of us",
                "is it going to rain?",
            ],
            Some([
                "You have told me \"were going on a day out\" and \"five of us\", so the order is the hour that cannot move. Book the two bookings tonight: the rest is only times.",
                "I have it all: \"were going on a day out\" and \"five of us\". The order is the hour that cannot move, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"were going on a day out\" and \"five of us\", and that is a day out with the order already in it: the hour that cannot move. Get the two bookings settled and the day runs itself.",
            ]),
        ),
        (
            "whats the plan for the visit",
            &["we have a train first", "three of us", "the same day back"],
            Some([
                "You have told me \"we have a train first\", \"three of us\" and \"the same day back\", so the order is the hour that cannot move. Book the two bookings tonight: the rest is only times.",
                "I have it all: \"we have a train first\", \"three of us\" and \"the same day back\". The order is the hour that cannot move, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"we have a train first\", \"three of us\" and \"the same day back\", and that is a day out with the order already in it: the hour that cannot move. Get the two bookings settled and the day runs itself.",
            ]),
        ),
        (
            "whats the plan for the evening",
            &["meet at the museum", "six of us", "stay over in a hostel"],
            Some([
                "You have told me \"meet at the museum\", \"six of us\" and \"stay over in a hostel\", so the order is the hour that cannot move, then the bed. Book the room tonight: the rest is only times.",
                "I have it all: \"meet at the museum\", \"six of us\" and \"stay over in a hostel\". The order is the hour that cannot move, then the bed, and the one thing left to find is the doors time - the venue's own page has that.",
                "Nothing here needs looking up - you have told me \"meet at the museum\", \"six of us\" and \"stay over in a hostel\", and that is a day out with the order already in it: the hour that cannot move, then the bed. Get the room settled and the day runs itself.",
            ]),
        ),
        (
            "so whats the plan for the day then",
            &[
                "HELLO FRIEND!",
                "we have a hotel",
                "two of us",
                "do you know the route",
            ],
            Some([
                "You have told me \"hello friend\", \"we have a hotel\" and \"two of us\", so the order is the hour that cannot move, then the bed. Book the room tonight: the rest is only times.",
                "I have it all: \"hello friend\", \"we have a hotel\" and \"two of us\". The order is the hour that cannot move, then the bed, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"hello friend\", \"we have a hotel\" and \"two of us\", and that is a day out with the order already in it: the hour that cannot move, then the bed. Get the room settled and the day runs itself.",
            ]),
        ),
        (
            "what shall we do next",
            &[
                "we booked a cinema for three",
                "dinner first",
                "what's on?",
                "keep it cheap",
            ],
            Some([
                "You have told me \"we booked a cinema for three\", \"dinner first\" and \"keep it cheap\", so the order is the hour that cannot move, then the meal. The one thing that runs out is done already, so the hours are all that is left.",
                "I have it all: \"we booked a cinema for three\", \"dinner first\" and \"keep it cheap\". The order is the hour that cannot move, then the meal, and the one thing left to find is the doors time - the venue's own page has that.",
                "Nothing here needs looking up - you have told me \"we booked a cinema for three\", \"dinner first\" and \"keep it cheap\", and that is a day out with the order already in it: the hour that cannot move, then the meal. Fix the hours round it and the day runs itself.",
            ]),
        ),
        (
            "what should we do then",
            &[
                "were going to a theatre",
                "dinner first",
                "tickets are booked",
                "i can drive",
            ],
            Some([
                "You have told me \"were going to a theatre\", \"tickets are booked\" and \"i can drive\", so the order is the hour that cannot move, then the meal. The one thing that runs out is done already, so the hours are all that is left.",
                "I have it all: \"were going to a theatre\", \"tickets are booked\" and \"i can drive\". The order is the hour that cannot move, then the meal, and the one thing left to find is the doors time - the venue's own page has that.",
                "Nothing here needs looking up - you have told me \"were going to a theatre\", \"tickets are booked\" and \"i can drive\", and that is a day out with the order already in it: the hour that cannot move, then the meal. Fix the hours round it and the day runs itself.",
            ]),
        ),
        (
            "plan out the trip",
            &[
                "were going to a theater",
                "flight on friday",
                "four of us",
                "we are looking at bed and breakfast",
            ],
            Some([
                "You have told me \"were going to a theater\", \"four of us\" and \"we are looking at bed and breakfast\", so the order is the hour that cannot move, then the meal, then the bed. Book the table and the room tonight: the rest is only times.",
                "I have it all: \"were going to a theater\", \"four of us\" and \"we are looking at bed and breakfast\". The order is the hour that cannot move, then the meal, then the bed, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"were going to a theater\", \"four of us\" and \"we are looking at bed and breakfast\", and that is a day out with the order already in it: the hour that cannot move, then the meal, then the bed. Get the table and the room settled and the day runs itself.",
            ]),
        ),
        (
            "sort the plan",
            &[
                "yes thanks",
                "nice!",
                "a trip to the museum",
                "on the saturday",
                "two of us",
            ],
            Some([
                "You have told me \"a trip to the museum\", \"on the saturday\" and \"two of us\", so the order is the hour that cannot move. Book the two bookings tonight: the rest is only times.",
                "I have it all: \"a trip to the museum\", \"on the saturday\" and \"two of us\". The order is the hour that cannot move, and the one thing left to find is the doors time - the venue's own page has that.",
                "Nothing here needs looking up - you have told me \"a trip to the museum\", \"on the saturday\" and \"two of us\", and that is a day out with the order already in it: the hour that cannot move. Get the two bookings settled and the day runs itself.",
            ]),
        ),
        (
            "whats the plan",
            &[
                "a train on tuesday",
                "three of us",
                "whats the plan",
                "go back that afternoon",
            ],
            Some([
                "You have told me \"a train on tuesday\", \"three of us\" and \"go back that afternoon\", so the order is the hour that cannot move. Book the two bookings tonight: the rest is only times.",
                "I have it all: \"a train on tuesday\", \"three of us\" and \"go back that afternoon\". The order is the hour that cannot move, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"a train on tuesday\", \"three of us\" and \"go back that afternoon\", and that is a day out with the order already in it: the hour that cannot move. Get the two bookings settled and the day runs itself.",
            ]),
        ),
        (
            "whats the plan for her",
            &[
                "were going to a gig on saturday",
                "three of us",
                "eating first",
                "cheap and cheerful",
            ],
            None,
        ),
        (
            "what should we do after dinner",
            &[
                "were going to a gig on saturday",
                "three of us",
                "eating first",
                "cheap and cheerful",
            ],
            None,
        ),
        (
            "what is the meal",
            &[
                "were going to a gig on saturday",
                "three of us",
                "eating first",
                "cheap and cheerful",
            ],
            None,
        ),
        (
            "what is the plan if she cannot come",
            &[
                "were going to a gig on saturday",
                "three of us",
                "eating first",
                "cheap and cheerful",
            ],
            None,
        ),
        (
            "what is the plan",
            &[
                "were cooking roast at home",
                "five of us",
                "lunch for the family",
            ],
            None,
        ),
        (
            "where do we start",
            &["we need a van", "few of us", "moving house"],
            None,
        ),
        (
            "where do we start",
            &[
                "the venue is on saturday",
                "go there after work",
                "the venue closes early",
            ],
            None,
        ),
        (
            "whats the plan",
            &["were going out for dinner", "bringing friends"],
            None,
        ),
        (
            "whats the plan",
            &[
                "were going to a show",
                "what restaurant are we going to?",
                "do we need a room?",
            ],
            None,
        ),
        (
            "whats the plan",
            &[
                "hello",
                "were going to the show",
                "thanks so much",
                "nice one",
            ],
            None,
        ),
        (
            "whats the plan",
            &[
                "we are heading to the harbour",
                "boarding the boat at eight",
                "we cant book ahead",
            ],
            None,
        ),
        (
            "whats the plan",
            &[
                "we need to pack the van tomorrow morning and decide who will hold the keys for the door as well",
                "three of us",
                "eat early",
            ],
            None,
        ),
        ("whats the plan", &[], None),
        (
            "whats the plan",
            &[
                "going to the cinema",
                "cheap and cheerful",
                "what time do the doors open",
            ],
            None,
        ),
        (
            "whats the plan",
            &[
                "the four of us leave tomorrow",
                "book a hotel",
                "need a room close to town",
            ],
            Some([
                "You have told me \"the four of us leave tomorrow\", \"book a hotel\" and \"need a room close to town\", so the order is the hour that cannot move, then the bed. Book the room tonight: the rest is only times.",
                "I have it all: \"the four of us leave tomorrow\", \"book a hotel\" and \"need a room close to town\". The order is the hour that cannot move, then the bed, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"the four of us leave tomorrow\", \"book a hotel\" and \"need a room close to town\", and that is a day out with the order already in it: the hour that cannot move, then the bed. Get the room settled and the day runs itself.",
            ]),
        ),
        (
            "what is the plan",
            &["stay over in a hotel", "cheap and cheerful", "two of us"],
            Some([
                "You have told me \"stay over in a hotel\", \"cheap and cheerful\" and \"two of us\", so the order is the hour that cannot move, then the bed. Book the room tonight: the rest is only times.",
                "I have it all: \"stay over in a hotel\", \"cheap and cheerful\" and \"two of us\". The order is the hour that cannot move, then the bed, and the one thing left to find is the hour you have to be there - the booking page has that.",
                "Nothing here needs looking up - you have told me \"stay over in a hotel\", \"cheap and cheerful\" and \"two of us\", and that is a day out with the order already in it: the hour that cannot move, then the bed. Get the room settled and the day runs itself.",
            ]),
        ),
        (
            "what should we do first",
            &[
                "we have tickets for the show",
                "three of us",
                "has the room been booked?",
            ],
            Some([
                "You have told me \"we have tickets for the show\" and \"three of us\", so the order is the hour that cannot move. Book the two bookings tonight: the rest is only times.",
                "I have it all: \"we have tickets for the show\" and \"three of us\". The order is the hour that cannot move, and the one thing left to find is the doors time - the venue's own page has that.",
                "Nothing here needs looking up - you have told me \"we have tickets for the show\" and \"three of us\", and that is a day out with the order already in it: the hour that cannot move. Get the two bookings settled and the day runs itself.",
            ]),
        ),
    ];
    for &(ask, past, options) in cases {
        let actual = pool(ask, &conversation(past));
        assert_eq!(
            actual
                .as_ref()
                .map(|replies| replies.each_ref().map(String::as_str)),
            options,
            "{ask:?} {past:?}"
        );
    }
}
