//! Pinned history table replies and guards, including declined and previous-turn reads.

pub(super) type Case = (
    &'static str,
    &'static str,
    &'static [(&'static str, &'static str)],
    bool,
    Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    (
        "cold-war",
        "what was the cold war",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda.",
        ),
    ),
    (
        "cold-war",
        "who won the cold war",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda. The West won it: the Soviet Union broke up in 1991.",
        ),
    ),
    (
        "cold-war",
        "who came out on top in the cold war",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda. The West won it: the Soviet Union broke up in 1991.",
        ),
    ),
    (
        "cold-war",
        "why was it called the cold war",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda.",
        ),
    ),
    (
        "cold-war",
        "did the west WIN",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda. The West won it: the Soviet Union broke up in 1991.",
        ),
    ),
    (
        "cold-war",
        "what did winning mean",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda.",
        ),
    ),
    (
        "cold-war",
        "did they get a victory, then",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda. The West won it: the Soviet Union broke up in 1991.",
        ),
    ),
    (
        "cold-war",
        "what side came out--on top",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda. The West won it: the Soviet Union broke up in 1991.",
        ),
    ),
    (
        "cold-war",
        "who got the victory in the cold war",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda. The West won it: the Soviet Union broke up in 1991.",
        ),
    ),
    (
        "cold-war",
        "who WON, in your view",
        &[],
        true,
        Some(
            "The Cold War was the long standoff between the United States and its allies and the Soviet Union from about 1947 to 1991. They never fought each other directly because both had nuclear weapons, so the conflict played out in proxy wars, the space race and propaganda. The West won it: the Soviet Union broke up in 1991.",
        ),
    ),
    (
        "the-past-named",
        "tell me about world war one",
        &[],
        true,
        Some(
            "World War One ran from 1914 to 1918. It began when Archduke Franz Ferdinand was assassinated and the alliances pulled the great powers in, and it ended with Germany's armistice on 11 November 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about world war 1",
        &[],
        true,
        Some(
            "World War One ran from 1914 to 1918. It began when Archduke Franz Ferdinand was assassinated and the alliances pulled the great powers in, and it ended with Germany's armistice on 11 November 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about ww1",
        &[],
        true,
        Some(
            "World War One ran from 1914 to 1918. It began when Archduke Franz Ferdinand was assassinated and the alliances pulled the great powers in, and it ended with Germany's armistice on 11 November 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about wwi",
        &[],
        true,
        Some(
            "World War One ran from 1914 to 1918. It began when Archduke Franz Ferdinand was assassinated and the alliances pulled the great powers in, and it ended with Germany's armistice on 11 November 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first world war",
        &[],
        true,
        Some(
            "World War One ran from 1914 to 1918. It began when Archduke Franz Ferdinand was assassinated and the alliances pulled the great powers in, and it ended with Germany's armistice on 11 November 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about great war",
        &[],
        true,
        Some(
            "World War One ran from 1914 to 1918. It began when Archduke Franz Ferdinand was assassinated and the alliances pulled the great powers in, and it ended with Germany's armistice on 11 November 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about world war two",
        &[],
        true,
        Some(
            "World War Two ran from 1939 to 1945. Britain and France declared war on Germany after it invaded Poland, and it ended with Germany's surrender on 8 May 1945 and Japan's on 2 September.",
        ),
    ),
    (
        "the-past-named",
        "tell me about world war 2",
        &[],
        true,
        Some(
            "World War Two ran from 1939 to 1945. Britain and France declared war on Germany after it invaded Poland, and it ended with Germany's surrender on 8 May 1945 and Japan's on 2 September.",
        ),
    ),
    (
        "the-past-named",
        "tell me about ww2",
        &[],
        true,
        Some(
            "World War Two ran from 1939 to 1945. Britain and France declared war on Germany after it invaded Poland, and it ended with Germany's surrender on 8 May 1945 and Japan's on 2 September.",
        ),
    ),
    (
        "the-past-named",
        "tell me about wwii",
        &[],
        true,
        Some(
            "World War Two ran from 1939 to 1945. Britain and France declared war on Germany after it invaded Poland, and it ended with Germany's surrender on 8 May 1945 and Japan's on 2 September.",
        ),
    ),
    (
        "the-past-named",
        "tell me about second world war",
        &[],
        true,
        Some(
            "World War Two ran from 1939 to 1945. Britain and France declared war on Germany after it invaded Poland, and it ended with Germany's surrender on 8 May 1945 and Japan's on 2 September.",
        ),
    ),
    (
        "the-past-named",
        "tell me about samuel pepys",
        &[],
        true,
        Some(
            "Samuel Pepys (1633-1703) was a London naval administrator whose diary, kept from 1660 to 1669 in shorthand, is the best eyewitness account we have of the plague, the Great Fire and the Restoration court. He gave it up because he thought he was going blind, and it was not read until the 1820s.",
        ),
    ),
    (
        "the-past-named",
        "tell me about pepys",
        &[],
        true,
        Some(
            "Samuel Pepys (1633-1703) was a London naval administrator whose diary, kept from 1660 to 1669 in shorthand, is the best eyewitness account we have of the plague, the Great Fire and the Restoration court. He gave it up because he thought he was going blind, and it was not read until the 1820s.",
        ),
    ),
    (
        "the-past-named",
        "tell me about isambard kingdom brunel",
        &[],
        true,
        Some(
            "Isambard Kingdom Brunel (1806-1859) was the Victorian engineer of the Great Western Railway, the Clifton Suspension Bridge, the SS Great Britain and the Great Eastern, and the tunnels and viaducts between them. He worked at a scale nobody had tried before, in a top hat, and died at 53 after a stroke, worn out by the Great Eastern.",
        ),
    ),
    (
        "the-past-named",
        "tell me about brunel",
        &[],
        true,
        Some(
            "Isambard Kingdom Brunel (1806-1859) was the Victorian engineer of the Great Western Railway, the Clifton Suspension Bridge, the SS Great Britain and the Great Eastern, and the tunnels and viaducts between them. He worked at a scale nobody had tried before, in a top hat, and died at 53 after a stroke, worn out by the Great Eastern.",
        ),
    ),
    (
        "the-past-named",
        "tell me about celts",
        &[],
        true,
        Some(
            "The Celts were the iron-working peoples of central Europe who spread west to Britain, Ireland, France and Spain from about 500 BC. They wrote nothing down themselves, so what is known comes from Roman writers and from their hillforts, torcs and burials - and from the Celtic languages still spoken in Wales, Scotland, Ireland and Cornwall.",
        ),
    ),
    (
        "the-past-named",
        "tell me about celtic people",
        &[],
        true,
        Some(
            "The Celts were the iron-working peoples of central Europe who spread west to Britain, Ireland, France and Spain from about 500 BC. They wrote nothing down themselves, so what is known comes from Roman writers and from their hillforts, torcs and burials - and from the Celtic languages still spoken in Wales, Scotland, Ireland and Cornwall.",
        ),
    ),
    (
        "the-past-named",
        "tell me about celtic britain",
        &[],
        true,
        Some(
            "The Celts were the iron-working peoples of central Europe who spread west to Britain, Ireland, France and Spain from about 500 BC. They wrote nothing down themselves, so what is known comes from Roman writers and from their hillforts, torcs and burials - and from the Celtic languages still spoken in Wales, Scotland, Ireland and Cornwall.",
        ),
    ),
    (
        "the-past-named",
        "tell me about ancient celts",
        &[],
        true,
        Some(
            "The Celts were the iron-working peoples of central Europe who spread west to Britain, Ireland, France and Spain from about 500 BC. They wrote nothing down themselves, so what is known comes from Roman writers and from their hillforts, torcs and burials - and from the Celtic languages still spoken in Wales, Scotland, Ireland and Cornwall.",
        ),
    ),
    (
        "the-past-named",
        "tell me about egyptians",
        &[],
        true,
        Some(
            "Ancient Egypt gave us the 365-day calendar, papyrus paper, hieroglyphic writing, glass and the water clock, and Egyptian surgeons were stitching wounds, setting broken bones and removing cataracts by about 2500 BC. Their maths was practical: fractions, a base of ten, and the 3-4-5 triangle for squaring a corner.",
        ),
    ),
    (
        "the-past-named",
        "tell me about ancient egyptians",
        &[],
        true,
        Some(
            "Ancient Egypt gave us the 365-day calendar, papyrus paper, hieroglyphic writing, glass and the water clock, and Egyptian surgeons were stitching wounds, setting broken bones and removing cataracts by about 2500 BC. Their maths was practical: fractions, a base of ten, and the 3-4-5 triangle for squaring a corner.",
        ),
    ),
    (
        "the-past-named",
        "tell me about egyptian people",
        &[],
        true,
        Some(
            "Ancient Egypt gave us the 365-day calendar, papyrus paper, hieroglyphic writing, glass and the water clock, and Egyptian surgeons were stitching wounds, setting broken bones and removing cataracts by about 2500 BC. Their maths was practical: fractions, a base of ten, and the 3-4-5 triangle for squaring a corner.",
        ),
    ),
    (
        "the-past-named",
        "tell me about domesday book",
        &[],
        true,
        Some(
            "The Domesday Book is the survey William the Conqueror ordered at Christmas 1085 and finished the next year: who held every manor, field, mill, wood and pig in England, and what each was worth before and after the Norman conquest. It was called Domesday because, like the Last Judgement, there was no appeal from it.",
        ),
    ),
    (
        "the-past-named",
        "tell me about domesday",
        &[],
        true,
        Some(
            "The Domesday Book is the survey William the Conqueror ordered at Christmas 1085 and finished the next year: who held every manor, field, mill, wood and pig in England, and what each was worth before and after the Norman conquest. It was called Domesday because, like the Last Judgement, there was no appeal from it.",
        ),
    ),
    (
        "the-past-named",
        "tell me about doomsday book",
        &[],
        true,
        Some(
            "The Domesday Book is the survey William the Conqueror ordered at Christmas 1085 and finished the next year: who held every manor, field, mill, wood and pig in England, and what each was worth before and after the Norman conquest. It was called Domesday because, like the Last Judgement, there was no appeal from it.",
        ),
    ),
    (
        "the-past-named",
        "tell me about doomsday",
        &[],
        true,
        Some(
            "The Domesday Book is the survey William the Conqueror ordered at Christmas 1085 and finished the next year: who held every manor, field, mill, wood and pig in England, and what each was worth before and after the Norman conquest. It was called Domesday because, like the Last Judgement, there was no appeal from it.",
        ),
    ),
    (
        "the-past-named",
        "tell me about normans invade",
        &[],
        true,
        Some(
            "The Normans invaded England in 1066: William of Normandy landed in September, killed King Harold at Hastings on 14 October, and was crowned on Christmas Day. Within twenty years his barons had built the castles and taken the land in the Domesday survey.",
        ),
    ),
    (
        "the-past-named",
        "tell me about norman invasion",
        &[],
        true,
        Some(
            "The Normans invaded England in 1066: William of Normandy landed in September, killed King Harold at Hastings on 14 October, and was crowned on Christmas Day. Within twenty years his barons had built the castles and taken the land in the Domesday survey.",
        ),
    ),
    (
        "the-past-named",
        "tell me about norman conquest",
        &[],
        true,
        Some(
            "The Normans invaded England in 1066: William of Normandy landed in September, killed King Harold at Hastings on 14 October, and was crowned on Christmas Day. Within twenty years his barons had built the castles and taken the land in the Domesday survey.",
        ),
    ),
    (
        "the-past-named",
        "tell me about normans came",
        &[],
        true,
        Some(
            "The Normans invaded England in 1066: William of Normandy landed in September, killed King Harold at Hastings on 14 October, and was crowned on Christmas Day. Within twenty years his barons had built the castles and taken the land in the Domesday survey.",
        ),
    ),
    (
        "the-past-named",
        "tell me about edward the confessor",
        &[],
        true,
        Some(
            "Edward the Confessor was king of England from 1042 to 1066, the last of the old West Saxon line: he rebuilt Westminster Abbey, where he is buried, and died childless in January 1066, which is why three men fought over the crown that year.",
        ),
    ),
    (
        "the-past-named",
        "tell me about admiral nelson",
        &[],
        true,
        Some(
            "Horatio Nelson (1758-1805) was the British admiral who beat the French fleet at the Nile in 1798 and the combined French and Spanish fleet at Trafalgar in 1805, where he was shot and died on his own ship. He lost an arm and an eye in earlier actions and is remembered for the signal 'England expects that every man will do his duty'.",
        ),
    ),
    (
        "the-past-named",
        "tell me about horatio nelson",
        &[],
        true,
        Some(
            "Horatio Nelson (1758-1805) was the British admiral who beat the French fleet at the Nile in 1798 and the combined French and Spanish fleet at Trafalgar in 1805, where he was shot and died on his own ship. He lost an arm and an eye in earlier actions and is remembered for the signal 'England expects that every man will do his duty'.",
        ),
    ),
    (
        "the-past-named",
        "tell me about lord nelson",
        &[],
        true,
        Some(
            "Horatio Nelson (1758-1805) was the British admiral who beat the French fleet at the Nile in 1798 and the combined French and Spanish fleet at Trafalgar in 1805, where he was shot and died on his own ship. He lost an arm and an eye in earlier actions and is remembered for the signal 'England expects that every man will do his duty'.",
        ),
    ),
    (
        "the-past-named",
        "tell me about nelson",
        &[],
        true,
        Some(
            "Horatio Nelson (1758-1805) was the British admiral who beat the French fleet at the Nile in 1798 and the combined French and Spanish fleet at Trafalgar in 1805, where he was shot and died on his own ship. He lost an arm and an eye in earlier actions and is remembered for the signal 'England expects that every man will do his duty'.",
        ),
    ),
    (
        "the-past-named",
        "tell me about richard the lionheart",
        &[],
        true,
        Some(
            "Richard I (1157-1199) was king of England for ten years and spent only about six months of them in England: he went on the Third Crusade, was captured on the way home and ransomed for a fortune, then died besieging a small castle in France.",
        ),
    ),
    (
        "the-past-named",
        "tell me about lionheart",
        &[],
        true,
        Some(
            "Richard I (1157-1199) was king of England for ten years and spent only about six months of them in England: he went on the Third Crusade, was captured on the way home and ransomed for a fortune, then died besieging a small castle in France.",
        ),
    ),
    (
        "the-past-named",
        "tell me about richard the first",
        &[],
        true,
        Some(
            "Richard I (1157-1199) was king of England for ten years and spent only about six months of them in England: he went on the Third Crusade, was captured on the way home and ransomed for a fortune, then died besieging a small castle in France.",
        ),
    ),
    (
        "the-past-named",
        "tell me about richard i",
        &[],
        true,
        Some(
            "Richard I (1157-1199) was king of England for ten years and spent only about six months of them in England: he went on the Third Crusade, was captured on the way home and ransomed for a fortune, then died besieging a small castle in France.",
        ),
    ),
    (
        "the-past-named",
        "tell me about saladin",
        &[],
        true,
        Some(
            "Saladin (about 1137-1193) was the Kurdish sultan who united Egypt and Syria and took Jerusalem back from the crusaders at Hattin in 1187. He fought Richard the Lionheart to a standstill in the Third Crusade and let the people of Jerusalem go rather than massacre them, which is why he is remembered well on both sides.",
        ),
    ),
    (
        "the-past-named",
        "tell me about suleiman the magnificent",
        &[],
        true,
        Some(
            "Suleiman the Magnificent (1494-1566) ruled the Ottoman Empire for 46 years, took Belgrade and Rhodes, besieged Vienna in 1529, and gave the empire the law code that had him called Suleiman the Lawgiver at home. It was the height of Ottoman power.",
        ),
    ),
    (
        "the-past-named",
        "tell me about suleiman",
        &[],
        true,
        Some(
            "Suleiman the Magnificent (1494-1566) ruled the Ottoman Empire for 46 years, took Belgrade and Rhodes, besieged Vienna in 1529, and gave the empire the law code that had him called Suleiman the Lawgiver at home. It was the height of Ottoman power.",
        ),
    ),
    (
        "the-past-named",
        "tell me about britain leave india",
        &[],
        true,
        Some(
            "Britain left India on 15 August 1947, when the country became independent and was partitioned into India and Pakistan at the same moment, which set off the largest migration and one of the worst episodes of violence of the century.",
        ),
    ),
    (
        "the-past-named",
        "tell me about britain left india",
        &[],
        true,
        Some(
            "Britain left India on 15 August 1947, when the country became independent and was partitioned into India and Pakistan at the same moment, which set off the largest migration and one of the worst episodes of violence of the century.",
        ),
    ),
    (
        "the-past-named",
        "tell me about british leave india",
        &[],
        true,
        Some(
            "Britain left India on 15 August 1947, when the country became independent and was partitioned into India and Pakistan at the same moment, which set off the largest migration and one of the worst episodes of violence of the century.",
        ),
    ),
    (
        "the-past-named",
        "tell me about british raj end",
        &[],
        true,
        Some(
            "Britain left India on 15 August 1947, when the country became independent and was partitioned into India and Pakistan at the same moment, which set off the largest migration and one of the worst episodes of violence of the century.",
        ),
    ),
    (
        "the-past-named",
        "tell me about british leave",
        &[],
        true,
        Some(
            "Britain left India on 15 August 1947, when the country became independent and was partitioned into India and Pakistan at the same moment, which set off the largest migration and one of the worst episodes of violence of the century.",
        ),
    ),
    (
        "the-past-named",
        "tell me about neville chamberlain",
        &[],
        true,
        Some(
            "Neville Chamberlain (1869-1940) was British prime minister from 1937 to 1940. He flew to Germany three times in 1938 and came back promising 'peace for our time' from Munich, gave up the Sudetenland, and was forced out after the Norway campaign in 1940.",
        ),
    ),
    (
        "the-past-named",
        "tell me about chamberlain",
        &[],
        true,
        Some(
            "Neville Chamberlain (1869-1940) was British prime minister from 1937 to 1940. He flew to Germany three times in 1938 and came back promising 'peace for our time' from Munich, gave up the Sudetenland, and was forced out after the Norway campaign in 1940.",
        ),
    ),
    (
        "the-past-named",
        "tell me about boer war",
        &[],
        true,
        Some(
            "The Boer War ran from 1899 to 1902: the British fought the Dutch-speaking Boer republics of the Transvaal and the Orange Free State over gold and empire. It took a quarter of a million soldiers to win, and the camps where Boer families were held killed about 26,000 people, mostly children.",
        ),
    ),
    (
        "the-past-named",
        "tell me about boers",
        &[],
        true,
        Some(
            "The Boer War ran from 1899 to 1902: the British fought the Dutch-speaking Boer republics of the Transvaal and the Orange Free State over gold and empire. It took a quarter of a million soldiers to win, and the camps where Boer families were held killed about 26,000 people, mostly children.",
        ),
    ),
    (
        "the-past-named",
        "tell me about franz ferdinand",
        &[],
        true,
        Some(
            "Archduke Franz Ferdinand of Austria-Hungary was shot in Sarajevo on 28 June 1914 by Gavrilo Princip, a Bosnian Serb nationalist. Austria-Hungary blamed Serbia, the alliances pulled every great power in, and the First World War began five weeks later.",
        ),
    ),
    (
        "the-past-named",
        "tell me about archduke ferdinand",
        &[],
        true,
        Some(
            "Archduke Franz Ferdinand of Austria-Hungary was shot in Sarajevo on 28 June 1914 by Gavrilo Princip, a Bosnian Serb nationalist. Austria-Hungary blamed Serbia, the alliances pulled every great power in, and the First World War began five weeks later.",
        ),
    ),
    (
        "the-past-named",
        "tell me about russian revolution",
        &[],
        true,
        Some(
            "The Russian Revolution ran through 1917: in February strikes and mutinies pushed the tsar to abdicate, and in October the Bolsheviks under Lenin took over in Petrograd. A civil war followed, the USSR was formed in 1922, and the last tsar was shot with his family in 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about bolshevik revolution",
        &[],
        true,
        Some(
            "The Russian Revolution ran through 1917: in February strikes and mutinies pushed the tsar to abdicate, and in October the Bolsheviks under Lenin took over in Petrograd. A civil war followed, the USSR was formed in 1922, and the last tsar was shot with his family in 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about october revolution",
        &[],
        true,
        Some(
            "The Russian Revolution ran through 1917: in February strikes and mutinies pushed the tsar to abdicate, and in October the Bolsheviks under Lenin took over in Petrograd. A civil war followed, the USSR was formed in 1922, and the last tsar was shot with his family in 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about rasputin",
        &[],
        true,
        Some(
            "Grigori Rasputin (1869-1916) was a Siberian peasant who became the tsarina's closest adviser because he seemed able to stop the haemophilia bleeding of the heir, Alexei. By 1916 his influence at court had become a scandal, and a group of nobles poisoned him, shot him and threw him in the Neva.",
        ),
    ),
    (
        "the-past-named",
        "tell me about attila the hun",
        &[],
        true,
        Some(
            "Attila (about 406-453) was king of the Huns, the horsemen from the steppe who extracted tribute from the Roman Empire rather than taking it. He raided across the Balkans and into Gaul, was beaten at Chalons in 451, and died in 453 - after which his empire fell apart within a generation.",
        ),
    ),
    (
        "the-past-named",
        "tell me about attila",
        &[],
        true,
        Some(
            "Attila (about 406-453) was king of the Huns, the horsemen from the steppe who extracted tribute from the Roman Empire rather than taking it. He raided across the Balkans and into Gaul, was beaten at Chalons in 451, and died in 453 - after which his empire fell apart within a generation.",
        ),
    ),
    (
        "the-past-named",
        "tell me about pearl harbour",
        &[],
        true,
        Some(
            "Japan attacked the US fleet at Pearl Harbor in Hawaii on 7 December 1941, sinking or damaging most of the battleships in a morning and killing about 2,400 people. The United States declared war the next day, which is how it entered the Second World War.",
        ),
    ),
    (
        "the-past-named",
        "tell me about pearl harbor",
        &[],
        true,
        Some(
            "Japan attacked the US fleet at Pearl Harbor in Hawaii on 7 December 1941, sinking or damaging most of the battleships in a morning and killing about 2,400 people. The United States declared war the next day, which is how it entered the Second World War.",
        ),
    ),
    (
        "the-past-named",
        "tell me about otto von bismarck",
        &[],
        true,
        Some(
            "Otto von Bismarck (1815-1898) was the Prussian statesman who united Germany in 1871 by three short wars and then ran it as chancellor until 1890. He called his method Realpolitik - what works rather than what is right - and built the welfare state and alliances that kept the peace in Europe for twenty years after he left.",
        ),
    ),
    (
        "the-past-named",
        "tell me about bismarck",
        &[],
        true,
        Some(
            "Otto von Bismarck (1815-1898) was the Prussian statesman who united Germany in 1871 by three short wars and then ran it as chancellor until 1890. He called his method Realpolitik - what works rather than what is right - and built the welfare state and alliances that kept the peace in Europe for twenty years after he left.",
        ),
    ),
    (
        "the-past-named",
        "tell me about ireland become a republic",
        &[],
        true,
        Some(
            "Ireland became a republic on 18 April 1949, when the Republic of Ireland Act came into force and the country cut its last constitutional link with the Crown. It had been the Irish Free State since 1922, after the war of independence and the treaty.",
        ),
    ),
    (
        "the-past-named",
        "tell me about ireland became a republic",
        &[],
        true,
        Some(
            "Ireland became a republic on 18 April 1949, when the Republic of Ireland Act came into force and the country cut its last constitutional link with the Crown. It had been the Irish Free State since 1922, after the war of independence and the treaty.",
        ),
    ),
    (
        "the-past-named",
        "tell me about ireland a republic",
        &[],
        true,
        Some(
            "Ireland became a republic on 18 April 1949, when the Republic of Ireland Act came into force and the country cut its last constitutional link with the Crown. It had been the Irish Free State since 1922, after the war of independence and the treaty.",
        ),
    ),
    (
        "the-past-named",
        "tell me about republic of ireland",
        &[],
        true,
        Some(
            "Ireland became a republic on 18 April 1949, when the Republic of Ireland Act came into force and the country cut its last constitutional link with the Crown. It had been the Irish Free State since 1922, after the war of independence and the treaty.",
        ),
    ),
    (
        "the-past-named",
        "tell me about irish republic",
        &[],
        true,
        Some(
            "Ireland became a republic on 18 April 1949, when the Republic of Ireland Act came into force and the country cut its last constitutional link with the Crown. It had been the Irish Free State since 1922, after the war of independence and the treaty.",
        ),
    ),
    (
        "the-past-named",
        "tell me about the depression start",
        &[],
        true,
        Some(
            "The Great Depression began with the Wall Street crash in October 1929 and lasted through the 1930s: banks failed, a quarter of American workers lost their jobs, and world trade shrank by two thirds. It ended only with the rearmament that came with the Second World War.",
        ),
    ),
    (
        "the-past-named",
        "tell me about the depression begin",
        &[],
        true,
        Some(
            "The Great Depression began with the Wall Street crash in October 1929 and lasted through the 1930s: banks failed, a quarter of American workers lost their jobs, and world trade shrank by two thirds. It ended only with the rearmament that came with the Second World War.",
        ),
    ),
    (
        "the-past-named",
        "tell me about the depression happen",
        &[],
        true,
        Some(
            "The Great Depression began with the Wall Street crash in October 1929 and lasted through the 1930s: banks failed, a quarter of American workers lost their jobs, and world trade shrank by two thirds. It ended only with the rearmament that came with the Second World War.",
        ),
    ),
    (
        "the-past-named",
        "tell me about the depression end",
        &[],
        true,
        Some(
            "The Great Depression began with the Wall Street crash in October 1929 and lasted through the 1930s: banks failed, a quarter of American workers lost their jobs, and world trade shrank by two thirds. It ended only with the rearmament that came with the Second World War.",
        ),
    ),
    (
        "the-past-named",
        "tell me about the depression finish",
        &[],
        true,
        Some(
            "The Great Depression began with the Wall Street crash in October 1929 and lasted through the 1930s: banks failed, a quarter of American workers lost their jobs, and world trade shrank by two thirds. It ended only with the rearmament that came with the Second World War.",
        ),
    ),
    (
        "the-past-named",
        "tell me about caesar augustus",
        &[],
        true,
        Some(
            "Augustus (63 BC-14 AD) was the first Roman emperor: Julius Caesar's adopted heir, who took the name Augustus when he came out on top of the civil wars in 27 BC and ruled for forty years. He kept the forms of the republic while holding the power of a king, and his reign opened the two centuries of peace Romans called the Pax Romana.",
        ),
    ),
    (
        "the-past-named",
        "tell me about augustus",
        &[],
        true,
        Some(
            "Augustus (63 BC-14 AD) was the first Roman emperor: Julius Caesar's adopted heir, who took the name Augustus when he came out on top of the civil wars in 27 BC and ruled for forty years. He kept the forms of the republic while holding the power of a king, and his reign opened the two centuries of peace Romans called the Pax Romana.",
        ),
    ),
    (
        "the-past-named",
        "tell me about octavian",
        &[],
        true,
        Some(
            "Augustus (63 BC-14 AD) was the first Roman emperor: Julius Caesar's adopted heir, who took the name Augustus when he came out on top of the civil wars in 27 BC and ruled for forty years. He kept the forms of the republic while holding the power of a king, and his reign opened the two centuries of peace Romans called the Pax Romana.",
        ),
    ),
    (
        "the-past-named",
        "tell me about octavius",
        &[],
        true,
        Some(
            "Augustus (63 BC-14 AD) was the first Roman emperor: Julius Caesar's adopted heir, who took the name Augustus when he came out on top of the civil wars in 27 BC and ruled for forty years. He kept the forms of the republic while holding the power of a king, and his reign opened the two centuries of peace Romans called the Pax Romana.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first roman emperor",
        &[],
        true,
        Some(
            "Augustus (63 BC-14 AD) was the first Roman emperor: Julius Caesar's adopted heir, who took the name Augustus when he came out on top of the civil wars in 27 BC and ruled for forty years. He kept the forms of the republic while holding the power of a king, and his reign opened the two centuries of peace Romans called the Pax Romana.",
        ),
    ),
    (
        "the-past-named",
        "tell me about women get the vote",
        &[],
        true,
        Some(
            "Women got the vote in Britain in two steps. In 1918 women over 30 who met a property test, and all men over 21, were given the vote; in 1928 women got it at 21, on the same terms as men. The suffragettes' campaign before the war, and women's work in the war, both pushed it through.",
        ),
    ),
    (
        "the-past-named",
        "tell me about women got the vote",
        &[],
        true,
        Some(
            "Women got the vote in Britain in two steps. In 1918 women over 30 who met a property test, and all men over 21, were given the vote; in 1928 women got it at 21, on the same terms as men. The suffragettes' campaign before the war, and women's work in the war, both pushed it through.",
        ),
    ),
    (
        "the-past-named",
        "tell me about women vote",
        &[],
        true,
        Some(
            "Women got the vote in Britain in two steps. In 1918 women over 30 who met a property test, and all men over 21, were given the vote; in 1928 women got it at 21, on the same terms as men. The suffragettes' campaign before the war, and women's work in the war, both pushed it through.",
        ),
    ),
    (
        "the-past-named",
        "tell me about suffragettes",
        &[],
        true,
        Some(
            "Women got the vote in Britain in two steps. In 1918 women over 30 who met a property test, and all men over 21, were given the vote; in 1928 women got it at 21, on the same terms as men. The suffragettes' campaign before the war, and women's work in the war, both pushed it through.",
        ),
    ),
    (
        "the-past-named",
        "tell me about suffrage",
        &[],
        true,
        Some(
            "Women got the vote in Britain in two steps. In 1918 women over 30 who met a property test, and all men over 21, were given the vote; in 1928 women got it at 21, on the same terms as men. The suffragettes' campaign before the war, and women's work in the war, both pushed it through.",
        ),
    ),
    (
        "the-past-named",
        "tell me about votes for women",
        &[],
        true,
        Some(
            "Women got the vote in Britain in two steps. In 1918 women over 30 who met a property test, and all men over 21, were given the vote; in 1928 women got it at 21, on the same terms as men. The suffragettes' campaign before the war, and women's work in the war, both pushed it through.",
        ),
    ),
    (
        "the-past-named",
        "tell me about woman get the vote",
        &[],
        true,
        Some(
            "Women got the vote in Britain in two steps. In 1918 women over 30 who met a property test, and all men over 21, were given the vote; in 1928 women got it at 21, on the same terms as men. The suffragettes' campaign before the war, and women's work in the war, both pushed it through.",
        ),
    ),
    (
        "the-past-named",
        "tell me about women the vote",
        &[],
        true,
        Some(
            "Women got the vote in Britain in two steps. In 1918 women over 30 who met a property test, and all men over 21, were given the vote; in 1928 women got it at 21, on the same terms as men. The suffragettes' campaign before the war, and women's work in the war, both pushed it through.",
        ),
    ),
    (
        "the-past-named",
        "tell me about slave trade",
        &[],
        true,
        Some(
            "It depends where: Britain banned the trade in 1807 and slavery in its colonies in 1833, France abolished it in 1848, the United States in 1865 with the 13th Amendment, and Brazil was last in 1888.",
        ),
    ),
    (
        "the-past-named",
        "tell me about slavery",
        &[],
        true,
        Some(
            "It depends where: Britain banned the trade in 1807 and slavery in its colonies in 1833, France abolished it in 1848, the United States in 1865 with the 13th Amendment, and Brazil was last in 1888.",
        ),
    ),
    (
        "the-past-named",
        "tell me about slaves",
        &[],
        true,
        Some(
            "It depends where: Britain banned the trade in 1807 and slavery in its colonies in 1833, France abolished it in 1848, the United States in 1865 with the 13th Amendment, and Brazil was last in 1888.",
        ),
    ),
    (
        "the-past-named",
        "tell me about slavery in america",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about american slavery",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about slavery in the us",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about slavery in the united states",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about america abolish slavery",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about america abolished slavery",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about us abolish slavery",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about us abolished slavery",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about thirteenth amendment",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about 13th amendment",
        &[],
        true,
        Some(
            "The United States abolished slavery with the 13th Amendment, ratified in December 1865, after the Civil War. The Emancipation Proclamation of 1863 had freed slaves only in the rebel states, so the amendment was what ended it everywhere.",
        ),
    ),
    (
        "the-past-named",
        "tell me about rosa parks",
        &[],
        true,
        Some(
            "Rosa Parks was the black seamstress in Montgomery, Alabama, who on 1 December 1955 refused to give up her bus seat to a white passenger. Her arrest set off the 381-day bus boycott led by a young Martin Luther King, and the Supreme Court struck down bus segregation the next year.",
        ),
    ),
    (
        "the-past-named",
        "tell me about charles dickens",
        &[],
        true,
        Some(
            "Charles Dickens (1812-1870) was the Victorian novelist who wrote Oliver Twist, A Christmas Carol, David Copperfield and Bleak House. He worked in a boot-polish factory as a boy, and that memory of child labour runs through his books, which were published in monthly parts and read aloud across the country.",
        ),
    ),
    (
        "the-past-named",
        "tell me about dickens",
        &[],
        true,
        Some(
            "Charles Dickens (1812-1870) was the Victorian novelist who wrote Oliver Twist, A Christmas Carol, David Copperfield and Bleak House. He worked in a boot-polish factory as a boy, and that memory of child labour runs through his books, which were published in monthly parts and read aloud across the country.",
        ),
    ),
    (
        "the-past-named",
        "tell me about stalin",
        &[],
        true,
        Some(
            "Joseph Stalin (1878-1953) ruled the Soviet Union from the late 1920s until he died. He forced through collectivisation of farming and crash industrialisation, at the cost of famines that killed millions, then led the country through the Second World War after Hitler invaded in 1941.",
        ),
    ),
    (
        "the-past-named",
        "tell me about joseph stalin",
        &[],
        true,
        Some(
            "Joseph Stalin (1878-1953) ruled the Soviet Union from the late 1920s until he died. He forced through collectivisation of farming and crash industrialisation, at the cost of famines that killed millions, then led the country through the Second World War after Hitler invaded in 1941.",
        ),
    ),
    (
        "the-past-named",
        "tell me about josef stalin",
        &[],
        true,
        Some(
            "Joseph Stalin (1878-1953) ruled the Soviet Union from the late 1920s until he died. He forced through collectivisation of farming and crash industrialisation, at the cost of famines that killed millions, then led the country through the Second World War after Hitler invaded in 1941.",
        ),
    ),
    (
        "the-past-named",
        "tell me about lenin",
        &[],
        true,
        Some(
            "Vladimir Lenin (1870-1924) was the revolutionary who led the Bolsheviks to power in Russia in October 1917, pulled the country out of the First World War, and founded the Soviet state. He died in 1924 after a series of strokes, and Stalin took over the party machine.",
        ),
    ),
    (
        "the-past-named",
        "tell me about vladimir lenin",
        &[],
        true,
        Some(
            "Vladimir Lenin (1870-1924) was the revolutionary who led the Bolsheviks to power in Russia in October 1917, pulled the country out of the First World War, and founded the Soviet state. He died in 1924 after a series of strokes, and Stalin took over the party machine.",
        ),
    ),
    (
        "the-past-named",
        "tell me about difference between ww1 and ww2",
        &[],
        true,
        Some(
            "World War One was mostly trench fighting in France and Belgium, and it ended with Germany intact and a peace the next war was blamed on. World War Two was a war of movement, bombing and genocide across Europe, Africa and the Pacific, it ended with both aggressors occupied, and it killed about four times as many people.",
        ),
    ),
    (
        "the-past-named",
        "tell me about difference between the world wars",
        &[],
        true,
        Some(
            "World War One was mostly trench fighting in France and Belgium, and it ended with Germany intact and a peace the next war was blamed on. World War Two was a war of movement, bombing and genocide across Europe, Africa and the Pacific, it ended with both aggressors occupied, and it killed about four times as many people.",
        ),
    ),
    (
        "the-past-named",
        "tell me about difference between world war one and world war two",
        &[],
        true,
        Some(
            "World War One was mostly trench fighting in France and Belgium, and it ended with Germany intact and a peace the next war was blamed on. World War Two was a war of movement, bombing and genocide across Europe, Africa and the Pacific, it ended with both aggressors occupied, and it killed about four times as many people.",
        ),
    ),
    (
        "the-past-named",
        "tell me about how were the world wars different",
        &[],
        true,
        Some(
            "World War One was mostly trench fighting in France and Belgium, and it ended with Germany intact and a peace the next war was blamed on. World War Two was a war of movement, bombing and genocide across Europe, Africa and the Pacific, it ended with both aggressors occupied, and it killed about four times as many people.",
        ),
    ),
    (
        "the-past-named",
        "tell me about ww1 and ww2 difference",
        &[],
        true,
        Some(
            "World War One was mostly trench fighting in France and Belgium, and it ended with Germany intact and a peace the next war was blamed on. World War Two was a war of movement, bombing and genocide across Europe, Africa and the Pacific, it ended with both aggressors occupied, and it killed about four times as many people.",
        ),
    ),
    (
        "the-past-named",
        "tell me about vietnam war",
        &[],
        true,
        Some(
            "The Vietnam War ran from 1955 to 1975, when North Vietnam and the Viet Cong beat South Vietnam and its American backers. It killed about 3 million people, more than half of them Vietnamese civilians.",
        ),
    ),
    (
        "the-past-named",
        "tell me about war in vietnam",
        &[],
        true,
        Some(
            "The Vietnam War ran from 1955 to 1975, when North Vietnam and the Viet Cong beat South Vietnam and its American backers. It killed about 3 million people, more than half of them Vietnamese civilians.",
        ),
    ),
    (
        "the-past-named",
        "tell me about colosseum",
        &[],
        true,
        Some(
            "The Colosseum was built in Rome between 72 and 80 AD, under the emperors Vespasian and Titus, with loot from the Jewish temple in Jerusalem. It held about 50,000 people for gladiator fights and animal hunts.",
        ),
    ),
    (
        "the-past-named",
        "tell me about coliseum",
        &[],
        true,
        Some(
            "The Colosseum was built in Rome between 72 and 80 AD, under the emperors Vespasian and Titus, with loot from the Jewish temple in Jerusalem. It held about 50,000 people for gladiator fights and animal hunts.",
        ),
    ),
    (
        "the-past-named",
        "tell me about tower of london",
        &[],
        true,
        Some(
            "The Tower of London was started by William the Conqueror in the 1070s, and later kings ringed it with walls and towers. It has been a fortress, a palace, a prison and a mint, and it holds the Crown Jewels.",
        ),
    ),
    (
        "the-past-named",
        "tell me about marshall plan",
        &[],
        true,
        Some(
            "The Marshall Plan was the American programme that sent about 13 billion dollars to western Europe between 1948 and 1952 to rebuild after the war. It was offered to the Soviet Union and its neighbours as well, and Stalin refused.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first president of the united states",
        &[],
        true,
        Some(
            "George Washington was the first president of the United States, from 1789 to 1797. The first electoral college chose him unanimously and he stepped down after two terms, which set the pattern every later president followed.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first president of the usa",
        &[],
        true,
        Some(
            "George Washington was the first president of the United States, from 1789 to 1797. The first electoral college chose him unanimously and he stepped down after two terms, which set the pattern every later president followed.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first president of america",
        &[],
        true,
        Some(
            "George Washington was the first president of the United States, from 1789 to 1797. The first electoral college chose him unanimously and he stepped down after two terms, which set the pattern every later president followed.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first us president",
        &[],
        true,
        Some(
            "George Washington was the first president of the United States, from 1789 to 1797. The first electoral college chose him unanimously and he stepped down after two terms, which set the pattern every later president followed.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first american president",
        &[],
        true,
        Some(
            "George Washington was the first president of the United States, from 1789 to 1797. The first electoral college chose him unanimously and he stepped down after two terms, which set the pattern every later president followed.",
        ),
    ),
    (
        "the-past-named",
        "tell me about george washington",
        &[],
        true,
        Some(
            "George Washington was the first president of the United States, from 1789 to 1797. The first electoral college chose him unanimously and he stepped down after two terms, which set the pattern every later president followed.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first president of the us",
        &[],
        true,
        Some(
            "George Washington was the first president of the United States, from 1789 to 1797. The first electoral college chose him unanimously and he stepped down after two terms, which set the pattern every later president followed.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first president",
        &[],
        true,
        Some(
            "George Washington was the first president of the United States, from 1789 to 1797. The first electoral college chose him unanimously and he stepped down after two terms, which set the pattern every later president followed.",
        ),
    ),
    (
        "the-past-named",
        "tell me about last tsar of russia",
        &[],
        true,
        Some(
            "Nicholas II was the last tsar of Russia. He gave up the throne in March 1917 as the war and the bread queues broke his country, and he and his family were shot by the Bolsheviks in Yekaterinburg in July 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about last tsar",
        &[],
        true,
        Some(
            "Nicholas II was the last tsar of Russia. He gave up the throne in March 1917 as the war and the bread queues broke his country, and he and his family were shot by the Bolsheviks in Yekaterinburg in July 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about tsar nicholas",
        &[],
        true,
        Some(
            "Nicholas II was the last tsar of Russia. He gave up the throne in March 1917 as the war and the bread queues broke his country, and he and his family were shot by the Bolsheviks in Yekaterinburg in July 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about nicholas ii",
        &[],
        true,
        Some(
            "Nicholas II was the last tsar of Russia. He gave up the throne in March 1917 as the war and the bread queues broke his country, and he and his family were shot by the Bolsheviks in Yekaterinburg in July 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about tsar of russia",
        &[],
        true,
        Some(
            "Nicholas II was the last tsar of Russia. He gave up the throne in March 1917 as the war and the bread queues broke his country, and he and his family were shot by the Bolsheviks in Yekaterinburg in July 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about nicholas the second",
        &[],
        true,
        Some(
            "Nicholas II was the last tsar of Russia. He gave up the throne in March 1917 as the war and the bread queues broke his country, and he and his family were shot by the Bolsheviks in Yekaterinburg in July 1918.",
        ),
    ),
    (
        "the-past-named",
        "tell me about jfk",
        &[],
        true,
        Some(
            "John F. Kennedy was the 35th US president, from 1961 until he was shot in Dallas on 22 November 1963. He faced the Soviet Union down over Cuba in 1962 and set the goal of landing a man on the moon before the decade was out.",
        ),
    ),
    (
        "the-past-named",
        "tell me about john f kennedy",
        &[],
        true,
        Some(
            "John F. Kennedy was the 35th US president, from 1961 until he was shot in Dallas on 22 November 1963. He faced the Soviet Union down over Cuba in 1962 and set the goal of landing a man on the moon before the decade was out.",
        ),
    ),
    (
        "the-past-named",
        "tell me about john fitzgerald kennedy",
        &[],
        true,
        Some(
            "John F. Kennedy was the 35th US president, from 1961 until he was shot in Dallas on 22 November 1963. He faced the Soviet Union down over Cuba in 1962 and set the goal of landing a man on the moon before the decade was out.",
        ),
    ),
    (
        "the-past-named",
        "tell me about president kennedy",
        &[],
        true,
        Some(
            "John F. Kennedy was the 35th US president, from 1961 until he was shot in Dallas on 22 November 1963. He faced the Soviet Union down over Cuba in 1962 and set the goal of landing a man on the moon before the decade was out.",
        ),
    ),
    (
        "the-past-named",
        "tell me about kennedy",
        &[],
        true,
        Some(
            "John F. Kennedy was the 35th US president, from 1961 until he was shot in Dallas on 22 November 1963. He faced the Soviet Union down over Cuba in 1962 and set the goal of landing a man on the moon before the decade was out.",
        ),
    ),
    (
        "the-past-named",
        "tell me about anne boleyn",
        &[],
        true,
        Some(
            "Anne Boleyn was Henry VIII's second wife, from 1533 to 1536, and the mother of Elizabeth I. When she gave him no son he had her tried for adultery and executed at the Tower of London in May 1536.",
        ),
    ),
    (
        "the-past-named",
        "tell me about first emperor of china",
        &[],
        true,
        Some(
            "Qin Shi Huang was the first emperor of a united China, from 221 to 210 BC. He standardised writing, money and weights, joined up the Great Wall, and was buried with the terracotta army.",
        ),
    ),
    (
        "the-past-named",
        "tell me about qin shi huang",
        &[],
        true,
        Some(
            "Qin Shi Huang was the first emperor of a united China, from 221 to 210 BC. He standardised writing, money and weights, joined up the Great Wall, and was buried with the terracotta army.",
        ),
    ),
    (
        "the-past-named",
        "tell me about qin shihuang",
        &[],
        true,
        Some(
            "Qin Shi Huang was the first emperor of a united China, from 221 to 210 BC. He standardised writing, money and weights, joined up the Great Wall, and was buried with the terracotta army.",
        ),
    ),
    (
        "the-past-named",
        "tell me about emperor of china",
        &[],
        true,
        Some(
            "Qin Shi Huang was the first emperor of a united China, from 221 to 210 BC. He standardised writing, money and weights, joined up the Great Wall, and was buried with the terracotta army.",
        ),
    ),
    (
        "the-past-named",
        "how many people died in world war one",
        &[],
        true,
        Some(
            "About 17 million people died in World War One, roughly 9 to 10 million of them soldiers, and about 20 million more were wounded.",
        ),
    ),
    (
        "the-past-named",
        "how long did world war one last",
        &[],
        true,
        Some(
            "World War One lasted four years and three months, from 28 July 1914 to the armistice on 11 November 1918.",
        ),
    ),
    (
        "the-past-named",
        "how long did world war two last",
        &[],
        true,
        Some(
            "World War Two lasted six years and a day, from 1 September 1939 to 2 September 1945.",
        ),
    ),
    (
        "the-past-named",
        "how many people died in isambard kingdom brunel",
        &[],
        true,
        Some(
            "Brunel died on 15 September 1859, aged 53, of a stroke, ten days after the first, unsuccessful launch of the Great Eastern. He had been ill for most of that year and the ship's failures were blamed for it. He is buried at Kensal Green; the Royal Albert Bridge at Saltash, finished in the year he died, is his last work.",
        ),
    ),
    (
        "the-past-named",
        "what did isambard kingdom brunel build",
        &[],
        true,
        Some(
            "Brunel built the Great Western Railway from London to Bristol, the Clifton Suspension Bridge over the Avon, the SS Great Britain (the first iron ship to cross the Atlantic under screw propeller), the Great Eastern, and the Box Tunnel and the viaducts and bridges along the GWR route. His last was the Royal Albert Bridge at Saltash, finished in the year he died.",
        ),
    ),
    (
        "the-past-named",
        "what did celts build",
        &[],
        true,
        Some(
            "The Celts built hillforts on high ground, with banks and ditches of earth and timber - Maiden Castle in Dorset is the largest in Britain - and lived in roundhouses of timber, wattle and thatch. They worked iron for swords, tools and ploughs, and fired pots without a wheel.",
        ),
    ),
    (
        "the-past-named",
        "how many people died in admiral nelson",
        &[],
        true,
        Some(
            "Nelson was shot by a French marksman on the quarterdeck of HMS Victory at Trafalgar on 21 October 1805, and died below decks about three hours later, with the battle already won. His body was put in a barrel of brandy for the sail home.",
        ),
    ),
    (
        "the-past-named",
        "how long did vietnam war last",
        &[],
        true,
        Some(
            "The Vietnam War ran about 20 years, from 1955 to the fall of Saigon on 30 April 1975; American combat troops were there from 1965 to 1973.",
        ),
    ),
    (
        "the-past-named",
        "was waterloo station opened in 1848",
        &[],
        false,
        None,
    ),
    (
        "the-past-named",
        "is attila a good name for my cat",
        &[],
        false,
        None,
    ),
    (
        "the-past-named",
        "why did stalin take power",
        &[],
        true,
        Some(
            "Joseph Stalin (1878-1953) ruled the Soviet Union from the late 1920s until he died. He forced through collectivisation of farming and crash industrialisation, at the cost of famines that killed millions, then led the country through the Second World War after Hitler invaded in 1941.",
        ),
    ),
    (
        "the-past-named",
        "tell me about napoleon and the berlin wall",
        &[],
        false,
        None,
    ),
    (
        "the-past-named",
        "what happened in word war one",
        &[],
        false,
        None,
    ),
    (
        "the-past-named",
        "who was napoleon and nelson",
        &[],
        true,
        Some(
            "Horatio Nelson (1758-1805) was the British admiral who beat the French fleet at the Nile in 1798 and the combined French and Spanish fleet at Trafalgar in 1805, where he was shot and died on his own ship. He lost an arm and an eye in earlier actions and is remembered for the signal 'England expects that every man will do his duty'.",
        ),
    ),
    (
        "the-past-named",
        "what did he build",
        &[("who was brunel", "")],
        true,
        Some(
            "Brunel built the Great Western Railway from London to Bristol, the Clifton Suspension Bridge over the Avon, the SS Great Britain (the first iron ship to cross the Atlantic under screw propeller), the Great Eastern, and the Box Tunnel and the viaducts and bridges along the GWR route. His last was the Royal Albert Bridge at Saltash, finished in the year he died.",
        ),
    ),
    (
        "the-past-named",
        "what did he build",
        &[("who was brunel", ""), ("it's going to rain", "yes")],
        true,
        None,
    ),
    (
        "the-past-follow-on",
        "what did he build",
        &[("who was brunel", "")],
        true,
        Some(
            "Brunel built the Great Western Railway from London to Bristol, the Clifton Suspension Bridge over the Avon, the SS Great Britain (the first iron ship to cross the Atlantic under screw propeller), the Great Eastern, and the Box Tunnel and the viaducts and bridges along the GWR route. His last was the Royal Albert Bridge at Saltash, finished in the year he died.",
        ),
    ),
    (
        "the-past-follow-on",
        "how did he die",
        &[("who was nelson", "Admiral Nelson fought at Trafalgar.")],
        true,
        Some(
            "Nelson was shot by a French marksman on the quarterdeck of HMS Victory at Trafalgar on 21 October 1805, and died below decks about three hours later, with the battle already won. His body was put in a barrel of brandy for the sail home.",
        ),
    ),
    (
        "the-past-follow-on",
        "how did he die",
        &[("who was nelson", "")],
        true,
        Some(
            "Nelson was shot by a French marksman on the quarterdeck of HMS Victory at Trafalgar on 21 October 1805, and died below decks about three hours later, with the battle already won. His body was put in a barrel of brandy for the sail home.",
        ),
    ),
    (
        "the-past-follow-on",
        "how did he die",
        &[("who was nelson", ""), ("what about the cat", "right")],
        true,
        None,
    ),
    (
        "the-past-follow-on",
        "what did he build right after getting home during his career",
        &[("who was brunel", "")],
        false,
        Some(
            "Brunel built the Great Western Railway from London to Bristol, the Clifton Suspension Bridge over the Avon, the SS Great Britain (the first iron ship to cross the Atlantic under screw propeller), the Great Eastern, and the Box Tunnel and the viaducts and bridges along the GWR route. His last was the Royal Albert Bridge at Saltash, finished in the year he died.",
        ),
    ),
    (
        "the-past-follow-on",
        "who did he fight",
        &[("what was the battle of hastings", "")],
        true,
        None,
    ),
    (
        "the-past-follow-on",
        "why did they build the berlin wall",
        &[("tell me about nelson", "")],
        false,
        Some(
            "Horatio Nelson (1758-1805) was the British admiral who beat the French fleet at the Nile in 1798 and the combined French and Spanish fleet at Trafalgar in 1805, where he was shot and died on his own ship. He lost an arm and an eye in earlier actions and is remembered for the signal 'England expects that every man will do his duty'.",
        ),
    ),
    (
        "the-past-follow-on",
        "what else",
        &[(
            "did anything interesting happen",
            "The Berlin Wall fell in 1989.",
        )],
        true,
        None,
    ),
    (
        "the-past-follow-on",
        "what else",
        &[("what is Waterloo", "Waterloo station is in London.")],
        true,
        None,
    ),
    (
        "the-past-follow-on",
        "where did he go",
        &[("who was henry viii", "")],
        true,
        None,
    ),
    ("the-past-follow-on", "how did he die", &[], true, None),
    (
        "the-past-follow-on",
        "how did he?",
        &[("tell me about nelson", "")],
        true,
        Some(
            "Horatio Nelson (1758-1805) was the British admiral who beat the French fleet at the Nile in 1798 and the combined French and Spanish fleet at Trafalgar in 1805, where he was shot and died on his own ship. He lost an arm and an eye in earlier actions and is remembered for the signal 'England expects that every man will do his duty'.",
        ),
    ),
    (
        "the-past-follow-on",
        "what was that",
        &[("what was the industrial revolution", "")],
        true,
        None,
    ),
    (
        "the-past-follow-on",
        "when did he get that job",
        &[("who was nelson", "")],
        true,
        Some(
            "Horatio Nelson (1758-1805) was the British admiral who beat the French fleet at the Nile in 1798 and the combined French and Spanish fleet at Trafalgar in 1805, where he was shot and died on his own ship. He lost an arm and an eye in earlier actions and is remembered for the signal 'England expects that every man will do his duty'.",
        ),
    ),
    (
        "who-invented-it",
        "who invented aeroplane",
        &[],
        true,
        Some(
            "The Wright brothers, Orville and Wilbur, made the first powered aeroplane flight on 17 December 1903 at Kitty Hawk: 12 seconds and 36 metres. Their edge was working out how to control a wing, not just how to lift one.",
        ),
    ),
    (
        "who-invented-it",
        "who invented airplane",
        &[],
        true,
        Some(
            "The Wright brothers, Orville and Wilbur, made the first powered aeroplane flight on 17 December 1903 at Kitty Hawk: 12 seconds and 36 metres. Their edge was working out how to control a wing, not just how to lift one.",
        ),
    ),
    (
        "who-invented-it",
        "who invented first flight",
        &[],
        false,
        None,
    ),
    (
        "who-invented-it",
        "who invented first plane",
        &[],
        true,
        Some(
            "The Wright brothers, Orville and Wilbur, made the first powered aeroplane flight on 17 December 1903 at Kitty Hawk: 12 seconds and 36 metres. Their edge was working out how to control a wing, not just how to lift one.",
        ),
    ),
    (
        "who-invented-it",
        "who invented car",
        &[],
        true,
        Some(
            "Karl Benz built the first car with a petrol engine in 1885 and patented it in 1886, and his wife Bertha drove it 100 km the next year to prove it worked. Henry Ford's Model T, from 1908, was the one that made cars ordinary.",
        ),
    ),
    (
        "who-invented-it",
        "who invented cars",
        &[],
        true,
        Some(
            "Karl Benz built the first car with a petrol engine in 1885 and patented it in 1886, and his wife Bertha drove it 100 km the next year to prove it worked. Henry Ford's Model T, from 1908, was the one that made cars ordinary.",
        ),
    ),
    (
        "who-invented-it",
        "who invented automobile",
        &[],
        true,
        Some(
            "Karl Benz built the first car with a petrol engine in 1885 and patented it in 1886, and his wife Bertha drove it 100 km the next year to prove it worked. Henry Ford's Model T, from 1908, was the one that made cars ordinary.",
        ),
    ),
    (
        "who-invented-it",
        "who invented motor car",
        &[],
        true,
        Some(
            "Karl Benz built the first car with a petrol engine in 1885 and patented it in 1886, and his wife Bertha drove it 100 km the next year to prove it worked. Henry Ford's Model T, from 1908, was the one that made cars ordinary.",
        ),
    ),
    (
        "who-invented-it",
        "who invented television",
        &[],
        true,
        Some(
            "John Logie Baird gave the first public demonstration of television in London in 1926, using a spinning disc. Philo Farnsworth's all-electronic system, shown in 1927, is the one every modern set works on.",
        ),
    ),
    (
        "who-invented-it",
        "who invented tv",
        &[],
        true,
        Some(
            "John Logie Baird gave the first public demonstration of television in London in 1926, using a spinning disc. Philo Farnsworth's all-electronic system, shown in 1927, is the one every modern set works on.",
        ),
    ),
    (
        "who-invented-it",
        "who invented steam engine",
        &[],
        true,
        Some(
            "Thomas Newcomen built the first practical steam engine in 1712, to pump water out of mines, and James Watt improved it from 1769 with a separate condenser that wasted far less heat. Watt's engine is the one that powered the Industrial Revolution.",
        ),
    ),
    (
        "who-invented-it",
        "who invented steam engines",
        &[],
        true,
        Some(
            "Thomas Newcomen built the first practical steam engine in 1712, to pump water out of mines, and James Watt improved it from 1769 with a separate condenser that wasted far less heat. Watt's engine is the one that powered the Industrial Revolution.",
        ),
    ),
    (
        "who-invented-it",
        "who invented electricity",
        &[],
        true,
        Some(
            "Nobody invented electricity: it is a force in nature, and people learned to make and use it over about 150 years. Franklin showed lightning was electric in 1752, Volta made the first battery in 1800, Faraday built the first generator in 1831, and Edison opened the first power station in 1882.",
        ),
    ),
    (
        "who-invented-it",
        "who invented electric power",
        &[],
        true,
        Some(
            "Nobody invented electricity: it is a force in nature, and people learned to make and use it over about 150 years. Franklin showed lightning was electric in 1752, Volta made the first battery in 1800, Faraday built the first generator in 1831, and Edison opened the first power station in 1882.",
        ),
    ),
    (
        "who-invented-it",
        "who invented internet",
        &[],
        true,
        Some(
            "No one person invented the internet: ARPANET linked four American university computers in 1969 and the network grew from there. Tim Berners-Lee invented the world wide web on top of it in 1989, at CERN, so documents could be linked.",
        ),
    ),
    (
        "who-invented-it",
        "who invented world wide web",
        &[],
        true,
        Some(
            "No one person invented the internet: ARPANET linked four American university computers in 1969 and the network grew from there. Tim Berners-Lee invented the world wide web on top of it in 1989, at CERN, so documents could be linked.",
        ),
    ),
    (
        "who-invented-it",
        "who invented the web",
        &[],
        true,
        Some(
            "No one person invented the internet: ARPANET linked four American university computers in 1969 and the network grew from there. Tim Berners-Lee invented the world wide web on top of it in 1989, at CERN, so documents could be linked.",
        ),
    ),
    (
        "who-invented-it",
        "who invented the car and the steam engine",
        &[],
        true,
        Some(
            "Thomas Newcomen built the first practical steam engine in 1712, to pump water out of mines, and James Watt improved it from 1769 with a separate condenser that wasted far less heat. Watt's engine is the one that powered the Industrial Revolution.",
        ),
    ),
    (
        "who-invented-it",
        "how much did the first car cost",
        &[],
        false,
        None,
    ),
    (
        "who-invented-it",
        "who invented tv for children",
        &[],
        true,
        Some(
            "John Logie Baird gave the first public demonstration of television in London in 1926, using a spinning disc. Philo Farnsworth's all-electronic system, shown in 1927, is the one every modern set works on.",
        ),
    ),
    (
        "who-invented-it",
        "what is a name for a cat from television",
        &[],
        false,
        None,
    ),
    (
        "who-invented-it",
        "who invented the television and the telephone",
        &[],
        true,
        Some(
            "John Logie Baird gave the first public demonstration of television in London in 1926, using a spinning disc. Philo Farnsworth's all-electronic system, shown in 1927, is the one every modern set works on.",
        ),
    ),
    (
        "who-invented-it",
        "who invented the web and electricity",
        &[],
        true,
        Some(
            "Nobody invented electricity: it is a force in nature, and people learned to make and use it over about 150 years. Franklin showed lightning was electric in 1752, Volta made the first battery in 1800, Faraday built the first generator in 1831, and Edison opened the first power station in 1882.",
        ),
    ),
    (
        "who-invented-it",
        "who invented the electronic chip",
        &[],
        false,
        None,
    ),
];
