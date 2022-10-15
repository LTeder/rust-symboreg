use super::*; 

pub fn example_points() -> Vec<Point> {
    let c1 = Point::new(1.0, 3.0);
    let c2 = Point::new(1.0, 2.0);
    let c3 = Point::new(1.0, 1.0);
    let c4 = Point::new(4.0, 3.0);
    let c5 = Point::new(2.0, 1.0);
    let c6 = Point::new(3.0, 3.0);
    let c7 = Point::new(3.0, 2.0);
    let c8 = Point::new(3.0, 1.0);
    let c9 = Point::new(4.0, 4.0);

    vec![c1, c2, c3, c4, c5, c6, c7, c8, c9]
}


pub fn example_one() -> (Vec<usize>, f64) {
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


pub fn example_two() -> f64 {
    let iterations: usize = 1000;
    let population_size: usize = 120; 
    let crossover_probability = 0.7;
    let mutation_probability = 0.001; 

    let mut n = 0;
    let t = 150;
    let score = 0.125;
    let solution = vec![
        vec![0, 1, 2, 4, 7, 6, 5, 3, 8], 
        vec![8, 3, 5, 6, 7, 4, 2, 1, 0]];
    
    let all_points = example_points(); // vec of 9 points

    for i in 0..t {
        let points = all_points.clone();
        let mut sim = Simulation::new(
            iterations,
            crossover_probability, 
            mutation_probability, 
            population_size,
            points);

        sim.run(0, 1);

        if sim.fitness == score {
            if sim.dna == solution[0] || sim.dna == solution[1] {
                n += 1;
            }
            println!("{}:{}", i, n);
        }
    }

    let percent_correct = 100.0 * n as f64 / t as f64;
    println!("\n --------------- \n {}% correct with {} simulations \n --------------- \n", percent_correct, t);
    percent_correct
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
        let b = v == answer1 || v == answer2;
        assert!(b, "expected DNA: {:?} or {:?}. \n found: {:?}", answer1, answer2, v);
    }

    #[test]
    fn test_two() {
        let x = example_two();
        assert!(x > 50.0, "Expected percent_correct > 50.0 % found {}", x);
    }
}