use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

fn load_graph_vector(path_dynamic: &str) -> Vec<String> {
    if let Ok(current_dir) = env::current_dir() {
        let mut file = File::open(current_dir.join(path_dynamic));
        let mut buffer = Vec::new();
        file.expect("Load the file")
            .read_to_end(&mut buffer)
            .expect("Try to read the file");
        let contents = String::from_utf8_lossy(&buffer);
        let mut edges: Vec<String> = contents.trim().split("\n").map(|s| s.to_string()).collect();
        edges.remove(0);
        edges
    } else {
        panic!("No such file or directory");
    }
}

struct Graph {
    graph: HashMap<i32, HashSet<i32>>,
    graph_matrix: Vec<Vec<i32>>,
}

impl Graph {
    fn new(data: Vec<String>) -> Graph {
        let clone_data = data.clone();
        Graph {
            graph: Self::graph(data),
            graph_matrix: Self::graph_matrix(clone_data),
        }
    }

    fn graph(list_set: Vec<String>) -> HashMap<i32, HashSet<i32>> {
        let mut graph = HashMap::new();
        for edge in list_set {
            let parse_edge: Vec<i32> = edge
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("Fail to parse edge"))
                .collect();
            if parse_edge.len() == 1 {
                continue;
            }
            graph
                .entry(parse_edge[1])
                .or_insert_with(HashSet::new)
                .insert(parse_edge[0]);

            graph
                .entry(parse_edge[0])
                .or_insert_with(HashSet::new)
                .insert(parse_edge[1]);
        }
        graph
    }

    fn graph_matrix(list_set: Vec<String>) -> Vec<Vec<i32>> {
        let size = 5;
        let mut arr = vec![vec![0; size]; size];
        for parse_edge in list_set {
            let edge: Vec<i32> = parse_edge
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("Falha ao analisar aresta"))
                .collect();
            if edge[0] == 1 {
                continue;
            };
            arr[edge[0] as usize - 1][edge[1] as usize - 1] = 1;
        }
        arr
    }

    fn dfs(&mut self, start_node: i32) {
        let mut visited = HashSet::new();
        let mut queue = vec![];

        visited.insert(start_node);
        queue.push(start_node);
        while !queue.is_empty() {
            let node: Option<i32> = queue.pop();
            if let Some(value) = node {
                print!("{:?} ", value);
                if let Some(neighbor) = self.graph.get(&value) {
                    for &no in neighbor.iter() {
                        if visited.get(&no).is_none() {
                            visited.insert(no);
                            queue.push(no);
                        }
                    }
                }
            }
        }
    }

    fn bfs(&mut self, start_node: i32) {
        let mut visited = HashSet::new();
        let mut queue = vec![];

        visited.insert(start_node);
        queue.push(start_node);
        while !queue.is_empty() {
            let node: i32 = queue.remove(0);
            print!("{:?} ", node);
            if let Some(neighbor) = self.graph.get(&node) {
                for &no in neighbor.iter() {
                    if visited.get(&no).is_none() {
                        visited.insert(no);
                        queue.push(no);
                    }
                }
            }
        }
    }

    fn eh_conexo(graph: Vec<Vec<i32>>) {
        let mut conexo: HashMap<usize, i32> = HashMap::new();
        for i in 0..graph.len() {
            for j in 0..graph[0].len() {
                if graph[i][j] == 1 {
                    conexo.insert(i, 1);
                    conexo.insert(j, 1);
                }
            }
        }
        if conexo.keys().len() == graph.len() {
            println!("É conexo");
        }
    }
}

fn graph_information(graph: HashMap<i32, HashSet<i32>>) {
    let mut number_of_edges = graph.keys().len();
    let mut number_of_vertices = 0;
    if let Ok(current_dir) = env::current_dir() {
        let mut file = File::create(current_dir.join("src/arquivo.txt")).expect("Create the file");
        let mut clone_graph = graph.clone();
        for edge in clone_graph.keys() {
            number_of_vertices += clone_graph.get(&edge).iter().len();
            if let Some(hash_set) = clone_graph.get(&edge) {
                for value in hash_set.iter() {
                    continue;
                    // clone_graph.get_mut(&edge).expect("Try").remove(&value);
                }
            }
        }
        file.write_all(number_of_edges.to_string().as_bytes())
            .expect("Panic Write edges");
        file.write_all(b"\n").expect("Panic Write edges");
        file.write_all(number_of_vertices.to_string().as_bytes())
            .expect("Panic Write vertices");
        file.write_all(b"\n").expect("Panic write vertices");
        for edge in graph.keys() {
            let len_vertices = graph.get(edge).iter().len();
            let line = format!("{:?} {:?}", edge, len_vertices);
            number_of_vertices += len_vertices;
            file.write_all(line.as_bytes())
                .expect("TODO: Panic on try to write in file");
            file.write_all(b"\n")
                .expect("TODO Panic on try to write in file");
        }
    }
}

fn main() {
    let mut edges = load_graph_vector("src/exemplo.txt");
    let mut graph = Graph::new(edges);
    println!("{:?}", graph.graph);
    println!("{:?}", graph.graph_matrix);
    // let mut graph = graph(edges.clone());

    // bfs(1, graph.clone());
    // println!();
    // dfs(1, graph.clone())
    // graph_information(graph);
}
