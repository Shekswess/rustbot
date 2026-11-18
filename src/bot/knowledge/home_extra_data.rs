//! Household reply data from the pinned PyBot source.

pub(super) const HAIR_DRYER_TRIGGERS: &[&str] = &[
    "buzzes", "buzzing", "dead", "fuse", "nothing", "power", "start", "starts", "stop", "stopped",
    "switch", "switches", "turn", "turns", "won't", "wont", "wont.",
];
pub(super) const HAIR_DRYER_OTHER: &str = "Check the air intake first: a hair dryer that blows cold usually has the grille at the back furred up with hair and fluff, and the thermal cut-out trips before the element ever fails. Unplug it, pull the hair off the grille and out of the vent, let it cool for ten minutes and try it again on the high setting. If it still runs cold it is the element or the cut-out, and on a cheap dryer that is a new one rather than a repair - do not open the case, the element is a live part.";
pub(super) const HAIR_DRYER_TRIGGERED: &str = "A hair dryer that is completely dead is nearly always the plug or the flex, not the element: try another socket, check the fuse in the plug, and look at the point where the flex meets the handle, because a cable that is wound up every day is what breaks. If a fresh fuse blows again, stop using it - that is a short in the element, and a new dryer is cheaper than the repair.";
pub(super) const DRYER_HEAT_TRIGGERS: &[&str] = &[
    "again", "ages", "ages.", "cycle", "cycles", "damp", "damp.", "forever", "go", "goes", "hours",
    "long", "longer", "slow", "slower", "slowly", "slowly.", "twice", "wet", "wet.",
];
pub(super) const DRYER_HEAT_OTHER: &str = "Nearly always the air or the setting, not a part: check the dial first, since most machines finish on a cool-down phase that heats nothing, then empty the fluff filter and check the vent hose is not kinked. If the drum turns and the air still comes out cold, it is the thermal fuse on the element - an engineer's job, cheap part.";
pub(super) const DRYER_HEAT_TRIGGERED: &str = "Slow drying is nearly always the air: empty the fluff filter, rinse the condenser if it has one, and check the vent hose is not kinked. A drum stuffed full will not dry either, so try half a load. If the air still comes out cold, it is the fuse on the element - an engineer's job, cheap part.";
pub(super) const WALL_NAIL_0: &str = "Tap it in at a slight downward angle so the weight hangs on the wall instead of pulling the head out, and put it into something solid - a stud, found by tapping, or brick under the plaster. On plasterboard with nothing behind it a bare nail or a picture hook pulls out: use a plasterboard plug or a self-drilling fixing, and keep the load light.";
pub(super) const WALL_NAIL_1: &str = "Grip the head with pincers and slide a scrap of wood under them so the plaster does not crush, then ease it out with a slight twist rather than levering against the wall. A nail that turns but will not come has lost its grip: punch it below the surface with another nail, fill the dent with filler, sand it flat and touch up the paint.";
pub(super) const WOOD_NAIL_0: &str = "Drill a pilot hole first, about three quarters as wide as the nail, and keep it at least 25mm in from the end of the board, because that is where a split starts. Blunt the point by tapping it once on a hard surface so it cuts the grain instead of wedging it apart, and clamp a scrap of wood behind the far face when the board is thin. A split that has already opened wants wood glue worked into it and a clamp overnight, not another nail.";
pub(super) const WOOD_NAIL_1: &str = "Size the nail to the job, not to the room: 40mm to 50mm lost-head or oval nails for skirting and architrave, 50mm to 65mm for a floorboard, 30mm for a 6mm batten, and thin panel pins for beading and a back panel. As a rule the nail wants to be about two and a half times as long as the piece it is holding, and the top third of it has to bite into something solid behind.";
pub(super) const PAPER_0: &str = "Cheap paper is made from wood pulp, and the lignin in it turns acidic and brown as it sits - light, heat and damp speed it up, which is why the box in the loft goes first. So keep paper cool, dry, dark and steady: a cupboard inside the house, not the loft, shed or cellar; acid-free folders and boxes rather than polythene bags, which trap damp; no sellotape and no lamination, both of which stain and cannot be undone. Yellowing that has already started can be slowed, not reversed.";
pub(super) const PAPER_1: &str = "Musty paper is damp paper, and the smell is mould growing in it: stand the books or letters up with the pages fanned in a dry room for a day or two - not in sunlight, which yellows them faster - then keep them somewhere dry and steady. A sachet of silica gel in the box, a cupboard inside the house rather than the loft, garage or cellar, and a dehumidifier in a room that has had a leak will stop it coming back. Don't wipe them with bleach or a wet cloth: it takes the print with it.";
pub(super) const PAPER_2: &str = "Prints fade from light and from the plastic in old albums, so keep them out of sunlight, in acid-free sleeves or a proper album rather than a PVC one, and scan the ones that matter - a digital copy is the only version that cannot fade. A framed photo wants UV-filtering glass and a wall that never gets direct sun.";
pub(super) const A_NAIL_COMES_OUT: &[&str] = &[
    "remove", "removing", "removed", "out", "puller", "pincers", "pull", "pulling", "pulled",
    "old", "stuck", "left", "loose", "snapped", "broke", "broken", "half",
];
pub(super) const A_SIZE_ASK: &[&str] = &[
    "size",
    "sizes",
    "what size",
    "which size",
    "what length",
    "how long",
    "how big",
    "how thick",
    "which nail",
    "what nail",
    "what nails",
    "how many mm",
    "mm",
    "gauge",
    "best nail",
    "should i use",
];
pub(super) const A_SPLIT_ASK: &[&str] = &[
    "split",
    "splits",
    "splitting",
    "crack",
    "cracks",
    "cracking",
    "splinter",
    "splinters",
    "splintering",
    "snapped",
];
pub(super) const PAPER_PHOTOS: &[&str] = &[
    "photo",
    "photos",
    "photograph",
    "photographs",
    "print",
    "prints",
    "album",
    "albums",
    "negatives",
    "slides",
];
pub(super) const PAPER_SMELL: &[&str] = &[
    "musty", "fusty", "smell", "smells", "smelly", "damp", "mildew", "mouldy", "moldy", "mould",
    "mold", "foxing",
];
pub(super) const PROPERTY_SERVICE_WORDS: &[&str] = &[
    "service charge",
    "service charges",
    "maintenance charge",
    "maintenance charges",
    "management fee",
    "management fees",
    "estate charge",
    "estate charges",
];
pub(super) const PROPERTY_SERVICE_CONTEXT: &[&str] = &[
    "flat",
    "flats",
    "block",
    "blocks",
    "lease",
    "leasehold",
    "freeholder",
    "freehold",
    "building",
    "landlord",
    "agent",
    "estate",
    "residents",
    "maintenance",
    "ground rent",
    "property",
    "buying",
    "bought",
    "mortgage",
    "conveyancer",
    "solicitor",
];
pub(super) const PROPERTY_WORDS: &[(&[&str], &str)] = &[
    (
        &["ground rent", "ground rents"],
        "Ground rent is the yearly charge a leaseholder pays to the freeholder for the land the flat stands on - often £50 to £250, though some newer leases set it much higher. Read the lease for how it reviews: one that doubles every ten years, or rises with the retail prices index, is a real problem, because a mortgage lender will refuse it and a buyer will too. Ask the seller for the last three years of demands.",
    ),
    (
        &[
            "service charge",
            "service charges",
            "maintenance charge",
            "maintenance charges",
            "management fee",
            "management fees",
            "estate charge",
            "estate charges",
        ],
        "A service charge is your share of what the block costs to run: buildings insurance, the cleaner, the lift, the lights, the roof, and the managing agent's fee. It is not profit for the landlord - you are entitled to the yearly accounts and to challenge anything unreasonable at a tribunal - so ask for the last three years of it and what the reserve fund holds for big jobs. A charge that jumps in one year is usually a repair or an insurance renewal, and the accounts will say which.",
    ),
    (
        &[
            "managing agent",
            "managing agents",
            "management company",
            "managing company",
        ],
        "The managing agent is the company the freeholder pays to run the block day to day: repairs, insurance, the service charge accounts and the cleaning. It is not the freeholder and it cannot sell you the freehold, but it is who you ring about the broken door. Ask for the agent's name and the last set of accounts before you buy, and put a complaint about the block in writing to the agent first, because the freeholder is only reachable through it.",
    ),
    (
        &["freeholder", "freeholders"],
        "The freeholder owns the building and the land; a leaseholder owns the right to live in one flat for a set number of years. The freeholder's jobs are the structure and the outside, paid for by the service charge, and the leaseholders have the right to buy that freehold between them if enough of them want to.",
    ),
    (
        &["freehold"],
        "Freehold means you own the building and the land it stands on outright, with no end date and no ground rent or service charge to anyone. A house is usually freehold; a flat is usually leasehold, or the flat is leasehold and the freehold is owned by a company the flat owners share.",
    ),
    (
        &["leasehold"],
        "Leasehold means you own the right to live in the property for the number of years left on the lease, not the land under it. You pay ground rent and a service charge to the freeholder, and the lease is the contract that says what you may and may not do - which is why the years left matter so much when you sell.",
    ),
    (
        &[
            "covenant",
            "covenants",
            "deed of covenant",
            "deeds of covenant",
            "restrictive covenant",
            "restrictive covenants",
        ],
        "A covenant is a promise written into a deed. In a lease it is what you must and must not do (no subletting, keep the hallway clear); on a freehold it is usually a restriction on the land, such as not running a business from the house, and it can still be enforced fifty years on. A deed of covenant on its own is also the name for a signed promise to pay a fixed sum - often to a charity, so it can claim the tax back.",
    ),
    (
        &["easement", "easements"],
        "An easement is a right to use someone else's land for one purpose: a right of way across a drive, drains running under a neighbour's garden, or access to lay pipes. It stays with the property rather than the person, so it is on the title and a buyer's solicitor will find it. You cannot build over one, and it needs registering to be safe.",
    ),
    (
        &[
            "title deed",
            "title deeds",
            "title register",
            "land registry",
        ],
        "The title deeds are the documents that prove who owns a property. In England and Wales they are now held electronically by HM Land Registry, so there is nothing for you to keep in a safe - a copy of the title register can be downloaded for a few pounds. It lists the owner, the price last paid, the boundaries on a map and anything registered against the property, such as a mortgage or a covenant.",
    ),
    (
        &["conveyancer", "conveyancers", "conveyancing"],
        "A conveyancer is the solicitor or licensed conveyancer who does the legal work of buying or selling a home: searches, the contract, the lease or title, the mortgage conditions and the money on completion. Fees are usually £800 to £2,000 plus searches and VAT; ask for the total quoted for your property, and use one who is on your lender's panel if you have a mortgage.",
    ),
    (
        &[
            "what is a lease extension",
            "whats a lease extension",
            "lease extension mean",
            "lease extension meaning",
            "what does lease extension mean",
            "define lease extension",
        ],
        "A lease extension is a new, longer lease on the same flat: you pay the freeholder a premium and the years are added, 90 on top of what is left if you use the legal right, which you have once you have owned the flat for two years. Your solicitor serves the notice and the new lease is registered at the Land Registry; the premium is the negotiation. Do it before the lease drops under about 80 years, because the price climbs sharply after that.",
    ),
    (
        &[
            "lease extension",
            "extend my lease",
            "extend the lease",
            "extending my lease",
            "short lease",
            "lease length",
            "how long is the lease",
            "how long is my lease",
            "how long is a lease",
            "how long a lease",
            "how many years left on the lease",
            "how many years left on my lease",
            "years left on the lease",
            "years are left on the lease",
            "years are left on my lease",
        ],
        "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
    ),
    (
        &[
            "what is a lease",
            "whats a lease",
            "what is a leasehold",
            "leasehold mean",
            "leasehold meaning",
        ],
        "A lease is the document that gives you the right to live in a flat for a set number of years - 99, 125 and 999 are the usual lengths - and it comes with ground rent, a service charge and rules about the building. You own the inside of the flat and the right to live there; the freeholder owns the structure and the land. The two numbers to check are how many years are left and what the ground rent does.",
    ),
    (
        &[
            "epc",
            "energy performance certificate",
            "energy performance",
        ],
        "An EPC is the energy rating for the property, A to G, and it has to be done before a home is sold or rented, and it lasts ten years. E is the lowest rating a landlord may let a home on in England and Wales, so a flat rated F or G is either a bargaining point or a bill for insulation and a new boiler. Read the recommendation list on page three, because that is where the cheap wins are.",
    ),
    (
        &["gazumping", "gazumped", "gazumping mean"],
        "Gazumping is a seller accepting a higher offer after they had already accepted yours, which in England and Wales is legal until contracts are exchanged - nothing is binding before that, and you can lose the survey and legal fees you have paid. The defences are to ask for the property to be taken off the market in writing and to move fast towards exchange; in Scotland the offer is binding much earlier, so the risk is different there.",
    ),
    (
        &["snagging list", "snagging", "snags list"],
        "A snagging list is the list of small faults in a new build - a door that rubs, a crooked tile, a scratch on the worktop - which you hand to the developer to put right. Do it in the first days after you get the keys rather than at the end of the year, with a photo of each snag and where it is, and keep the emailed copy: the warranty usually gives you two years for this kind of thing, and the list is what starts the clock.",
    ),
    (
        &["peppercorn", "peppercorn rent"],
        "A peppercorn rent is a ground rent of nothing at all - the name is the old token payment, and it means the lease has no ground rent to pay. A lease extended the statutory way ends up on one, so a peppercorn ground rent in a lease is a good sign rather than a puzzle.",
    ),
];
type Definition = (
    &'static [&'static str],
    &'static str,
    &'static [&'static str],
);
pub(super) const HOME_WORDS: &[Definition] = &[
    (
        &["ground rent"],
        "Ground rent is the yearly charge a leaseholder pays the freeholder for the land the flat stands on, on top of the service charge: often £100 to £300, and now usually a fixed sum written into the lease. It is not the same as the service charge, which pays for the building, and a lease with ground rent that doubles every ten years is the one to argue about before you buy.",
        &[
            "my",
            "how do i",
            "should i",
            "increase",
            "doubled",
            "doubling",
            "demand",
            "arrears",
            "paying",
            "pay it",
            "not paying",
        ],
    ),
    (
        &["tenancy deposit", "deposit scheme", "deposit protection"],
        "A tenancy deposit is the money you pay a landlord at the start to cover damage or unpaid rent, usually five weeks' rent, and it is not the landlord's to keep: in England and Wales it must be put in a government-backed protection scheme within 30 days and you get a certificate saying where. If it was not protected, or your landlord keeps it for wear and tear, you can claim up to three times the deposit.",
        &[
            "not paid back",
            "not returned",
            "withheld",
            "kept my",
            "claim",
            "how do i",
            "should i",
            "my landlord",
            "how much",
            "returned",
        ],
    ),
    (
        &["reserve fund", "major works fund", "reserve funds"],
        "A reserve fund - a sinking fund, in a block of flats - is money the leaseholders pay into each month so the big jobs of the next twenty years, a new roof or lifts or external paint, are already paid for when they come. It is separate from the service charge for this year's running costs, and the balance shows in the annual accounts, so check it before you buy a leasehold flat.",
        &[
            "my",
            "how do i",
            "should i",
            "start one",
            "set one up",
            "for a car",
            "house deposit",
            "savings",
        ],
    ),
    (
        &["break clause", "breakclause"],
        "A break clause is a term in a tenancy or a commercial lease that lets either side end it early, on a date and with notice written into the contract: you still pay the rent until that date, and the notice is often two months. Ask exactly how the notice has to be given, in writing, and diary the day two months before the break date.",
        &[
            "grammar",
            "sentence",
            "verb",
            "clause in",
            "relative clause",
            "subordinate",
            "my",
            "how do i",
            "should i",
            "can i leave",
            "exercise it",
        ],
    ),
    (
        &["inventory", "check in report", "check-in report"],
        "An inventory is the list of what was in the flat and what condition it was in when you moved in, with photos, and it is the document the deposit is judged against at the end. Check it against the flat on the day, write down anything missing or already damaged, date and sign it, and photograph any marks before you send it back.",
        &[
            "stock",
            "shop",
            "business",
            "warehouse",
            "my",
            "how do i",
            "should i",
            "do one",
        ],
    ),
    (
        &["contents insurance", "contents cover"],
        "Contents insurance covers the things in the home: furniture, clothes, a laptop, a bike if it is listed, the damage a leak or a fire does to them - where buildings insurance covers the structure itself, which is the landlord's job in a rented flat. Add up what it would cost to replace everything new, keep receipts for anything over a few hundred pounds, and check the single-item limit before you assume the bike is covered.",
        &[
            "my",
            "how do i",
            "should i",
            "claim",
            "not paying",
            "renew",
            "cancel",
            "cheapest",
            "compare",
            "do i need",
        ],
    ),
];
pub(super) const CHIP_VARIANTS: &[(&str, &str)] = &[
    (
        r#"\blaminate\b|\bveneer\b|\bformica\b|\bmelamine\b|\bchipboard\b|\bikea\b"#,
        "Seal a laminate chip rather than fill it: a repair wax or a touch-up kit in the same colour goes in with a fingertip and a clear top coat stops water reaching the chipboard core, which is the part that swells and lifts the surface. If the edge of the chip has lifted, press it down with a warm iron through a cloth first. A chip along a front edge that keeps catching wants a new worktop strip or a whole top, because a filler there takes the knocks for a few months only.",
    ),
    (
        r#"\bstone\b|\bgranite\b|\bquartz\b|\bmarble\b|\bcomposite\b|\bcorian\b|\bsilestone\b|solid surface"#,
        "A stone worktop chip is a two-part epoxy job, tinted to the stone: clean the chip out, build the epoxy slightly proud, let it cure hard and then flat it back with a fine diamond pad and polish. Granite and quartz are a fitter's call if the chip is bigger than a fingertip or on a joint, and most fitters will do a small one for a call-out fee. Never use a filler that shrinks or a heat gun near the stone, because a thermal shock cracks it.",
    ),
    (
        r#"\bworktop\b|\bworktops\b|\bcounter\b|\bcountertop\b|counter top|breakfast bar"#,
        "Find out what the top is made of before you buy anything, because the three repairs do not swap. A laminate or veneered top takes a repair wax in the matching colour and a clear seal over it, so water cannot reach the chipboard core and swell it; solid wood is sanded back and filled with a wax or wood filler and then re-oiled; stone or quartz takes a tinted two-part epoxy, built proud, flattened and polished. A chip on a front edge that keeps catching a sleeve is the one worth a new edge strip instead of a filler.",
    ),
    (
        r#"\bsink\b|\bsinks\b|\bbasin\b|\bceramic\b|\bporcelain\b|\bglazed\b|\bglaze\b"#,
        "Match the kit to the sink: a glazed ceramic one takes an enamel repair kit, and an acrylic or composite one takes an acrylic filler matched to the colour, because the enamel kit will not bond to plastic. Clean the chip right out, let it dry, and build two or three thin coats rather than one thick one. A chip through the glaze on a ceramic sink keeps dirt in the crater, so it is worth doing even though it will still show.",
    ),
    (
        r#"\benamel\b|\bbath\b|cast iron|steel bath|shower tray|\bcooker\b|\bhob\b"#,
        "An enamel chip wants an enamel repair kit: clean it, de-grease it, then build two or three thin coats of the enamel touch-up rather than one thick one, letting each go off. It is a cosmetic repair that stops the chip rusting and picking up dirt, and it will show a little; a bath chip through to the cast iron has to be done or the enamel round it starts to flake. A chip on a hob ring or a shower tray is the same kit, and a crack rather than a chip is a replacement.",
    ),
    (
        r#"\bplate\b|\bplates\b|\bmug\b|\bmugs\b|\bbowl\b|\bbowls\b|\bcup\b|\bcups\b|\bteapot\b|\bjug\b|\bdish\b|\bdishes\b|\bglass\b|\bglasses\b|\btumbler\b|wine glass|pint glass"#,
        "Retire a chipped plate, mug or glass: the chip is a sharp edge that will cut a lip, the glaze round it is no longer sealed, and there is no repair worth making. A chip on the foot or the underside is the one that is safe to keep for a plant pot or a pet bowl, and if it is glass, a chip anywhere near the rim is a break waiting to happen with hot water in it.",
    ),
    (
        r#"\btile\b|\btiles\b|\bgrout\b|\bsplashback\b"#,
        "A small chip in a tile hides with a dab of the same-colour grout, a touch-up pen or a drop of enamel, worked into the chip with a fingertip and wiped off the face at once. A chip out of a corner or an edge is the tile coming away from the wall, so press it back and check for a hollow sound; if it is hollow, the tile wants replacing rather than filling, and buy a spare now because the shade will not match later.",
    ),
    (
        r#"\bgap\b|\bgaps\b|crack between|cracks between|opening up|\bdraught\b|\bdraft\b"#,
        "A gap in a floor or under a skirting wants a flexible filler, not a hard one: rake the dust out, push in a foam backer rod if it is deep, then run a paintable acrylic sealant or a sliver of matching wood glued in and planed flat, because a hard filler in a floor cracks out with the first bit of movement. Between floorboards on a suspended floor, fill from above with a sliver of pine and glue rather than sealing it, so the boards can still move.",
    ),
    (
        r#"\bwood\b|\bwooden\b|\boak\b|\bpine\b|\bbeech\b|\bwalnut\b|\btable\b|\btabletop\b|\bfloorboard\b|\bfloor\b|\bdoor\b|\bskirting\b|\bfurniture\b"#,
        "Sand the chip back to sound wood, then fill it with a wood filler or a wax stick in the right colour, building it a little proud and paring it flat once it has gone off. Wipe a drop of oil or the same finish over it afterwards, or the bare patch shows as a light mark and dries out again. A chip on a corner that keeps being knocked wants a rounded edge or a corner trim rather than another fill.",
    ),
];
pub(super) const CHIP_OTHER: &str = "Match the repair to the surface, because a filler that is right for one is wrong for the next. A laminate chip takes a repair wax and a clear seal so water cannot reach the core; a stone top takes tinted two-part epoxy, flattened and polished; an enamel bath or basin takes an enamel touch-up kit in thin coats; a chipped plate or glass is one to retire, and a chipped tile takes colour-matched grout or enamel, or a replacement if the corner has lifted. Photograph it and take the chip's colour to a hardware shop rather than trusting the name on the tin.";
