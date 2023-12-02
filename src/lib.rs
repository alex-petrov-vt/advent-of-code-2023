pub mod day1 {
    pub fn part1(lines: std::str::Lines) -> i32 {
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
}
