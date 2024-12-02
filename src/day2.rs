trait VecValidation {
    fn is_valid(&self) -> bool;
}

impl VecValidation for [i16] {
    fn is_valid(&self) -> bool {
        let mut diffs = self.windows(2).map(|window| window[1] - window[0]);
        let mut prev = match diffs.next() {
            Some(diff) if (1..=3).contains(&diff.abs()) => diff,
            _ => return false,
        };

        diffs.all(|diff| {
            let abs = diff.abs();
            let valid = (1..=3).contains(&abs) && diff.signum() == prev.signum();
            prev = diff;
            valid
        })
    }
}

fn faster_parse(s: &str) -> i16 {
    let mut num = 0;
    for c in s.as_bytes() {
        num = num * 10 + (c - b'0') as i16;
    }
    num
}

fn parse_line_to_array(line: &str) -> ([i16; 8], usize) {
    let mut arr = [0; 8];
    let mut count = 0;

    for (i, num_str) in line.split_whitespace().enumerate() {
        if i >= 8 {
            break;
        }
        arr[i] = faster_parse(num_str);
        count += 1;
    }

    (arr, count)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u16 {
    input
        .lines()
        .filter(|line| {
            let (arr, count) = parse_line_to_array(line);
            arr[..count].is_valid()
        })
        .count() as u16
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u16 {
    input
        .lines()
        .filter(|line| {
            let (arr, count) = parse_line_to_array(line);
            let slice = &arr[..count];
            slice.is_valid()
                || (0..count).any(|i| {
                    let mut reduced_arr = [0; 8];
                    let mut k = 0;

                    for j in 0..count {
                        if j != i {
                            reduced_arr[k] = arr[j];
                            k += 1;
                        }
                    }

                    reduced_arr[..(count - 1)].is_valid()
                })
        })
        .count() as u16
}
