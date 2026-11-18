//! Garden-tool selection, maintenance and safe operation.

use std::borrow::Cow;

use crate::bot::Reading;

use super::{any, clean_or_typed, garden_words};

// https://www.hse.gov.uk/work-equipment-machinery/maintenance.htm
// https://www.hse.gov.uk/treework/safety-topics/chainsaw-operator.htm
pub(in super::super) fn advise(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = garden_words(clean_or_typed(reading));
    let says = |choices| any(&line, choices);
    let hedge = says(&[
        "hedge",
        "hedges",
        "hedge trimmer",
        "hedge cutters",
        "topiary",
    ]);
    let strimmer = says(&[
        "strimmer",
        "strimmers",
        "grass trimmer",
        "lawn edge",
        "edge my lawn",
    ]) || (says(&["edge", "edges", "edging"])
        && says(&[
            "lawn", "lawns", "grass", "path", "driveway", "kerb", "border",
        ]));
    let blower = says(&[
        "leaf blower",
        "leaf blowers",
        "leafblower",
        "blow leaves",
        "blow the leaves",
    ]);
    let saw = says(&[
        "chainsaw",
        "chainsaws",
        "chain saw",
        "fell a tree",
        "cut logs",
        "cut a log",
    ]);
    let tiller = says(&[
        "rotavator",
        "rotovator",
        "tiller",
        "cultivator",
        "dig over",
        "break up the soil",
    ]);
    let reply = if strimmer
        && says(&[
            "new line",
            "change the line",
            "change line",
            "spool",
            "wont feed",
            "not feeding",
            "snaps",
            "snapping",
            "snapped",
        ]) {
        "Isolate the power and wait for the head to stop. Follow the manual for the correct line size, winding direction and length, or fit the recommended replacement spool. Refit the guard and verify the spool is secure before use."
    } else if saw
        && says(&[
            "chain keeps coming off",
            "chain coming off",
            "chain comes off",
            "chain fell off",
            "chain wont stay",
            "chain came off",
        ])
    {
        "Stop using a chainsaw if the chain comes off. With the tool isolated, check the manufacturer's instructions for chain fit and tension. Arrange a service if the chain, bar, sprocket or tensioner may be damaged or the problem recurs."
    } else if says(&[
        "wont start",
        "won t start",
        "not starting",
        "not start",
        "stalls",
        "stalling",
        "fires then dies",
        "cuts out",
    ]) {
        "Check the manual for your exact model's starting and troubleshooting procedure. Check the approved fuel or battery, the controls and any accessible filter as instructed, with the tool stopped and isolated. Stop if fuel leaks, the cable is damaged or the cutting head jams, and arrange a repair."
    } else if says(&[
        "store", "storing", "stored", "storage", "put away", "winter", "shed", "garage", "rust",
        "rusty",
    ]) {
        if saw {
            "Turn it off and isolate it before maintenance. Clean and store it with the bar cover fitted, dry and secured away from children. Follow the manufacturer's instructions for fuel, chain oil and batteries during storage."
        } else {
            "Switch off and isolate the tool, clean it as instructed and refit the blade or head cover where provided. Store it dry and secure. Follow the manual for petrol handling and battery storage."
        }
    } else if says(&[
        "wet grass",
        "wet lawn",
        "damp grass",
        "raining",
        "in the rain",
    ]) {
        "Wait for dry conditions, especially with mains-powered tools. Wet material can block the head and make footing slippery. Isolate the power before clearing any jam."
    } else if says(&[
        "guard",
        "guards",
        "goggles",
        "ear defenders",
        "protection",
        "gloves",
        "safe",
        "safety",
        "ppe",
    ]) {
        if saw {
            "A chainsaw requires training for the job and suitable chainsaw-rated leg and foot protection, eye or face and hearing protection, and other equipment specified for the work. Ordinary tough trousers are insufficient. Use a trained professional if you lack the experience for the cut."
        } else {
            "Keep the manufacturer's guard fitted, use the eye and hearing protection specified for the model, and wear suitable footwear. Keep other people and pets outside the tool's operating area. For corded tools, keep the lead away from the cutting head."
        }
    } else if says(&[
        "how do i start",
        "how to start",
        "choke",
        "from cold",
        "get it started",
    ]) {
        "Follow the exact model's starting procedure. Check that guards are fitted and the cutting area is clear. If it is a chainsaw, get training for the tool and task before operating it."
    } else if says(&[
        "sharpen",
        "sharpening",
        "sharp",
        "blunt",
        "blades",
        "blade",
        "oil",
        "care",
        "maintenance",
        "not cutting",
        "wont cut",
        "won t cut",
        "tears",
        "jammed",
    ]) {
        "Switch the tool off and disconnect its energy source before touching the blade or cutting head. Clear debris and inspect parts as directed by the manual. Use manufacturer-specified maintenance or have a repairer sharpen or replace worn parts. Refit the guard before use."
    } else if says(&[
        "which",
        "should i buy",
        "buy",
        "buying",
        "should i get",
        "worth it",
        "do i need",
        "petrol or",
        "battery or",
        "corded or",
    ]) {
        "Choose for the work, weight, run time and power source you can handle safely. Consider a hand tool for a small job, battery power where a lead would get in the way, and check the maker's rated capacity. For chainsaw work, decide first whether a trained operator is needed."
    } else if says(&[
        "how do i use",
        "how to use",
        "how do i cut",
        "how to cut",
        "cut the hedge",
        "trim the hedge",
        "edging",
        "edge my lawn",
        "edge the lawn",
        "cutting my plants",
        "cuts the plants",
    ]) {
        if saw {
            "Get hands-on chainsaw training for the work and follow the model's instructions. Use a trained professional for tree felling, work at height or timber under tension. A short written description cannot establish a safe cutting plan."
        } else if strimmer {
            "Keep the guard fitted and use the tool's edging position if the manual allows it. Keep the spinning head away from stones, plants, feet and cables, and keep other people and pets clear. Use hand edging shears beside delicate plants."
        } else if hedge {
            "Trim from stable ground and cut only branches within the model's rated capacity. Keep both hands on the handles, avoid overreaching, and keep the lead and bystanders clear. Use loppers for thicker branches once the trimmer is stopped."
        } else if blower {
            "Work with the wind and collect the leaves in a pile. Keep debris away from people, roads and drains. A rake may work better on wet leaves."
        } else if tiller {
            "Clear stones, wire and debris before starting. Work from stable ground at the depth and speed specified by the manual and stay clear of the tines. Stop and isolate the machine before clearing anything caught in them."
        } else {
            return None;
        }
    } else if says(&[
        "not working",
        "keeps coming off",
        "keeps falling",
        "keeps breaking",
        "stopped working",
        "rattles",
    ]) {
        "Stop and isolate the power. Check accessible parts and any guards according to the manual. Seek a repair if a moving part is loose or damaged or if the fault persists."
    } else {
        let tools = [
            (
                strimmer,
                "A strimmer uses a spinning line to trim grass and lawn edges.",
            ),
            (
                hedge,
                "A hedge trimmer cuts hedge sides and tops with a reciprocating blade.",
            ),
            (
                blower,
                "A leaf blower moves loose leaves with a stream of air.",
            ),
            (
                saw,
                "A chainsaw uses a moving chain on a guide bar to cut timber.",
            ),
            (tiller, "A rotavator uses powered tines to break up soil."),
        ];
        let descriptions = tools
            .into_iter()
            .filter_map(|(named, reply)| named.then_some(reply))
            .collect::<Vec<_>>();
        if descriptions.is_empty() {
            return None;
        }
        return Some(Cow::Owned(descriptions.join(" ")));
    };
    Some(Cow::Borrowed(reply))
}
