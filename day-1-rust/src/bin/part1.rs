fn main() {
    let input = include_str!("../../input1.txt");
    let result = part1(input);
    dbg!(result);
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut num = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                num.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                num.push(c);
                break;
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
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
