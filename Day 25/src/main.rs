use std::{fs, path::Path, collections::HashMap};

use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut nodes = HashMap::new();
    let mut graph : UnGraph<(), ()> = UnGraph::new_undirected();

    data.split("\n").for_each(|r| {
        let row = r.split(": ").collect::<Vec<_>>();
        let src ;
        if !nodes.contains_key(row[0]) {
            src = graph.add_node(());
            nodes.insert(row[0], src);
        } else {
            src = *nodes.get(row[0]).unwrap();
        }
        row[1].split(" ").for_each(|o| {
            let dest;
            if !nodes.contains_key(o) {
                dest = graph.add_node(());
                nodes.insert(o, dest);
            } else {
                dest = *nodes.get(o).unwrap();
            }
            graph.extend_with_edges(&[(src, dest)]);
        })
    });

    let min_cut_res: Result<Option<(usize, Vec<_>)>> =
    stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (_min_cut, partition) = min_cut_res.unwrap().unwrap();

    let part1 = (graph.node_count() - partition.len()) * partition.len();
    let part2 = 0;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}