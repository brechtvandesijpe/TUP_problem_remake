
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

#[derive(Clone)]
pub struct Node<'a> {
    parent: Option<Box<Node<'a>>>,
    game: &'a Game,
    pub umpire_index: i32,
    pub score: i128,
    pub round_index: i32,
    dist: &'a Vec<Vec<i128>>,
    visited_teams: Vec<bool>,
}

impl<'a> Node<'a> {
    pub fn new(
        parent: Option<Box<Node<'a>>>,
        game: &'a Game,
        umpire_index: i32,
        dist: &'a Vec<Vec<i128>>,
        previous_team_score: i128,
    ) -> Self {
        let mut visited_teams = vec![false; dist.len()];
        visited_teams[(game.home_player - 1) as usize] = true;

        let mut round_index = 1;
        let mut score: i128 = previous_team_score;

        if let Some(parent) = &parent {
            round_index += parent.round_index;
            score += parent.score + dist[(parent.game.home_player - 1) as usize][(game.home_player - 1) as usize];
            
            for i in 0..visited_teams.len() {
                if parent.visited_teams[i] == true {
                    visited_teams[i] = true;
                }
            }
        }

        Self {
            parent,
            game,
            umpire_index,
            score,
            round_index,
            dist,
            visited_teams,
        }
    }

    pub fn is_visited(
        &self,
        new_game: &Game,
    ) -> bool {
        if self.game.home_player == new_game.home_player {
            return true;
        }

        false
    }

    pub fn is_officiated(
        &self,
        new_game: &Game,
    ) -> bool {
        if self.game.home_player == new_game.home_player || self.game.home_player == new_game.out_player || 
            self.game.out_player == new_game.home_player || self.game.out_player == new_game.out_player {
            return true;
        }

        false
    }

    pub fn is_previous(
        &self,
        new_assignment: &Game
    ) -> bool {
        if self.game.home_player == new_assignment.home_player && self.game.out_player == new_assignment.out_player {
            return true;
        }

        false
    }

    pub fn check_q1(
        &self,
        stop_round: i32,
        new_assignment: &Game,
    ) -> bool {
        let mut result = true;

        if stop_round < self.round_index {
            if let Some(parent) = &self.parent {
                result = parent.check_q1(stop_round, new_assignment);
            };
        }
        
        let is_visited = self.is_visited(new_assignment);
        result && !is_visited
    }

    pub fn check_q2(
        &self,
        stop_round: i32,
        new_assignment: &Game,
    ) -> bool {
        let mut result = true;
        
        if stop_round < self.round_index {
            if let Some(parent) = &self.parent {
                result = parent.check_q2(stop_round, new_assignment);
            }
        }
        
        let is_officiated = self.is_officiated(new_assignment);
        result && !is_officiated
    }

    pub fn check_global(
        &self,
        num_rounds_left: i32,
    ) -> bool {
        let mut counter = 0;

        for location in &self.visited_teams {
            if !location {
                counter += 1;
            }
        }

        counter <= num_rounds_left
    }

    pub fn check_previous(
        &self,
        new_assignment: &Game,
    ) -> bool {
        let mut result = true;
        if let Some(parent) = &self.parent {
            result = parent.check_previous(new_assignment);
        };
        
        let is_previous = self.is_previous(new_assignment);
        result && !is_previous
    }

    pub fn pre_evaluate(
        &self,
        new_assignment: &Game,
        upperbound: i128,
    ) -> bool {
        let mut score: i128 = self.score;

        if let Some(parent) = &self.parent {
            let from: i32 = parent.game.home_player - 1;
            let to: i32 = new_assignment.home_player - 1;
            score += self.dist[from as usize][to as usize];
        }

        score < upperbound
    }

    pub fn generate_children(
        &self,
        q1: i32,
        q2: i32,
        options: &'a Vec<&Game>,
        upperbound: i128,
    ) -> Vec<&Game> {
        let mut result = Vec::new();

        let num_checks_q1 = q1 - 2;
        let stop_round_q1 = self.round_index - num_checks_q1;

        let num_checks_q2 = q2 - 2;
        let stop_round_q2 = self.round_index - num_checks_q2;

        for option in options {
            let is_better = self.pre_evaluate(option, upperbound);
            if !is_better {
                continue;
            }

            let is_q1_feasible = self.check_q1(stop_round_q1, option);
            if !is_q1_feasible {
                continue;
            }

            let is_q2_feasible = self.check_q2(stop_round_q2, option);
            if !is_q2_feasible {
                continue;
            }

            let is_previous_feasible = self.check_previous(option);
            if !is_previous_feasible {
                continue;
            }

            result.push(*option);
        }

        result
    }
}

impl<'a> std::fmt::Debug for Node<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, r#"{:?}
({:?},{:?}) {}"#, parent, self.game.home_player, self.game.out_player, self.score)
        } else {
            write!(f, "({:?},{:?}) {:?}", self.game.home_player, self.game.out_player, self.score)
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
({:?},{:?})"#, parent, self.game.home_player, self.game.out_player)
        } else {
            write!(f, "({:?},{:?})", self.game.home_player, self.game.out_player)
        }
    }
}