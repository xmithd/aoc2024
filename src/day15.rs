use std::fs;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Error, Formatter};
use std::fmt::Debug;

const BOX: char = 'O';

// for part 2:
const BOX_R: char = ']';
const BOX_L: char = '[';

fn get_move_direction(move_char: char) -> (i32, i32) {
    match move_char {
        '^' => (0, -1),
        '>' => (1, 0),
        '<' => (-1, 0),
        'v' => (0, 1),
        _ => panic!("unknown move")
    }
}


//#[derive(Debug)]
struct Puzzle {
    board: Vec<Vec<char>>,
    columns: usize,
    rows: usize,
    moves: VecDeque<char>,
    robot: (i32 ,i32)
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for i in 0..self.rows {
            for j in 0..self.columns {
                let tile = self.get_usize(j, i);
                write!(f, "{}", tile).expect("err...");
            }
            write!(f, "\n").expect("err...");
        }
        write!(f, "")
    }
}

// returns first empty spot (if any) and if last one was a boulder
fn can_move(start_pos: (i32, i32), dir: (i32, i32), board: &[Vec<char>], is_boulder: bool) -> Option<((i32, i32), bool)> {
    let new_pos = (start_pos.0+dir.0, start_pos.1 + dir.1);
    match board.get(new_pos.1 as usize).unwrap().get(new_pos.0 as usize).unwrap() {
        '#' => None,
        '.' => Some((new_pos, is_boulder)),
        &BOX => can_move(new_pos, dir, board, true),
        _ => None
    }
}

fn update_bot(start: (i32, i32), move_char: char, board: &mut [Vec<char>]) -> (i32, i32) {
    let dir = get_move_direction(move_char);
    match can_move(start, dir, board, false) {
        None => start.clone(),
        Some((pos, is_boulder)) => {
            // draw empty space where robot was
            board[start.1 as usize][start.0 as usize] = '.';
            if is_boulder {
                // draw boulder at pos and robot at start+dir
                board[pos.1 as usize][pos.0 as usize] = BOX;
                let new_pos = (start.0 + dir.0, start.1 + dir.1);
                board[new_pos.1 as usize][new_pos.0 as usize] = '@';
                new_pos
            } else {
                // draw robot at pos
                board[pos.1 as usize][pos.0 as usize] = '@';
                pos
            }
        }
    }
}

// returns first empty spot (if any) and if last one was a boulder
fn can_move_2(start_pos: (i32, i32), dir: (i32, i32), board: &[Vec<char>], prev_char: char, updated_info: &mut HashMap<(i32, i32), char>) -> Option<Vec<(i32, i32)>> {
    let new_pos = (start_pos.0+dir.0, start_pos.1 + dir.1);
    updated_info.insert(new_pos, prev_char);
    let current_char = *board.get(new_pos.1 as usize).unwrap().get(new_pos.0 as usize).unwrap();
    let is_vertical = dir.1 == 1 || dir.1 == -1;
    if !is_vertical { // similar to v1 of can_move (will only take 1 empty spot)
        match current_char {
            '#' => None,
            '.' => Some(vec![new_pos]),
            BOX_L => can_move_2(new_pos, dir, board, current_char, updated_info),
            BOX_R => can_move_2(new_pos, dir, board, current_char, updated_info),
            _ => None
        }
    } else {
    match current_char {
        '#' => None,
        '.' => Some(vec![new_pos]),
        BOX_L => {
            let right_pos = (new_pos.0+1, new_pos.1);
            if !updated_info.contains_key(&right_pos) {
                updated_info.insert(right_pos, '.'); // only add if not already taken care of
            }
            let left = can_move_2(new_pos, dir, board, current_char, updated_info);
            let right = can_move_2(right_pos, dir, board, BOX_R, updated_info);
            if left.is_none() || right.is_none() {
                None
            } else {
                // return some with combined vectors
                let left = left.unwrap();
                let right = right.unwrap();
                Some([left, right].concat())
            }
        },
        BOX_R => {
            // TODO get one to the left and check if can move
            let left_pos = (new_pos.0-1, new_pos.1);
            if !updated_info.contains_key(&left_pos) {
                updated_info.insert(left_pos, '.'); // only add if not already taken care of
            }
            let right = can_move_2(new_pos, dir, board, current_char, updated_info);
            let left = can_move_2(left_pos, dir, board, BOX_L, updated_info);
            if left.is_none() || right.is_none() {
                None
            } else {
                // return some with combined vectors
                let left = left.unwrap();
                let right = right.unwrap();
                Some([left, right].concat())
            }
        },
        _ => None
    }
    }
}

fn update_bot_p2(start: (i32, i32), move_char: char, board: &mut [Vec<char>]) -> (i32, i32) {
    let dir = get_move_direction(move_char);
    let mut chars_to_update: HashMap<(i32, i32), char> = HashMap::new();
    match can_move_2(start, dir, board, '@', &mut chars_to_update) {
        None => {
            //println!("Move {} - no update", move_char);
            start.clone()
        },
        Some(_positions) => {
            // draw empty space where robot was
            board[start.1 as usize][start.0 as usize] = '.';
            let new_bot = (start.0 + dir.0, start.1 + dir.1);
            //board[new_bot.1 as usize][new_bot.0 as usize] = '@';
            //println!("Move {} to update position(s) {:?}", move_char, positions);
            for ((x, y), v) in chars_to_update {
                board[y as usize][x as usize] = v;
            }
            new_bot
        }
    }
}

impl Puzzle {
    pub fn _get(&self, col: i32, row: i32) -> char {
        self.get_usize(col as usize, row as usize)
    }

    pub fn get_usize(&self, col: usize, row: usize) -> char {
        *self.board.get(row).unwrap().get(col).unwrap()
    }

    pub fn run_moves(&mut self) {
        let mut board = &mut self.board;
        let moves = &mut self.moves;
        while !moves.is_empty() {
            let move_char = moves.pop_front().unwrap();
            self.robot = update_bot(self.robot, move_char, &mut board);
        }
    }

    pub fn run_moves_2(&mut self) {
        //let mut board = &mut self.board;
        //let moves = &mut self.moves;
        //let mut _buff: String = "\n".to_string();
        while !self.moves.is_empty() {
            //println!("{:?}", &self);
            //std::io::stdin().read_line(&mut _buff).expect("Did not enter a correct string");
            let move_char = self.moves.pop_front().unwrap();
            self.robot = update_bot_p2(self.robot, move_char, &mut self.board);
        }
    }

    pub fn compute_coords(&self) -> usize {
        let board = &self.board;
        let mut sum = 0;
        for (i, row) in board.into_iter().enumerate() {
            for (j, tile) in row.into_iter().enumerate() {
                if *tile == BOX {
                    sum += i*100 + j;
                }
            }
        }
        sum
    }

    pub fn compute_coords_2(&self) -> usize {
        let board = &self.board;
        let mut sum = 0;
        for (i, row) in board.into_iter().enumerate() {
            for (j, tile) in row.into_iter().enumerate() {
                if *tile == BOX_L {
                    sum += i*100 + j;
                }
            }
        }
        sum
    }
}

fn parse_puzzle(str: &str) -> Puzzle {
    let mut parse_moves = false;
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut moves: VecDeque<char> = VecDeque::new();
    let mut robot: (i32, i32) = (0, 0);
    for (i, line) in str.trim().split("\n").enumerate() {
        if line.is_empty() {
            parse_moves = true;
            continue;
        }
        if !parse_moves {
            let mut row: Vec<char> = Vec::new();
            for (j, c) in line.chars().enumerate() {
                if c == '@' {
                    robot = (j as i32, i as i32);
                }
                row.push(c);
            }
            board.push(row);
        } else {
            // push move characters to the queue
            for move_char in line.chars() {
                moves.push_back(move_char);
            }
        }
    }

    let columns = board.get(0).unwrap().len() ;
    let rows = board.len();

    Puzzle {
        board,
        columns,
        rows,
        moves,
        robot
    }
}

fn parse_puzzle_pt2(str: &str) -> Puzzle {
    let mut parse_moves = false;
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut moves: VecDeque<char> = VecDeque::new();
    let mut robot: (i32, i32) = (0, 0);
    for (i, line) in str.trim().split("\n").enumerate() {
        if line.is_empty() {
            parse_moves = true;
            continue;
        }
        if !parse_moves {
            let mut row: Vec<char> = Vec::new();
            for (j, c) in line.chars().enumerate() {
                if c == BOX {
                    row.push(BOX_L);
                    row.push(BOX_R);
                } else {
                    let mut extra = '.';
                    if c == '@' {
                        robot = (2*j as i32, i as i32);
                    } else if c == '#' {
                        extra = '#';
                    }
                    row.push(c);
                    row.push(extra);
                }
            }
            board.push(row);
        } else {
            // push move characters to the queue
            for move_char in line.chars() {
                moves.push_back(move_char);
            }
        }
    }

    let columns = board.get(0).unwrap().len() ;
    let rows = board.len();

    Puzzle {
        board,
        columns,
        rows,
        moves,
        robot
    }
}


fn solve_pt1(puzzle: &mut Puzzle) -> usize {
    puzzle.run_moves();
    //println!("{:?}", puzzle);
    puzzle.compute_coords()
}

fn solve_pt2(puzzle: &mut Puzzle) -> usize {
    puzzle.run_moves_2();
    puzzle.compute_coords_2()
}

pub fn day15() {
    let text = fs::read_to_string("inputs/day15.txt").unwrap();
    let mut puzzle = parse_puzzle(&text);
    let soln = solve_pt1(&mut puzzle);
    println!("Solution to day 15 part 1: {}", soln);
    puzzle = parse_puzzle_pt2(&text);
    let soln2 = solve_pt2(&mut puzzle);
    println!("Solution to day 15 part 2: {}", soln2);
}

#[cfg(test)]
mod tests {
    use super::{parse_puzzle, parse_puzzle_pt2, solve_pt1, solve_pt2};

    const SIMPLE_SAMPLE: &str = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>
v<<";

    const SAMPLE: &str = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    #[test]
    fn test_small_sample() {

      let mut pb = parse_puzzle(&SIMPLE_SAMPLE);
      println!("{:?}", pb);
      assert_eq!(solve_pt1(&mut pb), 2028);
    }

    #[test]
    fn test_larger_sample() {
        
      let mut pb = parse_puzzle(&SAMPLE);
      println!("{:?}", pb);
      assert_eq!(solve_pt1(&mut pb), 10092);
    }

    #[test]
    fn test_part2_small() {
        let part2_sample = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";
        let mut pb = parse_puzzle_pt2(&part2_sample);
        println!("{:?}", pb);
        let ans = solve_pt2(&mut pb);
        println!("{:?}", pb);
        assert_eq!(ans, 618);
    }

    #[test]
    fn test_part2() {
        
      let mut pb = parse_puzzle_pt2(&SAMPLE);
      println!("{:?}", pb);
      let ans = solve_pt2(&mut pb);
      println!("{:?}", pb);
      assert_eq!(ans, 9021);
    }

}
