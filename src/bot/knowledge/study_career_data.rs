//! Career aliases and replies from the pinned study tables, with regulated routes updated.

pub(super) const CAREER_SUBJECTS: &[(&[&str], &str)] = &[
    (
        &["nurse", "nursing"],
        "To become a registered nurse, choose an NMC-approved nursing degree or degree apprenticeship. Providers set their own school grades and accepted qualifications. English, maths and a science or health subject are useful starting points. A nursing associate programme leads to a different registration and may offer progression towards nursing.",
    ),
    (
        &["midwife", "midwifery"],
        "To qualify as a midwife, find an NMC-approved degree or degree apprenticeship where available. Check each provider for its GCSE and Level 3 subjects, grades and evidence of experience. Care or maternity experience can help you talk about the role at interview.",
    ),
    (
        &["doctor", "medicine", "medical school", "surgeon"],
        "Compare specific UK medical school entry requirements. Many require chemistry, biology or both at A level, plus specific GCSEs, but the subjects and grades differ. Check the current admissions test and application timetable for each school. Reflect on any care or volunteering experience you can discuss at interview.",
    ),
    (
        &[
            "teacher",
            "teaching",
            "primary school teacher",
            "secondary school teacher",
        ],
        "For teacher training in England, the usual GCSE requirement is grade 4 or equivalent in English and maths, plus science for primary. You can train towards qualified teacher status after a degree or through an undergraduate route such as a teacher degree apprenticeship. Check your provider's subject-specific entry criteria, especially for secondary teaching.",
    ),
    (
        &["vet", "veterinary", "vet school"],
        "Check the requirements of RCVS-accredited veterinary degrees. Many ask for high grades in chemistry and biology at A level or equivalent, but courses set their own subjects, grades and animal experience expectations. Foundation, gateway and graduate routes have different criteria.",
    ),
    (
        &["lawyer", "law", "solicitor", "barrister", "law degree"],
        "Choose A levels you can do well in and check individual course requirements. To qualify as a solicitor in England and Wales through the SQE route, you need a degree or equivalent, both SQE assessments, qualifying work experience and to meet suitability requirements. Barristers follow a separate Bar Standards Board route.",
    ),
    (
        &[
            "engineer",
            "engineering",
            "mechanical engineer",
            "civil engineer",
            "electrical engineer",
        ],
        "Maths and physics are often useful for engineering degrees. Check each course's subjects and whether it is accredited for the professional registration you want. A degree apprenticeship combines study and paid work, while a technician apprenticeship is another route.",
    ),
    (
        &[
            "accountant",
            "accountancy",
            "accounting",
            "chartered accountant",
        ],
        "Accountancy asks for no fixed subjects, though maths and a numerical subject help, and there are three ways in: A-levels then an AAT or ACCA/ACA training contract, a degree then the same exams, or an accountancy degree apprenticeship where the employer pays for the exams. The exams are the qualification either way, so the route with the least debt is usually the apprenticeship.",
    ),
    (
        &["paramedic", "paramedic science"],
        "To register as a paramedic, look for an HCPC-approved programme, such as a paramedic science degree or approved degree apprenticeship. Compare providers' science, maths and English entry requirements and whether they accept an Access course. Ambulance support roles can build experience and have their own entry criteria.",
    ),
    (
        &[
            "electrician",
            "plumber",
            "plumbing",
            "mechanic",
            "chef",
            "carpenter",
            "bricklayer",
            "joiner",
            "gas engineer",
        ],
        "Look for the specific apprenticeship or vocational course for the trade you want. Employers and providers set requirements for English, maths and other qualifications. For regulated work, ask the provider which qualifications, experience and registration you need, including Gas Safe registration for relevant gas work.",
    ),
];
pub(super) const GRADES_ROUTE: &[(&[&str], &str)] = &[
    (
        &["doctor", "medical school", "medicine"],
        "UK medical schools set their own GCSE and Level 3 requirements. Many ask for chemistry, biology or both. Check each course's specific grades, admissions test and work experience guidance. A medical degree from a school the GMC accepts is the start of the UK training route.",
    ),
    (
        &["pharmacist", "pharmacy"],
        "Check the science subjects and grades for each accredited MPharm course. To register as a pharmacist in Great Britain, complete the approved education and foundation training route and pass the registration assessment.",
    ),
    (
        &["physiotherapist", "physiotherapy"],
        "For physiotherapy, choose an HCPC-approved programme. Entry grades and science requirements vary by provider. Compare undergraduate routes and pre-registration postgraduate options if you already have a degree.",
    ),
    (
        &["architect", "architecture"],
        "Architecture course providers set their own subject, grade and portfolio requirements. Check that the course fits an Architects Registration Board accredited route, including the practical experience and qualifications applicable as the UK framework changes.",
    ),
    (
        &["social worker", "social work"],
        "Check subjects and entry grades for an approved social work programme, such as an undergraduate degree or qualifying postgraduate route. To practise in England you must register with Social Work England. Other UK nations have their own regulators.",
    ),
    (
        &["police", "police officer"],
        "In England and Wales, entry requirements depend on the police force and route. A Level 3 qualification or equivalent experience is common on the police constable entry programme and degree apprenticeship. Check the force's current requirements and recruitment assessments, including vetting.",
    ),
];
pub(super) const JOB_LENGTHS: &[(&[&str], &str, &str)] = &[
    (
        &["nurse", "nursing"],
        "A full-time nursing degree commonly takes three years. An NMC-approved nursing degree apprenticeship may take longer. A nursing associate programme is a separate qualification and may offer a route onwards.",
        "Check the NMC readmission criteria. If you lack the practice hours for readmission, an approved return-to-practice programme or a test of competence can be an option.",
    ),
    (
        &["midwife", "midwifery"],
        "A full-time midwifery degree commonly takes three years. Check the provider's length for an approved apprenticeship or other route.",
        "Check your NMC readmission requirements after a break. Depending on your practice hours, you may need an approved return-to-practice programme or another route the NMC accepts.",
    ),
    (
        &["dentist", "dentistry", "dental school", "dental surgeon"],
        "A UK dental degree typically takes five years. Ask the course and employer about General Dental Council registration and whether further foundation training applies to the job you want.",
        "Check the General Dental Council registration route for your qualification and experience. The length and any assessment depend on the route.",
    ),
    (
        &["doctor", "medicine", "medical school", "surgeon"],
        "A UK medical degree usually takes five years, or four on some graduate-entry courses. The Foundation Programme lasts two years. Specialty and GP training lengths vary.",
        "Ask the GMC which registration route fits your qualification and experience. PLAB is one route for some internationally qualified doctors. Return to practice or licensing arrangements depend on your registration and previous practice.",
    ),
    (
        &[
            "teacher",
            "teaching",
            "primary school teacher",
            "secondary school teacher",
        ],
        "Routes to qualified teacher status can take three or four years within a first degree or typically a year of postgraduate training after a degree. Check the length of any teacher degree apprenticeship you are considering.",
        "Ask your local teacher training or return-to-teaching service which refresher support is available and check any registration or induction requirement for your role.",
    ),
    (
        &["vet", "veterinary", "vet school"],
        "A UK veterinary degree typically takes five years. Some accredited graduate-entry routes take four years with a relevant prior degree.",
        "",
    ),
    (
        &["lawyer", "law", "solicitor", "barrister", "law degree"],
        "For the solicitor SQE route in England and Wales, allow for the degree or equivalent, both SQE assessments and two years of qualifying work experience. Some experience can be gained during study or through an apprenticeship. The barrister route has different stages.",
        "If you already qualified as a lawyer, check the relevant regulator's route for qualified lawyers. Assessment, experience and prior qualification rules can differ from the first-time route.",
    ),
    (
        &[
            "engineer",
            "engineering",
            "mechanical engineer",
            "civil engineer",
            "electrical engineer",
        ],
        "A BEng is commonly three years full-time in England and an MEng four. Degree apprenticeships and other countries' courses can take different lengths. Professional registration depends on demonstrating the required competence and experience.",
        "Ask the relevant professional engineering institution how it will assess your experience and any further learning needed. A degree apprenticeship is one route if you need another qualification.",
    ),
    (
        &[
            "accountant",
            "accountancy",
            "accounting",
            "chartered accountant",
        ],
        "Accountancy training often takes several years and can be combined with paid work. Check the qualification body's exams, experience requirements and exemptions for the route and starting level you choose.",
        "Ask the qualification body which exams and practice you can carry over, and what you would still need to finish or return to membership.",
    ),
    (
        &["paramedic", "paramedic science"],
        "A full-time paramedic science degree is commonly three years. Other HCPC-approved routes may take a different length. An ambulance support role can provide experience and has a separate qualification.",
        "",
    ),
    (
        &["pharmacist", "pharmacy"],
        "In Great Britain a common route is a four-year accredited MPharm plus 52 weeks of foundation training, with some courses integrating that training. You also need to pass the registration assessment.",
        "Ask the General Pharmaceutical Council which restoration requirements apply to your previous registration and recent practice.",
    ),
    (
        &["physiotherapist", "physiotherapy"],
        "A full-time physiotherapy degree commonly takes three years. Some approved pre-registration master's programmes take two years for graduates. Check the course and the HCPC approved list.",
        "Check HCPC readmission rules. Return to practice updating depends on how long you have been out, including 30 days after two to five years and 60 days after five or more years.",
    ),
    (
        &["architect", "architecture"],
        "The traditional UK architecture route commonly takes around seven years including study and experience. The Architects Registration Board is transitioning to a new framework, so check the accredited qualifications and practical training for your intake and pathway.",
        "Ask the Architects Registration Board which route applies to your existing qualifications and practical experience under its transition arrangements.",
    ),
    (
        &["social worker", "social work"],
        "A full-time social work degree commonly takes three years and a qualifying master's around two. Some employment-based routes differ. Check approval and registration requirements in the UK nation where you want to work.",
        "If previously registered in England, check Social Work England's restoration process and any requirements for updating your skills or demonstrating recent practice.",
    ),
    (
        &["police", "police officer"],
        "In England and Wales, where offered, the police constable entry programme lasts two years and the police constable degree apprenticeship three. Check current eligibility with your chosen force. You are paid from when you join.",
        "Ask your chosen force about rejoining, any recognition of prior service and further training or checks. The arrangements depend on the force and your previous role.",
    ),
    (
        &["electrician", "plumber", "plumbing", "gas engineer"],
        "Many apprenticeships in these trades take several years. Check the exact standard, work placements, assessments and any trade registration needed. Ask how your previous qualifications or experience affect the length.",
        "Ask an accredited provider or trade registration body what previous qualifications count and which assessments or updating you still need.",
    ),
    (
        &["chef"],
        "The length depends on whether you choose a commis chef apprenticeship, a college course or work-based training. An apprenticeship is paid employment. Ask the provider for the course length and the employer for the pay.",
        "Ask a kitchen employer or college which of your skills count and whether a short refresher or further qualification would help.",
    ),
    (
        &["mechanic", "carpenter", "bricklayer", "joiner"],
        "An apprenticeship in these trades typically takes several years. Check the exact occupation standard and level and whether the provider gives credit for prior experience.",
        "Ask the college or employer what previous training counts and whether you need to refresh skills or take another assessment.",
    ),
];
pub(super) const THE_GENERAL_CAREER: &str = "Check the entry requirements on the university, college or apprenticeship page for the exact role. Compare subjects, grades and any recognised work experience across a few providers. Ask a teacher or someone doing the job which choices would keep another route open.";
