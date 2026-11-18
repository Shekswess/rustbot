//! Indexed follow-ups on a named or recently announced dish.

use super::*;

const COOKED_SALMON: &str =
    "The salmon is cooked when the flesh is opaque and flakes easily with a fork.";

pub(in crate::bot::knowledge) fn named_salmon_doneness(
    reading: &Reading<'_>,
) -> Option<&'static str> {
    let text = &reading.line.clean;
    if !text.contains("salmon") {
        return None;
    }
    let words = tokens(text);
    (named(&words, text).is_some_and(|plan| plan.label == "salmon")
        && question(text, &words) == Some("done")
        && !matches(&SAUCE, text))
    .then_some(COOKED_SALMON)
}

fn matches(pattern: &Fancy, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

pub(in crate::bot::knowledge) fn has_rest_reply(
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> bool {
    let line = &reading.line;
    let text = if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    };
    let words = tokens(text);
    question(text, &words) == Some("rest")
        && named(&words, text)
            .or_else(|| plan_in_chat(&words, text, earlier))
            .is_some_and(|plan| !plan.rest.is_empty())
}

fn first_minutes(text: &str) -> Option<u32> {
    TIME_IN_ANSWER
        .captures_iter(text)
        .flatten()
        .find_map(|hit| {
            let start = hit.get(0)?.start();
            let head = text[..start].chars().rev().take(30).collect::<Vec<_>>();
            let head = head.into_iter().rev().collect::<String>().to_lowercase();
            if KEEPING_LEAD.iter().any(|lead| head.contains(lead)) {
                return None;
            }
            let value = hit[1].parse::<u32>().ok()?;
            Some(value * if hit[2].starts_with("hour") { 60 } else { 1 })
        })
}

fn holding_raw(text: &str, earlier: &[Exchange]) -> bool {
    std::iter::once(text)
        .chain(earlier.iter().rev().take(3).map(|turn| turn.user.as_str()))
        .any(|line| {
            tokens(line)
                .iter()
                .any(|word| RAW_WORDS.iter().any(|name| same(word, name)))
        })
}

fn keeping_question(text: &str) -> bool {
    matches(&KEEP_QUESTION, text) && !KEEP_BUT_COOKING.iter().any(|cue| text.contains(cue))
}

fn names_frosting(text: &str) -> bool {
    ["icing", "frosting", "buttercream", "ganache"]
        .into_iter()
        .any(|part| literal(text, part))
}

fn fit_plan_part(text: &str, last: &str, reply: &'static str) -> Option<Cow<'static, str>> {
    if !PART_BACKREF.iter().any(|phrase| literal(text, phrase)) {
        return None;
    }
    let part = PART_WORDS
        .iter()
        .filter(|part| literal(text, part) && !literal(last, part))
        .min()?;
    let mut letters = reply.chars();
    let first = letters.next()?.to_lowercase();
    Some(Cow::Owned(format!(
        "Same for the {part}: {first}{}",
        letters.as_str()
    )))
}

fn storage(plan: &Plan, original: &'static str) -> Cow<'static, str> {
    if !matches!(
        plan.label,
        "roast chicken"
            | "chicken thigh"
            | "chicken breast"
            | "sausages"
            | "curry"
            | "shepherd's pie"
            | "roast vegetables"
            | "spag bol"
            | "lasagne"
            | "beef stew"
            | "chilli con carne"
            | "fish pie"
            | "pasta bake"
            | "sunday roast"
    ) || !["3 to 4 days", "3 days", "4 days"]
        .iter()
        .any(|duration| original.contains(duration))
    {
        return Cow::Borrowed(original);
    }
    if plan.label == "sausages" {
        return Cow::Borrowed(
            "Cool cooked sausages and put them in the fridge within 2 hours. Eat them within 48 hours or freeze them. For raw sausages, follow the pack's storage instructions and use-by date.",
        );
    }
    let text = original
        .replace("3 to 4 days", "up to 48 hours")
        .replace("3 days", "up to 48 hours")
        .replace("4 days", "up to 48 hours");
    let refrigeration = if text.contains("within an hour") {
        ""
    } else {
        "Cool cooked leftovers and put them in the fridge within 2 hours. "
    };
    let reheat = if text.contains("reheat at ")
        || text.contains("reheats at ")
        || text.contains("reheat covered")
        || text.contains("reheats covered")
        || text.contains("reheat it gently")
    {
        " Reheat only once and make sure it is steaming hot throughout."
    } else {
        ""
    };
    Cow::Owned(format!("{refrigeration}{text}{reheat}"))
}

pub(in crate::bot::knowledge) fn reply(
    reading: &Reading<'_>,
    earlier: &[Exchange],
    memory: Option<&Memory>,
) -> Option<Cow<'static, str>> {
    let line = &reading.line;
    let text = if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    };
    let words = tokens(text);
    let direct = named(&words, text);
    if direct.is_none()
        && [&PERSON_STATE, &PERSON_REST, &BODY_TEMPERATURE]
            .into_iter()
            .any(|pattern| matches(pattern, text))
    {
        return None;
    }
    if matches(&SAUCE, text) {
        return None;
    }
    let plan = direct.or_else(|| plan_in_chat(&words, text, earlier));
    let mut kind = question(text, &words);
    if !PART_WORDS.iter().any(|part| literal(text, part)) && keeping_question(text) {
        kind = Some("keep");
    }
    if kind == Some("keep") && holding_raw(text, earlier) {
        return None;
    }
    if let (Some(plan), Some(kind)) = (plan, kind) {
        if kind == "time"
            && direct.is_none()
            && let Some(previous) = earlier.last().and_then(|turn| first_minutes(&turn.bot))
            && let Some(now) = first_minutes(plan.time)
            && previous != 0
            && now != 0
            && previous.max(now) > 2 * previous.min(now)
        {
            return None;
        }
        if kind == "frosting" && PART_NOT.iter().any(|phrase| literal(text, phrase)) {
            return None;
        }
        let selected = match kind {
            "time" => plan.time,
            "temp" => plan.temp,
            "done" if plan.label == "salmon" => COOKED_SALMON,
            "done" => plan.done,
            "keep" => plan.keep,
            "ahead" if plan.ahead.is_empty() => plan.keep,
            "ahead" => plan.ahead,
            "rest" => plan.rest,
            "skin" => plan.skin,
            "stuffing" => plan.stuffing,
            "frosting" => plan.frosting,
            "fill" => plan.fill,
            _ => "",
        };
        if !selected.is_empty() {
            if matches!(kind, "keep" | "ahead") {
                return Some(storage(plan, selected));
            }
            if let Some(last) = earlier.last().filter(|turn| turn.bot == selected)
                && let Some(reply) = fit_plan_part(text, &last.user, selected)
            {
                return Some(reply);
            }
            return Some(Cow::Borrowed(selected));
        }
    }
    if kind == Some("done")
        && let Some(memory) = memory
    {
        return doneness_reply::respond(reading, memory).map(Cow::Borrowed);
    }
    if plan.is_none()
        && names_frosting(text)
        && !PART_NOT.iter().any(|phrase| literal(text, phrase))
        && IMPLIED_ASKS.iter().any(|phrase| literal(text, phrase))
    {
        let cake = PLANS.iter().find(|plan| plan.label == "cake")?;
        return Some(Cow::Borrowed(cake.frosting));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_dish_followups_have_the_original_parts_and_approved_safety() {
        let prior = [Exchange {
            user: "i'm doing a roast chicken tonight".to_owned(),
            bot: String::new(),
            module: "answers",
        }];
        let hold = reply(&Reading::new("and how long does it keep?"), &prior, None).unwrap();
        assert!(hold.contains("up to 48 hours in the fridge"), "{hold}");
        let done = reply(&Reading::new("is my salmon done?"), &[], None).unwrap();
        assert_eq!(
            done,
            "The salmon is cooked when the flesh is opaque and flakes easily with a fork."
        );
        assert!(reply(&Reading::new("i have a temperature"), &prior, None).is_none());
        assert_eq!(
            reply(&Reading::new("how long does bread keep"), &[], None).as_deref(),
            Some("It keeps 3 days in a paper bag; slice and freeze what you won't eat.")
        );
    }
}
