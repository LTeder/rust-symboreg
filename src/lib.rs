use std::str::FromStr;

pub mod helper;
mod sbh;
mod individual;
mod simulation;

pub use sbh::{SymbolicBinaryHeap, Node, MAX_IDX};
pub use individual::Individual;
pub use simulation::Simulation;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {x, y}
    }
}

pub fn string_to_points(contents: &String) -> Vec<Point> {
    // To do: Error handling: Unwrapping of line + expected # elements 
    let mut points: Vec<Point> = Vec::new();
    for line in contents.lines() {
        let values: Vec<f32> = line.split(',')
                                   .map(|val| f32::from_str(val.trim())
                                   .unwrap())
                                   .collect();
        
        let c = Point::new(values[0], values[1]);
        points.push(c);
    }
    points
}

pub fn select_parents(w: &[f32]) -> (usize, usize) {
    let mom_index = helper::select_index(w);
    let dad_index = helper::select_index(w);  
    (mom_index, dad_index)
}

pub fn find_fittest(population: &[Individual]) -> Individual {
    let mut best_individual = &population[0];
    for individual in population {
        if best_individual.fitness < individual.fitness {
            best_individual = individual;
        }
    }
    best_individual.clone()
}

pub fn get_cumulative_weights(individuals: &[Individual]) -> Vec<f32> {
    let mut running_sum = f32::MIN_POSITIVE;
    let mut cumulative_weights: Vec<f32> = vec![0.0; individuals.len()];
    for (i, individual) in individuals.iter().enumerate() {
        running_sum += individual.fitness;
        cumulative_weights[i] += running_sum;
    }
    cumulative_weights
}

pub fn random_population(population_size: usize, points: &[Point]) -> Vec<Individual> {
    let mut individuals: Vec<Individual> = Vec::new();
    for _ in 0..population_size {
        let indiv = Individual::new(points);
        individuals.push(indiv);
    } 
    individuals
}
