use std::collections::{HashMap, HashSet};

use utils;

fn get_direction_vec(dir: DIRECTION) -> (i32, i32) {
    match dir {
        DIRECTION::NORTH => (0, -1),
        DIRECTION::EAST => (1, 0),
        DIRECTION::SOUTH => (0, 1),
        DIRECTION::WEST => (-1, 0)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum DIRECTION {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

enum TILE {
    NOTHING,
    GUARD(DIRECTION),
    WALL,
    UNKNOWN, // in case we add padding
}

struct Game {
   board: Vec<Vec<TILE>>,
   guard_pos: (i32, i32),
   guard_orientation: DIRECTION,
   rows: usize,
   cols: usize,
   init_pos: (i32, i32),
   init_orientation: DIRECTION,
}

impl Game {
    fn parse(txt: &str) -> Self {
        let parsed : Vec<Vec<TILE>> = txt.split("\n")
            .filter(|line| { !line.is_empty() })
            .map(|line| {
                line.chars().map(|c|
                {
                    let tile = match c {
                        '^' => TILE::GUARD(DIRECTION::NORTH),
                        '>' => TILE::GUARD(DIRECTION::EAST),
                        '<' => TILE::GUARD(DIRECTION::WEST),
                        'v' => TILE::GUARD(DIRECTION::SOUTH),
                        '#' => TILE::WALL,
                        '.' => TILE::NOTHING,
                        _ => TILE::UNKNOWN
                    };
                return tile;
                }).collect()
            }).collect();
        let mut pos: (i32, i32) = (-1,-1);
        let mut orientation: DIRECTION = DIRECTION::NORTH;
        for (i, row) in parsed.iter().enumerate() {
            for j in 0..row.len() {
                if let Some(TILE::GUARD(dir)) = row.get(j) {
                    pos = (j as i32, i as i32);
                    orientation = dir.clone();
                    break;
                }
            }
        }
        let rows = parsed.len();
        let cols = parsed.get(0).unwrap().len();
        Game {
            board: parsed,
            guard_pos: pos,
            guard_orientation: orientation,
            rows,
            cols,
            init_pos: pos,
            init_orientation: orientation,
        }
    }

    pub fn rotate_guard(&mut self) {
        self.guard_orientation = match self.guard_orientation {
            DIRECTION::EAST => DIRECTION::SOUTH,
            DIRECTION::NORTH => DIRECTION::EAST,
            DIRECTION::SOUTH => DIRECTION::WEST,
            DIRECTION::WEST => DIRECTION::NORTH 
        };
    }

    pub fn set_empty(&mut self, x: i32, y: i32) {
        if let Some(row) = self.board.get(y as usize) {
            if let Some(_cell) = row.get(x as usize) {
                self.board[y as usize][x as usize] = TILE::NOTHING;
            }
        }
    }

    fn set_tile(&mut self, row: usize, col: usize, tile: TILE) {
        self.board[row][col] = tile;
    }

    pub fn set_obstacle(&mut self, row: usize, col: usize) {
        self.set_tile(row, col, TILE::WALL);
    }

    pub fn remove_obstacle(&mut self, row: usize, col: usize) {
        self.set_tile(row, col, TILE::NOTHING);
    }

    pub fn reset(&mut self) {
        let (x, y) = self.guard_pos;
        self.board[y as usize][x as usize] = TILE::NOTHING;
        let (j, i) = self.init_pos;
        self.board[i as usize][j as usize] = TILE::GUARD(self.init_orientation);
        self.guard_pos = self.init_pos;
        self.guard_orientation = self.init_orientation;
    }

}

// return true on loop detected
fn run_simulation(game: &mut Game) -> (HashMap<i32, HashSet<DIRECTION>>, bool) {
    // add current starting position to the visited map
    let mut visited = HashMap::new();
    visited.insert(game.guard_pos.0 + (game.cols as i32)*game.guard_pos.1, HashSet::from([game.guard_orientation]));
    loop {
        let (x, y) = game.guard_pos;
        let (dirx, diry) = get_direction_vec(game.guard_orientation);
        let new_pos = (x+dirx, y+diry);
        // out of boundary -> stop condition and no looping detected
        if new_pos.0 < 0 || new_pos.0 >= game.cols as i32 {
            break;
        } else if new_pos.1 < 0 || new_pos.1 >= game.rows as i32 {
            break;
        }
        if let Some(row) = game.board.get(new_pos.1 as usize)  {
            if let Some(cell) = row.get(new_pos.0 as usize) {
                match cell {
                    TILE::NOTHING => {
                        // move to the new pos
                        game.set_empty(x, y);
                        game.guard_pos = new_pos;
                        // add to visited map
                        let raw_pos = new_pos.0 + (game.cols as i32)*new_pos.1;
                        if let Some(node) = visited.get_mut(&raw_pos) {
                            if node.contains(&game.guard_orientation) {
                                // loop detected
                                return (visited, true);
                            } else {
                                node.insert(game.guard_orientation);
                            }
                        } else {
                            visited.insert(raw_pos, HashSet::from([game.guard_orientation]));
                        }
                        game.board[new_pos.1 as usize][new_pos.0 as usize] = TILE::GUARD(game.guard_orientation);
                    },
                    TILE::WALL => {
                        game.rotate_guard();
                    },
                    TILE::UNKNOWN => {},
                    _ => {}
                }
            }
        }
    }
    return (visited, false);
}

fn count_ways_to_block(game: &mut Game, original_path: &HashMap<i32, HashSet<DIRECTION>>) -> i32 {
    let mut possible_blocks = 0;
    for (key, _) in original_path {
        let j = (key % (game.cols as i32)) as usize;
        let i = (key / (game.rows as i32)) as usize;
        game.reset();
        let tile = game.board.get(i).unwrap().get(j).unwrap();
        match tile {
            TILE::WALL => { continue; },
            TILE::NOTHING => {
                game.set_obstacle(i, j);
            }, 
            TILE::GUARD(_) => { continue; },
            TILE::UNKNOWN => { continue; }
        }
        let (_, looped) = run_simulation(game);
        if looped {
            possible_blocks += 1;
        }
        game.remove_obstacle(i, j);

    }
    return possible_blocks;
}

pub fn day6() {
    let text = utils::read_file_as_text("inputs/day6.txt");
    /*let text = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";*/
    let mut game = Game::parse(&text);
    let (visited, _found_loop) = run_simulation(&mut game);
    println!("answer day 6 part 1: {}", visited.len()); // 5212
    game.reset();
    let c = count_ways_to_block(&mut game, &visited);
    println!("answer to day 6 part 2: {}", c); // 1767
}