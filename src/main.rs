mod graph;

fn main() {
    let mut adj_list = graph::AdjacencyList::new();
    let a = adj_list.add_vertex();
    let b = adj_list.add_vertex();
    adj_list.add_edge(a, b);
    assert!(adj_list.adjacent(a, b));
}
