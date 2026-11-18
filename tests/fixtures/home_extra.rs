//! Pinned PyBot household reply variants and abstention cases.

pub(super) const CASES: &[(&str, &str, Option<&str>)] = &[
    (
        "home/dryer-not-heating",
        "dryer is blowing cold air",
        Some(
            "Nearly always the air or the setting, not a part: check the dial first, since most machines finish on a cool-down phase that heats nothing, then empty the fluff filter and check the vent hose is not kinked. If the drum turns and the air still comes out cold, it is the thermal fuse on the element - an engineer's job, cheap part.",
        ),
    ),
    (
        "home/dryer-not-heating",
        "dryer is taking ages",
        Some(
            "Slow drying is nearly always the air: empty the fluff filter, rinse the condenser if it has one, and check the vent hose is not kinked. A drum stuffed full will not dry either, so try half a load. If the air still comes out cold, it is the fuse on the element - an engineer's job, cheap part.",
        ),
    ),
    (
        "home/dryer-not-heating",
        "damp washing after two cycles",
        Some(
            "Slow drying is nearly always the air: empty the fluff filter, rinse the condenser if it has one, and check the vent hose is not kinked. A drum stuffed full will not dry either, so try half a load. If the air still comes out cold, it is the fuse on the element - an engineer's job, cheap part.",
        ),
    ),
    (
        "home/dryer-not-heating",
        "why is my tumble dryer so slow",
        Some(
            "Slow drying is nearly always the air: empty the fluff filter, rinse the condenser if it has one, and check the vent hose is not kinked. A drum stuffed full will not dry either, so try half a load. If the air still comes out cold, it is the fuse on the element - an engineer's job, cheap part.",
        ),
    ),
    (
        "home/dryer-not-heating",
        "no heat from tumble dryer",
        Some(
            "Nearly always the air or the setting, not a part: check the dial first, since most machines finish on a cool-down phase that heats nothing, then empty the fluff filter and check the vent hose is not kinked. If the drum turns and the air still comes out cold, it is the thermal fuse on the element - an engineer's job, cheap part.",
        ),
    ),
    (
        "home/dryer-not-heating",
        "wet clothes",
        Some(
            "Slow drying is nearly always the air: empty the fluff filter, rinse the condenser if it has one, and check the vent hose is not kinked. A drum stuffed full will not dry either, so try half a load. If the air still comes out cold, it is the fuse on the element - an engineer's job, cheap part.",
        ),
    ),
    (
        "home/dryer-not-heating",
        "dryer stopped",
        Some(
            "Nearly always the air or the setting, not a part: check the dial first, since most machines finish on a cool-down phase that heats nothing, then empty the fluff filter and check the vent hose is not kinked. If the drum turns and the air still comes out cold, it is the thermal fuse on the element - an engineer's job, cheap part.",
        ),
    ),
    (
        "home/hair-dryer-not-hot",
        "hair dryer turns on but won't heat",
        Some(
            "A hair dryer that is completely dead is nearly always the plug or the flex, not the element: try another socket, check the fuse in the plug, and look at the point where the flex meets the handle, because a cable that is wound up every day is what breaks. If a fresh fuse blows again, stop using it - that is a short in the element, and a new dryer is cheaper than the repair.",
        ),
    ),
    (
        "home/hair-dryer-not-hot",
        "hairdryer blows cold air",
        Some(
            "Check the air intake first: a hair dryer that blows cold usually has the grille at the back furred up with hair and fluff, and the thermal cut-out trips before the element ever fails. Unplug it, pull the hair off the grille and out of the vent, let it cool for ten minutes and try it again on the high setting. If it still runs cold it is the element or the cut-out, and on a cheap dryer that is a new one rather than a repair - do not open the case, the element is a live part.",
        ),
    ),
    (
        "home/hair-dryer-not-hot",
        "my hair dryer stopped working",
        Some(
            "A hair dryer that is completely dead is nearly always the plug or the flex, not the element: try another socket, check the fuse in the plug, and look at the point where the flex meets the handle, because a cable that is wound up every day is what breaks. If a fresh fuse blows again, stop using it - that is a short in the element, and a new dryer is cheaper than the repair.",
        ),
    ),
    (
        "home/hair-dryer-not-hot",
        "the hair dryer blows but feels cold",
        Some(
            "Check the air intake first: a hair dryer that blows cold usually has the grille at the back furred up with hair and fluff, and the thermal cut-out trips before the element ever fails. Unplug it, pull the hair off the grille and out of the vent, let it cool for ten minutes and try it again on the high setting. If it still runs cold it is the element or the cut-out, and on a cheap dryer that is a new one rather than a repair - do not open the case, the element is a live part.",
        ),
    ),
    (
        "home/hair-dryer-not-hot",
        "hair dryer cuts out",
        Some(
            "Check the air intake first: a hair dryer that blows cold usually has the grille at the back furred up with hair and fluff, and the thermal cut-out trips before the element ever fails. Unplug it, pull the hair off the grille and out of the vent, let it cool for ten minutes and try it again on the high setting. If it still runs cold it is the element or the cut-out, and on a cheap dryer that is a new one rather than a repair - do not open the case, the element is a live part.",
        ),
    ),
    (
        "home/hair-dryer-not-hot",
        "my hair dryer is dead",
        Some(
            "A hair dryer that is completely dead is nearly always the plug or the flex, not the element: try another socket, check the fuse in the plug, and look at the point where the flex meets the handle, because a cable that is wound up every day is what breaks. If a fresh fuse blows again, stop using it - that is a short in the element, and a new dryer is cheaper than the repair.",
        ),
    ),
    (
        "home/a-nail-in-a-wall",
        "how do i pull the old nail out of the wall",
        Some(
            "Grip the head with pincers and slide a scrap of wood under them so the plaster does not crush, then ease it out with a slight twist rather than levering against the wall. A nail that turns but will not come has lost its grip: punch it below the surface with another nail, fill the dent with filler, sand it flat and touch up the paint.",
        ),
    ),
    (
        "home/a-nail-in-a-wall",
        "which nails can hang a picture",
        Some(
            "Tap it in at a slight downward angle so the weight hangs on the wall instead of pulling the head out, and put it into something solid - a stud, found by tapping, or brick under the plaster. On plasterboard with nothing behind it a bare nail or a picture hook pulls out: use a plasterboard plug or a self-drilling fixing, and keep the load light.",
        ),
    ),
    (
        "home/a-nail-in-a-wall",
        "how do i remove nail from plaster",
        Some(
            "Grip the head with pincers and slide a scrap of wood under them so the plaster does not crush, then ease it out with a slight twist rather than levering against the wall. A nail that turns but will not come has lost its grip: punch it below the surface with another nail, fill the dent with filler, sand it flat and touch up the paint.",
        ),
    ),
    (
        "home/a-nail-in-a-wall",
        "getting nail out of wall",
        Some(
            "Grip the head with pincers and slide a scrap of wood under them so the plaster does not crush, then ease it out with a slight twist rather than levering against the wall. A nail that turns but will not come has lost its grip: punch it below the surface with another nail, fill the dent with filler, sand it flat and touch up the paint.",
        ),
    ),
    (
        "home/a-nail-in-a-wall",
        "loose nail in picture hook",
        Some(
            "Grip the head with pincers and slide a scrap of wood under them so the plaster does not crush, then ease it out with a slight twist rather than levering against the wall. A nail that turns but will not come has lost its grip: punch it below the surface with another nail, fill the dent with filler, sand it flat and touch up the paint.",
        ),
    ),
    (
        "home/wood-splits-when-nailed",
        "which nail for a floorboard",
        Some(
            "Size the nail to the job, not to the room: 40mm to 50mm lost-head or oval nails for skirting and architrave, 50mm to 65mm for a floorboard, 30mm for a 6mm batten, and thin panel pins for beading and a back panel. As a rule the nail wants to be about two and a half times as long as the piece it is holding, and the top third of it has to bite into something solid behind.",
        ),
    ),
    (
        "home/wood-splits-when-nailed",
        "what mm nail for skirting",
        Some(
            "Size the nail to the job, not to the room: 40mm to 50mm lost-head or oval nails for skirting and architrave, 50mm to 65mm for a floorboard, 30mm for a 6mm batten, and thin panel pins for beading and a back panel. As a rule the nail wants to be about two and a half times as long as the piece it is holding, and the top third of it has to bite into something solid behind.",
        ),
    ),
    (
        "home/wood-splits-when-nailed",
        "wood split while hammering nail",
        Some(
            "Drill a pilot hole first, about three quarters as wide as the nail, and keep it at least 25mm in from the end of the board, because that is where a split starts. Blunt the point by tapping it once on a hard surface so it cuts the grain instead of wedging it apart, and clamp a scrap of wood behind the far face when the board is thin. A split that has already opened wants wood glue worked into it and a clamp overnight, not another nail.",
        ),
    ),
    (
        "home/wood-splits-when-nailed",
        "which nail won't split plywood",
        Some(
            "Drill a pilot hole first, about three quarters as wide as the nail, and keep it at least 25mm in from the end of the board, because that is where a split starts. Blunt the point by tapping it once on a hard surface so it cuts the grain instead of wedging it apart, and clamp a scrap of wood behind the far face when the board is thin. A split that has already opened wants wood glue worked into it and a clamp overnight, not another nail.",
        ),
    ),
    (
        "home/wood-splits-when-nailed",
        "what length nail for wood",
        Some(
            "Size the nail to the job, not to the room: 40mm to 50mm lost-head or oval nails for skirting and architrave, 50mm to 65mm for a floorboard, 30mm for a 6mm batten, and thin panel pins for beading and a back panel. As a rule the nail wants to be about two and a half times as long as the piece it is holding, and the top third of it has to bite into something solid behind.",
        ),
    ),
    (
        "home/wood-splits-when-nailed",
        "what nail should I use",
        Some(
            "Size the nail to the job, not to the room: 40mm to 50mm lost-head or oval nails for skirting and architrave, 50mm to 65mm for a floorboard, 30mm for a 6mm batten, and thin panel pins for beading and a back panel. As a rule the nail wants to be about two and a half times as long as the piece it is holding, and the top third of it has to bite into something solid behind.",
        ),
    ),
    (
        "home/wood-splits-when-nailed",
        "can i nail trim without cracking it",
        Some(
            "Drill a pilot hole first, about three quarters as wide as the nail, and keep it at least 25mm in from the end of the board, because that is where a split starts. Blunt the point by tapping it once on a hard surface so it cuts the grain instead of wedging it apart, and clamp a scrap of wood behind the far face when the board is thin. A split that has already opened wants wood glue worked into it and a clamp overnight, not another nail.",
        ),
    ),
    (
        "home/paper-going-yellow",
        "how do i keep paper from turning yellow",
        Some(
            "Cheap paper is made from wood pulp, and the lignin in it turns acidic and brown as it sits - light, heat and damp speed it up, which is why the box in the loft goes first. So keep paper cool, dry, dark and steady: a cupboard inside the house, not the loft, shed or cellar; acid-free folders and boxes rather than polythene bags, which trap damp; no sellotape and no lamination, both of which stain and cannot be undone. Yellowing that has already started can be slowed, not reversed.",
        ),
    ),
    (
        "home/paper-going-yellow",
        "how do i store musty papers",
        Some(
            "Musty paper is damp paper, and the smell is mould growing in it: stand the books or letters up with the pages fanned in a dry room for a day or two - not in sunlight, which yellows them faster - then keep them somewhere dry and steady. A sachet of silica gel in the box, a cupboard inside the house rather than the loft, garage or cellar, and a dehumidifier in a room that has had a leak will stop it coming back. Don't wipe them with bleach or a wet cloth: it takes the print with it.",
        ),
    ),
    (
        "home/paper-going-yellow",
        "my letters are foxing",
        Some(
            "Musty paper is damp paper, and the smell is mould growing in it: stand the books or letters up with the pages fanned in a dry room for a day or two - not in sunlight, which yellows them faster - then keep them somewhere dry and steady. A sachet of silica gel in the box, a cupboard inside the house rather than the loft, garage or cellar, and a dehumidifier in a room that has had a leak will stop it coming back. Don't wipe them with bleach or a wet cloth: it takes the print with it.",
        ),
    ),
    (
        "home/paper-going-yellow",
        "my photo is fading",
        Some(
            "Prints fade from light and from the plastic in old albums, so keep them out of sunlight, in acid-free sleeves or a proper album rather than a PVC one, and scan the ones that matter - a digital copy is the only version that cannot fade. A framed photo wants UV-filtering glass and a wall that never gets direct sun.",
        ),
    ),
    (
        "home/paper-going-yellow",
        "mildew on photos",
        Some(
            "Musty paper is damp paper, and the smell is mould growing in it: stand the books or letters up with the pages fanned in a dry room for a day or two - not in sunlight, which yellows them faster - then keep them somewhere dry and steady. A sachet of silica gel in the box, a cupboard inside the house rather than the loft, garage or cellar, and a dehumidifier in a room that has had a leak will stop it coming back. Don't wipe them with bleach or a wet cloth: it takes the print with it.",
        ),
    ),
    (
        "home/paper-going-yellow",
        "does a brown postcard still last",
        Some(
            "Cheap paper is made from wood pulp, and the lignin in it turns acidic and brown as it sits - light, heat and damp speed it up, which is why the box in the loft goes first. So keep paper cool, dry, dark and steady: a cupboard inside the house, not the loft, shed or cellar; acid-free folders and boxes rather than polythene bags, which trap damp; no sellotape and no lamination, both of which stain and cannot be undone. Yellowing that has already started can be slowed, not reversed.",
        ),
    ),
    (
        "home/paper-going-yellow",
        "album photos smell",
        Some(
            "Musty paper is damp paper, and the smell is mould growing in it: stand the books or letters up with the pages fanned in a dry room for a day or two - not in sunlight, which yellows them faster - then keep them somewhere dry and steady. A sachet of silica gel in the box, a cupboard inside the house rather than the loft, garage or cellar, and a dehumidifier in a room that has had a leak will stop it coming back. Don't wipe them with bleach or a wet cloth: it takes the print with it.",
        ),
    ),
    (
        "home/paper-going-yellow",
        "my prints are fading",
        Some(
            "Prints fade from light and from the plastic in old albums, so keep them out of sunlight, in acid-free sleeves or a proper album rather than a PVC one, and scan the ones that matter - a digital copy is the only version that cannot fade. A framed photo wants UV-filtering glass and a wall that never gets direct sun.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "I chipped my laminate worktop",
        Some(
            "Seal a laminate chip rather than fill it: a repair wax or a touch-up kit in the same colour goes in with a fingertip and a clear top coat stops water reaching the chipboard core, which is the part that swells and lifts the surface. If the edge of the chip has lifted, press it down with a warm iron through a cloth first. A chip along a front edge that keeps catching wants a new worktop strip or a whole top, because a filler there takes the knocks for a few months only.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "chip in quartz counter",
        Some(
            "A stone worktop chip is a two-part epoxy job, tinted to the stone: clean the chip out, build the epoxy slightly proud, let it cure hard and then flat it back with a fine diamond pad and polish. Granite and quartz are a fitter's call if the chip is bigger than a fingertip or on a joint, and most fitters will do a small one for a call-out fee. Never use a filler that shrinks or a heat gun near the stone, because a thermal shock cracks it.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "chip in the worktop",
        Some(
            "Find out what the top is made of before you buy anything, because the three repairs do not swap. A laminate or veneered top takes a repair wax in the matching colour and a clear seal over it, so water cannot reach the chipboard core and swell it; solid wood is sanded back and filled with a wax or wood filler and then re-oiled; stone or quartz takes a tinted two-part epoxy, built proud, flattened and polished. A chip on a front edge that keeps catching a sleeve is the one worth a new edge strip instead of a filler.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "chipped porcelain sink",
        Some(
            "Match the kit to the sink: a glazed ceramic one takes an enamel repair kit, and an acrylic or composite one takes an acrylic filler matched to the colour, because the enamel kit will not bond to plastic. Clean the chip right out, let it dry, and build two or three thin coats rather than one thick one. A chip through the glaze on a ceramic sink keeps dirt in the crater, so it is worth doing even though it will still show.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "chip in cast iron bath",
        Some(
            "An enamel chip wants an enamel repair kit: clean it, de-grease it, then build two or three thin coats of the enamel touch-up rather than one thick one, letting each go off. It is a cosmetic repair that stops the chip rusting and picking up dirt, and it will show a little; a bath chip through to the cast iron has to be done or the enamel round it starts to flake. A chip on a hob ring or a shower tray is the same kit, and a crack rather than a chip is a replacement.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "chip in a mug",
        Some(
            "Retire a chipped plate, mug or glass: the chip is a sharp edge that will cut a lip, the glaze round it is no longer sealed, and there is no repair worth making. A chip on the foot or the underside is the one that is safe to keep for a plant pot or a pet bowl, and if it is glass, a chip anywhere near the rim is a break waiting to happen with hot water in it.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "chip in a tile",
        Some(
            "A small chip in a tile hides with a dab of the same-colour grout, a touch-up pen or a drop of enamel, worked into the chip with a fingertip and wiped off the face at once. A chip out of a corner or an edge is the tile coming away from the wall, so press it back and check for a hollow sound; if it is hollow, the tile wants replacing rather than filling, and buy a spare now because the shade will not match later.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "gap between two floorboards",
        Some(
            "A gap in a floor or under a skirting wants a flexible filler, not a hard one: rake the dust out, push in a foam backer rod if it is deep, then run a paintable acrylic sealant or a sliver of matching wood glued in and planed flat, because a hard filler in a floor cracks out with the first bit of movement. Between floorboards on a suspended floor, fill from above with a sliver of pine and glue rather than sealing it, so the boards can still move.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "chip on oak door",
        Some(
            "Sand the chip back to sound wood, then fill it with a wood filler or a wax stick in the right colour, building it a little proud and paring it flat once it has gone off. Wipe a drop of oil or the same finish over it afterwards, or the bare patch shows as a light mark and dries out again. A chip on a corner that keeps being knocked wants a rounded edge or a corner trim rather than another fill.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "tiny chip in it",
        Some(
            "Match the repair to the surface, because a filler that is right for one is wrong for the next. A laminate chip takes a repair wax and a clear seal so water cannot reach the core; a stone top takes tinted two-part epoxy, flattened and polished; an enamel bath or basin takes an enamel touch-up kit in thin coats; a chipped plate or glass is one to retire, and a chipped tile takes colour-matched grout or enamel, or a replacement if the corner has lifted. Photograph it and take the chip's colour to a hardware shop rather than trusting the name on the tin.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "a chipset",
        Some(
            "Match the repair to the surface, because a filler that is right for one is wrong for the next. A laminate chip takes a repair wax and a clear seal so water cannot reach the core; a stone top takes tinted two-part epoxy, flattened and polished; an enamel bath or basin takes an enamel touch-up kit in thin coats; a chipped plate or glass is one to retire, and a chipped tile takes colour-matched grout or enamel, or a replacement if the corner has lifted. Photograph it and take the chip's colour to a hardware shop rather than trusting the name on the tin.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "farmhouse sink with quartz counter",
        Some(
            "A stone worktop chip is a two-part epoxy job, tinted to the stone: clean the chip out, build the epoxy slightly proud, let it cure hard and then flat it back with a fine diamond pad and polish. Granite and quartz are a fitter's call if the chip is bigger than a fingertip or on a joint, and most fitters will do a small one for a call-out fee. Never use a filler that shrinks or a heat gun near the stone, because a thermal shock cracks it.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "chip in splashback",
        Some(
            "A small chip in a tile hides with a dab of the same-colour grout, a touch-up pen or a drop of enamel, worked into the chip with a fingertip and wiped off the face at once. A chip out of a corner or an edge is the tile coming away from the wall, so press it back and check for a hollow sound; if it is hollow, the tile wants replacing rather than filling, and buy a spare now because the shade will not match later.",
        ),
    ),
    (
        "home/chip-in-a-surface",
        "cracked gap between boards",
        Some(
            "A gap in a floor or under a skirting wants a flexible filler, not a hard one: rake the dust out, push in a foam backer rod if it is deep, then run a paintable acrylic sealant or a sliver of matching wood glued in and planed flat, because a hard filler in a floor cracks out with the first bit of movement. Between floorboards on a suspended floor, fill from above with a sliver of pine and glue rather than sealing it, so the boards can still move.",
        ),
    ),
    (
        "home/property-word",
        "what is ground rent",
        Some(
            "Ground rent is the yearly charge a leaseholder pays to the freeholder for the land the flat stands on - often £50 to £250, though some newer leases set it much higher. Read the lease for how it reviews: one that doubles every ten years, or rises with the retail prices index, is a real problem, because a mortgage lender will refuse it and a buyer will too. Ask the seller for the last three years of demands.",
        ),
    ),
    (
        "home/property-word",
        "what is ground rents",
        Some(
            "Ground rent is the yearly charge a leaseholder pays to the freeholder for the land the flat stands on - often £50 to £250, though some newer leases set it much higher. Read the lease for how it reviews: one that doubles every ten years, or rises with the retail prices index, is a real problem, because a mortgage lender will refuse it and a buyer will too. Ask the seller for the last three years of demands.",
        ),
    ),
    ("home/property-word", "what is service charge", None),
    ("home/property-word", "what is service charges", None),
    (
        "home/property-word",
        "what is maintenance charge",
        Some(
            "A service charge is your share of what the block costs to run: buildings insurance, the cleaner, the lift, the lights, the roof, and the managing agent's fee. It is not profit for the landlord - you are entitled to the yearly accounts and to challenge anything unreasonable at a tribunal - so ask for the last three years of it and what the reserve fund holds for big jobs. A charge that jumps in one year is usually a repair or an insurance renewal, and the accounts will say which.",
        ),
    ),
    (
        "home/property-word",
        "what is maintenance charges",
        Some(
            "A service charge is your share of what the block costs to run: buildings insurance, the cleaner, the lift, the lights, the roof, and the managing agent's fee. It is not profit for the landlord - you are entitled to the yearly accounts and to challenge anything unreasonable at a tribunal - so ask for the last three years of it and what the reserve fund holds for big jobs. A charge that jumps in one year is usually a repair or an insurance renewal, and the accounts will say which.",
        ),
    ),
    ("home/property-word", "what is management fee", None),
    ("home/property-word", "what is management fees", None),
    (
        "home/property-word",
        "what is estate charge",
        Some(
            "A service charge is your share of what the block costs to run: buildings insurance, the cleaner, the lift, the lights, the roof, and the managing agent's fee. It is not profit for the landlord - you are entitled to the yearly accounts and to challenge anything unreasonable at a tribunal - so ask for the last three years of it and what the reserve fund holds for big jobs. A charge that jumps in one year is usually a repair or an insurance renewal, and the accounts will say which.",
        ),
    ),
    (
        "home/property-word",
        "what is estate charges",
        Some(
            "A service charge is your share of what the block costs to run: buildings insurance, the cleaner, the lift, the lights, the roof, and the managing agent's fee. It is not profit for the landlord - you are entitled to the yearly accounts and to challenge anything unreasonable at a tribunal - so ask for the last three years of it and what the reserve fund holds for big jobs. A charge that jumps in one year is usually a repair or an insurance renewal, and the accounts will say which.",
        ),
    ),
    (
        "home/property-word",
        "what is managing agent",
        Some(
            "The managing agent is the company the freeholder pays to run the block day to day: repairs, insurance, the service charge accounts and the cleaning. It is not the freeholder and it cannot sell you the freehold, but it is who you ring about the broken door. Ask for the agent's name and the last set of accounts before you buy, and put a complaint about the block in writing to the agent first, because the freeholder is only reachable through it.",
        ),
    ),
    (
        "home/property-word",
        "what is managing agents",
        Some(
            "The managing agent is the company the freeholder pays to run the block day to day: repairs, insurance, the service charge accounts and the cleaning. It is not the freeholder and it cannot sell you the freehold, but it is who you ring about the broken door. Ask for the agent's name and the last set of accounts before you buy, and put a complaint about the block in writing to the agent first, because the freeholder is only reachable through it.",
        ),
    ),
    (
        "home/property-word",
        "what is management company",
        Some(
            "The managing agent is the company the freeholder pays to run the block day to day: repairs, insurance, the service charge accounts and the cleaning. It is not the freeholder and it cannot sell you the freehold, but it is who you ring about the broken door. Ask for the agent's name and the last set of accounts before you buy, and put a complaint about the block in writing to the agent first, because the freeholder is only reachable through it.",
        ),
    ),
    (
        "home/property-word",
        "what is managing company",
        Some(
            "The managing agent is the company the freeholder pays to run the block day to day: repairs, insurance, the service charge accounts and the cleaning. It is not the freeholder and it cannot sell you the freehold, but it is who you ring about the broken door. Ask for the agent's name and the last set of accounts before you buy, and put a complaint about the block in writing to the agent first, because the freeholder is only reachable through it.",
        ),
    ),
    (
        "home/property-word",
        "what is freeholder",
        Some(
            "The freeholder owns the building and the land; a leaseholder owns the right to live in one flat for a set number of years. The freeholder's jobs are the structure and the outside, paid for by the service charge, and the leaseholders have the right to buy that freehold between them if enough of them want to.",
        ),
    ),
    (
        "home/property-word",
        "what is freeholders",
        Some(
            "The freeholder owns the building and the land; a leaseholder owns the right to live in one flat for a set number of years. The freeholder's jobs are the structure and the outside, paid for by the service charge, and the leaseholders have the right to buy that freehold between them if enough of them want to.",
        ),
    ),
    (
        "home/property-word",
        "what is freehold",
        Some(
            "Freehold means you own the building and the land it stands on outright, with no end date and no ground rent or service charge to anyone. A house is usually freehold; a flat is usually leasehold, or the flat is leasehold and the freehold is owned by a company the flat owners share.",
        ),
    ),
    (
        "home/property-word",
        "what is leasehold",
        Some(
            "Leasehold means you own the right to live in the property for the number of years left on the lease, not the land under it. You pay ground rent and a service charge to the freeholder, and the lease is the contract that says what you may and may not do - which is why the years left matter so much when you sell.",
        ),
    ),
    (
        "home/property-word",
        "what is covenant",
        Some(
            "A covenant is a promise written into a deed. In a lease it is what you must and must not do (no subletting, keep the hallway clear); on a freehold it is usually a restriction on the land, such as not running a business from the house, and it can still be enforced fifty years on. A deed of covenant on its own is also the name for a signed promise to pay a fixed sum - often to a charity, so it can claim the tax back.",
        ),
    ),
    (
        "home/property-word",
        "what is covenants",
        Some(
            "A covenant is a promise written into a deed. In a lease it is what you must and must not do (no subletting, keep the hallway clear); on a freehold it is usually a restriction on the land, such as not running a business from the house, and it can still be enforced fifty years on. A deed of covenant on its own is also the name for a signed promise to pay a fixed sum - often to a charity, so it can claim the tax back.",
        ),
    ),
    (
        "home/property-word",
        "what is deed of covenant",
        Some(
            "A covenant is a promise written into a deed. In a lease it is what you must and must not do (no subletting, keep the hallway clear); on a freehold it is usually a restriction on the land, such as not running a business from the house, and it can still be enforced fifty years on. A deed of covenant on its own is also the name for a signed promise to pay a fixed sum - often to a charity, so it can claim the tax back.",
        ),
    ),
    (
        "home/property-word",
        "what is deeds of covenant",
        Some(
            "A covenant is a promise written into a deed. In a lease it is what you must and must not do (no subletting, keep the hallway clear); on a freehold it is usually a restriction on the land, such as not running a business from the house, and it can still be enforced fifty years on. A deed of covenant on its own is also the name for a signed promise to pay a fixed sum - often to a charity, so it can claim the tax back.",
        ),
    ),
    (
        "home/property-word",
        "what is restrictive covenant",
        Some(
            "A covenant is a promise written into a deed. In a lease it is what you must and must not do (no subletting, keep the hallway clear); on a freehold it is usually a restriction on the land, such as not running a business from the house, and it can still be enforced fifty years on. A deed of covenant on its own is also the name for a signed promise to pay a fixed sum - often to a charity, so it can claim the tax back.",
        ),
    ),
    (
        "home/property-word",
        "what is restrictive covenants",
        Some(
            "A covenant is a promise written into a deed. In a lease it is what you must and must not do (no subletting, keep the hallway clear); on a freehold it is usually a restriction on the land, such as not running a business from the house, and it can still be enforced fifty years on. A deed of covenant on its own is also the name for a signed promise to pay a fixed sum - often to a charity, so it can claim the tax back.",
        ),
    ),
    (
        "home/property-word",
        "what is easement",
        Some(
            "An easement is a right to use someone else's land for one purpose: a right of way across a drive, drains running under a neighbour's garden, or access to lay pipes. It stays with the property rather than the person, so it is on the title and a buyer's solicitor will find it. You cannot build over one, and it needs registering to be safe.",
        ),
    ),
    (
        "home/property-word",
        "what is easements",
        Some(
            "An easement is a right to use someone else's land for one purpose: a right of way across a drive, drains running under a neighbour's garden, or access to lay pipes. It stays with the property rather than the person, so it is on the title and a buyer's solicitor will find it. You cannot build over one, and it needs registering to be safe.",
        ),
    ),
    (
        "home/property-word",
        "what is title deed",
        Some(
            "The title deeds are the documents that prove who owns a property. In England and Wales they are now held electronically by HM Land Registry, so there is nothing for you to keep in a safe - a copy of the title register can be downloaded for a few pounds. It lists the owner, the price last paid, the boundaries on a map and anything registered against the property, such as a mortgage or a covenant.",
        ),
    ),
    (
        "home/property-word",
        "what is title deeds",
        Some(
            "The title deeds are the documents that prove who owns a property. In England and Wales they are now held electronically by HM Land Registry, so there is nothing for you to keep in a safe - a copy of the title register can be downloaded for a few pounds. It lists the owner, the price last paid, the boundaries on a map and anything registered against the property, such as a mortgage or a covenant.",
        ),
    ),
    (
        "home/property-word",
        "what is title register",
        Some(
            "The title deeds are the documents that prove who owns a property. In England and Wales they are now held electronically by HM Land Registry, so there is nothing for you to keep in a safe - a copy of the title register can be downloaded for a few pounds. It lists the owner, the price last paid, the boundaries on a map and anything registered against the property, such as a mortgage or a covenant.",
        ),
    ),
    (
        "home/property-word",
        "what is land registry",
        Some(
            "The title deeds are the documents that prove who owns a property. In England and Wales they are now held electronically by HM Land Registry, so there is nothing for you to keep in a safe - a copy of the title register can be downloaded for a few pounds. It lists the owner, the price last paid, the boundaries on a map and anything registered against the property, such as a mortgage or a covenant.",
        ),
    ),
    (
        "home/property-word",
        "what is conveyancer",
        Some(
            "A conveyancer is the solicitor or licensed conveyancer who does the legal work of buying or selling a home: searches, the contract, the lease or title, the mortgage conditions and the money on completion. Fees are usually £800 to £2,000 plus searches and VAT; ask for the total quoted for your property, and use one who is on your lender's panel if you have a mortgage.",
        ),
    ),
    (
        "home/property-word",
        "what is conveyancers",
        Some(
            "A conveyancer is the solicitor or licensed conveyancer who does the legal work of buying or selling a home: searches, the contract, the lease or title, the mortgage conditions and the money on completion. Fees are usually £800 to £2,000 plus searches and VAT; ask for the total quoted for your property, and use one who is on your lender's panel if you have a mortgage.",
        ),
    ),
    (
        "home/property-word",
        "what is conveyancing",
        Some(
            "A conveyancer is the solicitor or licensed conveyancer who does the legal work of buying or selling a home: searches, the contract, the lease or title, the mortgage conditions and the money on completion. Fees are usually £800 to £2,000 plus searches and VAT; ask for the total quoted for your property, and use one who is on your lender's panel if you have a mortgage.",
        ),
    ),
    (
        "home/property-word",
        "what is what is a lease extension",
        Some(
            "A lease extension is a new, longer lease on the same flat: you pay the freeholder a premium and the years are added, 90 on top of what is left if you use the legal right, which you have once you have owned the flat for two years. Your solicitor serves the notice and the new lease is registered at the Land Registry; the premium is the negotiation. Do it before the lease drops under about 80 years, because the price climbs sharply after that.",
        ),
    ),
    (
        "home/property-word",
        "what is whats a lease extension",
        Some(
            "A lease extension is a new, longer lease on the same flat: you pay the freeholder a premium and the years are added, 90 on top of what is left if you use the legal right, which you have once you have owned the flat for two years. Your solicitor serves the notice and the new lease is registered at the Land Registry; the premium is the negotiation. Do it before the lease drops under about 80 years, because the price climbs sharply after that.",
        ),
    ),
    (
        "home/property-word",
        "what is lease extension mean",
        Some(
            "A lease extension is a new, longer lease on the same flat: you pay the freeholder a premium and the years are added, 90 on top of what is left if you use the legal right, which you have once you have owned the flat for two years. Your solicitor serves the notice and the new lease is registered at the Land Registry; the premium is the negotiation. Do it before the lease drops under about 80 years, because the price climbs sharply after that.",
        ),
    ),
    (
        "home/property-word",
        "what is lease extension meaning",
        Some(
            "A lease extension is a new, longer lease on the same flat: you pay the freeholder a premium and the years are added, 90 on top of what is left if you use the legal right, which you have once you have owned the flat for two years. Your solicitor serves the notice and the new lease is registered at the Land Registry; the premium is the negotiation. Do it before the lease drops under about 80 years, because the price climbs sharply after that.",
        ),
    ),
    (
        "home/property-word",
        "what is what does lease extension mean",
        Some(
            "A lease extension is a new, longer lease on the same flat: you pay the freeholder a premium and the years are added, 90 on top of what is left if you use the legal right, which you have once you have owned the flat for two years. Your solicitor serves the notice and the new lease is registered at the Land Registry; the premium is the negotiation. Do it before the lease drops under about 80 years, because the price climbs sharply after that.",
        ),
    ),
    (
        "home/property-word",
        "what is define lease extension",
        Some(
            "A lease extension is a new, longer lease on the same flat: you pay the freeholder a premium and the years are added, 90 on top of what is left if you use the legal right, which you have once you have owned the flat for two years. Your solicitor serves the notice and the new lease is registered at the Land Registry; the premium is the negotiation. Do it before the lease drops under about 80 years, because the price climbs sharply after that.",
        ),
    ),
    (
        "home/property-word",
        "what is lease extension",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is extend my lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is extend the lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is extending my lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is short lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is lease length",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is how long is the lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is how long is my lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is how long is a lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is how long a lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is how many years left on the lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is how many years left on my lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is years left on the lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is years are left on the lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is years are left on my lease",
        Some(
            "A lease is your right to live there for a fixed number of years, and the length is what matters at the end: below about 80 years it gets expensive to extend and some lenders will not touch it. After two years of owning it you have the right to add 90 years at a peppercorn rent, which is a legal route rather than a favour, so a short lease is a negotiating point on the price and not a reason to walk away. Get the premium quoted by a surveyor who does lease extensions before you agree anything.",
        ),
    ),
    (
        "home/property-word",
        "what is what is a lease",
        Some(
            "A lease is the document that gives you the right to live in a flat for a set number of years - 99, 125 and 999 are the usual lengths - and it comes with ground rent, a service charge and rules about the building. You own the inside of the flat and the right to live there; the freeholder owns the structure and the land. The two numbers to check are how many years are left and what the ground rent does.",
        ),
    ),
    (
        "home/property-word",
        "what is whats a lease",
        Some(
            "A lease is the document that gives you the right to live in a flat for a set number of years - 99, 125 and 999 are the usual lengths - and it comes with ground rent, a service charge and rules about the building. You own the inside of the flat and the right to live there; the freeholder owns the structure and the land. The two numbers to check are how many years are left and what the ground rent does.",
        ),
    ),
    (
        "home/property-word",
        "what is what is a leasehold",
        Some(
            "Leasehold means you own the right to live in the property for the number of years left on the lease, not the land under it. You pay ground rent and a service charge to the freeholder, and the lease is the contract that says what you may and may not do - which is why the years left matter so much when you sell.",
        ),
    ),
    (
        "home/property-word",
        "what is leasehold mean",
        Some(
            "Leasehold means you own the right to live in the property for the number of years left on the lease, not the land under it. You pay ground rent and a service charge to the freeholder, and the lease is the contract that says what you may and may not do - which is why the years left matter so much when you sell.",
        ),
    ),
    (
        "home/property-word",
        "what is leasehold meaning",
        Some(
            "Leasehold means you own the right to live in the property for the number of years left on the lease, not the land under it. You pay ground rent and a service charge to the freeholder, and the lease is the contract that says what you may and may not do - which is why the years left matter so much when you sell.",
        ),
    ),
    (
        "home/property-word",
        "what is epc",
        Some(
            "An EPC is the energy rating for the property, A to G, and it has to be done before a home is sold or rented, and it lasts ten years. E is the lowest rating a landlord may let a home on in England and Wales, so a flat rated F or G is either a bargaining point or a bill for insulation and a new boiler. Read the recommendation list on page three, because that is where the cheap wins are.",
        ),
    ),
    (
        "home/property-word",
        "what is energy performance certificate",
        Some(
            "An EPC is the energy rating for the property, A to G, and it has to be done before a home is sold or rented, and it lasts ten years. E is the lowest rating a landlord may let a home on in England and Wales, so a flat rated F or G is either a bargaining point or a bill for insulation and a new boiler. Read the recommendation list on page three, because that is where the cheap wins are.",
        ),
    ),
    (
        "home/property-word",
        "what is energy performance",
        Some(
            "An EPC is the energy rating for the property, A to G, and it has to be done before a home is sold or rented, and it lasts ten years. E is the lowest rating a landlord may let a home on in England and Wales, so a flat rated F or G is either a bargaining point or a bill for insulation and a new boiler. Read the recommendation list on page three, because that is where the cheap wins are.",
        ),
    ),
    (
        "home/property-word",
        "what is gazumping",
        Some(
            "Gazumping is a seller accepting a higher offer after they had already accepted yours, which in England and Wales is legal until contracts are exchanged - nothing is binding before that, and you can lose the survey and legal fees you have paid. The defences are to ask for the property to be taken off the market in writing and to move fast towards exchange; in Scotland the offer is binding much earlier, so the risk is different there.",
        ),
    ),
    (
        "home/property-word",
        "what is gazumped",
        Some(
            "Gazumping is a seller accepting a higher offer after they had already accepted yours, which in England and Wales is legal until contracts are exchanged - nothing is binding before that, and you can lose the survey and legal fees you have paid. The defences are to ask for the property to be taken off the market in writing and to move fast towards exchange; in Scotland the offer is binding much earlier, so the risk is different there.",
        ),
    ),
    (
        "home/property-word",
        "what is gazumping mean",
        Some(
            "Gazumping is a seller accepting a higher offer after they had already accepted yours, which in England and Wales is legal until contracts are exchanged - nothing is binding before that, and you can lose the survey and legal fees you have paid. The defences are to ask for the property to be taken off the market in writing and to move fast towards exchange; in Scotland the offer is binding much earlier, so the risk is different there.",
        ),
    ),
    (
        "home/property-word",
        "what is snagging list",
        Some(
            "A snagging list is the list of small faults in a new build - a door that rubs, a crooked tile, a scratch on the worktop - which you hand to the developer to put right. Do it in the first days after you get the keys rather than at the end of the year, with a photo of each snag and where it is, and keep the emailed copy: the warranty usually gives you two years for this kind of thing, and the list is what starts the clock.",
        ),
    ),
    (
        "home/property-word",
        "what is snagging",
        Some(
            "A snagging list is the list of small faults in a new build - a door that rubs, a crooked tile, a scratch on the worktop - which you hand to the developer to put right. Do it in the first days after you get the keys rather than at the end of the year, with a photo of each snag and where it is, and keep the emailed copy: the warranty usually gives you two years for this kind of thing, and the list is what starts the clock.",
        ),
    ),
    (
        "home/property-word",
        "what is snags list",
        Some(
            "A snagging list is the list of small faults in a new build - a door that rubs, a crooked tile, a scratch on the worktop - which you hand to the developer to put right. Do it in the first days after you get the keys rather than at the end of the year, with a photo of each snag and where it is, and keep the emailed copy: the warranty usually gives you two years for this kind of thing, and the list is what starts the clock.",
        ),
    ),
    (
        "home/property-word",
        "what is peppercorn",
        Some(
            "A peppercorn rent is a ground rent of nothing at all - the name is the old token payment, and it means the lease has no ground rent to pay. A lease extended the statutory way ends up on one, so a peppercorn ground rent in a lease is a good sign rather than a puzzle.",
        ),
    ),
    (
        "home/property-word",
        "what is peppercorn rent",
        Some(
            "A peppercorn rent is a ground rent of nothing at all - the name is the old token payment, and it means the lease has no ground rent to pay. A lease extended the statutory way ends up on one, so a peppercorn ground rent in a lease is a good sign rather than a puzzle.",
        ),
    ),
    (
        "home/property-word",
        "what is the driver service fee for a booking",
        None,
    ),
    (
        "home/property-word",
        "what is a service charge on a leased flat",
        Some(
            "A service charge is your share of what the block costs to run: buildings insurance, the cleaner, the lift, the lights, the roof, and the managing agent's fee. It is not profit for the landlord - you are entitled to the yearly accounts and to challenge anything unreasonable at a tribunal - so ask for the last three years of it and what the reserve fund holds for big jobs. A charge that jumps in one year is usually a repair or an insurance renewal, and the accounts will say which.",
        ),
    ),
    (
        "home/property-word",
        "we paid the building maintenance charge",
        Some(
            "A service charge is your share of what the block costs to run: buildings insurance, the cleaner, the lift, the lights, the roof, and the managing agent's fee. It is not profit for the landlord - you are entitled to the yearly accounts and to challenge anything unreasonable at a tribunal - so ask for the last three years of it and what the reserve fund holds for big jobs. A charge that jumps in one year is usually a repair or an insurance renewal, and the accounts will say which.",
        ),
    ),
    ("home/property-word", "the dinner service charge", None),
    ("home/property-word", "where to go", None),
    (
        "home/home-word-2",
        "what is ground rent",
        Some(
            "Ground rent is the yearly charge a leaseholder pays the freeholder for the land the flat stands on, on top of the service charge: often £100 to £300, and now usually a fixed sum written into the lease. It is not the same as the service charge, which pays for the building, and a lease with ground rent that doubles every ten years is the one to argue about before you buy.",
        ),
    ),
    (
        "home/home-word-2",
        "what is tenancy deposit",
        Some(
            "A tenancy deposit is the money you pay a landlord at the start to cover damage or unpaid rent, usually five weeks' rent, and it is not the landlord's to keep: in England and Wales it must be put in a government-backed protection scheme within 30 days and you get a certificate saying where. If it was not protected, or your landlord keeps it for wear and tear, you can claim up to three times the deposit.",
        ),
    ),
    (
        "home/home-word-2",
        "what is deposit scheme",
        Some(
            "A tenancy deposit is the money you pay a landlord at the start to cover damage or unpaid rent, usually five weeks' rent, and it is not the landlord's to keep: in England and Wales it must be put in a government-backed protection scheme within 30 days and you get a certificate saying where. If it was not protected, or your landlord keeps it for wear and tear, you can claim up to three times the deposit.",
        ),
    ),
    (
        "home/home-word-2",
        "what is deposit protection",
        Some(
            "A tenancy deposit is the money you pay a landlord at the start to cover damage or unpaid rent, usually five weeks' rent, and it is not the landlord's to keep: in England and Wales it must be put in a government-backed protection scheme within 30 days and you get a certificate saying where. If it was not protected, or your landlord keeps it for wear and tear, you can claim up to three times the deposit.",
        ),
    ),
    (
        "home/home-word-2",
        "what is reserve fund",
        Some(
            "A reserve fund - a sinking fund, in a block of flats - is money the leaseholders pay into each month so the big jobs of the next twenty years, a new roof or lifts or external paint, are already paid for when they come. It is separate from the service charge for this year's running costs, and the balance shows in the annual accounts, so check it before you buy a leasehold flat.",
        ),
    ),
    (
        "home/home-word-2",
        "what is major works fund",
        Some(
            "A reserve fund - a sinking fund, in a block of flats - is money the leaseholders pay into each month so the big jobs of the next twenty years, a new roof or lifts or external paint, are already paid for when they come. It is separate from the service charge for this year's running costs, and the balance shows in the annual accounts, so check it before you buy a leasehold flat.",
        ),
    ),
    (
        "home/home-word-2",
        "what is reserve funds",
        Some(
            "A reserve fund - a sinking fund, in a block of flats - is money the leaseholders pay into each month so the big jobs of the next twenty years, a new roof or lifts or external paint, are already paid for when they come. It is separate from the service charge for this year's running costs, and the balance shows in the annual accounts, so check it before you buy a leasehold flat.",
        ),
    ),
    (
        "home/home-word-2",
        "what is break clause",
        Some(
            "A break clause is a term in a tenancy or a commercial lease that lets either side end it early, on a date and with notice written into the contract: you still pay the rent until that date, and the notice is often two months. Ask exactly how the notice has to be given, in writing, and diary the day two months before the break date.",
        ),
    ),
    (
        "home/home-word-2",
        "what is breakclause",
        Some(
            "A break clause is a term in a tenancy or a commercial lease that lets either side end it early, on a date and with notice written into the contract: you still pay the rent until that date, and the notice is often two months. Ask exactly how the notice has to be given, in writing, and diary the day two months before the break date.",
        ),
    ),
    (
        "home/home-word-2",
        "what is inventory",
        Some(
            "An inventory is the list of what was in the flat and what condition it was in when you moved in, with photos, and it is the document the deposit is judged against at the end. Check it against the flat on the day, write down anything missing or already damaged, date and sign it, and photograph any marks before you send it back.",
        ),
    ),
    (
        "home/home-word-2",
        "what is check in report",
        Some(
            "An inventory is the list of what was in the flat and what condition it was in when you moved in, with photos, and it is the document the deposit is judged against at the end. Check it against the flat on the day, write down anything missing or already damaged, date and sign it, and photograph any marks before you send it back.",
        ),
    ),
    (
        "home/home-word-2",
        "what is check-in report",
        Some(
            "An inventory is the list of what was in the flat and what condition it was in when you moved in, with photos, and it is the document the deposit is judged against at the end. Check it against the flat on the day, write down anything missing or already damaged, date and sign it, and photograph any marks before you send it back.",
        ),
    ),
    (
        "home/home-word-2",
        "what is contents insurance",
        Some(
            "Contents insurance covers the things in the home: furniture, clothes, a laptop, a bike if it is listed, the damage a leak or a fire does to them - where buildings insurance covers the structure itself, which is the landlord's job in a rented flat. Add up what it would cost to replace everything new, keep receipts for anything over a few hundred pounds, and check the single-item limit before you assume the bike is covered.",
        ),
    ),
    (
        "home/home-word-2",
        "what is contents cover",
        Some(
            "Contents insurance covers the things in the home: furniture, clothes, a laptop, a bike if it is listed, the damage a leak or a fire does to them - where buildings insurance covers the structure itself, which is the landlord's job in a rented flat. Add up what it would cost to replace everything new, keep receipts for anything over a few hundred pounds, and check the single-item limit before you assume the bike is covered.",
        ),
    ),
    ("home/home-word-2", "what is my contents cover", None),
    (
        "home/home-word-2",
        "what does a grammar break clause mean",
        None,
    ),
    (
        "home/home-word-2",
        "what does break clause mean",
        Some(
            "A break clause is a term in a tenancy or a commercial lease that lets either side end it early, on a date and with notice written into the contract: you still pay the rent until that date, and the notice is often two months. Ask exactly how the notice has to be given, in writing, and diary the day two months before the break date.",
        ),
    ),
    ("home/home-word-2", "whats up", None),
];
