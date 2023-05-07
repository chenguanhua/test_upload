use std::collections::VecDeque;

type Vertex = usize;
type ListofEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize,
    outedges: AdjacencyLists,
}

fn reverse_edges(list: &ListofEdges) -> ListofEdges {
    let mut new_list = vec![];
    for (u, v) in list {
        new_list.push((*v, *u));
    }
    new_list
}

impl Graph {
    fn add_directed_edges(&mut self, edges: &ListofEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    fn create_directed(n: usize, edges: &ListofEdges) -> Graph {
        let mut g = Graph {n, outedges: vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }

    fn create_undirected(n: usize, edges: &ListofEdges) -> Graph {
        let mut g = Self::create_directed(n, edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g
    }
}

fn main() {
    
    let n: usize = 10;
    let mut edges: ListofEdges = vec![(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)];
    edges.sort();
    // println!("{:?}", edges);
    let graph = Graph::create_undirected(n, &edges);
    // for (i, l) in graph.outedges.iter().enumerate() {
    //     println!("{} {:?}", i, *l);
    // }

    let start: Vertex = 2;

    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    
    distance[start] = Some(0);
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);

    println!("{:?}", queue);
    while let Some(v) = queue.pop_front() {
        println!("top {:?}",queue);
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] {
                distance[*u] = Some(distance[v].unwrap()+1);
                queue.push_back(*u);
                println!("In {:?}",queue);
            }
        }
    }

    print!("vertex:distance");
    for v in 0..graph.n {
        print!("   {}:{}",v,distance[v].unwrap());
    }
    println!();

}
