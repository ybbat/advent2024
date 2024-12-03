fn parse_num(bytes: &[u8], i: usize) -> Option<(u32, usize)> {
    if bytes[i].is_ascii_digit() {
        if bytes[i + 1].is_ascii_digit() {
            if bytes[i + 2].is_ascii_digit() {
                return Some((
                    (bytes[i + 2] - b'0') as u32
                        + (bytes[i + 1] - b'0') as u32 * 10
                        + (bytes[i] - b'0') as u32 * 100,
                    i + 3,
                ));
            } else {
                return Some((
                    (bytes[i + 1] - b'0') as u32 + (bytes[i] - b'0') as u32 * 10,
                    i + 2,
                ));
            }
        } else {
            return Some(((bytes[i] - b'0') as u32, i + 1));
        }
    } else {
        return None;
    }
}

fn parse_mul(bytes: &[u8], mut i: usize) -> Option<(u32, usize)> {
    let first: u32;
    let second: u32;

    if bytes[i] == b'm' && bytes[i + 1] == b'u' && bytes[i + 2] == b'l' && bytes[i + 3] == b'(' {
        i += 4;
        match parse_num(bytes, i) {
            Some((x, new_i)) => {
                first = x;
                i = new_i;
            }
            None => return None,
        }
        if bytes[i] == b',' {
            i += 1;
        } else {
            return None;
        }
        match parse_num(bytes, i) {
            Some((x, new_i)) => {
                second = x;
                i = new_i;
            }
            None => return None,
        }
        if bytes[i] == b')' {
            return Some((first * second, i + 1));
        } else {
            return None;
        }
    } else {
        return None;
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut acc = 0;

    while i < bytes.len() {
        match parse_mul(bytes, i) {
            Some((x, new_i)) => {
                acc += x;
                i = new_i;
            }
            None => i += 1,
        }
    }

    return acc as u32;
}

fn parse_do(bytes: &[u8], i: usize) -> bool {
    return bytes[i..].starts_with(b"do()");
}

fn parse_dont(bytes: &[u8], i: usize) -> bool {
    return bytes[i..].starts_with(b"don't()");
}

enum State {
    Do,
    Dont,
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut acc = 0;
    let mut state = State::Do;

    while i < bytes.len() - 8 {
        match state {
            State::Do => {
                if parse_dont(bytes, i) {
                    i += 7;
                    state = State::Dont;
                } else {
                    match parse_mul(bytes, i) {
                        Some((x, new_i)) => {
                            acc += x;
                            i = new_i;
                        }
                        None => i += 1,
                    }
                }
            }
            State::Dont => {
                if parse_do(bytes, i) {
                    i += 4;
                    state = State::Do;
                } else {
                    i += 1;
                }
            }
        }
    }

    return acc as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn part2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&input), 48);
    }
}
