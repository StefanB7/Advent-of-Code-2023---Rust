use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input1.txt");
    let result = part1(input);
    dbg!(result);
}

fn part1(input: &str) -> u64 {
    let mut number_map: HashMap<&str, i32> = HashMap::new();
    number_map.insert("one", 1);
    number_map.insert("two", 2);
    number_map.insert("three", 3);
    number_map.insert("four", 4);
    number_map.insert("five", 5);
    number_map.insert("six", 6);
    number_map.insert("seven", 7);
    number_map.insert("eight", 8);
    number_map.insert("nine", 9);
    number_map.insert("ten", 10);

    let number_words: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    let mut sum = 0;
    for line in input.lines() {
        let mut num = String::new();
        let mut buf = String::new();

        'outer: for c in line.chars() {
            buf.push(c);
            if c.is_digit(10) {
                num.push(c);
                break 'outer;
            }
            for &word in &number_words {
                if buf.contains(word) {
                    num.push_str(number_map.get(word).unwrap().to_string().as_str());
                    buf.clear();
                    break 'outer;
                }
            }
        }
        'outer: for c in line.chars().rev() {
            buf.insert(0, c);
            if c.is_digit(10) {
                num.push(c);
                break 'outer;
            }
            for word in number_words.iter() {
                if buf.contains(word) {
                    num.push_str(number_map.get(word).unwrap().to_string().as_str());
                    buf.clear();
                    break 'outer;
                }
            }
        }
        if num.len() > 0 {
            sum += num.parse::<u64>().unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = part1(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
