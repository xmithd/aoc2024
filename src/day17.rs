use std::fs;

enum Instruction {
    ADV=0,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV
}

fn parse_instruction(i: u64) -> Instruction {
    match i {
        0 => Instruction::ADV,
        1 => Instruction::BXL,
        2 => Instruction::BST,
        3 => Instruction::JNZ,
        4 => Instruction::BXC,
        5 => Instruction::OUT,
        6 => Instruction::BDV,
        7 => Instruction::CDV,
        _ => panic!("Bad instruction {}!", i)
    }
}

// identity op
fn lit_operand(op: u64) -> u64 {
    op
}

fn combo_operand(state: &State, op: u64) -> u64 {
    match op {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("Invalid program asking combo operand {}!", op)
    }
}

#[derive(Debug, Clone)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    output: Vec<u64>,
    func_ptr: usize,
}

impl State {
    pub fn operate(&self, i: Instruction, op: u64) -> State {
        let mut state = self.clone();
        match i {
            Instruction::ADV => {
                state.a = (self.a)/2_u64.pow(combo_operand(self, op) as u32);
            },
            Instruction::BXL => {
                state.b = self.b ^ lit_operand(op);
            },
            Instruction::BST => {
                state.b = combo_operand(self, op) % 8;
            },
            Instruction::JNZ => {
                if self.a != 0 {
                    state.func_ptr = lit_operand(op) as usize;
                    return state;
                }
            },
            Instruction::BXC => {
                state.b = self.b ^ self.c;
            },
            Instruction::OUT => {
                state.output = [self.output.clone(), [combo_operand(self, op) % 8].to_vec()].concat()
            },
            Instruction::BDV => {
                state.b = (self.a)/2_u64.pow(combo_operand(self, op) as u32)
            },
            Instruction::CDV => {
                state.c = self.a/2_u64.pow(combo_operand(self, op) as u32)
            }
        };
        state.func_ptr += 2;
        state
    }

    pub fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn print_output(&self) {
        for i in &self.output {
            print!("{},", i);
        }
        println!("")
    }
}

#[derive(Debug, Clone)]
struct Problem {
    a: u64,
    b: u64,
    c: u64,
    pg: Vec<u64>
}

// avoid infinite loop program
const COUNT_LIMIT: u64 = 100000000;

impl Problem {

    pub fn initial_state(&self, register_a: u64) -> State {
        State {
            a: register_a,
            b: self.b,
            c: self.c,
            output: vec![],
            func_ptr: 0,
        }
    }

    pub fn run_program(&self, initial_state: &State) -> State {
        let mut state = initial_state.clone();
        let mut counter = 0;
        while state.func_ptr <= self.pg.len() - 2 {
            if counter >= COUNT_LIMIT {
                panic!("Counter exceeded!");
            }
            let i = parse_instruction(*self.pg.get(state.func_ptr).unwrap());
            let op = *self.pg.get(state.func_ptr+1).unwrap();
            state = state.operate(i, op);
            counter += 1;
        }
        state
    }

    // tried to end early if wanted output was not matching but program kept running
    pub fn _run_debug(&self, initial_state: &State, wanted_output: &[u64]) -> Option<State> {
        let mut state = initial_state.clone();
        let mut counter = 0;
        //println!("\n Register A: {}", initial_state.a);
        while state.func_ptr <= self.pg.len() - 2 {
            if counter >= COUNT_LIMIT {
                panic!("Counter exceeded!");
            }
            let i = parse_instruction(*self.pg.get(state.func_ptr).unwrap());
            let op: u64 = *self.pg.get(state.func_ptr+1).unwrap();
            let new_state = state.operate(i, op);
            if new_state.output.len() > state.output.len() {
              if let Some(out) = new_state.output.last() {
                  //print!("{},", out);
                  if new_state.output.len() > wanted_output.len() {
                      return None;
                  }
                  if out != wanted_output.get(new_state.output.len()-1).unwrap() {
                    return None;
                  }
              }
              if new_state.output.len() > wanted_output.len()-2 {
                println!("A: {}", initial_state.a);
                new_state.print_output();
              }
            }
            state = new_state;
            
            counter += 1;
        }
        if state.output == wanted_output {
            Some(state)
        } else {
            None
        }
    }
}

fn parse_problem(text: &str) -> Problem {
    let mut program_turn = false;
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut pg = Vec::new();
    for line in text.trim().split("\n") {
        if line.is_empty() {
            program_turn = true;
            continue;
        }
        let parts: Vec<_> = line.split(":").collect();
        let label = parts[0];
        if program_turn {
            pg = parts[1].split(",").map(|it| it.trim().parse::<u64>().unwrap()).collect();
        } else {
            let val = parts[1].trim().parse::<u64>().unwrap();
            if label.contains("Register A") {
                 register_a = val;
            } else if label.contains("Register B") {
                register_b = val;
            } else if label.contains("Register C") {
                register_c = val;
            }
        }
    }
    Problem {
        a: register_a,
        b: register_b,
        c: register_c,
        pg: pg
    }
}

fn solve_pt1(pb: &Problem) -> String {
    //println!("{:?}", pb);
    let initial_state = pb.initial_state(pb.a);
    pb.run_program(&initial_state).get_output()
}

// brute force approach
fn _solve_pt2_naive(pb: &Problem) -> u64 {
    let mut register_a = 0;
    loop {
        let mut state = pb.initial_state(register_a);
        state = pb.run_program(&state);
        if state.output == pb.pg {
            break;
        }
        register_a += 1;
    }
    register_a
}

fn solve_pt2(pb: &Problem) -> Option<u64> {
    let mut current_round = (0..8).collect();
    let mut next_round = vec![];
    // needed help for this one
    for _ in 1..pb.pg.len() {
       for base in current_round {
         for final_bits in 0..8 {
            let register_a = 8 * base + final_bits;
            let initial_state = pb.initial_state(register_a);
            let state = pb.run_program(&initial_state);
            let l = state.output.len();
            let goal_l = pb.pg.len();
            let slice = &pb.pg[goal_l-l..goal_l];
            // noticed the pattern at the end
            if state.output == slice {
               //state.print_output();
               // try these at the next round
               next_round.push(register_a);
            }
            if state.output == pb.pg {
                return Some(register_a);
            }
         }
       }
       current_round = next_round.clone();
       next_round = vec![];
    }
    None
}


pub fn day17() {
    let text = fs::read_to_string("inputs/day17.txt").unwrap();
    let pb = parse_problem(&text);
    let soln = solve_pt1(&pb);
    println!("Solution to day 17 part 1: {}", soln);
    if let Some(soln2) = solve_pt2(&pb) {
    println!("Solution to day 17 part 2: {}", soln2);
    } else {
        println!("No solution for day 17 part 2??!?!")
    }
}

#[cfg(test)]
mod tests {

use super::{solve_pt1, parse_problem, lit_operand, solve_pt2};

    const SAMPLE: &str = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn test_functions() {
        assert_eq!(lit_operand(0), 0);
    }
    #[test]
    fn test_sample() {
        let pb = parse_problem(&SAMPLE);
        assert_eq!(solve_pt1(&pb), "4,6,3,5,6,3,5,2,1,0");
    }

    const SAMPLE2: &str = r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
    #[test]
    fn test_sample2() {
        let pb = parse_problem(&SAMPLE2);
        assert_eq!(solve_pt2(&pb).unwrap(), 117440);
    }

}
