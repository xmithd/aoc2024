use utils;

pub fn day2() {
    let input = utils::read_file_as_text("inputs/day2.txt");
    /*let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";*/
    let reports = utils::parse_line_by_line(&input);
    let ret = utils::check_safety(&reports);
    println!("number of safe reports: {}", ret);
    let ret2 = utils::check_safety_p2(&reports);
    println!("number of safe reports with dampener: {}", ret2);
}
