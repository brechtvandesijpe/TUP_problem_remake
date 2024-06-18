use TUP_problem_remake::algorithm::branch_and_bound;
use std::time::Instant;

fn bench(
    file_name: &str,
    q1: i32,
    q2: i32,
    expected: u64
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
            println!("{:<9} {:<7} ({},{}): Expected {:<11} Actual {:<11} {:?}", passed, &file_name[..file_name.len().min(7)], q1, q2, "INFEASIBLE", format!("{:<9}", "INFEASIBLE"), duration);
        } else {
            println!("{:<9} {:<7} ({},{}): Expected {:<11} Actual {:<11} {:?}", passed, &file_name[..file_name.len().min(7)], q1, q2, "INFEASIBLE", format!("{:<9}", result), duration);
        }
    } else {
        if result == 999999999 {
            println!("{:<9} {:<7} ({},{}): Expected {:<11} Actual {:<11} {:?}", passed, &file_name[..file_name.len().min(7)], q1, q2, expected, format!("{:<9}", "INFEASIBLE"), duration);
        } else {
            println!("{:<9} {:<7} ({},{}): Expected {:<11} Actual {:<11} {:?}", passed, &file_name[..file_name.len().min(7)], q1, q2, expected, format!("{:<9}", result), duration);
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
    
    bench("umps14", 8, 3, 172177);
    bench("umps14", 8, 2, 147824);
    bench("umps14", 7, 3, 164440);
    bench("umps14", 7, 2, 146656);
    bench("umps14", 6, 3, 158875);
    bench("umps14", 6, 2, 145124);
    bench("umps14", 5, 3, 154962);
    bench("umps14", 5, 2, 143357);
    
    // bench("umps14A", 8, 3, 166184);
    // bench("umps14A", 8, 2, 143043);
    // bench("umps14A", 7, 3, 158760);
    // bench("umps14A", 7, 2, 140562);
    // bench("umps14A", 6, 3, 152981);
    // bench("umps14A", 6, 2, 138927);
    // bench("umps14A", 5, 3, 149331);
    // bench("umps14A", 5, 2, 137853);
    
    // bench("umps14B", 8, 3, 165026);
    // bench("umps14B", 8, 2, 141312);
    // bench("umps14B", 7, 3, 157884);
    // bench("umps14B", 7, 2, 138998);
    // bench("umps14B", 6, 3, 152740);
    // bench("umps14B", 6, 2, 138241);
    // bench("umps14B", 5, 3, 149455);
    // bench("umps14B", 5, 2, 136069);
    
    // bench("umps14C", 8, 3, 161262);
    // bench("umps14C", 8, 2, 141015);
    // bench("umps14C", 7, 3, 154913);
    // bench("umps14C", 7, 2, 138832);
    // bench("umps14C", 6, 3, 150858);
    // bench("umps14C", 6, 2, 136394);
    // bench("umps14C", 5, 3, 148349);
    // bench("umps14C", 5, 2, 134916);
}