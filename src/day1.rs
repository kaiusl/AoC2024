use std::collections::HashMap;

const DAY: u8 = 1;
pub const INPUT: &str = include_str!("../inputs/day1.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

fn solve_part1(input: &str) -> u64 {
    let (mut l, mut r) = parse_part1(input);
    l.sort();
    r.sort();

    l.into_iter().zip(r.into_iter()).map(|(l, r)| l.abs_diff(r)).sum::<u64>()
}


fn parse_part1(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let (l, r) = line.split_once(' ').unwrap();
        left.push(l.trim().parse().unwrap());
        right.push(r.trim().parse().unwrap());
    }

    (left, right)
}

fn solve_part2(input: &str) -> u64 {
    let mut left = HashMap::<u64,u64>::new();
    let mut right = HashMap::<u64, u64>::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let (l, r) = line.split_once(' ').unwrap();
        let l = l.trim().parse().unwrap();
        let r = r.trim().parse().unwrap();

        left.entry(l).and_modify(|v| *v += 1).or_insert(1);
        right.entry(r).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut sum = 0;
    for (value, &left_count) in &left {
       if let Some(&mut right_count) = right.get_mut(value) {
           sum += value*left_count*right_count;
       }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 11);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 31);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 1320851);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 26859182);
    }
}
