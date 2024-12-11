pub fn main() {
    let input = include_str!("../../inputs/day03.txt");
    let output = part2(input);
    println!("{}", output);
}

#[derive(Debug)]
enum EnabledState {
    Searching,
    Preamble(usize),
    Do,
    Dont(usize),
}

#[derive(Debug)]
enum ParserState {
    Searching,
    Mul(usize),
    OpenParenthesis,
    FirstNumber(u32),
    Comma,
    SecondNumber(u32),
    CloseParenthesis,
}

fn check_for_mul(c: char) -> ParserState {
    if c == 'm' {
        ParserState::Mul(1)
    } else {
        ParserState::Searching
    }
}

fn part2(input: &str) -> String {
    let mul_str = "mul";
    let preamble_str = "do";
    let dont_str = "don't()";
    let mut current_num = 0;
    let mut state = ParserState::Searching;
    let mut enabled_state = EnabledState::Searching;
    let mut first_number = 0;
    let mut second_number = 0;
    let mut machine_enabled = true;

    let mut output = 0;

    for c in input.chars() {
        enabled_state = match enabled_state {
            EnabledState::Searching => {
                if c == 'd' {
                    EnabledState::Preamble(1)
                } else {
                    EnabledState::Searching
                }
            }
            EnabledState::Preamble(loc) => {
                if loc < preamble_str.len() && c == preamble_str.as_bytes()[loc] as char {
                    EnabledState::Preamble(loc + 1)
                } else if loc == preamble_str.len() && c == '(' {
                    EnabledState::Do
                } else if loc == preamble_str.len() && c == 'n' {
                    EnabledState::Dont(3)
                } else {
                    EnabledState::Searching
                }
            }
            EnabledState::Do => {
                if c == ')' {
                    machine_enabled = true;
                }
                EnabledState::Searching
            }
            EnabledState::Dont(loc) => {
                if loc < dont_str.len() && c == dont_str.as_bytes()[loc] as char {
                    EnabledState::Dont(loc + 1)
                } else if loc == dont_str.len() {
                    machine_enabled = false;
                    EnabledState::Searching
                } else {
                    EnabledState::Searching
                }
            }
        };
        state = match state {
            ParserState::Searching => check_for_mul(c),
            ParserState::Mul(loc) => {
                if loc < mul_str.len() && c == mul_str.as_bytes()[loc] as char {
                    ParserState::Mul(loc + 1)
                } else if loc == mul_str.len() && c == '(' {
                    ParserState::OpenParenthesis
                } else {
                    ParserState::Searching
                }
            }
            ParserState::OpenParenthesis => {
                if c.is_ascii_digit() {
                    current_num = c.to_digit(10).unwrap();
                    ParserState::FirstNumber(current_num)
                } else {
                    ParserState::Searching
                }
            }
            ParserState::FirstNumber(num) => {
                if c.is_ascii_digit() {
                    current_num = num * 10 + c.to_digit(10).unwrap();
                    ParserState::FirstNumber(current_num)
                } else if c == ',' {
                    first_number = current_num;
                    ParserState::Comma
                } else {
                    ParserState::Searching
                }
            }
            ParserState::Comma => {
                if c.is_ascii_digit() {
                    current_num = c.to_digit(10).unwrap();
                    ParserState::SecondNumber(current_num)
                } else {
                    ParserState::Searching
                }
            }
            ParserState::SecondNumber(num) => {
                if c.is_ascii_digit() {
                    current_num = num * 10 + c.to_digit(10).unwrap();
                    ParserState::SecondNumber(current_num)
                } else if c == ')' {
                    second_number = current_num;
                    ParserState::CloseParenthesis
                } else {
                    ParserState::Searching
                }
            }
            ParserState::CloseParenthesis => {
                if machine_enabled {
                    output += first_number * second_number;
                }
                check_for_mul(c)
            }
        };
    }

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), "48");
    }
}
