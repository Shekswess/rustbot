//! Pinned guards and replies unlocked by ordered text-field joins.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub bool,
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "child-coat",
        "what winter coat should i buy for my daughter",
        true,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "what kind of coat for my boy",
        true,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "does my child need a big coat",
        true,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "what coat size should i buy for an 8 year old",
        true,
        Some(
            "Buy the size that fits their height now rather than one to grow into: an oversized coat leaves a gap at the neck and wrists where the warmth goes, and the sleeves hang over their hands. Check there is room for a jumper underneath, that the hood stays up in wind, and that the cuffs and the hem can be adjusted.",
        ),
    ),
    Case(
        "child-coat",
        "what coat should a toddler wear in a car seat",
        true,
        Some(
            "Never put a thick coat under a car-seat harness: the straps cannot tighten on it, and in a crash a child can slip out of a loose seat. Strap them in a thin fleece or jumper and put the coat on once they are out, or lay it over them, and warm the car before you set off. In a pram a pramsuit or a coat over a tucked-in blanket is warmer than a stiff coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "is a thick coat okay in a car seat",
        true,
        Some(
            "Never put a thick coat under a car-seat harness: the straps cannot tighten on it, and in a crash a child can slip out of a loose seat. Strap them in a thin fleece or jumper and put the coat on once they are out, or lay it over them, and warm the car before you set off. In a pram a pramsuit or a coat over a tucked-in blanket is warmer than a stiff coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "what coat is best for a baby in a pushchair",
        true,
        Some(
            "Never put a thick coat under a car-seat harness: the straps cannot tighten on it, and in a crash a child can slip out of a loose seat. Strap them in a thin fleece or jumper and put the coat on once they are out, or lay it over them, and warm the car before you set off. In a pram a pramsuit or a coat over a tucked-in blanket is warmer than a stiff coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "can i use a coat in a buggy",
        true,
        Some(
            "Never put a thick coat under a car-seat harness: the straps cannot tighten on it, and in a crash a child can slip out of a loose seat. Strap them in a thin fleece or jumper and put the coat on once they are out, or lay it over them, and warm the car before you set off. In a pram a pramsuit or a coat over a tucked-in blanket is warmer than a stiff coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "kids coats for buying",
        true,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "my child refuses to wear a coat",
        false,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "my son hates wearing his school jacket",
        false,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "why does he fight getting dressed",
        false,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "child will not put his coat on",
        false,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "my toddler wore a raincoat all day",
        true,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "can i get a warm coat with a hood",
        true,
        Some(
            "For a child, look for a hood that stays up, a waterproof outer, room for a jumper underneath and no cords round the neck; a size that fits now beats one to grow into. A baby in a pram is warmer in a pramsuit than in a coat they have to be folded into.",
        ),
    ),
    Case(
        "child-coat",
        "is a puffer jacket too heavy in a stroller",
        true,
        Some(
            "Never put a thick coat under a car-seat harness: the straps cannot tighten on it, and in a crash a child can slip out of a loose seat. Strap them in a thin fleece or jumper and put the coat on once they are out, or lay it over them, and warm the car before you set off. In a pram a pramsuit or a coat over a tucked-in blanket is warmer than a stiff coat they have to be folded into.",
        ),
    ),
    Case(
        "split-bill",
        "how should we split the bill at dinner",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case(
        "split-bill",
        "we all ate different amounts, should i split the bill",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case(
        "split-bill",
        "How do I split a wine bill between friends?",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case(
        "split-bill",
        "should my rich partner pay more at a restaurant",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case(
        "split-bill",
        "what is fair to split after dinner",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case("split-bill", "one earns more than me", true, None),
    Case("split-bill", "we earn differently", true, None),
    Case("split-bill", "joint income split", true, None),
    Case(
        "split-bill",
        "can we split electricity costs",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case(
        "split-bill",
        "we moved in together and my partner earns way more",
        true,
        None,
    ),
    Case(
        "split-bill",
        "how do we split the household bills",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case(
        "split-bill",
        "my partner earns more but we want to share the meal bill",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case(
        "split-bill",
        "who's paying for dinner",
        true,
        Some(
            "Split by what people actually had when the gap is big, drinks usually being the culprit; even shares are fine and simpler when it isn't. Say what you'd like before the bill arrives, which is far less awkward than sorting it out at the table.",
        ),
    ),
    Case(
        "share-a-cost",
        "how do i split fuel costs with friends",
        true,
        Some(
            "Split fuel by the miles each person actually drove, not by who was in the car: note the odometer when you set off and when you get back, keep the receipt for the tank, and divide it by the miles rather than by the passengers. Agree before you set off whether the owner's wear is part of it - tyres and servicing are already in the driver's own costs, and a passenger is paying for the journey. One person pays at the pump, the rest send their share the same day, because a tank remembered a fortnight later is always argued about.",
        ),
    ),
    Case(
        "share-a-cost",
        "how to share the petrol cost for the ride",
        true,
        Some(
            "Split fuel by the miles each person actually drove, not by who was in the car: note the odometer when you set off and when you get back, keep the receipt for the tank, and divide it by the miles rather than by the passengers. Agree before you set off whether the owner's wear is part of it - tyres and servicing are already in the driver's own costs, and a passenger is paying for the journey. One person pays at the pump, the rest send their share the same day, because a tank remembered a fortnight later is always argued about.",
        ),
    ),
    Case(
        "share-a-cost",
        "can we split the cost of the taxi",
        true,
        Some(
            "Split a fare by the journey each person actually made, not by head: whoever was picked up first and dropped last pays most, and a taxi app already shows the breakdown, so screenshot it into the group chat the same night. Say who is paying the driver before anyone gets in, and let one person book and collect, so there is a single number to divide rather than three arguments about it.",
        ),
    ),
    Case(
        "share-a-cost",
        "how do we share the fare",
        true,
        Some(
            "Split a fare by the journey each person actually made, not by head: whoever was picked up first and dropped last pays most, and a taxi app already shows the breakdown, so screenshot it into the group chat the same night. Say who is paying the driver before anyone gets in, and let one person book and collect, so there is a single number to divide rather than three arguments about it.",
        ),
    ),
    Case(
        "share-a-cost",
        "how to split the cost of a house on holiday",
        true,
        Some(
            "Book the big things - the flights, the house, the car - to one card and split those equally, or by room if the rooms differ, then have everyone put the same amount into a kitty for meals, taxis and tickets. Photograph every receipt and count the kitty on the last night rather than after you get home, and refund what is left to the person who paid it in, because a group holiday is a lot of small shared costs rather than one bill.",
        ),
    ),
    Case(
        "share-a-cost",
        "how would you split an airbnb",
        true,
        Some(
            "Split a shared place by the room and the nights, not by head: a couple in the double pays more than the person on the sofa bed, and anyone staying three nights out of seven pays for three. Work the total out per room per night, round it to something easy, and take the deposits from everyone before the booking is made so the person whose card it is on is never the one chasing.",
        ),
    ),
    Case(
        "share-a-cost",
        "how do we split our holiday costs",
        true,
        Some(
            "Book the big things - the flights, the house, the car - to one card and split those equally, or by room if the rooms differ, then have everyone put the same amount into a kitty for meals, taxis and tickets. Photograph every receipt and count the kitty on the last night rather than after you get home, and refund what is left to the person who paid it in, because a group holiday is a lot of small shared costs rather than one bill.",
        ),
    ),
    Case(
        "share-a-cost",
        "how should we split the group trip budget",
        true,
        Some(
            "Book the big things - the flights, the house, the car - to one card and split those equally, or by room if the rooms differ, then have everyone put the same amount into a kitty for meals, taxis and tickets. Photograph every receipt and count the kitty on the last night rather than after you get home, and refund what is left to the person who paid it in, because a group holiday is a lot of small shared costs rather than one bill.",
        ),
    ),
    Case(
        "share-a-cost",
        "can we chip in for a leaving gift",
        true,
        Some(
            "Fix the amount before anybody is asked, and ask for it once: a whip round works when the target is a number - 'twenty pounds each, I'll get the vouchers' - and fails when it is a collection with no total. Buy from one person's card, keep the receipt and the list of who has paid in the group chat, and cover any shortfall yourself rather than going round a second time.",
        ),
    ),
    Case(
        "share-a-cost",
        "lets go halves on a retirement present",
        true,
        Some(
            "Fix the amount before anybody is asked, and ask for it once: a whip round works when the target is a number - 'twenty pounds each, I'll get the vouchers' - and fails when it is a collection with no total. Buy from one person's card, keep the receipt and the list of who has paid in the group chat, and cover any shortfall yourself rather than going round a second time.",
        ),
    ),
    Case(
        "share-a-cost",
        "how do we share the cost of something",
        true,
        Some(
            "Say the number before the thing is booked or ordered, then split by what each person actually had rather than by head, and keep the reckoning in one place - a note in the group chat or a splitting app - so nobody has to trust their memory. Whoever pays up front collects, and it is fair to ask the same day rather than a fortnight later.",
        ),
    ),
    Case(
        "share-a-cost",
        "who pays the driver",
        true,
        Some(
            "Say the number before the thing is booked or ordered, then split by what each person actually had rather than by head, and keep the reckoning in one place - a note in the group chat or a splitting app - so nobody has to trust their memory. Whoever pays up front collects, and it is fair to ask the same day rather than a fortnight later.",
        ),
    ),
    Case(
        "share-a-cost",
        "are we splitting the money in the office",
        true,
        Some(
            "Say the number before the thing is booked or ordered, then split by what each person actually had rather than by head, and keep the reckoning in one place - a note in the group chat or a splitting app - so nobody has to trust their memory. Whoever pays up front collects, and it is fair to ask the same day rather than a fortnight later.",
        ),
    ),
    Case(
        "share-a-cost",
        "funds for a joint birthday present",
        false,
        Some(
            "Fix the amount before anybody is asked, and ask for it once: a whip round works when the target is a number - 'twenty pounds each, I'll get the vouchers' - and fails when it is a collection with no total. Buy from one person's card, keep the receipt and the list of who has paid in the group chat, and cover any shortfall yourself rather than going round a second time.",
        ),
    ),
    Case(
        "share-a-cost",
        "the company pays mileage",
        false,
        Some(
            "Split fuel by the miles each person actually drove, not by who was in the car: note the odometer when you set off and when you get back, keep the receipt for the tank, and divide it by the miles rather than by the passengers. Agree before you set off whether the owner's wear is part of it - tyres and servicing are already in the driver's own costs, and a passenger is paying for the journey. One person pays at the pump, the rest send their share the same day, because a tank remembered a fortnight later is always argued about.",
        ),
    ),
    Case(
        "share-a-cost",
        "is my rent deposit shared",
        false,
        Some(
            "Say the number before the thing is booked or ordered, then split by what each person actually had rather than by head, and keep the reckoning in one place - a note in the group chat or a splitting app - so nobody has to trust their memory. Whoever pays up front collects, and it is fair to ask the same day rather than a fortnight later.",
        ),
    ),
    Case(
        "share-a-cost",
        "share the bill equally",
        true,
        Some(
            "Say the number before the thing is booked or ordered, then split by what each person actually had rather than by head, and keep the reckoning in one place - a note in the group chat or a splitting app - so nobody has to trust their memory. Whoever pays up front collects, and it is fair to ask the same day rather than a fortnight later.",
        ),
    ),
    Case(
        "golf-swing",
        "how can i improve my golf swing",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "what should i do about my swing from the tee",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "how do i put it in the garden",
        false,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "put this golf club somewhere safe",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "can i put my golf clubs away",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "i want to fix my putting",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "why do my puts always go left",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "how do i putt better",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "what is a putting stroke",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "should i practise with a 7 iron",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "is golf swing tempo important",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "what's a golf swing",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "can i put a nail in the wall",
        false,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "put a shelf above the door",
        false,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "how do i put together a golf putting mat",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
    Case(
        "golf-swing",
        "how do i put in more time on a golf swing",
        true,
        Some(
            "For most club golfers it is not the swing but the aim and the tempo: check that feet, hips and shoulders all point where you want the ball to start, then swing at eight out of ten with a smooth one-two count. Practise half-swings with a 7 iron and brush the grass - the divot should come after the ball. Two or three lessons with a pro filming you will save a season of guessing.",
        ),
    ),
];
