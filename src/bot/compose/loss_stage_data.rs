// Advice for open questions during a bereavement.
pattern!(
    OPEN_ASK,
    r####"^\s*(?:(?:and|so|ok|okay|but|well|right)\s*,?\s*)*(?:what\s+(?:do|should|shall|can|could)\s+(?:i|we)\s+(?:do|now|next)(?:\s+(?:now|next|today))?|what\s+(?:now|next|happens\s+now|do\s+i\s+do\s+now)|now\s+what|where\s+do\s+(?:i|we)\s+(?:start|begin)(?:\s+now)?|what\s+am\s+i\s+supposed\s+to\s+do(?:\s+now)?)(?:\s+about\s+(?:it|that|this|him|her|them|all this|all of it))?\s*[.?!]*$"####
);
pattern!(
    HELPERS,
    r####"(?i)\b(?:doctor|doctors|gp|gps|consultant|nurse|nurses|vet|vets|hospice|hospital|carers?)\b"####
);
const SOMEONE_DYING: &[&str] = &[
    r####"I'm sorry you are facing this. The waiting is the hardest part of it: ask the doctors what to expect and what will keep them comfortable, say what you would regret not saying, and let people help you - this is not a thing to carry on your own."####,
    r####"I'm sorry. Being there is most of it: sit with them, keep the room calm, and let them talk about it or not. Ask about the pain relief and who you ring at night, so you know what to do when the time comes."####,
    r####"I'm sorry. Spend the time on what they want rather than on what you think you should be doing. Tell the people around you what is happening - you will need them, and most of them will want to help."####,
    r####"I'm sorry. Ask the doctor to explain what the next few weeks look like, in plain words, and write it down - it is hard to hold in your head. Then let the rest go and be with them."####,
];

// PyBot's FIRST_DAYS_PERSON pool, updated for UK registration and notifications.
// https://www.gov.uk/after-a-death
// https://www.gov.uk/register-a-death
// https://www.gov.uk/after-a-death/organisations-you-need-to-contact-and-tell-us-once
const FIRST_DAYS_PERSON: &[&str] = &[
    "I'm sorry for your loss. If the death happened in England or Wales, wait for the medical examiner's office or coroner to confirm you can register it. Once you have confirmation, contact the register office. GOV.UK's Register a death service gives the steps for where the death happened.",
    "Start with registering the death. The doctor or hospital team can tell you who is handling the paperwork, and a funeral director can explain what they will need afterwards. GOV.UK's Register a death service gives the process for where the death happened.",
    "Take it a step at a time. Start with GOV.UK's Register a death service. If the person lived in England, Scotland or Wales, ask the registrar about Tell Us Once. It can notify most government organisations. Contact their bank and any private pension provider separately.",
];

// PyBot's THE_DAYS_BEFORE pool, with veterinary assessment before timing or treatment advice.
// https://www.rcvs.org.uk/veterinary-professionals/conduct-and-guidance/supporting-guidance/8-euthanasia-of-animals
// https://www.bluecross.org.uk/preparing-say-goodbye-your-pet
// https://www.pdsa.org.uk/pet-help-and-advice/looking-after-your-pet/all-pets/how-can-i-tell-if-my-pet-still-has-a-good-quality-of-life
const THE_DAYS_BEFORE: &[&str] = &[
    "I'm sorry you're facing this. Call your vet to talk through how your pet is doing and what could keep them comfortable. Ask which changes mean they need urgent help and who to ring out of hours.",
    "I'm sorry. Ask your vet to explain what they expect over the next few days and how to recognise pain or distress. Tell them what you've noticed about your pet's eating, movement and usual activities so they can help you judge comfort and quality of life.",
    "I'm sorry. Ask your vet to talk you through the options and what an appointment would involve. They can help you decide what is kindest for your pet and explain what to expect if the plan changes.",
];
const FIRST_DAYS_PET: &[&str] = &[
    r####"Nothing has to be decided today. Ring the vet when you can face it and they will explain cremation or a home burial, and what you can keep - a paw print, a lock of fur. The rest of the day can just be quiet."####,
    r####"The vet does the practical part, so the one thing to think about is what you would like done with their body, and whether you want them back home afterwards. Everything else can wait, and asking for a paw print or a clipping of fur is worth doing if you might want it."####,
    r####"There is no rush to decide anything. The vet will talk you through the choices when you ring them, and they are used to being asked to wait while you think. Take the day slowly - people often find the first one is the hardest."####,
];

const MISS_REPLIES: &[&str] = &[
    r####"Of course you do. It goes in waves, and the waves get further apart."####,
    r####"The ones we miss are the ones who mattered. Say as much or as little as you like."####,
    r####"That ache is real, and it doesn't keep office hours. Say more whenever you want to."####,
    r####"Missing like that makes sense. Those days can be long ones — I'm around if talking would help."####,
    r####"Some days the missing is louder than others. No need to do anything with it here; I'm just listening."####,
];

const DYING_SAY_REPLIES: &[&str] = &[
    r####"Say the plain things while they can hear them: that you love them, one memory you both laugh at, and that you will be all right. Nothing clever is needed."####,
    r####"Skip the big speech and tell them one specific thing you are grateful for. It is easier to say than "I love you" out loud, and it is what people remember being told."####,
    r####"Ask them what they want to talk about, and follow that. If they want the ordinary stuff - the garden, the football - that is not avoiding it, that is the visit."####,
    r####"Bring them something to decide or to hear: how the family is, what the doctor said, the small gossip. Being useful is how most people say it."####,
];
