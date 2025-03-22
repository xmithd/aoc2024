use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn read_file_as_text(path: &str) -> String {
    let text = fs::read_to_string(path).expect("Unable to read file");
    return text;
}

pub fn parse_line_by_line(txt: &str) -> Vec<Vec<i32>> {
    txt.split('\n')
    .filter(| line | { line.len() > 0})
    .map( | line | {
        line.split_whitespace().map( | val | {
            val.parse::<i32>().expect("Bad input")
        })
        .collect()
    }).collect()
}

pub fn is_safe(report: &[i32]) -> i32 {
    if report.len() == 1 {
        return 1; // safe
    }
    let mut increasing = false;
    for i in 0..report.len()-1 {
        let diff = report[i+1] - report[i];
        if i == 0 {
            increasing = diff > 0;
        }
        if (increasing && diff < 0) || (diff.abs() > 3 || diff.abs() < 1) || (!increasing && diff > 0) {
            return 0;
        }
    }
    return 1;
}

fn with_dampener<F>(vals: &[i32], func: F) -> i32
where F: Fn(&[i32]) -> i32 {
    if func(vals) == 1 {
        return 1;
    }
    // try with removing ith element
    for i in 0..vals.len() {
        let mut values: Vec<i32> = Vec::new();
        for j in 0..vals.len() {
            if j != i {
                values.push(vals[j]);
            }
        }
        if func(&values) == 1 {
            return 1;
        }
    }
    return 0;
}

pub fn check_safety(reports: &[Vec<i32>]) -> i32 {
    reports.into_iter()
        .map(| arg | { is_safe(arg) })
        .sum::<i32>()
}

pub fn check_safety_p2(reports: &[Vec<i32>]) -> i32 {
    reports.into_iter()
        .map(| arg | { with_dampener(arg, is_safe) })
        .sum::<i32>()
}

use regex::Regex;
pub fn parse_mul_pairs(text: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results = vec![];
    for (_, [d1, d2]) in re.captures_iter(text).map(|c| c.extract()) {
        results.push((d1.parse::<i32>().unwrap(), d2.parse::<i32>().unwrap()));
    }
    return results;
}

pub fn parse_do_mul_pairs(text: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"(?:mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don't\(\))").unwrap();
    //println!("Doing text: {}", text);
    let mut results = vec![];
    let mut skip_next = false;
    re.captures_iter(text).for_each(|c| {
        let mut d1: i32 = 0;
        let mut d2: i32 = 0;
        // todo in a more idiomatic way ^^;
        for i in 1..c.len() {
            let curr = c.get(i);
            if i == 1 && curr.is_some() {
                d1 = match curr {
                    Some(a) => a.as_str().parse::<i32>().unwrap(),
                    None => 0
                }
            } else if i == 2 && curr.is_some() {
                d2 = match curr {
                    Some(a) => a.as_str().parse::<i32>().unwrap(),
                    None => 0
                }
            }
            if i == 3 && curr.is_some() {
                skip_next = false;
                break;
            }
            if i == 4 && curr.is_some() {
                skip_next = true;
                break;
            }
        }
        if !skip_next {
            results.push((d1, d2));
        }
    });
    return results;
}

pub fn compute_multiplication_sum(list: &[(i32, i32)]) -> i32
{
    list.into_iter()
        .map(|(d1, d2)| { d1*d2 })
        .sum::<i32>()
}

pub fn parse_letter_grid(txt: &str) -> Vec<Vec<char>> {
    txt.split_whitespace()
        .filter( |line| {!line.is_empty()})
        .map( |line| { line.chars().collect() })
        .collect()
}


// Define a function to check if a cell is within the grid
fn is_valid(grid: &[Vec<char>], i: i32, j: i32) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    (i >= 0 && i < rows as i32) && (j >= 0 && j < cols as i32)
}


// Define a function to check if a pattern exists at position (i, j) in the grid
fn has_pattern(grid: &[Vec<char>], i: usize, j: usize, target: &str, th: usize, (dx,dy): (i32, i32)) ->
bool {
    //let cols = grid[0].len()
    if let Some(c) = target.chars().nth(th) {
        if grid[i][j] == c {
            if th == target.len() - 1 {
                return true;
            } else {
                if is_valid(grid, i as i32 + dy, j as i32 + dx) {
                    let y: usize = (i as i32 + dy) as usize;
                    let x: usize = (j as i32 + dx) as usize;
                    return has_pattern(grid, y, x, target, th+1, (dx, dy));
                } else {
                    return false;
                }
            }
        }
        return false;
    } else {
        return false;
    }
}

pub fn findall_in_grid(grid: &[Vec<char>], target: &str) -> i32 {
    let mut soln: i32 = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &_cell) in row.iter().enumerate() {
            let dxdy = vec![(-1,-1), (-1, 0), (-1,1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
            for direction in dxdy {
                if has_pattern(&grid, i, j, target, 0, direction) {
                    soln += 1;
                }
            }
        }
    }
    return soln;
}

pub fn findall_x_in_grid(grid: &[Vec<char>], target: &str) -> i32 {
    let mut soln: i32 = 0;
    let target_rev = target.chars().rev().collect::<String>();
    for (i, row) in grid.iter().enumerate() {
        for j in 0..row.len()-2 {
            // col,row
            let pairs = vec![/* ((1,-1),(-1,1)),*/ ((1,1), (-1,1))];
            for (dxdy1, dxdy2) in pairs {
                if (has_pattern(&grid, i, j, target, 0, dxdy1) ||
                    has_pattern(&grid, i, j, &target_rev, 0, dxdy1)) && (
                    has_pattern(grid, i, j+2, target, 0, dxdy2) ||
                    has_pattern(grid, i, j+2, &target_rev, 0, dxdy2)) 
                {
                    //println!("Soln at {}, {}", i, j);
                    soln += 1;
                }
            }
        }
    }
    return soln;
}

pub fn parse_page_order_pb(txt: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules_break = false;
    let mut printing_pages: Vec<Vec<i32>> = Vec::new();
    //let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let tokens = txt.split("\n");
    for tok in tokens {
        if tok.is_empty() {
            rules_break = true;
            continue;
        }
        if rules_break {
            printing_pages.push(tok.split(",").map(| numeric| { numeric.parse::<i32>().unwrap() }).collect());
        } else {
            let rule: Vec<i32> = tok.split("|").map(| numeric| { numeric.parse::<i32>().unwrap() }).collect();
            if rule.len() != 2 {
                panic!("Invalid rule!");
            }
            let key = rule[0];
            let val = rule[1];
            if let Some(vals) = rules.get_mut(&key) {
                //vals.push(val);
                vals.insert(val);
            } else {
                rules.insert(key,  HashSet::from([val]));
            }
        }
    }
    return (rules, printing_pages);
}

pub fn check_order_violation(key: &i32, after_l: &[i32], before_l: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> Option<(usize, usize)> {
    let curr_idx = before_l.len();
    if let Some(set) = rules.get(key) {
        for (idx, element) in before_l.iter().enumerate() {
            if set.contains(element) {
                // rule violated (elements in set must be after!)
                return Some((curr_idx, idx));
            }
        }
    }
    for (idx, element) in after_l.iter().enumerate() {
        if let Some(set) = rules.get(element) {
            if set.contains(key) {
                // rule violated
                return Some((curr_idx, 1+idx+before_l.len()));
            }
        }
    }
    return None;
}

fn get_violation_idx(order: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> Option<(usize, usize)> {
    for (i, item) in order.iter().enumerate() {
        let (before_l, after_l) = order.split_at(i);
        // note after_l contains the key element
        let violation = check_order_violation(item, &after_l[1..], before_l, rules);
        if violation.is_some() {
            return violation;
        }
    }
    return None;
}

pub fn middle_of_correct_orders(printing_pages: &[Vec<i32>], rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    printing_pages.iter()
        .filter(|list| { get_violation_idx(list, rules).is_none() })
        .map( |list| {
            let idx = list.len()/2;
            //println!("Order {:?}", list);
            if let Some(val) = list.get(idx) {
                //println!("Middle: {}", val);
                return val.clone();
            } else {
                return 0;
            }
        }).collect()
}

fn fix_order(order: &[i32], rules: &HashMap<i32, HashSet<i32>>, violating_idx: (usize, usize)) -> Vec<i32> {
    // swap violating indexes until it's good?
    // assumption: there is always a solution
    // println!("Bad idx {:?} for order: {:?}", violating_idx, order);
    let mut trial = Vec::from(order);
    let (idx1, idx2) = violating_idx;
    trial.swap(idx1, idx2);
    loop {
        if let Some((viol1, viol2)) = get_violation_idx(&trial, rules) {
            trial.swap(viol1, viol2);
        } else {
            break;
        }
    }
    return trial;
}

pub fn middle_of_corrected_orders(printing_pages: &[Vec<i32>], rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    printing_pages.iter()
    .map(|list| { (list, get_violation_idx(list, rules)) })
    .filter(|(_list, violation)| {violation.is_some()})
    .map(|(incorrect_order, violation)| {
        fix_order(incorrect_order, rules, violation.unwrap())
    })
    .map( |list| {
        let idx = list.len()/2;
        //println!("Order {:?}", list);
        if let Some(val) = list.get(idx) {
            //println!("Middle: {}", val);
            return val.clone();
        } else {
            return 0;
        }
    }).collect()
}

#[derive(Clone)]
pub enum OP {
    ADD,
    MULTIPLY,
    CONCAT
}

struct OpsCombinatorial<'a> {
    //total: i32, // n
    ops: &'a[OP],
    current: Vec<OP>, // has size m (empty spots)
    current_n: u64, // from 0 to (# of ops)^m
}

// generate all combinations
impl<'a> OpsCombinatorial<'a> {
    fn new(operations: &'a[OP], spots: u64) -> OpsCombinatorial<'a> {
        let mut current : Vec<OP> = Vec::new();
        for _ in 0..spots {
            current.push(operations.get(0).unwrap().clone());
        }
        Self {
            //total,
            ops: operations,
            current,
            current_n: 0,
        }
    }
}

impl Iterator for OpsCombinatorial<'_> {
    type Item = Vec<OP>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.current_n;
        if n != 0 {
            let len = self.current.len();
            let max = self.ops.len().pow(len as u32);
            if n > max as u64 {
                return None;
            }
            // set all the OPS in the vector
            let base = self.ops.len() as u64;
            for i in 0..len {
                // iterate x from tail until head
                let x = len - 1 - i;
                let curr_base = base.pow(i as u32);
                let shifted = (n - (n % curr_base)) / curr_base;
                self.current[x] = self.ops.get((shifted % base) as usize).unwrap().clone();
            }
        }
        self.current_n += 1;
        return Some(self.current.clone());
    }
}

fn concat_u64(lhs: u64, rhs: u64) -> u64 {
    let mut base = 10;
    while rhs/base != 0 {
        base *= 10;
    }
    return lhs*base + rhs;
}

fn apply_operations(operations: &[OP], operands: &[u64]) -> u64 {
    // put operations and operands in a queue
    let mut ops_queue: VecDeque<OP>= VecDeque::from(Vec::from(operations));
    let mut num_queue: VecDeque<u64> = VecDeque::from(Vec::from(operands));
    loop {
        if let Some(op) = ops_queue.pop_front() {
            let lhs = num_queue.pop_front().unwrap();
            let rhs = num_queue.pop_front().unwrap();
            let computed = match op {
                OP::ADD => lhs+rhs,
                OP::MULTIPLY => lhs*rhs,
                OP::CONCAT => concat_u64(lhs, rhs)
            };
            num_queue.push_front(computed);
        } else {
            break;
        }

    }
    if num_queue.len() != 1 {
        panic!("Oops, bug or mismatch in operators/operands")
    }
    return num_queue.pop_front().unwrap();
}

// returns the first valid match of operators
pub fn find_ops(answer: u64, operands: &[u64], operators: &[OP]) -> Option<Vec<OP>> {
    let combos = OpsCombinatorial::new(&operators, (operands.len()-1) as u64);
    for combo in combos {
        if answer == apply_operations(&combo, operands) {
            return Some(combo);
        }
    }
    return None;
}
