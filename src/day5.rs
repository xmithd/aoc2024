use utils;

pub fn day5() {
    let text = utils::read_file_as_text("inputs/day5.txt");
    /*let text = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";*/
    let (rules, printing_pages) = utils::parse_page_order_pb(&text);
    let middles = utils::middle_of_correct_orders(&printing_pages, &rules);
    println!("answer day 5 part 1: {}", middles.iter().sum::<i32>());
    let fixed_middles = utils::middle_of_corrected_orders(&printing_pages, &rules);
    println!("answer day 5 part 2: {}", fixed_middles.iter().sum::<i32>());
}