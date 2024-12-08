pub fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let output = part2(input);
    println!("{}", output);
}

fn is_safe(row: &[i32], skip: Option<usize>) -> bool {
    let new_row: Vec<_> = row
        .iter()
        .enumerate()
        .filter(|&(i, _)| skip.map_or(true, |s| i != s))
        .map(|(_, &x)| x)
        .collect();

    let mut increasing = true;
    let mut decreasing = true;

    for w in new_row.windows(2) {
        let (prev, curr) = (w[0], w[1]);

        if curr <= prev || curr > prev + 3 {
            increasing = false;
        }
        if curr >= prev || curr < prev - 3 {
            decreasing = false;
        }
        if !increasing && !decreasing {
            break;
        }
    }
    increasing || decreasing
}

fn part2(input: &str) -> String {
    let count = input
        .lines()
        .map(|row| {
            row.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|row| is_safe(row, None) || (0..row.len()).any(|i| is_safe(row, Some(i))))
        .count();

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part2(input), "4");
    }
}
