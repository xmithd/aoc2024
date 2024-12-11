use std::{collections::VecDeque, fmt::{Error, Formatter}};

use itertools::Itertools;
use utils;
use std::fmt::Debug;

const RADIX: u32 = 10;

fn parse_input(txt: &str) -> Vec<Vec<usize>> {
    txt.split("\n").filter(|line| {!line.is_empty()})
    .map(|line| {
        line.chars().map( |c| { c.to_digit(RADIX).unwrap() as usize })
        .collect()
    }).collect()
}

fn get_starting_points(pb: &[Vec<usize>], start_val: usize) -> Vec<Node> {
    let mut ret = Vec::new();
    for (i, row) in pb.into_iter().enumerate() {
        for (j, &val) in row.into_iter().enumerate() {
            if val == start_val {
                ret.push(Node::new(i, j, val));
            }
        }
    }
    return ret;
}

#[derive(Clone, Copy, PartialEq, Hash)]
struct Node {
    x: i32,
    y: i32,
    val: usize
}

impl Node {
    pub fn new(row: usize, col: usize, val: usize) -> Self {
        Self {
            x: col.try_into().unwrap(),
            y: row.try_into().unwrap(),
            val: val
        }
    } 
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {} -> {})", self.x, self.y, self.val)
        //Ok(f(|| {format!("({}, {} -> {})", self.x, self.y, self.val}))
    }
}

impl Eq for Node {}

fn is_out_of_bounds((x, y): (i32, i32), rows: usize, cols: usize) -> bool {
    y as usize >= rows || x as usize >= cols || y < 0 || x < 0
}

fn count_different_tails(paths: &[Vec<Node>]) -> usize {
    //let different: HashSet<Node> = HashSet::new();
    //for path in paths {
    //    let last = path.last().unwrap();
    //}
    paths.into_iter().map(| path | path.last().unwrap() )
    .unique().count()
}

fn find_trails(pb: &[Vec<usize>], starts: &[Node], goal: usize, distinct_tails: bool) -> Vec<usize> {
    let mut ret = vec![0; starts.len()];
    // use BFS to find all trails
    for (start_num, &start_pos) in starts.into_iter().enumerate() {
        let solutions = do_bfs_p1(pb, &start_pos, goal);
        //println!("Node {:?} has paths: {:?}", start_pos, solutions);
        // solutions contains all paths. Count how many are differnt
        if distinct_tails {
            // only different endings
            ret[start_num] = count_different_tails(&solutions);
        } else {
            // all paths
            ret[start_num] = solutions.len()
        }
    }
    return ret;
}

fn do_bfs_p1(board: &[Vec<usize>], start_pos: &Node, goal: usize) -> Vec<Vec<Node>>{
    let mut soln = Vec::new();
    let rows = board.len();
    let cols = board.get(0).unwrap().len();
    let mut frontier: VecDeque<Vec<Node>> = VecDeque::new();
    frontier.push_back(vec![start_pos.clone()]);
    //let visited; // need this?
    // 4 directions to explore right, left, down, up 
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        let latest_node = current.last().unwrap();
        if latest_node.val == goal {
            // current is a solution
            soln.push(current.clone());
            continue;
        }
        for (dx, dy) in dirs {
            if !is_out_of_bounds((latest_node.x + dx, latest_node.y + dy), rows, cols) {
                let next_x = (latest_node.x + dx) as usize;
                let next_y = (latest_node.y + dy) as usize;
                let next_val = board.get(next_y).unwrap().get(next_x).unwrap();
                if *next_val == latest_node.val + 1 {
                    let next_node = Node::new(next_y, next_x, *next_val);
                    let mut exploration = current.clone();
                    exploration.push(next_node);
                    frontier.push_back(exploration);
                }
            }
        }
    }
    return soln;
}

fn solve_pt1(pb: &[Vec<usize>]) -> usize {
    let start_nodes = get_starting_points(pb, 0);
    //println!("PB is {:?}", pb);
    let trail_counts = find_trails(pb, &start_nodes, 9, true);
    return trail_counts.into_iter().sum();
}

fn solve_pt2(pb: &[Vec<usize>]) -> usize {
    let start_nodes = get_starting_points(pb, 0);
    //println!("PB is {:?}", pb);
    let trail_counts = find_trails(pb, &start_nodes, 9, false);
    return trail_counts.into_iter().sum();
}

pub fn day10() {
    let text = utils::read_file_as_text("inputs/day10.txt");
    let pb = parse_input(&text);
    let soln = solve_pt1(&pb);
    println!("Solution to day 10 part 1: {}", soln);
    let soln2 = solve_pt2(&pb);
    println!("Solution to day 10 part 2: {}", soln2);
}

#[cfg(test)]
mod test {
    use super::{solve_pt1, solve_pt2, parse_input};

    #[test]
    fn test_example() {
        let text = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let pb = parse_input(&text);
        assert_eq!(solve_pt1(&pb), 36);
        assert_eq!(solve_pt2(&pb), 81);
    }

    #[test]
    fn test_evil_one() {
        let text = r"
9999999999999999999
9999999998999999999
9999999987899999999
9999999876789999999
9999998765678999999
9999987654567899999
9999876543456789999
9998765432345678999
9987654321234567899
9876543210123456789
9987654321234567899
9998765432345678999
9999876543456789999
9999987654567899999
9999998765678999999
9999999876789999999
9999999987899999999
9999999998999999999
9999999999999999999";
        let pb = parse_input(&text);
        assert_eq!(solve_pt1(&pb), 36);
        assert_eq!(solve_pt2(&pb), 2044);
    }
}