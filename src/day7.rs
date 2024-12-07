use utils;

fn parse_input(txt: &str) -> Vec<(u64, Vec<u64>)> {
    txt.split("\n")
      .filter(|line| { !line.is_empty()})
      .map(|line| {
        let parts: Vec<&str> = line.split(":").collect();
        (parts.get(0).unwrap().parse::<u64>().unwrap(),
        parts.get(1).unwrap().trim().split_whitespace().map(|num| { num.parse::<u64>().unwrap() }).collect())
      }).collect()
}

pub fn day7() {
    /*let text = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";*/
    let text = utils::read_file_as_text("inputs/day7.txt");
    let parsed: Vec<(u64, Vec<u64>)> = parse_input(&text);
    //println!("parsed: {:?}", parsed);
    let sum_correct = parsed.iter().map(|(answ, operands)| {
        (answ, utils::find_ops(*answ, operands))
    }).filter( | (_, res)| { res.is_some() } )
    .map(|(answ, _)| {
        answ
    }).sum::<u64>();
    println!("Answer for day 7 part 1: {}", sum_correct);
}