use super::pool;

#[test]
fn keeps_the_purpose_and_matches_the_number_in_the_pronoun() {
    let one = pool("im looking at an electric blanket for the winter").unwrap();
    assert!(one[0].starts_with("With an electric blanket for the winter"));
    assert!(one[0].contains("who repairs it"));

    let many = pool("im looking at bikes").unwrap();
    assert!(many[0].starts_with("With bikes"));
    assert!(many[0].contains("who repairs them"));
}

#[test]
fn passes_an_ask_a_search_and_an_unrelated_thing() {
    for line in [
        "what do you think of an electric blanket",
        "looking for my keys",
        "im thinking of buying a used car",
        "its a bigger job than i thought",
    ] {
        assert_eq!(pool(line), None, "{line}");
    }
}
