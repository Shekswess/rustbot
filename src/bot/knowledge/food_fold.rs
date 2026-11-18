//! The pantry's singular forms preserve the vocabulary of the pinned cooking rules.

pub(crate) fn fold(word: &str) -> String {
    let lower = word.to_lowercase().replace('’', "'");
    let word = lower.trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"']);
    let length = word.chars().count();
    if length > 4
        && let Some(stem) = word.strip_suffix("ies")
    {
        return format!("{stem}y");
    }
    if let Some(stem) = word.strip_suffix("es")
        && ((length > 4 && stem.ends_with('o'))
            || (length > 3 && stem.ends_with(['s', 'x', 'z', 'h'])))
    {
        return stem.to_owned();
    }
    if length > 2
        && !word.ends_with("ss")
        && let Some(stem) = word.strip_suffix('s')
    {
        return stem.to_owned();
    }
    word.to_owned()
}
