use utils;


fn parse_input(txt: &str) -> Vec<u64> {
    txt.split_whitespace()
    .map(|word| {
        word.parse::<u64>().unwrap()
    }).collect()
}

enum Either {
    NUM(u64),
    SPLIT(u64, u64),
}

fn apply_rule(num: u64) -> Either {
    if num == 0 {
        return Either::NUM(1);
    } else {
        let digits = count_digits(num);
        if digits % 2 == 0 {
        let (l, r) = split_number(num, digits);
            return Either::SPLIT(l, r);
        } else {
            return Either::NUM(num*2024);
        }
    }
}

fn count_digits(num: u64) -> u64 {
    let mut base = 10;
    let mut digits = 1;
    while num/base != 0 {
        base *= 10;
        digits += 1;
    }
    return digits;
}

fn split_number(num: u64, digits: u64) -> (u64, u64) {
    let midpoint_base: u64 = 10_u64.pow(digits as u32/2);
    let right = num % midpoint_base;
    let left = (num-right)/midpoint_base;
    return (left, right);
}

fn blink_transform(list: &[u64]) -> Vec<u64> {
    list.into_iter()
    .map( |&num| {
        match apply_rule(num) {
            Either::NUM(c) => vec![c],
            Either::SPLIT(l, r) => vec![l, r]
        }
    }).flatten()
    .collect()
}


fn solve(pb: &[u64], blink_times: u32) -> usize {
    let mut input = Vec::from(pb);
    for _ in 0..blink_times {
        input = blink_transform(&input)
    }
    return input.len()
}

pub fn day11() {
    let text = utils::read_file_as_text("inputs/day11.txt");
    let pb = parse_input(&text);
    let soln = solve(&pb, 25);
    println!("Solution to day 11 part 1: {}", soln); // 189167
    //let soln2 = solve(&pb, 75);
    //println!("Solution to day 11 part 2: {}", soln2);
}

#[cfg(test)]
mod test {
    use super::{blink_transform, count_digits, parse_input, solve, split_number};

    #[test]
    fn test_digits_count() {
        assert_eq!(count_digits(100), 3);
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(10), 2);
    }

    #[test]
    fn test_split_digits() {
        let (mut left, mut right) = split_number(1234, 4);
        assert_eq!(left, 12);
        assert_eq!(right, 34);
        (left, right) = split_number(10, 2);
        assert_eq!(left, 1);
        assert_eq!(right, 0);
    }
    #[test]
    fn test_one_blink() {
        let text = r"0 1 10 99 999";
        let pb = parse_input(&text);
        let output = blink_transform(&pb);
        assert_eq!(output, vec![1,2024,1,0,9,9,2021976])
    }

    #[test]
    fn test_example() {
        let text = r"125 17";
        let pb = parse_input(&text);
        assert_eq!(solve(&pb, 25), 55312);
        //assert_eq!(solve_pt2(&pb), 81);
    }
}