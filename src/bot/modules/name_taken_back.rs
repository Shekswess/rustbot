//! Clear a denied name before the proposals and ask for its replacement.
use crate::bot::{Memory, Reading, compose, memory::names};

pub(crate) fn observe(reading: &Reading<'_>, memory: &mut Memory) {
    if let Some(known) = &memory.name
        && names::takes_back(reading.raw, known)
    {
        memory.denied_name = memory.name.take().map(|name| (name, reading.clean.clone()));
    }
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let (name, note) = memory.denied_name.as_ref()?;
    if *note != reading.clean {
        return None;
    }
    let pool = [
        format!("Sorry — I had you as {name}. What should I call you?"),
        format!("My mistake on {name}. What should I call you?"),
        format!("Sorry about that — {name} was wrong. What do you go by?"),
        format!("Ah, sorry. Not {name}, then — what should I call you?"),
    ];
    Some(compose::pick(&pool.each_ref().map(String::as_str), memory, "").into())
}
