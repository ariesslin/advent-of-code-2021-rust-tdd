use common::{lines_from_file, Stack};
use std::{collections::HashMap, error::Error, path::Path};

struct AadjacencyMatrix {
    nodes: Vec<String>,
    edges: HashMap<String, Vec<usize>>,
}

fn get_position_in_vector(nodes: &[String], item: &str) -> usize {
    let position = nodes.iter().position(|r| r == item).unwrap();
    position
}

fn read_adjacency_matrix_from_file(
    filename: impl AsRef<Path>,
) -> Result<AadjacencyMatrix, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut nodes = Vec::new();
    let mut edges = HashMap::new();

    for line in lines_from_file {
        let mut splits = line.trim().split('-');
        let node1 = splits.next().unwrap().to_string();
        let node2 = splits.next().unwrap().to_string();
        if !nodes.contains(&(node1.to_string())) {
            nodes.push(node1.to_string());
            edges.insert(node1.to_string(), vec![]);
        }

        if !nodes.contains(&(node2.to_string())) {
            nodes.push(node2.to_string());
            edges.insert(node2.to_string(), vec![]);
        }

        let pos1 = get_position_in_vector(&nodes, &node1);
        let pos2 = get_position_in_vector(&nodes, &node2);
        edges.get_mut(&node1).unwrap().push(pos2);
        edges.get_mut(&node2).unwrap().push(pos1);
    }

    //println!("nodes: {:?}", nodes);
    //println!("edges: {:?}", edges);
    Ok(AadjacencyMatrix { nodes, edges })
}

fn get_all_paths_from_start_to_end(cave_graph: AadjacencyMatrix) -> Vec<Vec<String>> {
    let mut main_stack: Stack<String> = Stack::new();
    let mut side_stack: Stack<Vec<usize>> = Stack::new();

    let mut paths = vec![];

    let node = "start".to_string();
    let connected_nodes = cave_graph.edges.get(&node).unwrap().to_vec();
    main_stack.push(node);
    side_stack.push(connected_nodes);

    while !main_stack.is_empty() {
        //println!("main stack is {:?}", main_stack);

        let mut connected_nodes = side_stack.pop().unwrap();
        if connected_nodes.is_empty() {
            main_stack.pop();
            continue;
        } else {
            let next_pos = connected_nodes.pop().unwrap();
            side_stack.push(connected_nodes.clone());

            let node = cave_graph.nodes[next_pos].to_string();
            //println!("new node is {:?}", node);
            if !main_stack.stack.contains(&node)
                || (main_stack.stack.contains(&node) && !node.chars().all(char::is_lowercase))
            {
                let connected_nodes = cave_graph.edges.get(&node).unwrap().to_vec();
                main_stack.push(node);
                side_stack.push(connected_nodes);
            } else {
                continue;
            }
        }

        if main_stack.peek().unwrap() == "end" {
            paths.push(main_stack.stack.clone());

            main_stack.pop();
            side_stack.pop();
        }
    }

    paths
}

fn main() {
    let filename = "day12_input.txt";
    let cave_graph = read_adjacency_matrix_from_file(filename).unwrap();

    let paths = get_all_paths_from_start_to_end(cave_graph);

    println!("paths number is {:?}", paths.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_adjacency_matrix_from_file_given_the_filename() {
        let filename = "day12_test.txt";
        let cave_graph = read_adjacency_matrix_from_file(filename).unwrap();

        assert_eq!(cave_graph.nodes.len(), 6);

        let filename = "day12_test_2.txt";
        let cave_graph = read_adjacency_matrix_from_file(filename).unwrap();

        assert_eq!(cave_graph.nodes.len(), 7);
    }

    #[test]
    fn should_get_all_paths_from_start_to_end_given_adjacency_matrix() {
        let filename = "day12_test.txt";
        let cave_graph = read_adjacency_matrix_from_file(filename).unwrap();

        let paths = get_all_paths_from_start_to_end(cave_graph);

        //println!("paths are {:?}", paths);
        assert_eq!(paths.len(), 10);

        let filename = "day12_test_2.txt";
        let cave_graph = read_adjacency_matrix_from_file(filename).unwrap();

        let paths = get_all_paths_from_start_to_end(cave_graph);

        //println!("paths are {:?}", paths);
        assert_eq!(paths.len(), 19);

        let filename = "day12_test_3.txt";
        let cave_graph = read_adjacency_matrix_from_file(filename).unwrap();

        let paths = get_all_paths_from_start_to_end(cave_graph);

        //println!("paths are {:?}", paths);
        assert_eq!(paths.len(), 226);
    }
}
