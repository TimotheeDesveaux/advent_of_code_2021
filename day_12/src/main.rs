use std::collections::HashMap;

fn dfs<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    path: &mut Vec<&'a str>,
    src: &'a str,
    mut revisit: bool,
) -> u32 {
    if src == "end" {
        return 1;
    } else if src.chars().all(char::is_lowercase) && path.contains(&src) {
        if !revisit || src == "start" {
            return 0;
        }

        revisit = false;
    }

    path.push(src);
    let mut count = 0;
    for dest in &graph[src] {
        count += dfs(graph, path, dest, revisit);
    }
    path.pop();

    count
}

fn part_one(input: &str) -> u32 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (n1, n2) = line.split_once('-').unwrap();
        graph.entry(n1).or_insert(Vec::new()).push(n2);
        graph.entry(n2).or_insert(Vec::new()).push(n1);
    }

    dfs(&graph, &mut Vec::new(), "start", false)
}

fn part_two(input: &str) -> u32 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (n1, n2) = line.split_once('-').unwrap();
        graph.entry(n1).or_insert(Vec::new()).push(n2);
        graph.entry(n2).or_insert(Vec::new()).push(n1);
    }

    dfs(&graph, &mut Vec::new(), "start", true)
}

fn main() {
    let input = include_str!("input.txt");

    let number_of_paths1 = part_one(input);
    let number_of_paths2 = part_two(input);

    println!(
        "There are {} paths through the cave sytem without revisiting smaller caves",
        number_of_paths1
    );
    println!(
        "There are {} paths through the cave sytem while revisiting at most one smaller cave",
        number_of_paths2
    );
}
