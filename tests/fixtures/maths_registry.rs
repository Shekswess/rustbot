//! Direct pinned descriptive statistics and method-card guards.
pub(super) struct Case {
    pub id: &'static str,
    pub text: &'static str,
    pub guard: bool,
    pub reply: Option<&'static str>,
}
pub(super) const CASES: &[Case] = &[
    Case {
        id: "average-mean",
        text: "what does the median mean in maths",
        guard: true,
        reply: Some(
            "The median is the middle one once the numbers are in order: 1, 4 and 11 is 4, and 1, 4, 6 and 11 is halfway between 4 and 6, so 5. It is the one to use when a few big numbers would drag a mean up, like house prices or wages. The mean is the total shared out evenly, and the mode is the value that appears most often.",
        ),
    },
    Case {
        id: "average-mean",
        text: "what's a median",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "how do you find a mode",
        guard: true,
        reply: Some(
            "The mode is the value that appears most often: in 2, 3, 3 and 7 it is 3, and if nothing repeats there is no mode at all. It is the one to use for the commonest answer rather than a middle one, like the most usual shoe size. The mean is the total shared out evenly, and the median is the middle one once the numbers are in order.",
        ),
    },
    Case {
        id: "average-mean",
        text: "what do you mean",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "what does this mean",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "what's that mean",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "what is average rent in london",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "what does ordinal mean",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "how do you find the mean average",
        guard: true,
        reply: Some(
            "The mean, which is what most people mean by an average, is the total shared out evenly: 4, 6 and 11 add to 21, and 21 ÷ 3 = 7, so the mean is 7. The median is the middle one once they are in order (6 above), and the mode is the value that appears most often.",
        ),
    },
    Case {
        id: "average-mean",
        text: "what does mean mean",
        guard: true,
        reply: Some(
            "The mean, which is what most people mean by an average, is the total shared out evenly: 4, 6 and 11 add to 21, and 21 ÷ 3 = 7, so the mean is 7. The median is the middle one once they are in order (6 above), and the mode is the value that appears most often.",
        ),
    },
    Case {
        id: "average-mean",
        text: "what are the mean median and mode",
        guard: true,
        reply: Some(
            "The median is the middle one once the numbers are in order: 1, 4 and 11 is 4, and 1, 4, 6 and 11 is halfway between 4 and 6, so 5. It is the one to use when a few big numbers would drag a mean up, like house prices or wages. The mean is the total shared out evenly, and the mode is the value that appears most often.",
        ),
    },
    Case {
        id: "average-mean",
        text: "what is the difference between median and mean",
        guard: true,
        reply: Some(
            "The mean, which is what most people mean by an average, is the total shared out evenly: 4, 6 and 11 add to 21, and 21 ÷ 3 = 7, so the mean is 7. The median is the middle one once they are in order (6 above), and the mode is the value that appears most often.",
        ),
    },
    Case {
        id: "average-mean",
        text: "define median without a calculator",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "what is the mean average of 1 5 and 9",
        guard: true,
        reply: Some("The mean of 1, 5, 9 is 5: they add to 15 and 15 ÷ 3 = 5."),
    },
    Case {
        id: "average-mean",
        text: "calculate the median of 2, 10, 20 and 120",
        guard: true,
        reply: Some("The median of 2, 10, 20, 120 is 15: in order, that is the middle one."),
    },
    Case {
        id: "average-mean",
        text: "what is the mode of 3 3 5 5 2",
        guard: true,
        reply: Some("The mode of 2, 3, 3, 5, 5 is 3, the number that appears most often."),
    },
    Case {
        id: "average-mean",
        text: "work out the mode of 0.4, 0.40, 0.401 and 0.5",
        guard: true,
        reply: Some("The mode of 0.4, 0.4, 0.4, 0.5 is 0.4, the number that appears most often."),
    },
    Case {
        id: "average-mean",
        text: "median of 0.5 and 0.51",
        guard: true,
        reply: Some("The median of 0.5, 0.51 is 0.51: in order, that is the middle one."),
    },
    Case {
        id: "average-mean",
        text: "mode of 1, 2, 2 and 1",
        guard: true,
        reply: Some("The mode of 1, 1, 2, 2 is 1, the number that appears most often."),
    },
    Case {
        id: "average-mean",
        text: "what is the mode of 1 2 3",
        guard: true,
        reply: Some(
            "No number in 1, 2, 3 repeats, so there is no mode. The mean is 2 and the median is 2.",
        ),
    },
    Case {
        id: "average-mean",
        text: "what is the average of -4, 2 and -8",
        guard: true,
        reply: Some("The mean of 2, 4, 8 is 4.67: they add to 14 and 14 ÷ 3 = 4.67."),
    },
    Case {
        id: "average-mean",
        text: "average of abc23 and 4",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "what is the mean of 9999999999999999999999999999999 and 1111111111111111111111111111111",
        guard: true,
        reply: Some(
            "The mean of 1111111111111111111111111111111, 9999999999999999999999999999999 is 5555555555555555555555555555555: they add to 11111111111111111111111111111110 and 11111111111111111111111111111110 ÷ 2 = 5555555555555555555555555555555.",
        ),
    },
    Case {
        id: "average-mean",
        text: "what is the mean of 0.0001 and 0.0002",
        guard: true,
        reply: Some("The mean of 0, 0 is 0: they add to 0 and 0 ÷ 2 = 0."),
    },
    Case {
        id: "average-mean",
        text: "what is the mode of 0.125 0.125 0.375",
        guard: true,
        reply: Some("The mode of 0.12, 0.12, 0.38 is 0.12, the number that appears most often."),
    },
    Case {
        id: "average-mean",
        text: "what is the mean of ٣ and ٥",
        guard: true,
        reply: Some("The mean of 3, 5 is 4: they add to 8 and 8 ÷ 2 = 4."),
    },
    Case {
        id: "average-mean",
        text: "what is the median of １０.５ and １６.２",
        guard: true,
        reply: Some("The median of 10.5, 16.2 is 13.35: in order, that is the middle one."),
    },
    Case {
        id: "average-mean",
        text: "how do i work out average speed",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "my phone is in portrait mode",
        guard: true,
        reply: None,
    },
    Case {
        id: "average-mean",
        text: "mean of 2.66 2.67",
        guard: true,
        reply: Some("The mean of 2.66, 2.67 is 2.67: they add to 5.33 and 5.33 ÷ 2 = 2.67."),
    },
    Case {
        id: "average-mean",
        text: "what's the median of 1.0 and 2.01?",
        guard: true,
        reply: Some("The median of 1, 2.01 is 1.5: in order, that is the middle one."),
    },
    Case {
        id: "average-mean",
        text: "how does 1,000 and 2,000 average out",
        guard: true,
        reply: Some("The mean of 1000, 2000 is 1500: they add to 3000 and 3000 ÷ 2 = 1500."),
    },
    Case {
        id: "average-mean",
        text: "median of 9007199254740994 and 9007199254740993",
        guard: true,
        reply: Some(
            "The median of 9007199254740993, 9007199254740994 is 9007199254740994: in order, that is the middle one.",
        ),
    },
    Case {
        id: "average-mean",
        text: "mean of a 5 and a 6",
        guard: true,
        reply: Some("The mean of 5, 6 is 5.5: they add to 11 and 11 ÷ 2 = 5.5."),
    },
    Case {
        id: "rounding-how",
        text: "how do you round numbers",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "what are the rules for rounding",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "can you round the answer",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "round the total",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "how do i round the number",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "round the score",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "how do i round to 2 decimal places",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "where is the nearest cashpoint",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "what near the nearest decimal",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "how do you round ٢٫٤",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "round 18 to the nearest 10",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "what does round mean in maths",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "how do i fit this around the bath",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "what can i do round the back",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "how do i round off a whole number",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "how do people run round the office",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "the amount round up",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "how to round the digits up",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "work round the dog",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "which is the nearest chemist",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "round the circular hole",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "stair gate round a newel post",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "round to the nearest whole number",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "nearest pence",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "round it to the nearest",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "the nearest bus stop was 3 miles away",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "what are significant figures",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "how does rounding work",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "roundabouts",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "rounds the nearest",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "nearest 6",
        guard: true,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "it goes round the shop",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "rounding-how",
        text: "one round of golf",
        guard: false,
        reply: Some(
            "Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "what are brackets for?",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how do i use brackets",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "what is a bracket",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how do i expand the brackets",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how do i multiply out double brackets",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how do i expand and simplify algebra",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "when do i use brackets in prose",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how do i fit new brackets",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "drill 3 brackets into the wall",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how do i fix shelf brackets",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how to expand (x+2)(x+3)",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how do i work out brackets",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "what are the rules for negative brackets",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "brackets heading on 12 resume",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "understand the brackets",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "what are square brackets used for",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "how do i use a bracket next to ٣",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "why do numbers in brackets multiply",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "draw square brackets",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "my business needs to expand",
        guard: true,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
    Case {
        id: "expand-brackets",
        text: "dance bracket",
        guard: false,
        reply: Some(
            "Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.",
        ),
    },
];
