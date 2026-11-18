// Pinned proposals from PyBot 7d63c7f.
use super::*;
use crate::bot::Exchange;

type HistoryCases = (
    &'static [&'static str],
    &'static str,
    &'static str,
    &'static [(&'static str, Option<usize>)],
);

fn answers() -> Vec<&'static str> {
    data::PLACE_POOLS
        .iter()
        .flat_map(|&(_, pool)| pool.iter().copied())
        .chain(data::BAGGAGE_POOL.iter().copied())
        .collect()
}

fn conversation(users: &[&str], module: &'static str, reply: &str) -> Memory {
    Memory {
        turns: users
            .iter()
            .map(|&line| Exchange {
                user: line.into(),
                module,
                bot: reply.into(),
            })
            .collect(),
        ..Memory::default()
    }
}

#[test]
fn matches_authored_place_questions_and_abstentions() {
    let replies = answers();
    let cases: &[HistoryCases] = &[
        (
            &["my order hasnt come"],
            "greeting",
            "I don't know, sorry.",
            &[
                ("and how long", Some(0)),
                ("could i get another one", None),
                ("how much is it", Some(2)),
                ("what about this one", None),
                ("do they get one each", Some(2)),
                ("can we share it", Some(0)),
                ("can my kid get one", None),
                ("can i save anything", Some(1)),
                ("could we have one each", Some(1)),
                ("on thursday maybe", None),
                ("how do i clean it", None),
                ("what's the easiest way to clean it", None),
                ("is it ok to give water", None),
                ("where can i buy it", None),
                (
                    "do they get their own one in addition to the one i gave",
                    None,
                ),
                ("and the kids, do they get one each", None),
                ("are we all allowed to take one of these as well", None),
                ("Could you give me one? ", Some(2)),
                ("  AND   HOW\tLONG  ", Some(2)),
                ("and will it be okay?!", Some(1)),
                ("what about carry-on", Some(29)),
                ("what is the weight limit", Some(29)),
                ("my booking for hold baggage", None),
                ("the council has a luggage allowance", None),
                ("and can i book a new one", None),
                ("when can i use it", Some(0)),
                ("is it ok to put it back", Some(1)),
                ("where can i get seed", None),
                ("and could i change things", None),
                ("also can i still have it", Some(0)),
                (" +will they give it back?", Some(0)),
            ],
        ),
        (
            &["my orders have not come"],
            "greeting",
            "I don't know, sorry.",
            &[
                ("and how long", Some(0)),
                ("do they get one each", Some(2)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["it is still covered under guarantee"],
            "chat",
            "Could you say more?",
            &[
                ("and how long", Some(6)),
                ("do they get one each", Some(8)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["i lost the manual"],
            "greeting",
            "I don't know, sorry.",
            &[
                ("and how long", Some(9)),
                ("do they get one each", Some(11)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["the last train was cancelled"],
            "greeting",
            "I'm not sure.",
            &[
                ("and how long", Some(12)),
                ("do they get one each", Some(14)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["when does the tip close"],
            "greeting",
            "I don't know.",
            &[
                ("and how long", Some(15)),
                ("do they get one each", Some(17)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["it was an antihistamine"],
            "greeting",
            "I don't know, sorry.",
            &[
                ("and how long", Some(18)),
                ("do they get one each", Some(20)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["im on benefits"],
            "chat",
            "Could you say more?",
            &[
                ("and how long", Some(21)),
                ("do they get one each", Some(23)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["i got a note from my gp"],
            "greeting",
            "I don't know, sorry.",
            &[
                ("and how long", Some(24)),
                ("do they get one each", Some(26)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["one bag is 23 kilos"],
            "greeting",
            "Say more if you like.",
            &[
                ("and how long", Some(27)),
                ("do they get one each", Some(29)),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["the council said i need a permit"],
            "chat",
            "I see.",
            &[
                ("and how long", None),
                ("do they get one each", None),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["baggage and the council said i need a permit"],
            "chat",
            "I see.",
            &[
                ("and how long", None),
                ("do they get one each", None),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["my order hasnt come", "and how long"],
            "greeting",
            "Say more.",
            &[
                ("and how long", Some(2)),
                ("do they get one each", Some(2)),
                ("how do i clean it", None),
                ("what about carry-on", Some(28)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["my order hasnt come", "nice weather today"],
            "greeting",
            "Say more.",
            &[
                ("and how long", None),
                ("do they get one each", None),
                ("how do i clean it", None),
                ("what about carry-on", Some(28)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["my order hasnt come", "and how long", "and will it be ok"],
            "greeting",
            "Say more.",
            &[
                ("and how long", None),
                ("do they get one each", None),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["im on benefits"],
            "answers",
            "The restriction applies.",
            &[
                ("and how long", None),
                ("do they get one each", None),
                ("how do i clean it", None),
                ("what about carry-on", None),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["i asked nothing about it"],
            "greeting",
            "Your prescription is an NHS thing.",
            &[
                ("and how long", None),
                ("do they get one each", None),
                ("how do i clean it", None),
                ("what about carry-on", Some(29)),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &[],
            "",
            "",
            &[
                ("and how long", None),
                ("do they get one each", None),
                ("how do i clean it", None),
                ("what about carry-on", None),
                ("my booking for hold baggage", None),
            ],
        ),
        (
            &["my order hasnt come"],
            "greeting",
            "I don't know, sorry.",
            &[
                ("and can i save anything", Some(0)),
                ("\u{1c}AND HOW LONG\u{1c}", Some(2)),
                ("how much can i save again please", Some(2)),
                ("and it's there?", None),
                ("can itn go", Some(2)),
            ],
        ),
        (
            &["my order hasnt come", "and how long"],
            "greeting",
            "Say more.",
            &[
                ("and how long", Some(2)),
                ("can we share it", Some(0)),
                ("how much does it pay", None),
                ("will they be able to keep it", None),
            ],
        ),
    ];
    for &(users, module, reply, cases) in cases {
        let memory = conversation(users, module, reply);
        for &(line, expected) in cases {
            assert_eq!(
                respond(&Reading::new(line), &memory).as_deref(),
                expected.map(|at| replies[at]),
                "{users:?} {line:?}"
            );
        }
    }
}

#[test]
fn avoids_five_recent_exact_pool_replies_as_in_the_source() {
    let replies = answers();
    let cases: &[(&str, &str, &[usize], usize)] = &[
        ("my order hasnt come", "and how long", &[0], 1),
        ("my order hasnt come", "and how long", &[0, 1], 2),
        ("my order hasnt come", "and how long", &[0, 1, 2], 0),
        ("my order hasnt come", "and how long", &[2, 0, 1], 0),
        (
            "my order hasnt come",
            "and how long",
            &[0, 1, 1, 2, 1, 2],
            0,
        ),
        (
            "my order hasnt come",
            "and how long",
            &[2, 2, 2, 2, 2, 0],
            1,
        ),
        ("i lost the manual", "when can i use it", &[9], 10),
        ("i lost the manual", "when can i use it", &[9, 10], 11),
        ("i lost the manual", "when can i use it", &[9, 10, 11], 9),
        ("i lost the manual", "when can i use it", &[11, 9, 10], 9),
        (
            "i lost the manual",
            "when can i use it",
            &[9, 10, 10, 11, 10, 11],
            9,
        ),
        (
            "i lost the manual",
            "when can i use it",
            &[11, 11, 11, 11, 11, 9],
            10,
        ),
        ("one bag is 23 kilos", "do they get one each", &[27], 28),
        ("one bag is 23 kilos", "do they get one each", &[27, 28], 29),
        (
            "one bag is 23 kilos",
            "do they get one each",
            &[27, 28, 29],
            28,
        ),
        (
            "one bag is 23 kilos",
            "do they get one each",
            &[29, 27, 28],
            29,
        ),
        (
            "one bag is 23 kilos",
            "do they get one each",
            &[27, 28, 28, 29, 28, 29],
            27,
        ),
        (
            "one bag is 23 kilos",
            "do they get one each",
            &[29, 29, 29, 29, 29, 27],
            28,
        ),
    ];
    for &(past_line, current, past, expected) in cases {
        let turns = past
            .iter()
            .map(|&at| Exchange {
                user: past_line.into(),
                bot: replies[at].into(),
                module: "greeting",
            })
            .collect();
        let memory = Memory {
            turns,
            ..Memory::default()
        };
        assert_eq!(
            respond(&Reading::new(current), &memory).as_deref(),
            Some(replies[expected]),
            "{past_line:?} {current:?} {past:?}"
        );
    }
}
