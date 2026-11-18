//! Contract, workplace safety and expenses responses for the pinned work cards.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Reading, understanding::line::body_word};

use super::{cleaned, said_text, uses};

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(
    BODY_PART_THAT_HURTS,
    r"\b(?:my|his|her|their|our|the)\s+(?:(?:left|right|lower|upper|whole|other)\s+)?(\w+)\s+(?:is\s+|are\s+|feels\s+|feel\s+|has\s+been\s+|have\s+been\s+|been\s+)?(?:really\s+|properly\s+|absolutely\s+|so\s+|very\s+|killing\s+)?(?:hurts|hurting|hurt|aches|aching|ache|sore|stiff|killing|kills|throbbing|throbs|agony|numb|numbness|twinge|twinges)\b"
);
pattern!(
    AN_INCIDENT,
    r"\b(?:hurt\s+myself|injured|injury|accident|hazard|slipped|slipping|fell|fall|fainted|spilled|no\s+gloves|no\s+training|guard\s+off|near\s+miss|report)\b"
);
pattern!(
    WORK_IS_ADVICE,
    r"\b(?:that|it|this|those|they|which)\s+(?:work|works|working|worked)\b|\bsame\s+(?:work|works)\b"
);
pattern!(
    NOTICE_CAN_THEY,
    r"\b(?:can they|can my|can i be|do i have to|must i|make me|force me|stop me|they make|wont let me|won't let me|allowed|legal)\b"
);
pattern!(NOTICE_HOW_MUCH, r"\bhow (?:much|long|many)\b");
pattern!(NOTICE_GARDEN, r"\bgarden(?:ing)? leave\b");
pattern!(
    NOTICE_PAID_INSTEAD,
    r"\b(?:pay(?:ment)? in lieu|paid instead|pay me instead|pay instead|paid off|instead of working|instead of serving|buy (?:myself|me) out)\b"
);
pattern!(
    SHIFT_PERMISSION,
    r"\b(?:without asking|without telling|without a word|without notice|do that|get away with|allowed to|allowed|legal|lawful|is that ok|is that right)\b"
);

const CLAIMING_WORDS: &[&str] = &[
    "claim",
    "claims",
    "claimed",
    "claiming",
    "expense",
    "expenses",
    "receipt",
    "receipts",
    "reimburse",
    "reimbursed",
    "reimbursement",
    "mileage",
    "allowance",
    "tax",
    "payroll",
    "submit",
    "how much can i",
    "back",
    "policy",
    "deadline",
    "refund",
    "refunded",
];
const WEEKEND_AT_WORK: &[&str] = &[
    "work",
    "working",
    "worked",
    "works",
    "job",
    "shift",
    "shifts",
    "rota",
    "rotas",
    "on the rota",
    "contract",
    "employer",
    "boss",
    "manager",
    "colleague",
    "overtime",
    "lieu",
    "payslip",
    "union",
    "acas",
    "paid",
    "pay",
    "double time",
    "on call",
    "my hours",
    "leave",
    "holiday",
    "day off",
    "days off",
    "weekend off",
    "weekends off",
    "saturday off",
    "saturdays off",
    "sundays off",
    "every other weekend",
    "refuse",
    "refusing",
    "say no",
    "turn it down",
    "turned down",
    "asked",
    "asking",
    "asks",
    "make me",
    "make us",
    "made to",
    "they want",
];
const VIOLENT_AT_WORK: &[&str] = &[
    "threat",
    "threats",
    "threaten",
    "threatens",
    "threatened",
    "threatening",
    "violence",
    "violent",
    "attack",
    "attacked",
    "attacking",
    "assault",
    "assaulted",
    "punch",
    "punched",
    "hit",
    "hits",
    "hurt",
    "hurts",
    "kill",
    "killed",
    "stab",
    "stabbed",
    "knife",
    "weapon",
    "grabbed",
    "grabs",
    "shoved",
    "shoves",
    "kicked",
    "kicks",
];
const ROTA_WORDS: &[&str] = &[
    "shift",
    "shifts",
    "rota",
    "schedule",
    "timesheet",
    "hours",
    "onto nights",
    "onto days",
    "day off",
    "days off",
    "rest day",
    "rest days",
    "annual leave",
    "holiday",
    "holidays",
    "my leave",
    "booked leave",
];
const ROTA_CHANGE_WORDS: &[&str] = &[
    "keeps changing",
    "keep changing",
    "short notice",
    "last minute",
    "last-minute",
    "keeps moving",
    "keeps swapping",
    "unfair",
    "boss",
    "manager",
    "contract",
    "cancelled",
    "canceled",
    "cancelling",
    "canceling",
    "cancels",
    "cancel",
    "keeps cancelling",
    "keep cancelling",
    "getting cancelled",
    "called in",
    "sent home",
    "rota cut",
    "refuse",
    "say no",
    "on the day",
    "every week",
    "every fortnight",
    "cant plan",
    "can't plan",
    "plan anything",
    "never know",
    "no idea when",
    "changed",
    "changing",
    "changes",
    "moved",
    "swapped",
    "switched",
    "rotated",
];
const FEELING_WORDS: &[&str] = &[
    "feel",
    "feels",
    "feeling",
    "felt",
    "miserable",
    "fed up",
    "sick of",
    "stressed",
    "exhausted",
    "burnt out",
    "burned out",
    "resentful",
    "unfair to me",
    "always the one",
    "who bends",
    "cant cope",
    "can't cope",
    "had enough",
];
const DAYS_OFF: &[&str] = &[
    "day off",
    "days off",
    "dayoff",
    "called in",
    "called into work",
    "call me in",
    "called me in",
    "shift cancelled",
    "shifts cancelled",
    "shift canceled",
    "rota cancelled",
    "cancelled my shift",
    "cancelled my days",
    "cancel my days",
    "cancelling my days",
    "cancelling my shift",
    "cancels my days",
    "cancels my shift",
    "getting cancelled",
    "keep getting cancelled",
];
const LEAVE: &[&str] = &[
    "annual leave",
    "holiday",
    "my leave",
    "time off",
    "booked leave",
    "leave i booked",
    "holidays",
];
const TRAVEL: &[&str] = &[
    "flight",
    "flights",
    "airline",
    "plane",
    "holiday booking",
    "hotel",
    "package",
    "abroad",
    "airport",
    "train",
    "ferry",
    "cruise",
    "booking reference",
];

// https://www.gov.uk/tax-relief-for-employees/vehicles-you-use-for-work
const EXPENSES: &str = "Check your employer's policy for what it will reimburse, its mileage rate and the deadline. Keep the receipts and record the date, reason, amount and business miles. If the employer pays less than HMRC's approved mileage rate, you may be eligible for UK tax relief on the difference. For 2026 to 2027 the approved rates are 55p for the first 10,000 business miles in your own car or van, then 25p, 24p for a motorcycle and 20p for a bicycle. Check the HMRC rules for the tax year and expense you want to claim.";

// https://www.hse.gov.uk/contact/tell-us-about-a-health-and-safety-issue.htm
// https://www.acas.org.uk/health-and-safety-at-work/reporting-a-risk
const SAFETY_REPORT: &str = "If there is serious and imminent danger, get to safety and alert the people at risk if you can do so safely. Tell your manager or safety representative promptly and put the date, place and facts in writing. If anyone was hurt, ask for it to be recorded in the accident book or equivalent system. If the concern is unresolved or serious, contact the relevant regulator. In Great Britain, HSE can direct you to the right enforcing authority. If you face action for raising a safety concern or leaving a dangerous area, seek advice from your union or Acas.";

// https://www.gov.uk/rest-breaks-work
// https://www.gov.uk/sunday-working
const WEEKEND: &str = "Check your contract and written rota for what you agreed about weekends, changes and overtime. In Great Britain most adult workers are entitled to 11 hours' daily rest and either 24 hours off each week or 48 hours off each fortnight. Different rules can apply to some jobs and shifts. Eligible shop and betting shop workers can opt out of Sundays. If the employer is asking for a change, tell them what you can do and ask them to confirm the agreement in writing.";

macro_rules! bullying {
    ($opening:literal) => {
        concat!(
            $opening,
            "Write it down while it is fresh: date, what was said or done, who saw it. Then tell someone with authority - your manager if it is a colleague, their manager or HR if it is your manager - in writing, asking for a meeting and what you want to change. Keep the tone flat and factual. Your contract and your workplace's grievance policy set out the steps; a free advice line like ACAS or Citizens Advice will tell you where you stand, and you do not have to sort it out before you report it."
        )
    };
}

// https://www.acas.org.uk/notice-periods/notice-when-resigning
// https://www.acas.org.uk/notice-periods/when-an-employee-is-not-required-to-work-their-notice
macro_rules! notice {
    ($opening:literal) => {
        concat!(
            $opening,
            "In Great Britain, an employee resigning after at least a month's service usually owes at least one week's notice. The employer's minimum for dismissal is usually one week after a month, then one week per completed year from two years' service up to twelve weeks. Check your contract for longer notice and ask for any agreement to leave early in writing. You may be able to take unused holiday during notice."
        )
    };
}

pub(super) fn passes(id: &str, text: &str) -> bool {
    match id {
        "expenses" => uses(&cleaned(text), CLAIMING_WORDS),
        "health-safety-report" => {
            let lower = text.to_lowercase();
            AN_INCIDENT.is_match(&lower)
                || !BODY_PART_THAT_HURTS
                    .captures_iter(&lower)
                    .any(|hit| body_word(&hit[1]))
        }
        "weekend-working" => {
            !WORK_IS_ADVICE.is_match(&text.to_lowercase()) && uses(&cleaned(text), WEEKEND_AT_WORK)
        }
        _ => false,
    }
}

pub(super) fn reply(id: &str, reading: &Reading<'_>) -> Option<&'static str> {
    match id {
        "expenses" => Some(EXPENSES),
        "health-safety-report" => Some(SAFETY_REPORT),
        "weekend-working" => Some(WEEKEND),
        "bullying-at-work" => {
            let line = cleaned(said_text(reading));
            Some(if uses(&line, VIOLENT_AT_WORK) {
                bullying!(
                    "If someone could hurt you now, move somewhere safe and call 999 in the UK. Report threats of violence to the police as well as your employer. "
                )
            } else {
                bullying!("")
            })
        }
        "notice-period" => Some(notice_period(&said_text(reading).to_lowercase())),
        "shifts-changed" => shifts_changed(reading),
        _ => None,
    }
}

fn notice_period(line: &str) -> &'static str {
    if NOTICE_GARDEN.is_match(line) {
        notice!(
            "On garden leave you remain employed during some or all of your notice period and normally keep receiving your pay and contractual benefits. "
        )
    } else if NOTICE_PAID_INSTEAD.is_match(line) {
        notice!(
            "Check for a payment in lieu of notice clause. It may allow your employer to end employment straight away and pay you for the notice period. You can also ask whether your employer will agree to that. "
        )
    } else if NOTICE_CAN_THEY.is_match(line) && !NOTICE_HOW_MUCH.is_match(line) {
        notice!(
            "You normally have to work the notice you owe under your contract unless you and your employer agree otherwise, or another exception applies. "
        )
    } else {
        notice!("Start with your contract or written statement to find your notice period. ")
    }
}

// https://www.acas.org.uk/changing-an-employment-contract/advice-for-employees/if-your-employer-proposes-employment-contract-changes
// https://www.gov.uk/holiday-entitlement-rights/booking-time-off
fn shifts_changed(reading: &Reading<'_>) -> Option<&'static str> {
    let text = reading.line.clean.to_lowercase();
    let said = cleaned(if reading.raw.is_empty() {
        &text
    } else {
        reading.raw
    });
    if !uses(&said, ROTA_WORDS) && !uses(&said, ROTA_CHANGE_WORDS) && uses(&said, FEELING_WORDS) {
        return None;
    }
    let cleaned = cleaned(&text);
    if uses(&cleaned, DAYS_OFF) {
        return Some(
            "Check your contract for guaranteed hours, days off and the employer's notice requirements. Ask in writing why the shift or day off was cancelled and whether the hours will be paid. Keep a dated record. If it keeps happening, speak to your union or Acas before refusing a shift.",
        );
    }
    if uses(&cleaned, LEAVE) {
        return (!uses(&cleaned, TRAVEL)).then_some("For booked annual leave, check your contract first. In Great Britain the default is notice at least as long as the leave being cancelled plus one day. A contract can set a different notice period. Your employer still has to let you take your statutory leave. Ask for the reason and revised dates in writing, and keep the booking and rota.");
    }
    if SHIFT_PERMISSION.is_match(&text) {
        return Some(
            "It depends on your contract and the change. Changing a contractual shift pattern usually needs agreement unless a relevant flexibility clause applies. The employer should consult you, use it reasonably and give reasonable notice. Ask for the change and its basis in writing. If it keeps happening, speak to a union representative or Acas.",
        );
    }
    Some(
        "Ask for the rota a week ahead in writing and check what your contract says about changes. Then say it once, plainly, about the next change rather than the last one: \"Thursday I was told on the day - can we agree changes are confirmed 48 hours ahead?\" If it keeps happening, put it in an email and ask HR what the pattern is meant to be. Your union or Acas can explain where you stand.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn employment_cards_keep_their_subject_and_return_current_guidance() {
        assert!(!passes(
            "health-safety-report",
            "My lower back aches at work."
        ));
        assert!(passes(
            "health-safety-report",
            "My lower back aches after I slipped at work."
        ));
        assert!(!passes("weekend-working", "Does that work at weekends?"));
        assert!(!passes(
            "expenses",
            "Is my train ticket valid on any train?"
        ));
        let notice = reply("notice-period", &Reading::new("What is garden leave?")).unwrap();
        assert!(notice.starts_with("On garden leave"));
        let leave = reply(
            "shifts-changed",
            &Reading::new("My boss has cancelled my annual leave."),
        )
        .unwrap();
        assert!(leave.contains("plus one day"));
        assert!(
            reply(
                "shifts-changed",
                &Reading::new("My airline cancelled my holiday flight.")
            )
            .is_none()
        );
        assert!(
            reply(
                "bullying-at-work",
                &Reading::new("My manager threatened me.")
            )
            .is_some_and(|reply| reply.contains("call 999"))
        );
    }
}
