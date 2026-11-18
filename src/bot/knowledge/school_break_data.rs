//! Text and regional keys from the pinned PyBot school-calendar cards.

pub(super) const NATIONS: &[(&[&str], Option<&str>)] = &[
    (
        &[
            "scotland",
            "scottish",
            "glasgow",
            "edinburgh",
            "aberdeen",
            "dundee",
            "inverness",
            "stirling",
            "fife",
            "paisley",
            "perthshire",
            "highlands",
            "shetland",
            "orkney",
        ],
        Some(
            "Scottish schools go back in the middle of August, usually between the 15th and the 20th, and break up at the end of June. The holidays are a week in mid-October, about two weeks at Christmas, a week or so at Easter and the long summer break from the last week of June.",
        ),
    ),
    (
        &[
            "wales",
            "welsh",
            "cardiff",
            "swansea",
            "newport",
            "wrexham",
            "bangor",
            "caernarfon",
            "bridgend",
            "carmarthen",
        ],
        Some(
            "Welsh schools usually go back in the first days of September and break up in the third week of July, with half term a week in late October and about two weeks at Christmas. Each council sets its own dates and they differ by a few days.",
        ),
    ),
    (
        &[
            "northern ireland",
            "northern irish",
            "belfast",
            "derry",
            "londonderry",
            "armagh",
            "lisburn",
            "newry",
            "antrim",
            "ulster",
        ],
        Some(
            "Schools in Northern Ireland go back in the first days of September and finish at the end of June, with half terms in late October and mid-February, two weeks at Christmas and a week or two at Easter.",
        ),
    ),
    (
        &[
            "ireland",
            "irish",
            "dublin",
            "cork",
            "galway",
            "limerick",
            "republic of ireland",
        ],
        Some(
            "Irish schools go back at the end of August or in the first days of September and finish at the end of June, with a week off in late October, two weeks at Christmas, a week in February and Easter week.",
        ),
    ),
    (
        &[
            "england",
            "english",
            "london",
            "manchester",
            "birmingham",
            "leeds",
            "bristol",
            "newcastle",
            "liverpool",
            "sheffield",
            "nottingham",
            "yorkshire",
            "kent",
            "essex",
            "cornwall",
            "surrey",
            "devon",
        ],
        None,
    ),
];

pub(super) const TERM_STARTS: &[(&[&str], &str)] = &[
    (
        &["summer term", "summer terms"],
        "The summer term starts in the second half of April, a week after the Easter break, and runs to the third week of July",
    ),
    (
        &["autumn term", "autumn terms", "winter term"],
        "The autumn term starts in the first days of September and runs to about 19 December",
    ),
    (
        &["spring term", "spring terms"],
        "The spring term starts on the first Tuesday or Wednesday in January and runs to the Easter break",
    ),
];

pub(super) const DATES_ARE_LOCAL: &str = " Every council, education authority and academy sets its own exact dates, so check the council's school term dates page or the school's own calendar for the day.";
pub(super) const SUMMER: &str = "English state schools break up in the third week of July and go back in the first days of September";
pub(super) const WINTER: &str =
    "The Christmas break runs from just before Christmas to the first days of January";
pub(super) const EASTER: &str = "The Easter break is the two weeks around Easter weekend: schools usually break up on the Friday a week before Good Friday and go back the Monday after Easter Monday. Easter moves each year, so the two weeks move with it - your school's term dates name the Friday it breaks up and the Monday it goes back.";
pub(super) const HALF_TERM: &str = "Half term is one week off: the last week of October, the middle of February and the last week of May. Which week it falls in is set by your school, and the three dates for the year are on its term dates page - the October one is usually the week the clocks go back.";
pub(super) const RETURN: &str = "The summer return is the first days of September, after Christmas it is the first Tuesday or Wednesday in January, and after Easter it is the Monday a week after Easter Monday. Your school's term dates give the exact day, and a school can add an inset day at the start of term.";
pub(super) const STARTS_LOCAL: &str = "Your school's term dates give the exact day, and a school can add an inset day at the start of term.";
