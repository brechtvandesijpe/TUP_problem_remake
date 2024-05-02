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

#[derive(Clone, Debug)]
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
        let round_index = parent.clone().map(|n| n.round_index).unwrap_or(0) + 1;
        let mut visited_teams = parent.clone().map(|n| n.visited_teams).unwrap_or(vec![vec![false; new_assignments.len() * 2]; new_assignments.len()]);
        for i in 0..new_assignments.len() {
            let assignment = &new_assignments[i];
            visited_teams[i as usize][(assignment.0 - 1) as usize] = true;
        }

        // println!("round_index = {:?}, new_assignments = {:?}, visited_teams = {:?}", round_index, new_assignments, visited_teams);

        Self {
            parent: parent.clone(),
            new_assignments,
            score: parent.clone().map(|n| n.score).unwrap_or(0),
            round_index,
            dist,
            visited_teams,
        }

        // if round_index == 14 {
        //     me.export_string();
        //     println!("new_assignments = {:?}, visited_teams = {:?}", new_assignments, visited_teams);
        // }
    }

    pub fn evaluate(
        mut self,
    ) -> Self {
        if let Some(ref parent) = self.parent {
            let previous_locations: Vec<i32> = parent.get_current_locations();
            for i in 0..previous_locations.len() {
                let from: i32 = previous_locations[i] - 1;
                let to: i32 = self.new_assignments[i].0 - 1;
                self.score += self.dist[from as usize][to as usize];
            }
        }

        self
    }

    pub fn pre_evaluate(
        &self,
        assignments: &Vec<(i32, i32)>,
    ) -> i128 {
        let previous_locations: Vec<i32> = self.get_current_locations();
        let mut score: i128 = 0;

        for i in 0..previous_locations.len() {
            let from: i32 = previous_locations[i] - 1;
            let to: i32 = assignments[i].0 - 1;
            score += self.dist[from as usize][to as usize];
        }

        score
    }

    pub fn check_global(
        &self,
        num_rounds_left: i32,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        let mut counter: Vec<i32> = vec![0; self.visited_teams.len()];

        for i in 0..self.visited_teams.len() {
            for elem_inner in &self.visited_teams[i as usize] {
                if !elem_inner {
                    counter[i] += 1;
                }
            }
        }

        for i in 0..self.visited_teams.len() {
            let assignment_umpire = assignments[i];
            for (index, elem_inner) in self.visited_teams[i as usize].iter().enumerate() {
                if !elem_inner && assignment_umpire.0 as usize != index {
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
    ) -> (Vec<i128>, Vec<Vec<(i32, i32)>>) {
        let mut extra_scores = Vec::new();
        let mut result = Vec::new();
        // if !self.check_global(num_rounds - self.round_index, None) {
        //     return (extra_scores, result);
        // }

        permutate(&mut options, 0, &mut result);
        result = result.into_iter()
            .filter(|perm| {
                if !self.check_global(num_rounds - self.round_index, perm) {
                    return false;
                }

                let is_q1 = self.check_q1(q1, perm);
                if !is_q1 {
                    return false;
                }

                let is_q2 = self.check_q2(q2, perm);
                if !is_q2 {
                    return false;
                }

                let extra_score = self.pre_evaluate(perm);
                let is_pre_evaluated = extra_score < upperbound;

                if !is_pre_evaluated {
                    return false;
                }

                extra_scores.push(extra_score);
                true
            })
            .collect::<Vec<_>>();
        
        (extra_scores, result)
    }

    pub fn get_current_locations(
        &self,
    ) -> Vec<i32> {
        self.new_assignments.iter().map(|(from, _)| *from).collect()
    }

    pub fn check_q1(
        &self,
        q1: i32,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        let mut result = true;
        if q1 > 1 {
            if let Some(parent) = &self.parent {
                result = parent.check_q1(q1 - 1, assignments);
            }
        }
        
        // println!("q1 = {:?}, round = {:?}, new_assignments = {:?}, assignments = {:?}", q1, self.round_index , self.new_assignments, assignments);
        let is_visited = self.is_visited(assignments);
        result = result && !is_visited;
        // println!("result = {:?}, is_visited = {:?}", result, is_visited);
        result
    }

    pub fn check_q2(
        &self,
        q2: i32,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        let mut result = true;
        
        if q2 > 1 {
            if let Some(parent) = &self.parent {
                result = parent.check_q2(q2 - 1, assignments);
            }
        }
        
        // println!("q2 = {:?}, round = {:?}, new_assignments = {:?}, assignments = {:?}", q2, self.round_index , self.new_assignments, assignments);
        let is_officiated = self.is_officiated(assignments);
        result = result && !is_officiated;
        // println!("result = {:?}, is_visited = {:?}", result, is_officiated);
        result
    }

    pub fn is_visited(
        &self,
        assignments: &Vec<(i32, i32)>
    ) -> bool {
        for i in 0..assignments.len() {
            let assignment = assignments[i];
            let new_assignment = self.new_assignments[i];
            if assignment.0 == new_assignment.0 {
                // println!("Visited equals on assignment = {:?}, new_assignment = {:?}", assignment, new_assignment);
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
            if assignment.0 == new_assignment.0 || assignment.1 == new_assignment.1 {
                // println!("Visited equals on assignment = {:?}, new_assignment = {:?}", assignment, new_assignment);
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
            // println!("{:?} = {:?}", self.new_assignments, self.score);
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
        let mut result = self.export_vec();
        let mut file = File::create(format!("solution_{}.txt", name)).expect("Could not create file");
        // for i in 0..result.len() {
        //     for j in 0..result[i].len() {
        //         let elem = &result[i][j];
        //         file.write_all(format!("{}", elem).as_bytes()).expect("Could not write to file");
        //         if i != result.len() - 1 || j != result[i].len() - 1 {
        //             file.write_all(b",").expect("Could not write to file");
        //         }
        //     }
        // }
        let _ = file.write_all(format!("{}", self).as_bytes());
    }
}