use petgraph::Graph;

fn main() {
    let mut graph = Graph::<i32,i32>::new();
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    let three = graph.add_node(3);
    let four = graph.add_node(4);

    graph.extend_with_edges(&[(one, two), (two, one), 
    (one,three), (three, one), (two, four), (four, two),
    (three, four), (four, three)]);
    
    dbg!(graph);
}