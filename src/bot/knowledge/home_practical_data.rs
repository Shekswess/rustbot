//! Rule data from the pinned home and home_craft domains (Apache-2.0).

pub(super) const COLOUR_PARTNERS: &[(&str, &str)] = &[
    (
        "blue",
        concat!(
            "Warm company stops blue going cold: mustard, tan leather, brass, a warm white on the ",
            "walls, and one wooden thing.",
        ),
    ),
    (
        "navy",
        concat!(
            "Cream, camel and brass suit navy, or a pale grey with a shot of green; keep the metals ",
            "warm so it does not read as a uniform.",
        ),
    ),
    (
        "grey",
        concat!(
            "Warm grey up with wood, brass and one soft colour - blush, mustard or sage - because ",
            "grey with grey goes flat and grey with black goes cold.",
        ),
    ),
    (
        "green",
        concat!(
            "White or cream lets green breathe, then pink, mustard or brown leather; a dark green ",
            "wants brass and a pale wall behind it.",
        ),
    ),
    (
        "sage",
        concat!(
            "Off-white, pale wood and terracotta suit sage, with a darker green somewhere to give it ",
            "depth.",
        ),
    ),
    (
        "teal",
        "Mustard, coral, brass and a warm white suit teal, and most of the room should stay pale.",
    ),
    (
        "black",
        concat!(
            "Warm neutrals stop black reading as an empty room: cream, camel, walnut and brass, with ",
            "one bright colour in a cushion or a plant pot.",
        ),
    ),
    (
        "brown",
        concat!(
            "Light walls and a strong third colour: cream, olive, rust or a deep blue. Brown on brown",
            " in a small room needs a pale floor.",
        ),
    ),
    (
        "beige",
        concat!(
            "Texture, one darker tone from the same family and a green plant: flat beige is what ",
            "makes a room look unfinished.",
        ),
    ),
    (
        "cream",
        concat!(
            "Nearly everything suits cream, which is the risk - anchor it with wood, black or a deep ",
            "colour so it does not look like undercoat.",
        ),
    ),
    (
        "white",
        concat!(
            "Wood, black, brass and any one colour you like; the trick with white is texture, not ",
            "shade.",
        ),
    ),
    (
        "yellow",
        concat!(
            "Grey or white calms yellow, black or navy sharpens it, and orange is the one colour not ",
            "to put in the same room.",
        ),
    ),
    (
        "mustard",
        concat!(
            "Navy, charcoal or olive on a cream wall; mustard is strong enough to be the only colour ",
            "in the room.",
        ),
    ),
    (
        "pink",
        concat!(
            "Grey, olive or brass suit pink; a grown-up pink is dusty rather than bright, and it sits",
            " well with a brown wood floor.",
        ),
    ),
    (
        "red",
        concat!(
            "Small amounts, with a lot of white or black: a red sofa wants pale walls and no second ",
            "colour.",
        ),
    ),
    (
        "terracotta",
        "Cream, olive and pale wood suit terracotta, with a little black to stop it going sweet.",
    ),
    (
        "purple",
        concat!(
            "Grey or cream walls, a little gold and a white floor: a whole purple room goes dark in ",
            "the evening.",
        ),
    ),
    (
        "orange",
        concat!(
            "Grey or white walls with a cool blue somewhere, because orange dates fast in large ",
            "amounts.",
        ),
    ),
    (
        "silver",
        "Warm wood and cream, or one deep colour: cold metal against cold walls feels clinical.",
    ),
];

pub(super) const COLOUR_NEUTRAL: &[&str] = &[
    "beige", "black", "brown", "charcoal", "cream", "gray", "grey", "navy", "taupe", "white",
];

pub(super) const COLOUR_WARM: &[&str] = &[
    "brown",
    "mustard",
    "orange",
    "pink",
    "red",
    "terracotta",
    "yellow",
];

pub(super) const LOST_HOUSE_KEYS_REPLY: &str = concat!(
    "Retrace the last hour in order and check the boring places again - coat pocket, under ",
    "the car seat, the last counter you were at. If they are really gone: the spare first, ",
    "then whoever else has a set, and with no spare change the barrel rather than hope, which",
    " a locksmith does for about £60 to £120. If the ring had your address on it, do that ",
    "today.",
);

pub(super) const LOST_CAR_KEYS_REPLY: &str = concat!(
    "Search the car properly before anything else - between the seats, the door pocket, the ",
    "boot, and the pockets of the coat you wore. If it is gone, a car key is a dealer or an ",
    "auto locksmith job: about £150 to £250 to cut and code a new one, and you need the V5 ",
    "and photo ID. If it was stolen rather than mislaid, ring your insurer before you replace",
    " it, because theft cover often pays.",
);

pub(super) const KEYS_AGAIN_REPLY: &str = concat!(
    "Give them a place of their own and nothing else: a bowl or a hook by the front door, ",
    "emptied the moment you come in, so the same spot takes them every time. That is the part",
    " that stops it happening - a key hook by the door costs a few pounds and works better ",
    "than remembering. For today: retrace the last hour again - coat pocket, under the car ",
    "seat, the last counter you were at.",
);

pub(super) const CAR_KEYS_AGAIN_REPLY: &str = concat!(
    "A key bowl by the door is the fix for a fob that keeps vanishing, because it is the only",
    " place it ever gets put down. For today: search the car properly - between the seats, ",
    "the door pocket, the boot, the pockets of the coat you wore.",
);

pub(super) const CAR_KEY_LOST: &str =
    r#"(?i)\b(?:car|van|motorbike|motorcycle|vehicle|keyless|fob|fobs)\b"#;
pub(super) const KEYS_AGAIN: &str = concat!(
    r#"(?i)\b(?:keep|keeps|always|constantly|forever|again|over and over|every time|twice|second time"#,
    r#"|third time|lost them before|every (?:single |other |blooming )?(?:day|week|month)|every day"#,
    r#"|all the time|most days|a lot of the time|regularly|on a regular basis)\b"#,
);
pub(super) const KEYS_KEPT_SOMEWHERE: &str = r#"(?i)\bkeep(?:s|ing)?\s+(?:my\s+|the\s+|our\s+)?(?:keys|key|fob)?\s*(?:in|on|by|under|next to)\b"#;
pub(super) const DEFINITION_ASK: &str = concat!(
    r#"(?i)^\s*(?:what|whats|what's)\s+(?:is|are|does|do)\b|^\s*(?:what|whats)\s+does\b|\b(?:tell me about"#,
    r#"|explain|meaning of|difference between|definition of)\b"#,
);
pub(super) const GETTING_INTO: &str = concat!(
    r#"(?i)\b(?:want|wanted|would like|thinking of|thinking about|keen|hoping)\s+to\s+(?:start|begin"#,
    r#"|take\s+up|try|get\s+into|learn)\b|\b(?:get|getting)\s+into\b|\b(?:take|taking)\s+up\b"#,
    r#"|\bbeen\s+meaning\s+to\b"#,
);
pub(super) const SOCK: &str = r#"(?i)\bsocks?\b|\btwo\s+at\s+a\s+time\b"#;
pub(super) const JOIN_WOOL: &str = concat!(
    r#"(?i)\bjoin(?:ing)?\s+(?:in\s+)?(?:a\s+)?(?:new\s+)?(?:ball|wool|yarn|skein)\b"#,
    r#"|\bnew\s+ball\s+of\s+(?:wool|yarn)\b|\brun\s+out\s+of\s+(?:wool|yarn)\b"#,
    r#"|\bchange\s+(?:the\s+)?colou?r\b"#,
);
pub(super) const IN_THE_ROUND: &str = concat!(
    r#"(?i)\bin\s+the\s+round\b|\bcircular\s+(?:needles?|knitting)\b|\bround\s+needles?\b|\bmagic\s+loop\b"#,
    r#"|\bdouble[- ]pointed\b|\bdpn\b|\bknit\s+(?:a\s+)?(?:hat|jumper|sweater|blanket|sleeve|mittens?"#,
    r#"|gloves?|cushion|cardigan)\s+in\s+the\s+round\b"#,
);
pub(super) const SEWING_MACHINE: &str = r#"(?i)\bsewing\s+machine\b|\bmachine\s+sewing\b"#;
pub(super) const THREAD_NEEDLE: &str =
    r#"(?i)\bthread(?:ing)?\s+(?:a|the)\s+needle\b|\bneedle\s+threader\b"#;
pub(super) const POM_POM: &str = r#"(?i)\bpom\s?-?\s?poms?\b"#;
pub(super) const PATCH: &str = r#"(?i)\bpatch(?:es|ing|ed)?\b"#;
pub(super) const NEEDLE_SIZE: &str = concat!(
    r#"(?i)\bwhat\s+(?:size|thickness|kind of|sort of)\s+(?:needles?|yarn|wool|hook)\b|\b(?:needle"#,
    r#"|hook)\s+size\b|\bsize\s+(?:needles?|hook)\b|\bwhich\s+(?:needles?|yarn|wool|hook)\b"#,
    r#"|\bwhat\s+(?:needles?|yarn|wool)\b|\bhow\s+(?:thick|chunky|much)\s+(?:yarn|wool)\b"#,
);
pub(super) const PURL: &str = r#"(?i)\bpurl(?:s|wise|ed|ing)?\b"#;
pub(super) const LOOSE_STOOL: &str = concat!(
    r#"(?i)\b(?:stools|poos?|faeces|feces|bowels?)\b[^.?!]{0,24}?\b(?:loose|runny|watery|soft|sloppy)\b"#,
    r#"|\b(?:loose|runny|watery|sloppy)\s+(?:stools|poos?|faeces|feces)\b"#,
);
pub(super) const FURNITURE_WORDS: &str = concat!(
    r#"(?i)\b(?:chair|chairs|table|tables|bench|benches|desk|desks|leg|legs|glue|screw|screws|tighten|joint"#,
    r#"|joints|wood|wooden|seat|furniture|varnish|paint|repair|fix|wobbl\w*|rock\w*|rickety)\b"#,
);

pub(super) const CRAFT_KINDS: &[(&str, &str)] = &[
    ("knit", "knitting"),
    ("knitting", "knitting"),
    ("knitted", "knitting"),
    ("purl", "knitting"),
    ("wool", "knitting"),
    ("knitting needles", "knitting"),
    ("k2tog", "knitting"),
    ("kfb", "knitting"),
    ("crochet", "crochet"),
    ("crocheting", "crochet"),
    ("crochet hook", "crochet"),
    ("sewing", "sewing"),
    ("sew", "sewing"),
    ("dressmaking", "sewing"),
    ("fabric", "sewing"),
    ("paper pattern", "sewing"),
    ("seam allowance", "sewing"),
    ("tote bag", "sewing"),
];

pub(super) const KNITTING_MEANINGS: &[(&str, &str)] = &[
    (
        "k2tog",
        concat!(
            "k2tog means knit two stitches together as one. That loses a stitch and leans the work to",
            " the right, and it is how a jumper narrows at an armhole or a sleeve.",
        ),
    ),
    (
        "p2tog",
        concat!(
            "p2tog means purl two stitches together as one. It loses a stitch and, because it is ",
            "purled, the lean goes the other way from k2tog.",
        ),
    ),
    (
        "ssk",
        concat!(
            "ssk means slip one stitch knitwise, slip the next the same way, then knit both through ",
            "the back loops. It is a decrease that leans left, the mirror of k2tog.",
        ),
    ),
    (
        "kfb",
        concat!(
            "kfb means knit into the front of the stitch and then into the back of the same one ",
            "before slipping it off. It makes two stitches out of one with no hole, which is what a ",
            "thumb gusset or a raglan uses.",
        ),
    ),
    (
        "yarn over",
        concat!(
            "A yarn over (yo) is the yarn taken round the needle without working a stitch. It adds a ",
            "stitch and leaves the hole the pattern wants - an eyelet, a buttonhole or a lacy edge.",
        ),
    ),
    (
        "psso",
        concat!(
            "psso means pass the slipped stitch over: slip one, work the next, then lift the slipped ",
            "one over the stitch you just made. It loses a stitch.",
        ),
    ),
];

pub(super) const CAST_ON_REPLIES: &[&str] = &[
    concat!(
        "Cast on with the long tail, which is the one most patterns assume: pull out a tail about",
        " three times the width of the knitting, make a slip knot on the needle, then scoop the ",
        "thumb strand through the finger strand and put that loop on the needle, over and over ",
        "until you have the number the pattern asks for. Count them once at the end and, if you ",
        "are one out, it matters - a stitch lost here shows up as a lop-sided edge fifty rows ",
        "later.",
    ),
    concat!(
        "The long-tail cast on does nearly every job and gives a neat, stretchy edge: leave a ",
        "tail three times the width of what you are making, knot it on, and use your thumb and ",
        "finger to lift each new loop over the needle. The number you cast on is the pattern's ",
        "first number, so if it lists sizes take the one you circled.",
    ),
];

pub(super) const CROCHET_PATTERN_REPLIES: &[&str] = &[
    concat!(
        "Crochet reads a row at a time, but check whose stitch names the pattern uses before you ",
        "start: a US single crochet is a UK double crochet and a US double is a UK treble, and ",
        "most patterns online are American. Then 'ch 3, turn' starts the next row, brackets and ",
        "asterisks are a repeat worked as many times as the number after them says, and the ",
        "number in brackets at the end of a row is how many stitches you should have - count it, ",
        "because that is what tells you a stitch went missing ten rows earlier.",
    ),
    concat!(
        "Work down the page one row at a time and keep the count at the end of each row: the ",
        "pattern tells you how many stitches you should have, and that number is the only thing ",
        "that catches a mistake before it becomes a shape. Brackets and asterisks mean repeat, a ",
        "turning chain is the 'ch 1' or 'ch 3' at the start of a row, and if the stitch names ",
        "look wrong for the stitch you know, it is the country: US single crochet is UK double, ",
        "US double is UK treble.",
    ),
];

pub(super) const SEWING_PATTERN_REPLIES: &[&str] = &[
    concat!(
        "Circle your size on every piece before you cut anything: the pattern holds several sizes",
        " on the same lines, and the seam allowance is usually not included (1.5 cm, or 5/8 inch,",
        " is printed on the envelope). Lay the pieces out with the arrow on the straight grain, ",
        "match the notches, and fold the fabric wherever a piece says 'cut on the fold'. Read the",
        " instructions end to end once first - the order matters more than the sewing does.",
    ),
    concat!(
        "Read a paper pattern as a list of pieces and a list of steps, not as a diagram: the ",
        "notches and the grain arrow line the fabric up, the same line carries every size so you ",
        "trace yours with a pencil, and anything marked 'cut on the fold' is laid on a folded ",
        "edge. Check the envelope for whether the seam allowance is included - it usually is not ",
        "- and press each seam as you sew it, which is most of what makes the finished thing look",
        " right.",
    ),
];

pub(super) const KNIT_PATTERN_REPLIES: &[&str] = &[
    concat!(
        "Read a knitting pattern a line at a time: the number at the start is the row. k is knit,",
        " p is purl, k2tog knits two stitches together (a decrease), kfb knits into the front and",
        " back of one (an increase) and yo is a yarn over (a hole and an increase). Brackets or ",
        "asterisks are a repeat, worked as many times as the number after them says, and when a ",
        "line lists sizes the first number is the smallest - circle your own before you cast on. ",
        "Knit a 10 cm square in the pattern stitch first: if it comes out narrow, go up a needle ",
        "size.",
    ),
    concat!(
        "Start with the abbreviations key, then work a row at a time from the row number. Three ",
        "things trip people up: brackets with a number after them are a repeat, not a note; the ",
        "sizes are stacked in the same line with the smallest first, so ring yours in pencil; and",
        " the tension at the top of the pattern is the size check, so knit a 10 cm square and ",
        "change the needle rather than the pattern if it is out. k2tog loses a stitch, kfb and yo",
        " add one, and the stitch count at the end of a row should still match the pattern.",
    ),
];

pub(super) const SOCK_REPLIES: &[&str] = &[
    concat!(
        "A sock is the second project, not the first, and the reason is the heel: it is a turning",
        " point you knit back and forth on half the stitches, and nothing before it prepares you ",
        "for it - but it is four small steps in order, not one hard one. For an adult sock in ",
        "4-ply, cast on 60 stitches on 2.5 mm needles, join in the round without a twist, and ",
        "work 2x2 rib for 5 cm, then knit every round for 15 cm. Then the heel flap over 30 ",
        "stitches for about 30 rows, turn the heel, pick up the gusset, and decrease back to 60 ",
        "before the foot.",
    ),
    concat!(
        "Sock yarn wants nylon in it - about 75 per cent wool and 25 per cent nylon, sold as ",
        "4-ply or sock yarn - because pure wool wears through at the heel inside a season, and 60",
        " stitches on 2.5 mm needles is the usual adult size. Knit the leg in the round until it ",
        "is 15 cm, then the heel: half the stitches back and forth for 30 rows, turn the heel, ",
        "pick up the gusset, and decrease back to 60. Write the count down as you go, because a ",
        "sock is the one thing where losing count shows.",
    ),
];

pub(super) const JOIN_WOOL_REPLIES: &[&str] = &[
    concat!(
        "Join a new ball at the end of a row and leave 15 cm tails: knit three or four stitches ",
        "holding the old and new strands together, drop the old, and carry on. Then darn the two ",
        "tails into the back of the work at the end rather than knotting them - a knot works ",
        "loose in the wash and leaves a hard lump in the fabric. Same for a colour change: swap ",
        "the yarn at the start of a row, and the jog where the colours meet sits at the side seam",
        " if you plan it there.",
    ),
    concat!(
        "The join that does not show is a spit splice on pure wool: fray the two ends, overlap ",
        "them 5 cm, wet them and rub them between your palms until the fibres felt together. On ",
        "anything else, leave 15 cm tails, knit a few stitches with both strands, and weave the ",
        "tails in afterwards with a blunt needle. Never knot them - the knot is what comes undone",
        " and what you feel through the sole of a sock.",
    ),
];

pub(super) const ROUND_REPLIES: &[&str] = &[
    concat!(
        "Circular needles, one length that fits the job: 40 cm for a hat, 60 cm for a child's ",
        "jumper, 80 cm for an adult's. Cast on the whole round at once, check the cast-on edge is",
        " not twisted before you join, put a marker at the join and knit every stitch - no ",
        "turning - and you get stocking stitch without ever purling. A hat is 90 stitches on a 4 ",
        "mm needle, knit for 20 cm, then decrease 10 stitches every other round until 10 are left",
        " and pull the yarn through them.",
    ),
    concat!(
        "The join is the only tricky moment: the cast-on edge has to run straight round without a",
        " twist in it, or the whole piece is a Mobius and you find out 10 cm in. Lay it flat on ",
        "the table, line the bumps up along the inside, then knit the first stitch and put a ",
        "marker there. On a 40 cm needle a hat works in one piece; for a sleeve or a jumper neck ",
        "that is too small for the needle, pull a loop of cable out at the side - the magic loop ",
        "- rather than buying double-pointed needles.",
    ),
];

pub(super) const SEWING_MACHINE_REPLIES: &[&str] = &[
    concat!(
        "Thread it in this order and it will sew: spool, the arm guide, the tension dial, the ",
        "take-up lever, then the needle from front to back. Wind a bobbin, drop it in with the ",
        "thread trailing to the left, and turn the handwheel towards you with the needle down to ",
        "bring the bobbin thread up. Then sew on a scrap before the real thing: stitch length ",
        "about 2.5 mm, the presser foot down before the pedal, and let the machine pull the ",
        "fabric - pulling it bends the needle and skips stitches.",
    ),
    concat!(
        "The order is what matters: the spool, round the arm guide, through the tension dial, up ",
        "and through the take-up lever, down to the needle, threaded front to back. Then the ",
        "bobbin, wound and dropped in with its thread to the left, and one turn of the handwheel ",
        "brings the bobbin thread up through the plate. Set the stitch length to 2.5 mm, put the ",
        "foot down, and sew a line on scrap: if the stitches loop on the underside it is the top ",
        "tension, and if it puckers the stitch is too long.",
    ),
];

pub(super) const THREAD_NEEDLE_REPLIES: &[&str] = &[
    concat!(
        "Cut the thread at a slant rather than straight across, wet it and pinch it flat, then ",
        "push it through the eye against something white so you can see the hole. A needle ",
        "threader costs pennies and does it in a second. Knot the two ends together so you are ",
        "working with a double thread, and cut it to about a forearm's length - longer than that ",
        "tangles and snarls with every stitch.",
    ),
    concat!(
        "The thread goes through the eye far more easily if it is cut at an angle and pressed ",
        "flat between wet fingers, and a needle threader turns the whole thing into a two-second ",
        "job. Use a length about as long as your forearm, doubled and knotted at the end, because",
        " a long thread twists, knots and wears thin before it runs out.",
    ),
];

pub(super) const POM_POM_REPLIES: &[&str] = &[
    concat!(
        "Cut two circles of card the size you want, each with a hole in the middle the size of a ",
        "10p piece, and wind yarn round both together until the hole is filled. Cut the yarn ",
        "round the outside between the two circles, tie a long thread tight round the middle and ",
        "knot it twice, then take the card away and trim it round. Tie it hard - a loose tie is ",
        "the one thing that makes a pom pom fall apart.",
    ),
    concat!(
        "Two cardboard rings, a hole in the middle, yarn wound through the hole until it is full ",
        "and the middle is closed: then cut between the rings all the way round, slip a doubled ",
        "thread round the waist of the bundle, and pull it as tight as you can before you knot ",
        "it. Trim it while it is still on the card if you want a tidy ball, and use a plate or a ",
        "coin as the template so both rings match.",
    ),
];

pub(super) const PATCH_REPLIES: &[&str] = &[
    concat!(
        "Patch it from both sides: put a piece of similar-weight fabric or iron-on denim over the",
        " hole, press it with a hot iron through a damp cloth, then sew round the edge with a ",
        "straight or zigzag stitch so it cannot lift. On a knee, put the patch inside and stitch ",
        "the shape of the hole closed first - a patch on the outside of a knee rubs through in ",
        "weeks, and on the seat it looks like a repair from the bus.",
    ),
    concat!(
        "The fix is a patch as close to the weight of the denim as you can find, ironed on with a",
        " hot iron through a cloth and then stitched round the edge; the stitching is what ",
        "survives the washing. Put it inside the leg for a knee or a seat and outside for a rip ",
        "at the hem, and stitch the frayed edges flat first so the patch has something to hold.",
    ),
];

pub(super) const HAND_SEWING_REPLIES: &[&str] = &[
    concat!(
        "Hand-sewing is three stitches: running stitch, in and out in an even line; back stitch, ",
        "one back for every two forward, which is the one that holds a seam that takes a pull; ",
        "and slip stitch, worked inside a fold so it barely shows, which is what a hem wants. ",
        "Start and finish with two tiny back stitches on the spot instead of a knot, and keep the",
        " thread to a forearm's length.",
    ),
    concat!(
        "Learn running stitch first - in and out in an even line, which is gathering, basting and",
        " half the repairs you will ever do - then back stitch, which is the strong one and goes ",
        "one stitch back for every two forward. A hem is slip stitch, worked along the inside of ",
        "the fold. An iron between steps does more for how the finished thing looks than neat ",
        "stitches do.",
    ),
];

pub(super) const NEEDLE_SIZE_REPLIES: &[&str] = &[
    concat!(
        "Match the needle to the yarn and the pattern does the rest: 4 mm for double knit, 5 mm ",
        "for aran, 6 mm for chunky, and the ball band prints the size that suits it. For a scarf,",
        " 5 mm with aran and 30 stitches gives about 20 cm wide, and two 100 g balls make one ",
        "about 1.5 m long; a hat is 5 mm and about 80 stitches worked in the round.",
    ),
    concat!(
        "The ball band is the answer - it prints the needle size the yarn was made for, and 4 mm ",
        "for double knit, 5 mm for aran and 6 mm for chunky are the usual. Then the pattern's ",
        "tension square is the check: knit 10 cm, and change the needle rather than the pattern ",
        "if the square comes out the wrong size. Thirty stitches on 5 mm is a scarf about 20 cm ",
        "wide.",
    ),
];

pub(super) const CROCHET_START_REPLIES: &[&str] = &[
    concat!(
        "Start with a 4 mm hook and a smooth double knit: a slip knot, then chain 12, then one ",
        "double crochet into the second chain from the hook and into each chain after it, turning",
        " at the end of every row. That is a square, and the same stitch in rows is a blanket. ",
        "Check which country the pattern is from before you learn its names: a US double crochet ",
        "is a UK treble.",
    ),
    concat!(
        "A 4 mm hook, a smooth double knit and one stitch gets you going: chain 12, then double ",
        "crochet into the second chain and every one after it, turn, chain one, and work back ",
        "along the row. Count at the end of each row - the count is the only thing that catches a",
        " dropped stitch before it becomes a sloping edge.",
    ),
];

pub(super) const PURL_REPLIES: &[&str] = &[
    concat!(
        "A purl is a knit made with the yarn in front: keep the yarn towards you, put the right ",
        "needle into the front of the stitch from right to left, wrap the yarn over the needle ",
        "and pull the loop through to the back. Knit one and purl one and you have ribbing, which",
        " is what makes a cuff or a hat brim pull in and stay up.",
    ),
    concat!(
        "For a purl the yarn goes in front of the work, the needle goes into the front of the ",
        "stitch from right to left, and the new loop is brought through to the back. Ribbing - ",
        "knit one, purl one - is the reason to learn it: it stretches and pulls back, so it is ",
        "the cuff, the neckband and the bottom of a hat.",
    ),
];

pub(super) const KNIT_START_REPLIES: &[&str] = &[
    concat!(
        "Start with one square you can finish today: 5 mm needles and a smooth aran yarn, cast on",
        " about 30 stitches and knit every stitch of every row. That is garter stitch, it is flat",
        " and it does not curl, and 30 stitches is a scarf about 20 cm wide - two 100 g balls ",
        "make one about 1.5 m long. Your tension evens out after ten rows, so the first rows ",
        "looking uneven is not a mistake.",
    ),
    concat!(
        "The way in is the smallest finished thing: cast on 30 stitches on 5 mm needles, knit ",
        "every row until it looks like a scarf, and cast off. Knit every row is garter stitch and",
        " needs no pattern at all. When a row goes wrong, undo it stitch by stitch rather than ",
        "pulling it all out - a dropped stitch is easy to pick back up with a crochet hook.",
    ),
];

pub(super) const TAPE_REPLIES: &[&str] = &[
    concat!(
        "On a metric tape the big numbers are centimetres and the small marks between them are ",
        "millimetres, ten to the centimetre, so three marks past the 7 is 73 mm. The imperial ",
        "side runs the other way up: between two inches the longest small mark is the half, the ",
        "next longest the quarter, then eighths, then sixteenths. Lay the tape flat and read it ",
        "with your eye straight above the mark, and add the width of the hook if you are ",
        "measuring to an outside edge.",
    ),
    concat!(
        "The numbers on a tape are centimetres with millimetres marked between them, and the hook",
        " at the end is loose on purpose - about a millimetre of play, so it works hooked over an",
        " edge and butted against one. For anything inside a frame or a recess, measure from the ",
        "100 mm mark instead and take 100 mm off the reading, and the hook never matters. On a ",
        "ruler the zero is at the very end, so check a new ruler against a tape once: some are ",
        "trimmed a millimetre short.",
    ),
];

pub(super) const LASER_LEVEL_REPLIES: &[&str] = &[
    concat!(
        "A laser level throws a line or a dot along the wall instead of a bubble: stand it on ",
        "something solid, switch it on, and wait for the line to stop blinking before you trust ",
        "it - that blink is it levelling itself. Once it is steady, work to the line, and check ",
        "it by turning the unit round and seeing whether the line lands in the same place; a ",
        "laser that has been dropped can be a few millimetres out over a room.",
    ),
    concat!(
        "Set the laser on a tripod or a firm surface and give it a moment to settle - most ",
        "self-level within a few degrees, and the light flashes until they have. The line is your",
        " reference at whatever height you set it, so mark both ends of a shelf from the line ",
        "rather than measuring from one end, and if the room is bright, or the run is longer than",
        " about five metres, use the detector rather than your eye.",
    ),
];

pub(super) const BUBBLE_LEVEL_REPLIES: &[&str] = &[
    concat!(
        "The bubble sits between the two lines when the surface is level, and if it is touching a",
        " line the surface is off by about 2 mm over a metre. Check along the shelf and then ",
        "across its depth - a shelf can be level one way and tilted the other - and turn the ",
        "level end for end over the same spot: if the bubble moves, the level is out, not the ",
        "shelf. Use the longest one that fits, because a 600 mm level shows four times the gap a ",
        "150 mm one does on the same slope.",
    ),
    concat!(
        "Put the level on the surface and read the bubble: in the middle, between the two lines, ",
        "is level, and the further it drifts towards a line the steeper the surface. Then test ",
        "the other way across the same spot, and swap the level's ends to check the tool itself -",
        " a dropped level is out by more than the shelf is. A shelf wants to be within a couple ",
        "of millimetres over its length, so mark the fixings from the level rather than trusting ",
        "the wall.",
    ),
];
