//! Pinned transit timetable choices over exact completed prior exchanges.

const ANSWER_1: &str = "Find your stop's row first and read across it: the minutes are printed under the hour, so a column is one bus and a row is one stop. The letters next to a time are notes - schooldays only, not Saturdays, bank holidays - and there is usually a separate table for Sunday. Be at the stop five minutes early, because a bus that comes every half hour is the one you cannot afford to miss, and check live times on the operator's app before you set off: the printed table is the plan, not what is happening.";
const ANSWER_2: &str = "Read a timetable by column, not by line: each column is one journey, so run your finger down it to see the time at your stop and the one after. Times after noon are often in 24-hour form (17:40 is 5:40 in the evening), the small letters beside a time are explained in the key at the bottom - S for schooldays, NS for not Sundays - and Sunday and bank holiday services have their own table with the last bus much earlier. The operator's app or the display at the stop is the only thing that knows about today's delays.";

pub(super) const CASES: &[(&[&str], &str)] = &[
    (&[], ANSWER_1),
    (&["Got it."], ANSWER_2),
    (&[ANSWER_1], ANSWER_2),
    (&[ANSWER_2], ANSWER_1),
    (&[ANSWER_1, ANSWER_2], ANSWER_1),
    (&[ANSWER_2, ANSWER_1], ANSWER_2),
    (&[ANSWER_1, ANSWER_2, "What else?"], ANSWER_1),
    (&[ANSWER_2, ANSWER_1, "What else?"], ANSWER_2),
    (&["hi", "hello", "got it"], ANSWER_2),
    (&["Got it.", "Hello there"], ANSWER_2),
];
