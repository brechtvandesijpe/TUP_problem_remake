use TUP_problem_remake::algorithm::branch_and_bound;
use std::time::Instant;

fn bench(
    file_name: &str,
    q1: i32,
    q2: i32,
    expected: i128
) {
    let start = Instant::now();
    let result = branch_and_bound(file_name, q1, q2);
    let duration = start.elapsed();
    let passed = match result == expected {
        true => "\x1b[32m[PASSED]\x1b[0m", // Green
        false => "\x1b[31m[FAILED]\x1b[0m", // Red
    };

    if expected == 999999999 {
        if result == 999999999 {
            println!("{:<9} {:<7} ({},{}): Expected {:<11} Actual {:<9} {:?}", passed, &file_name[..file_name.len().min(7)], q1, q2, "INFEASIBLE", format!("{:<9}", "INFEASIBLE"), duration);
        } else {
            println!("{:<9} {:<7} ({},{}): Expected {:<11} Actual {:<9} {:?}", passed, &file_name[..file_name.len().min(7)], q1, q2, "INFEASIBLE", format!("{:<9}", result), duration);
        }
    } else {
        if result == 999999999 {
            println!("{:<9} {:<7} ({},{}): Expected {:<11} Actual {:<9} {:?}", passed, &file_name[..file_name.len().min(7)], q1, q2, expected, format!("{:<9}", "INFEASIBLE"), duration);
        } else {
            println!("{:<9} {:<7} ({},{}): Expected {:<11} Actual {:<9} {:?}", passed, &file_name[..file_name.len().min(7)], q1, q2, expected, format!("{:<9}", result), duration);
        }
    }

}

fn main() {
    bench("umps4", 2, 1, 5176);

    bench("umps8", 4, 2, 34311);
    bench("umps8A", 4, 2, 31490);
    bench("umps8B", 4, 2, 32731);
    bench("umps8C", 4, 2, 29879);

    bench("umps10", 5, 2, 48942);
    bench("umps10A", 5, 2, 46551);
    bench("umps10B", 5, 2, 45609);
    bench("umps10C", 5, 2, 43149);
    
    bench("umps12", 7, 2, 86889);
    bench("umps12",6, 3, 999999999);
    bench("umps12", 5, 3, 93679);
    bench("umps12", 4, 3, 89826);
}