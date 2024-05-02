// use std::fs::File;
// use std::io::{self, BufRead, BufReader};
// use regex::Regex;

// #[derive(Debug)]
// pub struct Data {
//     pub n_teams: i32,
//     pub dist: Vec<Vec<i128>>,
//     pub opponents: Vec<Vec<i32>>,
// }

// pub fn read_data(file_path: &str) -> io::Result<Data> {
//     let content = std::fs::read_to_string(file_path)?;
//     let mut re = Regex::new(r"nTeams=(\d+);").unwrap();
//     let mut caps = re.captures(&content).unwrap();
//     let n_teams: i32 = caps.get(1).unwrap().as_str().parse().unwrap();

//     re = Regex::new(r"dist=\s*\[\s*((?:\[\s*(?:\d+\s*)+\]\s*)+)\]").unwrap();
//     let caps = re.captures(&content).unwrap();
//     let dist_block = caps.get(1).unwrap().as_str();
//     let mut dist: Vec<Vec<i128>> = dist_block
//         .lines()
//         .map(|line| {
//             line.trim_matches(|c| c == '[' || c == ']')
//                 .split_whitespace()
//                 .map(|num| num.parse().unwrap())
//                 .collect()
//         })
//         .collect();
    
//     dist.pop(); // Remove the last row

//     re = Regex::new(r"opponents=\s*\[\s*((?:\[\s*(?:-?\d+\s*)+\]\s*)+)\]").unwrap();
//     let caps = re.captures(&content).unwrap();
//     let opponents_block = caps.get(1).unwrap().as_str();
//     let mut opponents: Vec<Vec<i32>> = opponents_block
//         .lines()
//         .map(|line| {
//             line.trim_matches(|c| c == '[' || c == ']')
//                 .split_whitespace()
//                 .map(|num| num.parse().unwrap())
//                 .collect()
//         })
//         .collect();
//         opponents.pop();

//     Ok(Data {
//         n_teams,
//         dist,
//         opponents,
//     })
// }

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
pub struct Data {
    pub num_teams: i32,
    pub dist: Vec<Vec<i128>>,
    pub opponents: Vec<Vec<i32>>,
}

pub fn read_data(file_path: &str) -> io::Result<Data> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut num_teams = 0;
    let mut dist = Vec::new();
    let mut opponents = Vec::new();

    while let Some(line) = lines.next() {
        let line = line?;
        if line.contains("nTeams") {
            num_teams = line.split("=").nth(1).unwrap().split(";").nth(0).unwrap().trim().parse().unwrap();
        }
        if line.contains("dist") {
            dist = read_array_i128(&mut lines, num_teams as usize)?;
        }
        if line.contains("opponents") {
            opponents = read_array_i32(&mut lines, 2 * num_teams as usize - 2)?;
        }
    }

    Ok(Data {
        num_teams,
        dist,
        opponents,
    })
}

fn read_array_i128(
    lines: &mut std::io::Lines<BufReader<File>>,
    rows: usize
) -> io::Result<Vec<Vec<i128>>> {
    let mut array = Vec::new();

    for _ in 0..rows {
        let line = lines.next().unwrap()?;
        let row: Vec<i128> = line.split(|c: char| c == '[' || c == ']' || c.is_whitespace())
            .filter(|part| !part.is_empty())
            .map(|part| i128::from_str(part).unwrap())
            .collect();
        array.push(row);
    }

    Ok(array)
}

fn read_array_i32(
    lines: &mut std::io::Lines<BufReader<File>>,
    rows: usize
) -> io::Result<Vec<Vec<i32>>> {
    let mut array = Vec::new();

    for _ in 0..rows {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split(|c: char| c == '[' || c == ']' || c.is_whitespace())
            .filter(|part| !part.is_empty())
            .map(|part| i32::from_str(part).unwrap())
            .collect();
        array.push(row);
    }

    Ok(array)
}