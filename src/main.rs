// use TUP_problem_remake::algorithm::branch_and_bound;

use TUP_problem_remake::algorithm::branch_and_bound;

const Q1: i32 = 5;
const Q2: i32 = 2;

const FILE_NAME: &str = "umps10B";

fn main() {
    println!("result for {}_{}_{} = {}", FILE_NAME, Q1, Q2, branch_and_bound(FILE_NAME, Q1, Q2));
}   