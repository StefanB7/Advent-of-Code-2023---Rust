fn main() {
    let input = include_str!("../../input1.txt");
    let result = part1(input);
    dbg!(result);
}

fn part1(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = part1("");
        assert_eq!(result, "4".to_string());
    }
}
