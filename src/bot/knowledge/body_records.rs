//! Human anatomy records, selected by the named part and the requested fact.

use std::borrow::Cow;

use crate::bot::Reading;

use super::choosing::names;

struct Record {
    parts: &'static [&'static str],
    asks: &'static [&'static str],
    answer: &'static str,
}

const ANIMALS: &[&str] = &[
    "horse", "horses", "cow", "cows", "sheep", "pig", "pigs", "dog", "dogs", "cat", "cats",
    "puppy", "puppies", "kitten", "kittens", "rabbit", "rabbits", "hamster", "bird", "birds",
    "fish", "paw", "paws", "beak", "beaks", "tail", "tails", "animal", "animals", "pet", "pets",
];
const GENERIC: &[&str] = &["substance", "thing", "part", "material"];
const BODY: &[&str] = &["body", "bodies", "human", "humans", "animal", "mammal"];
const FINGERS: &[&str] = &[
    "finger",
    "fingers",
    "fingertip",
    "fingertips",
    "hand",
    "hands",
    "thumb",
];
const BONES: &[&str] = &["bone", "bones", "thigh bone", "femur"];
const MUSCLES: &[&str] = &["muscle", "muscles"];

const RECORDS: &[Record] = &[
    Record {
        parts: BONES,
        asks: &["strongest", "sturdiest"],
        answer: "The femur, or thigh bone, is generally described as the strongest human bone. It supports body weight and transmits forces between the hip and knee. It's also the longest human bone.",
    },
    Record {
        parts: BONES,
        asks: &["biggest", "largest", "heaviest", "thickest"],
        answer: "The femur, or thigh bone, is the largest and heaviest human bone. It runs from the hip to the knee. In many adults it's around 45 centimetres long, depending on height.",
    },
    Record {
        parts: BONES,
        asks: &["longest", "tallest", "length"],
        answer: "The longest human bone is the femur in the thigh. In many adults it's around 45 centimetres long. The smallest is the stapes in the middle ear, just a few millimetres long.",
    },
    Record {
        parts: &["bone", "bones", "ear"],
        asks: &[
            "smallest",
            "tiniest",
            "littlest",
            "shortest",
            "smallest bone",
        ],
        answer: "The stapes is the smallest human bone, only a few millimetres long. It is one of three tiny bones in the middle ear that pass vibrations from the eardrum to the inner ear.",
    },
    Record {
        parts: &[
            "substance",
            "thing",
            "part",
            "material",
            "enamel",
            "tooth enamel",
        ],
        asks: &["hardest", "toughest", "strongest"],
        answer: "Tooth enamel, the outer layer of a tooth, is the hardest material in the human body. Saliva and fluoride can help restore minerals lost in early decay. Once there's a cavity, enamel cannot grow back to fill it.",
    },
    Record {
        parts: &["teeth", "tooth", "adult teeth", "milk teeth"],
        asks: &[
            "how many", "many", "number", "count", "adults", "adult", "children", "kids", "baby",
            "people",
        ],
        answer: "A full adult set has 32 permanent teeth, including four wisdom teeth. Some people have fewer. Children typically have 20 primary teeth, often called milk or baby teeth.",
    },
    Record {
        parts: MUSCLES,
        asks: &["longest", "length"],
        answer: "The sartorius is the longest muscle in the human body. It runs diagonally down the front of your thigh from the hip to the inside of the knee and helps move both joints.",
    },
    Record {
        parts: MUSCLES,
        asks: &["strongest"],
        answer: "It depends on how you measure strength. The jaw's masseter can produce a large force for its size. The gluteus maximus is a large, powerful hip muscle that helps you stand up and climb stairs.",
    },
    Record {
        parts: MUSCLES,
        asks: &["biggest", "largest", "heaviest"],
        answer: "The gluteus maximus, the large muscle in each buttock, is usually described as the largest human muscle. It helps straighten the hip when you stand up, climb stairs or run.",
    },
    Record {
        parts: MUSCLES,
        asks: &["how many", "many", "number", "count"],
        answer: "The human body has more than 600 muscles. There isn't a single exact count, because different references count some parts of muscles separately.",
    },
    Record {
        parts: &[
            "fingerprint",
            "fingerprints",
            "fingertips",
            "finger tips",
            "prints",
        ],
        asks: &["why", "what for", "what are", "purpose", "for", "how"],
        answer: "Fingerprints are the patterns made by the ridges on your fingertips. They form before birth. Experiments show the ridges can help regulate moisture where your finger contacts a surface, which can help you keep your grip.",
    },
];

fn answer(text: &str) -> Option<&'static str> {
    if names(text, ANIMALS) && !names(text, &["human"]) {
        return None;
    }
    let human = names(text, BODY);
    RECORDS.iter().find_map(|record| {
        let mut parts = record
            .parts
            .iter()
            .copied()
            .filter(|part| names(text, &[*part]));
        let first = parts.next()?;
        let specific =
            !GENERIC.contains(&first) || parts.clone().any(|part| !GENERIC.contains(&part));
        if !(specific || human) || !names(text, record.asks) {
            return None;
        }
        let prints = first == "prints" || parts.any(|part| part == "prints");
        (!prints || names(text, FINGERS)).then_some(record.answer)
    })
}

pub(crate) fn reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    answer(&reading.clean).map(Cow::Borrowed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_require_the_human_part_and_the_requested_fact() {
        assert!(
            answer("what is the longest bone")
                .unwrap()
                .contains("femur")
        );
        assert!(
            answer("what is the strongest muscle")
                .unwrap()
                .contains("how you measure")
        );
        assert!(
            answer("what are fingerprints for")
                .unwrap()
                .contains("grip")
        );
        assert!(answer("how many teeth does a horse have").is_none());
        assert!(answer("what is the hardest thing about being a parent").is_none());
    }
}
