//! Patterns from PyBot money cards at the pinned revision.
pub(super) const A_MONTH_NAMED: &str = "\\b(?:this|next|every|each|a|the) month\\b|\\bmonthly\\b";
pub(super) const A_SUM_NAMED: &str =
    "£|\\b\\d+\\b|\\bpounds?\\b|\\bquid\\b|\\btenner\\b|\\bfiver\\b";
pub(super) const A_THING_TO_COVER: &str = "(?i)\\b(?:food|foods|feed|feeding|eat|eating|dinner|tea|supper|lunch|breakfast|meals?|groceries|grocery|shop|shopping|supermarket|kids?|children|child|family|families|baby|nappies|nappy|formula|rent|bills?|electric|electricity|gas|heating|oil|coal|wood|petrol|diesel|fares?|travel|bus|train|school|uniform|clothes|nursery|childcare|medicines?|prescription)\\b";
pub(super) const A_SPEND_ON_A_THING: &str =
    "\\bto spend\\b|\\bspend (?:it )?on\\b|\\bspending on\\b|\\bspend on\\b";
pub(super) const A_FOOD_WORD: &str = "\\b(?:food|eat|eating|dinner|dinners|tea|lunch|breakfast|supper|meal|meals|cook|cooking|feed|groceries|supermarket|shop|shopping|eat|trolley|staples)\\b";
pub(super) const AN_OCCASION_OR_A_TREAT: &str = "\\bbirthdays?\\b|\\bpresent\\b|\\bpresents\\b|\\bgift\\b|\\bgifts\\b|\\bchristmas\\b|\\btreat\\b|\\bsomething nice\\b|\\bsomething special\\b|\\ba day out\\b|\\bday out\\b|\\btake (?:my|the|her|him|them|us) \\w+ out\\b|\\bout for (?:a|an) (?:meal|drink|drinks|coffee|lunch|dinner)\\b|\\banniversary\\b|\\bhen do\\b|\\bstag do\\b";
pub(super) const A_BIRTHDAY: &str = "\\bbirthdays?\\b";
pub(super) const A_THIN_WEEK: &str = "\\bno money\\b|\\bnothing (?:much|left|at all)\\b|\\bnot much\\b|\\bbroke\\b|\\bskint\\b|\\btight\\b|\\bcheap\\b|\\bbudget\\b|\\bcan'?t afford\\b|\\bcant afford\\b|\\blittle\\b";
pub(super) const ASKS_FOR_SOMETHING_TO_DO: &str = "\\b(?:this weekend|at the weekend|next weekend|weekend|week off|day off|time off|holiday|bored|something to do|anything to do|nothing to do|things to do|keep (?:the )?kids|entertain|fun|days out|day out)\\b";
pub(super) const ASKS_WHAT_TO_COOK: &str = "\\bwhat (?:can|could|should|shall|do) i (?:cook|make|eat|have)|\\bwhat to (?:cook|make|eat)|\\bmeal ideas?\\b|\\bwhats for (?:dinner|tea|supper)\\b";
pub(super) const SAYS_NOTHING_AT_ALL: &str = "\\bno money\\b|\\bnothing to eat\\b|\\bnothing for food\\b|\\bno food\\b|\\bcan'?t afford\\b|\\bcant afford\\b|\\bnothing at all\\b|\\bnothing left\\b";
pub(super) const A_TRIP: &str = "\\bholiday|\\bholidays\\b|\\babroad\\b|\\bflights?\\b|\\bhotel\\b|\\b(?:week|weekend|days?)\\s+away\\b|\\bin the sun\\b|\\b(?:in|for|during|by|next|early|late)\\s+(?:january|february|march|april|may|june|july|august|september|october|november|december)\\b";
pub(super) const A_STATED_BUDGET: &str =
    "\\b(?:my|our|the)\\s+(?:food\\s+)?budget\\b|\\bbudget for food\\b|\\bfood budget\\b";
pub(super) const ASKS_THE_PRICE: &str = "\\bhow much (?:does|do|is|are|would|will)\\b|\\bhow much should i (?:spend|budget|pay|set aside)\\b|\\bhow much (?:a|per|each)\\b|\\bcost of\\b|\\bcosts?\\b|\\bwhat does (?:\\w+ ){0,4}cost\\b|\\bhow expensive\\b|\\bprice of\\b|\\baverage (?:cost|spend|bill|shop)\\b|\\bwhat'?s the average\\b|\\bhow much do (?:you|people)\\b|\\b(?:good|normal|typical|reasonable|sensible)\\s+(?:\\w+\\s+)?(?:budget|spend|amount to spend)\\b|\\ba (?:food\\s+)?budget (?:for|of)\\b|\\bwhat should i (?:spend|budget|set aside)\\b";
pub(super) const ASKS_HOW_LONG: &str = "\\b(?:last|make it go|get through|get us to|til|until|till|skint|left|cheap|no money|stretch)\\b";
pub(super) const NO_MONEY: &str = "\\bno money\\b|\\bnot got any money\\b|\\bhav(?:e|ing) n(?:o|ot any) (?:money|cash)\\b|\\bnothing (?:left|at all|to my name)\\b|\\bnowt\\b|\\bnada\\b|\\bzero\\b|\\b£0\\b|\\b0 pounds\\b|\\bnothing coming in\\b|\\bdidnt get paid\\b|\\bdidn't get paid\\b|\\bbenefits? (?:hasnt|hasn't|havent|haven't) (?:come|been paid)\\b|\\bcant afford (?:any )?food\\b|\\bcan't afford (?:any )?food\\b|\\bcan't afford to eat\\b|\\bcant afford to eat\\b|\\bno food in the (?:house|cupboard)\\b|\\bnothing to eat\\b|\\bnot eating (?:today|tonight)\\b|\\bskip(?:ping)? meals?\\b";
pub(super) const THE_FIGURE: &str = "(?i)£\\s*([0-9][0-9,]*(?:\\.[0-9]+)?)|\\b([0-9][0-9,]*(?:\\.[0-9]+)?)\\s*(?:pounds?|quid|bucks)\\b";
pub(super) const A_MEAL: &str =
    "\\b(?:dinner|dinners|tea|lunch|supper|breakfast|cook|cooking|make|meal|meals|eat|eating)\\b";
pub(super) const A_HEADCOUNT: &str = "\\b(?:feed|for|cook for|serves?)\\s+(\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b|\\b(\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+(?:people|of us|adults|mouths)\\b|\\bfamily of\\s+(\\d{1,2}|one|two|three|four|five|six|seven|eight)";
pub(super) const A_THE_HOUSEHOLD: &str = "\\bfamily\\b|\\bcouple\\b|\\b(?:for|feed(?:ing)?|between)\\s+(?:two|three|four|five|six|seven|eight|\\d+)\\b|\\btwo of us\\b|\\bthe kids\\b|\\bwe (?:are|'re) a\\b|\\bhousehold\\b|\\bpartner\\b|\\bhusband\\b|\\bwife\\b|\\b(?:feed|feeding) us\\b";
pub(super) const DEBT_DUE: &str = "\\b(cant pay|can't pay|cannot pay|cant make|can't make|missed|miss|missing|forgot|forgotten|paid late|pay late|paying late|late payment|payment is late|didnt pay|didn't pay|not paid|unpaid|behind on|overdue|maxed|maxed out|payment plan|interest freeze|default|defaulted|in arrears|struggling with|can't afford|cant afford|dont pay|don't pay)\\b";
pub(super) const A_DEBT: &str = "\\b(card|credit card|store card|overdraft|debt|debts|loan|loans|payment|payments|balance|interest|minimum|payday loan|buy now pay later|klarna|clearpay)\\b";
pub(super) const SERVICING_A_DEBT: &str = "\\b(pay|pays|paying|paid|minimum|balance|interest|statement|direct debit|standing order|repayment)\\b";
pub(super) const DAY_NAMED: &str = "(?i)\\b(today|tonight|tomorrow|monday|tuesday|wednesday|thursday|friday|saturday|sunday|the weekend|next week|a week|the week|this week|the end of the week|end of the week|two weeks|a fortnight|the fortnight|payday|the end of the month|end of the month|next friday)\\b";
pub(super) const A_WEEK_DAY_NAMED: &str = "(?i)\\b(?:today|tonight|tomorrow|monday|tuesday|wednesday|thursday|friday|saturday|sunday|payday)\\b";
pub(super) const TWO_OF_US: &str = "(?i)\\b(?:us|we|weve|we're|were|both of us|the two of us|couple|me and my partner|me and my wife|me and my husband|me and my boyfriend|me and my girlfriend|our two)\\b";
pub(super) const A_HOUSEHOLD: &str = "(?i)\\b(?:our|ours|family|household|kids|children|child|partner|wife|husband|boyfriend|girlfriend|son|daughter|baby|toddler|mum|dad|mother|father|parents|flatmates?|housemates?)\\b";
pub(super) const A_STATED_SIZE: &str =
    "(?i)\\b(?:family|household|party|group)\\s+of\\s+(\\d+|two|three|four|five|six)\\b";
pub(super) const ASKING_FOR_AN_OVERDRAFT: &[&str] = &[
    "ask my bank",
    "ask the bank",
    "ask our bank",
    "ask about an overdraft",
    "about an overdraft",
    "for an overdraft",
    "get an overdraft",
    "getting an overdraft",
    "open an overdraft",
    "set up an overdraft",
    "arrange an overdraft",
    "apply for an overdraft",
    "an overdraft application",
    "overdraft limit",
    "increase my overdraft",
    "increase the overdraft",
    "raise my overdraft",
    "bigger overdraft",
    "larger overdraft",
    "overdraft with bad credit",
];
pub(super) const SPENT_UP: &[&str] = &[
    "spent up",
    "spent it all",
    "spent all my money",
    "spent all of it",
    "spent all my wages",
    "spent my wages",
    "spent every penny",
    "spent my pay",
    "spent all my pay",
    "spent my money",
];
