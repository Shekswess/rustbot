use super::respond;
use crate::bot::{Memory, Reading};

#[test]
fn hears_the_hard_event_the_person_describes_as_past() {
    let result = respond(
        &Reading::new("my dad walked out when i was ten"),
        &Memory::default(),
    );
    assert_eq!(
        result.as_deref(),
        Some("That is a lot to have been through. Tell me as much or as little as you like.")
    );
}

#[test]
fn passes_an_ask_a_world_fact_and_a_closed_or_other_kind_of_event() {
    let memory = Memory::default();
    for line in [
        "was your dad an alcoholic too",
        "the prison was built in 1800",
        "i was in rehab in 2019 and im clean since",
        "my baby was stillborn",
        "we are getting a divorce",
    ] {
        assert_eq!(respond(&Reading::new(line), &memory), None, "{line}");
    }
}
