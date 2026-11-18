//! Authored readers and reply pools for references to the previous subject.

pub(super) const LEANING: &str = r#"(?i)^(?:(?:and|also|so|but|ok|okay|then|plus)\b|\W*(?:what|how|where|when|who|which|is|are|do|does|can|could|should|shall|will|would|have|has|did)\b)"#;
pub(super) const A_METHOD: &str = r#"(?i)^\W*(?:how\s+(?:do|can|could|should|would|shall)\s+(?:i|you|we)\b|how\s+to\b|(?:what|whats|what'?s)\s+the\s+(?:best|easiest|quickest|right)\s+way\b|the\s+(?:best|easiest|quickest|right)\s+way\s+to\b)"#;
pub(super) const BAGGAGE: &str = r#"(?i)\b(?:baggage|luggage|hand luggage|cabin bag|carry[- ]on|hold bag|checked bag|holdall|suitcase|allowance|weight limit|kilos?|kilograms?|oversize|oversized|extra bag|bag drop|legroom)\b"#;

pub(super) const ASKING: &str = r#"
a about add adds again all also am an and any anyone anything anywhere are arent as ask asks at
back be been being big bit bits boil boils book books both bring brings but buy buys by call
calls can cant change changes check checks clean cleans come comes cook cooks could couldnt cut
cuts did didnt do does doesnt don done down drink drinks each early eat eats either else
elsewhere ever everything far fast few fix fixes for freeze freezes from gave get gets give
gives go goes got had has have he heavy help helps her him how however i if in into is isnt it
itn its just keep keeps know knows late least less ll long lot lots make makes many may me might
more most much must my near need needed needs never no nobody none not nothing of off often ok
okay old on one ones only or order orders other our out over pay pays piece pieces please pour
pours put puts quick re renew renews ring rings run runs save saves say says send sends shall
share shares she should shouldnt so some someone something somewhere soon start starts still
stop stops store stores t take takes tell tells than thanks that the their them then there these
they this those to too tries try under up us use uses ve wait waits walk walks was wash washes
wasnt way ways we wear wears were what whatever when whenever where whether which whichever who
whom whose why will with wont work works would wouldnt yes you your
"#;

pub(super) const PLACE_POOLS: &[(&str, &[&str])] = &[
    (
        r#"(?i)\bmy (?:account|balance|bill|statement|invoice|order|parcel|delivery|meter|direct debit|tariff|contract|policy|claim|application|booking|reservation|ticket)s?\b|\bmy (?:bank|energy|water|broadband|mobile|phone) (?:account|bill|app)\b"#,
        &[
            r#"That is in your own account rather than anywhere I can see, sorry - your bill or your order there has the figure to the penny, and I'll help you work out what to do with it."#,
            r#"I can't see your account, sorry. Open the app or the last bill: the figure there is the one that counts, and tell me what it says and I'll help you work out where it came from."#,
            r#"Your own account has that, sorry - it is the only place with your numbers in it. What it says is worth reading against the last one, and I'll help you spot what moved."#,
        ],
    ),
    (
        r#"(?i)\bcouncil\b|\bpermit\b|\blicen[cs]e\b|\bplanning permission\b|\bbin (?:day|collection)|\brecycling\b|\bcouncil tax\b|\bparking (?:permit|zone)\b|\bblue badge\b|\bpassport\b|\bdriving licence\b|\bDVLA\b|\bregister(?:ing)? to vote\b"#,
        &[
            r#"Rules like that are the council's or the department's own, sorry, and they differ by where you live - the page for your postcode has the current one, and I'll help you read it."#,
            r#"That one is set by the council or the department, so I can't say it from here, sorry. The letter or the site with your address on it is the one that counts; bring me what it says and I'll help you fill it in."#,
            r#"I can't call that one: it is their rule, not mine, and it changes. The council's own page (or the number on the letter) will say, and I'll help with the rest of it."#,
        ],
    ),
    (
        r#"(?i)\bwarrant(?:y|ies)\b|\bguarantee\b|\bunder guarantee\b|\breturn it\b|\brefund\b|\b(?:the|a|this|that) shop\b[^?]{0,30}\b(?:replace|replacement|refund|return|repair|fix)\b|\bstill cover(?:ed)?\b|\bout of warranty\b"#,
        &[
            r#"That is the maker's terms rather than mine, sorry - the date on the receipt is what they go by, and their warranty page says what is covered and for how long. Tell me what it says and I'll help you work out whether to claim or replace."#,
            r#"I can't see the terms from here, sorry. The receipt has the date and the maker's own page has the cover; together they answer it. Bring me both and I'll help you word the claim."#,
            r#"Cover like that is on the maker's page, sorry - the model number and the date of the receipt are what it turns on, and I'll help you with the claim once you know."#,
        ],
    ),
    (
        r#"(?i)\b(?:this|that|the) (?:model|brand|version|make|one)\b|\bmodel number\b|\bwhich model\b|\bthe manual\b|\binstructions\b|\bwarranty\b|\bguarantee\b|\bspare part\b|\bpart number\b|\bwhere can i buy\b|\bis it in stock\b|\bin stock\b|\bhow much (?:is|are) (?:a|an|the)\b|\bhow much (?:does|do) (?:it|this|that|one|they|a|an|the|my|your)\b[^.!?]{0,24}?\b(?:cost|costs|charge|charges|go for|goes for|come to|comes to|pay)\b"#,
        &[
            r#"The maker's own page has that, sorry - the model number on the label under it is what to put in, and it will give you the manual, the parts and the price. Tell me what it says and I'll help you work out whether it is worth it."#,
            r#"That is the retailer's or the maker's to say, sorry, and prices and stock move daily. The model number is the thing to search, and I'll help you weigh what you find against a repair."#,
            r#"I can't see the listing from here, sorry. With the model number off the label, the maker's page will have the manual and the parts, and I'll help you with the bit after that."#,
        ],
    ),
    (
        r#"(?i)\b(?:train|bus|flight|ferry|coach|tube|tram|plane)s?\b.*\b(?:time|times|platform|delayed|cancelled|strike|leaves?)\b|\bwhat time (?:does|do|is|will)\b|\btimetable\b|\blast (?:train|bus|entry|orders?)\b|\bopening (?:time|times|hours)\b|\bopen (?:until|till|late)\b|\bwhat(?:'s| is) on\b|\bticket (?:price|prices)\b|\bsold out\b|\bdoors open\b|\bthe venue\b|\bplatform\b|\bwhich stop\b"#,
        &[
            r#"That is live, sorry - timetables and listings change by the hour. The operator's or the venue's own app has the current one, today's included, and I'll help you plan around whatever it says."#,
            r#"I can't check live times, sorry. The operator's app (or the board at the stop) is the one that knows, and tell me the time and I'll work out the rest of the day with you."#,
            r#"Times like that I can't see, sorry. Put the journey into the operator's own app and it will show today's, and I'll help you with the bit either side of it."#,
        ],
    ),
    (
        r#"(?i)\b(?:the|my|a) (?:shop|store|garage|chemist|pharmacy|butcher|bakery|chippy|takeaway|restaurant|cafe|pub|salon|barber|garage|tip|dump|library|leisure centre|pool)\b[^?]{0,60}?\b(?:hours?|open|opens|opening|closed|closes?|shut|shuts|shutting|late|early|sell|sells|stock|stocks|in stock|got any|have they got|do they have|does it have|price|prices|cost|costs|cheap|much|phone|number|contact|where is|where's|nearest|which one)\b|\b(?:what time|when|how late|how much|is|are|does|do|will|can)\b[^?]{0,40}?(?:the|my|a) (?:shop|store|garage|chemist|pharmacy|butcher|bakery|chippy|takeaway|restaurant|cafe|pub|salon|barber|tip|dump|library|leisure centre|pool)\b[^?]{0,30}?\b(?:open|opens|opening|closed|closes?|shut|shuts|hours?)\b|\b(?:what hours|what time|when|how late)\b[^?]{0,45}?\b(?:the|my|a) (?:shop|store|garage|chemist|pharmacy|butcher|bakery|chippy|takeaway|restaurant|cafe|pub|salon|barber|tip|dump|library|leisure centre|pool)\b"#,
        &[
            r#"That is the shop's own to say, sorry - its listing on a map app shows today's hours and its number, holidays included, and I'll help you with what to ask when you ring."#,
            r#"I can't see their opening hours from here, sorry. A map app's listing for the branch is usually right about today, and ringing takes a minute; tell me what they say and I'll help with the next step."#,
            r#"Their hours are on their own listing, sorry - search the branch by name on a map app and it shows today's. I'll help with the rest once you know."#,
        ],
    ),
    (
        r#"(?i)\b(?:medicine|medication|tablet|tablets|pill|pills|dose|dosage|antibiotic|painkiller|ibuprofen|paracetamol|antihistamine|cream|ointment|inhaler|prescription|dispensary)\b|\b(?:from|at|in) the (?:chemist|pharmacy|pharmacist)\b|\bwhat (?:can|could|should) i (?:get|take|buy)\b[^?]{0,30}?\b(?:for|from)\b[^?]{0,30}?\b(?:pain|ache|aching|headache|sore|cough|cold|flu|fever|temperature|rash|itch|itching|heartburn|indigestion|diarrhoea|constipation|sick|nausea|hayfever|hay fever|allergy|allergic|throat|earache|toothache|back|stomach|tummy|period|cramp|burns?|bites?|sting|symptom|symptoms)\b"#,
        &[
            r#"A pharmacist will answer that one properly and for nothing, without an appointment - take the packet in and they will check it against whatever else you take. Tell me what they say and I'll help you with the next bit."#,
            r#"That is a pharmacist's question rather than mine, sorry - it is free, no appointment, and they see the whole packet. Bring me what they say and I'll help you keep it straight."#,
            r#"I can't weigh a dose safely from here, sorry. A pharmacist will, in a few minutes and for free; say what they tell you and I'll help you with the rest."#,
        ],
    ),
    (
        r#"(?i)\bbenefit\b|\buniversal credit\b|\btax credit\b|\bpension\b|\bstate pension\b|\bHMRC\b|\bincome tax\b|\bself assessment\b|\brebate\b|\bowed\b|\bowe\b|\bdebt\b|\bcreditor\b|\bIVA\b|\bbankrupt\b"#,
        &[
            r#"That is a free advice service's call rather than mine, sorry - Citizens Advice or the department's own line will know your rules and your numbers, and I'll help you get the figures straight before you ring."#,
            r#"I can't call that one, sorry: who is owed what depends on your record, and a free adviser can see it. Bring me their answer and I'll help you work out what to do next."#,
            r#"Their rules, their answer, sorry - a free advice line will tell you in one call. Say what they say and I'll help you plan around it."#,
        ],
    ),
    (
        r#"(?i)\b(?:my|the) (?:gp|doctor|surgery|consultant|hospital|clinic|appointment|referral|scan|x-?ray|test results?|results?|biopsy|waiting list)\b|\bnhs\b"#,
        &[
            r#"That is your surgery's to say, sorry - they have the notes and the wait list in front of them, and the number is on the letter. Tell me what they say and I'll help with the next step."#,
            r#"I can't see your notes or the list from here, sorry. One call to the surgery answers it, and I'll help you work out what to ask them."#,
            r#"Your surgery holds that one, sorry. Ringing and asking for the wait for that clinic is the quick way; bring me the answer and I'll help you plan around it."#,
        ],
    ),
];

pub(super) const BAGGAGE_POOL: &[&str] = &[
    r#"That is your airline's rather than mine, sorry - the allowance is set by the fare, so two people on the same plane can have different ones. Your booking's own page shows the kilos you have paid for, and adding a bag before the airport is always the cheaper way. Tell me what it says and I'll help you pack to it."#,
    r#"The airline sets that and the fare decides it, sorry, so it is your booking that answers it: open it and look for the baggage line rather than the airline's general page. Bring me the figure and I'll help you fit everything in."#,
    r#"I can't call that one - it belongs to the ticket rather than to the plane, sorry. Your booking shows what you have paid for and the desk weighs it to the kilo; tell me the number and I'll help you sort the rest."#,
];
