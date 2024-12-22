use std::fs;
use std::collections::BinaryHeap;
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;
use std::fmt::{Error, Formatter};
use std::fmt::Debug;
use std::i32;

// straight from the BinaryHeap docs example
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    positions: Vec<(i32, i32)>,
}

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

// straight from the BinaryHeap docs example
#[derive(Clone, Eq, PartialEq)]
struct StateCheap {
    cost: i32,
    node: (i32, i32),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for StateCheap {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            //.then_with(|| other.node.cmp(&self.node))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for StateCheap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//#[derive(Debug)]
struct Puzzle {
    board: Vec<Vec<char>>,
    columns: usize,
    rows: usize,
    start: (i32 ,i32),
    end: (i32 ,i32),
    walls: HashSet<(i32, i32)>
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for i in 0..self.rows {
            for j in 0..self.columns {
                let tile = self.get_usize(j, i);
                write!(f, "{}", tile).expect("er>r...");
            }
            write!(f, "\n").expect("err...");
        }
        write!(f, "")
    }
}

fn manhattan_distance(start: &(i32, i32), end: &(i32, i32)) -> usize {
    ((start.0 - end.0).abs() + (start.1 - end.1).abs()) as usize 
}


// returns first empty spot (if any) and if last one was a boulder

impl Puzzle {
    pub fn get(&self, col: i32, row: i32) -> char {
        self.get_usize(col as usize, row as usize)
    }

    pub fn get_usize(&self, col: usize, row: usize) -> char {
        *self.board.get(row).unwrap().get(col).unwrap()
    }

    pub fn compute_score(&self) -> i32 {
       //if let Some((cost, path)) = self.a_star() {
       //     println!("{:?}", path);
       if let Some(cost) = self.a_star_cheap(&None, &None) {
            return cost - self.heuristic(self.end);
        }
        0
    }

    pub fn compute_cheat_score(&self, skips: HashSet<(i32, i32)>, max_cost: i32) -> Option<i32> {
        if let Some(cost) = self.a_star_cheap(&Some(skips), &Some(max_cost)) {
            return Some(cost - self.heuristic(self.end));
        }
        None
    }

    fn _draw_final_board(&self, path: &[(i32, i32)]) {
        let mut board = self.board.clone();
        for (j, i) in path {
            board[*i as usize][*j as usize] = 'O';
        }
        for row in board {
            for val in row {
                print!("{}", val);
            }
            println!("");
        }
    }

    pub fn heuristic(&self, state: (i32, i32)) -> i32 {
        // Note to self: other A* is supposed to be optimal as long as h function is less than or
        // equal to cost... DO NOT USE for finding all best paths!
        // Manhattan distance
        (state.0 - self.end.0).abs() + (state.1 - self.end.1).abs()
    }

    pub fn cost(&self, _state: (i32, i32)) -> i32 {
        return 1;
    }

    fn possible_next_positions(&self, current: (i32, i32), skip_walls: &Option<HashSet<(i32, i32)>>) -> Vec<(i32, i32)> {
        // 4 directions
        [(1,0), (-1, 0), (0, -1), (0, 1)].into_iter()
            .map( |(dx, dy)| (current.0 + dx, current.1 + dy))
            .filter(|(j, i)| *i >= 0 && *i < self.rows as i32 && *j >= 0 && *j < self.columns as i32)
            .filter(|(j, i)| {
                self.get(*j, *i) != '#' ||
                    match skip_walls { None => false, Some(skip) => skip.contains(&(*j, *i)) }
            })
            .collect()
    }

    // returns the list of best paths
    pub fn a_star_all_paths(&self) -> Vec<Vec<(i32, i32)>> { // instead of Option<(i32,Vec<(i32, i32))>
        let mut ret: Vec<Vec<(i32, i32)>> = Vec::new();
        let mut best_score: i32 = -1;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut frontier = BinaryHeap::new();
        let start_position = (self.start.0, self.start.1);
        frontier.push(State { cost: self.heuristic(self.start), positions: vec![start_position] });
        while let Some(State { cost, positions }) = frontier.pop() {
            let current = positions.last().unwrap();
            let current_pos = (current.0, current.1);
            if  current_pos == self.end {
                // found goal
                if best_score == -1 {
                    best_score = cost;
                }
                if cost > best_score {
                    // no need to search for other paths
                    break;
                }
                ret.push(positions.clone());
            }

            visited.insert(current.clone());
            for child in self.possible_next_positions(current_pos, &None) {
                let child_node = (child.0, child.1);
                if visited.contains(&child_node) {
                    continue;
                }
                let new_cost = cost + self.cost(child) - self.heuristic(current_pos) + self.heuristic(child);
                frontier.push(State { cost: new_cost, positions: [positions.clone(), [child_node].to_vec()].concat() });
            }
        }
        ret
    }

    pub fn a_star_cheap(&self, skip_walls: &Option<HashSet<(i32, i32)>>, max_cost: &Option<i32>) -> Option<i32> {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut frontier = BinaryHeap::new();
        let start_position = (self.start.0, self.start.1);
        frontier.push(StateCheap { cost: self.heuristic(self.start), node: start_position });
        while let Some(StateCheap { cost, node }) = frontier.pop() {
            let current = &node;
            let current_pos = (current.0, current.1);
            if  current_pos == self.end { return Some(cost)} 
            visited.insert(current_pos);
            if let Some(max) = max_cost {
                let actual_cost = cost - self.heuristic(current_pos);
                if actual_cost > *max {
                    return None;
                }
            }
            for child in self.possible_next_positions(current_pos, skip_walls) {
                if visited.contains(&child) {
                    continue;
                }
                let new_cost = cost + self.cost(child) - self.heuristic(current_pos) + self.heuristic(child);
                let child_node = (child.0, child.1);
                frontier.push(StateCheap { cost: new_cost, node: child_node });
            }
        }
        None
    }

    fn is_not_surrounded(&self, pos: &(i32, i32)) -> bool {
        [(1,0), (-1, 0), (0, -1), (0, 1)].into_iter()
            .map( |(dx, dy)| (pos.0 + dx, pos.1 + dy))
            .filter(|(j, i)| *i >= 0 && *i < self.rows as i32 && *j >= 0 && *j < self.columns as i32)
            .filter( |(j, i)| self.get(*j, *i) != '#')
            .count() > 0
    }

    fn all_points_within_radius_better_than(&self, idx: usize, radius: usize, best_path: &[(i32, i32)], min_improvement: usize) -> Vec<(i32, i32)> {
        let mut ret = Vec::new();
        let pt = best_path.get(idx).unwrap();
        for i in idx+1..best_path.len() {
            let destination = best_path.get(i).unwrap();
            let shortcut_cost = manhattan_distance(pt, destination);
            if shortcut_cost <= radius {
                let real_cost = i-idx;
                if real_cost - shortcut_cost >= min_improvement {
                    ret.push(destination.clone());
                }
            }
        }
        //if ret.len() > 0 {
        //    println!("All points within radius better than {:?}: {:?}", pt, ret);
        //}
        ret
    }

    pub fn count_better_cheat_paths(&self, best_path: &[(i32, i32)], radius: usize, min_improvement: usize) -> usize {
        // for each point in best path, find all points within radius in the bath and add to
        // shortut if distance in better than in best_path by min_improvement.
        let mut count: usize = 0;
        //let path_pts = HashSet::from(best_path);
        for (idx, _potential_start) in best_path.into_iter().enumerate() {
            for _potential_end in self.all_points_within_radius_better_than(idx, radius, best_path, min_improvement) {
                //cheat_score_cut = (potential_end.0 - potential_start.0).abs + (potential_end.1 - potential_start.1).abs();
                count += 1;
            }
        }
        count
    }

}

fn parse_puzzle(str: &str) -> Puzzle {
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    for (i, line) in str.trim().split("\n").enumerate() {
        if line.is_empty() { continue; }
        let mut row: Vec<char> = Vec::new();
        for (j, tile) in line.chars().enumerate() {
            let current = (j as i32, i as i32);
            if tile == 'S' {
                start = current;
            } else if tile == 'E' {
                end = current;
            } else if tile == '#' && (i != 0 && j != 0) {
                walls.insert(current);
            }
            row.push(tile);
        }
        board.push(row);
    }

    let rows = board.len();
    let columns = board.get(0).unwrap().len();
    //println!("walls: {:?}", walls);

    Puzzle {
        board,
        columns,
        rows,
        start,
        end,
        walls
    }
}

fn solve_pt1(puzzle: &Puzzle, savings: usize) -> usize {
    // brute force is very slow (6 seconds for this part)
    //let orig_score = puzzle.compute_score();
    //let mut mapping: HashMap<i32, usize> = HashMap::new();
    //puzzle.walls.iter().map(|wall| {
    //    let score = puzzle.compute_cheat_score(HashSet::from([*wall; 1]), orig_score-savings);
    //    if let Some(actual) = score {
    //        return orig_score - actual;
    //    }
    //    -1
    //})
    //.filter( |diff| *diff > 0)
    //.for_each( | diff | {
    //    *mapping.entry(diff).or_insert(0) += 1;
    //});
    ////println!("mapping: {:?}", mapping);
    //let mut cheats_saving = 0;
    //for (saved, count) in mapping {
    //    if saved >= savings {
    //        cheats_saving += count;
    //    }
    //}
    //cheats_saving
    let paths = puzzle.a_star_all_paths();
    let best_path = paths.get(0).unwrap();
    //println!("Best path: {:?}", best_path);
    // part 1 radius is 2. Now it should be 20
    puzzle.count_better_cheat_paths(&best_path, 2, savings) 
}

fn solve_pt2(puzzle: &Puzzle, savings: usize) -> usize {
    let paths = puzzle.a_star_all_paths();
    let best_path = paths.get(0).unwrap();
    //println!("Best path: {:?}", best_path);
    // part 1 radius is 2. Now it should be 20
    puzzle.count_better_cheat_paths(&best_path, 20, savings) 
}

pub fn day20() {
    let text = fs::read_to_string("inputs/day20.txt").unwrap();
    let puzzle = parse_puzzle(&text);
    //println!("{:?}", puzzle);
    let soln = solve_pt1(&puzzle, 100);
    println!("Solution to day 20 part 1: {}", soln); // 1372
    let soln2 = solve_pt2(&puzzle, 100);
    println!("Solution to day 20 part 2: {:?}", soln2); // 979014
}

#[cfg(test)]
mod tests {
    use super::{parse_puzzle, solve_pt1, solve_pt2};

    const FIRST_SAMPLE: &str = r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    //There are 14 cheats that save 2 picoseconds.
    //There are 14 cheats that save 4 picoseconds.
    //There are 2 cheats that save 6 picoseconds.
    //There are 4 cheats that save 8 picoseconds.
    //There are 2 cheats that save 10 picoseconds.
    //There are 3 cheats that save 12 picoseconds.
    //There is one cheat that saves 20 picoseconds.
    //There is one cheat that saves 36 picoseconds.
    //There is one cheat that saves 38 picoseconds.
    //There is one cheat that saves 40 picoseconds.
    //There is one cheat that saves 64 picoseconds.

    // solve_pt1 gets the sum of cheats that save *at least* N number of picoseconds
    // so we need the sum of the ones >= N
    #[test]
    fn test_first_sample() {
        let pb = parse_puzzle(&FIRST_SAMPLE);
        assert_eq!(pb.compute_score(), 84);
        assert_eq!(solve_pt1(&pb, 2), 44);
        assert_eq!(solve_pt1(&pb, 64), 1);
        assert_eq!(solve_pt1(&pb, 12), 8);
    }

    // similarly for part 2:
    // There are 32 cheats that save 50 picoseconds.
    // There are 31 cheats that save 52 picoseconds.
    // There are 29 cheats that save 54 picoseconds.
    // There are 39 cheats that save 56 picoseconds.
    // There are 25 cheats that save 58 picoseconds.
    // There are 23 cheats that save 60 picoseconds.
    // There are 20 cheats that save 62 picoseconds.
    // There are 19 cheats that save 64 picoseconds.
    // There are 12 cheats that save 66 picoseconds.
    // There are 14 cheats that save 68 picoseconds.
    // There are 12 cheats that save 70 picoseconds.
    // There are 22 cheats that save 72 picoseconds.
    // There are 4 cheats that save 74 picoseconds.
    // There are 3 cheats that save 76 picoseconds.

    #[test]
    fn test_sample_pt2() {
        let pb = parse_puzzle(&FIRST_SAMPLE);
        assert_eq!(solve_pt2(&pb, 76), 3);
        assert_eq!(solve_pt2(&pb, 74), 7);
        assert_eq!(solve_pt2(&pb, 70), 12+22+4+3);
    }
   

}
