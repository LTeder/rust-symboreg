extern crate rand;

use self::rand::{thread_rng, Rng};
use std::fmt::Debug;
use std::process;
use std::fs::File;
use std::io::prelude::*;

pub fn print_vec<T: Debug>(v: &[T]) {
    for i in v.iter() { println!("{:?}", i); }   
}

pub fn select_index(cumulative_weights: &[f32]) -> usize {
    // To do: Error Handling
    let last = cumulative_weights.last();
    //let max = cumulative_weights.iter().reduce(f32::max).max();
    let w_sum = last.unwrap().min(f32::MAX);
    let r: f32 = thread_rng().gen_range(0.0, w_sum);
    cumulative_weights.iter().rposition(|&w| w < r).unwrap_or({
        thread_rng().gen_range(0, cumulative_weights.len())
    })
}

pub fn read_file(filename: &String) -> String {
    let mut file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("Problem opening file. {:?}\n error: {}\n ", filename, err);
        process::exit(1)
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|err| { 
        eprintln!("Problem reading file.\n error: {}", err); 
        process::exit(1)
    });
    contents
}

pub fn parse_specs(contents: &str) -> Result<(usize, usize, usize, usize, f64, f64), String> { 
    // To do: Expected number of arguments + Expected type and range of arguments
    let v: Vec<String> = contents.split(',')
                               .map(|val| val.trim().to_string())
                               .collect();
    if v.len() != 6 {
        return Err("Unexpected number of specs (must be exactly 6)".to_string());
    }

    let debug_level: usize = v[0].parse().map_err(|err| {
        format!("debug_level = {} can't be parsed as integer.\nerror: {}\n", v[0], err)
    })?;

    let skip: usize = v[1].parse().map_err(|err| {
        format!("skip = {} can't be parsed as integer.\nerror: {}\n", v[1], err)
    })?;

    let iterations: usize = v[2].parse().map_err(|err| {
        format!("iterations = {} can't be parsed as integer.\nerror: {}\n", v[2], err)
    })?;

    let population_size: usize = v[3].parse().map_err(|err| {
        format!("population_size = {} can't be parsed as integer.\nerror: {}\n", v[3], err)
    })?;

    let crossover_probability: f64 = v[4].parse().map_err(|err| {
        format!("crossover_probability = {} can't be parsed as a float.\nerror: {}\n", v[4], err)
    })?;

    let mutation_probability: f64 = v[5].parse().map_err(|err| {
        format!("mutation_probability = {} can't be parsed as a float.\nerror: {}\n", v[5], err)
    })?;

    Ok((
        debug_level,
        skip,
        iterations,
        population_size,
        crossover_probability,
        mutation_probability
    ))
}
