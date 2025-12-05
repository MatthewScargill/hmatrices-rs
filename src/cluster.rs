use crate::node::{Nodes, BBox};
use crate::kernel::{Kernel, Laplace, Helmholtz}; // may only need kernel and keep it flexible

// may need a refactor of the Nodes struct because too many "nodes" about

// individual node of the tree containing indices of a fraction of the total nodes
// relationships between nodes to be kept with each node
pub struct ClusterNode<const D: usize> {
    pub bbox: BBox<D>, // bounding box of this node
    pub indices: Vec<usize>,  // indices into Nodes.points
    pub children: Option<[usize; 2]>, // indices into ClusterTree.nodes
    pub level: u32, // how many splits have we had to this box
    pub center: [f64; D] // centre of the BBox for distance checking between boxes
}

impl<const D: usize> ClusterNode<D> {
    // simple build function
}



// just a collection of cluster nodes, read of relationships from individual nodes
pub struct ClusterTree<const D: usize> {
    pub nodes: Vec<ClusterNode<D>>,
}

impl<const D: usize> ClusterTree<D> {
    // tree builder which will recursively split until each "tree tip" reaches a certain amount of points 
    // lump all the cluster nodes together with logical indices 

}