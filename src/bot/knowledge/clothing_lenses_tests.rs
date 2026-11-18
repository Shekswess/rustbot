//! Lens predicates and two-turn exclusions from PyBot 7d63c7f.
use super::*;

type Case = (
    &'static [&'static str],
    &'static str,
    bool,
    Option<&'static str>,
);

#[test]
fn clothing_lens_branches_match_the_reference() {
    let cases: &[Case] = &[
        (&[], "My glasses are scratched.", true, Some(EXPECTED_LENS)),
        (
            &[],
            "My sunglasses have a scratch.",
            true,
            Some(EXPECTED_LENS),
        ),
        (&[], "Will it scratch the glass?", true, Some(EXPECTED_LENS)),
        (&[], "Can I polish my oven door glass?", false, None),
        (&[], "Can worn spectacles scratch a window?", false, None),
        (
            &[],
            "Do I have scratches on my lenses?",
            true,
            Some(EXPECTED_LENS),
        ),
        (&[], "Will the bicarb scrath the glass?", false, None),
        (&[], "My pint glass is scratched.", false, None),
        (&[], "Are there scratches on my glass table?", false, None),
        (&[], "What about the windscreen scratches?", false, None),
        (
            &[],
            "Would a cloth scratch my sunglasses on the shower shelf?",
            false,
            None,
        ),
        (
            &[],
            "Can I clean scratches on glass?",
            true,
            Some(EXPECTED_LENS),
        ),
        (
            &["Our oven door looks greasy."],
            "Will it scratch the glass?",
            true,
            None,
        ),
        (
            &["Our oven door looks greasy.", "I found a cloth."],
            "My spectacles are scratched.",
            true,
            None,
        ),
        (
            &["Our oven door looks greasy.", "I found a cloth.", "Thanks."],
            "My spectacles are scratched.",
            true,
            Some(EXPECTED_LENS),
        ),
        (
            &["I dropped my phone.", "There is a crack."],
            "Would it scratch the glass?",
            true,
            Some(EXPECTED_LENS),
        ),
        (
            &["We had wine out of a tumbler."],
            "My sunglasses are scratched.",
            true,
            Some(EXPECTED_LENS),
        ),
        (&["Car MOT tomorrow."], "Can I clean my lenses?", true, None),
        (
            &["The mirror was dusty!"],
            "Will it scratch the glass?",
            true,
            None,
        ),
        (
            &["There is a new \"SHOWER\"."],
            "What about scratches on my lenses?",
            true,
            None,
        ),
    ];
    for &(before, line, admitted, expected) in cases {
        let prior: Vec<_> = before
            .iter()
            .map(|user| Exchange {
                user: (*user).into(),
                bot: "All right.".into(),
                module: "greeting",
            })
            .collect();
        let reading = Reading::new(line);
        assert_eq!(not_a_pane(&reading), admitted, "{line:?}");
        assert_eq!(
            scratch(&reading, &prior).as_deref(),
            expected,
            "{line:?} after {before:?}"
        );
    }
}

const EXPECTED_LENS: &str = "A scratch in the coating can't be polished out, and paper towels and tissues are what make most of them: wash the lenses with water and washing-up liquid, then dry them with a microfibre cloth. An optician can fit new lenses into your own frame.";
