extern crate rand;

use std::mem::swap;
use self::rand::{Rng, thread_rng};

use super::{Point, SymbolicBinaryHeap, MAX_IDX};

#[derive(Debug, Clone)]
pub struct Individual {
    pub dna: SymbolicBinaryHeap<f32>,
    pub fitness: f32,
    pub evaluations: usize
}

impl Individual {
    pub fn new(points: &[Point]) -> Self {
        let mut dna = SymbolicBinaryHeap::<f32>::new();
        dna.spawn();
        let fitness = fitness(&mut dna, &points);
        let evaluations: usize = 1;
        Individual {dna, fitness, evaluations}
    }

    pub fn new_from(dna: SymbolicBinaryHeap<f32>, points: &[Point]) -> Self {
        let mut dna = dna; 
        let fitness = fitness(&mut dna, &points);
        let evaluations: usize = 1;
        Individual {dna, fitness, evaluations}
    }
    
    /// Choose a random target depth from each parent, and swap a random branch at that depth
    /// Of these four individuals, remove the worst performers or the most complicated
    /// Gendering of the individuals is done for clarity and dark humor
    pub fn cross_over(mut self, father: &mut Individual, points: &[Point])
                                               -> (Individual, Individual) {
        let mut swap_idxs: Vec<(usize, usize)> = Vec::new();
        swap_idxs.push((self.dna.get_swap_idx(), father.dna.get_swap_idx()));
        self.dna.check_swap_idx(swap_idxs[0].0);
        father.dna.check_swap_idx(swap_idxs[0].1);
        // Swap values between the potential offspring
        let (mut daughter_dna, mut son_dna) = (self.dna.clone(), father.dna.clone());
        while let Some((mom_idx, dad_idx)) = swap_idxs.pop() {
            swap(&mut daughter_dna.heap[mom_idx], &mut son_dna.heap[dad_idx]);
            let daughter_left_idx = 2 * mom_idx + 1;
            let daughter_right_idx = 2 * mom_idx + 2;
            let son_left_idx = 2 * dad_idx + 1;
            let son_right_idx = 2 * dad_idx + 2;
            if daughter_left_idx < MAX_IDX && son_left_idx < MAX_IDX {
                swap_idxs.push((daughter_left_idx, son_left_idx));
            }
            if daughter_right_idx <= MAX_IDX && son_right_idx <= MAX_IDX {
                swap_idxs.push((daughter_right_idx, son_right_idx));
            }
        }
        // Choose the two best individuals, carrying evalations into offspring
        let mut son = Individual::new_from(son_dna, points);
        if (father.fitness > son.fitness && father.dna.complexity() == son.dna.complexity())
                || father.dna.complexity()  < son.dna.complexity() {
            son = Individual::new_from(father.dna.clone(), points); // father becomes son
            son.evaluations += 1;
        }
        let daughter = Individual::new_from(daughter_dna, points);
        if daughter.fitness > self.fitness {
            return (daughter, son);
        } else {
            self.evaluations += 1;
            return (self, son);
        }
    }

    /// Perform a random mutation from an array of possible actions
    pub fn mutate(&mut self, points: &[Point]) {
        let mut rng = thread_rng();
        if self.dna.depth() > 2 {
            match rng.gen_range(0, 4) {
                0 => self.dna.mutate_constant(),
                1 => self.dna.mutate_clip(),
                2 => self.dna.mutate_swap(),
                _ => self.dna.mutate_similar()
            };
        } else {
            match rng.gen_range(0, 2) {
                0 => self.dna.mutate_constant(),
                _ => self.dna.mutate_similar()
            };
        }
        self.update_fitness(&points);
    }

    pub fn update_fitness(&mut self, points: &[Point]) {
        self.fitness = fitness(&mut self.dna, &points);
        self.evaluations += 1;
    }
}

/// Sum of the squared error at each point
fn fitness(dna: &mut SymbolicBinaryHeap<f32>, points: &[Point]) -> f32 {
    let mut squared_error: f32 = f32::MIN_POSITIVE;
    for point in points {
        let difference = point.y - dna.collapse(point.x);
        squared_error +=  difference.powi(2);
    }
    if !dna.has_variable() { // Penalize constant functions
        squared_error *= 10.0;
    }
    1.0 / squared_error
}
