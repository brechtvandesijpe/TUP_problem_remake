mod model;
use model::*;

mod data;
use data::*;

fn main() {
    let data = read_data("resources/umps8.txt").unwrap();
    let model = Model::new(data);
    println!("{:?}", model);
}