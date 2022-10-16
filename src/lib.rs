pub mod examples; 
pub mod dataset;
pub mod helper;
mod individual;
mod simulation;

pub use dataset::Point;
pub use individual::Individual;
pub use simulation::Simulation;

pub fn select_parents<'a>(w: &[f64], individuals: &'a [Individual]) -> (&'a Individual, &'a Individual) {
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

pub fn get_cumulative_weights(individuals: &[Individual]) -> Vec<f64> {
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
        let indiv = Individual::new(points);
        indiv.spawn(2);
        individuals.push(indiv);
    } 
    individuals
}
