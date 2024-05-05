
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::prelude::*;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use itertools::Itertools;

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

fn calculate_lowerbound(
    dist: Arc<Vec<Vec<i128>>>,
    rounds_lbs: Arc<Mutex<Vec<Vec<i128>>>>,
    max_rounds: i32,
    q1: i32,
    q2: i32,
    model: Model,
) {
    let mut rounds_lbs = rounds_lbs.lock().unwrap().clone();
    for k in 1..max_rounds {
        let r: i32 = max_rounds - 1 - k;

        let mut source = Node::new(
            None,
            model.get_round_ints(r + 1).clone(),
            &dist,
        );

        source = source.set_round_index(r);

        let mut best: i128 = 0;
        let mut nodes: Vec<Node> = Vec::new();
        nodes.push(source);

        // START BRANCH AND BOUND
        while nodes.len() > 0 {
            // POP NEW STATE FROM STACK
            let current_state = nodes.pop().unwrap();
            
            // EVALUATE
            let val = current_state.score;
            if val >= best {
                if (current_state.round_index as usize) == (r + k) as usize {
                    println!("Update on {}", r + k);
                    best = val;
                } else {
                    // ADD ALL FEASIBLE CHILDREN TO EXPLORE
                    let options = model.get_round_ints(current_state.round_index + 1);
                    let children = current_state.generate_children(q1, q2, options, best, max_rounds, false);
                    
                    // CREATE AND ADD ALL CHILDREN
                    if !children.is_empty() {
                        for child in children {
                            let new_node = Node::new(
                                Some(Box::new(current_state.clone())),
                                child.clone(),
                                &dist,
                            );
                            nodes.push(new_node);
                        }
                    }
                }
            }
        }

        // println!("best = {}, k = {}, r = {}", best, k, r);
        for r1 in (0..=r).rev() {
            for r2 in (r + k)..max_rounds {
                // println!("r1 = {}", r1);
                // println!("r2 = {}", r2);
                let val_1: i128 = rounds_lbs[r1 as usize][r2 as usize].borrow_mut().clone();
                let val_2: i128 = rounds_lbs[r1 as usize][r as usize].borrow().clone() + best;
                let val_3: i128 = rounds_lbs[(r + k) as usize][r2 as usize].borrow_mut().clone();
                let best_val = std::cmp::max(val_1, std::cmp::max(val_2, val_3));
                // println!("val_1 = {}", val_1);
                // println!("val_2 = {}", val_2);
                // println!("val_3 = {}", val_3);
                // println!("best_val = {}", best_val);
                *rounds_lbs[r1 as usize][r2 as usize].borrow_mut() = best_val;
            }
        }
        // println!("r + k = {}, r = {}, k = {}", r + k, r, k);
        pretty_print(&rounds_lbs);

        // println!("{:?}", rounds_lbs);
    }
}

pub fn branch_and_bound(
    file_name: &str,
    q1: i32,
    q2: i32
) -> Result<i128, &'static str> {
    let data = read_data(format!("resources/{}.txt", file_name).as_str()).unwrap();
    let model = Model::new(&data);

    let initial = model.get_round_ints(1);

    let mut upperbound: i128 = std::i128::MAX;
    let mut best_solution: Option<Node> = None;

    let source = Node::new(
        None,
        initial.clone(),
        &data.dist,
    );
    
    // ADD SOURCE NODE TO STACK
    let mut nodes: Vec<Node> = Vec::new();
    nodes.push(source.clone());

    // START LWOERBOUND_THREAD
    let lowerbound:Arc<Mutex<Vec<Vec<i128>>>> = Arc::new(Mutex::new(vec![vec![0; model.num_rounds as usize]; model.num_rounds as usize]));
    let lowerbound_clone:Arc<Mutex<Vec<Vec<i128>>>> = Arc::clone(&lowerbound);

    let dist_clone = Arc::new(data.dist.clone());
    let dist_clone_lb = Arc::clone(&dist_clone);

    let num_rounds = model.num_rounds;
    let model_clone = model.clone();
    let _ = thread::spawn(
        move || {
            calculate_lowerbound(
                dist_clone_lb,
                lowerbound_clone,
                num_rounds,
                q1,
                q2,
                model_clone,
            )
        }
    );

    // START BRANCH AND BOUND
    while nodes.len() > 0 {
        // POP NEW STATE FROM STACK
        let current_state = nodes.pop().unwrap();
        
        // EVALUATE
        let val = current_state.score;
        let lb_val = lowerbound.lock().unwrap().clone();
        if val >= lb_val[(num_rounds - 1) as usize][(current_state.round_index - 1) as usize] && val < upperbound {
            if (current_state.round_index as usize) < num_rounds as usize {
                // ADD ALL FEASIBLE CHILDREN TO EXPLORE
                let children = current_state.generate_children(q1, q2, model.get_round_ints(current_state.round_index + 1), upperbound, num_rounds, true);

                // CREATE AND ADD ALL CHILDREN
                if !children.is_empty() {
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
                println!("lb = {}, ub = {}", lb_val[(num_rounds - 1) as usize][(current_state.round_index - 1) as usize], upperbound);
                best_solution = Some(current_state.clone());
            }
        }

        if lb_val[(current_state.round_index - 1) as usize][(num_rounds - 1) as usize] == upperbound {
            break;
        }
    }
    
    if let Some(best_solution) = best_solution {
        best_solution.export(file_name);
        return Ok(best_solution.score);
    }

    Err("No solution found")
}

#[derive(Clone)]
pub struct Node<'a> {
    parent: Option<Box<Node<'a>>>,
    new_assignments: Vec<(i32, i32)>,
    pub score: i128,
    pub round_index: i32,
    dist: &'a Vec<Vec<i128>>,
    visited_teams: Vec<Vec<bool>>,
}

// impl<'a> std::fmt::Debug for Node<'a> {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>
//     ) -> std::fmt::Result {
//         if let Some(parent) = &self.parent {
//             write!(f, r#"{}
// {:?}"#, parent, self.new_assignments)
//         } else {
//             for i in 0..self.new_assignments.len() {
//                 let tuple = &self.new_assignments[i];
//                 write!(f, "{:?}", tuple.0);
//                 if i != self.new_assignments.len() - 1 {
//                     write!(f, " ");
//                 }
//             }
//             write!(f, "{:?}")
//         }
//     }
// }

impl<'a> std::fmt::Debug for Node<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, r#"{:?}
{:?} {}"#, parent, self.new_assignments, self.score)
        } else {
            write!(f, "{:?} {:?}", self.new_assignments, self.score)
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

impl<'a> Node<'a> {
    pub fn new(
        parent: Option<Box<Node<'a>>>,
        new_assignments: Vec<(i32, i32)>,
        dist: &'a Vec<Vec<i128>>,
    ) -> Self {
        let mut visited_teams: Vec<Vec<bool>> = vec![vec![false; new_assignments.len() * 2]; new_assignments.len()];
        for i in 0..new_assignments.len() {
            let assignment = &new_assignments[i];
            visited_teams[i as usize][(assignment.0 - 1) as usize] = true;
        }
        let mut round_index = 1;
        let mut score: i128 = 0;
        if let Some(parent) = &parent {
            round_index += parent.round_index;
            score += parent.score;
            for i in 0..new_assignments.len() {
                let new_assignment = &new_assignments[i];
                let previous_assignment = parent.new_assignments[i];
                let from = previous_assignment.0 - 1;
                let to = new_assignment.0 - 1;
                score += dist[from as usize][to as usize];
            }

            for i in 0..visited_teams.len() {
                for j in 0..visited_teams[i].len() {
                    if parent.visited_teams[i][j] == true {
                        visited_teams[i][j] = true;
                    }
                }
            }
        }

        Self {
            parent: parent,
            new_assignments,
            score,
            round_index,
            dist,
            visited_teams,
        }
    }

    pub fn set_round_index(
        mut self,
        index: i32,
    ) -> Self {
        self.round_index = index;
        self
    }

    pub fn pre_evaluate(
        &self,
        assignments: &Vec<(i32, i32)>,
        upperbound: i128,
    ) -> bool {
        let previous_locations: Vec<i32> = self.get_current_locations();
        let mut score: i128 = self.score;

        for i in 0..previous_locations.len() {
            let from: i32 = previous_locations[i] - 1;
            let to: i32 = assignments[i].0 - 1;
            score += self.dist[from as usize][to as usize];
        }

        score < upperbound
    }

    pub fn check_global(
        &self,
        num_rounds_left: i32,
    ) -> bool {
        let mut counter: Vec<i32> = vec![0; self.visited_teams.len()];

        for i in 0..self.visited_teams.len() {
            for elem_inner in &self.visited_teams[i as usize] {
                if !elem_inner {
                    counter[i] += 1;
                }
            }
        }

        *counter.iter().max().unwrap() <= num_rounds_left
    }

    pub fn check_global_mutations(
        &self,
        num_rounds_left: i32,
        mutations: &Vec<(i32, i32)>,
    ) -> bool {
        let mut counter: Vec<i32> = vec![0; self.visited_teams.len()];
        let mut new_visited: Vec<Vec<bool>> = self.visited_teams.clone();

        for (i, mutation) in mutations.iter().enumerate() {
            new_visited[i][(mutation.0 - 1) as usize] = true;
        }

        for i in 0..new_visited.len() {
            for elem_inner in &new_visited[i as usize] {
                if !elem_inner {
                    counter[i] += 1;
                }
            }
        }

        *counter.iter().max().unwrap() <= num_rounds_left
    }

    pub fn generate_children(
        &self,
        q1: i32,
        q2: i32,
        mut options: Vec<(i32, i32)>,
        best: i128,
        num_rounds: i32,
        is_minimizing: bool,
    ) -> Vec<Vec<(i32, i32)>> {
        let mut result = Vec::new();

        if is_minimizing && !self.check_global(num_rounds - self.round_index - 1) {
            return result;
        }

        let num_checks_q1 = q1 - 2;
        let stop_round_q1 = self.round_index - num_checks_q1;

        let num_checks_q2 = q2 - 2;
        let stop_round_q2 = self.round_index - num_checks_q2;

        // permutate(&mut options, 0, &mut result);
        // println!("result.len() = {}", result.len());
        result = options.iter().permutations(options.len()).map(|p| p.into_iter().cloned().collect()).collect();
        let mut counter = 0;

        result.into_iter()
            .filter(|perm| {
                // let is_global = self.check_global_mutations(num_rounds - self.round_index, perm);
                // if !is_global {
                //     return false;
                // }

                let is_q1 = self.check_q1(stop_round_q1, perm);
                if !is_q1 {
                    counter += 1;
                    // if !is_minimizing {
                    //     println!("Q1! Counter: {}", counter);
                    // }
                    return false;
                }

                let is_q2 = self.check_q2(stop_round_q2, perm);
                if !is_q2 {
                    counter += 1;
                    // if !is_minimizing {
                    //     println!("Q2! Counter: {}", counter);
                    // }
                    return false;
                }

                let is_pre_evaluated = self.pre_evaluate(perm, best);

                if is_minimizing {
                    if !is_pre_evaluated {
                        counter += 1;
                        // println!("EVAL! Counter: {}", counter);
                        return false;
                    }
                } else {
                    if is_pre_evaluated {
                        counter += 1;
                        // println!("EVAL! Counter: {}", counter);
                        return false;
                    }
                }

                true
            })
            .collect::<Vec<_>>()
    }

    pub fn get_current_locations(
        &self,
    ) -> Vec<i32> {
        self.new_assignments.iter().map(|(from, _)| *from).collect()
    }

    pub fn check_q1(
        &self,
        stop_round: i32,
        perm: &Vec<(i32, i32)>
    ) -> bool {
        let mut result = true;

        if stop_round < self.round_index {
            if let Some(parent) = &self.parent {
                result = parent.check_q1(stop_round, perm);
            };
        }
        
        let is_visited = self.is_visited(perm);
        result && !is_visited
    }

    pub fn check_q2(
        &self,
        stop_round: i32,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        let mut result = true;
        
        if stop_round < self.round_index {
            if let Some(parent) = &self.parent {
                result = parent.check_q2(stop_round, assignments);
            }
        }
        
        let is_officiated = self.is_officiated(assignments);
        result && !is_officiated
    }

    pub fn is_visited(
        &self,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        for i in 0..assignments.len() {
            let assignment = assignments[i];
            let new_assignment = self.new_assignments[i];
            if assignment.0 == new_assignment.0 {
                return true;
            }
        }

        false
    }

    pub fn is_officiated(
        &self,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        for i in 0..assignments.len() {
            let assignment = assignments[i];
            let new_assignment = self.new_assignments[i];
            
            if assignment.0 == new_assignment.0 || assignment.0 == new_assignment.1 || 
               assignment.1 == new_assignment.0 || assignment.1 == new_assignment.1 {
                return true;
            }
        }

        false
    }

    pub fn export_vec(
        &self,
    ) -> Vec<Vec<i32>> {
        let mut result;
        
        if let Some(parent) = &self.parent {
            result = parent.export_vec();
        } else {
            result = Vec::new();
            for i in 0..self.new_assignments.len() {
                result.push(Vec::new());
            }
        }

        for i in 0..self.new_assignments.len() {
            result[i].push(self.new_assignments[i].0)
        }

        result
    }

    pub fn export(
        &self,
        name: &str,
    ) {
        let result = self.export_vec();
        let _  =File::create(format!("solution_{}.txt", name))
                            .expect("Could not create file")
                            .write_all(format!("{}", self)
                            .as_bytes());
    }

    pub fn export_pdf(
        &self,
        name: &str,
    ) {
        let result = self.export_vec();
        let mut file = File::create(format!("solution_{}.txt", name)).expect("Could not create file");
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                let elem = &result[i][j];
                file.write_all(format!("{}", elem).as_bytes()).expect("Could not write to file");
                if i != result.len() - 1 || j != result[i].len() - 1 {
                    file.write_all(b",").expect("Could not write to file");
                }
            }
        }
        let _ = file.write_all(format!("{}", self).as_bytes());
    }
}