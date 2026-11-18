//! Direct decisions for PyBot's vehicle reply and guard callbacks.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub Option<bool>,
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "car-damp",
        "my windows have condensation inside",
        &[],
        Some(false),
        None,
    ),
    Case(
        "car-damp",
        "my car windows have condensation inside",
        &[],
        Some(true),
        None,
    ),
    Case(
        "car-damp",
        "my living room wall is wet inside",
        &[],
        Some(false),
        None,
    ),
    Case(
        "car-damp",
        "the foot well is damp inside the van",
        &[],
        Some(true),
        None,
    ),
    Case(
        "car-damp",
        "my windows are wet in the morning",
        &[],
        Some(false),
        None,
    ),
    Case(
        "car-damp",
        "my windows, ceiling and floor are damp",
        &[],
        Some(false),
        None,
    ),
    Case(
        "car-damp",
        "condensation on the inside of my windows",
        &[],
        Some(false),
        None,
    ),
    Case(
        "car-damp",
        "the windows, are wet inside",
        &[],
        Some(true),
        None,
    ),
    Case(
        "car-damp",
        "my car window is wet inside",
        &[],
        Some(true),
        None,
    ),
    Case(
        "car-damp",
        "why do my windows mist up inside",
        &[],
        Some(false),
        None,
    ),
    Case(
        "car-damp",
        "there is water in my Mondeo",
        &[],
        Some(true),
        None,
    ),
    Case(
        "car-damp",
        "my Focus has condensation on the rear window",
        &[],
        Some(true),
        None,
    ),
    Case("car-damp", "the boot smells musty", &[], Some(true), None),
    Case(
        "car-damp",
        "does a cupboard need ventilation",
        &[],
        Some(false),
        None,
    ),
    Case("car-damp", "the damp in my Prius", &[], Some(true), None),
    Case(
        "tyre-price",
        "what should i pay for a car tyre",
        &[],
        None,
        Some(
            "A budget tyre fitted is about £55 to £80 a corner, mid-range £75 to £120 and premium £120 to £200, with fitting, balancing and disposal £15 to £25 of that. Read the size off the sidewall - 205/55 R16 - and ring two or three fitters with it: the same tyre can differ by £40. Two at a time is fine, new pair on the back, and change them at 3mm of tread.",
        ),
    ),
    Case(
        "tyre-price",
        "how much are tyres fitted",
        &[],
        None,
        Some(
            "A budget tyre fitted is about £55 to £80 a corner, mid-range £75 to £120 and premium £120 to £200, with fitting, balancing and disposal £15 to £25 of that. Read the size off the sidewall - 205/55 R16 - and ring two or three fitters with it: the same tyre can differ by £40. Two at a time is fine, new pair on the back, and change them at 3mm of tread.",
        ),
    ),
    Case(
        "tyre-price",
        "price for two 205/55 R16 tyres",
        &[],
        None,
        Some(
            "A budget tyre fitted is about £55 to £80 a corner, mid-range £75 to £120 and premium £120 to £200, with fitting, balancing and disposal £15 to £25 of that. Read the size off the sidewall - 205/55 R16 - and ring two or three fitters with it: the same tyre can differ by £40. Two at a time is fine, new pair on the back, and change them at 3mm of tread.",
        ),
    ),
    Case(
        "tyre-price",
        "i need a new bike tyre",
        &[],
        None,
        Some(
            "A budget tyre fitted is about £55 to £80 a corner, mid-range £75 to £120 and premium £120 to £200, with fitting, balancing and disposal £15 to £25 of that. Read the size off the sidewall - 205/55 R16 - and ring two or three fitters with it: the same tyre can differ by £40. Two at a time is fine, new pair on the back, and change them at 3mm of tread.",
        ),
    ),
    Case(
        "tyre-price",
        "do i need to pay for two tyres",
        &[],
        None,
        Some(
            "A budget tyre fitted is about £55 to £80 a corner, mid-range £75 to £120 and premium £120 to £200, with fitting, balancing and disposal £15 to £25 of that. Read the size off the sidewall - 205/55 R16 - and ring two or three fitters with it: the same tyre can differ by £40. Two at a time is fine, new pair on the back, and change them at 3mm of tread.",
        ),
    ),
    Case(
        "tyre-price",
        "how much does a tyre cost",
        &[],
        None,
        Some(
            "A budget tyre fitted is about £55 to £80 a corner, mid-range £75 to £120 and premium £120 to £200, with fitting, balancing and disposal £15 to £25 of that. Read the size off the sidewall - 205/55 R16 - and ring two or three fitters with it: the same tyre can differ by £40. Two at a time is fine, new pair on the back, and change them at 3mm of tread.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "should i scrap my car",
        &[],
        None,
        Some(
            "Money against value: if the quote is more than the car is worth, or more than half of it in one go on an old car, let it go. A 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are. Scrap value is only £150 to £400.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "my car needs 900 pounds of work should i sell it",
        &[],
        None,
        Some(
            "£900 of work is the number to judge it by: if the car would sell for less than about £1,800, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "cost me 2000 and the garage wants 900",
        &[],
        None,
        Some(
            "Money against value: if the quote is more than the car is worth, or more than half of it in one go on an old car, let it go. A 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are. Scrap value is only £150 to £400.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "the car needs £1,300 for the suspension",
        &[],
        None,
        Some(
            "£1,300 of work is the number to judge it by: if the car would sell for less than about £2,600, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "is £150 of work worth it",
        &[],
        None,
        Some(
            "£150 of work is the number to judge it by: if the car would sell for less than about £300, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "the quote is 120",
        &[],
        None,
        Some(
            "Money against value: if the quote is more than the car is worth, or more than half of it in one go on an old car, let it go. A 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are. Scrap value is only £150 to £400.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "scrap value 149.9",
        &[],
        None,
        Some(
            "Money against value: if the quote is more than the car is worth, or more than half of it in one go on an old car, let it go. A 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are. Scrap value is only £150 to £400.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "my car needs 1,800.75 pounds of work",
        &[],
        None,
        Some(
            "£1,800 of work is the number to judge it by: if the car would sell for less than about £3,600, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "repair it for 2500 or scrap it",
        &[],
        None,
        Some(
            "£2,500 of work is the number to judge it by: if the car would sell for less than about £5,000, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "would it be worth spending £899.50 on my old vehicle",
        &[],
        None,
        Some(
            "£899 of work is the number to judge it by: if the car would sell for less than about £1,798, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "the quote is over 4000.90",
        &[],
        None,
        Some(
            "£4,000 of work is the number to judge it by: if the car would sell for less than about £8,000, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "quote 150 and some spare parts",
        &[],
        None,
        Some(
            "£150 of work is the number to judge it by: if the car would sell for less than about £300, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "vehicle worth £1200 and repair costs £300",
        &[],
        None,
        Some(
            "Money against value: if the quote is more than the car is worth, or more than half of it in one go on an old car, let it go. A 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are. Scrap value is only £150 to £400.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "is it worth a repair at £1,030.20",
        &[],
        None,
        Some(
            "£1,030 of work is the number to judge it by: if the car would sell for less than about £2,060, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "the cost of the repair is 1500 not 800",
        &[],
        None,
        Some(
            "Money against value: if the quote is more than the car is worth, or more than half of it in one go on an old car, let it go. A 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are. Scrap value is only £150 to £400.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "is the 99999 quote worth it",
        &[],
        None,
        Some(
            "£99,999 of work is the number to judge it by: if the car would sell for less than about £199,998, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "the garage's repair estimate is 99999999999999999999999999999999999",
        &[],
        None,
        Some(
            "£99,999,999,999,999,996,863,366,107,917,975,552 of work is the number to judge it by: if the car would sell for less than about £199,999,999,999,999,993,726,732,215,835,951,104, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "i need to fix my 2013 fiesta",
        &[],
        None,
        Some(
            "£2,013 of work is the number to judge it by: if the car would sell for less than about £4,026, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are.",
        ),
    ),
    Case(
        "repair-or-scrap",
        "the work costs nine hundred pounds",
        &[],
        None,
        Some(
            "Money against value: if the quote is more than the car is worth, or more than half of it in one go on an old car, let it go. A 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are. Scrap value is only £150 to £400.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "my car is making a clicking noise when i turn",
        &[],
        None,
        Some(
            "A click or clunk that keeps time with the steering is usually a CV joint on the driveshaft: the rubber boot splits, the grease comes out and the joint wears. Look behind the front wheels for a torn boot and grease sprayed round the inside of the arch. A garage replaces the joint or the shaft, an hour or two of work, and it is worth doing before it fails - a joint that lets go leaves the car with no drive. A single clunk on full lock can also be a top mount or a drop link, which is cheaper.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "what is a cv joint",
        &[],
        None,
        Some(
            "A CV joint - constant velocity joint - is the flexible coupling at each end of a driveshaft, so the shaft can turn the wheel while the suspension moves and the wheel steers. It is packed with grease and wrapped in a rubber boot; when the boot splits the grease goes, grit gets in and it starts clicking on turns. Outer joints sit behind each front wheel, the inner ones at the gearbox.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "what does a drive shaft do",
        &[],
        None,
        Some(
            "A CV joint - constant velocity joint - is the flexible coupling at each end of a driveshaft, so the shaft can turn the wheel while the suspension moves and the wheel steers. It is packed with grease and wrapped in a rubber boot; when the boot splits the grease goes, grit gets in and it starts clicking on turns. Outer joints sit behind each front wheel, the inner ones at the gearbox.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "what is that clicking when i turn",
        &[],
        None,
        Some(
            "A click or clunk that keeps time with the steering is usually a CV joint on the driveshaft: the rubber boot splits, the grease comes out and the joint wears. Look behind the front wheels for a torn boot and grease sprayed round the inside of the arch. A garage replaces the joint or the shaft, an hour or two of work, and it is worth doing before it fails - a joint that lets go leaves the car with no drive. A single clunk on full lock can also be a top mount or a drop link, which is cheaper.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "what does a CV joint do?",
        &[],
        None,
        Some(
            "A CV joint - constant velocity joint - is the flexible coupling at each end of a driveshaft, so the shaft can turn the wheel while the suspension moves and the wheel steers. It is packed with grease and wrapped in a rubber boot; when the boot splits the grease goes, grit gets in and it starts clicking on turns. Outer joints sit behind each front wheel, the inner ones at the gearbox.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "whats a driveshaft",
        &[],
        None,
        Some(
            "A CV joint - constant velocity joint - is the flexible coupling at each end of a driveshaft, so the shaft can turn the wheel while the suspension moves and the wheel steers. It is packed with grease and wrapped in a rubber boot; when the boot splits the grease goes, grit gets in and it starts clicking on turns. Outer joints sit behind each front wheel, the inner ones at the gearbox.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "click when i turn the key",
        &[],
        None,
        Some(
            "A click or clunk that keeps time with the steering is usually a CV joint on the driveshaft: the rubber boot splits, the grease comes out and the joint wears. Look behind the front wheels for a torn boot and grease sprayed round the inside of the arch. A garage replaces the joint or the shaft, an hour or two of work, and it is worth doing before it fails - a joint that lets go leaves the car with no drive. A single clunk on full lock can also be a top mount or a drop link, which is cheaper.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "clunk when the wheel is on full lock",
        &[],
        None,
        Some(
            "A click or clunk that keeps time with the steering is usually a CV joint on the driveshaft: the rubber boot splits, the grease comes out and the joint wears. Look behind the front wheels for a torn boot and grease sprayed round the inside of the arch. A garage replaces the joint or the shaft, an hour or two of work, and it is worth doing before it fails - a joint that lets go leaves the car with no drive. A single clunk on full lock can also be a top mount or a drop link, which is cheaper.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "it clicks when turning left",
        &[],
        None,
        Some(
            "A click or clunk that keeps time with the steering is usually a CV joint on the driveshaft: the rubber boot splits, the grease comes out and the joint wears. Look behind the front wheels for a torn boot and grease sprayed round the inside of the arch. A garage replaces the joint or the shaft, an hour or two of work, and it is worth doing before it fails - a joint that lets go leaves the car with no drive. A single clunk on full lock can also be a top mount or a drop link, which is cheaper.",
        ),
    ),
    Case(
        "clicking-on-a-turn",
        "what is that CV joint",
        &[],
        None,
        Some(
            "A CV joint - constant velocity joint - is the flexible coupling at each end of a driveshaft, so the shaft can turn the wheel while the suspension moves and the wheel steers. It is packed with grease and wrapped in a rubber boot; when the boot splits the grease goes, grit gets in and it starts clicking on turns. Outer joints sit behind each front wheel, the inner ones at the gearbox.",
        ),
    ),
    Case(
        "clutch-biting-point",
        "how do i find the biting point",
        &[],
        None,
        Some(
            "A manual car goes because the clutch reaches the biting point before any gas: clutch down, first gear, then let the pedal up slowly until the bonnet lifts and the revs dip, hold it there, and add a little gas as you let it up the rest of the way. Practise finding that point five or ten times on a flat quiet road, because pulling away is only that point held while the car moves. Take your foot right off the pedal once you are moving - riding it is what wears a clutch out.",
        ),
    ),
    Case(
        "clutch-biting-point",
        "how do i stop stalling in a manual",
        &[],
        None,
        Some(
            "A manual car goes because the clutch reaches the biting point before any gas: clutch down, first gear, then let the pedal up slowly until the bonnet lifts and the revs dip, hold it there, and add a little gas as you let it up the rest of the way. Practise finding that point five or ten times on a flat quiet road, because pulling away is only that point held while the car moves. Take your foot right off the pedal once you are moving - riding it is what wears a clutch out.",
        ),
    ),
    Case(
        "clutch-biting-point",
        "how do i pull away on ice",
        &[],
        None,
        None,
    ),
    Case(
        "clutch-biting-point",
        "the gearbox is stuck in first gear",
        &[],
        None,
        None,
    ),
    Case(
        "clutch-biting-point",
        "how do i know when to release the clutch?",
        &[],
        None,
        Some(
            "A manual car goes because the clutch reaches the biting point before any gas: clutch down, first gear, then let the pedal up slowly until the bonnet lifts and the revs dip, hold it there, and add a little gas as you let it up the rest of the way. Practise finding that point five or ten times on a flat quiet road, because pulling away is only that point held while the car moves. Take your foot right off the pedal once you are moving - riding it is what wears a clutch out.",
        ),
    ),
    Case(
        "clutch-biting-point",
        "how do i pull away smoothly in a manual car",
        &[],
        None,
        Some(
            "A manual car goes because the clutch reaches the biting point before any gas: clutch down, first gear, then let the pedal up slowly until the bonnet lifts and the revs dip, hold it there, and add a little gas as you let it up the rest of the way. Practise finding that point five or ten times on a flat quiet road, because pulling away is only that point held while the car moves. Take your foot right off the pedal once you are moving - riding it is what wears a clutch out.",
        ),
    ),
    Case("clutch-biting-point", "my clutch smells", &[], None, None),
    Case(
        "change-gear-smoothly",
        "how do i change gear smoothly",
        &[],
        None,
        Some(
            "Change up at about 2,000 to 2,500 revs in a petrol car, or 1,800 to 2,000 in a diesel, and let the clutch up to the biting point before you add gas. The lurch is a mismatch between engine speed and road speed, so on a downward change give the revs a small blip as the clutch comes up. Guide the lever gently through the gate instead of yanking it: the gearbox only has one way in.",
        ),
    ),
    Case(
        "change-gear-smoothly",
        "clutch jerks when i drive",
        &[],
        None,
        Some(
            "Change up at about 2,000 to 2,500 revs in a petrol car, or 1,800 to 2,000 in a diesel, and let the clutch up to the biting point before you add gas. The lurch is a mismatch between engine speed and road speed, so on a downward change give the revs a small blip as the clutch comes up. Guide the lever gently through the gate instead of yanking it: the gearbox only has one way in.",
        ),
    ),
    Case(
        "change-gear-smoothly",
        "jumpy when changing up",
        &[],
        None,
        Some(
            "Change up at about 2,000 to 2,500 revs in a petrol car, or 1,800 to 2,000 in a diesel, and let the clutch up to the biting point before you add gas. The lurch is a mismatch between engine speed and road speed, so on a downward change give the revs a small blip as the clutch comes up. Guide the lever gently through the gate instead of yanking it: the gearbox only has one way in.",
        ),
    ),
    Case(
        "change-gear-smoothly",
        "why is my car grinding when i shift",
        &[],
        None,
        None,
    ),
    Case(
        "change-gear-smoothly",
        "how do i downshift smoothly",
        &[],
        None,
        Some(
            "Change up at about 2,000 to 2,500 revs in a petrol car, or 1,800 to 2,000 in a diesel, and let the clutch up to the biting point before you add gas. The lurch is a mismatch between engine speed and road speed, so on a downward change give the revs a small blip as the clutch comes up. Guide the lever gently through the gate instead of yanking it: the gearbox only has one way in.",
        ),
    ),
    Case(
        "change-gear-smoothly",
        "it is stuck in gear",
        &[],
        None,
        None,
    ),
    Case(
        "change-gear-smoothly",
        "how do i find second gear after a collision",
        &[],
        None,
        None,
    ),
    Case(
        "change-gear-smoothly",
        "could the price of grinding matter",
        &[],
        None,
        None,
    ),
    Case(
        "three-point-turn",
        "how do i do a three point turn",
        &[],
        None,
        Some(
            "Left lock across to the far kerb, reverse back on right lock, then forward on left lock again - three moves and you are round, which is why it is usually called a turn in the road now. Every move starts with a mirror and a shoulder check, and the kerb is a limit rather than a target. Keep it slow on the clutch: speed is what makes a turn in the road go wrong.",
        ),
    ),
    Case(
        "three-point-turn",
        "how do you steer on a 3 point turn",
        &[],
        None,
        Some(
            "Left lock across to the far kerb, reverse back on right lock, then forward on left lock again - three moves and you are round, which is why it is usually called a turn in the road now. Every move starts with a mirror and a shoulder check, and the kerb is a limit rather than a target. Keep it slow on the clutch: speed is what makes a turn in the road go wrong.",
        ),
    ),
    Case(
        "three-point-turn",
        "whats the sequence for a three-point turn",
        &[],
        None,
        Some(
            "Left lock across to the far kerb, reverse back on right lock, then forward on left lock again - three moves and you are round, which is why it is usually called a turn in the road now. Every move starts with a mirror and a shoulder check, and the kerb is a limit rather than a target. Keep it slow on the clutch: speed is what makes a turn in the road go wrong.",
        ),
    ),
    Case(
        "three-point-turn",
        "how do i do a three point turn on ice",
        &[],
        None,
        None,
    ),
    Case(
        "three-point-turn",
        "i hear grinding on a three point turn",
        &[],
        None,
        None,
    ),
    Case(
        "three-point-turn",
        "is it hard to turn around on a wet road",
        &[],
        None,
        None,
    ),
    Case(
        "driving-in-the-dark",
        "what should i do driving at night",
        &[],
        None,
        Some(
            "At night on an unlit road with nothing in front, use full beam; dip the moment a car appears, because a main beam left on dazzles the driver coming towards you. Drive so you could stop within the distance your lights actually show, which is usually slower than in daylight.",
        ),
    ),
    Case(
        "driving-in-the-dark",
        "how to use full beam in the dark",
        &[],
        None,
        Some(
            "At night on an unlit road with nothing in front, use full beam; dip the moment a car appears, because a main beam left on dazzles the driver coming towards you. Drive so you could stop within the distance your lights actually show, which is usually slower than in daylight.",
        ),
    ),
    Case(
        "driving-in-the-dark",
        "is it safe to drive after dark on a winding road",
        &[],
        None,
        Some(
            "At night on an unlit road with nothing in front, use full beam; dip the moment a car appears, because a main beam left on dazzles the driver coming towards you. Drive so you could stop within the distance your lights actually show, which is usually slower than in daylight.",
        ),
    ),
    Case(
        "driving-in-the-dark",
        "should i drive at night on ice",
        &[],
        None,
        None,
    ),
    Case(
        "driving-in-the-dark",
        "night driving with a warning light",
        &[],
        None,
        None,
    ),
    Case(
        "driving-in-the-dark",
        "when am i supposed to dip headlights",
        &[],
        None,
        Some(
            "At night on an unlit road with nothing in front, use full beam; dip the moment a car appears, because a main beam left on dazzles the driver coming towards you. Drive so you could stop within the distance your lights actually show, which is usually slower than in daylight.",
        ),
    ),
    Case(
        "driving-in-the-dark",
        "driving in the dark in snow",
        &[],
        None,
        None,
    ),
    Case(
        "driving-in-the-dark",
        "if it's dark and no other cars",
        &[],
        None,
        Some(
            "At night on an unlit road with nothing in front, use full beam; dip the moment a car appears, because a main beam left on dazzles the driver coming towards you. Drive so you could stop within the distance your lights actually show, which is usually slower than in daylight.",
        ),
    ),
    Case(
        "driving-in-the-dark",
        "full beam lights keep cutting out",
        &[],
        None,
        None,
    ),
    Case(
        "tyre-price",
        "how much do tyres cost",
        &["buying a secondhand bike", "seller says it needs new tyres"],
        None,
        None,
    ),
    Case(
        "tyre-price",
        "how much do tyres cost",
        &["the spare inner tube has a hole"],
        None,
        None,
    ),
    Case(
        "tyre-price",
        "how much do tyres cost",
        &[
            "the weather is good for cycling",
            "okay",
            "and the price",
            "tesco",
            "i need tyres",
        ],
        None,
        Some(
            "A budget tyre fitted is about £55 to £80 a corner, mid-range £75 to £120 and premium £120 to £200, with fitting, balancing and disposal £15 to £25 of that. Read the size off the sidewall - 205/55 R16 - and ring two or three fitters with it: the same tyre can differ by £40. Two at a time is fine, new pair on the back, and change them at 3mm of tread.",
        ),
    ),
    Case(
        "tyre-price",
        "my car needs tyres how much",
        &["my bicycle punctured", "now my car needs service"],
        None,
        None,
    ),
    Case(
        "tyre-price",
        "how much does a bike tyre cost",
        &["my car is up for service"],
        None,
        Some(
            "A budget tyre fitted is about £55 to £80 a corner, mid-range £75 to £120 and premium £120 to £200, with fitting, balancing and disposal £15 to £25 of that. Read the size off the sidewall - 205/55 R16 - and ring two or three fitters with it: the same tyre can differ by £40. Two at a time is fine, new pair on the back, and change them at 3mm of tread.",
        ),
    ),
    Case(
        "tyre-price",
        "how much are tyres fitted",
        &["what is a chainring", "bike lessons"],
        None,
        None,
    ),
];
