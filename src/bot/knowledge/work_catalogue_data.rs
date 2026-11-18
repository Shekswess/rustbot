pub(super) const DAY_TABLE: &[(&str, &str)] = &[
    (
        "plumber",
        "A plumber's day is a list of jobs, not one job: a dripping tap, a blocked loo, a boiler that will not fire, a bathroom that has to be piped and then signed off. Most of the work is diagnosis under a sink or in a loft, and the good plumbers earn their money on the second call-out to the same house. It is dirty and it is on your knees, and the money is in gas and heating rather than taps.",
    ),
    (
        "electrician",
        "An electrician's day is part first fix, part fault-finding: chasing walls and pulling cable on a new build, then testing and certifying, or tracing why one circuit keeps tripping in a house that has been added to four times. Every finished job ends with a certificate, so the paperwork is part of the trade. It is cleaner than plumbing and more cramped - lofts, cupboards and floorboards.",
    ),
    (
        "mechanic",
        "A mechanic's day is servicing and diagnosing: oil and filters on a routine service, then brakes, clutches and suspension on older cars, and an hour or two chasing a warning light that could be a sensor or a wiring fault. Fault codes point at systems, not parts, so the skill is testing rather than guessing, and the job is standing, lifting and lying under a car in all weathers.",
    ),
    (
        "bricklayer",
        "A bricklayer's day is blocks and bricks to a line: setting out the corners, running a course up, keeping it plumb and level, and leaving the openings where the drawings say. It is the fastest visible trade on a site and the hardest on the back and shoulders, and cold weather stops mortar going off. Piecework rates make a good bricklayer's pay look high until you count the days lost to rain.",
    ),
    (
        "carpenter",
        "A carpenter's day is either second fix in a finished house - doors, skirting, stairs, kitchens - or a workshop bench making what goes in. Site carpenters do more cutting and fitting, bench joiners do more setting out and jointing, and both need to read a drawing and work to a millimetre. Dust, noise and knees, and the good ones are never out of work.",
    ),
    (
        "plasterer",
        "A plasterer's day is walls and ceilings: boarding, then skim, then the corners and the trowel work, and the finish is judged by whether a light shows a ripple across it. It is wet, heavy and fast - a ceiling has to be got right in one go before it sets - and the shoulders and wrists take the wear.",
    ),
    (
        "painter and decorator",
        "A painter's day is preparation, and the painting is the easy part: filling, sanding, caulking, masking, then two coats and a careful line where the wall meets the ceiling. It is quiet, indoors and tidy work, and the difference between a good job and a bad one is nearly all in what happened before the paint went on.",
    ),
    (
        "roofer",
        "A roofer's day is up a ladder or on a roof: replacing slipped tiles, fixing a leak that has been chased around a ceiling for months, felting and battening a new roof, or clearing gutters. It is the most weather-dependent trade there is, and the one where the work is done at height in the cold, so head for a roofer with insurance and a written quote.",
    ),
    (
        "builder",
        "A builder's day is the whole site rather than one trade: ordering materials, setting out, keeping the trades in the right order, checking the work, and being the person the customer rings. The tools come out less than people expect and the phone never stops. Small jobs are one person and a van, and a house extension is a month of juggling.",
    ),
    (
        "hgv driver",
        "A lorry driver's day is mostly waiting and then concentrating hard: loading at a depot, driving to a timed slot, waiting to be tipped, then the next one. The law allows 9 hours of driving a day, 10 twice a week, with a 45-minute break after four and a half hours, and tachograph rules are enforced. Nights out and early starts are normal, and the pay is in the hours and the overtime.",
    ),
    (
        "delivery driver",
        "A delivery driver's day is a route of 80 to 150 drops and a van: load in the morning, then drive, park, carry, knock, repeat, with the phone tracking you all day. The skill is knowing where you can legally stop and how long each drop really takes. It is physical, the weather is the office, and the day is usually over when the van is empty.",
    ),
    (
        "bus driver",
        "A bus driver's day is a rota of short runs with a break in the middle: the same route, the same stops, and traffic that moves the timetable around. The driving is the easy part and the passengers are the job - fares, buggies, wheelchairs, and the last bus on a Saturday night. Shifts are split, so an early start and a late finish are the same day.",
    ),
    (
        "taxi driver",
        "A taxi driver's day is the waiting as much as the driving: ranks, bookings, airport runs and the school contract if you can get one. The money is in knowing when the town is busy, and the cost is in the car and the licence, which come out of the takings. Evenings and weekends are the work, so it suits people who do not want office hours.",
    ),
    (
        "farmer",
        "A farmer's day follows the animals and the season: milking twice a day every day, or moving stock, checking fences and water, then the field work in its window - drilling, spraying, silage, harvest. There is no weekend in livestock, and harvest means long days while the weather holds. The income is lumpy, and a lot of it comes from subsidy and contracting as much as from the produce.",
    ),
    (
        "nurse",
        "A nurse's day on a ward is a handover, then a list of patients with their medicines, observations, washes, dressings and notes, and a doctor's round somewhere in the middle. Shifts are 12 hours, three or four a week, days and nights, and the last hour is usually the busiest. The clinical work is a third of it; the rest is families, paperwork and keeping a dozen things moving at once.",
    ),
    (
        "care worker",
        "A care worker's day is a round of visits or a shift in a home: getting people up, washed, dressed and fed, medicines, moving and handling, and writing down what happened. Home care is often 15-minute slots with travel between them and no paid travel time at some employers. It is personal work done at speed, and the good ones know the person as well as the task.",
    ),
    (
        "midwife",
        "A midwife's day is either antenatal clinic, postnatal checks on the ward, or labour ward looking after one woman through her labour and writing the record as it happens. Shifts are 12 hours, days and nights, and labour ward is unpredictable, so a quiet night can turn into a section and a transfer in an hour. It is the most hands-on job in the hospital and the one with the most joy in it.",
    ),
    (
        "paramedic",
        "A paramedic's day is a series of 999 calls, each one starting with almost no information: chest pain, a fall, a mental health crisis, a road collision. You assess, treat within your scope, decide who needs hospital and who can stay at home, and write it up between jobs. Shifts are 12 hours and breaks are whatever the calls allow.",
    ),
    (
        "police officer",
        "A police officer's day is calls and paperwork: attending incidents, taking statements, arresting and processing, then a file that has to stand up in court. Response teams work shifts over 24 hours, and the honest description is long stretches of tedium with short periods of real danger. Most of the job is talking to people who are having their worst day.",
    ),
    (
        "firefighter",
        "A firefighter's day is drills, checks and training around the shouts: testing kit, learning building layouts, fitness, and community visits, with the crew cooking and eating together on a 24-hour shift. Fires are a small part of the calls - more are road collisions, floods, and helping an ambulance crew get a heavy person downstairs.",
    ),
    (
        "teacher",
        "A teacher's day is teaching five or six lessons, plus the parts nobody sees: planning, marking, data, parents' evening, and the break duty. Directed hours are about 1,265 a year over 195 days, and the planning and marking on top make 45 to 55 hours in term time. The classroom itself is the good part, and the holidays are the trade for the term-time hours.",
    ),
    (
        "teaching assistant",
        "A teaching assistant's day is support in the classroom: helping the children who are stuck, running small groups for reading or phonics, and managing behaviour alongside the teacher. The hours are term-time only and usually finish with the children, so it fits round a family, and the pay is hourly rather than a salary. The work is with the same children all year.",
    ),
    (
        "social worker",
        "A social worker's day is visits, assessments and the phone: seeing a family, writing up the visit, chasing a school or a housing officer, and a meeting about a child or an adult at risk. The caseload is the job, and the paperwork is the reason people leave. It is the work where one decision can change a family's year, so supervision and recording are taken seriously.",
    ),
    (
        "train driver",
        "A train driver's day is a diagram of trips: sign on, check the unit, drive the same route with the same stops, and hand over at the other end. A computer does much of the driving, so the job is watching the line, the signals and the speed, and taking over when it does not. Shifts are unsocial and start early, and a driver's year is about 35 hours a week with rest days in the roster.",
    ),
    (
        "barrister",
        "A barrister's day is either in court or in chambers preparing for it: reading the papers, drafting the questions, then standing up and doing the case, and afterwards the note on what the judge said. The self-employed model is the norm, so fees, chambers rent and the VAT are yours to run.",
    ),
    (
        "architect",
        "An architect's day is drawings and coordination: developing a design, meeting the engineer and the client, checking the building regulations, and answering site queries as it is built. A project runs for years, so most days are a step on something that will not be finished for a long time.",
    ),
    (
        "security guard",
        "A security guard's day is a patrol, a post or a control room: walking the building, checking passes, watching screens, and writing the log. Shifts cover 24 hours and nights are the norm, and most of the job is being alert for a long time and being calm when something happens.",
    ),
    (
        "office worker",
        "An office worker's day is a list of tasks and meetings: email, a spreadsheet, a document, a call, and the meeting that interrupts all of it. The hours are usually 9 to 5 with 37.5 or 40 on paper, and the skill is holding a deadline while people ask for things. It is quiet work done sitting down.",
    ),
    (
        "factory worker",
        "A factory worker's day is a station on a line: the same operation, at the pace the line sets, for a shift, with a break in the middle. Shifts run round the clock and often rotate, the noise needs ear protection, and the pay is a rate per hour plus shift allowance.",
    ),
    (
        "pharmacist",
        "A pharmacist's day is prescriptions: checking what the doctor has written, dispensing, answering the counter, and giving advice on a cough or a rash. The clinical part is spotting a dose that is wrong or a pair of medicines that should not mix, and the shop work is the rest.",
    ),
    (
        "librarian",
        "A librarian's day is the desk, the stock and the enquiries: helping people find things, ordering and cataloguing, running sessions for children or for a reading group, and keeping the computers working. The quiet is a myth - it is public-facing work with a research job attached.",
    ),
    (
        "doctor",
        "A doctor's day is a clinic list or a ward round: ten-minute appointments in general practice, or a hospital round with the notes and the tests for each patient, then the jobs it produces - referrals, prescriptions, a call to a family. GPs carry a list of 30 or more appointments a day and the hospital work is shifts. Most of it is pattern-spotting and explaining, and the pressure is the clock.",
    ),
    (
        "dentist",
        "A dentist's day is a list of 15 to 30 minute appointments: check-ups, fillings, and the emergencies squeezed in at the end of the morning. Most patients need a clean, a filling or reassurance, and the work is done sitting down in a lit box with an assistant. The practice is a business, so the diary and the recall list matter as much as the drill.",
    ),
    (
        "vet",
        "A vet's day is a morning of consultations - vaccinations, skin, limps, a cat that will not eat - and an afternoon of operations, dentals and x-rays. Large animal vets are out on farms instead, and out-of-hours cover is shared between practices. It is a business as well as a profession, so the conversation about cost is part of the job.",
    ),
    (
        "chef",
        "A chef's day is prep in the morning and service in the evening: deliveries, stocks, sauces, mise en place, then the rush where every plate is timed against a table's order. Split shifts and 50 to 60 hour weeks are normal in restaurants, and the kitchen is hot, loud and close. The satisfaction is at the pass when it all lands at once.",
    ),
    (
        "waiter",
        "A waiter's day is sections and covers: taking orders, carrying, serving, clearing, and knowing the menu well enough to answer a question about an allergy. Shifts are split around lunch and dinner, the pay is minimum wage plus tips, and the job is on your feet for eight hours. Good service is mostly reading the table and being there when needed.",
    ),
    (
        "hairdresser",
        "A hairdresser's day is a column of appointments, each with a consultation and a finish: colour work in the morning, cuts in the afternoon, and regulars who expect the same result every six weeks. It is on your feet with your arms up, and the money depends on building a clientele who ask for you. Saturdays are the busiest day of the week.",
    ),
    (
        "cleaner",
        "A cleaner's day is a list of rooms and a time limit: bathrooms, kitchens, floors, bins, and the parts a household forgets. Domestic cleaning is usually hourly and self-employed, office cleaning is an early or late shift, and hotel work is rooms at a set number of minutes each. It is physical, mostly invisible, and judged only when it is done badly.",
    ),
    (
        "retail worker",
        "A retail worker's day is the shop floor and the till: deliveries and replenishment before opening, then serving, fitting rooms, stock checks and the queue at the end of the day. Hours are shifts across the week, often including weekends, and the pay is close to the minimum wage with staff discount. Most of the job is being pleasant to people who are in a hurry.",
    ),
    (
        "postman",
        "A postman's day is sorting in the depot, then a walk or a van round of the same streets with a bag and a bundle of parcels. The round is fixed, the weather is not, and the walking is 10 to 15 miles on a foot round. It finishes when the round is done, and Christmas is the season everyone remembers.",
    ),
    (
        "warehouse worker",
        "A warehouse worker's day is on a pick list: a scanner, a trolley, and the same aisles at a rate the system measures. Goods-in and loading is heavier, packing is quicker, and shifts run round the clock in the big sheds. It is standing, lifting and walking all day, and the target is the reason the job is tiring.",
    ),
    (
        "accountant",
        "An accountant's day is client work against a deadline: bookkeeping, VAT returns, year-end accounts and tax, with January and the filing dates setting the rhythm. Practice accountants see a dozen small businesses a week; in-house ones close the month and explain it to managers. The work is detail and software, and the busy season is genuinely busy.",
    ),
    (
        "solicitor",
        "A solicitor's day is files and clients: reading, advice, letters, negotiation, and time recorded in six-minute units. Conveyancing is volume, family and crime are court dates, and commercial work is documents. The pressure is deadlines and other people's money, and most of the job is writing clearly about what should happen next.",
    ),
    (
        "software developer",
        "A developer's day is a stand-up, then a task: reading code someone else wrote, writing a small piece of it, testing it, then reviewing someone else's change. The hard part is understanding the system, not typing, and most days include one meeting and one surprise. It is desk work, and it can be done from home more than most jobs.",
    ),
    (
        "engineer",
        "An engineer's day depends which engineer: a design engineer works in CAD and calculations, a site engineer sets out and checks work against drawings, and a maintenance engineer is called when a machine stops. All of them are solving a problem with a constraint, and the paperwork is the record that the thing is safe.",
    ),
    (
        "journalist",
        "A journalist's day is the diary, the calls and the deadline: find the story, check it with two sources, write it to length, and file before the page closes. Local reporting is council meetings and court lists; newsroom work is a running feed. The hours are unpredictable and the pay is low at the start.",
    ),
    (
        "civil servant",
        "A civil servant's day is policy, casework or a service: reading, drafting, briefing a minister or a manager, and answering correspondence to a deadline. The work is steady, the process is the point, and the hours are usually 37.5 a week with flexitime. Change of government changes the priorities, not the building.",
    ),
    (
        "banker",
        "A banker's day is clients who need money: reading accounts, structuring a loan, credit papers and meetings. High street banking is branch appointments and mortgages; investment banking is deals and very long hours. The pay is high at the top and ordinary at the counter, and the two jobs have almost nothing in common.",
    ),
];
pub(super) const PAY_TABLE: &[(&str, &str)] = &[
    (
        "nurse",
        "Many newly qualified NHS nursing posts are advertised at Agenda for Change Band 5. In England in 2026 to 2027, Band 5 runs from £32,073 to £39,043 basic pay. Specialist or management posts can be on higher bands. Check the advertised band, region and any unsocial-hours or high-cost-area supplements.",
    ),
    (
        "care worker",
        "Care worker pay varies with the employer, location, qualifications and whether the role involves nights or sleep-ins. Compare current hourly pay and guaranteed hours, and ask what the employer pays for travel between clients, night work or senior duties.",
    ),
    (
        "midwife",
        "NHS midwifery pay depends on the advertised Agenda for Change band, experience and country. Newly qualified and experienced posts can be on different bands. Check the current national pay scale and whether the advert lists extra pay for unsocial hours.",
    ),
    (
        "paramedic",
        "An NHS paramedic's basic pay depends on the advertised Agenda for Change band, experience and country. Newly qualified, experienced and specialist posts may have different bands. Check the current scale and separate salary from unsocial-hours payments.",
    ),
    (
        "doctor",
        "In the NHS, foundation doctors, specialty doctors, salaried GPs and consultants are on different contracts or pay scales. Check the current NHS medical pay circular for the country and grade, and separate basic pay from rota supplements and additional hours.",
    ),
    (
        "dentist",
        "Dentistry earnings vary with employment status, NHS and private work, location, contract and practice costs. For a salaried role use the advertised salary or current national pay scale. A practice owner's turnover and a patient's treatment charge are different from personal earnings.",
    ),
    (
        "pharmacist",
        "Hospital pharmacists can be on NHS Agenda for Change bands that depend on the role and country. Community pharmacy and locum pay depend on the employer and location. Check current job adverts or the NHS scale, comparing basic pay, hours and any supplements.",
    ),
    (
        "vet",
        "£36,000 to £50,000 for a small animal vet with a few years in, and £60,000 upwards with a certificate or a partnership. Out-of-hours cover is paid on top and a locum day is £300 to £450.",
    ),
    (
        "social worker",
        "Newly qualified is about £32,000, and with two or three years in it is £34,000 to £44,000; a senior practitioner or team manager is £44,000 to £55,000. Agency social workers are paid by the hour, often £30 to £40.",
    ),
    (
        "teacher",
        "For a teacher in England, check the current Department for Education school teachers' pay document for the main, upper or leadership range and the relevant London area. Other UK nations publish separate scales. Check the school's pay policy, especially if it is an academy.",
    ),
    (
        "teaching assistant",
        "About £23,000 to £26,000 full time, £12.50 to £15 an hour, and the pay is spread over 12 months because the job is term time only. A higher level teaching assistant with a specialism gets £27,000 to £30,000.",
    ),
    (
        "police officer",
        "A constable's basic pay rises through the force's pay scale; promotion to sergeant moves to a different scale. Check the force's current pay table and ask how overtime, night duty and regional allowances are paid.",
    ),
    (
        "firefighter",
        "A firefighter's basic pay depends on the current service pay scale and whether you are a trainee, developing or competent. Ask the local fire service for its current figures and how the stated pay accounts for shift work and any allowances.",
    ),
    (
        "hgv driver",
        "£14 to £18 an hour, about £32,000 to £42,000 a year. Nights out, ADR and weekend or overtime shifts push a lorry driver past £45,000, and agency work pays more in the weeks nobody wants.",
    ),
    (
        "delivery driver",
        "Check current delivery-driver adverts for hourly, salaried or per-drop pay. If you would be self-employed, compare what is left after fuel, insurance, van and maintenance costs. Pay and the number of drops can vary substantially by season.",
    ),
    (
        "bus driver",
        "£13 to £16 an hour, about £28,000 to £34,000, with London a few thousand more. Split shifts and unsocial hours are the trade-off, and overtime is regular.",
    ),
    (
        "train driver",
        "£45,000 to £65,000 as a qualified driver, and more on long-distance and freight work. Training pay is lower and the training takes nine to twelve months.",
    ),
    (
        "taxi driver",
        "Self-employed, so it is takings minus the car: £400 to £700 a week coming in, which is about £25,000 to £35,000 a year before fuel, insurance, maintenance and licensing.",
    ),
    (
        "chef",
        "£13 to £17 an hour, about £26,000 to £35,000; a head chef is £32,000 to £45,000, and hotels and fine dining pay more. The hours are long and the pay is per hour worked.",
    ),
    (
        "waiter",
        "Waiter pay varies with the employer and location. Check the hourly rate before tips, whether tips are pooled, and what a supervisor earns. Compare the rate with the current UK minimum wage for your age and apprenticeship status, if applicable.",
    ),
    (
        "retail worker",
        "Compare current retail vacancies for the basic hourly rate and guaranteed hours. Team leaders and supervisors may be on a higher rate. Ask whether Sunday or bank holiday premiums apply and check the current minimum wage for your age.",
    ),
    (
        "warehouse worker",
        "Compare current warehouse adverts for the hourly rate, guaranteed hours and night premium. Agency and directly employed rates can differ even in the same area. Check the current minimum wage that applies to you.",
    ),
    (
        "factory worker",
        "Factory pay depends on the shift pattern, employer and whether the role involves skilled machine setting or maintenance. Compare the current hourly basic rate and any night or rotating shift premium separately.",
    ),
    (
        "security guard",
        "Security guard pay varies by site, hours and licensing requirements. Ask for the current basic rate, what nights or weekends pay, and what the role requires in terms of SIA licensing.",
    ),
    (
        "postman",
        "£13 to £14 an hour, about £26,000 to £28,000, with driving duties and Christmas overtime on top. A delivery office manager is £30,000 to £35,000.",
    ),
    (
        "office worker",
        "£23,000 to £30,000 for an administrator, and an office manager is £30,000 to £38,000. London adds £3,000 to £5,000, and the sectors that pay best are finance and law.",
    ),
    (
        "accountant",
        "£35,000 to £55,000 once qualified, and about £38,000 newly qualified in practice. A finance manager in industry is £50,000 to £65,000, and London and the big firms pay more again.",
    ),
    (
        "solicitor",
        "About £40,000 newly qualified outside London, and £60,000 to £90,000 with five years in. A City firm pays £100,000 and up from day one, for hours to match.",
    ),
    (
        "software developer",
        "£40,000 to £65,000 with a few years in, and senior or lead roles £70,000 to £100,000. Contract work is £400 to £600 a day with no holiday pay, which is the trade.",
    ),
    (
        "engineer",
        "£28,000 to £35,000 starting, £35,000 to £50,000 chartered, and £50,000 upwards for a principal or a specialist. Site, offshore and defence work pays a premium over an office desk.",
    ),
    (
        "architect",
        "£32,000 to £40,000 newly qualified, £45,000 to £60,000 with a decade in, and a partner or a practice owner more than that. The years of low pay before qualification are the part people do not expect.",
    ),
    (
        "journalist",
        "£24,000 to £32,000 at a local paper or a trade title, and £35,000 to £50,000 in a national newsroom. Freelance is paid per word and the money arrives late.",
    ),
    (
        "librarian",
        "£26,000 to £34,000, and a chartered or senior librarian is £35,000 to £45,000. Council library budgets are why the top of that range is thin.",
    ),
    (
        "civil servant",
        "An administrative officer is £25,000 to £28,000, an executive officer £28,000 to £34,000, and a higher executive officer £34,000 to £40,000. The London pay bands add £4,000 to £5,000.",
    ),
    (
        "banker",
        "Branch and mortgage staff are £25,000 to £35,000 plus a small bonus; an investment banking analyst is £60,000 to £100,000 with a bonus that can double it, for twice the hours.",
    ),
    (
        "barrister",
        "£25,000 to £40,000 in the first years of practice, £60,000 to £150,000 with a decade in, and a handful earn far more. Chambers rent, insurance and no sick pay come out of it.",
    ),
    (
        "mechanic",
        "£13 to £18 an hour, £27,000 to £36,000, and a master technician or a main dealer specialist is £35,000 to £45,000. Your own tools are part of the deal.",
    ),
    (
        "plumber",
        "An employed plumber is £28,000 to £36,000. Self-employed, the takings are £40 to £80 an hour, which sounds like more until the van, tools, insurance, pension and quiet weeks are paid for - about £40,000 to £60,000 a year.",
    ),
    (
        "electrician",
        "An employed electrician is £30,000 to £38,000, and self-employed takings are £40 to £70 an hour before the van, the test gear, the insurance and the certification. A commercial or industrial spark earns more than a domestic one.",
    ),
    (
        "hairdresser",
        "An employed hairdresser's pay depends on hours, experience, commission and the current minimum wage. When renting a chair, compare the takings per appointment with chair rent, products, insurance and quiet periods.",
    ),
    (
        "bricklayer",
        "£15 to £20 an hour employed, £30,000 to £40,000, and piecework on a site can pay more in a good week. Rain and winter are the unpaid part.",
    ),
    (
        "carpenter",
        "£15 to £20 an hour employed, about £30,000 to £40,000, and a bench joiner or a site carpenter with their own tools and van can clear £45,000 on price work.",
    ),
    (
        "painter and decorator",
        "£14 to £18 an hour employed, £28,000 to £36,000, and self-employed day rates of £150 to £250. The preparation nobody sees is what the day rate is really for.",
    ),
    (
        "builder",
        "A site manager is £40,000 to £55,000, a general builder running small jobs £35,000 to £50,000, and a self-employed builder's profit depends on the jobs they price well.",
    ),
    (
        "roofer",
        "£14 to £19 an hour employed, £28,000 to £38,000, and a self-employed roofer's day rate is £180 to £280. Bad weather weeks are unpaid, which is why the day rate is high.",
    ),
    (
        "plasterer",
        "£15 to £20 an hour employed, £30,000 to £40,000, and pricework on a new build is £3 to £5 a square metre. A good plasterer's day rate is £200 to £250.",
    ),
    (
        "nursery nurse",
        "Compare local nursery vacancies for the hourly rate, hours, required qualification and progression to room leader or manager. Check the current minimum wage for your age and any applicable apprenticeship status.",
    ),
    (
        "kitchen porter",
        "A kitchen porter's pay varies by employer and shifts. Compare local hourly rates and guaranteed hours against the current minimum wage that applies to you. If you want to become a commis chef, ask whether the kitchen offers training.",
    ),
    (
        "bar staff",
        "Compare current bar vacancies for the basic rate, tips, late shifts and any supervisor premium. Check the current minimum wage for your age or apprenticeship status separately from tips.",
    ),
    (
        "refuse collector",
        "Pay varies by council or contractor, shift pattern and whether the job is loading, driving or leading a crew. Compare current adverts for basic pay, the hours and any allowance for early starts.",
    ),
    (
        "receptionist",
        "Reception pay depends on sector, employer, location and whether the role includes management. Compare the hourly rate and guaranteed hours in current medical, dental and hotel adverts and check the minimum wage that applies to you.",
    ),
    (
        "soldier",
        "Army pay varies by rank, role, service and specialist pay. Check the Army's current pay and benefits information for the role you are applying for, including the terms and charges for accommodation and food.",
    ),
    (
        "farmer",
        "Pay for an employed farm worker or manager varies by job, region and applicable minimum rate. A farm owner's income depends on profit after costs and can vary greatly from year to year.",
    ),
];
pub(super) const DAY_ALIASES: &[(&str, &str)] = &[
    ("lorry driver", "hgv driver"),
    ("hgv driver", "hgv driver"),
    ("hgv", "hgv driver"),
    ("truck driver", "hgv driver"),
    ("lorry drivers", "hgv driver"),
    ("sparky", "electrician"),
    ("sparkies", "electrician"),
    ("chippy", "carpenter"),
    ("chippies", "carpenter"),
    ("joiner", "carpenter"),
    ("brickie", "bricklayer"),
    ("brickies", "bricklayer"),
    ("painter and decorator", "painter and decorator"),
    ("decorator", "painter and decorator"),
    ("care worker", "care worker"),
    ("carer", "care worker"),
    ("carers", "care worker"),
    ("support worker", "care worker"),
    ("shop worker", "retail worker"),
    ("retail worker", "retail worker"),
    ("sales assistant", "retail worker"),
    ("police officer", "police officer"),
    ("copper", "police officer"),
    ("software developer", "software developer"),
    ("programmer", "software developer"),
    ("developer", "software developer"),
    ("web developer", "software developer"),
    ("teaching assistant", "teaching assistant"),
    ("ta", ""),
    ("solicitor", "solicitor"),
    ("lawyer", "solicitor"),
    ("accountant", "accountant"),
    ("bin man", ""),
];
pub(super) const PAY_ALIASES: &[(&str, &str)] = &[
    ("nursery nurse", "nursery nurse"),
    ("nursery nurses", "nursery nurse"),
    ("nursery worker", "nursery nurse"),
    ("nursery practitioner", "nursery nurse"),
    ("kitchen porter", "kitchen porter"),
    ("kitchen porters", "kitchen porter"),
    ("bar staff", "bar staff"),
    ("bartender", "bar staff"),
    ("bartenders", "bar staff"),
    ("bar worker", "bar staff"),
    ("receptionist", "receptionist"),
    ("receptionists", "receptionist"),
    ("soldier", "soldier"),
    ("soldiers", "soldier"),
    ("army", "soldier"),
    ("support worker", "care worker"),
    ("support workers", "care worker"),
    ("care assistant", "care worker"),
    ("care assistants", "care worker"),
    ("shop assistant", "retail worker"),
    ("shop assistants", "retail worker"),
    ("sales assistant", "retail worker"),
    ("sales assistants", "retail worker"),
    ("lorry driver", "hgv driver"),
    ("lorry drivers", "hgv driver"),
    ("truck driver", "hgv driver"),
    ("truck drivers", "hgv driver"),
    ("binman", "refuse collector"),
    ("bin man", "refuse collector"),
    ("binmen", "refuse collector"),
    ("bin men", "refuse collector"),
    ("refuse collector", "refuse collector"),
    ("refuse collectors", "refuse collector"),
];
pub(super) const WORK_WORDS: &[(&[&str], &str, &[&str])] = &[
    (
        &[
            "redundancy",
            "being made redundant",
            "made redundant",
            "redundant",
        ],
        "Redundancy is a dismissal because the employer no longer needs the role or needs fewer people doing that work. In Great Britain, eligible employees normally qualify for statutory redundancy pay after two years with the employer. The amount depends on age, completed years of service and capped weekly pay. Use the GOV.UK calculator for the limit that applies to the dismissal date.",
        &[
            "how much",
            "how do i claim",
            "claim",
            "my employer",
            "i have been",
            "i was",
            "voluntary",
            "offer",
            "payout",
            "calculate",
            "calculator",
            "notice",
            "letter",
            "consultation",
        ],
    ),
    (
        &[
            "performance review",
            "appraisal",
            "one to one",
            "1 to 1",
            "probation review",
        ],
        "A performance review is the regular sit-down where your manager says how the job is going and what changes, usually once or twice a year, with a rating that decides any pay rise. Go in with three things you did that made a difference and one thing you want to learn, in numbers where you can, and ask for the rating and the pay in the same conversation.",
        &[
            "how do i prepare",
            "prepare",
            "what should i say",
            "nervous",
            "my review",
            "went badly",
            "bad review",
            "salary",
            "pay rise",
            "ask for",
        ],
    ),
    (
        &["btec", "btecs", "btec diploma", "btec extended diploma"],
        "A BTEC is a vocational qualification taken at school or college, sometimes alongside A levels. BTEC Nationals come in different sizes and use a mix of assessment methods depending on the qualification. Check the exact award and grade in the UCAS Tariff calculator, and check whether the university course accepts it.",
        &[
            "how do i", "should i", "worth it", "my", "results", "grade", "ucas", "points",
            "apply", "college",
        ],
    ),
    (
        &[
            "foundation year",
            "foundation course",
            "foundation degree",
            "access course",
        ],
        "A foundation year is the extra year at the start of a degree for someone without the usual grades or the right subjects: you do a year of the basics, and passing it moves you onto the first year of the degree. It is not the same as a foundation degree, which is a two-year qualification of its own, or an Access course, which is a year at a college for adults going to university.",
        &[
            "how do i",
            "should i",
            "worth it",
            "my",
            "student loan",
            "fees",
            "apply",
            "ucas",
            "which university",
        ],
    ),
    (
        &["personal statement", "ucas statement"],
        "A UCAS personal statement explains why you want the course and how your studies and other experience have prepared you. From 2026 entry, UCAS asks three questions. The combined limit is 4,000 characters including spaces, and each answer needs at least 350. Use specific examples and ask someone who knows the course to read it.",
        &[
            "how do i write",
            "how to write",
            "help me write",
            "start",
            "example",
            "opening",
            "my personal statement",
            "draft",
            "word count",
            "characters",
            "ucas",
            "for a job",
            "job application",
            "job interview",
            "cover letter",
            "applying for a job",
        ],
    ),
    (
        &["gcse", "gcsEs", "gcses"],
        "GCSEs are qualifications commonly taken around age 16. In England the grades run from 9 to 1, with 9 highest, 4 a standard pass and 5 a strong pass. Grading systems differ in Wales and Northern Ireland. Maths and English grades often matter for entry to a sixth form, college course or apprenticeship.",
        &[
            "how do i revise",
            "revise",
            "revising",
            "results",
            "grades",
            "resit",
            "retake",
            "how many",
            "which",
            "choose",
            "my",
            "equivalent",
            "entry requirements",
            "do i need",
            "need to be",
            "need for",
            "to become",
            "to be a",
            "to be an",
            "require",
        ],
    ),
    (
        &["a level", "a levels", "as level", "as levels"],
        "A levels are the two-year exams taken at 16 to 18 in England, Wales and Northern Ireland, usually three subjects, graded A star to E. Each is mostly decided by exams at the end of the second year, and three good A level grades are the standard way into a degree.",
        &[
            "how do i revise",
            "revise",
            "revising",
            "results",
            "grades",
            "resit",
            "retake",
            "how many",
            "which",
            "choose",
            "my",
            "equivalent",
            "entry requirements",
            "ucas points",
            "level 1",
            "level 2",
            "level 3",
            "level 4",
            "level 5",
            "grade",
            "grades",
            "do i need",
            "need to be",
            "to be a",
            "to be an",
            "require",
        ],
    ),
];
pub(super) const DAY_WORDS: &[&str] = &[
    "what does",
    "what do",
    "what is it like",
    "whats it like",
    "day to day",
    "all day",
    "a typical day",
    "typical day",
    "actually do",
    "actually does",
    "is being a",
    "is being an",
    "hard work",
    "like being",
    "the job like",
    "what happens in",
    "what is a day",
];
