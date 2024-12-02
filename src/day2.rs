fn check_vec(vec: Vec<i32>) -> bool {
    vec.windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|window| {
            let abs0 = window[0].abs();
            let abs1 = window[1].abs();
            if abs0 > 3 || abs1 > 3 || abs0 < 1 || abs1 < 1 {
                false
            } else {
                if window[0].signum() != window[1].signum() {
                    false
                } else {
                    true
                }
            }
        })
        .all(|b| b == true)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut result = 0;

    input.lines().for_each(|l| {
        check_vec(
            l.split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect::<Vec<i32>>(),
        )
        .then(|| result += 1);
    });

    result
}

fn generate_removals(vec: Vec<i32>) -> Vec<Vec<i32>> {
    std::iter::once(vec.clone())
        .chain((0..vec.len()).map(|i| {
            vec.iter()
                .enumerate()
                .filter(|&(index, _)| index != i)
                .map(|(_, &value)| value)
                .collect()
        }))
        .collect()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut result = 0;

    input.lines().for_each(|l| {
        generate_removals(
            l.split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect::<Vec<i32>>(),
        )
        .into_iter()
        .map(check_vec)
        .any(|b| b == true)
        .then(|| result += 1);
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1example() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part2example() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

        assert_eq!(part2(input), 4);
    }
}
