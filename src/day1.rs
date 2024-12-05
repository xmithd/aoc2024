use utils;

pub fn day1() {
    /*let text = r"3   4
4   3
2   5
1   3
3   9
3   3";*/
    let text = utils::read_file_as_text("inputs/day1.txt");    
    let (l1, l2) = utils::parse_two_lists(&text);
    let res = utils::sum_distance_bw_list(&l1, &l2);
    println!("Day 1 answer: {}", res);
    let res = utils::compute_similarity(&l1, &l2);
    println!("Day 1 part 2 answer: {}", res);
}

