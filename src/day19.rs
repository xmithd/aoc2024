use std::fs;
use std::collections::HashMap;


#[derive(Debug)]
struct Puzzle {
   patterns: Vec<String>,
   desired_designs: Vec<String>,
}

fn parse_problem(txt: &str) -> Puzzle {
    let mut available: Vec<String> = vec![];
    let mut desired: Vec<String> = vec![];
    for line in txt.trim().split("\n") {
        if line.is_empty() {
            continue;
        }
        if line.contains(",") {
            available = line.split(",").map(|token| token.trim().to_string()).collect();
        } else {
            desired.push(line.trim().to_string());
        }
    }
    Puzzle {
        patterns: available,
        desired_designs: desired,
    }
}

fn count_all_combos<'a>(patterns: &[String], design: &'a str, cache: &HashMap<&'a str, bool>, count_cache: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(c) = count_cache.get(design) {
        return *c;
    }
    let target_len = design.len();
    if target_len == 0 {
        //println!("Made it!");
        return 1;
    }
    if let Some(c) = cache.get(design) {
        if !c {
            return 0;
        }
    }
    let mut combo_val = 0;
    for pattern in patterns {
        let len = pattern.len();
        if len > target_len { continue; }
        let (slice, rest) = design.split_at(len);
        //println!("Design: {}: slice {} and rest {}", design, slice, rest);
        if slice == pattern {
            let extra = count_all_combos(patterns, rest, cache, count_cache);
            count_cache.insert(rest, extra);
            combo_val = combo_val+extra
        }
    }
    count_cache.insert(design, combo_val);
    return combo_val;
}

fn is_design_possible_memoized<'a>(patterns: &[String], design: &'a str, cache: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(ret) = cache.get(design) {
        return *ret;
    }
    let target_len = design.len();
    if target_len == 0 {
        return true;
    }
    for pattern in patterns {
        let len = pattern.len();
        if len > target_len { continue; }
        //let slice = &design[0..len];
        let (slice, rest) = design.split_at(len);
        //println!("Slice: {}, rest: {}", slice, rest);
        if slice == pattern {
            if is_design_possible_memoized(patterns, rest, cache) {
                cache.insert(design, true);
                return true;
            }
        }
    }
    cache.insert(design, false);
    return false;
}

// no memoization
fn _is_design_possible_str(patterns: &[String], design: &str) -> bool {
    let target_len = design.len();
    if target_len == 0 {
        return true;
    }
    for pattern in patterns {
        let len = pattern.len();
        if len > target_len { continue; }
        let slice = &design[0..len];
        if slice == pattern {
            if _is_design_possible_str(patterns, &design[len..target_len]) {
                return true;
            }
        }
    }
    return false;
}


fn solve_pt1(puzzle: &Puzzle) -> usize {
    let mut cache = HashMap::new();
    puzzle.desired_designs.iter()
        .filter( |design| {
            if !is_design_possible_memoized(&puzzle.patterns, design, &mut cache) {
                //println!("{:?} CANNOT be formed", design);
                return false;
            }
            //println!("{:?} CAN be formed", design);
            return true;
        })
        .count()
}

fn solve_pt2(puzzle: &Puzzle) -> u64 {
    let mut cache = HashMap::new();
    let mut count_cache = HashMap::new();
    puzzle.desired_designs.iter()
        .filter( |design| is_design_possible_memoized(&puzzle.patterns, design, &mut cache))
        .collect::<Vec<_>>().into_iter()
        .map( |design| count_all_combos(&puzzle.patterns, design, &cache, &mut count_cache))
        .sum::<u64>()
}

pub fn day19() {
    let text = fs::read_to_string("inputs/day19.txt").unwrap();
    let pb = parse_problem(&text);
    let soln = solve_pt1(&pb);
    println!("The solution to day 19 part 1 is: {}", soln); // 313
    let soln2 = solve_pt2(&pb);
    println!("The solution to day 19 part 2 is: {}", soln2); // 666491493769758
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, parse_problem, solve_pt2};

    const SAMPLE: &str = r"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_sample_pt1() {
       let pb = parse_problem(&SAMPLE);
       assert_eq!(solve_pt1(&pb), 6);
    }

    #[test]
    fn test_part2() {
       let pb = parse_problem(&SAMPLE);
       assert_eq!(solve_pt2(&pb), 16);
    }

}
