//! Pasta families, shape-specific timings, and dish pairings.

use std::borrow::Cow;

use crate::bot::Reading;

use super::{PASTA_FOR_DISH, PASTA_KINDS_REPLY, PASTA_MINUTES, PASTA_NOODLES_REPLY};

#[derive(Clone, Copy)]
pub(super) enum Ask {
    Timing,
    Kinds,
    ForDish,
}

pub(super) fn answer(ask: Ask, reading: &Reading<'_>) -> Cow<'static, str> {
    match ask {
        Ask::Kinds => Cow::Borrowed(PASTA_KINDS_REPLY),
        Ask::Timing => Cow::Owned(timing(reading)),
        Ask::ForDish => for_dish(reading),
    }
}

fn timing(reading: &Reading<'_>) -> String {
    let words = reading.line.words.join(" ").to_lowercase();
    let minutes: Cow<'_, str> = if words.split_whitespace().any(|word| word == "fresh") {
        "Fresh pasta is 2 to 3 minutes, not the packet time: it is done the moment it floats and loses its chalky middle, so taste a strand at 2 minutes.".into()
    } else if words.contains("noodle") {
        PASTA_NOODLES_REPLY.into()
    } else if let Some((_, minutes)) = PASTA_MINUTES
        .iter()
        .find(|(aliases, _)| aliases.iter().any(|alias| words.contains(alias)))
    {
        format!("About {minutes} for that one.").into()
    } else {
        "Dried pasta is 8 to 12 minutes, depending on the shape: 8 to 9 for thin spaghetti.".into()
    };
    format!(
        "{minutes} Use a big pan with about 10g of salt per litre, keep a mug of the water for the sauce, and taste a piece a minute before the packet says: shapes differ, and so do pans."
    )
}

fn for_dish(reading: &Reading<'_>) -> Cow<'static, str> {
    let words = if reading.line.words.is_empty() {
        reading.line.clean.to_lowercase()
    } else {
        reading.line.words.join(" ").to_lowercase()
    };
    if let Some((_, shapes, why)) = PASTA_FOR_DISH
        .iter()
        .find(|(aliases, _, _)| aliases.iter().any(|alias| words.contains(alias)))
    {
        return format!("{shapes}, because {why}.").into();
    }
    "Match the shape to the sauce rather than to the packet: a thick meat sauce wants a wide strand or a tube, a thin one wants a long strand, and a bake wants penne or rigatoni.".into()
}
