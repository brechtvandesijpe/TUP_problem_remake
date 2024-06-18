use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::thread;
use std::fs;
use std::fmt;
use std::io::Write;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Instant;

// DEBUGGING
const ENABLE_DEBUG_PRINT: bool = false;
const ENABLE_UPDATE_PRINTS: bool = false;
const EXPORT_RESULT: bool = false;
const PRINT_BEST_SOLUTION: bool = false;
const PRINT_PRUNING_DEBUG: bool = false;
const PRINT_INTERMEDIATE_MATRIX: bool = false;
const PRINT_MODEL: bool = false;
const PRINT_LB_PREPROCESSING_MESSAGE: bool = false;
const PRINT_LB_EXPORT_MESSAGE: bool = false;
const PRINT_K_VALUE: bool = false;
const ENABLE_UPPERBOUND_UPDATE_PRINTS: bool = false;

// LOWERBOUND CALCULATIONS
const ENABLE_LOWERBOUND: bool = true;
const ENABLE_LOWERBOUND_PRUNING: bool = true;
const PARRALLELIZE_LOWERBOUND: bool = true;
const EXPORT_LB_MATRIX: bool = true;
const EXPORT_LB_PREPROCESSING_MATRIX: bool = true;
const FIXATE_LB: bool = true;
const SOLVE_PROBLEM: bool = true;

// PARTIAL MATCHING
const ENABLE_PARTIAL_MATCHING: bool = false;

// GLOBAL PROBLEM
const ENABLE_UPPERBOUND_PRUNING: bool = true;
const FIXATE_GLOBAL: bool = true;

// CONSTRAINTS
const ENABLE_GLOBAL_PRUNING: bool = true;
const ENABLE_ASSIGNMENT_PRUNING: bool = true;
const ENABLE_Q1_PRUNING: bool = true;
const ENABLE_Q2_PRUNING: bool = true;


#[derive(Debug, Clone)]
pub struct Data {
    pub num_teams: i32,
    pub dist: Vec<Vec<u64>>,
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
            dist = read_array_u64(&mut lines, num_teams as usize)?;
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

fn read_array_u64(
    lines: &mut std::io::Lines<BufReader<File>>,
    rows: usize
) -> io::Result<Vec<Vec<u64>>> {
    let mut array = Vec::new();

    for _ in 0..rows {
        let line = lines.next().unwrap()?;
        let row: Vec<u64> = line.split(|c: char| c == '[' || c == ']' || c.is_whitespace())
            .filter(|part| !part.is_empty())
            .map(|part| u64::from_str(part).unwrap())
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

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for game in &self.games {
            write!(f, "({}, {}) ", game.home_player, game.out_player)?;
        }
        Ok(())
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

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for round in &self.rounds {
            write!(f, "{}\n", round)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Solution {
    assignments: Vec<Vec<(Option<i32>, Option<i32>)>>,
    pub num_umpires: usize,
    pub num_rounds: usize,
    score: u64,
}

impl fmt::Display for Solution {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
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

                write!(f, "({}, {}) ", home, out)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Solution {
    pub fn new(
        num_rounds: usize,
        num_umpires: usize,
    ) -> Self {
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

    pub fn get_assignment(
        &self,
        umpire_team: i32,
        round: i32,
    ) -> (Option<i32>, Option<i32>) {
        self.assignments[round as usize][umpire_team as usize]
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
    ) -> u64 {
        if round <= 0 {
            return 0;
        }
        
        let previous_location = self.assignments[(round - 1) as usize][umpire_team as usize].0;
        
        if previous_location.is_none() {
            return 0;                           // TODO: check if allowed
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
    
    pub fn export_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        let mut wtr = Writer::from_writer(file);

        // Write the header
        wtr.write_record(&["Round", "Umpire", "Home", "Out"])?;

        for (round_index, round) in self.assignments.iter().enumerate() {
            for (umpire_index, game) in round.iter().enumerate() {
                let home = game.0.as_ref().map_or("None".to_string(), |h| h.to_string());
                let out = game.1.as_ref().map_or("None".to_string(), |o| o.to_string());

                // Write each game's data
                wtr.write_record(&[
                    (round_index + 1).to_string(),
                    (umpire_index + 1).to_string(),
                    home,
                    out,
                ])?;
            }
        }

        wtr.flush()?;
        Ok(())
    }

    pub fn get_unsucceeded(
        &self,
        round_index: usize,
    ) -> Vec<(Option<i32>, Option<i32>)> {
        let mut output = Vec::new();

        if round_index + 1 >= self.num_rounds {
            for _ in 0..self.num_umpires {
                output.push((None, None));
            }

            return output;
        }

        for (i, game) in self.assignments[round_index + 1].iter().enumerate() {
            let predecessor = &self.assignments[round_index][i];
            if game.0.is_none() {
                output.push(self.assignments[round_index][i]);
            }
        }

        output
    }

    pub fn get_unused(
        &self,
        round_index: usize,
        model: &Model,
    ) -> Vec<(Option<i32>, Option<i32>)> {
        let mut output = Vec::new();
        let assignments = &self.assignments[round_index];

        for game in model.rounds[round_index].games.iter() {
            let tuple = game.as_tuple();
            if !assignments.contains(&tuple) {
                output.push(tuple);
            }
        }

        output
    }
}

fn pretty_print(
    matrix: &Vec<Vec<u64>>
) {
    for row in matrix {
        println!("{:?}", row);
    }
}

pub fn branch_and_bound(
    file_name: &str,
    q1: i32,
    q2: i32
) -> u64 {
    let data = read_data(format!("resources/{}.txt", file_name).as_str()).unwrap();
    let model = Model::new(&data);

    let (tx, rx) = mpsc::channel();

    let model_clone = Arc::new(model.clone());
    let model_clone_lb = Arc::clone(&model_clone);

    let data_clone = Arc::new(data.clone());
    let data_clone_lb = Arc::clone(&data_clone);

    let mut input_round_lbs = vec![vec![0; model.num_rounds as usize]; model.num_rounds as usize];

    if ENABLE_LOWERBOUND {
        let file_name_arc = Arc::new(file_name.to_owned());
        let handle = thread::spawn(
            move || {
                calculate_lb(
                    &model_clone_lb,
                    &data_clone_lb,
                    q1,
                    q2,
                    tx,
                    file_name_arc.as_str(),
                )
            }
        );
    
        if !PARRALLELIZE_LOWERBOUND {
            handle.join().unwrap();

            loop {
                match rx.try_recv() {
                    Ok(new_round_lbs) => {
                        input_round_lbs = new_round_lbs;
                    },
                    Err(std::sync::mpsc::TryRecvError::Empty) => break, // Channel is empty
                    Err(_) => break,                                    // Handle other errors (e.g., disconnected)
                }
            }
            // pretty_print(&input_round_lbs);
        }

        // pretty_print(&round_lbs.lock().unwrap());
    }

    if SOLVE_PROBLEM {
        if PRINT_MODEL {
            println!("Rounds:");
            println!("{}", model);
        }
    
        let mut solution = Solution::new(model.num_rounds as usize, (data.num_teams / 2) as usize);
        let initial = model.get_round(0);
        
        let mut first_round: i32 = 0;
        if FIXATE_GLOBAL {
            solution.fixate(initial, first_round as usize);
            first_round += 1;
        }
    
        let best_solution = solution.clone();
        let (best_solution, best_score, _, _, _) =
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
                &rx,
                input_round_lbs,
            );
        
        // println!("{}", best_solution);
        if PRINT_BEST_SOLUTION {
            println!("\nBest solution:");
            println!("{}", best_solution);
        }
    
        
        if EXPORT_RESULT {
            let file_path = format!("results/{}_{}_{}.csv", file_name, q1, q2);
            if let Err(e) = fs::create_dir_all("results") {
                eprintln!("Failed to create directory 'results': {}", e);
                return best_score; // Or handle the error as appropriate for your application
            }
    
            match best_solution.export_to_csv(&file_path) {
                Ok(_) => println!("Successfully exported the solution to {}", file_path),
                Err(e) => eprintln!("Failed to export the solution: {}", e),
            }
        }
    
        best_score
    } else {
        0
    }
}

pub fn calculate_lb(
    model: &Model,
    data: &Data,
    q1: i32,
    q2: i32,
    tx: std::sync::mpsc::Sender<Vec<Vec<u64>>>,
    file_name: &str,
) {
    let mut round_lbs: Vec<Vec<u64>> = vec![vec![0; model.num_rounds as usize]; model.num_rounds as usize];

    let start = Instant::now();
    for round_index in 0..(model.num_rounds-1) as usize {
        let start_round = round_index as usize;
        let end_round = (round_index + 1) as usize;
        // println!("start_round = {}, end_round = {}", start_round, end_round);

        let mut solution = Solution::new(model.num_rounds as usize, (data.num_teams / 2) as usize);
        let initial = model.get_round(start_round as i32);
        

        let mut first_round: i32 = start_round as i32;
        if FIXATE_LB {
            solution.fixate(initial, first_round as usize);
            first_round += 1;
        }

        let best_solution = solution.clone();
        // println!("{}", end_round);
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
                &round_lbs
            );

        let next_round = (round_index + 1) as usize;

        for i in 0..=round_index as usize {
            for j in next_round..model.num_rounds as usize {
                round_lbs[i][j] = 
                std::cmp::max(
                    round_lbs[i][j],
                    round_lbs[i][round_index] + best_solution.score + round_lbs[round_index][j]
                );
            }
        }
    }

    if PRINT_LB_PREPROCESSING_MESSAGE {
        let duration = start.elapsed();
        println!("Lowerbound preprocessing done in {:?}", duration);
    }

    if EXPORT_LB_PREPROCESSING_MATRIX {
        let file_path = format!("results/{}_{}_{}_preprocessed_lb.txt", file_name, q1, q2);
        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create file: {}", e);
                return;
            }
        };

        for row in round_lbs.iter() {
            let row_str = row.iter()
                            .map(|&num| num.to_string())
                            .collect::<Vec<String>>()
                            .join(" ");
            if let Err(e) = writeln!(file, "{}", row_str) {
                eprintln!("Failed to write to file: {}", e);
                return;
            }
        }

        if PRINT_LB_EXPORT_MESSAGE {
            let duration = start.elapsed();
            println!("Successfully exported the solution to {}. LB thread finished in {:?}", file_path, duration);
        }
    }

    for k in 1..model.num_rounds {
        let r = model.num_rounds - k - 1;
        let start_round = r as usize;
        let end_round = (r + k) as usize;

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
                &round_lbs
            );

        for r1 in (0..=r).rev() {
            for r2 in (r + k)..model.num_rounds {
                round_lbs[r1 as usize][r2 as usize] = 
                    std::cmp::max(
                        round_lbs[r1 as usize][r2 as usize],
                        round_lbs[r1 as usize][r as usize] + best_solution.score + round_lbs[(r + k) as usize][r2 as usize]
                    );
            }
            match tx.send(round_lbs.clone()) {
                Ok(_) => {},
                Err(_) => return,
            }
        }


        if PRINT_K_VALUE {
            let duration = start.elapsed();
            println!("{} in {:?}", k, duration);
        }

        if PRINT_INTERMEDIATE_MATRIX {
            pretty_print(&round_lbs);
        }

    }
    
    if EXPORT_LB_MATRIX {
        let file_path = format!("results/{}_{}_{}_lb.txt", file_name, q1, q2);
        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create file: {}", e);
                return;
            }
        };

        for row in round_lbs.iter() {
            let row_str = row.iter()
                            .map(|&num| num.to_string())
                            .collect::<Vec<String>>()
                            .join(" ");
            if let Err(e) = writeln!(file, "{}", row_str) {
                eprintln!("Failed to write to file: {}", e);
                return;
            }
        }

        if PRINT_LB_EXPORT_MESSAGE {
            let duration = start.elapsed();
            println!("Successfully exported the solution to {}. LB thread finished in {:?}", file_path, duration);
        }
    }
}

fn is_terminal_lb(
    solution: &Solution,
    current_umpire: i32,
    current_round: i32,
    end_round: usize,
    lowerbound: u64,
    upperbound: u64,
) -> bool {
    lowerbound == upperbound || current_umpire + 1 == solution.num_umpires as i32 && current_round == end_round as i32
}

fn traverse_lb(
    mut best_solution: Solution,
    mut best_score: u64,
    mut solution: Solution,
    mut upperbound: u64,
    current_umpire: i32,
    current_round: i32,
    q1: i32,
    q2: i32,
    model: &Model,
    data: &Data,
    start_round: usize,
    end_round: usize,
    round_lbs: &Vec<Vec<u64>>,
) -> (Solution, u64, Solution, u64) {
    let next_umpire = (current_umpire + 1) % (solution.num_umpires as i32);
    let next_round = if current_umpire == solution.num_umpires as i32 - 1 { current_round + 1 } else { current_round };

    let mut round = model.get_round(current_round);
    round.sort_by_key(|element| solution.get_extra_distance(element.home_player, current_umpire, current_umpire, data));

    // println!("current_round = {}", current_round);
    for game in round {
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
                if PRINT_PRUNING_DEBUG {
                    println!("({}, {}) -> LB ASSIGNMENT", game.home_player, game.out_player);
                }
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
                if PRINT_PRUNING_DEBUG {
                    println!("({}, {}) -> LB Q1", game.home_player, game.out_player);
                }
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
                if PRINT_PRUNING_DEBUG {
                    println!("({}, {}) -> LB Q2", game.home_player, game.out_player);
                }
                continue;
            }
        }
        
        let extra_distance = solution.get_extra_distance(game.home_player, current_umpire, current_round, data);
        let mut lowerbound = round_lbs[current_round as usize][end_round];
        
        if !ENABLE_LOWERBOUND_PRUNING {
            lowerbound = 0;
        }

        let mut m = 0;

        if ENABLE_PARTIAL_MATCHING && current_round > 0 && current_round < (solution.num_rounds - 1) as i32 {
            // println!("{}", solution);
            let my_previous = solution.get_assignment(current_umpire, current_round - 1);
            // dbg!(my_previous);
            let unsucceeded = solution.get_unsucceeded((current_round - 1) as usize);
            let unused = solution.get_unused(current_round as usize, model);

            // dbg!(&unsucceeded);
            // dbg!(&unused);
            // println!("{}", unsucceeded.len() == unused.len());

            if unsucceeded.len() == unused.len() 
            && !unused.iter().any(|(first, second)| first.is_none() || second.is_none())
            && !unsucceeded.iter().any(|(first, second)| first.is_none() || second.is_none()) {
                let size = unsucceeded.len();
                let mut matrix = vec![0; size * size];
                for (from_index, from) in unsucceeded.iter().enumerate() {
                    for (to_index, to) in unused.iter().enumerate() {
                        matrix[from_index * size + to_index] = data.dist[(from.0.unwrap() - 1) as usize][(to.0.unwrap() - 1) as usize];
                    }
                }

                let previous_home = solution.get_home_player(current_umpire, current_round - 1);
                let previous_out = solution.get_out_player(current_umpire, current_round - 1);

                let mut my_index: i32 = -1;
                for i in 0..unsucceeded.len() {
                    let element = unsucceeded[i];
                    if element.0 == previous_home && element.1 == previous_out {
                        my_index = i as i32;
                        break;
                    }
                }

                if my_index < 0 {
                    let optimal_assignments = hungarian::minimize(&matrix, size, size);
                    let my_assignment = optimal_assignments[my_index as usize];
                    m = data.dist[(previous_home.unwrap() - 1) as usize][(my_assignment.unwrap() - 1) as usize];
                    // m = optimal_assignments.iter()
                    //                     .enumerate()
                    //                     .filter_map(|(i, &a)| {
                    //                         a.map(|j| matrix[i*size + j])
                    //                     })
                    //                     .sum();
                }
            }
        }
        
        if solution.score + extra_distance + lowerbound + m >= upperbound {
            if PRINT_PRUNING_DEBUG {
                println!("LB EVAL");
            }
            continue;
        }
        
        solution.assign(game, current_umpire, current_round, data);

        if is_terminal_lb(&solution, current_umpire, current_round, end_round, lowerbound, upperbound) {
            if solution.score < best_score {
                best_score = solution.score;

                if ENABLE_UPDATE_PRINTS {
                    println!("best_score = {}", best_score);
                    println!("{}", solution);
                }
                
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
    lowerbound: u64,
    upperbound: u64,
) -> bool {
    lowerbound == upperbound || current_umpire + 1 == solution.num_umpires as i32 && current_round + 1 == solution.num_rounds as i32
}

fn traverse(
    mut best_solution: Solution,
    mut best_score: u64,
    mut solution: Solution,
    mut upperbound: u64,
    current_umpire: i32,
    current_round: i32,
    q1: i32,
    q2: i32,
    model: &Model,
    data: &Data,
    rx: &std::sync::mpsc::Receiver<Vec<Vec<u64>>>,
    mut input_round_lbs: Vec<Vec<u64>>,
) -> (Solution, u64, Solution, u64, Vec<Vec<u64>>) {
    if ENABLE_DEBUG_PRINT {
        println!("current_umpire = {}, current_round = {}, best_score = {}, upperbound = {}", current_umpire, current_round, best_score, upperbound); 
    }

    if PRINT_PRUNING_DEBUG {
        println!("{}", solution);
    }

    // PRUNING BASED ON GLOBAL CONSTRAINT
    // -> PRUNE WHEN THE NUMBER OF UNVISITED LOCATIONS IS NOT GREATER THAN THE NUMBER OF ROUNDS LEFT
    if ENABLE_GLOBAL_PRUNING {
        let mut unvisited_teams: Vec<i32> = (1..data.num_teams).collect();
        for round in 0..current_round {
            let home_player = solution.get_home_player(current_umpire, round);
            match home_player {
                Some(home_player) => {
                    for i in 0..unvisited_teams.len() {
                        if unvisited_teams[i] == home_player {
                            unvisited_teams.remove(i);
                            break;
                        }
                    }
                },
                None => panic!("Home player is None when pruning global")
            }
        }

        let num_unvisited = unvisited_teams.len();
        if num_unvisited > (model.num_rounds - current_round) as usize {
            if PRINT_PRUNING_DEBUG {
                println!("-> GLOBAL");
            }
            return (best_solution, best_score, solution, upperbound, input_round_lbs);
        }
    }

    let next_umpire = (current_umpire + 1) % (solution.num_umpires as i32);
    let next_round = if current_umpire == solution.num_umpires as i32 - 1 { current_round + 1 } else { current_round };
    
    let mut round = model.get_round(current_round);
    round.sort_by_key(|element| solution.get_extra_distance(element.home_player, current_umpire, current_umpire, data));

    // println!("current_round = {}", current_round);
    for game in round {
        // PRUNE BASED ON ASSIGNMENTS OF PREVIOUS UMPIRE TEAMS IN THE SAME ROUND
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
                if PRINT_PRUNING_DEBUG {
                    println!("({}, {}) -> ASSIGNMENT", game.home_player, game.out_player);
                }
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
                if PRINT_PRUNING_DEBUG {
                    println!("({}, {}) -> Q1", game.home_player, game.out_player);
                }
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
                if PRINT_PRUNING_DEBUG {
                    println!("({}, {}) -> Q2", game.home_player, game.out_player);
                }
                continue;
            }
        }
        
        let extra_distance = solution.get_extra_distance(game.home_player, current_umpire, current_round, data);
        let mut received_lbs = None;

        if PARRALLELIZE_LOWERBOUND {
            loop {
                match rx.try_recv() {
                    Ok(new_round_lbs) => {
                        // println!("Received");
                        received_lbs = Some(new_round_lbs);
                    },
                    Err(std::sync::mpsc::TryRecvError::Empty) => break, // Channel is empty
                    Err(_) => break,                                    // Handle other errors (e.g., disconnected)
                }
            }
        }

        // println!("here");
        // pretty_print(&input_round_lbs);
        
        let lowerbound;
        
        match &received_lbs {
            None => lowerbound = input_round_lbs[current_round as usize][(solution.num_rounds - 1) as usize],
            _ => {
                lowerbound = received_lbs.clone().unwrap()[current_round as usize][(solution.num_rounds - 1) as usize];
                input_round_lbs = received_lbs.unwrap();
                // pretty_print(&input_round_lbs);
                // println!("{}", lowerbound);
            },
        };

        let mut m = 0;

        if ENABLE_PARTIAL_MATCHING && current_round > 0 && current_round < (solution.num_rounds - 1) as i32 {
            // println!("{}", solution);
            let my_previous = solution.get_assignment(current_umpire, current_round - 1);
            // dbg!(my_previous);
            let unsucceeded = solution.get_unsucceeded((current_round - 1) as usize);
            let unused = solution.get_unused(current_round as usize, model);

            // dbg!(&unsucceeded);
            // dbg!(&unused);
            // println!("{}", unsucceeded.len() == unused.len());

            if unsucceeded.len() == unused.len() 
            && !unused.iter().any(|(first, second)| first.is_none() || second.is_none())
            && !unsucceeded.iter().any(|(first, second)| first.is_none() || second.is_none()) {
                let size = unsucceeded.len();
                let mut matrix = vec![0; size * size];
                for (from_index, from) in unsucceeded.iter().enumerate() {
                    for (to_index, to) in unused.iter().enumerate() {
                        matrix[from_index * size + to_index] = data.dist[(from.0.unwrap() - 1) as usize][(to.0.unwrap() - 1) as usize];
                    }
                }

                let previous_home = solution.get_home_player(current_umpire, current_round - 1);
                let previous_out = solution.get_out_player(current_umpire, current_round - 1);

                let mut my_index: i32 = -1;
                for i in 0..unsucceeded.len() {
                    let element = unsucceeded[i];
                    if element.0 == previous_home && element.1 == previous_out {
                        my_index = i as i32;
                        break;
                    }
                }

                if my_index < 0 {
                    let optimal_assignments = hungarian::minimize(&matrix, size, size);
                    let my_assignment = optimal_assignments[my_index as usize];
                    m = data.dist[(previous_home.unwrap() - 1) as usize][(my_assignment.unwrap() - 1) as usize];
                    // m = optimal_assignments.iter()
                    //                     .enumerate()
                    //                     .filter_map(|(i, &a)| {
                    //                         a.map(|j| matrix[i*size + j])
                    //                     })
                    //                     .sum();
                }
            }
        }
        
        if solution.score + extra_distance + lowerbound + m >= upperbound {
            if PRINT_PRUNING_DEBUG {
                println!("EVAL");
            }
            continue;
        }

        solution.assign(game, current_umpire, current_round, data);

        if is_terminal(&solution, current_umpire, current_round, lowerbound, upperbound) {
            if ENABLE_GLOBAL_PRUNING {
                for umpire in 0..solution.num_umpires {
                    let mut unvisited_teams: Vec<i32> = (1..data.num_teams+1).collect();
                    for round in 0..current_round+1 {
                        let home_player = solution.get_home_player(umpire as i32, round);
                        match home_player {
                            Some(home_player) => {
                                for i in 0..unvisited_teams.len() {
                                    if unvisited_teams[i] == home_player {
                                        unvisited_teams.remove(i);
                                        break;
                                    }
                                }
                            },
                            None => panic!("Home player is None when pruning global")
                        }
                    }
                    
                    // println!("{}", unvisited_teams.len());

                    if unvisited_teams.len() > 0 as usize {
                        if PRINT_PRUNING_DEBUG {
                            println!("-> GLOBAL");
                        }
                        solution.unassign( current_umpire, current_round, data);
                        return (best_solution, best_score, solution, upperbound, input_round_lbs);
                    }
                }
            }

            if solution.score < best_score {
                best_score = solution.score;

                if ENABLE_UPDATE_PRINTS {
                    println!("best_score = {}", best_score);
                    println!("{}", solution);
                }

                if ENABLE_UPPERBOUND_PRUNING {
                    upperbound = solution.score;
                    if ENABLE_UPPERBOUND_UPDATE_PRINTS {
                        println!("{}", upperbound);
                    }
                }

                best_solution = solution.clone();
            }
        } else {
            (best_solution, best_score, solution, upperbound, input_round_lbs) = 
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
                    rx,
                    input_round_lbs,
                );
        }
        solution.unassign( current_umpire, current_round, data);
    }

    (best_solution, best_score, solution, upperbound, input_round_lbs)
}