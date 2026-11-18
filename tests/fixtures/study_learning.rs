//! Direct PyBot callback fixtures from the pinned migration reference.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "grade-meaning",
        "Is GCSE grade 4 good?",
        &[],
        Some(
            "A 4 is a standard pass, the old C: you are over the line most colleges ask for, and a 5 is the strong pass above it. English and maths are the two that colleges and employers look at first, whatever else you take.",
        ),
    ),
    Case(
        "grade-meaning",
        "Is 6 a good pass in GCSE maths?",
        &[],
        Some(
            "A 6 is a good pass, about the old B. English and maths are the two that colleges and employers look at first, whatever else you take.",
        ),
    ),
    Case(
        "grade-meaning",
        "What does a 9 mean?",
        &[],
        Some(
            "A 9 is the top grade, above the old A*. English and maths are the two that colleges and employers look at first, whatever else you take.",
        ),
    ),
    Case(
        "grade-meaning",
        "I got a 1 in maths",
        &[],
        Some(
            "A 1 is below a pass at GCSE. Most sixth forms and colleges take you on other subjects and have you retake English or maths alongside, and that retake is free until you are 18, so it does not close anything on its own.",
        ),
    ),
    Case(
        "grade-meaning",
        "I got a 2 in English",
        &[],
        Some(
            "A 2 is below a pass at GCSE. Most sixth forms and colleges take you on other subjects and have you retake English or maths alongside, and that retake is free until you are 18, so it does not close anything on its own.",
        ),
    ),
    Case(
        "grade-meaning",
        "I got a 3 in English",
        &[],
        Some(
            "A 3 is below a pass at GCSE. Most sixth forms and colleges take you on other subjects and have you retake English or maths alongside, and that retake is free until you are 18, so it does not close anything on its own.",
        ),
    ),
    Case(
        "grade-meaning",
        "I got a 5 in maths",
        &[],
        Some(
            "A 5 is a strong pass, the old high C, and over the line most colleges ask for. English and maths are the two that colleges and employers look at first, whatever else you take.",
        ),
    ),
    Case(
        "grade-meaning",
        "I got a 7 in maths",
        &[],
        Some(
            "A 7 is a top grade, the old A and above. English and maths are the two that colleges and employers look at first, whatever else you take.",
        ),
    ),
    Case(
        "grade-meaning",
        "I got an 8 in maths",
        &[],
        Some(
            "An 8 is a top grade, the old A and above. English and maths are the two that colleges and employers look at first, whatever else you take.",
        ),
    ),
    Case(
        "grade-meaning",
        "My daughter got 3 A stars in her GCSEs",
        &[],
        None,
    ),
    Case("grade-meaning", "My son got straight 9s", &[], None),
    Case("grade-meaning", "I got 2 A levels and a BTEC", &[], None),
    Case(
        "grade-meaning",
        "I took 6 subjects and got a 7 in physics",
        &[],
        Some(
            "A 7 is a top grade, the old A and above. English and maths are the two that colleges and employers look at first, whatever else you take.",
        ),
    ),
    Case(
        "grade-meaning",
        "What are GCSE grades?",
        &[],
        Some(
            "GCSE grades run from 9 down to 1: 4 is a standard pass, 5 a strong one, and 7 to 9 are the top grades. Below 4 is not a pass yet, and English and maths can be retaken at college until you are 18. Tell me the grade and the subject and I will say what it opens up.",
        ),
    ),
    Case(
        "grade-meaning",
        "What does 4 out of 9 mean in my homework?",
        &[],
        Some(
            "That is 4 out of 9, about half the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case(
        "grade-meaning",
        "5 marks out of 10",
        &[],
        Some(
            "That is 5 out of 10, about half the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case(
        "grade-meaning",
        "12 out of 30 on the maths test",
        &[],
        Some(
            "That is 12 out of 30, about half the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case(
        "grade-meaning",
        "4 out of 200",
        &[],
        Some(
            "That is 4 out of 200, under half the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case(
        "grade-meaning",
        "121 out of 200",
        &[],
        Some(
            "That is 121 out of 200, about half the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case(
        "grade-meaning",
        "123 out of 200",
        &[],
        Some(
            "That is 123 out of 200, most the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case(
        "grade-meaning",
        "79 out of 200",
        &[],
        Some(
            "That is 79 out of 200, about half the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case(
        "grade-meaning",
        "78 out of 200",
        &[],
        Some(
            "That is 78 out of 200, under half the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case(
        "grade-meaning",
        "200 out of 200",
        &[],
        Some(
            "That is 200 out of 200, most the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case("grade-meaning", "199 out of 201", &[], None),
    Case("grade-meaning", "4 out of 4", &[], None),
    Case("grade-meaning", "9 out of 8", &[], None),
    Case("grade-meaning", "0 out of 200", &[], None),
    Case(
        "grade-meaning",
        "4% is the score",
        &[],
        Some(
            "GCSE grades run from 9 down to 1: 4 is a standard pass, 5 a strong one, and 7 to 9 are the top grades. Below 4 is not a pass yet, and English and maths can be retaken at college until you are 18. Tell me the grade and the subject and I will say what it opens up.",
        ),
    ),
    Case("grade-meaning", "I got 6 As and 3 Bs", &[], None),
    Case(
        "grade-meaning",
        "I got a 4 as well",
        &[],
        Some(
            "A 4 is a standard pass, the old C: you are over the line most colleges ask for, and a 5 is the strong pass above it. English and maths are the two that colleges and employers look at first, whatever else you take.",
        ),
    ),
    Case(
        "grade-meaning",
        "I got a grade of 8",
        &[],
        Some(
            "GCSE grades run from 9 down to 1: 4 is a standard pass, 5 a strong one, and 7 to 9 are the top grades. Below 4 is not a pass yet, and English and maths can be retaken at college until you are 18. Tell me the grade and the subject and I will say what it opens up.",
        ),
    ),
    Case(
        "grade-meaning",
        "٤ out of ١٠ in my homework",
        &[],
        Some(
            "That is 4 out of 10, about half the marks. Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time.",
        ),
    ),
    Case("grade-meaning", "What are two prime numbers?", &[], None),
    Case(
        "mock-grade",
        "I got a 6 in my GCSE mock, can I go to university?",
        &[],
        Some(
            "A 6 is a good pass, about the old B. A mock or a predicted grade is a signal, not a verdict: find out which topics cost you the marks — the mark scheme for a mock, your teacher for a predicted grade — and give the worst two a 25-minute slot each week. Moving a grade or two in a term is normal. For the university question, what counts is what the course asks for: check two or three you like and their entry requirements, because a 6 at GCSE rarely closes a door on its own.",
        ),
    ),
    Case(
        "mock-grade",
        "I got 4 out of 9 in the mock",
        &[],
        Some(
            "That is 4 out of 9, about half the marks. A mock or a predicted grade is a signal, not a verdict: find out which topics cost you the marks — the mark scheme for a mock, your teacher for a predicted grade — and give the worst two a 25-minute slot each week. Moving a grade or two in a term is normal.",
        ),
    ),
    Case(
        "mock-grade",
        "What does my predicted grade mean?",
        &[],
        Some(
            "A mock or a predicted grade is a signal, not a verdict: find out which topics cost you the marks — the mark scheme for a mock, your teacher for a predicted grade — and give the worst two a 25-minute slot each week. Moving a grade or two in a term is normal.",
        ),
    ),
    Case(
        "mock-grade",
        "I got 4 out of 9 and a 6 in the mock, will college accept me?",
        &[],
        Some(
            "A 6 is a good pass, about the old B. That is 4 out of 9, about half the marks. A mock or a predicted grade is a signal, not a verdict: find out which topics cost you the marks — the mark scheme for a mock, your teacher for a predicted grade — and give the worst two a 25-minute slot each week. Moving a grade or two in a term is normal. For the university question, what counts is what the course asks for: check two or three you like and their entry requirements, because a 6 at GCSE rarely closes a door on its own.",
        ),
    ),
    Case(
        "mock-grade",
        "What does 3 marks out of 10 in my mock mean for sixth form?",
        &[],
        Some(
            "That is 3 out of 10, under half the marks. A mock or a predicted grade is a signal, not a verdict: find out which topics cost you the marks — the mark scheme for a mock, your teacher for a predicted grade — and give the worst two a 25-minute slot each week. Moving a grade or two in a term is normal. For the university question, what counts is what the course asks for: check two or three you like and their entry requirements, because a 6 at GCSE rarely closes a door on its own.",
        ),
    ),
    Case(
        "revision-start",
        "How do I start revising for a test?",
        &[],
        Some(
            "Start with something tiny today: open the past paper or the topic list and write down what you already know about one topic — ten minutes, that is the whole job. Then work backwards from the exam date and give each topic a day, in 25-minute blocks with a 5-minute break. The last week should be past papers under time, not reading notes.",
        ),
    ),
    Case("revision-start", "How do I test a car battery?", &[], None),
    Case(
        "revision-start",
        "How do I test my progress in maths?",
        &[],
        Some(
            "Start with something tiny today: open the past paper or the topic list and write down what you already know about one topic — ten minutes, that is the whole job. Then work backwards from the exam date and give each topic a day, in 25-minute blocks with a 5-minute break. The last week should be past papers under time, not reading notes.",
        ),
    ),
    Case(
        "revision-start",
        "How should I revise chemistry?",
        &[],
        Some(
            "Start with something tiny today: open the past paper or the topic list and write down what you already know about one topic — ten minutes, that is the whole job. Then work backwards from the exam date and give each topic a day, in 25-minute blocks with a 5-minute break. The last week should be past papers under time, not reading notes.",
        ),
    ),
    Case(
        "revision-start",
        "How should I test a smoke alarm?",
        &[],
        None,
    ),
    Case(
        "revision-start",
        "How do I test it tomorrow?",
        &[],
        Some(
            "Start with something tiny today: open the past paper or the topic list and write down what you already know about one topic — ten minutes, that is the whole job. Then work backwards from the exam date and give each topic a day, in 25-minute blocks with a 5-minute break. The last week should be past papers under time, not reading notes.",
        ),
    ),
    Case(
        "revision-start",
        "Test again the week before the exam",
        &[],
        Some(
            "Start with something tiny today: open the past paper or the topic list and write down what you already know about one topic — ten minutes, that is the whole job. Then work backwards from the exam date and give each topic a day, in 25-minute blocks with a 5-minute break. The last week should be past papers under time, not reading notes.",
        ),
    ),
    Case("revision-start", "Test for a water leak", &[], None),
    Case(
        "exam-nerves",
        "How do I deal with exam stress?",
        &[],
        Some(
            "Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "exam-nerves",
        "I go blank before an exam",
        &[],
        Some(
            "Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "exam-nerves",
        "My hands shake before the exam",
        &[],
        Some(
            "Shaky hands before an exam are adrenaline, not a bug: eat something plain, take water in with you, and breathe out slowly for a minute in the queue. Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "exam-nerves",
        "I have sweaty palms before an exam",
        &[],
        Some(
            "Clammy hands before an exam are adrenaline, not a bug: eat something plain, take water in with you, and breathe out slowly for a minute in the queue. Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "exam-nerves",
        "My stomach flips and my heart races before an exam",
        &[],
        Some(
            "A racing heart before an exam is adrenaline, not a bug: eat something plain, take water in with you, and breathe out slowly for a minute in the queue. Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "exam-nerves",
        "Shaky hands in an exam",
        &[],
        Some(
            "Shaky hands before an exam are adrenaline, not a bug: eat something plain, take water in with you, and breathe out slowly for a minute in the queue. Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "exam-nerves",
        "I feel sick before my exam",
        &[],
        Some(
            "Feeling sick before an exam is adrenaline, not a bug: eat something plain, take water in with you, and breathe out slowly for a minute in the queue. Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "exam-nerves",
        "Before a test I start trembling",
        &[],
        Some(
            "Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "exam-nerves",
        "My stomach is in knots before an exam",
        &[],
        Some(
            "Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.",
        ),
    ),
    Case(
        "study-age",
        "Am I too old to do a course?",
        &[],
        Some(
            "You would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case(
        "study-age",
        "Am I too old to do a course at 40?",
        &[],
        Some(
            "At 40 you would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case(
        "study-age",
        "Is 50 too old to become a nurse?",
        &[],
        Some(
            "At 50 you would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case(
        "study-age",
        "Is it too late to become an accountant at 70?",
        &[],
        Some(
            "At 70 you would be older than most of the class, and that is normal on these courses: what decides it is not the age but the routine - one fixed evening in class and one more for reading, kept for the length of the course - and a subject you will still open on a bad week. Ask the college what share of last year's class finished, and whether any of it can be done in the daytime or online, because that is what makes it stick.",
        ),
    ),
    Case(
        "study-age",
        "Is it too late to become an accountant at 90?",
        &[],
        Some(
            "At 90 you would be older than most of the class, and that is normal on these courses: what decides it is not the age but the routine - one fixed evening in class and one more for reading, kept for the length of the course - and a subject you will still open on a bad week. Ask the college what share of last year's class finished, and whether any of it can be done in the daytime or online, because that is what makes it stick.",
        ),
    ),
    Case(
        "study-age",
        "Am I too old at 91 for evening classes?",
        &[],
        Some(
            "You would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case(
        "study-age",
        "Can I still do a part-time degree at 18?",
        &[],
        Some(
            "At 18 you would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case(
        "study-age",
        "Could I still study at 17?",
        &[],
        Some(
            "You would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case("study-age", "Am I too old to have a baby?", &[], None),
    Case(
        "study-age",
        "Am I too old to start a marina business?",
        &[],
        None,
    ),
    Case("study-age", "Am I too old to learn guitar?", &[], None),
    Case(
        "study-age",
        "How do I get into university as a mature student?",
        &[],
        None,
    ),
    Case(
        "study-age",
        "Am I too old at 58?",
        &["I was thinking about an evening class."],
        Some(
            "At 58 you would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case(
        "study-age",
        "Is it still worth it at 60?",
        &["I was thinking about an evening class.", "I am busy."],
        Some(
            "At 60 you would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case(
        "study-age",
        "Is it still worth it at 61?",
        &[
            "I was thinking about an evening class.",
            "I am busy.",
            "I work here.",
        ],
        None,
    ),
    Case(
        "study-age",
        "I am considering college at 42.",
        &[],
        Some(
            "At 42 you would not be late: evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.",
        ),
    ),
    Case(
        "course-hours",
        "How many hours a week would it be?",
        &[],
        None,
    ),
    Case(
        "course-hours",
        "How many hours a week would that be?",
        &["I am considering an evening class."],
        Some(
            "Budget two to three hours of reading and homework for every hour in class: a three-hour evening class is usually 9 to 12 hours a week including the class, and a part-time degree is 16 to 20 hours a week because it is half of a full-time course. The college has to publish those guided-learning hours on the course page, so ask for them before you enrol and pick the two evenings you would protect. Reading is what gets dropped first, so put it in the diary as an appointment rather than a hope.",
        ),
    ),
    Case(
        "course-hours",
        "What would the hours be like?",
        &["Is 50 too old to become a nurse?"],
        Some(
            "Budget two to three hours of reading and homework for every hour in class: a three-hour evening class is usually 9 to 12 hours a week including the class, and a part-time degree is 16 to 20 hours a week because it is half of a full-time course. The college has to publish those guided-learning hours on the course page, so ask for them before you enrol and pick the two evenings you would protect. Reading is what gets dropped first, so put it in the diary as an appointment rather than a hope.",
        ),
    ),
    Case(
        "course-hours",
        "What hours would it be?",
        &["Could I do a part time degree?", "I work afternoons."],
        Some(
            "Budget two to three hours of reading and homework for every hour in class: a three-hour evening class is usually 9 to 12 hours a week including the class, and a part-time degree is 16 to 20 hours a week because it is half of a full-time course. The college has to publish those guided-learning hours on the course page, so ask for them before you enrol and pick the two evenings you would protect. Reading is what gets dropped first, so put it in the diary as an appointment rather than a hope.",
        ),
    ),
    Case(
        "course-hours",
        "What hours would it be?",
        &[
            "Could I do a part time degree?",
            "I work afternoons.",
            "I can meet you.",
        ],
        None,
    ),
    Case(
        "course-hours",
        "How many hours a week is a part-time degree?",
        &["Is 50 too old to become a nurse?"],
        None,
    ),
    Case(
        "course-hours",
        "How long is it?",
        &["I'm 29", "I might try an access course."],
        Some(
            "Budget two to three hours of reading and homework for every hour in class: a three-hour evening class is usually 9 to 12 hours a week including the class, and a part-time degree is 16 to 20 hours a week because it is half of a full-time course. The college has to publish those guided-learning hours on the course page, so ask for them before you enrol and pick the two evenings you would protect. Reading is what gets dropped first, so put it in the diary as an appointment rather than a hope.",
        ),
    ),
    Case(
        "course-hours",
        "Hours per week?",
        &["I lost my keys.", "I work from home."],
        None,
    ),
];
