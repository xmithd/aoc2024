use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const RIGHT: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (-1, 0);
const DIRECTIONS: [(i32, i32);4] = [RIGHT, LEFT, UP, DOWN];

fn is_within_bounds(pt: &(usize, usize), dir: (i32, i32), rows: usize, columns: usize) -> bool {
    //pt is x, y (or j, i or col, row)
    let (x, y) = pt;
    let (dx, dy) = dir;
    let new_pos_x: i32 = *x as i32 + dx;
    let new_pos_y: i32 = *y as i32 + dy;
    let max_row = rows as i32 - 1;
    let max_column = columns as i32 - 1;
    return (new_pos_x <= max_column) && (new_pos_x >= 0) && (new_pos_y <= max_row) && (new_pos_y >= 0);
}

fn read_file(filepath: &str) -> String {
    fs::read_to_string(filepath).unwrap()
}

fn parse_input(text: &str) -> Vec<Vec<char>> {
    text.split("\n")
        .filter(|line| { !line.is_empty() })
        .map(|line| { line.chars().into_iter().collect() })
        .collect()
}

fn get_points_map(pb: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut mapping = HashMap::new();
    for (i, row) in pb.into_iter().enumerate() {
        for (j, &val) in row.into_iter().enumerate() {
            mapping.entry(val).or_insert(vec![])
            .push((j, i));
        }
    }
    mapping
}

fn new_point(pt: &(usize, usize), dir: (i32, i32)) -> (usize, usize) {
    let (old_x, old_y) = pt;
    let (dx, dy) = dir;
    return ((*old_x as i32 + dx) as usize, (*old_y as i32 + dy) as usize);
}

fn get_connected_components(plot: char, point: (usize, usize), board: &[Vec<char>]) -> HashSet<(usize, usize)> {
// use BFS to get connected components
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    //let mut visited: HashMap<(usize, usize), i32> = HashMap::new();
    //let mut frontier: VecDeque<Vec<(usize, usize)>> = VecDeque::from(points.into_iter().map(|pt| {vec![*pt]}).collect::<Vec<_>>());
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::from(vec![point]);
    let rows = board.len();
    let columns = board.get(0).unwrap().len();
    // TODO region labeling
    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        //let latest = current.last().unwrap();
        if visited.contains(&current) {
            continue;
        } else {
            visited.insert(current);
        }
        for dir in DIRECTIONS {
            if is_within_bounds(&current, dir, rows, columns) {
                //let mut grown_list = current.clone();
                let newone = new_point(&current, dir);
                //println!("new point {}, {}", x, y);
                let (x, y) = newone;
                if plot == *board.get(y).unwrap().get(x).unwrap() {
                    //grown_list.push((x, y));
                    //frontier.push_back(grown_list);
                    frontier.push_back(newone);
                }
            }
        }
    }
    //println!("plot {} Visited labels: {:?}", plot, visited);
    return visited;
    //return frontier.into_iter().collect();
    //return frontier.collect();
}

// (area, perimeter)
fn get_area_perimeter(region: &HashSet<(usize, usize)>, rows: usize, columns: usize) -> (usize, usize) {
    let mut perimeter: i32 = 0;
    for point in region {
        perimeter += 4;
        for dir in DIRECTIONS {
            if is_within_bounds(&point, dir, rows, columns) {
                let pt = new_point(&point, dir);
                if region.contains(&pt) {
                    // contains point
                    perimeter -= 1;
                }
            }
        }
    }

    (region.len(), perimeter as usize)
}

fn get_area_sides(region: &HashSet<(usize, usize)>, rows: usize, columns: usize) -> (usize, usize) {
    let mut _visited: HashSet<(usize, usize)> = HashSet::new();
    let mut _sides: i32 = 0;
    let mut vertices: i32 = 0;
    let mut inner_vertices: i32 = 0;
    if region.len() <= 2 {
        return (region.len(), 4);
    }


    for point in region {
        // count vertices (ugly but it works!)
        let mut up = false;
        let mut down = false;
        let mut left = false;
        let mut right = false;
        if is_within_bounds(&point, UP, rows, columns) {
            let pt = new_point(&point, UP);
            if region.contains(&pt) {
                up = true;
            }
        }
        if is_within_bounds(&point, DOWN, rows, columns) {
            let pt = new_point(&point, DOWN);
            if region.contains(&pt) {
                down = true;
            }
        }
        if is_within_bounds(&point, LEFT, rows, columns) {
            let pt = new_point(&point, LEFT);
            if region.contains(&pt) {
                left = true;
            }
        }
        if is_within_bounds(&point, RIGHT, rows, columns) {
            let pt = new_point(&point, RIGHT);
            if region.contains(&pt) {
                right = true;
            }
        }
        // if only 1 neighbour => 2 vertices
        let num_neighbours = [up, down, left, right].into_iter().filter(|it| { *it }).count();
        if  num_neighbours == 1 {
            vertices += 2;
            continue;
        } else if num_neighbours == 2 {
            if up && right || up && left || down && right || down && left {
                vertices += 1;
            }
        }
        // check diagonals
        if up && right {
            let topright = (UP.0 + RIGHT.0, UP.1 + RIGHT.1);
            let pt = new_point(&point, topright);
            if !region.contains(&pt) {
                inner_vertices += 1;
            }
        }
        if up && left {
            let topleft = (UP.0 + LEFT.0, UP.1 + LEFT.1);
            let pt = new_point(&point, topleft);
            if !region.contains(&pt) {
                inner_vertices += 1;
            }
        }
        if down && left {
            let bottomleft = (DOWN.0 + LEFT.0, DOWN.1 + LEFT.1);
            let pt = new_point(&point, bottomleft);
            if !region.contains(&pt) {
                inner_vertices += 1;
            }
        }
        if down && right {
            let bottomright = (DOWN.0 + RIGHT.0, DOWN.1 + RIGHT.1);
            let pt = new_point(&point, bottomright);
            if !region.contains(&pt) {
                inner_vertices += 1;
            }
        }

    }

    return (region.len(), (vertices+inner_vertices) as usize)
}

fn get_regions_list(pb: &[Vec<char>]) -> Vec<HashSet<(usize, usize)>> {
    //println!("{:?}", pb);
    let graph = get_points_map(pb);
    let mut label: i32 = 0;
    let mut regions : HashMap<(usize, usize), i32> = HashMap::new();
    let mut inverted_regions: Vec<HashSet<(usize, usize)>>= Vec::new();
    for (key, val) in graph {
        for point in val {
              if !regions.contains_key(&point) {
                let connected_pts = get_connected_components(key, point, pb);
                label = label+1;
                for point in &connected_pts {
                    regions.insert(*point, label);
                }
                //println!("Plots {}: {:?}", key, &connected_pts);
                inverted_regions.push(connected_pts);
            }
        }
    }
    return inverted_regions;
}

fn solve_pt1(pb: &[Vec<char>]) -> usize {
    let rows = pb.len();
    let columns = pb.get(0).unwrap().len();
    let regions = get_regions_list(pb);
    //println!("Regions: {:?}", regions);
    regions.into_iter().map(| region | {
        get_area_perimeter(&region, rows, columns)
    })
    .map( |(area, perimeter)| { area*perimeter })
    .sum()
}

fn solve_pt2(pb: &[Vec<char>]) -> usize {
    let rows = pb.len();
    let columns = pb.get(0).unwrap().len();
    let regions = get_regions_list(pb);
    regions.into_iter().map(| region | {
        get_area_sides(&region, rows, columns)
    })
    .map( |(area, sides)| { area*sides })
    .sum()
}

pub fn day12() {
    let text = read_file("inputs/day12.txt");
    let pb = parse_input(&text);
    let soln = solve_pt1(&pb);

    println!("Solution to day 12 part 1: {}", soln); // 1457298
    let soln2 = solve_pt2(&pb);
    println!("Solution to day 12 part 2: {}", soln2); // 921636
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, parse_input, solve_pt2, get_regions_list, get_area_sides};

    #[test]
    fn test_sample() {
        let sample = r"
AAAA
BBCD
BBCC
EEEC
";
        let pb = parse_input(&sample);
        assert_eq!(solve_pt1(&pb), 140);
        assert_eq!(solve_pt2(&pb), 80);
    }

    #[test]
    fn test_contained_sample() {
        let sample = r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";      let pb = parse_input(&sample);
        assert_eq!(solve_pt1(&pb), 772);
    }

    #[test]
    fn test_larger_example() {
        let sample = r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        let pb = parse_input(&sample);
        assert_eq!(solve_pt1(&pb), 1930);
        assert_eq!(solve_pt2(&pb), 1206);
    }

    #[test]
    fn test_pt2_e() {
        let sample = r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

        let pb = parse_input(&sample);
        assert_eq!(solve_pt2(&pb), 236);
    }

    #[test]
    fn test_pt2_last() {
        let sample = r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

        let pb = parse_input(&sample);
        assert_eq!(solve_pt2(&pb), 368);
    }

    #[test]
    fn test_regions_sides() {
        let sample = r"
XXXX
XXAA
AAAA
";
        let pb = parse_input(&sample);
        let regions = get_regions_list(&pb);
        assert_eq!(regions.len(), 2);
        let (area, sides) = get_area_sides(regions.get(0).unwrap(), pb.len(), pb.get(0).unwrap().len());
        assert_eq!(area, 6);
        assert_eq!(sides, 6);
    }
}
