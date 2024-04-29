use crate::data::*;

#[derive(Debug)]
pub struct Game {
    home_player: i32,
    out_player: i32,
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

#[derive(Debug)]
pub struct Round {
    games: Vec<Game>,
}

impl Round {
    pub fn new(opponents: Vec<i32>) -> Self {
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
}

#[derive(Debug)]
pub struct Model {
    rounds: Vec<Round>,
    num_rounds: i32,
}

impl Model {
    pub fn new(
        data: Data,
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
}