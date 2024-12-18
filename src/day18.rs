use std::fs;
use std::collections::BinaryHeap;
use std::collections::HashSet;
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
    corrupted: Vec<(i32, i32)>,
    bytes_fallen: usize,
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
       if let Some(cost) = self.a_star_cheap() {
            return cost - self.heuristic(self.end);
        }
        0
    }

    fn draw_final_board(&self, path: &[(i32, i32)]) {
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

    pub fn compute_path(&self) -> i32 {
        //let mut paths: HashSet<(i32,i32)> = HashSet::new();
        let solns = self.a_star_all_paths();
        //println!("{:?}", solns);
        /*for soln in solns {
            for node in soln {
                let pos = (node.0, node.1);
                paths.insert(pos);
            }
        }
        paths.len() as i32
        //solns.len() as i32*/
        let mut minone = i32::MAX;
        let mut ith = 0;
        for (i, soln) in solns.iter().enumerate() {
            if (soln.len() as i32) < minone {
                ith = i;
                minone = (soln.len() as i32);
            }
        }
        //if let Some(thebest) = solns.get(ith) {
        //    self.draw_final_board(&thebest);
        //}
        //println!("Found {} best paths!", solns.len());
        minone - 1
    }

    pub fn heuristic(&self, state: (i32, i32)) -> i32 {
        // Note to self: other A* is supposed to be optimal as long as h function is less than or
        // equal to cost... but I am not having luck with chosing an h val...
        //0
        // No luck with Manhattan distance
        ((state.0 - self.end.0).abs() + (state.1 - self.end.1).abs())*1
        // No luck with Euclidean distance either
        //((state.0-self.end.0).pow(2) as f64 + (state.1 - self.end.1).pow(2) as f64).sqrt() as i32

    }

    pub fn cost(&self, state: (i32, i32)) -> i32 {
        // TODO orientation?
        if self.get(state.0, state.1) == '#' {
            // cost of being in the wall becomes prohibitively expensive
            return 10000;
        }
        return 1;
    }

    fn possible_next_positions(&self, current: (i32, i32)) -> Vec<(i32, i32)> {
        // 4 directions
        [(1,0), (-1, 0), (0, -1), (0, 1)].into_iter()
            .map( |(dx, dy)| (current.0 + dx, current.1 + dy))
            .filter(|(j, i)| *i >= 0 && *i < self.rows as i32 && *j >= 0 && *j < self.columns as i32)
            .filter(|(j, i)| self.get(*j, *i) != '#')
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
                //return Some((cost, positions))
            }
            //if visited.contains(current) {
            //    continue;
            //} else {
                visited.insert(current.clone());
            //}
            for child in self.possible_next_positions(current_pos) {
                //for (x, y, _) in &positions {
                //    if child == (*x, *y) {
                //        continue;
                //    }
                //}
                //let orientation = self.get_orientation(current, child);
                //let added_cost = self.get_orientation_cost(current.2, orientation);
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

    pub fn a_star_cheap(&self) -> Option<i32> {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut frontier = BinaryHeap::new();
        let start_position = (self.start.0, self.start.1);
        frontier.push(StateCheap { cost: self.heuristic(self.start), node: start_position });
        while let Some(StateCheap { cost, node }) = frontier.pop() {
            let current = &node;
            let current_pos = (current.0, current.1);
            if  current_pos == self.end { return Some(cost)}
            visited.insert(current_pos);
            for child in self.possible_next_positions(current_pos) {
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
}

fn parse_puzzle(str: &str, rows: usize, columns: usize, first_corrupted_bytes: Option<usize>) -> Puzzle {
    let mut board: Vec<Vec<char>> = vec![vec!['.'; columns]; rows];
    let start: (i32, i32) = (0, 0);
    let end: (i32, i32) = ((columns-1) as i32, (rows-1) as i32);
    let mut corrupted: Vec<(i32, i32)> = Vec::new();
    for line in str.trim().split("\n") {
        let parts = line.split(",").map(|token| token.parse::<i32>().unwrap()).collect::<Vec<_>>();
        if parts.len() != 2 {
            panic!("Input error: tokens.len() != 2");
        }

        corrupted.push((*parts.get(0).unwrap(), *parts.get(1).unwrap()));
    }

    //let columns = board.get(0).unwrap().len() ;
    //let rows = board.len();
    let mut corrupted_set: HashSet<(i32, i32)> = HashSet::new();
    let until = match first_corrupted_bytes {
        None => corrupted.len(),
        Some(c) => c
    };
    for (i, val) in corrupted.iter().enumerate() {
        if i == until {
            break;
        }
        corrupted_set.insert(*val);
    }
    for i in 0..rows {
        for j in 0..columns {
            if corrupted_set.contains(&(j as i32, i as i32)) {
                board[i][j] = '#';
            }
        }
    }

    Puzzle {
        board,
        columns,
        rows,
        start,
        end,
        corrupted,
        bytes_fallen: until
    }
}

fn solve_pt1(puzzle: &Puzzle) -> i32 {
    puzzle.compute_score()
}

fn solve_pt2(puzzle: &mut Puzzle) -> Option<(i32,i32)> {
    // brute force
    for i in puzzle.bytes_fallen..puzzle.corrupted.len() {
        let next_corrupted = puzzle.corrupted.get(i).unwrap();
        puzzle.board[next_corrupted.1 as usize][next_corrupted.0 as usize] = '#';
        let c = puzzle.compute_score();
        if c == 0 {
            // found it
            return Some(*next_corrupted);
        }
    }
    None
}

pub fn day18() {
    let text = fs::read_to_string("inputs/day18.txt").unwrap();
    let mut puzzle = parse_puzzle(&text, 71, 71, Some(1024));
    //println!("{:?}", puzzle);
    let soln = solve_pt1(&puzzle);
    println!("Solution to day 16 part 1: {}", soln); // 308
    let soln2 = solve_pt2(&mut puzzle);
    println!("Solution to day 16 part 2: {:?}", soln2);
}

#[cfg(test)]
mod tests {
    use super::{parse_puzzle, solve_pt1, solve_pt2};

    const FIRST_SAMPLE: &str = r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
    #[test]
    fn test_first_sample() {
        let mut pb = parse_puzzle(&FIRST_SAMPLE, 7, 7, Some(12));
        println!("{:?}", pb);
        assert_eq!(solve_pt1(&pb), 22);
        assert_eq!(solve_pt2(&mut pb), Some((6, 1)));
    }

}
