use distances::vectors::euclidean;

// Ingest enum, match to dimension and run methods with D: const
pub enum DynamicNodes {
    D2(Nodes<2>),
    D3(Nodes<3>),
}

// Nodes with dimension D defined at runtime
pub struct Nodes<const D: usize> {
    pub points: Vec<[f64; D]>,
    pub len: usize,

    // pub weights: Vec<[f64; 1]>,
    // pub normals: Vec<[f64; D]>, 
    // potentially, for normal derivative Helmholtz formulation
}

impl<const D: usize> Nodes<D> {
    
    // create Nodes from Vec<[f64; D]> 
    pub fn new(points: Vec<[f64; D]>) -> Self {
        assert!(!points.is_empty());
        let len: usize = points.len();
        Self { points, len} 
    }

    // main idea: avoid rewrites of Nodes data by reading off from indices
    pub fn bbox_from_indices(&self, indices: &[usize]) -> BBox<D> {

        assert!(!indices.is_empty()); 

        let mut min: [f64; D] = self.points[indices[0]];
        let mut max: [f64; D] = self.points[indices[0]];

        for &i in indices.iter().skip(1) { 
            let p: [f64; D] = self.points[i];
            for d in 0..D { // finding min/max over each spatial dimension
                if p[d] < min[d] { min[d] = p[d]}
                if p[d] > max[d] { max[d] = p[d]}
            }
        }
        BBox { min, max}
    }
}

// Bounding boxes used to subdived Nodes (indices) and for distance calculations for block admissibility (see block.rs)
#[derive(Debug, Clone, Copy)] 
pub struct BBox<const D: usize> {
    pub min: [f64; D],      // (x_min, y_min, ...)
    pub max: [f64; D],      // (x_max, y_max, ...)
}

impl <const D: usize> BBox<D> {

    // for distance calculation
    pub fn centre(&self) -> Vec<f64>{ 

        let mut centre: Vec<f64> = Vec::with_capacity(D);
        let dim: f64 = D as f64;

        for d in 0..D {
            let centre_i: f64 = (self.min[d] + self.max[d])/ dim ;
            centre.push(centre_i);
        }
        centre 
    }

    //maybe this should be a &self function or taken out the impl
    pub fn bbox_distance(source_bbox: &BBox<D>, target_bbox: &BBox<D>) -> f64 {
        
        let source_centre: Vec<f64> = source_bbox.centre();
        let target_centre: Vec<f64> = target_bbox.centre();
        let distance: f64 = euclidean(&source_centre, &target_centre);

        distance
    }

    // i have in fact forgotten what this was for 
    pub fn prox_dims(&self) -> Vec<f64> {
        let mut test = Vec::new();
        test.push(2.3);
        test.push(1.2);
        test
    }
}


// don't mind this i'm working on unit tests, the plan is currently create randomized data and see if it throws errors 
// and keeps precision between full construction and hmat 

// this template can be used across all files
#[cfg(test)] // don't compile at runtime
mod node_struct_test { // all need different names for each struct
    use super::*; // calls all the file imports

    #[test] // call it a test
    fn newtest() { // tests for each function + new names
        let testpoints = vec![
            [0.0, 0.0, 2.0],
            [0.4, 0.2, 0.3],
            [0.5, 0.5, 0.3],
            [0.0, 1.0, 0.0],
        ];

        let _nodes = Nodes::new(testpoints);

        assert_eq!(4, 4); // needs to return a bool 
    }

    // add more tests here
}