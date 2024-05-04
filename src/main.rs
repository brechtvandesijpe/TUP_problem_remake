mod model;
use model::*;

mod data;
use data::*;

pub mod algorithm;
use crate::algorithm::Node;

const Q1: i32 = 4;
const Q2: i32 = 2;

const FILE_NAME: &str = "umps8";

fn main() {
    // READIN DATA
    let data = read_data(format!("resources/{}.txt", FILE_NAME).as_str()).unwrap();
    // println!("{:?}", data);

    let model = Model::new(&data);
    
    // CREATE INITIAL NODE
    let initial = model.get_round_ints(1);

    let mut upperbound: i128 = std::i128::MAX;
    let mut best_solution: Option<Node> = None;

    let source = Node::new(
        None,
        initial,
        &data.dist,
    );
    
    // ADD SOURCE NODE TO STACK
    let mut nodes: Vec<Node> = Vec::new();
    nodes.push(source);


    // START BRANCH AND BOUND
    while nodes.len() > 0 {
        // POP NEW STATE FROM STACK
        let current_state = nodes.pop().unwrap();
        
        // EVALUATE
        // let current_state = current_state.evaluate();
        let val = current_state.score;
        if val < upperbound {
            if (current_state.round_index as usize) < data.opponents.len() {
                // ADD ALL FEASIBLE CHILDREN TO EXPLORE
                let children = current_state.generate_children(Q1, Q2, model.get_round_ints(current_state.round_index + 1), upperbound, model.num_rounds);

                // println!("round_index = {}, len children = {}, len stack = {}", current_state.round_index, children.len(), nodes.len());
                if children.len() > 0 {
                    for child in children {
                        let new_node = Node::new(
                            Some(Box::new(current_state.clone())),
                            child.clone(),
                            &data.dist,
                        );
                        nodes.push(new_node);
                    }
                }
            } else {
                upperbound = val;
                best_solution = Some(current_state.clone());
                println!("upperbound = {:?}", upperbound);
                println!(r#"New best solution
{:?}
"#, current_state);
            }
            
            // println!("best_solution = {:?}", best_solution.export_string())
        }
    }
    
    if let Some(best_solution) = best_solution {
        println!("{:?}", best_solution);
        // best_solution.clone().evaluate();
        println!("EVAL = {}", best_solution.score);
        println!("FEASIBLE? = {}", best_solution.clone().check_global(0));
        best_solution.export(FILE_NAME)
    }
}