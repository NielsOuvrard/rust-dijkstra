struct Node {
    label: String,
    dist: u32,         // weight from start point
    path: Vec<String>, // path from start point
}

struct Edge {
    src: String,
    dest: String,
    weight: u32,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

// fn get_closest_node(nodes: &[Node]) -> Option<&Node> {
//     if nodes.is_empty() {
//         return None;
//     }
//     let mut closest = nodes.first().unwrap();
//     for node in nodes {
//         if node.dist < closest.dist {
//             closest = node;
//         }
//     }
//     Some(closest)
// }

fn get_node_with_label(nodes: &[Node], label: String) -> Option<&Node> {
    for node in nodes {
        if node.label == label {
            return Some(node);
        }
    }
    None
}

fn get_closest_node_name(nodes: &[Node], available: Vec<String>) -> Option<String> {
    let first_node_name = available.first()?;
    let mut closest = get_node_with_label(nodes, first_node_name.clone())?;
    for name in available {
        let node = get_node_with_label(nodes, name.clone())?;
        if node.dist < closest.dist {
            closest = node;
        }
    }
    Some(closest.label.clone())
}

fn get_neighbors_of_node(graph: &Graph, name_node: String) -> Option<Vec<&Node>> {
    let mut neighbors: Vec<&Node> = Vec::new();
    for edge in &graph.edges {
        if edge.src == name_node {
            for node in &graph.nodes {
                if node.label == edge.dest {
                    neighbors.push(node);
                }
            }
        }
    }
    if neighbors.is_empty() {
        None
    } else {
        Some(neighbors)
    }
}

fn dijkstra(graph: &mut Graph, node_name: String) -> Option<()> {
    let mut unvisited: Vec<String> = Vec::new();

    for node in graph.nodes.iter_mut() {
        if node.label == node_name {
            node.dist = 0;
        } else {
            node.dist = u32::MAX;
        }
        unvisited.push(node.label.clone());
    }

    while !unvisited.is_empty() {
        let current_node = get_closest_node_name(&graph.nodes, unvisited.clone())?;

        println!("\x1b[0;31mCurrent node: {}\x1b[0m", current_node);
        println!("unvisited:");
        for elem in unvisited.clone() {
            println!("\t- {}", elem);
        }
        println!("Neighbor of {} are:", current_node);
        let neighbors = get_neighbors_of_node(graph, current_node.clone())?;
        for neighbor in neighbors {
            println!("\t- {} dist {}", neighbor.label, neighbor.dist);
            // distance = distances[current_node]['dist'] + neighbor.weight
        }
        unvisited.retain(|label| label != &current_node);
    }
    Some(())
}

fn main() {
    // Define nodes
    let node_a = Node {
        label: "A".to_string(),
        dist: 0,
        path: vec!["A".to_string()],
    };
    let node_b = Node {
        label: "B".to_string(),
        dist: 0,
        path: vec!["B".to_string()],
    };
    let node_c = Node {
        label: "C".to_string(),
        dist: 0,
        path: vec!["C".to_string()],
    };
    let node_d = Node {
        label: "D".to_string(),
        dist: 0,
        path: vec!["D".to_string()],
    };
    let node_e = Node {
        label: "E".to_string(),
        dist: 0,
        path: vec!["E".to_string()],
    };
    let node_f = Node {
        label: "F".to_string(),
        dist: 0,
        path: vec!["F".to_string()],
    };

    // Define edges
    let edge_ab = Edge {
        src: "A".to_string(),
        dest: "B".to_string(),
        weight: 2,
    };
    let edge_bc = Edge {
        src: "B".to_string(),
        dest: "C".to_string(),
        weight: 1,
    };
    let edge_ce = Edge {
        src: "C".to_string(),
        dest: "E".to_string(),
        weight: 3,
    };
    let edge_eb = Edge {
        src: "E".to_string(),
        dest: "B".to_string(),
        weight: 2,
    };
    let edge_ac = Edge {
        src: "A".to_string(),
        dest: "C".to_string(),
        weight: 4,
    };
    let edge_db = Edge {
        src: "D".to_string(),
        dest: "B".to_string(),
        weight: 4,
    };
    let edge_de = Edge {
        src: "D".to_string(),
        dest: "E".to_string(),
        weight: 3,
    };
    let edge_df = Edge {
        src: "D".to_string(),
        dest: "F".to_string(),
        weight: 2,
    };
    let edge_fe = Edge {
        src: "F".to_string(),
        dest: "E".to_string(),
        weight: 2,
    };

    // Create graph
    let mut graph = Graph {
        nodes: vec![node_a, node_b, node_c, node_d, node_e, node_f],
        edges: vec![
            edge_ab, edge_bc, edge_ce, edge_eb, edge_ac, edge_db, edge_de, edge_df, edge_fe,
        ],
    };

    dijkstra(&mut graph, String::from("A"));
    // let new_nodes =

    // println!(
    //     "Graph created with {} nodes and {} edges",
    //     graph.nodes.len(),
    //     graph.edges.len()
    // );
}
