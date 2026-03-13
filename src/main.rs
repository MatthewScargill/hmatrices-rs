use hmats_rs::{nodes::DynamicNodes, *};

fn main() {
    const D: usize=2; //dimension needs to be set early on in computation as a const for openess -- see kenel definition
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


    let cn = cardioid_nodes(5);
    // let card_nodes = DynamicNodes::D2(Nodes::new(cn));
    let card_nodes = Nodes::new(cn);
    
    fn constructor(nodes: &Nodes<D>, greensfunction: impl Kernel<D>) { // accepts anything with Kernel trait
        let len: usize = nodes.len;
        for i in 0..len as usize {
            for j in 0..len as usize {
                let coord1 = nodes.points[i];
                let coord2 = nodes.points[j];
                let laptest = greensfunction.eval(&coord1, &coord2);
                println!("{}th row, {}th column, cell value = {:?}", i, j, laptest);
            }
        }
    }
    



    constructor(&card_nodes, Helmholtz{wavenumber: 3.0});
    println!("-------------------");
    //constructor(&card_nodes, Laplace);
    // let idx = [0,1,3];
    //let bboxtest: BBox<D> = nodetest.bbox_from_indices(&idx);


    //println!("min values of the bounding box = {:?}", bboxtest.min);
    //println!("centre of the bounding box = {:?}", bboxtest.centre());


    // ----------- EXAMPLE WORKFLOW

    // importing points as vector 
    let ecn = cardioid_nodes(5);
    // creating dynamic nodes as top level type to interact with top level functions
    let examplenodes = DynamicNodes::D2(Nodes::new(ecn)); // this should be its own function in nodes 
    // what kind of kernel do we fancy 
    let kernelfunc = Helmholtz{wavenumber:3.0};

    fn toplevelconstructor_impl<const D: usize, K>(nodes: &Nodes<D>, greensfunction: &K)
    where
        K: Kernel<D>,
    {
    let len = nodes.len;

    for i in 0..len {
        for j in 0..len {
            let coord1 = nodes.points[i];
            let coord2 = nodes.points[j];
            let val = greensfunction.eval(&coord1, &coord2);

            println!(
                "{}th row, {}th column, cell value = {:?}",
                i, j, val
                );
        }
    }
    }

    fn toplevelconstructor<K>(nodes: &DynamicNodes, greensfunction: &K)
    where
        K: Kernel<2> + Kernel<3>,
    {
        match nodes {
            DynamicNodes::D2(nodes) => toplevelconstructor_impl::<2, K>(nodes, greensfunction),
            DynamicNodes::D3(nodes) => toplevelconstructor_impl::<3, K>(nodes, greensfunction),
        }
    }

    toplevelconstructor(&examplenodes, &kernelfunc);

    // ----------------------------------


    let testclustertree: ClusterTree<D> = ClusterTree::build_tree(&card_nodes, 1);
    let _testblocktree: BlockTree = BlockTree::build_tree(&testclustertree, &testclustertree, 0.4);

    testclustertree.print();

}
