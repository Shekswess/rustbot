//! Travel callback outcomes from the pinned PyBot reference.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "city-day-trip",
        "What day trips are there from Lisbon?",
        &[],
        Some(
            "Day trips from Lisbon: Sintra for the palaces (40 minutes by train), Cascais for the beach (40 minutes on the same line), and Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there). Buy the train ticket at the machine and go before 9am.",
        ),
    ),
    Case(
        "city-day-trip",
        "day trips from London",
        &[],
        Some(
            "Day trips from London: Bath (80 minutes by train), Oxford and Cambridge (about an hour), and Brighton (an hour) for the sea. Book the train a week ahead and you halve the fare.",
        ),
    ),
    Case(
        "city-day-trip",
        "Where should we go for a day trip from Bath?",
        &[],
        Some(
            "Day trips from Bath: Stonehenge (an hour by bus) with a booked slot, Lacock for the village and the abbey, and Bradford-on-Avon for the canal and the tithe barn, 15 minutes by train.",
        ),
    ),
    Case(
        "city-day-trip",
        "places to visit near Split",
        &[],
        Some(
            "Day trips from Split: Hvar by catamaran (an hour) for the old town and the lavender, Trogir (30 minutes by bus) for the walled centre, and Krka for the waterfalls, which needs a car or a tour.",
        ),
    ),
    Case(
        "city-day-trip",
        "can i get in the bath before a day trip?",
        &[],
        None,
    ),
    Case(
        "city-day-trip",
        "I'm thinking of a picnic in Cork",
        &[],
        None,
    ),
    Case(
        "city-day-trip",
        "Day trip from Copenhagen",
        &[],
        Some(
            "Day trips from Copenhagen: Roskilde for the cathedral and the Viking ships (25 minutes), Malmo across the bridge (35 minutes) for a cheaper lunch, and the Louisiana museum up the coast, which is the best half-day of the lot.",
        ),
    ),
    Case(
        "city-day-trip",
        "day trips from Bangkok and Amsterdam",
        &[],
        Some(
            "Day trips from Amsterdam: Haarlem (15 minutes), Utrecht (25 minutes) and the Zaanse Schans windmills (40 minutes). Keukenhof is worth it only in tulip season, mid-March to mid-May.",
        ),
    ),
    Case("city-day-trip", "day out from Riga", &[], None),
    Case(
        "city-day-trip",
        "day trip from Edinburgh",
        &[],
        Some(
            "Day trips from Edinburgh: Glasgow (50 minutes by train), Stirling for the castle, and North Berwick for the beach and the seabird centre. The Highlands need a car or a full day tour.",
        ),
    ),
    Case(
        "city-day-trip",
        "What day trips are there?",
        &["We are going to Lisbon."],
        Some(
            "Day trips from Lisbon: Sintra for the palaces (40 minutes by train), Cascais for the beach (40 minutes on the same line), and Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there). Buy the train ticket at the machine and go before 9am.",
        ),
    ),
    Case(
        "city-day-trip",
        "What day trips are there?",
        &["We are going to Lisbon.", "I like the sound of the train."],
        Some(
            "Day trips from Lisbon: Sintra for the palaces (40 minutes by train), Cascais for the beach (40 minutes on the same line), and Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there). Buy the train ticket at the machine and go before 9am.",
        ),
    ),
    Case(
        "city-day-trip",
        "What day trips are there?",
        &[
            "We are going to Lisbon.",
            "I like the sound of the train.",
            "That was all we asked.",
        ],
        None,
    ),
    Case(
        "city-day-trip",
        "What day trips are there?",
        &["I was going to split my sandwich.", "I went to Bath"],
        Some(
            "Day trips from Bath: Stonehenge (an hour by bus) with a booked slot, Lacock for the village and the abbey, and Bradford-on-Avon for the canal and the tithe barn, 15 minutes by train.",
        ),
    ),
    Case(
        "city-day-trip",
        "What day trips are there?",
        &["I was going to split my sandwich."],
        Some(
            "Day trips from Split: Hvar by catamaran (an hour) for the old town and the lavender, Trogir (30 minutes by bus) for the walled centre, and Krka for the waterfalls, which needs a car or a tour.",
        ),
    ),
    Case(
        "city-day-trip",
        "What day trips are there?",
        &["We booked a week in Paris", "Then we're off to Rome"],
        Some(
            "Day trips from Rome: Ostia Antica for the Roman ruins (30 minutes on the metro and train), Tivoli for the villa gardens, and Orvieto for the cathedral. Pompeii is two hours each way, so it is a full day.",
        ),
    ),
    Case(
        "city-airport",
        "What is the closest airport to Rome?",
        &[],
        Some(
            "Rome has two: Fiumicino (FCO), the main one, 30km south-west, and Ciampino (CIA) for budget flights, 15km east. From Fiumicino the Leonardo Express is 32 minutes to Termini for €14.",
        ),
    ),
    Case(
        "city-airport",
        "what airport should i use for Bologna",
        &[],
        None,
    ),
    Case(
        "city-airport",
        "what airport is closest to Lisbon",
        &[],
        Some(
            "Lisbon Humberto Delgado (LIS) is 7km north of the centre, and the red metro line takes 20 minutes for about €2 with a Viva card.",
        ),
    ),
    Case(
        "city-airport",
        "what is the airport called in Edinburgh",
        &[],
        Some(
            "Edinburgh (EDI) is 13km west, and the tram or the 100 bus to the centre takes about 30 minutes for £6.50.",
        ),
    ),
    Case(
        "city-airport",
        "what is the airport in Bath",
        &[],
        Some(
            "Bath has no airport: Bristol (BRS) is 30km west, and the Flyer bus runs to Bath in 55 minutes. By train, come into Bath Spa from Bristol Temple Meads.",
        ),
    ),
    Case(
        "city-airport",
        "is there an airport near Bath",
        &[],
        Some(
            "Bath has no airport: Bristol (BRS) is 30km west, and the Flyer bus runs to Bath in 55 minutes. By train, come into Bath Spa from Bristol Temple Meads.",
        ),
    ),
    Case("city-airport", "airport for getting to Bilbao", &[], None),
    Case(
        "city-airport",
        "do I need an airport for Bath mat",
        &[],
        None,
    ),
    Case(
        "city-airport",
        "What airport should I fly into?",
        &["I plan to visit Venice."],
        Some(
            "Venice has two: Marco Polo (VCE) for most flights, 13km north, and Treviso (TSF) for budget ones, 40km away. From Marco Polo the ATVO bus to Piazzale Roma is 20 minutes and €10.",
        ),
    ),
    Case(
        "city-airport",
        "What airport is closest?",
        &["I plan to visit Venice.", "I have booked a week off."],
        Some(
            "Venice has two: Marco Polo (VCE) for most flights, 13km north, and Treviso (TSF) for budget ones, 40km away. From Marco Polo the ATVO bus to Piazzale Roma is 20 minutes and €10.",
        ),
    ),
    Case(
        "city-airport",
        "What airport is closest?",
        &[
            "I plan to visit Venice.",
            "I have booked a week off.",
            "All sorted.",
        ],
        None,
    ),
    Case(
        "city-airport",
        "What is the airport called there?",
        &["I plan to visit Lisbon.", "I am also looking at Madrid."],
        Some(
            "Madrid-Barajas (MAD) is 12km north-east, and the metro line 8 into the centre is about €5 and 40 minutes; a taxi is a flat €33.",
        ),
    ),
    Case(
        "region-days",
        "what should we do in cornwall?",
        &[],
        Some(
            "In Cornwall: St Ives for the beach and the Tate, Padstow for the food and the Camel trail, the Eden Project's two biomes near St Austell, the Minack theatre cut into the cliff above Porthcurno, and Tintagel for the Arthur legend. The coast path between St Ives and Padstow is the day out that beats all of them.",
        ),
    ),
    Case(
        "region-days",
        "best beaches in cornwall",
        &[],
        Some(
            "Porthmeor at St Ives for surf and a town behind it, Kynance Cove on the Lizard for the water, and Sennen or Watergate Bay for space. Car parks are full by ten in August, so go early or go in the evening.",
        ),
    ),
    Case(
        "region-days",
        "best beaches in the lake district?",
        &[],
        None,
    ),
    Case(
        "region-days",
        "what are the best beaches near St Ives?",
        &[],
        Some(
            "Porthmeor at St Ives for surf and a town behind it, Kynance Cove on the Lizard for the water, and Sennen or Watergate Bay for space. Car parks are full by ten in August, so go early or go in the evening.",
        ),
    ),
    Case(
        "region-days",
        "what is there to do in padstow in september?",
        &[],
        None,
    ),
    Case(
        "region-days",
        "what is there to do in padstow?",
        &[],
        Some(
            "In Cornwall: St Ives for the beach and the Tate, Padstow for the food and the Camel trail, the Eden Project's two biomes near St Austell, the Minack theatre cut into the cliff above Porthcurno, and Tintagel for the Arthur legend. The coast path between St Ives and Padstow is the day out that beats all of them.",
        ),
    ),
    Case(
        "region-days",
        "what can I do in Skye in January?",
        &[],
        None,
    ),
    Case("region-days", "is Padstow worth a trip in May?", &[], None),
    Case(
        "region-days",
        "what to do in Dorset",
        &[],
        Some(
            "In Dorset: The Jurassic Coast: Lulworth Cove and Durdle Door for the walk, Chesil Beach and Portland, Swanage for the steam train to Corfe Castle, and Studland for the sand. The fossil beaches are west of Charmouth.",
        ),
    ),
    Case(
        "region-days",
        "best beaches in Bamburgh?",
        &[],
        Some(
            "Bamburgh for the castle above the sand, Embleton for the dunes, and Alnmouth for a village at the end of the beach.",
        ),
    ),
    Case(
        "region-days",
        "what should we do in Alnwick?",
        &[],
        Some(
            "In Northumberland: Hadrian's Wall at Housesteads and Vindolanda, Alnwick Castle and the garden, Bamburgh and its beach, and Lindisfarne across the causeway at low tide. The dark skies park is the evening.",
        ),
    ),
    Case(
        "region-days",
        "what are the beaches on the Isle of Wight?",
        &[],
        Some(
            "Sandown and Shanklin for sand and a promenade, Compton Bay for surf and the cliffs, and Bembridge for rock pools when the tide is out.",
        ),
    ),
    Case(
        "region-days",
        "what is there to do on the Isle of Wight?",
        &[],
        Some(
            "On the Isle of Wight: The Needles and Alum Bay, Shanklin and Ventnor for sand and the coastal path, Carisbrooke Castle, and the cycle path that goes round the island. It is small enough for a long weekend.",
        ),
    ),
    Case(
        "region-days",
        "what are some places to see on the Peak District?",
        &[],
        Some(
            "In the Peak District: Mam Tor and the Great Ridge for the classic day, Chatsworth for the house and the garden, Dovedale for the stepping stones, and Bakewell for the pudding. Edale is the station if you come by train.",
        ),
    ),
    Case(
        "region-days",
        "is the Cotswolds worth a visit",
        &[],
        Some(
            "In the Cotswolds: Bourton-on-the-Water for the river running through the village, Castle Combe for the street everyone photographs, Chipping Campden and Broadway for the stone, and the Slaughters for a walk between them. It is a place for driving and stopping rather than one long walk.",
        ),
    ),
    Case("region-days", "what do you do in cornwallis?", &[], None),
    Case("region-days", "things to do in Edinburgh", &[], None),
    Case("region-days", "Cornwall beach, Lake District", &[], None),
    Case(
        "region-days",
        "is there something to see in Exmoor?",
        &[],
        None,
    ),
    Case(
        "region-days",
        "anything good to do there?",
        &["We are staying in Cornwall."],
        None,
    ),
    Case(
        "get-to-place",
        "how do i get to the isle of wight",
        &[],
        None,
    ),
    Case(
        "get-to-place",
        "how can i get from here to the lake district",
        &[],
        None,
    ),
    Case(
        "get-to-place",
        "what is the best way to get to the amalfi coast",
        &[],
        Some(
            "Getting to the Amalfi coast: fly to Naples, then the ferry from Molo Beverello to Amalfi or Positano in season, or the Circumvesuviana train to Sorrento and the SITA bus along the coast. A private transfer is the only sensible option with heavy luggage.",
        ),
    ),
    Case("get-to-place", "how do i get from here to skye", &[], None),
    Case("get-to-place", "what's the way to the cotswolds", &[], None),
    Case(
        "get-to-place",
        "whats the easiest way to get to stonehenge?",
        &[],
        None,
    ),
    Case("get-to-place", "how do i get to Paris?", &[], None),
    Case(
        "get-to-place",
        "how i can reach Cornwall and the Isle of Wight",
        &[],
        None,
    ),
    Case(
        "get-to-place",
        "how can I get there?",
        &["I have planned a trip to the Isle of Wight."],
        None,
    ),
];
