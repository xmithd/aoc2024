use std::fs;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(Debug)]
struct Puzzle {
    graph: HashMap<String, Vec<String>>
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Triplets {
   tuple : [String; 3]
}

impl Triplets {
    pub fn new(input: &[&String; 3]) -> Self {
        let mut items = Vec::from(input);
        items.sort();
        Self {
            tuple: [items.get(0).unwrap().to_string(),
                    items.get(1).unwrap().to_string(),
                    items.get(2).unwrap().to_string()],
        }
    }
}



fn parse_puzzle(text: &str) -> Puzzle {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in text.trim().split("\n") {
        let tokens: Vec<String> = line.split("-").map(|a| a.to_string()).collect::<Vec<_>>();
        let (a, b) = (tokens.get(0).unwrap(), tokens.get(1).unwrap());
        graph.entry(a.to_string()).or_insert(vec![]).push(b.to_string());
        // do also in the other direction
        graph.entry(b.to_string()).or_insert(vec![]).push(a.to_string());
    }
    Puzzle {
        graph
    }
}

fn dfs_graph(graph: &HashMap<String, Vec<String>>, current: &String, visited: &mut HashSet<String>, depth: usize, max_depth: usize, max_connected: usize) {
    if visited.contains(current) {
        return;
    }
    if depth == max_depth {
        return;
    }
    if visited.len() == max_connected {
        return;
    }
    if let Some(connected) = graph.get(current) {
        visited.insert(current.clone());
        for item in connected {
            dfs_graph(graph, item, visited, depth+1, max_depth, max_connected);
        }
    }
}


fn dfs_graph_triangles(graph: &HashMap<String, Vec<String>>, current: &String, visited: &mut HashSet<String>, triangles: &mut HashSet<Triplets>) {
    //if visited.contains(current) {
    //    return;
    //}
    if let Some(connected) = graph.get(current) {
        visited.insert(current.clone());
        for i in 0..connected.len() {
            for j in i+1..connected.len() {
                let neighbour = connected.get(j).unwrap();
                let other_neighbour = connected.get(i).unwrap();
                if graph.get(other_neighbour).unwrap().contains(neighbour) {
                    triangles.insert(Triplets::new(&[current, neighbour, other_neighbour]));
                }
            }
        }
        for item in connected {
            if !visited.contains(item) {
                dfs_graph_triangles(graph, item, visited, triangles);
            }
        }
    }
}

fn get_connected_components(graph: &HashMap<String, Vec<String>>) -> HashSet<Triplets> {
    let mut triangles = HashSet::new();
    let mut visited: HashSet<String> = HashSet::new();
    for (k, _v) in graph {
        if visited.contains(k) {
            continue;
        }
        //dfs_graph(graph, k, &mut visited, 0, max_connected, max_connected);
        dfs_graph_triangles(graph, k, &mut visited, &mut triangles);
    }
    //println!("Connected components: {:?}", triangles);
    triangles
}

fn solve_pt1(pb: &Puzzle) -> usize {
    let connected_ones = get_connected_components(&pb.graph);
    //println!("Connected: {:?}", connected_ones);
    connected_ones.into_iter()
        //.filter(|set| set.len() == 3)
        .filter(|triplet| {
            for node in &triplet.tuple {
                if node.starts_with("t") {
                    return true;
                }
            }
            false
        })
        .count()
}

pub fn day23() {
    let text = fs::read_to_string("inputs/day23.txt").unwrap();
    let pb = parse_puzzle(&text);
    let soln = solve_pt1(&pb);
    println!("Solution to day 23 part 1: {}", soln);
}

#[cfg(test)]
mod tests {

    use super::{solve_pt1, parse_puzzle};

    #[test]
    fn test_sample() {
        let sample = r"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
        let pb = parse_puzzle(&sample);
        println!("{:?}", pb);
        assert_eq!(solve_pt1(&pb), 7);
    }
}
