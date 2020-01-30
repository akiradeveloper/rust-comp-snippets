#[snippet = "Graph"]
pub struct Graph {
    out_e: Vec<Vec<usize>>,
    in_e: Vec<Vec<usize>>,
}
#[snippet = "Graph"]
impl Graph {
    pub fn new(n: usize) -> Graph {

    }
    pub fn add_edge(&mut self, u: usize, v: usize) {

    }
    pub fn remove_edge(&mut self, u: usize, v: usize) {

    }
    pub fn in_edges(&self, u: usize) -> Vec<usize> {

    }
    pub fn out_edges(&self, u: usize) -> Vec<usize> {

    }
}