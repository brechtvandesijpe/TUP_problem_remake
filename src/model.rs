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
}

#[derive(Debug)]
pub struct Umpire {
    visit_history: Vec<i32>,
    officiated_history: Vec<(i32, i32)>,
    visited: Vec<bool>,
    officiated: Vec<bool>,
    current_round_index: i32,
    q1: i32,
    q2: i32,
}

impl Umpire {
    pub fn new(
        num_teams: i32,
        q1: i32,
        q2: i32,
    ) -> Self {
        let mut visited = Vec::new();
        let mut officiated = Vec::new();

        for i in 0..num_teams {
            visited.push(false);
            officiated.push(false);
        }

        Self {
            visit_history: Vec::new(),
            officiated_history: Vec::new(),
            visited,
            officiated,
            current_round_index: 0,
            q1,
            q2,
        }
    }

    pub fn assign_game(
        mut self,
        home_index: i32,
        out_index: i32,
    ) -> Result<Self, String> {
        if !self.is_allowed_visited(home_index) {
            return Err(format!("Q1 violation for home = {}, out = {}", home_index, out_index));
        }
        
        if !self.is_allowed_officiate(home_index, out_index) {
            return Err(format!("Q2 violation for home = {}, out = {}", home_index, out_index));
        }

        self.visit_history.push(home_index - 1);
        self.officiated_history.push((home_index - 1, out_index - 1));
        self.visited[(home_index - 1) as usize] = true;
        self.officiated[(home_index - 1) as usize] = true;
        self.officiated[(out_index - 1) as usize] = true;
        self.current_round_index += 1;
        Ok(self)
    }

    pub fn unassign_game(
        mut self,
    ) -> Result<Self, String> {
        self.current_round_index -= 1;

        let v = self.visit_history.pop().expect("Valid home_index") ;
        let start_visit = self.current_round_index;
        let mut end_visit = self.current_round_index - self.q1 + 1;
        if end_visit < 0 {
            end_visit = 0;
        }

        self.visited[v as usize] = false;
        for index in end_visit..start_visit {
            if self.visit_history[index as usize] == v{
                self.visited[index as usize] = true;
            }
        }

        let tuple = self.officiated_history.pop().expect("Valid home_index and out_index") else { todo!() };
        let start_officiated = self.current_round_index;
        let end_officiated = self.current_round_index - self.q2 + 1;

        self.officiated[tuple.0 as usize] = false;
        self.officiated[tuple.1 as usize] = false;
        for index in end_visit..start_visit {
            if self.officiated_history[index as usize].0 == tuple.0 {
                self.officiated[tuple.0 as usize] = false;
            }
            
            if self.officiated_history[index as usize].1 == tuple.1 {
                self.officiated[tuple.1 as usize] = false;
            }
        }

        Ok(self)
    }

    pub fn is_allowed_visited(
        &self,
        home_index: i32,
    ) -> bool {
        !self.visited[(home_index - 1) as usize]
    }

    pub fn is_allowed_officiate(
        &self,
        home_index: i32,
        out_index: i32,
    ) -> bool {
        !self.officiated[(home_index - 1) as usize] && !self.officiated[(out_index - 1) as usize]
    }
}