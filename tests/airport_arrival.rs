use rustbot::{Bot, lookup_knowledge};

#[test]
fn airport_arrival_uses_departure_guidance_for_the_named_airport() {
    for (prompt, source_wording) in [
        (
            "When should we get to the airport before our flight?",
            "departure airport's arrival guidance",
        ),
        (
            "How early should I get to Heathrow airport before my flight?",
            "two hours before a UK or European flight",
        ),
        (
            "What time should I arrive at Gatwick airport?",
            "check-in, bag-drop and gate deadlines",
        ),
        (
            "How early should I get to the gate for my connecting flight at Heathrow airport?",
            "itinerary's minimum connection time",
        ),
    ] {
        let card = lookup_knowledge(prompt).unwrap();
        assert_eq!((card.domain, card.id), ("travel", "airport-arrival"));
        assert!(card.reply.contains(source_wording), "{prompt}");
        let turn = Bot::new(Some("0")).unwrap().turn(prompt);
        assert_eq!(turn.reply, card.reply, "{prompt}");
    }
}

#[test]
fn supplied_buffer_uses_departure_clock_in_the_conversation() {
    for (prompt, time) in [
        (
            "My flight leaves at 6am and the airline says arrive 2 hours before. What time should I get to the airport?",
            "04:00",
        ),
        (
            "My flight leaves at 12:15am and I need to be there two hours before departure. What time do I get to the airport?",
            "22:15 the previous day",
        ),
        (
            "My flight departs at 3:30 pm. The airport says arrive 2 hours and 30 minutes before. What time should I arrive for my flight?",
            "13:00",
        ),
    ] {
        let turn = Bot::new(Some("0")).unwrap().turn(prompt);
        assert_eq!(turn.module, Some("answers"), "{prompt}");
        assert!(turn.reply.contains(time), "{prompt}: {}", turn.reply);
    }
    let ambiguous = Bot::new(Some("0")).unwrap().turn(
        "My flight leaves at 6. What time should I arrive at the airport if I need to arrive two hours before?",
    );
    assert!(
        ambiguous
            .reply
            .contains("departure airport's arrival guidance")
    );
}

#[test]
fn live_queues_refer_to_the_airport_for_current_waits() {
    for prompt in [
        "What's the queue like at the airport?",
        "What is the security queue like at Heathrow airport right now?",
        "How long is the wait at airport check-in?",
    ] {
        let turn = Bot::new(Some("0")).unwrap().turn(prompt);
        assert!(
            turn.reply.starts_with("I can't see the current queue."),
            "{prompt}: {}",
            turn.reply
        );
        assert!(turn.reply.contains("airport's website or app"));
    }
}

#[test]
fn airport_arrival_defers_to_other_travel_subjects() {
    for prompt in [
        "What time is my flight?",
        "Which gate is my flight from and what time?",
        "How long is the flight from the airport to Rome?",
        "How quickly can I get a refund after my flight was cancelled at the airport?",
        "How early should I check in for a cruise?",
        "How long should I allow for the airport transfer between the hotel and town?",
        "What food should I take on a long flight?",
        "Is October too early to buy flights?",
    ] {
        assert!(
            lookup_knowledge(prompt)
                .is_none_or(|card| (card.domain, card.id) != ("travel", "airport-arrival")),
            "{prompt}"
        );
    }
}
