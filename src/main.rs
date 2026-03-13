use hmats_rs::*;

fn main() {

    use std::time::Instant;
    let now = Instant::now();


    const D: usize=3; // keeping this around for ease of testing but will kill off like with constructor for all top level functions
    let x = [0.0, 0.0];
    let y = [4.0, 0.0];
    let val = Laplace.eval(&x, &y);
    println!("Laplace Greens function = {:?}", val.re); // can chuck .re on it when using laplace 

    // trying out the nodes 
    let mut testpoints: Vec<[f64; 3]> = Vec::new();
    testpoints.push([0.0, 0.0, 2.0]);
    testpoints.push([0.4, 0.2, 0.3]);
    testpoints.push([0.5, 0.5, 0.3]);
    testpoints.push([0.0, 1.0, 0.]);

    let nodetest = Nodes::new(testpoints);
    println!("ith node value = {:?}", nodetest.points[2]);

    // ----------- EXAMPLE WORKFLOW - with fixed top level function implementation

    // importing points as vector 
    let ecn = cardioid_nodes(10);
    // creating dynamic nodes as top level type to interact with top level functions
    let examplenodes = DynamicNodes::D2(Nodes::new(ecn)); // this should be its own function in nodes 
    // what kind of kernel do we fancy 
    let kernelfunc = Helmholtz{wavenumber:3.0};

    fullresconstructor(&examplenodes, &kernelfunc);

    // ----------------------------------

    
    
    //constructor(&card_nodes, Laplace);
    // let idx = [0,1,3];
    //let bboxtest: BBox<D> = nodetest.bbox_from_indices(&idx);


    //println!("min values of the bounding box = {:?}", bboxtest.min);
    //println!("centre of the bounding box = {:?}", bboxtest.centre());


    let testclustertree: ClusterTree<D> = ClusterTree::build_tree(&nodetest, 1);
    let _testblocktree: BlockTree = BlockTree::build_tree(&testclustertree, &testclustertree, 0.4);

    testclustertree.print();


    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
