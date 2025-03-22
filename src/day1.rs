use utils;
use std::collections::HashMap;

fn parse_two_lists(txt: &str) -> (Vec<i32>, Vec<i32>) {
    let cols: Vec<&str> = txt.split_whitespace().collect();
    if cols.len() % 2 != 0 {
        panic!("Text must contain exactly one whitespace-separated column");
    }
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    for i in 0..cols.len() {
        let val = cols[i].parse::<i32>().expect("Bad input");
        if i % 2 == 0 {
            col1.push(val);
        } else {
            col2.push(val);
        }
    }
    return (col1,col2)
}

fn sum_distance_bw_list(l1: &Vec<i32>, l2: &Vec<i32>) -> i32 {
    let mut l1sorted =  l1.clone();
    l1sorted.sort();
    let mut l2sorted = l2.clone();
    l2sorted.sort();
    l1sorted.into_iter()
    .zip(l2sorted.into_iter())
    .map(| (a,b) | distance(a,b))
    .sum::<i32>()
}

fn distance(a: i32, b: i32) -> i32 {
    return (a-b).abs();
}

fn vec_to_hashmap(vec: &Vec<i32>) -> HashMap<i32, i32>{
    let mut map = HashMap::new();
    for &num in vec.iter() {
        *map.entry(num).or_insert(0) += 1;
    }
    map
}

/// for each value in list 1
fn compute_similarity(l1: &Vec<i32>, l2: &Vec<i32>) -> i32 {
    let l2map = vec_to_hashmap(l2);
    return l1.into_iter()
    .map( | it | {
        let counted = match l2map.get(it) {
            Some(val) => val.clone(),
            None => 0,
        };
        it*counted
    })
    .sum::<i32>();
}

fn solve_pt1(l1: &Vec<i32>, l2: &Vec<i32>) -> i32 {
    sum_distance_bw_list(l1, l2)
}

fn solve_pt2(l1: &Vec<i32>, l2: &Vec<i32>) -> i32 {
    compute_similarity(l1, l2)
}

pub fn day1() {

    let text = utils::read_file_as_text("inputs/day1.txt");    
    let (l1, l2) = parse_two_lists(&text);
    let res = solve_pt1(&l1, &l2);
    println!("Day 1 answer: {}", res);
    let res = solve_pt2(&l1, &l2);
    println!("Day 1 part 2 answer: {}", res);
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2, parse_two_lists};

    #[test]
    fn test_day1() {
       let text = r"3   4
4   3
2   5
1   3
3   9
3   3";
        let (l1, l2) = parse_two_lists(&text);
        assert_eq!(solve_pt1(&l1, &l2), 11);
        assert_eq!(solve_pt2(&l1, &l2), 31);
    }
}

