fn check_vec(vec: &Vec<i32>) -> bool {
    vec.windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>()
        .windows(2)
        .all(|window| {
            let abs0 = window[0].abs();
            let abs1 = window[1].abs();
            abs0 >= 1
                && abs0 <= 3
                && abs1 >= 1
                && abs1 <= 3
                && window[0].signum() == window[1].signum()
        })
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let vec = line
                .split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect::<Vec<i32>>();
            check_vec(&vec)
        })
        .count() as u32
}

fn generate_removals(vec: &Vec<i32>) -> Vec<Vec<i32>> {
    (0..vec.len())
        .map(|i| {
            vec.iter()
                .enumerate()
                .filter(|&(index, _)| index != i)
                .map(|(_, &value)| value)
                .collect()
        })
        .collect()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let vec = line
                .split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            check_vec(&vec)
                || generate_removals(&vec)
                    .into_iter()
                    .any(|vec: Vec<i32>| check_vec(&vec))
        })
        .count() as u32
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
