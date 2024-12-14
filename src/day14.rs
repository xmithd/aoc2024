use utils;
use regex::Regex;
use std::{collections::HashSet, process::Command, thread::sleep, time::Duration};

type M2DVec = (i32, i32);

#[derive(Debug)]
struct Problem {
    list: Vec<(M2DVec, M2DVec)>,
    rows: i32,
    columns: i32,
}

fn create_board(pb: &Problem) -> Vec<Vec<i32>> {
    let mut board = vec![vec![0;pb.columns as usize];pb.rows as usize];
    for ((col,row), _) in pb.list.clone() {
        board[row as usize][col as usize] += 1;
    }
    board
}

fn print_board(board: &[Vec<i32>]) {
    for row in board {
        for &el in row {
            let to_print = if el == 0 { "." } else { &el.clone().to_string() };
            print!("{}", to_print);
        }
        println!();
    }
}

fn parse_input(txt: &str, width: i32, height: i32) -> Problem {
    let regex = Regex::new(r"p=([0-9]+),([0-9]+)\s+?v=(-?[0-9]+),(-?[0-9]+)").unwrap();

 
    let list = txt.split("\n").filter(|l| !l.is_empty())
    .map(|line| {
        let (_, [p1, p2, p3, p4]) = regex.captures_iter(line).map(|c| c.extract()).take(1).next().unwrap();
        let pos: M2DVec = (p1.parse::<i32>().unwrap(), p2.parse::<i32>().unwrap());
        let vel: M2DVec = (p3.parse::<i32>().unwrap(), p4.parse::<i32>().unwrap());
        (pos, vel)
    }).collect();

    Problem {
        list,
        rows: height,
        columns: width,
    }
}


fn simulate_with_time(pb: &Problem, time: i32)  -> (Vec<Vec<i32>>, Problem) {
    let updated: Vec<(M2DVec, M2DVec)> = pb.list.clone().into_iter()
    .map(|((x, y), (dx, dy))| {
        let new_x = (x+dx*time) % pb.columns;
        let new_y = (y + dy*time) % pb.rows;
        ((if new_x < 0 { pb.columns + new_x } else { new_x }, if new_y < 0 { pb.rows + new_y } else {new_y}), (dx, dy))
    }).collect();
    let after_board: Problem = Problem {
        list: updated,
        rows: pb.rows,
        columns: pb.columns
    };
    (create_board(&after_board), after_board)
}

fn compute_quadrant_factor(board: &[Vec<i32>], row_range: (usize, usize), col_range: (usize, usize)) -> u32 {
    let mut sum: u32= 0;
    let (row_start, row_end) = row_range;
    let (col_start, col_end) = col_range;
    for i in row_start..row_end {
        for j in col_start..col_end {
            sum += *board.get(i).unwrap().get(j).unwrap() as u32;
        }
    }
    sum
}

fn compute_safety_factor(pb: &Problem, board: &[Vec<i32>]) -> u32 {
    // number of ships per quadrant
    let rows = pb.rows as usize;
    let columns = pb.columns as usize;
    let halway_row = rows / 2; 
    let halfway_col = columns / 2;
    let q1 = compute_quadrant_factor(board, (0, halway_row), (0, halfway_col));
    let q2 = compute_quadrant_factor(board, (0, halway_row), (halfway_col+1, columns));
    let q3 = compute_quadrant_factor(board, (halway_row+1, rows), (0, halfway_col));
    let q4 = compute_quadrant_factor(board, (halway_row+1, rows), (halfway_col+1, columns));
    q1*q2*q3*q4
}

fn solve_pt1(pb: &Problem) -> u32 {
    let dt = 100;
    let (board, _) = simulate_with_time(pb, dt);
    //print_board(&board);
    compute_safety_factor(pb, &board)
}

fn _run_simulation_drawing(pb: &Problem) {
    let mut dt = 326;
    loop {
        Command::new("sh")
          .arg("-c")
          .arg("clear")
          .output().expect("Failed to run clear command");

        dt = dt+1;
        println!("time: {} seconds:", dt);
        let (board, _) = simulate_with_time(pb, dt);
        print_board(&board);
        sleep(Duration::from_millis(200));
    }
}


fn do_dfs(current: (i32, i32), board: &[Vec<i32>], visited: &mut HashSet<(i32, i32)>, line_length: u32, is_pt_valid: &dyn Fn((i32, i32)) -> bool) -> u32 
{
    if visited.contains(&current) {
        return line_length;
    }
    visited.insert(current);
    let b_val = board.get(current.1 as usize).unwrap().get(current.0 as usize).unwrap();
    if *b_val != 0 {
        let dirs = [(-1,-1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
        let mut max = line_length+1; // current line length
        for dir in dirs {
            let new_pt = (current.0 + dir.0, current.1 + dir.1);
            if is_pt_valid(new_pt) {
                let path_length = do_dfs(new_pt, board, visited, line_length+1, is_pt_valid);
                if path_length > max {
                    max = path_length;
                }
            }
        }
        return max;
    } else {
        return line_length;
    }
}

fn  find_longest_line(board: &[Vec<i32>], positions: &Problem) -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut max_l = 0;
    for (pos, _) in positions.list.clone() {
        let line_length = do_dfs(pos, board, &mut visited, 0, &|pos: (i32, i32)| {
            let x = pos.0;
            let y = pos.1;
            x >= 0 && x < positions.columns && y >= 0 && y < positions.rows
        });
        if line_length > max_l { 
            max_l = line_length
        }
    }
    return max_l;
}

fn solve_pt2(pb: &Problem) {
   let mut dt = 0;
   const THRESHOLD: u32 = 50;
   loop {
        dt = dt+1;
        let (board, updated_pb) = simulate_with_time(pb, dt);
        let b = find_longest_line(&board, &updated_pb);
        if b > THRESHOLD {
            print_board(&board);    
            break;
        }
   } 
   println!("solution to day 14 part 2: {}", dt);
}

pub fn day14() {

    let text = utils::read_file_as_text("inputs/day14.txt");
    let pb = parse_input(&text, 101, 103);
    let sol1 = solve_pt1(&pb);
    println!("Solution to day 14 part 1: {}", sol1);
    solve_pt2( &pb);

}

#[cfg(test)]

mod tests {
    use super::{solve_pt1, parse_input};

    #[test]
    fn test_example() {
        let sample = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
        let pb = parse_input(&sample, 11, 7);
        //println!("{:?}", pb);
        assert_eq!(pb.list.len(), 12);
        //assert_eq!(solve_pt1(&pb), 0);
        assert_eq!(solve_pt1(&pb), 12);
    }

    #[test]
    fn test_single_sample() {
        let single = "p=2,4 v=2,-3";
        let pb = parse_input(&single, 11, 7);
        assert_eq!(solve_pt1(&pb), 0);
    }
}

