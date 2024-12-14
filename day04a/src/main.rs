pub fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let output = part1(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    DiagUpRight,
    DiagUpLeft,
    DiagDownRight,
    DiagDownLeft,
}

impl Direction {
    fn get(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::DiagUpRight => (1, 1),
            Direction::DiagUpLeft => (1, -1),
            Direction::DiagDownRight => (-1, 1),
            Direction::DiagDownLeft => (-1, -1),
        }
    }

    fn iter() -> &'static [Direction] {
        &[
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
            Direction::DiagUpRight,
            Direction::DiagUpLeft,
            Direction::DiagDownRight,
            Direction::DiagDownLeft,
        ]
    }
}

fn part1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let xmas = "XMAS";

    // queue: row, col,
    let mut stack = Vec::new();
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                for dir in Direction::iter() {
                    stack.push((i, j, 1, dir));
                }
            }
        }
    }

    while let Some((i, j, index, direction)) = stack.pop() {
        if index == xmas.len() {
            count += 1;
            continue;
        }

        // Continue in the direction
        let (ii, jj) = (i as i32 + direction.get().0, j as i32 + direction.get().1);

        if ii < 0 || ii >= grid.len() as i32 || jj < 0 || jj >= grid[0].len() as i32 {
            continue;
        }

        if grid[ii as usize][jj as usize] == xmas.chars().nth(index).unwrap() {
            stack.push((ii as usize, jj as usize, index + 1, direction));
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
        assert_eq!(part1(input), "18");
    }
}
