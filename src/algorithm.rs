
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::prelude::*;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use itertools::Itertools;
use std::io::{BufWriter, Write};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Data {
    pub num_teams: i32,
    pub dist: Vec<Vec<i128>>,
    pub opponents: Vec<Vec<i32>>,
}

pub fn read_data(file_path: &str) -> io::Result<Data> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut num_teams = 0;
    let mut dist = Vec::new();
    let mut opponents = Vec::new();

    while let Some(line) = lines.next() {
        let line = line?;
        if line.contains("nTeams") {
            num_teams = line.split("=").nth(1).unwrap().split(";").nth(0).unwrap().trim().parse().unwrap();
        }
        if line.contains("dist") {
            dist = read_array_i128(&mut lines, num_teams as usize)?;
        }
        if line.contains("opponents") {
            opponents = read_array_i32(&mut lines, 2 * num_teams as usize - 2)?;
        }
    }

    Ok(Data {
        num_teams,
        dist,
        opponents,
    })
}

fn read_array_i128(
    lines: &mut std::io::Lines<BufReader<File>>,
    rows: usize
) -> io::Result<Vec<Vec<i128>>> {
    let mut array = Vec::new();

    for _ in 0..rows {
        let line = lines.next().unwrap()?;
        let row: Vec<i128> = line.split(|c: char| c == '[' || c == ']' || c.is_whitespace())
            .filter(|part| !part.is_empty())
            .map(|part| i128::from_str(part).unwrap())
            .collect();
        array.push(row);
    }

    Ok(array)
}

fn read_array_i32(
    lines: &mut std::io::Lines<BufReader<File>>,
    rows: usize
) -> io::Result<Vec<Vec<i32>>> {
    let mut array = Vec::new();

    for _ in 0..rows {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split(|c: char| c == '[' || c == ']' || c.is_whitespace())
            .filter(|part| !part.is_empty())
            .map(|part| i32::from_str(part).unwrap())
            .collect();
        array.push(row);
    }

    Ok(array)
}

#[derive(Debug, Clone)]
pub struct Game {
    pub home_player: i32,
    pub out_player: i32,
}

impl Game {
    pub fn new(
        home_player: i32,
        out_player: i32
    ) -> Self {
        Self {
            home_player,
            out_player,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Round {
    games: Vec<Game>,
}

impl Round {
    pub fn new(
        opponents: Vec<i32>
    ) -> Self {
        let mut games = Vec::new();
        for player in 0..opponents.len() {
            if opponents[player as usize] < 0 {
                continue;
            } else {
                games.push(Game::new(player as i32 + 1, opponents[player as usize]));
            }
        }

        Self {
            games,
        }
    }
    
    pub fn to_ints(
        &self,
    ) -> Vec<(i32, i32)> {
        let mut output = Vec::new();

        for game in &self.games {
            output.push((game.home_player, game.out_player));
        }

        output
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    rounds: Vec<Round>,
    pub num_rounds: i32,
}

impl Model {
    pub fn new(
        data: &Data,
    ) -> Self {
        let num_rounds: i32 = data.opponents.len().try_into().unwrap();
        let mut rounds = Vec::new();
        for i in 0..num_rounds {
            rounds.push(Round::new(data.opponents[i as usize].clone()));
        }

        Self {
            rounds,
            num_rounds,
        }
    }

    pub fn get_round_ints(
        &self,
        round_index: i32,
    ) -> Vec<(i32, i32)> {
        self.rounds[(round_index - 1) as usize].to_ints()
    }
}

fn pretty_print(
    matrix: &Vec<Vec<i128>>,
) {
    println!("--");
    for row in matrix {
        println!("{:?}", row);
    }
}

// fn calculate_lowerbound(
//     dist: Arc<Vec<Vec<i128>>>,
//     rounds_lbs: Arc<Mutex<Vec<Vec<i128>>>>,
//     max_rounds: i32,
//     q1: i32,
//     q2: i32,
//     model: Model,
// ) {
//     for k in 1..max_rounds {
//         let r: i32 = max_rounds - 1 - k;
//         println!("LB calculation of start = {}, end = {}", r+1, r+k+1);

//         let mut source = Node::new(
//             None,
//             model.get_round_ints(r + 1).clone(),
//             &dist,
//         );

//         source = source.set_round_index(r + 1);

//         let mut upperbound: i128 = 999999;
//         let mut best_solution: Option<Node> = None;

//         let mut nodes: Vec<Node> = Vec::new();
//         nodes.push(source);

//         // START BRANCH AND BOUND
//         while nodes.len() > 0 {
//             // POP NEW STATE FROM STACK
//             let current_state = nodes.pop().unwrap();
            
//             // EVALUATE
//             let val = current_state.score;
            
//             if val < upperbound {
//                 if (current_state.round_index as usize) == (r + k + 1) as usize {
//                     upperbound = val;
//                     best_solution = Some(current_state.clone());
//                 } else {
//                     // ADD ALL FEASIBLE CHILDREN TO EXPLORE
//                     let options = model.get_round_ints(current_state.round_index + 1);
//                     let children = current_state.generate_children_lowerbound(q1, q2, options, upperbound, &rounds_lbs.lock().unwrap(), r + k + 1);
                    
//                     // CREATE AND ADD ALL CHILDREN
//                     if !children.is_empty() {
//                         for child in children {
//                             let new_node = Node::new(
//                                 Some(Box::new(current_state.clone())),
//                                 child.clone(),
//                                 &dist,
//                             );
//                             nodes.push(new_node);
//                         }
//                     }
//                 }
//             }
//         }

//         // println!("{:?}", best_solution.unwrap());

//         for r1 in (0..=r).rev() {
//             for r2 in (r + k)..max_rounds {
//                 let mut data = rounds_lbs.lock().unwrap();
//                 let val_1: i128 = data[r1 as usize][r2 as usize].borrow_mut().clone();
//                 let val_2: i128 = data[r1 as usize][r as usize].borrow().clone() + upperbound;
//                 let val_3: i128 = data[(r + k) as usize][r2 as usize].borrow_mut().clone();
//                 let best_val = std::cmp::max(val_1, std::cmp::max(val_2, val_3));
//                 *data[r1 as usize][r2 as usize].borrow_mut() = best_val;
//                 pretty_print(&data);
//             }
//         }

//     }
// }

// pub fn branch_and_bound(
//     file_name: &str,
//     q1: i32,
//     q2: i32
// ) -> Result<i128, &'static str> {
//     let data = read_data(format!("resources/{}.txt", file_name).as_str()).unwrap();
//     let model = Model::new(&data);

//     let initial = model.get_round_ints(1);

//     let mut upperbound: i128 = 999999;
//     let mut best_solution: Option<Node> = None;

//     let source = Node::new(
//         None,
//         initial.clone(),
//         &data.dist,
//     );
    
//     // ADD SOURCE NODE TO STACK
//     let mut nodes: Vec<Node> = Vec::new();
//     nodes.push(source.clone());

//     // START LWOERBOUND_THREAD
//     let lowerbound: Arc<Mutex<Vec<Vec<i128>>>> = Arc::new(Mutex::new(vec![vec![0; model.num_rounds as usize]; model.num_rounds as usize]));
//     let lowerbound_clone:Arc<Mutex<Vec<Vec<i128>>>> = Arc::clone(&lowerbound);

//     let dist_clone = Arc::new(data.dist.clone());
//     let dist_clone_lb = Arc::clone(&dist_clone);

//     let num_rounds = model.num_rounds;
//     let model_clone = model.clone();
//     let handle = thread::spawn(
//         move || {
//             calculate_lowerbound(
//                 dist_clone_lb,
//                 lowerbound_clone,
//                 num_rounds,
//                 q1,
//                 q2,
//                 model_clone,
//             )
//         }
//     );

//     handle.join().unwrap();
//     pretty_print(&lowerbound.lock().unwrap());

//     // START BRANCH AND BOUND
//     while nodes.len() > 0 {
//         // POP NEW STATE FROM STACK
//         let current_state = nodes.pop().unwrap();
        
//         // EVALUATE
//         let val = current_state.score;
//         let lb_val = lowerbound.lock().unwrap().clone();
        
//         if val < upperbound {
//             if (current_state.round_index as usize) < num_rounds as usize {
//                 // ADD ALL FEASIBLE CHILDREN TO EXPLORE
//                 let children = current_state.generate_children(q1, q2, model.get_round_ints(current_state.round_index + 1), upperbound, num_rounds, &lb_val);

//                 // CREATE AND ADD ALL CHILDREN
//                 if !children.is_empty() {
//                     for child in children {
//                         let new_node = Node::new(
//                             Some(Box::new(current_state.clone())),
//                             child.clone(),
//                             &data.dist,
//                         );
//                         nodes.push(new_node);
//                     }
//                 }
//             } else {
//                 upperbound = val;
//                 let lb: &i128 = &lowerbound.lock().unwrap()[0][(num_rounds - 1) as usize];
//                 let gap = (upperbound as f64 - *lb as f64) / upperbound as f64;
//                 // println!("lowerbound = {}, upperbound = {}, gap = {}", lb, upperbound, gap);
//                 best_solution = Some(current_state.clone());
//             }
//         }
//     }
    
//     if let Some(best_solution) = best_solution {
//         best_solution.export(file_name);
//         return Ok(best_solution.score);
//     }

//     Err("No solution found")
// }

#[derive(Clone)]
pub struct Node<'a> {
    parent: Option<Box<Node<'a>>>,
    assignment: (i32, i32),
    pub score: i128,
    pub round_index: i32,
    dist: &'a Vec<Vec<i128>>,
    visited_teams: Vec<bool>,
}

impl<'a> Node<'a> {
    pub fn new(
        parent: Option<Box<Node<'a>>>,
        assignment: (i32, i32),
        dist: &'a Vec<Vec<i128>>,
    ) -> Self {
        let mut visited_teams = vec![false; dist.len()];
        visited_teams[(assignment.0 - 1) as usize] = true;

        let mut round_index = 1;
        let mut score: i128 = 0;

        if let Some(parent) = &parent {
            round_index += parent.round_index;
            score += parent.score + dist[(parent.assignment.0 - 1) as usize][(assignment.0 - 1) as usize];
            
            for i in 0..visited_teams.len() {
                if parent.visited_teams[i] == true {
                    visited_teams[i] = true;
                }
            }
        }

        Self {
            parent,
            assignment,
            score,
            round_index,
            dist,
            visited_teams,
        }
    }
}

impl<'a> std::fmt::Debug for Node<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, r#"{:?}
{:?} {}"#, parent, self.assignment, self.score)
        } else {
            write!(f, "{:?} {:?}", self.assignment, self.score)
        }
    }
}

impl<'a> std::fmt::Display for Node<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, r#"{}
{:?}"#, parent, self.new_assignments)
        } else {
            write!(f, "{:?}", self.new_assignments)
        }
    }
}