use utils;

pub fn day3() {
    //let text = r"mul(5,6)don't()mul(3,5)";
    //let text = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    //let text = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let text = utils::read_file_as_text("inputs/day3.txt");
    let pairs = utils::parse_mul_pairs(&text);
    let res = utils::compute_multiplication_sum(&pairs);
    println!("Result for day3 part 1 is: {}", res); // 184122457
    let pairs = utils::parse_do_mul_pairs(&text);
    let ret = utils::compute_multiplication_sum(&pairs);
    println!("Result for day3 part 2 is: {}", ret); // 107862689
}