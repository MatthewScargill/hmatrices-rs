use num_complex::Complex64;

use crate::kernel::Kernel;
use crate::cluster::ClusterTree;
use crate::block::BlockTree;


// turning BlockTree into that sweet sweet Hmatrix 
// also useful methods etc  


// aptly named storage method for blocks
pub enum BlockStorage {
    Dense(DenseBlock),
    LowRank(LowRankBlock),
} // dense and lowrank for sorting during Hmatrix construction

// high resolution block
pub struct DenseBlock {
    pub rows: Vec<usize>, // indices into target Nodes 
    pub cols: Vec<usize>,  // indices into source Nodes
    pub data: Vec<Complex64>, // going to store data in row major order ie. A00 -> A0n, A10 -> A1n, etc 
    // Aij = data[i * len[cols] + j]
}

// approximation using A = UV^T, where U and V are essentially basis vectors since you can approximate the full Aij as basically linearly dependent 
// ACA algorithm needed in eval
pub struct LowRankBlock {
    pub rows: Vec<usize>, // indices into target Nodes 
    pub cols: Vec<usize>, // indices into source Nodes 

    // keep rank < min(rows, columns) == number of columns in U and V, how much resolution do you want to keep?
    pub rank: usize,

    pub u: Vec<Complex64>, // len(rows) x rank matrix
    pub v: Vec<Complex64>, // len(col) x rank matrix 
    // row major as above 
    
}


// wamt hmatrix to look something like this 
pub struct HMatrix<const D: usize, K: Kernel<D>> {

    // blocktree backbone
    pub block_tree: BlockTree,
    // kernel we want to use 
    pub kernel: K,

    // distance for near/far taken into account in Blocktree
    // nodes and stuff will be important but don't need to be stored here

    // dimensions of the matrix?
    pub n_rows: usize,
    pub n_cols: usize,

    // ofc need to figure out how to store actual blocks 
    
}