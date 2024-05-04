use itertools::Itertools; 
use std::fs::File;
use std::io::prelude::*;

fn permutate(vec: &mut Vec<(i32, i32)>, start: usize, result: &mut Vec<Vec<(i32, i32)>>) {
    if start >= vec.len() {
        result.push(vec.clone());
    } else {
        for i in start..vec.len() {
            vec.swap(start, i);
            permutate(vec, start + 1, result);
            vec.swap(start, i);
        }
    }
}

#[derive(Clone)]
pub struct Node<'a> {
    parent: Option<Box<Node<'a>>>,
    new_assignments: Vec<(i32, i32)>,
    pub score: i128,
    pub round_index: i32,
    dist: &'a Vec<Vec<i128>>,
    visited_teams: Vec<Vec<bool>>,
}

// impl<'a> std::fmt::Debug for Node<'a> {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>
//     ) -> std::fmt::Result {
//         if let Some(parent) = &self.parent {
//             write!(f, r#"{}
// {:?}"#, parent, self.new_assignments)
//         } else {
//             for i in 0..self.new_assignments.len() {
//                 let tuple = &self.new_assignments[i];
//                 write!(f, "{:?}", tuple.0);
//                 if i != self.new_assignments.len() - 1 {
//                     write!(f, " ");
//                 }
//             }
//             write!(f, "{:?}")
//         }
//     }
// }

impl<'a> std::fmt::Debug for Node<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, r#"{:?}
{:?} {}"#, parent, self.new_assignments, self.score)
        } else {
            write!(f, "{:?} {:?}", self.new_assignments, self.score)
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
{:?}"#, parent, self.new_assignments)
        } else {
            write!(f, "{:?}", self.new_assignments)
        }
    }
}

impl<'a> Node<'a> {
    pub fn new(
        parent: Option<Box<Node<'a>>>,
        new_assignments: Vec<(i32, i32)>,
        dist: &'a Vec<Vec<i128>>,
    ) -> Self {
        let mut visited_teams: Vec<Vec<bool>> = vec![vec![false; new_assignments.len() * 2]; new_assignments.len()];
        for i in 0..new_assignments.len() {
            let assignment = &new_assignments[i];
            visited_teams[i as usize][(assignment.0 - 1) as usize] = true;
        }
        let mut round_index = 1;
        let mut score: i128 = 0;
        if let Some(parent) = &parent {
            round_index += parent.round_index;
            score += parent.score;
            for i in 0..new_assignments.len() {
                let new_assignment = &new_assignments[i];
                let previous_assignment = parent.new_assignments[i];
                let from = previous_assignment.0 - 1;
                let to = new_assignment.0 - 1;
                score += dist[from as usize][to as usize];
            }

            for i in 0..visited_teams.len() {
                for j in 0..visited_teams[i].len() {
                    if parent.visited_teams[i][j] == true {
                        visited_teams[i][j] = true;
                    }
                }
            }
        }

        Self {
            parent: parent,
            new_assignments,
            score,
            round_index,
            dist,
            visited_teams,
        }
    }

    pub fn pre_evaluate(
        &self,
        assignments: &Vec<(i32, i32)>,
        upperbound: i128,
    ) -> bool {
        let previous_locations: Vec<i32> = self.get_current_locations();
        let mut score: i128 = self.score;

        for i in 0..previous_locations.len() {
            let from: i32 = previous_locations[i] - 1;
            let to: i32 = assignments[i].0 - 1;
            score += self.dist[from as usize][to as usize];
        }

        score < upperbound
    }

    pub fn check_global(
        &self,
        num_rounds_left: i32,
    ) -> bool {
        let mut counter: Vec<i32> = vec![0; self.visited_teams.len()];

        for i in 0..self.visited_teams.len() {
            for elem_inner in &self.visited_teams[i as usize] {
                if !elem_inner {
                    counter[i] += 1;
                }
            }
        }

        *counter.iter().max().unwrap() <= num_rounds_left
    }

    pub fn check_global_mutations(
        &self,
        num_rounds_left: i32,
        mutations: &Vec<(i32, i32)>,
    ) -> bool {
        let mut counter: Vec<i32> = vec![0; self.visited_teams.len()];
        let mut new_visited: Vec<Vec<bool>> = self.visited_teams.clone();

        for (i, mutation) in mutations.iter().enumerate() {
            new_visited[i][(mutation.0 - 1) as usize] = true;
        }

        for i in 0..new_visited.len() {
            for elem_inner in &new_visited[i as usize] {
                if !elem_inner {
                    counter[i] += 1;
                }
            }
        }

        *counter.iter().max().unwrap() <= num_rounds_left
    }

    pub fn generate_children(
        &self,
        q1: i32,
        q2: i32,
        mut options: Vec<(i32, i32)>,
        upperbound: i128,
        num_rounds: i32,
    ) -> Vec<Vec<(i32, i32)>> {
        let mut result = Vec::new();
        if !self.check_global(num_rounds - self.round_index - 1) {
            return result;
        }

        let num_checks_q1 = q1 - 1;
        let stop_round_q1 = self.round_index - num_checks_q1;

        if let Some(parent) = &self.parent {
            if !parent.check_q1(num_checks_q1, &self.new_assignments) {
                return result;
            }
        }

        let num_checks_q2 = q1 - 1;
        let stop_round_q2 = self.round_index - num_checks_q2;

        if let Some(parent) = &self.parent {
            if !parent.check_q1(num_checks_q1, &self.new_assignments) {
                return result;
            }
        }
        
        if !self.check_global(num_rounds - self.round_index - 1) {
            return result;
        }

        permutate(&mut options, 0, &mut result);
        result.into_iter()
            .filter(|perm| {
                let is_global = self.check_global_mutations(num_rounds - self.round_index, perm);
                if !is_global {
                    return false;
                }

                let is_q1 = self.check_q1(stop_round_q1, perm);
                if !is_q1 {
                    return false;
                }

                let is_q2 = self.check_q2(stop_round_q2, perm);
                if !is_q2 {
                    return false;
                }

                let is_pre_evaluated = self.pre_evaluate(perm, upperbound);
                if !is_pre_evaluated {
                    return false;
                }

                true
            })
            .collect::<Vec<_>>()
    }

    pub fn get_current_locations(
        &self,
    ) -> Vec<i32> {
        self.new_assignments.iter().map(|(from, _)| *from).collect()
    }

    pub fn check_q1(
        &self,
        stop_round: i32,
        perm: &Vec<(i32, i32)>
    ) -> bool {
        let mut result = true;

        if self.round_index != 1 && stop_round < self.round_index {
            if let Some(parent) = &self.parent {
                result = parent.check_q1(stop_round, perm);
            } else {};
        }
        
        let is_visited = self.is_visited(perm);
        result && !is_visited
    }

    pub fn check_q2(
        &self,
        stop_round: i32,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        let mut result = true;
        
        if self.round_index != 1 && stop_round < self.round_index {
            if let Some(parent) = &self.parent {
                result = parent.check_q2(stop_round, assignments);
            }
        }
        
        let is_officiated = self.is_officiated(assignments);
        result && !is_officiated
    }

    pub fn is_visited(
        &self,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        for i in 0..assignments.len() {
            let assignment = assignments[i];
            let new_assignment = self.new_assignments[i];
            if assignment.0 == new_assignment.0 {
                return true;
            }
        }

        false
    }

    pub fn is_officiated(
        &self,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        for i in 0..assignments.len() {
            let assignment = assignments[i];
            let new_assignment = self.new_assignments[i];
            
            if assignment.0 == new_assignment.0 || assignment.0 == new_assignment.1 || 
               assignment.1 == new_assignment.0 || assignment.1 == new_assignment.1 {
                return true;
            }
        }

        false
    }

    pub fn export_vec(
        &self,
    ) -> Vec<Vec<i32>> {
        let mut result;
        
        if let Some(parent) = &self.parent {
            result = parent.export_vec();
        } else {
            result = Vec::new();
            for i in 0..self.new_assignments.len() {
                result.push(Vec::new());
            }
        }

        for i in 0..self.new_assignments.len() {
            result[i].push(self.new_assignments[i].0)
        }

        result
    }

    pub fn export(
        &self,
        name: &str,
    ) {
        let result = self.export_vec();
        let _  =File::create(format!("solution_{}.txt", name))
                            .expect("Could not create file")
                            .write_all(format!("{}", self)
                            .as_bytes());
    }

    pub fn export_pdf(
        &self,
        name: &str,
    ) {
        let result = self.export_vec();
        let mut file = File::create(format!("solution_{}.txt", name)).expect("Could not create file");
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                let elem = &result[i][j];
                file.write_all(format!("{}", elem).as_bytes()).expect("Could not write to file");
                if i != result.len() - 1 || j != result[i].len() - 1 {
                    file.write_all(b",").expect("Could not write to file");
                }
            }
        }
        let _ = file.write_all(format!("{}", self).as_bytes());
    }
}