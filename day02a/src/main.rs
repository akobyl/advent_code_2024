pub fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> String {
    let rows = input.lines().map(|row| {
        row.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut count = 0;

    for row in rows {
        let mut increasing = true;
        let mut decreasing = true;

        for i in 1..row.len() {
            if row[i] <= row[i - 1] || row[i] > (row[i - 1] + 3) {
                increasing = false;
            }
            if row[i] >= row[i - 1] || row[i] < (row[i - 1] - 3) {
                decreasing = false;
            }
        }
        if increasing || decreasing {
            count += 1;
        }
    }
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

        assert_eq!(part1(input), "2");
    }
}
