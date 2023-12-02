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

pub mod day2 {
    use std::cmp;

    #[derive(Debug)]
    struct Game {
        id: i32,
        max_red: i32,
        max_green: i32,
        max_blue: i32,
    }

    impl Game {
        fn is_possible(&self, contents: (i32, i32, i32)) -> bool {
            return self.max_red <= contents.0
                && self.max_green <= contents.1
                && self.max_blue <= contents.2;
        }

        fn power_set(&self) -> i32 {
            return self.max_red * self.max_green * self.max_blue;
        }
    }

    pub fn part1(lines: std::str::Lines) -> i32 {
        let mut total: i32 = 0;
        let contents = (12, 13, 14);

        for line in lines {
            let game = load_game(line);

            if game.is_possible(contents) {
                total += game.id;
            }
        }

        total
    }

    pub fn part2(lines: std::str::Lines) -> i32 {
        let mut total: i32 = 0;

        for line in lines {
            let game = load_game(line);
            total += game.power_set();
        }

        total
    }

    fn load_game(line: &str) -> Game {
        let input: Vec<&str> = line.split(":").collect();

        let mut game = Game {
            id: input[0][5..input[0].len()].parse::<i32>().unwrap(),
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };

        let contents: Vec<&str> = input[1].split(";").collect();
        for content in contents {
            let counts: Vec<&str> = content.split(",").collect();
            for count in counts {
                let parts: Vec<&str> = count.split_whitespace().collect();

                let cubes = parts[0].parse::<i32>().unwrap();

                match parts[1] {
                    "red" => game.max_red = cmp::max(game.max_red, cubes),
                    "green" => game.max_green = cmp::max(game.max_green, cubes),
                    "blue" => game.max_blue = cmp::max(game.max_blue, cubes),
                    _ => {}
                }
            }
        }

        game
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

    #[test]
    fn day2_par1() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        assert_eq!(crate::day2::part1(input.lines()), 8)
    }

    #[test]
    fn day2_par2() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        assert_eq!(crate::day2::part2(input.lines()), 2286)
    }
}
