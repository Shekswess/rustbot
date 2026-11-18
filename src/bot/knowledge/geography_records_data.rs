// Geographic records from the pinned PyBot geography and travel tables.
pub(super) const WORLD: &[(&str, &str)] = &[
    (
        "countries-count",
        "The usual answer is 195: 193 members of the United Nations plus the Vatican and Palestine, which are observers. Some lists count Taiwan or Kosovo, which is why you sometimes see 196 or 197.",
    ),
    (
        "oceans-count",
        "There are five oceans: Pacific, Atlantic, Indian, Southern and Arctic. The Pacific is the biggest and deepest; its Mariana Trench goes down about 11 km, deeper than Everest is tall.",
    ),
    (
        "ocean-depth",
        "The ocean averages about 3.7 km deep. The deepest point is the Mariana Trench in the Pacific, at about 10.9 km down - if Everest stood there, its summit would still be more than 2 km under the surface.",
    ),
    (
        "continents-count",
        "Seven is the usual count: Africa, Antarctica, Asia, Europe, North America, Oceania and South America. Some countries teach six, joining Europe and Asia into Eurasia.",
    ),
    (
        "world-population",
        "About 8.2 billion people, and it is still rising, though more slowly. India passed China as the most populous country in 2023, and both have around 1.4 billion.",
    ),
    (
        "largest-island",
        "Greenland is the largest island, at about 2.2 million square kilometres. Australia is bigger but is counted as a continent rather than an island, which is the usual quiz gotcha.",
    ),
    (
        "largest-continent",
        "Asia is the largest continent, at about 44.6 million square kilometres - roughly 30% of the world's land. Africa is second and Antarctica third.",
    ),
    (
        "smallest-continent",
        "Australia is the smallest continent, at about 8.6 million square kilometres. Some lists call the region Oceania, which also includes New Zealand and the Pacific islands, but Australia is the continent itself.",
    ),
    (
        "highest-waterfall",
        "Angel Falls in Venezuela is the highest waterfall, at 979 metres. Its water drops in one unbroken fall of about 807 metres, so it is still falling long after Niagara would have hit the bottom.",
    ),
    (
        "longest-mountain-range",
        "The Andes is the longest mountain range above sea level, running about 7,000 km down the west of South America. The Mid-Atlantic Ridge is longer, but almost all of it is under the sea.",
    ),
    (
        "hottest-place",
        "Death Valley in California holds the highest air temperature reliably recorded: 56.7C in 1913. The Danakil Depression in Ethiopia and Dasht-e Lut in Iran are often called the hottest places on average, because they are hot for longer.",
    ),
    (
        "coldest-place",
        "Antarctica is the coldest place on Earth: Vostok Station recorded -89.2C in 1983. The coldest place with people living in it is Oymyakon in Siberia, where winter temperatures reach about -50C.",
    ),
    (
        "most-people-country",
        "India has the most people, at about 1.45 billion, just ahead of China at about 1.41 billion since 2023. The United States is a distant third, at about 340 million.",
    ),
];
pub(super) const REGION_RECORD: &[(&str, &str, &str)] = &[
    (
        "highest-mountain",
        "mountain",
        "Mount Everest is the highest, at 8,849 metres (29,032 feet) above sea level. Measured from the base to the summit, Mauna Kea in Hawaii is taller, because most of it is under the sea.",
    ),
    (
        "largest-desert",
        "desert",
        "Antarctica is technically the largest desert, because a desert is defined by how little it rains. The Sahara is the largest hot desert, at about 9 million square kilometres, roughly the size of the United States.",
    ),
    (
        "largest-lake",
        "lake",
        "The Caspian Sea is the largest lake, at about 371,000 square kilometres, and it is salty. Lake Superior is the largest freshwater lake by area; Lake Baikal holds the most fresh water.",
    ),
    (
        "deepest-lake",
        "deep lake",
        "Lake Baikal in Siberia is the deepest lake, at 1,642 metres. It holds about a fifth of the world's unfrozen fresh water, more than all the Great Lakes together.",
    ),
    (
        "longest-river",
        "river",
        "The Nile, at about 6,650 km, is usually called the longest, with the Amazon a close second and carrying far more water. Which one wins depends on where you say the Amazon starts.",
    ),
];
pub(super) const COUNTRY_RECORD: &[(&str, &str, &str)] = &[
    (
        "smallest-country",
        "small",
        "Vatican City is the smallest country, at about 0.49 square kilometres and roughly 800 people. Monaco is the second smallest, and both are smaller than most city parks.",
    ),
    (
        "biggest-country",
        "area",
        "Russia, at about 17 million square km, nearly twice the size of Canada in second place. China and the United States come next.",
    ),
];
pub(super) const REGION_NAMES: &[(&str, &str)] = &[
    ("north american", "north america"),
    ("south american", "south america"),
    ("north america", "north america"),
    ("south america", "south america"),
    ("african", "africa"),
    ("africa", "africa"),
    ("european", "europe"),
    ("europe", "europe"),
    ("asian", "asia"),
    ("asia", "asia"),
    ("oceania", "oceania"),
    ("antarctica", "antarctica"),
];
pub(super) const PLACES: &[&str] = &[
    "north america",
    "south america",
    "central america",
    "caribbean",
    "africa",
    "europe",
    "asia",
    "oceania",
    "antarctica",
];
pub(super) const REGION_FACTS: &[(&str, &[(&str, &str)])] = &[
    (
        "africa",
        &[
            (
                "river",
                "The Nile, at about 6,650 km - the longest river in Africa and usually counted the longest in the world. The Congo carries far more water.",
            ),
            (
                "mountain",
                "Kilimanjaro, at 5,895 m, in Tanzania - the highest mountain in Africa, and a walk-up rather than a climb, though the altitude is the hard part.",
            ),
            (
                "desert",
                "The Sahara, at about 9 million square km - the largest hot desert in the world, roughly the size of the United States.",
            ),
        ],
    ),
    (
        "europe",
        &[
            (
                "river",
                "The Volga, at about 3,530 km, running through Russia to the Caspian Sea. The Danube is second, at about 2,850 km, and passes through ten countries.",
            ),
            (
                "mountain",
                "Mount Elbrus, at 5,642 m, in the Russian Caucasus - the highest in Europe and one of the seven summits.",
            ),
        ],
    ),
    (
        "asia",
        &[
            (
                "river",
                "The Yangtze, at about 6,300 km - the longest river wholly in Asia. Russia's Ob and Yenisei are longer but are only partly in Asia.",
            ),
            (
                "mountain",
                "Mount Everest, at 8,849 m - the highest in Asia and in the world.",
            ),
            (
                "desert",
                "The Arabian Desert, at about 2.3 million square km. The Gobi is the biggest cold desert, and the Sahara is the biggest hot one - it is in Africa.",
            ),
        ],
    ),
    (
        "north america",
        &[
            (
                "river",
                "The Missouri-Mississippi, at about 6,275 km from source to the Gulf of Mexico. The Mississippi alone is about 3,770 km.",
            ),
            (
                "mountain",
                "Denali, at 6,190 m, in Alaska. Mount Logan in Canada is second, at 5,959 m.",
            ),
            (
                "desert",
                "The Great Basin Desert, at about 490,000 square km, in the western United States. The Chihuahuan, shared with Mexico, is the biggest hot one.",
            ),
        ],
    ),
    (
        "south america",
        &[
            (
                "river",
                "The Amazon, at about 6,400 km, and far and away the biggest by water - about a fifth of all the river water on Earth.",
            ),
            (
                "mountain",
                "Aconcagua, at 6,961 m, in Argentina - the highest mountain outside Asia.",
            ),
            (
                "desert",
                "The Patagonian Desert, at about 670,000 square km. The Atacama is the driest desert, not the biggest.",
            ),
        ],
    ),
    (
        "oceania",
        &[
            (
                "river",
                "The Murray, at about 2,508 km, with the Darling - the longest river system in Oceania, running through Australia.",
            ),
            (
                "mountain",
                "Puncak Jaya, at 4,884 m, in Indonesian Papua. Australia's own highest is Mount Kosciuszko, at 2,228 m.",
            ),
            (
                "desert",
                "The Great Victoria Desert, at about 348,000 square km, in Australia.",
            ),
        ],
    ),
    (
        "uk",
        &[
            (
                "river",
                "The Severn, at about 354 km (220 miles), is the longest river in the UK, rising in Wales and reaching the sea past Bristol. The Thames is second at about 346 km.",
            ),
            (
                "lake",
                "Lough Neagh in Northern Ireland is the largest lake in the UK by area, at about 383 square km. Loch Ness holds the most water, about 7.5 cubic km.",
            ),
            (
                "deep lake",
                "Loch Morar is the deepest lake in Britain, at 310 metres, deeper than the North Sea is in most places. Loch Ness is 230 metres but holds far more water.",
            ),
            (
                "desert",
                "There are no deserts in the UK: nowhere here is dry enough to count as one. The closest thing is a dune system such as the Sefton coast or Braunton Burrows.",
            ),
        ],
    ),
    (
        "britain",
        &[
            (
                "river",
                "The Severn, at about 354 km, is the longest river in Britain, with the Thames second at about 346 km and the Trent third.",
            ),
            (
                "lake",
                "Loch Lomond, at about 71 square km, is the largest lake in Britain by area. Windermere is the largest in England at about 14.7 square km.",
            ),
            (
                "deep lake",
                "Loch Morar, at 310 metres, is the deepest lake in Britain. Loch Ness is second at 230 metres and holds more water than all the lakes of England and Wales together.",
            ),
        ],
    ),
    (
        "england",
        &[
            (
                "river",
                "The Severn, at about 354 km (220 miles), is the longest river in England, though it rises in Wales. The Thames is second at about 346 km, and the longest river wholly inside England.",
            ),
            (
                "lake",
                "Windermere is the largest lake in England, about 14.7 square km and 18 km long. Wastwater is the deepest, at 74 metres, and was voted Britain's favourite view.",
            ),
            (
                "deep lake",
                "Wastwater in the Lake District is the deepest lake in England, at 74 metres, and the coldest: it is a long narrow trench under the Screes.",
            ),
        ],
    ),
    (
        "scotland",
        &[
            (
                "river",
                "The Tay, at about 188 km (117 miles), is the longest river in Scotland and the biggest by water, carrying more than the Thames and the Severn together.",
            ),
            (
                "lake",
                "Loch Lomond is the largest lake in Scotland by area, about 71 square km. Loch Ness is bigger by volume and holds more fresh water than all of England and Wales put together.",
            ),
            (
                "deep lake",
                "Loch Morar in the west Highlands is the deepest lake in Scotland and in Britain, at 310 metres.",
            ),
        ],
    ),
    (
        "wales",
        &[
            (
                "river",
                "The Severn rises in the Welsh hills and is the longest river to run through Wales, at about 354 km. The longest river wholly inside Wales is the Tywi, at about 120 km.",
            ),
            (
                "lake",
                "Llyn Tegid, also called Bala Lake, is the largest natural lake in Wales at about 4.8 square km, and it has its own species of whitefish, the gwyniad.",
            ),
        ],
    ),
    (
        "northern ireland",
        &[
            (
                "river",
                "The Bann, at about 129 km, is the longest river in Northern Ireland: the Upper Bann feeds Lough Neagh and the Lower Bann carries it to the sea at Coleraine.",
            ),
            (
                "lake",
                "Lough Neagh is the largest lake in Northern Ireland and in the British Isles, at about 383 square km, and it supplies much of Belfast's drinking water.",
            ),
        ],
    ),
    (
        "ireland",
        &[
            (
                "river",
                "The Shannon, at about 360 km, is the longest river in Ireland, running from the Cavan hills south to Limerick and the Atlantic.",
            ),
            (
                "lake",
                "Lough Neagh, in Northern Ireland, is the largest lake on the island of Ireland at about 383 square km. The largest in the Republic is Lough Corrib, about 176 square km, in County Galway.",
            ),
        ],
    ),
    (
        "antarctica",
        &[
            (
                "river",
                "There are no real rivers: the Onyx is a seasonal meltwater stream about 30 km long, and it is the longest in Antarctica.",
            ),
            (
                "mountain",
                "Vinson Massif, at 4,892 m - the highest in Antarctica, and the coldest of the seven summits.",
            ),
            (
                "desert",
                "Antarctica is itself the largest desert on Earth, at about 14 million square km: a desert is about how little it rains, not about sand.",
            ),
        ],
    ),
];
pub(super) const COUNTRY_FACTS: &[super::CountryRecord] = &[
    (
        "africa",
        &[
            (
                "area",
                "Algeria is the largest country in Africa by area, at about 2.4 million square km - most of it Sahara. The Democratic Republic of the Congo is second.",
            ),
            (
                "small",
                "The Seychelles is the smallest African country by area, an island nation of about 115 islands. If you mean the smallest on the mainland, it is The Gambia.",
            ),
            (
                "populous",
                "Nigeria has the most people in Africa, about 230 million. Algeria is the biggest by area and the Seychelles is the smallest country.",
            ),
            (
                "least",
                "The Seychelles has the fewest people in Africa, about 100,000, and it is the smallest country there by area too.",
            ),
            (
                "top",
                "The three biggest countries in Africa by area are Algeria (2.4 million square km), DR Congo (2.3 million) and Sudan (1.9 million). By people it is Nigeria first, then Ethiopia and Egypt.",
            ),
            (
                "bottom",
                "The three smallest countries in Africa by area are the Seychelles (455 square km), São Tomé and Príncipe (about 1,000) and Comoros (about 1,900).",
            ),
        ],
        &["Nigeria", "Ethiopia", "Egypt", "DR Congo", "South Africa"],
    ),
    (
        "europe",
        &[
            (
                "area",
                "Ukraine is the largest country wholly inside Europe, at about 600,000 square km; European Russia is bigger but is usually counted separately.",
            ),
            (
                "small",
                "Vatican City is the smallest country in Europe, and in the world, at about 0.49 square km. Monaco is second, at about 2 square km.",
            ),
            (
                "populous",
                "Russia has the most people in Europe, about 110 million in its European part, and Germany the most of any country entirely in Europe, about 84 million. Vatican City is the smallest.",
            ),
            (
                "least",
                "Vatican City has the fewest people in Europe, a few hundred, and it is the smallest by area too.",
            ),
            (
                "top",
                "The three biggest countries in Europe by area are Russia's European part, Ukraine (604,000 square km) and France (551,000, without its overseas parts). By people it is Russia first, then Germany and the United Kingdom.",
            ),
            (
                "bottom",
                "The three smallest countries in Europe by area are Vatican City (half a square km), Monaco (about 2) and San Marino (61).",
            ),
        ],
        &["Russia", "Germany", "United Kingdom", "France", "Italy"],
    ),
    (
        "asia",
        &[
            (
                "area",
                "China is the largest country in Asia, at about 9.6 million square km. India is second; Russia is bigger but is counted as transcontinental.",
            ),
            (
                "small",
                "The Maldives is the smallest country in Asia, at about 300 square km spread over some 1,200 islands.",
            ),
            (
                "populous",
                "India has the most people in Asia, about 1.45 billion. China is the biggest country entirely in Asia and the Maldives is the smallest.",
            ),
            (
                "least",
                "Brunei has the fewest people in Asia, about 455,000, though the Maldives is smaller by area.",
            ),
            (
                "top",
                "The three biggest countries in Asia by area are China (9.6 million square km), India (3.3 million) and Kazakhstan (2.7 million). By people it is India first, then China and Indonesia.",
            ),
            (
                "bottom",
                "The three smallest countries in Asia by area are the Maldives (about 300 square km), Singapore (about 730) and Bahrain (about 780).",
            ),
        ],
        &["India", "China", "Indonesia", "Pakistan", "Bangladesh"],
    ),
    (
        "north america",
        &[
            (
                "area",
                "Canada is the largest country in North America, at about 10 million square km; the United States is a close second.",
            ),
            (
                "small",
                "Saint Kitts and Nevis is the smallest country in North America, at about 260 square km. Grenada and Saint Vincent are close behind.",
            ),
            (
                "populous",
                "The United States has the most people in North America, about 340 million. Canada is the biggest by area and Saint Kitts and Nevis is the smallest.",
            ),
            (
                "least",
                "Saint Kitts and Nevis has the fewest people in North America, about 47,000, and it is the smallest by area too.",
            ),
            (
                "top",
                "The three biggest countries in North America by area are Canada (10 million square km), the United States (9.8 million) and Mexico (1.96 million). By people it is the United States first, then Mexico and Canada.",
            ),
            (
                "bottom",
                "The three smallest countries in North America by area are Saint Kitts and Nevis (about 260 square km), Grenada (about 340) and Saint Vincent and the Grenadines (about 390).",
            ),
        ],
        &["United States", "Mexico", "Canada", "Guatemala", "Haiti"],
    ),
    (
        "central america",
        &[
            (
                "area",
                "Nicaragua is the largest country in Central America, at about 130,000 square km; Honduras and Guatemala come next.",
            ),
            (
                "small",
                "El Salvador is the smallest country in Central America, at about 21,000 square km, and the most densely populated on the mainland.",
            ),
        ],
        &[],
    ),
    (
        "south america",
        &[
            (
                "area",
                "Brazil is the largest country in South America, at about 8.5 million square km - nearly half the continent. Argentina is second.",
            ),
            (
                "small",
                "Suriname is the smallest country in South America, at about 164,000 square km. Uruguay and Guyana are next.",
            ),
            (
                "populous",
                "Brazil has the most people in South America, about 215 million. It is the biggest by area too, and Suriname is the smallest.",
            ),
            (
                "least",
                "Suriname has the fewest people in South America, about 620,000, and it is the smallest by area too.",
            ),
            (
                "top",
                "The three biggest countries in South America by area are Brazil (8.5 million square km), Argentina (2.8 million) and Peru (1.3 million). By people it is Brazil first, then Colombia and Argentina.",
            ),
            (
                "bottom",
                "The three smallest countries in South America by area are Suriname (164,000 square km), Uruguay (176,000) and Guyana (215,000).",
            ),
        ],
        &["Brazil", "Colombia", "Argentina", "Peru", "Venezuela"],
    ),
    (
        "oceania",
        &[
            (
                "area",
                "Australia is by far the largest country in Oceania, at about 7.7 million square km; Papua New Guinea is a distant second.",
            ),
            (
                "small",
                "Nauru is the smallest country in Oceania, at about 21 square km - the smallest island country in the world. Tuvalu is next.",
            ),
            (
                "populous",
                "Australia has the most people in Oceania, about 26 million. It is the biggest by area too, and Nauru is the smallest.",
            ),
            (
                "least",
                "Nauru has the fewest people in Oceania, about 12,000, and it is the smallest by area too.",
            ),
            (
                "top",
                "The three biggest countries in Oceania by area are Australia (7.7 million square km), Papua New Guinea (463,000) and New Zealand (268,000). Australia has by far the most people too, then Papua New Guinea and New Zealand.",
            ),
            (
                "bottom",
                "The three smallest countries in Oceania by area are Nauru (21 square km), Tuvalu (26) and the Marshall Islands (181).",
            ),
        ],
        &[
            "Australia",
            "Papua New Guinea",
            "New Zealand",
            "Fiji",
            "Solomon Islands",
        ],
    ),
    (
        "caribbean",
        &[
            (
                "area",
                "Cuba is the largest country in the Caribbean, at about 110,000 square km; the Dominican Republic is second.",
            ),
            (
                "small",
                "Saint Kitts and Nevis is the smallest country in the Caribbean, at about 260 square km, and the smallest in the Americas.",
            ),
        ],
        &[],
    ),
    (
        "antarctica",
        &[
            (
                "area",
                "Antarctica has no countries, so there is no biggest one: the whole continent is governed by the Antarctic Treaty and the only people there are scientists at research stations.",
            ),
            (
                "small",
                "Antarctica has no countries, so there is no smallest one: it is governed by the Antarctic Treaty and the only people there are scientists at research stations.",
            ),
            (
                "populous",
                "Antarctica has no countries and no permanent residents, only scientists at research stations under the Antarctic Treaty.",
            ),
            (
                "least",
                "Antarctica has no countries and no permanent residents, only scientists at research stations under the Antarctic Treaty.",
            ),
            (
                "top",
                "Antarctica has no countries, so there is no ranking: it is governed by the Antarctic Treaty and the only people there are scientists at research stations.",
            ),
            (
                "bottom",
                "Antarctica has no countries, so there is no ranking: it is governed by the Antarctic Treaty and the only people there are scientists at research stations.",
            ),
        ],
        &[],
    ),
];
pub(super) const PEAKS: &[(&str, &str)] = &[
    (
        "table mountain",
        "Table Mountain is 1,085 metres (3,560 feet) high: the flat-topped mountain over Cape Town, and the cable car or the Platteklip Gorge path gets you up.",
    ),
    (
        "slieve donard",
        "Slieve Donard is 850 metres (2,789 feet) high: the highest mountain in Northern Ireland, right above Newcastle in County Down.",
    ),
    (
        "carrauntoohil",
        "Carrauntoohil is 1,038 metres (3,406 feet) high: the highest mountain in Ireland, in the MacGillycuddy's Reeks, and the usual way up is through the Devil's Ladder.",
    ),
    (
        "scafell pike",
        "Scafell Pike is 978 metres (3,209 feet) high: the highest mountain in England, in the Lake District, and the hardest of the three national peaks to walk because of the loose stone.",
    ),
    (
        "kilimanjaro",
        "Kilimanjaro is 5,895 metres (19,341 feet) high: the highest mountain in Africa and the tallest free-standing mountain in the world, rising straight out of the plain with no range around it.",
    ),
    (
        "mont blanc",
        "Mont Blanc is 4,808 metres (15,774 feet) high: the highest peak in the Alps and in western Europe, on the French-Italian border.",
    ),
    (
        "matterhorn",
        "The Matterhorn is 4,478 metres (14,692 feet) high: the pyramid on the Swiss-Italian border, and the last of the great Alpine peaks to be climbed, in 1865.",
    ),
    (
        "kosciuszko",
        "Mount Kosciuszko is 2,228 metres (7,310 feet) high: the highest mountain in mainland Australia, a long walk rather than a climb.",
    ),
    (
        "ben nevis",
        "Ben Nevis is 1,345 metres (4,413 feet) high: the highest mountain in the UK, in Scotland above Fort William. The walk up from Glen Nevis is about 8 hours there and back.",
    ),
    (
        "yr wyddfa",
        "Snowdon (Yr Wyddfa) is 1,085 metres (3,560 feet) high: the highest mountain in Wales, and the busiest in the UK.",
    ),
    (
        "pen y fan",
        "Pen y Fan is 886 metres (2,907 feet) high: the highest point in the Brecon Beacons and the most popular walk in south Wales, about 4 miles there and back.",
    ),
    (
        "helvellyn",
        "Helvellyn is 950 metres (3,117 feet) high: the third highest in England, and Striding Edge is the ridge that makes it famous - a scramble, not a walk.",
    ),
    (
        "aconcagua",
        "Aconcagua is 6,961 metres (22,838 feet) high: the highest mountain in South America and the highest anywhere outside Asia.",
    ),
    (
        "snowdon",
        "Snowdon (Yr Wyddfa) is 1,085 metres (3,560 feet) high: the highest mountain in Wales, and the busiest in the UK. The Snowdon Ranger path is the gentlest way up without the train.",
    ),
    (
        "denali",
        "Denali is 6,190 metres (20,308 feet) high: the highest mountain in North America, in Alaska, and the biggest rise of any mountain measured base to summit.",
    ),
    (
        "elbrus",
        "Mount Elbrus is 5,642 metres (18,510 feet) high: the highest mountain in Europe if the Caucasus counts as Europe; otherwise the European record is Mont Blanc.",
    ),
    (
        "aoraki",
        "Aoraki / Mount Cook is 3,724 metres (12,218 feet) high: the highest mountain in New Zealand, in the Southern Alps, and a serious climb rather than a walk.",
    ),
    (
        "fuji",
        "Mount Fuji is 3,776 metres (12,388 feet) high: the highest mountain in Japan, a volcano that last erupted in 1707, and a day's climb in summer.",
    ),
    (
        "k2",
        "K2 is 8,611 metres (28,251 feet) high: the second highest mountain in the world, on the Pakistan-China border, and a much harder and deadlier climb than Everest.",
    ),
];
pub(super) const HIGHEST: &[(&str, &str)] = &[
    (
        "northern ireland",
        "Slieve Donard is the highest, at 850 metres (2,789 ft).",
    ),
    (
        "great britain",
        "Ben Nevis is the highest, at 1,345 metres (4,413 ft).",
    ),
    (
        "north america",
        "Denali is the highest, at 6,190 metres (20,310 ft).",
    ),
    (
        "south america",
        "Aconcagua is the highest, at 6,961 metres (22,838 ft).",
    ),
    (
        "united states",
        "Denali is the highest, at 6,190 metres (20,310 ft).",
    ),
    (
        "new zealand",
        "Aoraki / Mount Cook is the highest, at 3,724 metres (12,218 ft).",
    ),
    (
        "antarctica",
        "Mount Vinson is the highest, at 4,892 metres (16,050 ft).",
    ),
    (
        "australia",
        "Mount Kosciuszko is the highest on the mainland, at 2,228 metres (7,310 ft). The highest in Australian territory is Mawson Peak on Heard Island.",
    ),
    (
        "scotland",
        "Ben Nevis is the highest, at 1,345 metres (4,413 ft).",
    ),
    (
        "england",
        "Scafell Pike is the highest, at 978 metres (3,209 ft).",
    ),
    (
        "ireland",
        "Carrauntoohil is the highest, at 1,038 metres (3,406 ft).",
    ),
    (
        "britain",
        "Ben Nevis is the highest, at 1,345 metres (4,413 ft).",
    ),
    (
        "oceania",
        "Puncak Jaya in New Guinea is the highest, at 4,884 metres (16,024 ft).",
    ),
    (
        "europe",
        "Mount Elbrus is the highest, at 5,642 metres (18,510 ft), if the Caucasus counts as Europe. If it does not, the record is Mont Blanc at 4,808 metres (15,774 ft).",
    ),
    (
        "africa",
        "Kilimanjaro is the highest, at 5,895 metres (19,341 ft).",
    ),
    (
        "canada",
        "Mount Logan is the highest, at 5,959 metres (19,551 ft).",
    ),
    (
        "france",
        "Mont Blanc is the highest, at 4,808 metres (15,774 ft).",
    ),
    (
        "wales",
        "Snowdon (Yr Wyddfa) is the highest, at 1,085 metres (3,560 ft).",
    ),
    (
        "spain",
        "Mount Teide in Tenerife is the highest, at 3,715 metres (12,188 ft).",
    ),
    (
        "italy",
        "Mont Blanc (Monte Bianco) is the highest, at 4,808 metres (15,774 ft), on the French border.",
    ),
    (
        "japan",
        "Mount Fuji is the highest, at 3,776 metres (12,388 ft).",
    ),
    (
        "india",
        "Kangchenjunga is the highest, at 8,586 metres (28,169 ft).",
    ),
    (
        "asia",
        "Everest is the highest, at 8,849 metres (29,032 ft).",
    ),
    (
        "alps",
        "Mont Blanc is the highest, at 4,808 metres (15,774 ft).",
    ),
    ("usa", "Denali is the highest, at 6,190 metres (20,310 ft)."),
    (
        "uk",
        "Ben Nevis is the highest, at 1,345 metres (4,413 ft).",
    ),
];
pub(super) const CONTINENT_COUNTS: &[(&str, &str)] = &[
    (
        "north america",
        "North America has 23 countries, from Canada down to Panama; the Caribbean islands are counted separately as part of the region.",
    ),
    (
        "south america",
        "South America has 12 countries, from Colombia and Venezuela round to Argentina and Chile.",
    ),
    (
        "africa",
        "Africa has 54 countries, or 55 if you count Western Sahara, which is disputed. Nigeria is the most populous and Algeria the largest by area.",
    ),
    (
        "europe",
        "Europe has 44 countries, or about 50 if you count the transcontinental ones such as Russia and Turkey. The exact number depends on where you draw the eastern edge.",
    ),
    (
        "asia",
        "Asia has 48 countries, or 49 if you count Taiwan. Russia and Turkey are usually counted in Europe as well, which is why some lists differ.",
    ),
    (
        "oceania",
        "Oceania has 14 countries, spread across Australia, New Zealand, Papua New Guinea and the Pacific island states.",
    ),
    (
        "antarctica",
        "Antarctica has no countries. It is governed by the Antarctic Treaty, and the only people there are scientists at research stations.",
    ),
];
pub(super) const RIVERS: &[(&str, &str)] = &[
    (
        "nile",
        "The Nile runs about 6,650 km from its source to the Mediterranean, which makes it the longest river in Africa and usually the longest in the world - with the Amazon a close second and carrying far more water.",
    ),
    (
        "amazon",
        "The Amazon runs about 6,400 km from its source in the Peruvian Andes to the Atlantic, and it is the biggest river in the world by water - about a fifth of all the river water on Earth.",
    ),
    (
        "mississippi",
        "The Mississippi is about 3,770 km from Minnesota to the Gulf of Mexico, or about 6,275 km if you count the Missouri from its source, which is how it is usually measured.",
    ),
    (
        "missouri",
        "The Missouri is about 3,767 km, from Montana to where it joins the Mississippi near St Louis - the longest river in North America on its own.",
    ),
    (
        "danube",
        "The Danube runs about 2,850 km from Germany's Black Forest to the Black Sea, through ten countries - more than any other river in the world.",
    ),
    (
        "volga",
        "The Volga runs about 3,530 km through Russia to the Caspian Sea, the longest river in Europe.",
    ),
    (
        "yangtze",
        "The Yangtze runs about 6,300 km across China to the sea at Shanghai, the longest river in Asia.",
    ),
    (
        "congo",
        "The Congo runs about 4,700 km, the deepest river in the world and second only to the Amazon by water.",
    ),
    (
        "rhine",
        "The Rhine runs about 1,230 km from the Swiss Alps to the North Sea at Rotterdam, and it is the busiest freight river in Europe.",
    ),
    (
        "seine",
        "The Seine runs about 775 km, from Burgundy through Paris to the sea at Le Havre.",
    ),
    (
        "thames",
        "The Thames runs about 346 km, from the Cotswolds through London to the Thames Estuary.",
    ),
    (
        "loire",
        "The Loire runs about 1,006 km, the longest river in France, from the Massif Central to the Atlantic at Saint-Nazaire.",
    ),
    (
        "po",
        "The Po runs about 652 km across northern Italy to the Adriatic, the longest river in Italy.",
    ),
    (
        "tagus",
        "The Tagus runs about 1,007 km from Spain through Portugal to Lisbon and the Atlantic, the longest river on the Iberian peninsula.",
    ),
    (
        "elbe",
        "The Elbe runs about 1,094 km from the Czech Republic through Germany to the North Sea at Hamburg.",
    ),
    (
        "oder",
        "The Oder runs about 840 km from the Czech Republic to the Baltic, most of it along the border between Germany and Poland.",
    ),
    (
        "ganges",
        "The Ganges runs about 2,525 km from the Himalayas to the Bay of Bengal, and with its delta it is one of the most populous river basins in the world.",
    ),
    (
        "indus",
        "The Indus runs about 3,180 km from Tibet through Pakistan to the Arabian Sea, and the civilisation along it was one of the first cities anywhere.",
    ),
    (
        "mekong",
        "The Mekong runs about 4,350 km from the Tibetan plateau through China, Laos, Thailand, Cambodia and Vietnam to the South China Sea.",
    ),
    (
        "yellow river",
        "The Yellow River runs about 5,464 km across northern China, the second longest in the country after the Yangtze.",
    ),
    (
        "ob",
        "The Ob runs about 3,650 km across Siberia to the Arctic Ocean, or about 5,410 km with its main tributary, the Irtysh.",
    ),
    (
        "yenisei",
        "The Yenisei runs about 3,487 km across Siberia to the Arctic, and with its headwaters it is one of the longest river systems in the world.",
    ),
    (
        "murray",
        "The Murray runs about 2,508 km through Australia to the sea in South Australia, and with the Darling it is the longest river system in the country.",
    ),
    (
        "darling",
        "The Darling runs about 1,472 km through New South Wales and Queensland, the longest single river in Australia.",
    ),
    (
        "colorado",
        "The Colorado runs about 2,330 km from the Rocky Mountains to the Gulf of California, through the Grand Canyon on the way.",
    ),
    (
        "rio grande",
        "The Rio Grande runs about 3,051 km from Colorado to the Gulf of Mexico, and it forms the border between Texas and Mexico.",
    ),
    (
        "parana",
        "The Parana runs about 4,880 km through Brazil, Paraguay and Argentina to the Rio de la Plata, second in South America only to the Amazon.",
    ),
    (
        "orinoco",
        "The Orinoco runs about 2,140 km through Venezuela to the Atlantic, and it links to the Amazon through a natural channel, the Casiquiare.",
    ),
    (
        "niger",
        "The Niger runs about 4,200 km in a great curve through West Africa to the Gulf of Guinea, the longest river in the region.",
    ),
    (
        "zambezi",
        "The Zambezi runs about 2,574 km through southern Africa to the Indian Ocean, and Victoria Falls is on it.",
    ),
    (
        "amur",
        "The Amur runs about 2,824 km along the border between Russia and China to the Pacific, or about 4,444 km with its headwaters.",
    ),
    (
        "lena",
        "The Lena runs about 4,400 km across Siberia to the Arctic Ocean, and it freezes solid for months each winter.",
    ),
    (
        "irrawaddy",
        "The Irrawaddy runs about 2,170 km through Myanmar to the sea, the country's main river for transport and rice.",
    ),
    (
        "euphrates",
        "The Euphrates runs about 2,800 km from Turkey through Syria and Iraq to the Shatt al-Arab, where it joins the Tigris.",
    ),
    (
        "tigris",
        "The Tigris runs about 1,850 km from Turkey through Iraq to the Shatt al-Arab, and the land between it and the Euphrates is ancient Mesopotamia.",
    ),
];
pub(super) const RIVER_NAME_NEEDS_RIVER: &[&str] = &[
    "amazon", "amur", "colorado", "congo", "darling", "elbe", "lena", "murray", "niger", "ob",
    "oder", "parana", "po",
];
pub(super) const RECORD_SMALLEST: &[&str] = &["smallest", "smallest by area", "tiniest"];
pub(super) const RECORD_POPULOUS: &[&str] = &[
    "biggest population",
    "by people",
    "by population",
    "in population",
    "in terms of people",
    "in terms of population",
    "largest population",
    "most people",
    "most populated",
    "most populous",
    "population wise",
];
pub(super) const LEAST: &[&str] = &[
    "fewest in population",
    "fewest people",
    "fewest population",
    "least in population",
    "least people",
    "least populated",
    "least populous",
    "smallest by population",
    "smallest population",
];
pub(super) const LIST_PATTERN: &str = r###"\b(?:list|top|rank|ranking)\b|\b(?:two|three|four|five|six|seven|eight|nine|ten|\d+)\b\s+(?:of\s+the\s+)?(?:biggest|largest|smallest|tiniest|populous)|\b(?:biggest|largest|smallest|tiniest|populous)\s+countries\s+(?:in|of)\b"###;
pub(super) const UK_COMPARISON: &str = "Great Britain is the island: England, Scotland and Wales. The UK is the country, which is that island plus Northern Ireland. So England is in Britain and in the UK, Northern Ireland is in the UK but not in Britain, and the British Isles is only geography - both islands, including the Republic of Ireland, which is a separate country.";
pub(super) const UK_PARTS: &[(&str, [&str; 3])] = &[
    (
        "northern ireland",
        [
            "Yes - Northern Ireland is part of the UK. The UK is England, Scotland, Wales and Northern Ireland; Great Britain is the island with the first three on it.",
            "Northern Ireland is called a province or a region rather than a country, but it is part of the UK all the same: the UK is England, Scotland, Wales and Northern Ireland.",
            "No - Northern Ireland is not a separate country: it is one of the four parts of the UK, along with England, Scotland, Wales and Northern Ireland. The UK is the country.",
        ],
    ),
    (
        "ireland",
        [
            "No - Ireland is not part of the UK. The Republic of Ireland is a separate country; Northern Ireland is the part of the UK on the same island. Great Britain is England, Scotland and Wales.",
            "No - Ireland is not part of the UK. The Republic of Ireland is a separate country; Northern Ireland is the part of the UK on the same island. Great Britain is England, Scotland and Wales.",
            "No - Ireland is not part of the UK. The Republic of Ireland is a separate country; Northern Ireland is the part of the UK on the same island. Great Britain is England, Scotland and Wales.",
        ],
    ),
    (
        "england",
        [
            "Yes - England is part of the UK. The UK is England, Scotland, Wales and Northern Ireland; Great Britain is the island with the first three on it.",
            "Yes - England is a country, one of the four in the UK. It is not a sovereign state on its own: the UK is the country that sits at the UN and signs treaties.",
            "No - England is not a separate country: it is one of the four parts of the UK, along with England, Scotland, Wales and Northern Ireland. The UK is the country.",
        ],
    ),
    (
        "scotland",
        [
            "Yes - Scotland is part of the UK. The UK is England, Scotland, Wales and Northern Ireland; Great Britain is the island with the first three on it.",
            "Yes - Scotland is a country, one of the four in the UK. It is not a sovereign state on its own: the UK is the country that sits at the UN and signs treaties.",
            "No - Scotland is not a separate country: it is one of the four parts of the UK, along with England, Scotland, Wales and Northern Ireland. The UK is the country.",
        ],
    ),
    (
        "wales",
        [
            "Yes - Wales is part of the UK. The UK is England, Scotland, Wales and Northern Ireland; Great Britain is the island with the first three on it.",
            "Yes - Wales is a country, one of the four in the UK. It is not a sovereign state on its own: the UK is the country that sits at the UN and signs treaties.",
            "No - Wales is not a separate country: it is one of the four parts of the UK, along with England, Scotland, Wales and Northern Ireland. The UK is the country.",
        ],
    ),
];
pub(super) const UK_PAIRS: &[(&str, &str, &str)] = &[
    (
        "britain",
        "europe",
        "Yes - Britain is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent.",
    ),
    (
        "britain",
        "great britain",
        "Yes - Britain is part of Great Britain, the island that also has England and Scotland on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "england",
        "britain",
        "Yes - England is part of Great Britain, the island that also has Scotland and Wales on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "england",
        "europe",
        "Yes - England is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent.",
    ),
    (
        "england",
        "great britain",
        "Yes - England is part of Great Britain, the island that also has Scotland and Wales on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "england",
        "ireland",
        "No - England is not part of Ireland. Ireland is a separate country; it shares its island with Northern Ireland, which is part of the UK.",
    ),
    (
        "england",
        "northern ireland",
        "No - England is not part of Northern Ireland. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "england",
        "scotland",
        "No - England is not part of Scotland. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "england",
        "wales",
        "No - England is not part of Wales. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "great britain",
        "britain",
        "Yes - Great Britain is part of Great Britain, the island that also has England and Scotland on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "great britain",
        "europe",
        "Yes - Great Britain is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent.",
    ),
    (
        "ireland",
        "britain",
        "No - Ireland is not in Great Britain: Great Britain is England, Scotland and Wales. The Republic of Ireland is a separate country.",
    ),
    (
        "ireland",
        "europe",
        "Yes - Ireland is in Europe, and in the European Union. It is a separate country from the UK: only Northern Ireland is part of the UK.",
    ),
    (
        "ireland",
        "great britain",
        "No - Ireland is not in Great Britain: Great Britain is England, Scotland and Wales. The Republic of Ireland is a separate country.",
    ),
    (
        "northern ireland",
        "britain",
        "No - Northern Ireland is not in Great Britain: Great Britain is England, Scotland and Wales. Northern Ireland is in the UK all the same, on the island of Ireland.",
    ),
    (
        "northern ireland",
        "england",
        "No - Northern Ireland is not part of England. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "northern ireland",
        "europe",
        "Yes - Northern Ireland is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent.",
    ),
    (
        "northern ireland",
        "great britain",
        "No - Northern Ireland is not in Great Britain: Great Britain is England, Scotland and Wales. Northern Ireland is in the UK all the same, on the island of Ireland.",
    ),
    (
        "northern ireland",
        "ireland",
        "No - Northern Ireland is not part of Ireland. Ireland is a separate country; Northern Ireland is the part of the UK on the same island.",
    ),
    (
        "northern ireland",
        "scotland",
        "No - Northern Ireland is not part of Scotland. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "northern ireland",
        "wales",
        "No - Northern Ireland is not part of Wales. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "scotland",
        "britain",
        "Yes - Scotland is part of Great Britain, the island that also has England and Wales on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "scotland",
        "england",
        "No - Scotland is not part of England. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "scotland",
        "europe",
        "Yes - Scotland is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent.",
    ),
    (
        "scotland",
        "great britain",
        "Yes - Scotland is part of Great Britain, the island that also has England and Wales on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "scotland",
        "ireland",
        "No - Scotland is not part of Ireland. Ireland is a separate country; it shares its island with Northern Ireland, which is part of the UK.",
    ),
    (
        "scotland",
        "northern ireland",
        "No - Scotland is not part of Northern Ireland. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "scotland",
        "wales",
        "No - Scotland is not part of Wales. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "uk",
        "britain",
        "Yes - the UK is part of Great Britain, the island that also has England and Scotland on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "uk",
        "europe",
        "Yes - the UK is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent.",
    ),
    (
        "uk",
        "great britain",
        "Yes - the UK is part of Great Britain, the island that also has England and Scotland on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "united kingdom",
        "britain",
        "Yes - the UK is part of Great Britain, the island that also has England and Scotland on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "united kingdom",
        "europe",
        "Yes - the UK is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent.",
    ),
    (
        "united kingdom",
        "great britain",
        "Yes - the UK is part of Great Britain, the island that also has England and Scotland on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "wales",
        "britain",
        "Yes - Wales is part of Great Britain, the island that also has England and Scotland on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "wales",
        "england",
        "No - Wales is not part of England. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "wales",
        "europe",
        "Yes - Wales is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent.",
    ),
    (
        "wales",
        "great britain",
        "Yes - Wales is part of Great Britain, the island that also has England and Scotland on it. Great Britain and Northern Ireland together are the UK.",
    ),
    (
        "wales",
        "ireland",
        "No - Wales is not part of Ireland. Ireland is a separate country; it shares its island with Northern Ireland, which is part of the UK.",
    ),
    (
        "wales",
        "northern ireland",
        "No - Wales is not part of Northern Ireland. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
    (
        "wales",
        "scotland",
        "No - Wales is not part of Scotland. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland.",
    ),
];
pub(super) const UK_NATION: &str = r###"\b(northern ireland|great britain|united kingdom|england|scotland|wales|britain|uk|ireland|europe)\b"###;
