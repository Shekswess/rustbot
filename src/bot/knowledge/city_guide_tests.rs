//! Conditioned callback responses from PyBot 7d63c7f.
use super::*;

type Offer = fn(&Reading<'_>, &[Exchange]) -> Option<Cow<'static, str>>;

struct Case {
    offer: Offer,
    ask: &'static str,
    prior: &'static [(&'static str, &'static str)],
    reply: Option<&'static str>,
}

const CASES: &[Case] = &[
    Case {
        offer: guide,
        ask: "What should I see in Rome?",
        prior: &[],
        reply: Some(
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        ),
    },
    Case {
        offer: guide,
        ask: "We are going to Edinburgh in July",
        prior: &[],
        reply: Some(
            "In Edinburgh: the Royal Mile and the castle, Arthur's Seat for the view, and Leith for food.",
        ),
    },
    Case {
        offer: guide,
        ask: "I am going to Edinburgh in December",
        prior: &[],
        reply: Some(
            "In Edinburgh: the Royal Mile and the castle, Arthur's Seat for the view, and Leith for food. In December it's dark by 4pm and the Christmas market fills the centre.",
        ),
    },
    Case {
        offer: guide,
        ask: "We may go to Lisbon",
        prior: &[],
        reply: Some(
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        ),
    },
    Case {
        offer: guide,
        ask: "Should I book Rome?",
        prior: &[],
        reply: None,
    },
    Case {
        offer: guide,
        ask: "Is Venice worth visiting in August?",
        prior: &[],
        reply: None,
    },
    Case {
        offer: guide,
        ask: "What should I do after a hot bath?",
        prior: &[],
        reply: None,
    },
    Case {
        offer: guide,
        ask: "Is Bath worth visiting?",
        prior: &[],
        reply: Some(
            "In Bath: the Roman baths and the Pump Room, the Royal Crescent and the Circus, and a walk along the canal. It is small enough for a day, and the baths sell timed tickets that are worth booking the night before.",
        ),
    },
    Case {
        offer: guide,
        ask: "What should I eat in Rome?",
        prior: &[],
        reply: None,
    },
    Case {
        offer: guide,
        ask: "What should I eat in Naples?",
        prior: &[],
        reply: Some(
            "In Naples: the archaeology museum first, then pizza in the old town, and Pompeii or Herculaneum on a day trip by train. It is loud and fast, so keep bags in front of you and use the metro rather than the buses.",
        ),
    },
    Case {
        offer: guide,
        ask: "What should I see?",
        prior: &[(
            "I am going to Rome next week.",
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        )],
        reply: Some(
            "Day trips from Rome: Ostia Antica for the Roman ruins (30 minutes on the metro and train), Tivoli for the villa gardens, and Orvieto for the cathedral. Pompeii is two hours each way, so it is a full day.",
        ),
    },
    Case {
        offer: guide,
        ask: "What should I see visiting Rome",
        prior: &[(
            "I am going to Rome next week.",
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        )],
        reply: Some(
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        ),
    },
    Case {
        offer: guide,
        ask: "What should I see in Lisbon?",
        prior: &[(
            "What should I see in Lisbon?",
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        )],
        reply: Some(
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        ),
    },
    Case {
        offer: guide,
        ask: "What should I see?",
        prior: &[],
        reply: None,
    },
    Case {
        offer: next,
        ask: "What should I see?",
        prior: &[(
            "I am going to Rome next week.",
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        )],
        reply: Some(
            "Day trips from Rome: Ostia Antica for the Roman ruins (30 minutes on the metro and train), Tivoli for the villa gardens, and Orvieto for the cathedral. Pompeii is two hours each way, so it is a full day.",
        ),
    },
    Case {
        offer: next,
        ask: "what should i eat",
        prior: &[(
            "I am staying in Lisbon for the weekend.",
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        )],
        reply: Some(
            "In Lisbon a bifana or a pastel de nata at a counter costs a couple of euros, and the prato do dia at lunch in Alfama or Graça is the full cheap meal. Drink at a kiosk rather than a terrace, and the Time Out Market is the easy one-stop version.",
        ),
    },
    Case {
        offer: next,
        ask: "What should I see first?",
        prior: &[(
            "I am going to Rome next week.",
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        )],
        reply: Some("Start with the Colosseum with a timed ticket, then the Pantheon."),
    },
    Case {
        offer: next,
        ask: "what about museums",
        prior: &[(
            "I am going to Rome next week.",
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        )],
        reply: Some(
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner.",
        ),
    },
    Case {
        offer: next,
        ask: "what should i see",
        prior: &[(
            "I am staying in Lisbon for the weekend.",
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        )],
        reply: Some(
            "In Lisbon: by area, Alfama for the old streets and the views but steep and noisy, Baixa and Chiado to be in the middle of it, or Principe Real for quiet and smart.",
        ),
    },
    Case {
        offer: next,
        ask: "what should i see",
        prior: &[
            (
                "I am staying in Lisbon for the weekend.",
                "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
            ),
            (
                "And what is there to do",
                "In Lisbon: by area, Alfama for the old streets and the views but steep and noisy, Baixa and Chiado to be in the middle of it, or Principe Real for quiet and smart.",
            ),
        ],
        reply: Some(
            "Day trips from Lisbon: Sintra for the palaces (40 minutes by train), Cascais for the beach (40 minutes on the same line), and Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there). Buy the train ticket at the machine and go before 9am.",
        ),
    },
    Case {
        offer: next,
        ask: "What should we see",
        prior: &[
            (
                "I am going to Edinburgh in June.",
                "In Edinburgh: the Royal Mile and the castle, Arthur's Seat for the view, and Leith for food. In December it's dark by 4pm and the Christmas market fills the centre.",
            ),
            ("And the museum?", "I know what you mean."),
        ],
        reply: Some(
            "In Edinburgh: by area, the New Town for Georgian calm and everything on foot, the Old Town for the Royal Mile and tourists, or Leith for the food.",
        ),
    },
    Case {
        offer: next,
        ask: "What should we do when we step outside the door beside this big parking lot",
        prior: &[(
            "I am going to Rome next week.",
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        )],
        reply: None,
    },
    Case {
        offer: next,
        ask: "What can I see in Lisbon?",
        prior: &[(
            "I am going to Rome next week.",
            "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        )],
        reply: None,
    },
    Case {
        offer: followup,
        ask: "what should i see there",
        prior: &[(
            "What should I see in Lisbon?",
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        )],
        reply: Some(
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        ),
    },
    Case {
        offer: followup,
        ask: "what should i see there",
        prior: &[(
            "I am staying in Lisbon for the weekend.",
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        )],
        reply: Some(
            "In Lisbon: by area, Alfama for the old streets and the views but steep and noisy, Baixa and Chiado to be in the middle of it, or Principe Real for quiet and smart.",
        ),
    },
    Case {
        offer: followup,
        ask: "what about the food",
        prior: &[(
            "I am staying in Lisbon for the weekend.",
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        )],
        reply: Some(
            "In Lisbon a bifana or a pastel de nata at a counter costs a couple of euros, and the prato do dia at lunch in Alfama or Graça is the full cheap meal. Drink at a kiosk rather than a terrace, and the Time Out Market is the easy one-stop version.",
        ),
    },
    Case {
        offer: followup,
        ask: "where is the nearest museum",
        prior: &[(
            "I am staying in Lisbon for the weekend.",
            "In Lisbon: Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        )],
        reply: None,
    },
    Case {
        offer: followup,
        ask: "what should i see there",
        prior: &[],
        reply: None,
    },
];

#[test]
fn city_guide_callbacks_match_the_pinned_reference() {
    for case in CASES {
        let earlier: Vec<_> = case
            .prior
            .iter()
            .map(|&(user, bot)| Exchange {
                user: user.into(),
                bot: bot.into(),
                module: "answers",
            })
            .collect();
        let got = (case.offer)(&Reading::new(case.ask), &earlier);
        assert_eq!(
            got.as_deref(),
            case.reply,
            "{} after {:?}",
            case.ask,
            case.prior
        );
    }
}

#[test]
fn city_meals_match_the_authored_guard_and_rendering() {
    let cases = [
        (
            "Where should I eat in Lisbon?",
            None,
            true,
            Some(
                "In Lisbon a bifana or a pastel de nata at a counter costs a couple of euros, and the prato do dia at lunch in Alfama or Graça is the full cheap meal. Drink at a kiosk rather than a terrace, and the Time Out Market is the easy one-stop version.",
            ),
        ),
        (
            "Best value in Madrid?",
            None,
            false,
            Some(
                "In Madrid the menú del día at lunch is the cheap meal, and tapas around La Latina or Lavapiés are a few euros each, with the drink included in most bars. Standing at the bar is cheaper than a table, and dinner starts late, so lunch is the meal to spend on.",
            ),
        ),
        (
            "What should I eat in Naples?",
            None,
            true,
            Some(
                "Three habits make any city cheap to eat in: take the fixed lunch rather than dinner - menu del día, plat du jour, prato do dia, Mittagsmenü - which is the same kitchen at half the price; eat where people work and shop, at a market hall, a bakery counter or a busy bar, rather than where they sightsee; and stand up to eat, because pizza by the slice, a kebab, a bowl of noodles at a counter or a picnic from a supermarket costs a fraction of a table with a view. Coffee and breakfast at a bar, and tap water in a bottle you refill, save the rest.",
            ),
        ),
        (
            "How much does a meal cost in Rome?",
            None,
            true,
            Some(
                "In Rome the cheap meal is pizza al taglio, sold by weight and eaten standing, or a panino from a bakery counter; supplì and a slice of pizza bianca are a couple of euros. Eat in Testaccio or Trastevere rather than beside a monument, and take your coffee standing at the bar, where it costs a third of the table price.",
            ),
        ),
        (
            "Cheapest way from London to Manchester?",
            None,
            false,
            Some(
                "Three habits make any city cheap to eat in: take the fixed lunch rather than dinner - menu del día, plat du jour, prato do dia, Mittagsmenü - which is the same kitchen at half the price; eat where people work and shop, at a market hall, a bakery counter or a busy bar, rather than where they sightsee; and stand up to eat, because pizza by the slice, a kebab, a bowl of noodles at a counter or a picnic from a supermarket costs a fraction of a table with a view. Coffee and breakfast at a bar, and tap water in a bottle you refill, save the rest.",
            ),
        ),
        ("Where should I eat there?", None, true, None),
        (
            "Where should I eat there?",
            Some("I am going to Rome next week."),
            true,
            Some(
                "In Rome the cheap meal is pizza al taglio, sold by weight and eaten standing, or a panino from a bakery counter; supplì and a slice of pizza bianca are a couple of euros. Eat in Testaccio or Trastevere rather than beside a monument, and take your coffee standing at the bar, where it costs a third of the table price.",
            ),
        ),
    ];
    for (line, prior, guard, wanted) in cases {
        let earlier = prior.map(|line| Exchange {
            user: line.into(),
            bot: String::new(),
            module: "answers",
        });
        let reading = Reading::new(line);
        assert_eq!(is_eating(&reading), guard, "{line}");
        assert_eq!(
            cheap_eats(&reading, earlier.as_slice()).as_deref(),
            wanted,
            "{line}"
        );
    }
}
