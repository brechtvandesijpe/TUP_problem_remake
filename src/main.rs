mod model;
use model::*;

mod data;
use data::*;

const Q1: i32 = 1;
const Q2: i32 = 0;

let mut branch_states = Vec::new();

fn main() {
    // READ IN DATA
    let data = read_data("resources/umps8.txt").unwrap();
    let model = Model::new(&data);

    let num_teams = &data.n_teams;
    let num_umpires = num_teams / 2;

    let mut umpires = Vec::new();
    for i in 0..num_umpires {
        umpires.push(Umpire::new(*num_teams, Q1, Q2));
    }

    println!("{:?}", umpires);
}