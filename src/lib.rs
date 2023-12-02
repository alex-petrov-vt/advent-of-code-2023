pub mod day1 {
    use std::collections::HashMap;

    pub fn part1(lines: std::str::Lines) -> i32 {
        let mut total: i32 = 0;

        for line in lines {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            let first_last = String::from_iter(vec![digits[0], digits[digits.len() - 1]]);
            total += first_last.parse::<i32>().unwrap()
        }

        total
    }

    pub fn part2(lines: std::str::Lines) -> i32 {
        let digits_map = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);

        let mut total: i32 = 0;

        for line in lines {
            let mut digits: Vec<char> = vec![];

            for (i, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    digits.push(c);
                }

                for j in 3..6 {
                    if i >= (j - 1) && digits_map.contains_key(&line[(1 + i - j)..(i + 1)]) {
                        digits.push(
                            digits_map
                                .get(&line[(1 + i - j)..(i + 1)])
                                .unwrap()
                                .to_owned(),
                        )
                    }
                }
            }

            let first_last = String::from_iter(vec![digits[0], digits[digits.len() - 1]]);
            total += first_last.parse::<i32>().unwrap()
        }

        total
    }
}

mod tests {
    #[test]
    fn day1_part1() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        assert_eq!(crate::day1::part1(input.lines()), 142)
    }

    #[test]
    fn day1_part2() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        assert_eq!(crate::day1::part2(input.lines()), 281)
    }
}
