
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

    pub fn as_tuple(
        &self,
    ) -> (i32, i32) {
        (self.home_player, self.out_player)
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
    assignments: Vec<Vec<(i32, i32)>>,
    pub num_umpires: usize,
    pub num_rounds: usize,
    score: i128,
}

impl std::fmt::Display for Solution {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        for round in &self.assignments {
            for game in round {
                write!(f, "{:?} ", game)?;
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
            assignments: vec![vec![(0, 0); num_umpires]; num_rounds],
            num_umpires,
            num_rounds,
            score: 0,
        }
    }

    pub fn get_home_player(
        &self,
        umpire_team: i32,
        round: i32,
    ) -> i32 {
        self.assignments[round as usize][umpire_team as usize].0
    }

    pub fn get_out_player(
        &self,
        umpire_team: i32,
        round: i32,
    ) -> i32 {
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
        data.dist[(previous_location - 1) as usize][(next_location - 1) as usize]
    }

    pub fn assign(
        &mut self,
        game: &Game,
        umpire_team: i32,
        round: i32,
        data: &Data,
    ) {
        // let current_val = self.assignments[round  as usize][umpire_team as usize];
        // if current_val.0 != 0 || current_val.1 != 0 {
        //     self.unassign(umpire_team, round, data);
        // }

        self.score += self.get_extra_distance(game.home_player, umpire_team, round, data);
        self.assignments[round  as usize][umpire_team as usize] = game.as_tuple();
    }

    pub fn unassign(
        &mut self,
        umpire_team: i32,
        round: i32,
        data: &Data,
    ) {
        self.score -= self.get_extra_distance(self.assignments[round as usize][umpire_team as usize].0, umpire_team, round, data);
        self.assignments[round as usize][umpire_team as usize] = (0, 0);
    }
}

pub fn branch_and_bound(
    file_name: &str,
    q1: i32,
    q2: i32
) -> i128 {
    let data = read_data(format!("resources/{}.txt", file_name).as_str()).unwrap();
    let model = Model::new(&data);
    let solution = Solution::new(model.num_rounds as usize, (data.num_teams / 2) as usize);
    let best_solution = solution.clone();
    let (best_solution, solution, upperbound) = traverse(best_solution, solution, 0, 0, 0, q1, q2, &model, &data);
    println!("{}", best_solution);
    upperbound
}

fn is_terminal(
    solution: &Solution,
    current_umpire: i32,
    current_round: i32,
) -> bool {
    current_umpire + 1 == (solution.num_umpires as i32) - 1 && current_round + 1 == solution.num_rounds as i32
}

fn traverse(
    mut best_solution: Solution,
    mut solution: Solution,
    mut upperbound: i128,
    current_umpire: i32,
    current_round: i32,
    q1: i32,
    q2: i32,
    model: &Model,
    data: &Data,
) -> (Solution, Solution, i128) {
    let mut visited_teams: Vec<bool> = vec![false; data.num_teams as usize];
    for round in 0..current_round {
        visited_teams[(solution.get_home_player(current_umpire, round) - 1) as usize] = true;
    }

    let num_unvisited = visited_teams.iter().filter(|&v| *v == false).count();
    if num_unvisited > model.num_rounds as usize - current_round as usize {
        return (best_solution, solution, upperbound);
    }

    // println!("current_umpire = {}, current_round = {}", current_umpire, current_round);
    // println!("{}", solution);
    let next_umpire = (current_umpire + 1) % (solution.num_umpires as i32);
    let next_round = if current_umpire == solution.num_umpires as i32 - 1 { current_round + 1 } else { current_round };
    // println!("next_umpire = {}, next_round = {}", next_umpire, next_round);

    for game in model.get_round(current_round) {
        // FEASIBILITY CHECK OF THE GAMES:
        // - GLOBAL FEASIBILITY
        let mut global_feasible = true;
        for round in 0..current_umpire {
            if game.home_player == solution.get_home_player(current_umpire, round) {
                global_feasible = false;
                break;
            }
        }

        if !global_feasible {
            continue;
        }

        // - PREVIOUS UMPIRE ASSIGNMENTS FEASIBILITY
        let mut assignment_feasible = true;
        for umpire in 0..current_umpire {
            if game.home_player == solution.get_home_player(umpire, current_round) && game.out_player == solution.get_out_player(umpire, current_round){
                assignment_feasible = false;
                break;
            }
        }

        if !assignment_feasible {
            continue;
        }
    
        // - Q1 CONSTRAINT
        let mut q1_feasible = true;
        let stop_round_q1 = std::cmp::max(0, current_round - q1 + 1);
        for round in stop_round_q1..current_round {
            if game.home_player == solution.get_home_player(current_umpire, round) {
                q1_feasible = false;
                break;
            }
        }

        if !q1_feasible {
            continue;
        }
    
        // - Q2 CONSTRAINT
        let mut q2_feasible = true;
        let stop_round_q2 = std::cmp::max(0, current_round - q2 + 1);
        for round in stop_round_q2..current_round {
            if game.home_player == solution.get_home_player(current_umpire, round) ||
               game.out_player == solution.get_out_player(current_umpire, round) {
                q2_feasible = false;
                break;
            }
        }

        if !q2_feasible {
            continue;
        }
        
        // println!("current_umpire = {}, current_round = {}, game = {:?}", current_umpire, current_round, game);

        
        let extra_distance = solution.get_extra_distance(game.home_player, current_umpire, current_round, data);
        let lowerbound = 0;
        
        if solution.score + extra_distance + lowerbound >= upperbound && upperbound != 0 {
            continue;
        }

        solution.assign(game, current_umpire, current_round, data);
        if is_terminal(&solution, current_umpire, current_round) {
            if solution.score < upperbound || upperbound == 0 {
                upperbound = solution.score;
                best_solution = solution.clone();
            }
        } else {
            (best_solution, solution, upperbound) = traverse(best_solution, solution, upperbound, next_umpire, next_round, q1, q2, model, data);
        }

        solution.unassign( current_umpire, current_round, data);
        // if !is_best {
        // }
    }

    (best_solution, solution, upperbound)
}