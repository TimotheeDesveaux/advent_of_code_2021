use std::collections::{HashMap, HashSet};

fn dfs<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    src: &'a str,
) -> u32 {
    if src == "end" {
        return 1;
    } else if visited.contains(src) {
        return 0;
    }

    if src.chars().all(char::is_lowercase) {
        visited.insert(src);
    }

    let mut count = 0;
    for dest in &graph[src] {
        count += dfs(graph, visited, dest);
    }

    visited.remove(src);

    count
}

fn part_one(input: &str) -> u32 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (n1, n2) = line.split_once('-').unwrap();
        graph.entry(n1).or_insert(Vec::new()).push(n2);
        graph.entry(n2).or_insert(Vec::new()).push(n1);
    }

    dfs(&graph, &mut HashSet::new(), "start")
}

fn main() {
    let input = include_str!("input.txt");

    let number_of_paths = part_one(input);

    println!("There are {} paths through the cave sytem", number_of_paths);
}
