fn main() {
    let input = include_str!("../../input1.txt");
    let result = part1(input);
    dbg!(result);
}

fn get_game_num(line: &str) -> u32 {
    line.trim()
        .chars()
        .skip(5)
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn parse_ball_counts(line: &str) -> (u32, u32, u32) {
    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;

    for next_part in line.split(", ") {
        if next_part.contains("blue") {
            blue += next_part.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
        } else if next_part.contains("green") {
            green += next_part.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
        } else if next_part.contains("red") {
            red += next_part.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
        }
    }

    (blue, green, red)
}

fn part1(input: &str) -> u32 {
    let mut power_sum = 0;

    for line in input.lines() {
        let mut required_red = 0;
        let mut required_green = 0;
        let mut required_blue = 0;

        let mut line_temp = String::from(line);
        let game_num = get_game_num(&line_temp);
        line_temp = line.split(": ").nth(1).unwrap().to_string();
        for game in line_temp.split("; ") {
            let (blue, green, red) = parse_ball_counts(game);
            if blue > required_blue {
                required_blue = blue;
            }
            if green > required_green {
                required_green = green;
            }
            if red > required_red {
                required_red = red;
            }
        }

        let power = required_blue * required_green * required_red;
        println!("Game {}: {}", game_num, power);

        power_sum += power;
    }

    power_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
