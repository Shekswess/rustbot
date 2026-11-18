//! Compare structured native readings with the reference observations at PYBOT_REVISION.
#[path = "fixtures/understanding.rs"]
mod reference;
use serde_json::Value;

#[test]
fn matches_reference_reader_fields() {
    compare(reference::EXPECTED, reference::CASES);
}

#[path = "fixtures/topics.rs"]
mod topics;

#[test]
fn matches_reference_topic_scores_and_ambiguities() {
    compare(topics::EXPECTED, topics::CASES);
}

fn compare(expected_readings: &[&str], cases: &[(&str, usize)]) {
    let expected: Vec<Value> = expected_readings
        .iter()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    for &(prompt, index) in cases {
        let fields = expected[index].as_object().unwrap();
        let got = serde_json::to_value(rustbot::inspect_line(prompt)).unwrap();
        for (name, value) in fields {
            // Python serializes whole numbers without a decimal point.
            if name == "numbers" {
                let normalize = |list: &Value| {
                    list.as_array()
                        .unwrap()
                        .iter()
                        .map(|q| (q["value"].as_f64(), q["unit"].clone(), q["text"].clone()))
                        .collect::<Vec<_>>()
                };
                assert_eq!(normalize(&got[name]), normalize(value), "{prompt:?} {name}");
            } else {
                assert_eq!(got.get(name), Some(value), "{prompt:?} {name}");
            }
        }
    }
}

#[path = "fixtures/details.rs"]
mod details;
#[test]
fn matches_additional_reference_fields() {
    compare(details::EXPECTED, details::CASES);
}
