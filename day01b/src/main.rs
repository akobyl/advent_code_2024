use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> String {
    let mut map2 = HashMap::new();
    let mut list1 = Vec::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let number1 = numbers[0].parse::<u64>().unwrap();
        let number2 = numbers[1].parse::<u64>().unwrap();

        list1.push(number1);
        map2.entry(number2).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut sum = 0;
    for number in list1 {
        sum += number * *map2.entry(number).or_insert(0);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = part2(input);
        assert_eq!(result, "31");
    }
}
