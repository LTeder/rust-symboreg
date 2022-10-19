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

        assert_eq!(population_size % 10, 0, "population_size:{} should be divisible by 10", population_size);

        let number_of_points = points.len();
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
            number_of_mutations,
            number_of_crossovers,
            champion
        }
    }

    fn generate_children(&mut self, mom: Individual, dad: &mut Individual) -> (Individual, Individual) {
        if thread_rng().gen_bool(self.crossover_probability) {
            self.number_of_crossovers += 2;
            mom.cross_over(dad, &self.points)
        } else {
            (mom.clone(), dad.clone())
        }
    }

    fn might_mutate_child(&mut self, child: &mut Individual) {
        if thread_rng().gen_bool(self.mutation_probability) {
            child.mutate(&self.points);
            self.number_of_mutations += 1;
        }
    }

    pub fn generate_population(&mut self, mut individuals: Vec<Individual>) -> Vec<Individual> {
        assert_eq!(self.population_size % 2, 0, "population_size:{} should be divisible by 2", self.population_size);
        
        let mut cumulative_weights = get_cumulative_weights(&individuals);
        let mut next_population = Vec::new();

        for _ in 0..(self.population_size / 2 ) { // generate two individuals per iteration 

            let (mom_index, dad_index) =
                select_parents(&mut cumulative_weights);
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

    pub fn run(&mut self, debug_level: usize, skip: usize) {
        assert!(skip > 0, "skip must be 1 or larger");

        let mut population = random_population(self.population_size, &self.points);
        let mut champion = find_fittest(&population);

        for i in (0..self.iterations).progress() {
            population = self.generate_population(population);
            let challenger = find_fittest(&population);
            debug_print(debug_level, skip, i + 1, &population, &champion, &challenger, self.number_of_points);

            if champion.fitness <= challenger.fitness {
                champion = challenger;
            }
        }

        self.champion = champion;

        if debug_level >= 2 { self.print(); }
    }

    pub fn print(&self) {
        let x = self.population_size * self.iterations;

        println!("\n --------------- \n STATS \n --------------- \n");
        println!("BEST TRAVEL PATH: {:?}", self.champion.dna);
        println!("Fitness Score: {} ", self.champion.fitness);
        println!("{} mutations out of {} individuals produced", self.number_of_mutations, x);
        println!("{} cross-overs out of {} individuals produced", self.number_of_crossovers, x);

        println!("\n --------------- \n SPECS \n --------------- \n");
        println!("iterations: {:?}", self.iterations);
        println!("crossover_probability: {:?}", self.crossover_probability);
        println!("mutation_probability: {:?}", self.mutation_probability);
        println!("population_size: {:?}", self.population_size);
        println!("number_of_points: {:?}", self.number_of_points);
        println!("\n points: ");
        print_vec(&self.points);

        println!("\n --------------- \n END \n --------------- \n");
    }
}

fn debug_print(debug_level: usize, 
               skip: usize, 
               i: usize, 
               population: &[Individual],
               champion: &Individual, 
               challenger: &Individual, 
               n: usize) {
    if debug_level == 1 && (i % skip == 0) {
        print!("{}, {}, {}, {},", i, n, champion.fitness, challenger.fitness);

        for i in 0..n {
            print!(" {},", champion.dna); // [i]
        }

        for i in 0..(n - 1) {
            print!(" {},", challenger.dna); // [i]
        }

        println!(" {}", challenger.dna); // [n - 1]
    }

    if debug_level == 3 {
        println!("#{}: \n current_champion: {:?} \n challenger: {:?}", 
            i, champion, challenger);
    }

    if debug_level == 4 {
        println!("\n --------------- \n {}: Current Population \n --------------- \n", i);
        print_vec(population);
    }
}
