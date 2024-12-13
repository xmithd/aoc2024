use utils;
use regex::Regex;

const MAX_PRESSES: usize = 100;
const MAX_PRESSES_PT2: usize = 100000;

#[derive(Debug)]
struct Puzzle {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize_target: (usize, usize),
}

fn get_pair_from_line(line: &str) -> (usize, usize) {
    let regex = Regex::new(r"X(?:\+|=)([0-9]+),\s+?Y(?:\+|=)([0-9]+)").unwrap();

    let (_, [p1, p2]) = regex.captures_iter(line).map(|c| c.extract()).take(1).next().unwrap();
    let d1 = p1.parse::<usize>().unwrap();
    let d2 = p2.parse::<usize>().unwrap();
    return (d1, d2);
}

fn parse_input(txt: &str) -> Vec<Puzzle> {
    let mut pb = Vec::new();
    let mut pair_a = (0, 0);
    let mut pair_b = (0, 0);
    let mut pair_target = (0, 0);
    let mut all_set = false;
    for line in txt.split("\n") {
        if line.is_empty() {
            if all_set {
                pb.push(Puzzle {
                    button_a: pair_a.clone(),
                    button_b: pair_b.clone(),
                    prize_target: pair_target.clone()
                });
                all_set = false;
            } // else ignore empty line where nothing is set
        } else if line.contains("Button A") {
            pair_a = get_pair_from_line(line);
        } else if line.contains("Button B") {
            pair_b = get_pair_from_line(line);
        } else if line.contains("Prize") {
            pair_target = get_pair_from_line(line);
            all_set = true;
        }
    }
    if all_set {
        // add last one in case input doesn't end on line end
        pb.push(Puzzle {
            button_a: pair_a.clone(),
            button_b: pair_b.clone(),
            prize_target: pair_target.clone()
        })
    }
    return pb;
}

struct Combo {
    current_n: usize,
    options: usize,
    max: usize,
    limit: usize,
}

impl Combo {
    pub fn new(options: usize, max: usize) -> Self {
        Combo {
            current_n: 0,
            options: options,
            max: max,
            limit: max.pow(options as u32)
        }
    }
}

impl Iterator for Combo {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_n >= self.limit {
            return None;
        }
        let mut ret = vec![0; self.options];
        for i in 0..self.options {
            let j = self.options-1-i;
            let base = self.current_n / (self.max.pow(i as u32));
            ret[j] = base % self.max;
            //ret.push(base%self.max);
        }
        //ret.reverse();
        self.current_n += 1;
        return Some(ret);
    }
}

fn get_cost(soln: &(usize, usize)) -> usize {
    return 3*soln.0+1*soln.1;
}

fn cheapest_solution(solns: &[(usize, usize)]) -> Option<(usize, usize)> {
    if solns.len() == 0 {
        return None;
    }
    let mut min = usize::MAX;
    let mut min_idx: usize = 0;
    solns.into_iter().enumerate()
    .for_each(|(i, soln)|{ 
        let cost = get_cost(soln);
        if cost < min {
            min = cost;
            min_idx = i;
        }
    });
    Some(solns.get(min_idx).unwrap().clone())
}

fn solve_puzzle_naive(puzzle: &Puzzle, max_presses: usize) -> Vec<(usize, usize)> {
    // brute force all combinations
    let (x_a, y_a) = puzzle.button_a;
    let (x_b, y_b) = puzzle.button_b;
    let (final_x, final_y) = puzzle.prize_target;
    // find press_a and press_b such that 
    // x_a * press_a + x_b * press_b = final_x
    // and y_a * press_a + y_b * press_b = final_y
    let mut solutions: Vec<(usize, usize)> = Vec::new();
    for trial in Combo::new(2, max_presses+1) {
        let press_a = trial[0];
        let press_b = trial[1];
        //println!("Trial PRESS_A {}, PRESS B {}", press_a, press_b);
        let computation_x = x_a * press_a + x_b * press_b;
        let computation_y = y_a * press_a + y_b * press_b;
        if  computation_x == final_x && computation_y == final_y {
            solutions.push((press_a, press_b));
        }
    }

    solutions
}

fn solve_puzzle(puzzle: &Puzzle, _max_presses: usize) -> Vec<(usize, usize)> {
    // solve system of equations
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let (x_a, y_a) = puzzle.button_a;
    let (x_b, y_b) = puzzle.button_b;
    let (final_x, final_y) = puzzle.prize_target;
    // convert to i64 for negative values
    let x_a = x_a as i64;
    let y_b = y_b as i64;
    let x_b = x_b as i64;
    let y_a = y_a as i64;
    let final_x = final_x as i64;
    let final_y = final_y as i64;
    if x_a*y_b != y_a*x_b { // determinant should not be 0
        let pb = (x_a*final_y - y_a*final_x) / (x_a*y_b - y_a*x_b);
        let pa = (final_x - x_b*pb) / x_a;
        // double check
        let computation_x = x_a * pa + x_b * pb;
        let computation_y = y_a * pa + y_b * pb;
        //println!("PA: {}, PB: {}", pa, pb);
        if  computation_x == final_x && computation_y == final_y {
            ret.push((pa as usize, pb as usize));
        }
    }
    return ret;

}

fn transform_puzzle_for_part2(input: &[Puzzle]) -> Vec<Puzzle> 
{
    const ADDED: usize = 10000000000000;
    input.into_iter()
    .map(|puzz| {
        Puzzle {
            button_a: puzz.button_a,
            button_b: puzz.button_b,
            prize_target: (puzz.prize_target.0 + ADDED, puzz.prize_target.1 + ADDED)
        }
    })
    .collect()
}

fn solve_pt1(input: &[Puzzle]) -> usize {
    input.into_iter().map(|puzz| {
        solve_puzzle(puzz, MAX_PRESSES)
    })
    .map(|candidates|{cheapest_solution(&candidates)})
    .map(|maybesoln| {
        if let Some(soln) = maybesoln {
            return get_cost(&soln)
        } else {
            return 0;
        }
    })
    .sum::<usize>()
}

fn solve_pt2(input: &[Puzzle]) -> usize {
    let transformed_puzz = transform_puzzle_for_part2(input);
    transformed_puzz.into_iter().map(|puzz| {
        solve_puzzle(&puzz, MAX_PRESSES_PT2)
    })
    .map(|candidates|{cheapest_solution(&candidates)})
    .map(|maybesoln| {
        if let Some(soln) = maybesoln {
            return get_cost(&soln)
        } else {
            return 0;
        }
    })
    .sum::<usize>()
}

pub fn day13() {
    let text = utils::read_file_as_text("inputs/day13.txt");
    let pb = parse_input(&text);
    let sol1 = solve_pt1(&pb);
    println!("Solution to part 1: {}", sol1);
    let sol2 = solve_pt2(&pb);
    println!("Solution to part 2: {}", sol2);
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, parse_input, solve_pt2, transform_puzzle_for_part2};
const EXAMPLE: &str = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    #[test]
    fn test_sample() {
        
        let pb = parse_input(EXAMPLE);
        //println!("pb: {:?}", pb);
        assert_eq!(pb.len(), 4);
        assert_eq!(solve_pt1(&pb), 480);
    }

    #[test]
    fn test_part2() {
        let pb = parse_input(EXAMPLE);
        assert_eq!(transform_puzzle_for_part2(&pb).len(), 4);
        assert_eq!(solve_pt2(&pb), 875318608908);
    }
}