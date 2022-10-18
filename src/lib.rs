use std::str::FromStr;

pub mod examples;
pub mod helper;
mod individual;
mod simulation;

pub use individual::Individual;
pub use simulation::Simulation;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {x, y}
    }
}

pub fn string_to_points(contents: &String) -> Vec<Point> {
    // To do: Error handling: Unwrapping of line + expected # elements 
    let mut points: Vec<Point> = Vec::new();

    for line in contents.lines() {
        let values: Vec<f64> = line.split(',')
                                   .map(|val| f64::from_str(val.trim())
                                   .unwrap())
                                   .collect();
        
        let c = Point::new(values[1], values[2]);
        points.push(c);
    }
    points
}

pub fn select_parents<'a>(w: &[f32], individuals: &'a [Individual]) -> (&'a Individual, &'a Individual) {
    let mom_index = helper::select_index(w);
    let dad_index = helper::select_index(w);  
    (&individuals[mom_index], &individuals[dad_index])
}

// max_by_key: Ord not implemented for f64
// population.iter().max_by_key(|i| i.fitness).unwrap().clone()
pub fn find_fittest(population: &[Individual]) -> Individual {
    let mut best_individual = &population[0];
    
    for individual in population {
        if best_individual.fitness > individual.fitness {
            best_individual = individual;
        }
    }
    best_individual.clone()
}

pub fn get_cumulative_weights(individuals: &[Individual]) -> Vec<f32> {
    let mut running_sum = 0.0;
    let mut cumulative_weights = vec![running_sum];

    for i in individuals {
        running_sum += i.fitness;
        cumulative_weights.push(running_sum);
    }
    cumulative_weights
}

pub fn random_population(population_size: usize, points: &[Point]) -> Vec<Individual> {
    let number_of_points = points.len();
    let mut individuals:Vec<Individual> = Vec::new();
    
    for _ in 0..population_size {
        let mut indiv = Individual::new(points);
        indiv.spawn(2);
        individuals.push(indiv);
    } 
    individuals
}
