extern crate rand;

use std::mem::swap;
use self::rand::{Rng, thread_rng};

use super::{Point, SymbolicBinaryHeap, MAX_IDX};

#[derive(Debug, Clone)]
pub struct Individual {
    pub dna: SymbolicBinaryHeap<f32>,
    pub fitness: f32,
}

impl Individual {
    pub fn new(points: &[Point]) -> Self {
        let mut dna = SymbolicBinaryHeap::<f32>::new();
        dna.spawn();
        let fitness = fitness(&mut dna, &points);
        Individual {dna, fitness}
    }

    pub fn new_from(dna: SymbolicBinaryHeap<f32>, points: &[Point]) -> Self {
        let mut dna = dna; 
        let fitness = fitness(&mut dna, &points);
        Individual {dna, fitness}
    }

    pub fn complexity(&mut self) -> u32 {
        self.dna.complexity()
    }
    
    /// Choose a random target depth from each parent, and swap a random branch at that depth
    /// Of these four individuals, remove the worst performers or the most complicated
    /// Gendering of the individuals is done for clarity and dark humor
    pub fn cross_over(mut self, father: &mut Individual, points: &[Point]) -> (Individual, Individual) {
        let mut swap_idxs: Vec<(usize, usize)> = Vec::new();
        swap_idxs.push((self.dna.get_swapping_index(), father.dna.get_swapping_index()));
        // Swap values between the potential offspring
        let (mut daughter_dna, mut son_dna) = (self.dna.clone(), father.dna.clone());
        while let Some((mom_idx, dad_idx)) = swap_idxs.pop() {
            swap(&mut daughter_dna.heap[mom_idx], &mut son_dna.heap[dad_idx]);
            let daughter_left_idx = 2 * mom_idx + 1;
            let daughter_right_idx = 2 * mom_idx + 2;
            let son_left_idx = 2 * dad_idx + 1;
            let son_right_idx = 2 * dad_idx + 2;
            if daughter_left_idx <= MAX_IDX && son_left_idx <= MAX_IDX {
                swap_idxs.push((daughter_left_idx, son_left_idx));
            }
            if daughter_right_idx <= MAX_IDX && son_right_idx <= MAX_IDX {
                swap_idxs.push((daughter_right_idx, son_right_idx));
            }
        }
        let daughter = Individual::new_from(daughter_dna, points);
        let mut son = Individual::new_from(son_dna, points);
        // Choose the two best individuals
        if father.fitness > son.fitness || father.complexity() <= son.complexity() {
            son = Individual::new_from(father.dna.clone(), points);
            if daughter.fitness > self.fitness {
                return (daughter, son);
            } else {
                return (self, son);
            }
        } else {
            if daughter.fitness > self.fitness {
                return (daughter, son);
            } else {
                return (self, son);
            }
        }
    }

    /// Perform a random mutation from an array of possible actions
    pub fn mutate(&mut self, points: &[Point]) {
        let mut rng = thread_rng();
        if self.dna.depth() > 2 {
            let op_idx = rng.gen_range(0, 4);
            match op_idx {
                0 => self.dna.mutate_constant(),
                1 => self.dna.mutate_clip(),
                2 => self.dna.mutate_swap(),
                _ => self.dna.mutate_similar()
            };
        } else {
            let op_idx = rng.gen_range(0, 2);
            match op_idx {
                0 => self.dna.mutate_constant(),
                _ => self.dna.mutate_similar()
            };
        }
        self.fitness = fitness(&mut self.dna, &points);
    }
}

/// Sum of the squared error at each point
fn fitness(dna: &mut SymbolicBinaryHeap<f32>, points: &[Point]) -> f32 {
    let mut score: f32 = f32::MIN_POSITIVE;

    for point in points {
        let difference = point.y - dna.collapse(point.x);
        score +=  difference.powi(2);
    }
    1.0 / score
}
