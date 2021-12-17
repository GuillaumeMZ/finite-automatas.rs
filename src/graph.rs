pub struct Graph<T> {
    transitions : Vec<T>,
    size: usize
}

fn compute_index<T>(graph: &Graph<T>, index : (usize, usize)) -> usize {
    index.0 * graph.transitions.len() + index.1
}

impl<T> core::ops::Index<(usize, usize)> for Graph<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = compute_index(self, index);
        &self.transitions[index]
    }
}

impl<T> core::ops::IndexMut<(usize, usize)> for Graph<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = compute_index(self, index);
        &mut self.transitions[index]
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            transitions: Vec::new(),
            size: 0
        }
    }

    pub fn add_node(&mut self) {
            
    }

    pub fn remove_node(&mut self) {

    }

    pub fn add_edge(&mut self) {
       
    }

    pub fn remove_edge(&mut self) {

    }
}