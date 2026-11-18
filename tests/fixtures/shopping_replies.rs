//! Direct pinned-source shopping reply outcomes.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [(&'static str, &'static str)],
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "gift-dad",
        "what do i get my dad for his birthday",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones.",
        ),
    ),
    Case(
        "gift-dad",
        "gift for my dad who doesnt drink",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "gift-dad",
        "a present for my dad who gave up alcohol",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones.",
        ),
    ),
    Case(
        "gift-dad",
        "gift for my dad who is vegan",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. And buy into the vegetarian side of it: a good vegetarian cookbook, a voucher for a veggie restaurant, or a decent olive oil and a spice set rather than a hamper of pate.",
        ),
    ),
    Case(
        "gift-dad",
        "my dad has a nut allergy gift ideas",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. And nothing that might hold what they cannot eat: a voucher, a plant, or something for a hobby beats a hamper or a box of chocolates.",
        ),
    ),
    Case(
        "gift-dad",
        "he is diabetic",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. Keep the sugar out of it: no chocolates or biscuit hamper - a magazine or audiobook subscription, or something for a hobby they already have, works better.",
        ),
    ),
    Case(
        "gift-dad",
        "gift for a diabetic vegetarian dad",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. And buy into the vegetarian side of it: a good vegetarian cookbook, a voucher for a veggie restaurant, or a decent olive oil and a spice set rather than a hamper of pate.",
        ),
    ),
    Case(
        "gift-dad",
        "gift for a teetotal vegetarian dad",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "gift-dad",
        "dad wants no sugar",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. Keep the sugar out of it: no chocolates or biscuit hamper - a magazine or audiobook subscription, or something for a hobby they already have, works better.",
        ),
    ),
    Case(
        "gift-dad",
        "gift for a grandfather",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones.",
        ),
    ),
    Case(
        "gift-dad",
        "gift for a sober dad",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "gift-dad",
        "gift for someone with lactose intolerance",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. And nothing that might hold what they cannot eat: a voucher, a plant, or something for a hobby beats a hamper or a box of chocolates.",
        ),
    ),
    Case(
        "gift-dad",
        "Dad is a non-drinker; what's a good present?",
        &[],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "gift-mum",
        "gift for mum",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything.",
        ),
    ),
    Case(
        "gift-mum",
        "what should i buy my mother who is coeliac",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. And nothing that might hold what they cannot eat: a voucher, a plant, or something for a hobby beats a hamper or a box of chocolates.",
        ),
    ),
    Case(
        "gift-mum",
        "gift for nana who is vegan",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. And buy into the vegetarian side of it: a good vegetarian cookbook, a voucher for a veggie restaurant, or a decent olive oil and a spice set rather than a hamper of pate.",
        ),
    ),
    Case(
        "gift-mum",
        "gift for someone with an allergy",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. And nothing that might hold what they cannot eat: a voucher, a plant, or something for a hobby beats a hamper or a box of chocolates.",
        ),
    ),
    Case(
        "gift-mum",
        "gift ideas for my mum who doesnt eat meat",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. And buy into the vegetarian side of it: a good vegetarian cookbook, a voucher for a veggie restaurant, or a decent olive oil and a spice set rather than a hamper of pate.",
        ),
    ),
    Case(
        "gift-mum",
        "need a present for diabetic grandma",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. Keep the sugar out of it: no chocolates or biscuit hamper - a magazine or audiobook subscription, or something for a hobby they already have, works better.",
        ),
    ),
    Case(
        "gift-mum",
        "ideas for a pint and a gift for a sober mum",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "gift-mum",
        "christmas present for my mum who is non drinking",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything.",
        ),
    ),
    Case(
        "gift-mum",
        "gift for a mum with allergies",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. And nothing that might hold what they cannot eat: a voucher, a plant, or something for a hobby beats a hamper or a box of chocolates.",
        ),
    ),
    Case(
        "gift-mum",
        "gift for grandma who is watching sugar",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. Keep the sugar out of it: no chocolates or biscuit hamper - a magazine or audiobook subscription, or something for a hobby they already have, works better.",
        ),
    ),
    Case(
        "gift-mum",
        "gift for my nan a pescatarian",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. And buy into the vegetarian side of it: a good vegetarian cookbook, a voucher for a veggie restaurant, or a decent olive oil and a spice set rather than a hamper of pate.",
        ),
    ),
    Case(
        "gift-mum",
        "what should i get my mum who has no alcohol",
        &[],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "watch-buy",
        "should i buy a watch",
        &[],
        Some(
            "For a first proper watch, a plain quartz from Casio or Timex at £40 to £150 keeps better time than anything dearer; a mechanical watch is for people who enjoy them, not for accuracy. Look for a sapphire crystal if you can, 50m water resistance and a strap you can change yourself. What will you mostly wear it with?",
        ),
    ),
    Case("watch-buy", "what should i watch this evening", &[], None),
    Case(
        "watch-buy",
        "a watch for running",
        &[],
        Some(
            "Buy for the battery first: most smartwatches last a day or two, so one that lasts a week changes how you use it. A fitness band at £40 to £80 covers steps, sleep and notifications; pay £200 to £300 only if you want a screen and apps on your wrist, and expect the step and heart-rate numbers to be rough rather than medical. Is it for exercise, notifications, or looks?",
        ),
    ),
    Case(
        "watch-buy",
        "watch for steps",
        &[],
        Some(
            "Buy for the battery first: most smartwatches last a day or two, so one that lasts a week changes how you use it. A fitness band at £40 to £80 covers steps, sleep and notifications; pay £200 to £300 only if you want a screen and apps on your wrist, and expect the step and heart-rate numbers to be rough rather than medical. Is it for exercise, notifications, or looks?",
        ),
    ),
    Case("watch-buy", "watch for symptoms", &[], None),
    Case("watch-buy", "watch fOr changes", &[], None),
    Case(
        "watch-buy",
        "watch for swelling",
        &[],
        Some(
            "For a first proper watch, a plain quartz from Casio or Timex at £40 to £150 keeps better time than anything dearer; a mechanical watch is for people who enjoy them, not for accuracy. Look for a sapphire crystal if you can, 50m water resistance and a strap you can change yourself. What will you mostly wear it with?",
        ),
    ),
    Case("watch-buy", "things to watch out for", &[], None),
    Case("watch-buy", "anything to watch for?", &[], None),
    Case("watch-buy", "how do i watch on netflix", &[], None),
    Case(
        "watch-buy",
        "what app should i buy for my apple watch",
        &[],
        Some(
            "Buy for the battery first: most smartwatches last a day or two, so one that lasts a week changes how you use it. A fitness band at £40 to £80 covers steps, sleep and notifications; pay £200 to £300 only if you want a screen and apps on your wrist, and expect the step and heart-rate numbers to be rough rather than medical. Is it for exercise, notifications, or looks?",
        ),
    ),
    Case(
        "watch-buy",
        "is a watch worth buying",
        &[],
        Some(
            "For a first proper watch, a plain quartz from Casio or Timex at £40 to £150 keeps better time than anything dearer; a mechanical watch is for people who enjoy them, not for accuracy. Look for a sapphire crystal if you can, 50m water resistance and a strap you can change yourself. What will you mostly wear it with?",
        ),
    ),
    Case("watch-buy", "is it worth watching", &[], None),
    Case("watch-buy", "a watch for me", &[], None),
    Case("watch-buy", "who watched football", &[], None),
    Case("watch-buy", "safe to watch it", &[], None),
    Case(
        "watch-buy",
        "galaxy watch",
        &[],
        Some(
            "Buy for the battery first: most smartwatches last a day or two, so one that lasts a week changes how you use it. A fitness band at £40 to £80 covers steps, sleep and notifications; pay £200 to £300 only if you want a screen and apps on your wrist, and expect the step and heart-rate numbers to be rough rather than medical. Is it for exercise, notifications, or looks?",
        ),
    ),
    Case(
        "watch-buy",
        "should i watch the smartwatch announcement",
        &[],
        Some(
            "Buy for the battery first: most smartwatches last a day or two, so one that lasts a week changes how you use it. A fitness band at £40 to £80 covers steps, sleep and notifications; pay £200 to £300 only if you want a screen and apps on your wrist, and expect the step and heart-rate numbers to be rough rather than medical. Is it for exercise, notifications, or looks?",
        ),
    ),
    Case(
        "watch-buy",
        "should i get a car watch",
        &[],
        Some(
            "For a first proper watch, a plain quartz from Casio or Timex at £40 to £150 keeps better time than anything dearer; a mechanical watch is for people who enjoy them, not for accuracy. Look for a sapphire crystal if you can, 50m water resistance and a strap you can change yourself. What will you mostly wear it with?",
        ),
    ),
    Case(
        "watch-buy",
        "which smart watches",
        &[],
        Some(
            "Buy for the battery first: most smartwatches last a day or two, so one that lasts a week changes how you use it. A fitness band at £40 to £80 covers steps, sleep and notifications; pay £200 to £300 only if you want a screen and apps on your wrist, and expect the step and heart-rate numbers to be rough rather than medical. Is it for exercise, notifications, or looks?",
        ),
    ),
    Case("watch-buy", "a watch for signs", &[], None),
    Case(
        "watch-buy",
        "should i buy an apple-watch",
        &[],
        Some(
            "Buy for the battery first: most smartwatches last a day or two, so one that lasts a week changes how you use it. A fitness band at £40 to £80 covers steps, sleep and notifications; pay £200 to £300 only if you want a screen and apps on your wrist, and expect the step and heart-rate numbers to be rough rather than medical. Is it for exercise, notifications, or looks?",
        ),
    ),
    Case(
        "coat-worth",
        "how should i choose a coat",
        &[],
        Some(
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders.",
        ),
    ),
    Case(
        "coat-worth",
        "what should i look for in a coat",
        &[],
        Some(
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders. Check the hood stays up in wind, the zip works with gloves on, the seams are taped and there is room for a jumper underneath. Look at the lining where it rubs under the arm and at the stitching on the pockets: those are the two places a cheap coat shows itself first.",
        ),
    ),
    Case(
        "coat-worth",
        "coat for wet weather",
        &[],
        Some(
            "For rain, a waterproof shell with taped seams (Gore-Tex or similar) at £150 to £250 outlasts three cheap ones: check the hood, the zip and the seams under the arms, and try it on over a jumper. A shower-resistant jacket is fine for a dash to the shops and no good for a wet walk.",
        ),
    ),
    Case(
        "coat-worth",
        "does real down work for rain",
        &[],
        Some(
            "Down is the warmest for its weight and packs down small, so look for 600 fill power or more and it will last a decade, but it insulates badly once it is wet. Synthetic padding costs about half as much, is heavier and bulkier, and still keeps you warm when damp, which suits a wet winter better than down unless you want it for real cold.",
        ),
    ),
    Case(
        "coat-worth",
        "a wool coat for a wedding",
        &[],
        Some(
            "For anything smart - a wedding, a funeral, a winter interview - a wool or wool-blend coat is the answer: it hangs properly over a dress or a suit, and a mid-calf length in navy, grey, camel or black goes with everything. Check that it closes over the jacket you will wear underneath and try it on in the shoes you will be standing in, because a heavy coat pushes the shoulders up.",
        ),
    ),
    Case(
        "coat-worth",
        "work coat for rain",
        &[],
        Some(
            "For anything smart - a wedding, a funeral, a winter interview - a wool or wool-blend coat is the answer: it hangs properly over a dress or a suit, and a mid-calf length in navy, grey, camel or black goes with everything. Check that it closes over the jacket you will wear underneath and try it on in the shoes you will be standing in, because a heavy coat pushes the shoulders up.",
        ),
    ),
    Case(
        "coat-worth",
        "school coat for showery weather",
        &[],
        Some(
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders.",
        ),
    ),
    Case(
        "coat-worth",
        "quilted for a wedding",
        &[],
        Some(
            "Down is the warmest for its weight and packs down small, so look for 600 fill power or more and it will last a decade, but it insulates badly once it is wet. Synthetic padding costs about half as much, is heavier and bulkier, and still keeps you warm when damp, which suits a wet winter better than down unless you want it for real cold.",
        ),
    ),
    Case(
        "coat-worth",
        "is a fancy winter coat worth it",
        &[],
        Some(
            "Yes, if it is one you will wear for years: a coat is the piece of clothing where the money shows, and £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders. Cheap coats cost more in the end because the zip goes and the seams soak through, so buy the best you can in a plain colour you will still wear in five years.",
        ),
    ),
    Case(
        "coat-worth",
        "where coat, fit over arm",
        &[],
        Some(
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders. Check the hood stays up in wind, the zip works with gloves on, the seams are taped and there is room for a jumper underneath. Look at the lining where it rubs under the arm and at the stitching on the pockets: those are the two places a cheap coat shows itself first.",
        ),
    ),
    Case(
        "coat-worth",
        "where should i look a raincoat",
        &[],
        Some(
            "For rain, a waterproof shell with taped seams (Gore-Tex or similar) at £150 to £250 outlasts three cheap ones: check the hood, the zip and the seams under the arms, and try it on over a jumper. A shower-resistant jacket is fine for a dash to the shops and no good for a wet walk.",
        ),
    ),
    Case(
        "coat-worth",
        "jacket length at 5 foot 4",
        &[],
        Some(
            "At 5 foot 4, look for a coat about 26 to 32 inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own.",
        ),
    ),
    Case(
        "coat-worth",
        "im about 6 feet 2",
        &[],
        Some(
            "At 6 foot 2, look for a coat about 31 to 37 inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own.",
        ),
    ),
    Case(
        "coat-worth",
        "around 5ft 07",
        &[],
        Some(
            "At 5 foot 07, look for a coat about 28 to 33 inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own.",
        ),
    ),
    Case(
        "coat-worth",
        "im 5'11",
        &[],
        Some(
            "At 5 foot 11, look for a coat about 29 to 35 inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own.",
        ),
    ),
    Case(
        "coat-worth",
        "HOW DO I PICK THE RIGHT LENGTH AT 5 FT 4",
        &[],
        Some(
            "At 5 foot 4, look for a coat about 26 to 32 inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own.",
        ),
    ),
    Case(
        "coat-worth",
        "what is good at 5 foot 19",
        &[],
        Some(
            "At 5 foot 19, look for a coat about 33 to 39 inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own.",
        ),
    ),
    Case(
        "coat-worth",
        "coat for ٤ foot ٧",
        &[],
        Some(
            "At ٤ foot ٧, look for a coat about 23 to 27 inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own.",
        ),
    ),
    Case(
        "coat-worth",
        "how much should i pay for a puffer",
        &[],
        Some(
            "Down is the warmest for its weight and packs down small, so look for 600 fill power or more and it will last a decade, but it insulates badly once it is wet. Synthetic padding costs about half as much, is heavier and bulkier, and still keeps you warm when damp, which suits a wet winter better than down unless you want it for real cold.",
        ),
    ),
    Case(
        "coat-worth",
        "does a damp shoulder matter",
        &[],
        Some(
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders.",
        ),
    ),
    Case(
        "coat-worth",
        "will it fit me",
        &[],
        Some(
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders. Check the hood stays up in wind, the zip works with gloves on, the seams are taped and there is room for a jumper underneath. Look at the lining where it rubs under the arm and at the stitching on the pockets: those are the two places a cheap coat shows itself first.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where do i buy batteries",
        &[],
        Some(
            "A supermarket, a hardware shop or a pound shop has them. Take the old one with you, or photograph the numbers printed on it, because the size is the thing people get wrong: a bulb needs its fitting and its wattage, a battery its number, a fuse its amps.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where can i buy a birthday card",
        &[],
        Some(
            "A card shop, a supermarket or a petrol station will have one, and the supermarket's own range is two pounds rather than four. If it is a big one, buy it this week rather than the day before - the good ones go first.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where can i buy signed for stamps",
        &[],
        Some(
            "Any post office counter has them, and so do most supermarkets, corner shops and newsagents. A book of 12 costs less per stamp than singles, and Royal Mail's own site will post a book out if you are not going past a counter.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where do i get a camping stove",
        &[],
        Some(
            "An outdoor shop is where the real ones are: Decathlon for the cheap end, Go Outdoors or a proper mountaineering shop for the rest. A charity shop or a car boot sale often has a decent sleeping bag for a tenner, and a borrowed one for a first trip is better than buying.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where can i find the light bulb in the loft",
        &[],
        None,
    ),
    Case(
        "where-to-buy-everyday",
        "where do i find my tent poles",
        &[],
        None,
    ),
    Case("where-to-buy-everyday", "where can i find them", &[], None),
    Case(
        "where-to-buy-everyday",
        "can i find paint in the shop",
        &[],
        Some(
            "A decorator's merchant rather than a chain for the paint itself: the trade brands cover better, cost less and they will mix a colour to a sample. For one room, a DIY shop for the brushes, trays and sheets is fine, and cheap brushes are the false economy - they shed.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "can i find batteries on the website",
        &[],
        None,
    ),
    Case(
        "where-to-buy-everyday",
        "where can i find a fuse in the remote",
        &[],
        None,
    ),
    Case(
        "where-to-buy-everyday",
        "where can i get a moving box",
        &[],
        None,
    ),
    Case(
        "where-to-buy-everyday",
        "do they sell primer and paper",
        &[],
        Some(
            "A decorator's merchant rather than a chain for the paint itself: the trade brands cover better, cost less and they will mix a colour to a sample. For one room, a DIY shop for the brushes, trays and sheets is fine, and cheap brushes are the false economy - they shed.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where can i get a birthday card and a sleeping bag",
        &[],
        Some(
            "A card shop, a supermarket or a petrol station will have one, and the supermarket's own range is two pounds rather than four. If it is a big one, buy it this week rather than the day before - the good ones go first.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "i want some wrapping paper",
        &[],
        Some(
            "A stationery shop or the back-to-school aisle of a supermarket has it, and for a batch envelopes or a box of paper an office supplier is cheaper and delivers. A card shop sells envelopes on their own if you only need a few.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "do you know where to buy boxes",
        &[],
        Some(
            "A supermarket will give you banana boxes free if you ask at the customer desk, and a removal firm sells proper double-walled ones for a few pounds each. Ask on a Monday, before the delivery cages are taken back.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "a pen or envelopes",
        &[],
        Some(
            "A stationery shop or the back-to-school aisle of a supermarket has it, and for a batch envelopes or a box of paper an office supplier is cheaper and delivers. A card shop sells envelopes on their own if you only need a few.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where do i get birthday cards in the chemist's",
        &[],
        Some(
            "A card shop, a supermarket or a petrol station will have one, and the supermarket's own range is two pounds rather than four. If it is a big one, buy it this week rather than the day before - the good ones go first.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where do i buy a replacement bulb",
        &[],
        Some(
            "A supermarket, a hardware shop or a pound shop has them. Take the old one with you, or photograph the numbers printed on it, because the size is the thing people get wrong: a bulb needs its fitting and its wattage, a battery its number, a fuse its amps.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "where do i buy a penelope notebook",
        &[],
        None,
    ),
    Case(
        "where-to-buy-everyday",
        "where can i get first class light bulb",
        &[],
        Some(
            "Any post office counter has them, and so do most supermarkets, corner shops and newsagents. A book of 12 costs less per stamp than singles, and Royal Mail's own site will post a book out if you are not going past a counter.",
        ),
    ),
    Case(
        "where-to-buy-everyday",
        "can i get a light bulb under my stairs",
        &[],
        None,
    ),
    Case(
        "haggle-price",
        "is it rude to haggle",
        &[],
        Some(
            "No - asking once is normal in a market, a car boot sale, a small independent shop or on anything big like a sofa, and in a chain with price tags it is still fine to ask. One question, then take the answer and either buy or leave: asking twice or arguing after a no is the part that gets awkward.",
        ),
    ),
    Case(
        "haggle-price",
        "feels awkward to haggle",
        &[],
        Some(
            "No - asking once is normal in a market, a car boot sale, a small independent shop or on anything big like a sofa, and in a chain with price tags it is still fine to ask. One question, then take the answer and either buy or leave: asking twice or arguing after a no is the part that gets awkward.",
        ),
    ),
    Case(
        "haggle-price",
        "should i negotiate with a roofer",
        &[],
        Some(
            "Get three written quotes first, then ask each trader the same question: what would bring that down? Supplying the materials, being flexible on dates so the job fills a gap in their diary, and paying a deposit up front are the three that work. Compare the quotes line by line rather than on the total, and treat one miles under the others as a warning: the extras come later.",
        ),
    ),
    Case(
        "haggle-price",
        "what can bring down a contractor's quote",
        &[],
        Some(
            "Get three written quotes first, then ask each trader the same question: what would bring that down? Supplying the materials, being flexible on dates so the job fills a gap in their diary, and paying a deposit up front are the three that work. Compare the quotes line by line rather than on the total, and treat one miles under the others as a warning: the extras come later.",
        ),
    ),
    Case(
        "haggle-price",
        "are builders open to negotiation",
        &[],
        Some(
            "Get three written quotes first, then ask each trader the same question: what would bring that down? Supplying the materials, being flexible on dates so the job fills a gap in their diary, and paying a deposit up front are the three that work. Compare the quotes line by line rather than on the total, and treat one miles under the others as a warning: the extras come later.",
        ),
    ),
    Case(
        "haggle-price",
        "how should i haggle for furniture",
        &[],
        Some(
            "Ask once, cheerfully, once you have picked the thing - \"is there anything you can do on the price?\" - then stop talking and be ready to walk away: silence and a customer who will leave are what move a price. Take the display model or last season's colour, offer to pay in full today, check the online price so they can match it, and never say your maximum. At a market or car boot sale, offer about two thirds and settle in the middle.",
        ),
    ),
    Case(
        "haggle-price",
        "haggle at a market",
        &[],
        Some(
            "Ask once, cheerfully, once you have picked the thing - \"is there anything you can do on the price?\" - then stop talking and be ready to walk away: silence and a customer who will leave are what move a price. Take the display model or last season's colour, offer to pay in full today, check the online price so they can match it, and never say your maximum. At a market or car boot sale, offer about two thirds and settle in the middle.",
        ),
    ),
    Case(
        "haggle-price",
        "how can i get a better price on a motor",
        &[],
        Some(
            "Ask once, cheerfully, once you have picked the thing - \"is there anything you can do on the price?\" - then stop talking and be ready to walk away: silence and a customer who will leave are what move a price. Take the display model or last season's colour, offer to pay in full today, check the online price so they can match it, and never say your maximum. At a market or car boot sale, offer about two thirds and settle in the middle.",
        ),
    ),
    Case(
        "haggle-price",
        "can i haggle without feeling embarrassed",
        &[],
        Some(
            "No - asking once is normal in a market, a car boot sale, a small independent shop or on anything big like a sofa, and in a chain with price tags it is still fine to ask. One question, then take the answer and either buy or leave: asking twice or arguing after a no is the part that gets awkward.",
        ),
    ),
    Case(
        "haggle-price",
        "how should i ask for a cheaper labour rate",
        &[],
        Some(
            "Get three written quotes first, then ask each trader the same question: what would bring that down? Supplying the materials, being flexible on dates so the job fills a gap in their diary, and paying a deposit up front are the three that work. Compare the quotes line by line rather than on the total, and treat one miles under the others as a warning: the extras come later.",
        ),
    ),
    Case(
        "gift-dad",
        "what should i get him",
        &[("he is teetotal", "Okay.")],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "gift-mum",
        "what should i get her",
        &[("my mum just stopped drinking", "Okay.")],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "gift-mum",
        "a gift for grandma",
        &[("my father just stopped drinking", "Okay.")],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything.",
        ),
    ),
    Case(
        "gift-mum",
        "a gift for her",
        &[
            ("my father is diabetic", "Okay."),
            ("i saw nobody", "Okay."),
            ("it rained", "Okay."),
        ],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything.",
        ),
    ),
    Case(
        "gift-dad",
        "what can i get him",
        &[
            ("he is diabetic", "Okay."),
            ("he is also a vegetarian", "Okay."),
        ],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. And buy into the vegetarian side of it: a good vegetarian cookbook, a voucher for a veggie restaurant, or a decent olive oil and a spice set rather than a hamper of pate.",
        ),
    ),
    Case(
        "gift-mum",
        "what can i get her",
        &[("she is teetotal", "Okay."), ("had a good day", "Okay.")],
        Some(
            "Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything. Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out.",
        ),
    ),
    Case(
        "gift-dad",
        "present for a dad who is vegan",
        &[("he is sober", "Okay.")],
        Some(
            "Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones. And buy into the vegetarian side of it: a good vegetarian cookbook, a voucher for a veggie restaurant, or a decent olive oil and a spice set rather than a hamper of pate.",
        ),
    ),
    Case(
        "coat-worth",
        "what should i look for",
        &[(
            "i need a new coat",
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders.",
        )],
        Some(
            "Check the hood stays up in wind, the zip works with gloves on, the seams are taped and there is room for a jumper underneath. Look at the lining where it rubs under the arm and at the stitching on the pockets: those are the two places a cheap coat shows itself first.",
        ),
    ),
    Case(
        "coat-worth",
        "how should it fit",
        &[(
            "i need a new coat",
            "For rain, a waterproof shell with taped seams",
        )],
        Some(
            "Check the hood stays up in wind, the zip works with gloves on, the seams are taped and there is room for a jumper underneath. Look at the lining where it rubs under the arm and at the stitching on the pockets: those are the two places a cheap coat shows itself first.",
        ),
    ),
    Case(
        "coat-worth",
        "how should it fit",
        &[("i need a new coat", "A wool coat is warm")],
        Some(
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders. Check the hood stays up in wind, the zip works with gloves on, the seams are taped and there is room for a jumper underneath. Look at the lining where it rubs under the arm and at the stitching on the pockets: those are the two places a cheap coat shows itself first.",
        ),
    ),
    Case(
        "coat-worth",
        "what should i look for",
        &[
            (
                "i need a new coat",
                "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders.",
            ),
            ("it's cold", "Yes."),
        ],
        Some(
            "Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders. Check the hood stays up in wind, the zip works with gloves on, the seams are taped and there is room for a jumper underneath. Look at the lining where it rubs under the arm and at the stitching on the pockets: those are the two places a cheap coat shows itself first.",
        ),
    ),
];
