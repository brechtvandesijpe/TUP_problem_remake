use itertools::Itertools; 

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

#[derive(Debug, Clone)]
pub struct Node<'a> {
    parent: Option<Box<Node<'a>>>,
    new_assignments: Vec<(i32, i32)>,
    pub score: i128,
    pub round_index: i32,
    dist: &'a Vec<Vec<i128>>,
    visited_teams: Vec<Vec<bool>>,
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
        upperbound: i128,
    ) -> bool {
        let previous_locations: Vec<i32> = self.get_current_locations();
        let mut score: i128 = 0;

        for i in 0..previous_locations.len() {
            let from: i32 = previous_locations[i] - 1;
            let to: i32 = assignments[i].0 - 1;
            score += self.dist[from as usize][to as usize];
        }

        score < upperbound
    }

    pub fn is_global_feasible(
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

    pub fn generate_children(
        &self,
        q1: i32,
        q2: i32,
        mut options: Vec<(i32, i32)>,
        upperbound: i128,
        num_rounds: i32,
    ) -> Vec<Vec<(i32, i32)>> {
        let mut result = Vec::new();
        if !self.is_global_feasible(num_rounds - self.round_index) {
            return result;
        }

        permutate(&mut options, 0, &mut result);
        result.into_iter()
              .filter(|perm| {
                  let is_q1 = self.check_q1(q1, perm);
                  let is_q2 = self.check_q2(q2, perm);
                  let is_pre_evaluated = self.pre_evaluate(perm, upperbound);
                  // println!("Permutation {:?}, is_q1 = {:?}, is_q2 = {:?}, is_pre_evaluated = {:?}", perm, is_q1, is_q2, is_pre_evaluated);
                  is_q1 && is_q2 && is_pre_evaluated
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

    pub fn export_string(
        &self,
    ) -> String {
        let vec = self.export_vec();
        let mut result = String::new();

        for row in vec {
            for i in row.len()..0 {
                result += &row[i].to_string();
                if i != row.len() - 1 {
                    result += " ";
                }
            }
            result += "\n";
        }

        result
    }
    
    pub fn export_vec(
        &self,
    ) -> Vec<Vec<i32>> {
        let mut result;
        
        if let Some(parent) = &self.parent {
            result = parent.export_vec();
            println!("{:?} = {:?}", self.new_assignments, self.score);
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
}