extern crate rand;
extern crate indicatif;

use self::rand::{thread_rng, Rng};
use self::indicatif::ProgressIterator;
use helper::print_vec;

use super::*;
use crate::individual::Individual;

pub struct Simulation {
    iterations: usize,

    crossover_probability: f64,
    mutation_probability: f64,
    population_size: usize, 

    number_of_points: usize,
    points: Vec<Point>,

    evaluations: usize,
    number_of_mutations: usize,
    number_of_crossovers: usize,

    pub champion: Individual
}

impl Simulation {
    pub fn new(iterations: usize,
               crossover_probability: f64,
               mutation_probability: f64,
               population_size: usize,
               points: Vec<Point>) -> Self {
                
        assert_eq!(population_size % 10, 0,
                   "population_size:{} should be divisible by 10", population_size);

        let number_of_points = points.len();
        let evaluations = 0;
        let number_of_mutations = 0;
        let number_of_crossovers = 0;
        let champion = Individual::new(&points); 

        Simulation { 
            iterations, 
            crossover_probability, 
            mutation_probability, 
            population_size, 
            number_of_points, 
            points,
            evaluations,
            number_of_mutations,
            number_of_crossovers,
            champion
        }
    }

    fn generate_children(&mut self, mut mom: Individual, dad: &mut Individual)
                                                   -> (Individual, Individual) {
        if thread_rng().gen_bool(self.crossover_probability) {
            // Can't cross over when depth == 1
            if mom.dna.depth() == 1 {
                mom.dna.random_instantiate(0, 2);
                mom.update_fitness(&self.points);
            }
            if dad.dna.depth() == 1 {
                dad.dna.random_instantiate(0, 2);
                dad.update_fitness(&self.points);
            }
            self.number_of_crossovers += 2;
            assert!(mom.dna.depth() > 1 && dad.dna.depth() > 1,
                    "\na:\n{}\nb:\n{}\n", mom.dna, dad.dna);
            mom.cross_over(dad, &self.points)
        } else {
            (mom, dad.clone())
        }
    }

    fn might_mutate_child(&mut self, child: &mut Individual) {
        if thread_rng().gen_bool(self.mutation_probability) {
            child.mutate(&self.points);
            self.number_of_mutations += 1;
        }
    }

    pub fn generate_population(&mut self, mut individuals: Vec<Individual>) -> Vec<Individual> {
        assert_eq!(self.population_size % 2, 0,
                   "population_size:{} should be divisible by 2", self.population_size);
        
        let mut cumulative_weights = get_cumulative_weights(&individuals);
        let mut next_population = Vec::new();

        for _ in 0..(self.population_size / 2 ) { // generate two individuals per iteration
            let (mom_index, dad_index) = select_parents(&mut cumulative_weights);
            let mom: Individual = individuals[mom_index].clone();
            let dad: &mut Individual = &mut individuals[dad_index];
            let (mut daughter, mut son) = self.generate_children(mom, dad);
            self.might_mutate_child(&mut daughter);
            self.might_mutate_child(&mut son);

            next_population.push(daughter);
            next_population.push(son);
        }
        next_population
    }

    /// Increments self.evaluations by the sum of individual.fitness in population
    fn update_evaluations(&mut self, population: &[Individual]) {
        let mut population_evals: Vec<usize> = vec![0; self.population_size];
        for (i, individual) in population.iter().enumerate() {
            population_evals[i] = individual.evaluations;
        }
        self.evaluations += population_evals.iter().sum::<usize>();
    }

    pub fn run(&mut self, debug_level: usize, skip: usize) {
        assert!(skip > 0, "skip must be 1 or larger");
        let mut population = random_population(self.population_size, &self.points);
        let mut champion = find_fittest(&population);
        for i in (0..self.iterations).progress() {
            population = self.generate_population(population);
            let challenger = find_fittest(&population);
            if (i + 1) % skip == 0 {
                debug_print(debug_level, i + 1, self.evaluations, &population,
                            &champion, &challenger);
            }
            if champion.fitness <= challenger.fitness {
                champion = challenger;
            }
        }
        self.update_evaluations(&population);
        self.champion = champion;
        
        let x = self.population_size * self.iterations;
        println!("\n---------------\nSPECS\n---------------");
        println!("iterations: {:?}", self.iterations);
        println!("crossover_probability: {:?}", self.crossover_probability);
        println!("mutation_probability: {:?}", self.mutation_probability);
        println!("population_size: {:?}", self.population_size);
        println!("number_of_points: {:?}", self.number_of_points);
        println!("\n---------------\nSTATS\n ---------------");
        println!("Champion:\n{}", self.champion.dna);
        println!("Fitness Score: {}", self.champion.fitness);
        println!("Total Evaluations: {}", self.evaluations);
        println!("{} mutations out of {} individuals produced", self.number_of_mutations, x);
        println!("{} cross-overs out of {} individuals produced", self.number_of_crossovers, x);
        println!("\n---------------\nEND\n---------------\n");
    }
}

fn debug_print(debug_level: usize, epoch: usize,
               evaluations: usize, population: &[Individual],
               champion: &Individual, challenger: &Individual) {
    if debug_level == 1 {
        println!("{}, {}, {}, {}", epoch, evaluations, champion.fitness, challenger.fitness);
    } else if debug_level >= 2 {
        println!("\n\nepoch {}\nevaluations: {}\nchampion fitness: {}\nchallenger fitness: {}",
                    epoch, evaluations, champion.fitness, challenger.fitness);
        println!("champion:\n{}\nchallenger:\n{}", champion.dna, challenger.dna);
        if debug_level == 3 {
            println!("\n\n---------------\nepoch {} population\n---------------", epoch);
            print_vec(population);
        } 
    }
}
