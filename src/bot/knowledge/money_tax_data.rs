//! Selection patterns from the pinned money cards.
pub(super) const ABOUT_THE_WIN: &str = "(?i)\\b(?:about|over)\\s+it\\b|\\bto\\s+(?:tell|let)\\s+me\\b|\\bto\\s+say\\b|\\bconfirming\\b|\\bwell\\s+done\\b|\\bcame\\s+after\\b|\\bafterwards\\b|\\bthe\\s+details\\b|\\bto\\s+let\\s+us\\s+know\\b";
pub(super) const ASKS_FOR_SOMETHING_BACK: &str = "(?i)\\b(?:fee|fees|pay|paid|send|transfer|wire|release|admin|processing|clearance|customs|bank details|account details|card details|sort code|final notice|get in touch|contact us|text them|text back|call them|ring them|reply|respond|confirm your)\\b|£";
pub(super) const A_WIN_NAMED: &str =
    "(?i)\\b(?:won|win|winner|winnings|prize|prizes|jackpot|lottery|competition|sweepstake)\\b";
pub(super) const MONEY_LEFT_NAMED: &str =
    "(?i)\\b(?:(?:inherit\\w*|windfall|unclaimed|bequest|next of kin|beneficiary))\\b";
pub(super) const NEVER_ENTERED: &str = "(?i)\\b(?:never|did not|didnt|didn't|do not|dont|don't)\\s+(?:enter|entered|apply|applied|sign up|signed up)\\b";
pub(super) const MONEY_OUT: &str = "(?i)\\b(?:pay|paid|paying|send|sending|wire|transfer|hand over)\\b(?:(?!\\b(?:it|them)\\b).){0,40}?\\b(?:fee|fees|charge|charges|clearance|deposit|bank details|account details|card details|sort code)\\b|\\b(?:fee|fees|charge|charges|clearance|release|held|withheld|deposit|processing|admin|bank details|account details|card details|sort code|upfront|up front)\\b|£";
pub(super) const TAX_ASKED: &str = "(?i)\\b(?:how much|how many|what|whats|what's|when|why|which|is|are|does|do|do i|can i|should i|explain|threshold|rate|rules)\\b[^.?!]{0,40}\\btax\\b";
pub(super) const BANK_DETAILS_ASKED: &str = "(?i)\\b(?:bank details|account details|account number|sort code|card details|bank account|iban|swift)\\b";
pub(super) const A_WIN_OR_AN_INHERITANCE: &str = "(?i)\\b(?:(?:won|win|winner|winnings|prize|prizes|jackpot|lottery|competition|sweepstake)|inherit\\w*)\\b";
pub(super) const THE_ESTATE_BEING_WOUND_UP: &str = "(?i)\\b(?:solicitor|solicitors|probate|executor|executors|executrix|conveyancer|grant of probate|letters of administration|the estate|my late|my (?:father|mother|dad|mum|husband|wife|partner|uncle|aunt|gran|grandad|nan) (?:has )?died|died (?:last|in)\\b)";
pub(super) const MY_PAY: &str = "(?i)\\b(?:i|we)(?:'m|m|'re|re|'ve|ve)?\\s+(?:earn|earns|make|makes|get|gets|take|bring|on|am)\\b|\\b(?:my|our)\\s+(?:salary|wage|wages|pay|income)\\b";
pub(super) const GIFT_FROM_WORK: &[&str] = &[
    " from work ",
    " at work ",
    " from my job ",
    " from the job ",
    " my employer ",
    " employer gave",
    " employers gave",
    " company gave",
    " my company ",
    " from the company",
    " my boss gave",
    " from my boss",
    " payroll ",
    " work gave me",
    " work gave us",
    " works christmas",
    " staff gift",
    " christmas bonus",
    " from head office",
];
pub(super) const GIFT_ARRIVES: &[&str] = &[
    " from my ",
    " from our ",
    " from a parent",
    " from parents",
    " from his ",
    " from her ",
    " parents give",
    " parent gives",
    " parents gave",
    " give me ",
    " gave me ",
    " gives me ",
    " given to me",
    " gave us ",
    " given to us",
    " gifts me",
    " gifted me",
    " being given",
    " i receive",
    " receiving",
    " i get ",
    " i got ",
    " left me ",
    " in my name",
];
