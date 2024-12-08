use std::collections::HashMap;
use utils;
use itertools::Itertools;

#[derive(Debug, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
    //freq: char
}

impl Point {
    pub fn new(row: usize, col: usize, _freq: char) -> Self {
        Point {
            x: col as i32,
            y: row as i32,
            //freq: freq
        }
    }

    pub fn minus(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            //freq: self.freq
        }
    }

    pub fn plus(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            //freq: self.freq
        }
    }

    pub fn within_bounds(&self, rows: usize, cols: usize) -> bool {
        self.x >= 0 && self.x < cols as i32
            && self.y >= 0 && self.y < rows as i32
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

fn parse_input(txt: &str) -> Vec<Vec<char>> {
    txt.split("\n")
        .filter(| line | { !line.is_empty() })
        .map( | line | { line.chars().collect() })
        .collect()
}

fn find_pairs(board: &[Vec<char>]) -> Vec<(Point, Point)> {
    let mut mapping: HashMap<char, Vec<Point>> = HashMap::new();
    for (i, row) in board.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if *element != '.' {
               let pt = Point::new(i, j, *element);
               if let Some(list) = mapping.get_mut(element) {
                   list.push(pt);
               } else {
                   mapping.insert(*element, vec![pt]);
               }
            }
        }
    }
    let mut pairs: Vec<(Point, Point)> = Vec::new();
    for (_element, list) in mapping {
        for combo in list.into_iter().combinations(2) {
            pairs.push((combo.get(0).unwrap().clone(), combo.get(1).unwrap().clone()));
        }
    }
    //println!("pairs found: {:?}", pairs);
    return pairs;
}

fn compute_antinodes(pairs: &[(Point, Point)]) -> Vec<Point> {
   pairs.iter().map(| (p1, p2) | {
       let diff = p2.minus(p1);
       vec![p1.minus(&diff), p2.plus(&diff)]
   }).flatten()
   .collect()
}

fn compute_antinodes_p2(pairs: &[(Point, Point)], rows: usize, cols: usize) -> Vec<Point> {
   pairs.iter().map(| (p1, p2) | {
       let diff = p2.minus(p1);
       let mut a_in_line = Vec::new();
       a_in_line.push(p1.clone());
       let mut line1 = p1.minus(&diff);
       while line1.within_bounds(rows, cols) {
           a_in_line.push(line1.clone());
           line1 = line1.minus(&diff);
       }
       a_in_line.push(p2.clone());
       let mut line2 = p2.plus(&diff);
       while line2.within_bounds(rows, cols) {
           a_in_line.push(line2.clone());
           line2 = line2.plus(&diff);
       }
       //vec![p1.minus(&diff), p2.plus(&diff)]
       return a_in_line;
   }).flatten()
   .collect()
}

fn valid_antinodes(antinodes: &[Point], rows: usize, columns: usize) -> Vec<Point> {
    antinodes.into_iter().filter(| pt | {
        pt.within_bounds(rows, columns)
    })
    .unique()
    .map(|pt|{ pt.clone() })
    .collect()
}

fn solve_pt1(board: &[Vec<char>]) -> usize {
    let pairs = find_pairs(board);
    let antinodes = compute_antinodes(&pairs);
    //println!("Antinodes: {:?}", antinodes);
    let cols = board.get(0).unwrap().len();
    let valids = valid_antinodes(&antinodes, board.len(), cols);
    // draw for debugging:
    //for (i, row) in board.iter().enumerate() {
    //    for (j, element) in row.iter().enumerate() {
    //        if *element == '.' &&  valids.contains(&Point::new(i, j, '_')) {
    //            print!("#");
    //        } else {
    //            print!("{}", element);
    //        }
    //    }
    //    print!("\n");
    //}

    return valids.len();
}

fn solve_pt2(board: &[Vec<char>]) -> usize {
    let pairs = find_pairs(board);
    let cols = board.get(0).unwrap().len();
    let rows = board.len();
    let antinodes = compute_antinodes_p2(&pairs, rows, cols);
    //println!("Antinodes: {:?}", antinodes);
    let valids = valid_antinodes(&antinodes, rows, cols);
    // draw for debugging:
    //for (i, row) in board.iter().enumerate() {
    //    for (j, element) in row.iter().enumerate() {
    //        if *element == '.' && valids.contains(&Point::new(i, j, *element)) {
    //            print!("#");
    //        } else {
    //            print!("{}", element);
    //        }
    //    }
    //    print!("\n");
    //}

    return valids.len();
}


pub fn day8() {
    let text = utils::read_file_as_text("inputs/day8.txt");
    /*let text = r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";*/
    let board = parse_input(&text);
    let sol_p1 = solve_pt1(&board);
    println!("answer day 8 part 1: {}", sol_p1); // 220
    let sol_p2 = solve_pt2(&board);
    println!("answer to day 8 part 2: {}", sol_p2); // 818
}
