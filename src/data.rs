use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

#[derive(Debug)]
pub struct Data {
    pub n_teams: i32,
    pub dist: Vec<Vec<i128>>,
    pub opponents: Vec<Vec<i32>>,
}

pub fn read_data(file_path: &str) -> io::Result<Data> {
    let content = std::fs::read_to_string(file_path)?;
    let mut re = Regex::new(r"nTeams=(\d+);").unwrap();
    let mut caps = re.captures(&content).unwrap();
    let n_teams: i32 = caps.get(1).unwrap().as_str().parse().unwrap();

    re = Regex::new(r"dist=\s*\[\s*((?:\[\s*(?:\d+\s*)+\]\s*)+)\]").unwrap();
    let caps = re.captures(&content).unwrap();
    let dist_block = caps.get(1).unwrap().as_str();
    let mut dist: Vec<Vec<i128>> = dist_block
        .lines()
        .map(|line| {
            line.trim_matches(|c| c == '[' || c == ']')
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    
    dist.pop(); // Remove the last row

    re = Regex::new(r"opponents=\s*\[\s*((?:\[\s*(?:-?\d+\s*)+\]\s*)+)\]").unwrap();
    let caps = re.captures(&content).unwrap();
    let opponents_block = caps.get(1).unwrap().as_str();
    let mut opponents: Vec<Vec<i32>> = opponents_block
        .lines()
        .map(|line| {
            line.trim_matches(|c| c == '[' || c == ']')
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
        opponents.pop();

    Ok(Data {
        n_teams,
        dist,
        opponents,
    })
}