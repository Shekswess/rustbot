/// Return the value of an ASCII or Unicode decimal digit.
pub(crate) fn decimal_digit(character: char) -> Option<u8> {
    use regex::Regex;
    use std::sync::LazyLock;

    if let Some(digit) = character.to_digit(10) {
        return Some(digit as u8);
    }
    static DIGIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").expect("decimal digit"));
    let is_digit =
        |code| char::from_u32(code).is_some_and(|one| DIGIT.is_match(one.encode_utf8(&mut [0; 4])));
    let code = character as u32;
    if !is_digit(code) {
        return None;
    }
    // Unicode decimal digits form consecutive sets of ten beginning with zero.
    let run_start = (0..code).rev().find(|&previous| !is_digit(previous))? + 1;
    Some(((code - run_start) % 10) as u8)
}

// Python str.split()/strip() include these four ASCII information separators.
pub(crate) fn is_space(ch: char) -> bool {
    matches!(ch, '\u{0009}'..='\u{000d}' | '\u{001c}'..='\u{0020}' | '\u{0085}' |
        '\u{00a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' |
        '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}')
}

/// Collapse whitespace for the CLI's one-reply-per-line contract.
pub fn one_line(reply: &str) -> String {
    let mut words = reply.split(is_space).filter(|word| !word.is_empty());
    let Some(first) = words.next() else {
        return "Hello!".into();
    };
    let mut result = String::with_capacity(reply.len());
    result.push_str(first);
    for word in words {
        result.push(' ');
        result.push_str(word);
    }
    result
}
