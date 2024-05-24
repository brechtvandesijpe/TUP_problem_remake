
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
        self.rounds[(round_index - 1) as usize].as_vec()
    }
}

struct Solution {
    assignments: Vec<Vec<(i32, i32)>>,

}

impl Solution {
    pub fn new(num_rounds: usize, num_umpire_teams: usize) -> Self {
        Self {
            assignments: vec![vec![(0, 0) ; num_umpire_teams] ; num_rounds],
        }
    }
}

pub fn branch_and_bound(
    file_name: &str,
    q1: i32,
    q2: i32
) -> i128 {
    let data = read_data(format!("resources/{}.txt", file_name).as_str()).unwrap();
    let model = Model::new(&data);

    let mut result = 0;

    let mut solution = Solution::new();
    let (solution, result) = traverse(solution, result, 0, 0, Vec::new());

    result
}

fn traverse(
    mut solution: Solution,
    result: i128,
    current_umpire: i32,
    current_round: i32,
    banned_games: Vec<&Game>,
) -> (Solution, i128) {
    // STEP 1 : ASSIGN A ALLOWED GAME BASED ON
    // - GLOBAL FEASIBILITY
    // - Q1 CONSTRAINT
    // - Q2 CONSTRAINT

    (solution, result)
}