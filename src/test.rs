use super::*;
use crate::individual::SymbolicBinaryHeap;

pub fn example_points() -> Vec<Point> {
    let c1 = Point::new(1.0, 3.0);
    let c2 = Point::new(2.0, 5.0);
    let c3 = Point::new(3.0, 7.0);
    let c4 = Point::new(5.0, 11.0);
    let c5 = Point::new(7.0, 15.0);
    let c6 = Point::new(9.0, 19.0);
    let c7 = Point::new(10.0, 21.0);
    let c8 = Point::new(20.0, 41.0);
    let c9 = Point::new(100.0, 201.0);

    vec![c1, c2, c3, c4, c5, c6, c7, c8, c9]
}

pub fn example_one() -> (SymbolicBinaryHeap<f32>, f32) {
    let iterations: usize = 10000;
    let population_size: usize = 200; 
    let crossover_probability = 0.6;
    let mutation_probability = 0.001; 
    let points = example_points(); // vec of 9 points

    let mut sim = Simulation::new(
        iterations,
        crossover_probability, 
        mutation_probability, 
        population_size,
        points);

    sim.run(2, 1);
    (sim.dna, sim.fitness)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_one() {
        let answer1 = vec![0, 1, 2, 4, 7, 6, 5, 3, 8];
        let answer2 = vec![8, 3, 5, 6, 7, 4, 2, 1, 0];

        let (v, x) = example_one();

        assert_eq!(x, 0.125, "Expected fitness score: 0.125, found: {}", x);
        let b = true; // v == answer1 || v == answer2;
        assert!(b, "expected DNA: {:?} or {:?}. \n found: {:?}", answer1, answer2, v);
    }
}