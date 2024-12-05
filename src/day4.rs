use utils;

pub fn day4() {
    /*let text = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";*/
    let text = utils::read_file_as_text("inputs/day4.txt");
    let grid = utils::parse_letter_grid(&text);
    let res = utils::findall_in_grid(&grid, "XMAS");
    println!("Result for day4 part 1 is: {}", res); // 2591
    /*let text2 = r"
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";*/
    //let grid2 = utils::parse_letter_grid(&text);
    let ret = utils::findall_x_in_grid(&grid, "MAS");
    println!("Result for day4 part 2 is: {}", ret); // 1880
}
