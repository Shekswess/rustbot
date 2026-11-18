//! Authored readers for day-out plans assembled in the conversation.

pub(super) const PLAN_ASK: &str = r#"(?:whats? (?:the |our )?plan|what is the plan|the plan for (?:the|our|this) (?:day|trip|weekend|evening|visit)|what (?:should|shall|do) we do (?:first|next|then)|where (?:do|should) we start|whats? the order|how (?:should|do|shall) we plan|plan (?:out )?the (?:day|trip|weekend)|sort (?:out )?the plan)(?=\s*(?:$|[.?!]|(?:then|now|today|tonight|tomorrow|weekend)\b|for (?:the|this|our|next|\w+day)\b|on \w+day\b))"#;
pub(super) const PLAN_PLACE: &str = r#"\b(?:gig|gigs|venue|concert|show|match|game|festival|tour|theatre|cinema|museum|exhibition|station|train|flight|airport|coach|trip|day out|weekend away|night out)\b"#;
pub(super) const PLAN_MEAL: &str = r#"\b(?:eat|eats|eating|food|meal|dinner|lunch|breakfast|restaurant|table|bite|pub|pizza|curry)\b"#;
pub(super) const PLAN_BED: &str = r#"\b(?:stay over|stay the night|staying over|hotel|hotels|bed|room|sleep|sleeping|airbnb|hostel)\b"#;
pub(super) const PLAN_BOOKED: &str =
    r#"\b(?:booked|reserved|sorted|got a table|have a table|already got)\b"#;
pub(super) const PLAN_TOGETHER: &str =
    r#"\b(?:\d+|two|three|four|five|six|seven|eight|several|few|both) of (?:us|them|you)\b"#;
pub(super) const PLAN_DOORS: &str = r#"\b(?:gig|gigs|venue|concert|show|match|game|festival|tour|theatre|cinema|museum|exhibition)\b"#;
pub(super) const PLAN_QUESTION: &str = r#"^(?:so |ok |okay |and |but |then )*(?:what|why|how|when|where|who|which)\b|^(?:so |ok |okay |and |but |then )*(?:can|could|should|shall|do|does|did|is|are|was|were|will|would|have|has|am|any)\s+(?:i|you|we|they|he|she|it|there|this|that|one)\b"#;

pub(super) const PLAN_EMPTY: &str = r#"
hi hello hey ok okay cool nice great thanks thank you yes yeah no nope sure right ah oh hmm lol haha
good fine maybe alright
"#;
