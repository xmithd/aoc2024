use std::fs;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(Debug)]
struct Puzzle {
    graph: HashMap<String, Vec<String>>
}

fn parse_puzzle(text: &str) -> Puzzle {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in text.trim().split("\n") {
        let tokens: Vec<String> = line.split("-").map(|a| a.to_string()).collect::<Vec<_>>();
        let (a, b) = (tokens.get(0).unwrap(), tokens.get(1).unwrap());
        graph.entry(a.to_string()).or_insert(vec![]).push(b.to_string());
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

fn dfs_graph_interconnected(graph: &HashMap<String, Vec<String>>, current: &String, parent: Option<&String>, visited: &mut HashSet<String>, path: &Vec<String>) {
    if path.len() > 0 && path.first().unwrap() == current {
        // found loop!
        return;
    }
    if visited.contains(current) {
        return;
    }
    /*if path.len() == max_connected {
        return;
    }*/
    if let Some(connected) = graph.get(current) {
        visited.insert(current.clone());
        for item in connected {
            let mut new_path = path.clone();
            new_path.push(current.clone());
            dfs_graph_interconnected(graph, item, Some(current), visited, &new_path);
        }
    }
}

fn get_connected_components(graph: &HashMap<String, Vec<String>>, max_connected: usize) -> Vec<HashSet<String>> {
    let mut ret = vec![];
    let mut global_visited: HashSet<String> = HashSet::new();
    for (k, _v) in graph {
        if global_visited.contains(k) {
            continue;
        }
        global_visited.insert(k.clone());
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        //dfs_graph(graph, k, &mut visited, 0, max_connected, max_connected);
        dfs_graph_interconnected(graph, k, None, &mut visited, &path);
        for node in &visited {
            global_visited.insert(node.clone());
        }
        ret.push(visited.clone());
    }
    ret
}

fn solve_pt1(pb: &Puzzle) -> usize {
    let connected_ones = get_connected_components(&pb.graph, 3);
    println!("Connected: {:?}", connected_ones);
    connected_ones.into_iter()
        .filter(|set| set.len() == 3)
        .filter(|set| {
            for node in set {
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
