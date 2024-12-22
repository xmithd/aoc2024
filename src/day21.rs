use std::fs;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;

// straight from the BinaryHeap docs example
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    positions: Vec<(i32, i32, char)>,
}

const START_CHAR: char = '#';

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            //.then_with(|| other.positions.cmp(&self.positions))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



fn manhattan_distance(start: &(i32, i32), end: &(i32, i32)) -> usize {
    ((start.0 - end.0).abs() + (start.1 - end.1).abs()) as usize 
}

fn euclidean_distance(start: &(i32, i32), end: &(i32, i32)) -> usize {

    ((start.0 - end.0).pow(2) as f64 + (start.1 - end.1).pow(2) as f64).sqrt().floor() as usize
}

// might need all of the best paths so disable heuristics!
pub fn a_star_best_paths(start: &(i32, i32), goal: &(i32, i32),
    cost_fn: &dyn Fn(&(i32, i32)) -> usize,
    _h: &dyn Fn(&(i32, i32)) -> usize,
    next_positions: &dyn Fn(&(i32, i32)) -> Vec<(i32, i32)>,
    start_dir: char) -> Vec<(usize,Vec<(i32, i32, char)>)> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut frontier = BinaryHeap::new();
    let start_position = (start.0, start.1, start_dir);
    let mut best_cost: Option<usize> = None;
    let mut ret: Vec<(usize, Vec<(i32, i32, char)>)> = Vec::new();
    frontier.push(State { cost: 0, positions: vec![start_position] });
    while let Some(State { cost, positions }) = frontier.pop() {
        let current = positions.last().unwrap();
        let current_pos = (current.0, current.1);
        if let Some(score) = best_cost {
            if cost > score {
                break;
            }
        }
        if  current_pos == *goal {
            // found goal
            if best_cost.is_none() {
                best_cost = Some(cost);
            }
            ret.push((cost, positions.clone()));
        }
        let current_dir = current.2;
        visited.insert(current_pos);
        for child in next_positions(&current_pos) {
            if visited.contains(&child) {
                continue;
            }
            let new_dir = get_direction_char(&current_pos, &child);
            // prefer path with straight lines (least change in directions)
            let added_cost = if current_dir == START_CHAR || new_dir == current_dir { 0 } else { 3 };
            let new_cost = cost + cost_fn(&child) + added_cost*0; // ignore added cost for now?
            let child_node = (child.0, child.1, new_dir);
            frontier.push(State { cost: new_cost, positions: [positions.clone(), [child_node].to_vec()].concat() });
        }
    }
    ret
}


struct Puzzle {
    numpad_board: Vec<Vec<char>>,
    keypad_board: Vec<Vec<char>>,
    codes: Vec<String>
}

fn get_keypad_position(input: char) -> (i32, i32) {
    match input {
        ' ' => (0, 0),
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("Invalid input: {}", input)
    }
}

fn get_numpad_position(input: char) -> (i32, i32) {
    match input {
        ' ' => (0, 3),
        '0' => (1, 3),
        'A' => (2, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        _ => panic!("Oops, invalid character {} requested", input)
    }
}

fn get_direction_char(p: &(i32, i32), n: &(i32, i32)) -> char {
    let dxdy = (n.0 - p.0, n.1 - p.1);
    match dxdy {
        (1, 0) => '>',
        (-1, 0) => '<',
        (0, 1) => 'v',
        (0, -1) => '^',
        _ => panic!("Oops, somehow got an invalid direction vector {:?}", dxdy)
    }
}

fn deduce_directions(positions: &[(i32, i32)]) -> Vec<char> {
    let mut ret = Vec::with_capacity(positions.len()-1);
    let mut prev = positions.get(0).unwrap();
    for i in 1..positions.len() {
        let next = positions.get(i).unwrap();
        ret.push(get_direction_char(prev, next));
        prev = next;
    }
    ret
}

fn parse_puzzle(text: &str) -> Puzzle {
    let numpad_board = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A']
    ];
    let keypad_board = vec![
        vec![' ', '^', 'A'],
        vec!['<', 'v', '>']
    ];
    let codes = text.trim().split("\n").map(|line| line.trim().to_string() ).collect();
    Puzzle {
        numpad_board,
        keypad_board,
        codes
    }
}

impl Puzzle {
    fn next_positions_numpad(&self, current: &(i32, i32)) -> Vec<(i32, i32)> {
        // 4 directions
        [(-1, 0), (0, -1), (0, 1), (1, 0)].into_iter()
            .map( |(dx, dy)| (current.0 + dx, current.1 + dy))
            .filter(|(j, i)| *i >= 0 && *i < 4 && *j >= 0 && *j < 3)
            .filter( |candidate| *candidate != (0, 3)) // avoid space
            .collect()
    }

    fn next_positions_keypad(&self, current: &(i32, i32)) -> Vec<(i32, i32)> {
        // 4 directions
        [(-1, 0), (0, -1), (0, 1), (1, 0)].into_iter()
            .map( |(dx, dy)| (current.0 + dx, current.1 + dy))
            .filter(|(j, i)| *i >= 0 && *i < 2 && *j >= 0 && *j < 3)
            .filter(|candidate| *candidate != (0, 0)) // should never have to go here
            .collect()
    }

    fn cost_fn(&self, _current: &(i32, i32)) -> usize {
        1
    }

    // layer 0 -> numpad, the rest use keypad, returns command sequence as string (cost is simply the length)
    pub fn get_moves_per_path(&self, pattern: &str, layer: u32) -> Vec<String> {
        let mut prev = 'A';
        let mut sum_moves: Vec<usize> = Vec::new();
        let mut moves: Vec<Vec<String>> = vec![vec![];pattern.len()];
        let mut start_dir = START_CHAR; //if layer == 0 { '<' } else { START_CHAR };
        let next_pos: &dyn Fn(&(i32, i32)) -> Vec<(i32, i32)> = match layer {
            0 => &|arg| { self.next_positions_numpad(arg)},
            _ => &|arg| {self.next_positions_keypad(arg)}
        };
        for (slot_id, current) in pattern.chars().enumerate() {
            let start = if layer == 0 { get_numpad_position(prev) } else { get_keypad_position(prev) };
            let end = if layer == 0 { get_numpad_position(current) } else { get_keypad_position(current) };
            //print!("Go from {} to {}: ", prev, current);
            prev = current;
            let best_paths  = a_star_best_paths(
                &start, &end,
                &|arg| self.cost_fn(arg),
                &|state| manhattan_distance(state, &end),
                next_pos,
                start_dir
            );
            for (_cost, path) in best_paths {
                let mut possible_ways = moves.get_mut(slot_id).unwrap();
                if path.len() > 1 { // have to move
                    start_dir = path.last().unwrap().2;
                    let mut directions = deduce_directions(&path.into_iter().map(|(x, y, _)| (x, y)).collect::<Vec<_>>());
                    directions.push('A');
                    let actual_cost = manhattan_distance(&start, &end);
                    //sum_moves += actual_cost + 1;
                    //moves = [moves.clone(), directions].concat();
                    possible_ways.push(directions.into_iter().collect());
                } else { // don't have to move
                    //moves = [moves.clone(), ['A'].to_vec()].concat();
                    //sum_moves += 1;
                    possible_ways.push("A".to_string());
                }
            }
        }
        // build all the paths into a single vec
        let mut total_possibilites = 1;
        for slot in &moves {
            total_possibilites *= slot.len();
        }
        let mut ret = Vec::with_capacity(total_possibilites);
        for i in 0..total_possibilites {
            let mut base = 1;
            // below is wasteful but I don't know how to populate by index
            let mut curr = vec!["".to_string(); moves.len()];
            for j in 0..moves.len() {
                let kth = moves.len() - 1 - j;
                let kth_moves = moves.get(kth).unwrap();
                let idx = (i / base) % kth_moves.len();
                curr[kth] = kth_moves.get(idx).unwrap().clone();
                base *= kth_moves.len();
            }
            ret.push(curr.join(""));
        }
        return ret;

    }

    pub fn compute_pt1_score(&self, pattern: &str) -> usize {
        let mut best_score = usize::MAX;
        let mut best_path: String = "".to_string();
        for l1path in self.get_moves_per_path(pattern, 0) {
            for l2path in self.get_moves_per_path(&l1path, 1) {
                for l3path in self.get_moves_per_path(&l2path, 2) {
                    if l3path.len() < best_score {
                        best_score = l3path.len();
                        best_path = l3path.clone();
                    }
                }
            }
        }
        println!("Best path for pattern {}: {}", pattern, best_path);
        let numerical_val = pattern.chars().filter(|digit| digit.is_digit(10)).collect::<String>().parse::<usize>().unwrap();
        best_score*numerical_val
    }

    pub fn compute_pt2_score(&self, pattern: &str, layer_num: usize, target_layer: usize) -> u64 {
        // use recursion + cache for part 2!
        // TODO
        0
    }

}

fn solve_pt1(pb: &Puzzle) -> usize {
    pb.codes.iter().map(
        |pattern| pb.compute_pt1_score(pattern)
    ).sum::<usize>()
}

fn solve_pt2(pb: &Puzzle) -> u64 {
    pb.codes.iter().map(
        |pattern| pb.compute_pt2_score(pattern, 0, 25)
    ).sum::<u64>()
}

pub fn day21() {
    let text = fs::read_to_string("inputs/day21.txt").unwrap();
    let puzzle = parse_puzzle(&text);
    let soln = solve_pt1(&puzzle);
    println!("Solution to day 21 part 1: {}", soln); // 94426
    let soln2 = solve_pt2(&puzzle);
    println!("Solution to day 21 part 2: {}", soln2);
}

#[cfg(test)]
mod tests {
    use super::{parse_puzzle, solve_pt1};

    const SAMPLE: &str = r"
029A
980A
179A
456A
379A
";

    #[test]
    fn test_029A() {
        let pb = parse_puzzle(&SAMPLE);
        let pattern = "029A";
        let third_cost = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len();
        assert_eq!(pb.compute_pt1_score(pattern), (third_cost*29).try_into().unwrap());
    }

//029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
//179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
//379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

//029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
//179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
//379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A


    #[test]
    fn test_980A() {
        let pb = parse_puzzle(&SAMPLE);
        let pattern = "980A";
        let third_cost = "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len();
        assert_eq!(pb.compute_pt1_score(pattern), (third_cost*980).try_into().unwrap());
    }

    #[test]
    fn test_179A() {
        let pb = parse_puzzle(&SAMPLE);
        let pattern = "179A";
        let third_cost = "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len();
        assert_eq!(pb.compute_pt1_score(pattern), (third_cost*179).try_into().unwrap());
    }

    #[test]
    fn test_456A() {
        let pb = parse_puzzle(&SAMPLE);
        let pattern = "456A";
        let third_cost = "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len();
        assert_eq!(pb.compute_pt1_score(pattern), (third_cost*456).try_into().unwrap());
    }

    #[test]
    fn test_379A() {
        let pb = parse_puzzle(&SAMPLE);
        let pattern = "379A";
        let third_cost = "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len();
        assert_eq!(pb.compute_pt1_score(pattern), (third_cost*379).try_into().unwrap());
    }

    #[test]
    fn test_all_best_moves() {
        let pb = parse_puzzle(&SAMPLE);
        let pattern = "179A";
        let moves = pb.get_moves_per_path(pattern, 0);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 2);
        let next_layer = pb.get_moves_per_path(moves.get(0).unwrap(), 1);
        println!("{:?}", next_layer);
        assert_eq!(next_layer.len(), 16);
    }

    #[test]
    fn test_sample() {
        let pb = parse_puzzle(&SAMPLE);
        assert_eq!(solve_pt1(&pb), 126384);
    }
}
