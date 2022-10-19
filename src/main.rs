extern crate symboreg;

use std::process;
use std::env;

use symboreg::SymbolicBinaryHeap;
use symboreg::{Point, Simulation};
use symboreg::helper;
use symboreg::string_to_points;

use symboreg::{Individual, Node};

fn main() {
    let c1 = Point::new(1.0, 3.0);
    let c2 = Point::new(2.0, 5.0);
    let c3 = Point::new(3.0, 7.0);
    let c4 = Point::new(5.0, 11.0);
    let c5 = Point::new(7.0, 15.0);
    let c6 = Point::new(9.0, 19.0);
    let c7 = Point::new(10.0, 21.0);
    let c8 = Point::new(20.0, 41.0);
    let c9 = Point::new(100.0, 201.0);

    let test = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9];
    let brain: Vec<Option<Node<f32>>> = vec![Some(Node::Add),
        Some(Node::Multiply),     Some(Node::Number(2.0)),
        Some(Node::Variable), Some(Node::Number(2.0)),     None, None,
        None, None,     None, None,     None, None,     None, None];
    let skull: SymbolicBinaryHeap<f32> = SymbolicBinaryHeap::<f32>::new_from(brain);
    let solution = Individual::new_from(skull, &test);
    println!("{:?} {:?}", solution, solution.fitness);
    process::exit(0);

    // ----------------------
    // PARSE ARGUMENTS
    // ----------------------
    let mut args = env::args().skip(1);

    let specs_filename = args.next()
        .unwrap_or_else( || {
            eprintln!("Please specify filename containing simulation specifications. \
            \n USAGE: cargo run ./specs.csv /points.csv > output.csv");
            process::exit(1)
            }
        );

    let points_filename = args.next()
        .unwrap_or_else( || {
            eprintln!("Please specify filename containing dataset. \
            \n USAGE: cargo run ./specs.csv /points.csv > output.csv");
            process::exit(1)
            }
        );

    // ----------------------
    // EXTRACT SPECS AND DATA
    // ----------------------
    let contents = helper::read_file(&specs_filename);
    let (debug_level, 
         skip,
         iterations, 
         population_size,
         crossover_probability,
         mutation_probability) = helper::parse_specs(&contents).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
         }
    );

    let contents = helper::read_file(&points_filename);
    let points: Vec<Point> = string_to_points(&contents);

    // ----------------------
    // RUN SIMULATION
    // ----------------------
    let mut sim = Simulation::new(
        iterations,
        crossover_probability, 
        mutation_probability, 
        population_size,
        points
    );
    
    sim.run(debug_level, skip);
}
