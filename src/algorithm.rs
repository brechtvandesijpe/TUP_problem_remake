use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::borrow::BorrowMut;

// DEBUGGING
const ENABLE_DEBUG_PRINT: bool = false;             // Print each time a new solution is found in Global
const ENABLE_UPDATE_PRINTS: bool = false;           // Print each time the best score found by LB

// LOWERBOUND CALCULATIONS
const ENABLE_LOWERBOUND: bool = true;
const ENABLE_LOWERBOUND_PRUNING: bool = true;
const PARRALLELIZE_LOWERBOUND: bool = false;
const FIXATE_LB: bool = true;

// GLOBAL PROBLEM
const ENABLE_UPPERBOUND_PRUNING: bool = true;
const FIXATE_GLOBAL: bool = true;
const PRINT_BEST_SOLUTION: bool = true;

// CONSTRAINTS
const ENABLE_GLOBAL_PRUNING: bool = true;
const ENABLE_ASSIGNMENT_PRUNING: bool = true;
const ENABLE_Q1_PRUNING: bool = true;
const ENABLE_Q2_PRUNING: bool = true;


#[derive(Debug, Clone)]
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

    pub fn as_tuple(
        &self,
    ) -> (Option<i32>, Option<i32>) {
        (Some(self.home_player), Some(self.out_player))
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
    
    pub fn as_vec(
        &self,
    ) -> Vec<&Game> {
        let mut output = Vec::new();

        for game in &self.games {
            output.push(game);
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

    pub fn get_round(
        &self,
        round_index: i32,
    ) -> Vec<&Game> {
        self.rounds[round_index as usize].as_vec()
    }
}

#[derive(Clone)]
struct Solution {
    assignments: Vec<Vec<(Option<i32>, Option<i32>)>>,
    pub num_umpires: usize,
    pub num_rounds: usize,
    score: i128,
}

impl std::fmt::Display for Solution {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        let max_val = self.num_umpires * 2;
        let max_val_chars = max_val.to_string().len();

        for round in &self.assignments {
            for game in round {
                let home = match game.0 {
                    Some(home) => format!("{:width$}", home, width = max_val_chars),
                    None => format!("{:width$}", "None", width = max_val_chars),
                };

                let out = match game.1 { // Assuming this should be game.1 for the 'out' value
                    Some(out) => format!("{:width$}", out, width = max_val_chars),
                    None => format!("{:width$}", "None", width = max_val_chars),
                };

                write!(f, "({}, {})", home, out)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Solution {
    pub fn new(
        num_rounds: usize,
        num_umpires: usize) -> Self {
        Self {
            assignments: vec![vec![(None, None); num_umpires]; num_rounds],
            num_umpires,
            num_rounds,
            score: 0,
        }
    }

    pub fn get_home_player(
        &self,
        umpire_team: i32,
        round: i32,
    ) -> Option<i32> {
        self.assignments[round as usize][umpire_team as usize].0
    }

    pub fn get_out_player(
        &self,
        umpire_team: i32,
        round: i32,
    ) -> Option<i32> {
        self.assignments[round as usize][umpire_team as usize].1
    }

    pub fn get_extra_distance(
        &self,
        next_location: i32,
        umpire_team: i32,
        round: i32,
        data: &Data,
    ) -> i128 {
        if round <= 0 {
            return 0;
        }
        
        let previous_location = self.assignments[(round - 1) as usize][umpire_team as usize].0;
        
        if previous_location.is_none() {
            panic!("Previous location is None");
        }

        if previous_location.unwrap() <= 0 {
            return 0;
        }

        data.dist[(previous_location.unwrap() - 1) as usize][(next_location - 1) as usize]
    }

    pub fn assign(
        &mut self,
        game: &Game,
        umpire_team: i32,
        round: i32,
        data: &Data,
    ) {
        let current_val = self.assignments[round  as usize][umpire_team as usize];

        if !current_val.0.is_none() || !current_val.1.is_none() {
            self.unassign(umpire_team, round, data);
        }

        self.score += self.get_extra_distance(game.home_player, umpire_team, round, data);
        self.assignments[round  as usize][umpire_team as usize] = game.as_tuple();
    }

    pub fn unassign(
        &mut self,
        umpire_team: i32,
        round: i32,
        data: &Data,
    ) {
        let home_location = self.assignments[round as usize][umpire_team as usize].0;

        match home_location {
            Some(home_location) => {
                self.score -= self.get_extra_distance(home_location, umpire_team, round, data);
                self.assignments[round as usize][umpire_team as usize] = (None, None);
            },
            None => panic!("Location was not assigned but is unassigned"),
        }
    }

    pub fn fixate(
        &mut self,
        round: Vec<&Game>,
        start_round: usize,
    ) {
        for (i, game) in round.iter().enumerate() {
            self.assignments[start_round][i] = game.as_tuple();
        }
    }
}

fn pretty_print(
    matrix: &Vec<Vec<i128>>
) {
    for row in matrix {
        println!("{:?}", row);
    }
}

pub fn branch_and_bound(
    file_name: &str,
    q1: i32,
    q2: i32
) -> i128 {
    let data = read_data(format!("resources/{}.txt", file_name).as_str()).unwrap();
    let model = Model::new(&data);

    let round_lbs: Arc<Mutex<Vec<Vec<i128>>>> = Arc::new(Mutex::new(vec![vec![0; model.num_rounds as usize]; model.num_rounds as usize]));
    let round_lbs_clone:Arc<Mutex<Vec<Vec<i128>>>> = Arc::clone(&round_lbs);

    let model_clone = Arc::new(model.clone());
    let model_clone_lb = Arc::clone(&model_clone);

    let data_clone = Arc::new(data.clone());
    let data_clone_lb = Arc::clone(&data_clone);

    if ENABLE_LOWERBOUND {
        let handle = thread::spawn(
            move || {
                calculate_lb(
                    &model_clone_lb,
                    &data_clone_lb,
                    q1,
                    q2,
                    &round_lbs_clone
                )
            }
        );
    
        if !PARRALLELIZE_LOWERBOUND {
            handle.join().unwrap();
        }

        pretty_print(&round_lbs.lock().unwrap());
    }

    let mut solution = Solution::new(model.num_rounds as usize, (data.num_teams / 2) as usize);
    let initial = model.get_round(0);
    
    let mut first_round: i32 = 0;
    if FIXATE_GLOBAL {
        solution.fixate(initial, first_round as usize);
        first_round += 1;
    }
    

    let best_solution = solution.clone();
    let (best_solution, best_score, _, _) =
        traverse(
            best_solution,
            999999999,
            solution,
            999999999,
            0,
            first_round,
            q1,
            q2,
            &model,
            &data,
            &round_lbs
        );
    
    // println!("{}", best_solution);
    if PRINT_BEST_SOLUTION {
        println!("\nBest solution:");
        println!("{}", best_solution);
    }
    best_score
}

pub fn calculate_lb(
    model: &Model,
    data: &Data,
    q1: i32,
    q2: i32,
    round_lbs: &Arc<Mutex<Vec<Vec<i128>>>>,
) {
    for k in 1..model.num_rounds-3 {
        let r = model.num_rounds - k - 1;
        // println!("r = {}, num_rounds = {}, k = {}", r, model.num_rounds, k);
        let start_round = r as usize;
        let end_round = (r + k) as usize;
        // println!("start_round = {}, end_round = {}", start_round, end_round);

        let mut solution = Solution::new(model.num_rounds as usize, (data.num_teams / 2) as usize);
        let initial = model.get_round(start_round as i32);
        
        let mut first_round: i32 = start_round as i32;
        if FIXATE_LB {
            solution.fixate(initial, first_round as usize);
            first_round += 1;
        }

        let best_solution = solution.clone();
        
        let (best_solution, _, _, _) = 
            traverse_lb(
                best_solution,
                999999999,
                solution,
                999999999,
                0,
                first_round,
                q1,
                q2,
                &model,
                &data,
                start_round,
                end_round,
                round_lbs
            );

        // println!("Score = {}", best_solution.score);
        // println!("{}", best_solution);

        let mut matrix = round_lbs.lock().unwrap();
        for r1 in (0..=r).rev() {
            for r2 in (r + k)..model.num_rounds {
                *matrix[r1 as usize][r2 as usize].borrow_mut() = 
                    std::cmp::max(
                        matrix[r1 as usize][r2 as usize],
                        matrix[r1 as usize][r as usize] + best_solution.score + matrix[(r + k) as usize][r2 as usize]
                    );
            }
        }

        // pretty_print(&matrix);
    }
}

fn is_terminal_lb(
    solution: &Solution,
    current_umpire: i32,
    current_round: i32,
    end_round: usize,
) -> bool {
    current_umpire + 1 == solution.num_umpires as i32 && current_round == end_round as i32
}

fn traverse_lb(
    mut best_solution: Solution,
    mut best_score: i128,
    mut solution: Solution,
    mut upperbound: i128,
    current_umpire: i32,
    current_round: i32,
    q1: i32,
    q2: i32,
    model: &Model,
    data: &Data,
    start_round: usize,
    end_round: usize,
    round_lbs: &Arc<Mutex<Vec<Vec<i128>>>>,
) -> (Solution, i128, Solution, i128) {
    let next_umpire = (current_umpire + 1) % (solution.num_umpires as i32);
    let next_round = if current_umpire == solution.num_umpires as i32 - 1 { current_round + 1 } else { current_round };

    // println!("current_round = {}", current_round);
    for game in model.get_round(current_round) {
        // FEASIBILITY CHECK OF THE GAMES:
        // - PREVIOUS UMPIRE ASSIGNMENTS FEASIBILITY
        if ENABLE_ASSIGNMENT_PRUNING {
            let mut assignment_feasible = true;
            for umpire in 0..current_umpire {
                let home_player = solution.get_home_player(umpire, current_round);

                if home_player.is_none() {
                    eprintln!("Home player is None in LB assignment pruning!");
                    assignment_feasible = false;
                    break;
                }

                let out_player = solution.get_out_player(umpire, current_round);

                if out_player.is_none() {
                    eprintln!("Out player is None in LB assignment pruning!");
                    assignment_feasible = false;
                    break;
                }

                if game.home_player == home_player.unwrap() && game.out_player == out_player.unwrap() {
                    assignment_feasible = false;
                    break;
                }
            }

            if !assignment_feasible {
                continue;
            }
        }
        
        // - Q1 CONSTRAINT
        if ENABLE_Q1_PRUNING {
            let mut q1_feasible = true;
            let stop_round_q1 = std::cmp::max(start_round as i32, current_round - q1 + 1);

            for round in stop_round_q1..current_round {
                let home_player = solution.get_home_player(current_umpire, round);

                match home_player {
                    Some(home_player) => {
                        if game.home_player == home_player {
                            q1_feasible = false;
                            break;
                        }
                    },
                    None => panic!("Home player is None when pruning Q1 in LB")
                }
            }

            if !q1_feasible {
                continue;
            }
        }
    
        // - Q2 CONSTRAINT
        if ENABLE_Q2_PRUNING {
            let mut q2_feasible = true;
            let stop_round_q2 = std::cmp::max(start_round as i32, current_round - q2 + 1);
            for round in stop_round_q2..current_round {
                let home_player = solution.get_home_player(current_umpire, round);

                if home_player.is_none() {
                    eprintln!("Home player is None in LB Q2 pruning!");
                    q2_feasible = false;
                    break;
                }

                let out_player = solution.get_out_player(current_umpire, round);

                if out_player.is_none() {
                    eprintln!("Out player is None in LB Q2 pruning!");
                    q2_feasible = false;
                    break;
                }

                if game.home_player == home_player.unwrap() ||
                   game.home_player == out_player.unwrap() ||
                   game.out_player == home_player.unwrap() ||
                   game.out_player == out_player.unwrap()
                {
                    q2_feasible = false;
                    break;
                }
            }
    
            if !q2_feasible {
                continue;
            }
        }
        
        solution.assign(game, current_umpire, current_round, data);
        let mut lowerbound = round_lbs.lock().unwrap()[current_round as usize][end_round];

        if !ENABLE_LOWERBOUND_PRUNING {
            lowerbound = 0;
        }

        if solution.score + lowerbound >= upperbound {
            solution.unassign( current_umpire, current_round, data);
            continue;
        }

        let is_terminal = is_terminal_lb(&solution, current_umpire, current_round, end_round);
        if is_terminal {
            if solution.score < best_score {
                best_score = solution.score;
                
                if ENABLE_UPPERBOUND_PRUNING {
                    upperbound = solution.score;
                }

                best_solution = solution.clone();
            }
        } else {
            // println!("next_round = {}, next_umpire = {}", next_round, next_umpire);
            (best_solution, best_score, solution, upperbound) = 
                traverse_lb(
                    best_solution,
                    best_score,
                    solution,
                    upperbound,
                    next_umpire,
                    next_round,
                    q1,
                    q2,
                    model,
                    data,
                    start_round,
                    end_round,
                    round_lbs
                );
        }
        solution.unassign( current_umpire, current_round, data);
    }

    (best_solution, best_score, solution, upperbound)
}

fn is_terminal(
    solution: &Solution,
    current_umpire: i32,
    current_round: i32,
    round_lbs: &Arc<Mutex<Vec<Vec<i128>>>>,
    end_round: usize,
    upperbound: i128,
) -> bool {
    let lowerbound = round_lbs.lock().unwrap()[current_round as usize][end_round];
    lowerbound == upperbound || current_umpire + 1 == solution.num_umpires as i32 && current_round + 1 == solution.num_rounds as i32
}

fn traverse(
    mut best_solution: Solution,
    mut best_score: i128,
    mut solution: Solution,
    mut upperbound: i128,
    current_umpire: i32,
    current_round: i32,
    q1: i32,
    q2: i32,
    model: &Model,
    data: &Data,
    round_lbs: &Arc<Mutex<Vec<Vec<i128>>>>,
) -> (Solution, i128, Solution, i128) {
    if ENABLE_DEBUG_PRINT {
        println!("current_umpire = {}, current_round = {}, best_score = {}, upperbound = {}", current_umpire, current_round, best_score, upperbound); 
    }

    if ENABLE_GLOBAL_PRUNING {
        let mut visited_teams: Vec<bool> = vec![false; data.num_teams as usize];
        for round in 0..current_round {
            let home_player = solution.get_home_player(current_umpire, round);
            match home_player {
                Some(home_player) => {
                    visited_teams[(home_player - 1) as usize] = true;
                },
                None => panic!("Home player is None when pruning global")
            }
        }

        let num_unvisited = visited_teams.iter().filter(|&v| *v == false).count();
        if num_unvisited >= (model.num_rounds - current_round) as usize {
            return (best_solution, best_score, solution, upperbound);
        }
    }

    let next_umpire = (current_umpire + 1) % (solution.num_umpires as i32);
    let next_round = if current_umpire == solution.num_umpires as i32 - 1 { current_round + 1 } else { current_round };

    for game in model.get_round(current_round) {
        if ENABLE_ASSIGNMENT_PRUNING {
            let mut assignment_feasible = true;
            for umpire in 0..current_umpire {
                let home_player = solution.get_home_player(umpire, current_round);

                if home_player.is_none() {
                    eprintln!("Home player is None in assignment pruning!");
                    assignment_feasible = false;
                    break;
                }

                let out_player = solution.get_out_player(umpire, current_round);

                if out_player.is_none() {
                    eprintln!("Out player is None in assignment pruning!");
                    assignment_feasible = false;
                    break;
                }

                if game.home_player == home_player.unwrap() && game.out_player == out_player.unwrap() {
                    assignment_feasible = false;
                    break;
                }
            }

            if !assignment_feasible {
                continue;
            }
        }
    
        // - Q1 CONSTRAINT
        if ENABLE_Q1_PRUNING {
            let mut q1_feasible = true;
            let stop_round_q1 = std::cmp::max(0, current_round - q1 + 1);

            for round in stop_round_q1..current_round {
                let home_player = solution.get_home_player(current_umpire, round);

                match home_player {
                    Some(home_player) => {
                        if game.home_player == home_player {
                            q1_feasible = false;
                            break;
                        }
                    },
                    None => panic!("Home player is None when pruning Q1 in LB")
                }
            }

            if !q1_feasible {
                continue;
            }
        }
    
        // - Q2 CONSTRAINT
        if ENABLE_Q2_PRUNING {
            let mut q2_feasible = true;
            let stop_round_q2 = std::cmp::max(0, current_round - q2 + 1);

            for round in stop_round_q2..current_round {
                let home_player = solution.get_home_player(current_umpire, round);

                if home_player.is_none() {
                    eprintln!("Home player is None in LB Q2 pruning!");
                    q2_feasible = false;
                    break;
                }

                let out_player = solution.get_out_player(current_umpire, round);

                if out_player.is_none() {
                    eprintln!("Out player is None in LB Q2 pruning!");
                    q2_feasible = false;
                    break;
                }

                if game.home_player == home_player.unwrap() ||
                   game.home_player == out_player.unwrap() ||
                   game.out_player == home_player.unwrap() ||
                   game.out_player == out_player.unwrap()
                {
                    q2_feasible = false;
                    break;
                }
            }

            if !q2_feasible {
                continue;
            }
        }
        
        let extra_distance = solution.get_extra_distance(game.home_player, current_umpire, current_round, data);
        let mut lowerbound = round_lbs.lock().unwrap()[current_round as usize][(solution.num_rounds - 1) as usize];
        
        if !ENABLE_LOWERBOUND_PRUNING {
            lowerbound = 0;
        }

        if solution.score + extra_distance + lowerbound >= upperbound {
            continue;
        }

        solution.assign(game, current_umpire, current_round, data);
        if is_terminal(&solution, current_umpire, current_round, round_lbs, solution.num_rounds - 1, upperbound) {
            if solution.score < best_score {
                best_score = solution.score;

                if ENABLE_UPDATE_PRINTS {
                    println!("best_score = {}", best_score);
                }

                if ENABLE_UPPERBOUND_PRUNING {
                    upperbound = solution.score;
                }

                best_solution = solution.clone();
            }
        } else {
            (best_solution, best_score, solution, upperbound) = 
                traverse(
                    best_solution,
                    best_score,
                    solution,
                    upperbound,
                    next_umpire,
                    next_round,
                    q1,
                    q2,
                    model,
                    data,
                    round_lbs
                );
        }
        solution.unassign( current_umpire, current_round, data);
    }

    (best_solution, best_score, solution, upperbound)
}