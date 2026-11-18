//! Route high-risk DIY requests to an appropriate trade (a_job_that_is_not_a_diy_job.py).
use crate::bot::{Memory, Reading, compose};
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

const TRADES: &[(&str, &str, &str)] = &[
    (
        r"\b(?:chimney breast|(?:remove|take|knock|cut) (?:the |out the )?chimney (?:breast|stack))\b",
        "a builder with a structural engineer",
        "whether it is carrying anything above it is an engineer's call, and that call decides how the rest of the job is done",
    ),
    (
        r"\b(?:chimney|flaunching)\b",
        "a chimney sweep",
        "it is done from a scaffold above the roofline, and the fall is the part that goes wrong",
    ),
    (
        r"\b(?:gas (?:hob|fire|oven|cooker|pipe|pipes|pipework|supply|valve|appliance|boiler|meter)|lpg|calor gas)\b",
        "a Gas Safe registered engineer",
        "on gas the work belongs to somebody on the register, and the number is one anyone can look up before they start",
    ),
    (
        r"\b(?:fuse *box|fuse board|consumer unit|rewire|rewiring|ring main|socket circuit|extra socket|new socket|the wiring|the electrics|electric shower|shower unit)\b",
        "an electrician",
        "the mistake that matters sits behind the plaster and starts a fire where nobody is looking",
    ),
    (
        r"\b(?:(?:remove|removing|knock through|knock out|take out|taking out|widen|open up|put a door in) (?:a |the |this |that |our )?(?:walls?|load bearing wall|wall between|partition)|(?:take|taking|knock|knocking|cut|cutting|rip|ripping) (?:a |the |this |that )?(?:walls?|partition) (?:out|down)|load *bearing|load-bearing|(?:holding|holds) (?:the )?(?:house|roof|floor|ceiling) up|holding anything up|structural)\b",
        "a builder with a structural engineer",
        "whether a wall is holding the floor up above it is decided by an engineer, not by how it looks once the plaster is off",
    ),
    (
        r"\b(?:(?:fell|felling|cut down|cutting down|take down|taking down|pollard|pollarding|coppice|prune|pruning|trim) (?:a |the |my |this |that |our )?(?:trees?|conifers?|poplar|willow|oak|sycamore|leylandii)|tree surgery|stump grinding|grind the stump)\b",
        "a tree surgeon",
        "a tree goes where it leans once the last cut is made, and a bad one takes a fence, a roof or a person with it",
    ),
    (
        r"\b(?:wasps? nest|hornets? nest|(?:wasps|hornets|bees) in the (?:loft|roof|wall|cavity|chimney)|nest of (?:wasps|hornets))\b",
        "a pest controller",
        "it is a suit and a long reach, and the ones that sting come at you when it goes wrong",
    ),
    (
        r"\b(?:asbestos|artex|old lagging)\b",
        "a licensed asbestos remover",
        "it has to be identified before anyone cuts into it, and the test is cheap next to guessing",
    ),
    (
        r"\b(?:move|moving|remove|removing|replace|replacing|swap|swapping) (?:the |a |my |this |that |one of the )?(?:leaking |old |broken |rusty |stuck |noisy )?(?:radiators?|rads?)\b",
        "a plumber",
        "a radiator comes off wet and goes back on needing the system filled and bled, and a joint that weeps under the floor costs more than the fitting",
    ),
    (
        r"\b(?:roof|roofing|flashing|re-?felt|re-?roof|ridge tiles?)\b|\b(?:fit|fitting|replace|replacing|repair|repairing|fix|fixing|mend|mending|install|installing|leak|leaking|broken) (?:around )?(?:the |a |my )?(?:broken |cracked |old |damaged |stuck |leaking )?(?:skylight|velux|roof window)\b",
        "a roofer",
        "it is scaffold and two people, and the water from a job that fails costs more than the job",
    ),
];

static JOB: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(TRADES.iter().map(|x| x.0)).unwrap());
static SELF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(?:how (?:do|can|could|would|should) (?:i|we|you)|how to|what'?s the (?:best|right) way to|is it (?:easy|hard|difficult|safe|worth it) to|(?:is|would) (?:this|that|it) (?:a|an) (?:job|thing) (?:for me|i can)|can (?:i|we) (?:just )?(?:do|fix|fit|replace|remove|take|knock|cut|fell|wire|rewire|re-?felt|repoint|install|cap|climb|get up|dig|sand|scrape|strip|mend|move|swap|widen|plaster|prune|pollard|bleed|drain|have a go|try)|should (?:i|we) (?:do|try|have a go|tackle)|myself|my own|diy|ourselves|have a go|have a bash|a go at it|a go at that)\b").unwrap()
});
static OUTSOURCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(?:find|hire|hiring|recommend|recommended|looking for|need a|need an|someone|somebody|anyone|anybody|quotes?|prices?|costs?|how much|near me|local|book|booked|call out|callout|directory|reviews|christmas|xmas|family tree|decision tree|tree diagram)\b").unwrap()
});
static NOW: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(?:smell|smells|smelling|leaking|leak|leaks|hissing|sparking|sparks|smoke|smoking|on fire|emergency|right now|just now|tonight|burst|flooding|collapsed|falling down)\b").unwrap()
});
static HAZARD_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(a|an|the|my|our|your|his|her|their|this|that|these|those) (?:leaking|leaky|smoking|smoky|hissing|sparking|burst|collapsed|flooding|blocked) ([a-z])").unwrap()
});
static UNPLUGGED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:lamps?|bedside|plugs?|flex|lamp holder|bayonet|ceiling rose)\b").unwrap()
});
static WIRING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(?:fuse box|fuse board|consumer unit|the wiring|the electrics|socket circuit|ring main|electric shower|shower unit|shed|garage|extension|loft)\b").unwrap()
});

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let text = &reading.clean;
    if OUTSOURCE.is_match(text)
        || !SELF.is_match(text)
        || NOW.is_match(&HAZARD_NAME.replace_all(text, "${1} ${2}"))
        || UNPLUGGED.is_match(text) && !WIRING.is_match(text)
    {
        return None;
    }
    let job = JOB.matches(text).into_iter().next()?;
    let (_, who, why) = TRADES[job];
    let title = format!("{}{}", who[..1].to_uppercase(), &who[1..]);
    let mut options = [
        format!(
            "That is a job for {who}, not a ladder and an afternoon: {why}. Two quotes from people near you sets the price, and the second one usually says something the first did not."
        ),
        format!(
            "{title} is who takes this one on: {why}. Ring two near you - a quote costs nothing, and it tells you whether this is a small job or a bigger one than it looks."
        ),
    ];
    let extra = match who {
        "a Gas Safe registered engineer" => " Ask for the Gas Safe number before you book.",
        "an electrician" => " Ask which competent-person scheme they are with before you book.",
        _ => "",
    };
    for option in &mut options {
        option.push_str(extra);
    }
    let last = memory.turns.last().map(|turn| turn.bot.as_str());
    let mut valid = options
        .into_iter()
        .filter(|option| Some(option.as_str()) != last)
        .collect::<Vec<_>>();
    let chosen = compose::draw(&mut memory.rng.borrow_mut().0, valid.len());
    Some(valid.swap_remove(chosen))
}
