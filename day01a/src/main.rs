use std::collections::BinaryHeap;

pub fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> String {
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        heap1.push(numbers[0].parse::<i32>().unwrap());
        heap2.push(numbers[1].parse::<i32>().unwrap());
    }

    let mut sum = 0;
    for _i in 0..heap1.len() {
        sum += (heap1.pop().unwrap() - heap2.pop().unwrap()).abs();
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
        let result = part1(input);
        assert_eq!(result, "11");
    }
}
