// use crate::kernels::Kernel;
// use crate::nodes::Nodes;
// use crate::cluster::{ClusterTree, ClusterNode};
// use crate::block::{BlockTree, BlockType};

// place for main callables which just take in nodes, kernels, etc


// things we need:
// - construct hmatrix from nodes and kernel
// - return approximate matvec solution using hmatrix and RHS term (as additional thing, 0 as default)
// - ??


// do we need a different place for error bound, space allocation, and graphing stuff?

use std::f64::consts::{PI};

pub fn cardioid_nodes(n: usize) -> Vec<[f64; 2]> {
    assert!(n >= 2, "theoretical minimum");

    let mut pts = Vec::with_capacity(n);
    for i in 0..n {
        let theta: f64 = 2.0 * PI * (i as f64) / (n as f64);
        let r: f64 = 1.0 - theta.cos();
        let x: f64 = r * theta.cos();
        let y: f64 = r * theta.sin();
        pts.push([x, y]);
    }
    pts
}


// random plotting function
use plotters::prelude::*;
pub fn plot(points: &[[f64; 2]]) {
    let root = BitMapBackend::new("plot.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(-2.0..2.0, -2.0..2.0)
        .unwrap();

    chart
        .draw_series(
            points
                .iter()
                .map(|p| Circle::new((p[0], p[1]), 2, RED.filled())),
        )
        .unwrap();
}
