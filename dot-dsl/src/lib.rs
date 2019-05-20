pub mod graph {
    pub mod graph_items{
        pub mod edge {
            #[derive(Debug, Clone)]
            pub struct Edge {
                a: &'static str,
                b: &'static str,
            }

            impl Edge {
                pub fn new(a: &'static str, b:&'static str) -> Self {
                    Edge {
                        a,b
                    }
                }
            }

            impl std::cmp::PartialEq for Edge {
                fn eq(&self, other: &Edge) -> bool {
                    self.a == other.a && self.b == other.b
                }
            }
        }

        pub mod node {
            #[derive(Debug, Clone)]
            pub struct Node {
                name: String,
                attrs: Vec<(&'static str, &'static str)>
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name:name.to_string(),
                        attrs:Vec::new(),
                    }
                }

                pub fn with_attrs(&mut self, attrs:&[(&'static str, &'static str)]) -> Self {
                    self.attrs = attrs.to_vec();
                    self.clone()
                }
            }

            impl std::cmp::PartialEq for Node {
                fn eq(&self, other: &Node) -> bool {
                    self.name == other.name
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct Graph{
        pub edges:Vec<graph_items::edge::Edge>,
        pub nodes:Vec<graph_items::node::Node>,
        pub attrs:Vec<(&'static str, &'static str)>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph{
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: Vec::new(),
            }
        }

        pub fn with_nodes(&mut self, nodes: &Vec<graph_items::node::Node>) -> &Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(&mut self, edges: &Vec<graph_items::edge::Edge>) -> &Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(&mut self, attrs:&[(&'static str, &'static str)]) -> Self {
            self.attrs = attrs.to_vec();
            self.clone()
        }
    }

    impl std::cmp::PartialEq for Graph{
        fn eq(&self, other: &Graph) -> bool {
            self.nodes == other.nodes && self.edges == other.edges
        }
    }
}
