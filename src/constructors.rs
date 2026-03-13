use crate::kernels::Kernel;
use crate::nodes::{DynamicNodes,Nodes};


// ----------- FULL RES CONSTRUCTOR ----------------

// impl version of the function where we don't have runtime variables 
fn fullresconstructorq_impl<const D: usize, K>(nodes: &Nodes<D>, greensfunction: &K) where K: Kernel<D> {

    let len = nodes.len;

    for i in 0..len {
        for j in 0..len {
            let coord1 = nodes.points[i];
            let coord2 = nodes.points[j];
            let val = greensfunction.eval(&coord1, &coord2);

            println!("{}th row, {}th column, cell value = {:?}", i, j, val);
        }
    }
}

// Dynamic version with runtime D 
pub fn fullresconstructor<K>(nodes: &DynamicNodes, greensfunction: &K) where K: Kernel<2> + Kernel<3> {

    match nodes {
        DynamicNodes::D2(nodes) => fullresconstructorq_impl::<2, K>(nodes, greensfunction),
        DynamicNodes::D3(nodes) => fullresconstructorq_impl::<3, K>(nodes, greensfunction),
    }
}