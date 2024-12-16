use std::fs;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::fmt::{Error, Formatter};
use std::fmt::Debug;

// straight from the BinaryHeap docs example
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    positions: Vec<(i32, i32, char)>, // orientation 'N', 'E', 'S', 'W'
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
            .then_with(|| other.positions.cmp(&self.positions))
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
    node: (i32, i32, char), // orientation 'N', 'E', 'S', 'W'
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
            .then_with(|| other.node.cmp(&self.node))
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
            return cost;
        }
        0
    }

    pub fn compute_pt2(&self) -> i32 {
        let mut paths: HashSet<(i32,i32)> = HashSet::new();
        let solns = self.a_star_all_paths();
        println!("{:?}", solns);
        for soln in solns {
            for node in soln {
                let pos = (node.0, node.1);
                paths.insert(pos);
            }
        }
        paths.len() as i32
        //solns.len() as i32
    }

    pub fn heuristic(&self, state: (i32, i32)) -> i32 {
        // Manhattan distance
        (state.0 - self.end.0).abs() + (state.1 - self.end.1).abs()
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

    fn possible_next_positions_v2(&self, current: (i32, i32), orientation: char) -> Vec<(i32, i32)> {
        let forbidden_dir = match orientation {
            'N' => (0, 1),
            'S' => (0, -1),
            'E' => (-1, 0),
            'W' => (1, 0),
            _ => panic!("Unknown direction!")
        };
        // 4 directions (E, W, N, S)
        [(1,0), (-1, 0), (0, -1), (0, 1)].into_iter()
            .filter(|it| { *it != forbidden_dir })
            .map( |(dx, dy)| (current.0 + dx, current.1 + dy))
            .filter(|(j, i)| *i >= 0 && *i < self.rows as i32 && *j >= 0 && *j < self.columns as i32)
            .filter(|(j, i)| self.get(*j, *i) != '#')
            .collect()
    }

    fn get_orientation(&self, current: &(i32, i32, char), child: (i32, i32)) -> char {
        let dir = (child.0 - current.0, child.1 - current.1);
        match dir {
            (0, -1) => 'N',
            (1, 0) => 'E',
            (-1, 0) => 'W',
            (0, 1) => 'S',
            _ => panic!("Unknown direction or calulation error!")
        }
    }

    fn get_orientation_cost(&self, current: char, next: char) -> i32 {
        if current == next {
            0
        } else if current == 'N' && next == 'S'
        || current == 'S' && next == 'N'
        || current == 'E' && next == 'W'
        || current == 'W' && next == 'E' {
            2*1000
        } else {
            1000
        }
    }

    // returns the list of best paths
    pub fn a_star_all_paths(&self) -> Vec<Vec<(i32, i32, char)>> { // instead of Option<(i32,Vec<(i32, i32, char))>
        let mut ret: Vec<Vec<(i32, i32, char)>> = Vec::new();
        let mut best_score: i32 = -1;
        //let mut visited: HashSet<(i32, i32, char)> = HashSet::new();
        let mut frontier = BinaryHeap::new();
        let start_position = (self.start.0, self.start.1, 'E');
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
            //    visited.insert(current.clone());
            //}
            for child in self.possible_next_positions_v2(current_pos, current.2) {
                for (x, y, _) in &positions {
                    if child == (*x, *y) {
                        continue;
                    }
                }
                let orientation = self.get_orientation(current, child);
                let added_cost = self.get_orientation_cost(current.2, orientation);
                let new_cost = cost + self.cost(child) - self.heuristic(current_pos) + self.heuristic(child) + added_cost;
                let child_node = (child.0, child.1, orientation);
                frontier.push(State { cost: new_cost, positions: [positions.clone(), [child_node].to_vec()].concat() });
            }
        }
        ret
    }

    pub fn a_star_cheap(&self) -> Option<i32> {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut frontier = BinaryHeap::new();
        let start_position = (self.start.0, self.start.1, 'E');
        frontier.push(StateCheap { cost: self.heuristic(self.start), node: start_position });
        while let Some(StateCheap { cost, node }) = frontier.pop() {
            let current = &node;
            let current_pos = (current.0, current.1);
            if  current_pos == self.end { return Some(cost)}
            if visited.contains(&current_pos) {
                continue;
            } else {
                visited.insert(current_pos);
            }
            for child in self.possible_next_positions(current_pos) {
                // TODO add for rotation
                let orientation = self.get_orientation(current, child);
                let added_cost = self.get_orientation_cost(current.2, orientation);
                let new_cost = cost + self.cost(child) - self.heuristic(current_pos) + self.heuristic(child) + added_cost;
                let child_node = (child.0, child.1, orientation);
                frontier.push(StateCheap { cost: new_cost, node: child_node });
            }
        }
        None
    }

}

fn parse_puzzle(str: &str) -> Puzzle {
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    for (i, line) in str.trim().split("\n").enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (j as i32, i as i32);
            } else if c == 'E' {
                end = (j as i32, i as i32);
            }
            row.push(c);
        }
        board.push(row);
    }

    let columns = board.get(0).unwrap().len() ;
    let rows = board.len();

    Puzzle {
        board,
        columns,
        rows,
        start,
        end
    }
}

fn solve_pt1(puzzle: &Puzzle) -> i32 {
    //println!("{:?}", puzzle);
    puzzle.compute_score()
}

fn solve_pt2(puzzle: &Puzzle) -> i32 {
    puzzle.compute_pt2()
}

pub fn day16() {
    let text = fs::read_to_string("inputs/day16.txt").unwrap();
    let mut puzzle = parse_puzzle(&text);
    let soln = solve_pt1(&mut puzzle);
    println!("Solution to day 16 part 1: {}", soln);
    let soln2 = solve_pt2(&puzzle);
    println!("Solution to day 16 part 2: {}", soln2);
}

#[cfg(test)]
mod tests {
    use super::{parse_puzzle, solve_pt1, solve_pt2};

    const FIRST_SAMPLE: &str = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const SECOND_SAMPLE: &str = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    const THIRD_SAMPLE: &str = r"
###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################
";

    const FOURTH_SAMPLE: &str = r"
####################################################
#......................................#..........E#
#......................................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.............................#
#S...................#.............................#
####################################################
";
    #[test]
    fn test_first_sample() {
        let pb = parse_puzzle(&FIRST_SAMPLE);
        println!("{:?}", pb);
        assert_eq!(solve_pt1(&pb), 7036);
        assert_eq!(solve_pt2(&pb), 45);
    }

    #[test]
    fn test_second_sample() {
        let pb = parse_puzzle(&SECOND_SAMPLE);
        println!("{:?}", pb);
        assert_eq!(solve_pt1(&pb), 11048);
        assert_eq!(solve_pt2(&pb), 64);
    }

    #[test]
    fn test_third_sample() {
        let pb = parse_puzzle(&THIRD_SAMPLE);
        assert_eq!(solve_pt1(&pb), 21148);
        assert_eq!(solve_pt2(&pb), 149);
    }

    #[test]
    fn test_fourth_sample() {
        let pb = parse_puzzle(&FOURTH_SAMPLE);
        assert_eq!(solve_pt1(&pb), 5078);
        assert_eq!(solve_pt2(&pb), 413);
    }

}
