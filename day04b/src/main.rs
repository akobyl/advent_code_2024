pub fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] != 'A' {
                continue;
            }
            let is_ms = |c1: char, c2: char| (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M');

            if is_ms(grid[i + 1][j + 1], grid[i - 1][j - 1])
                && is_ms(grid[i + 1][j - 1], grid[i - 1][j + 1])
            {
                count += 1;
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4a() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(input), "9");
    }
}
