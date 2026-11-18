//! Fill a fault name or a relation into a source-authored answer.

use std::borrow::Cow;

pub(super) fn about_person(
    words: &[String],
    template: &str,
    male: &[&str],
    female: &[&str],
) -> String {
    let (subject, object, owned, reflexive) = words
        .iter()
        .find_map(|word| {
            if male.contains(&word.as_str()) {
                Some(("he", "him", "his", "himself"))
            } else if female.contains(&word.as_str()) {
                Some(("she", "her", "her", "herself"))
            } else {
                None
            }
        })
        .unwrap_or(("they", "them", "their", "themselves"));
    let plural = subject == "they";
    let mut reply = template.to_owned();
    for (place, word) in [
        ("{they}", subject),
        ("{them}", object),
        ("{their}", owned),
        ("{self}", reflexive),
        ("{has}", if plural { "have" } else { "has" }),
        ("{is}", if plural { "are" } else { "is" }),
        ("{likes}", if plural { "like" } else { "likes" }),
        ("{loves}", if plural { "love" } else { "loves" }),
    ] {
        reply = reply.replace(place, word);
    }
    let mut chars = reply.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
        None => reply,
    }
}

pub(super) fn name_fault(
    line: &str,
    template: &'static str,
    faults: &[(&str, &str)],
    default: &'static str,
) -> Cow<'static, str> {
    let inside_word = |c: char| c.is_alphanumeric() || c == '_';
    for &(label, fault) in faults {
        if line.match_indices(label).any(|(start, matched)| {
            !line[..start].ends_with(inside_word)
                && !line[start + matched.len()..].starts_with(inside_word)
        }) {
            return Cow::Owned(template.replace("{fault}", fault));
        }
    }
    Cow::Borrowed(default)
}
