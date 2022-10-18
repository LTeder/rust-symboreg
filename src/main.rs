extern crate symboreg;

use std::process;
use std::env;

use symboreg::{Point, Simulation};
use symboreg::helper;
use symboreg::string_to_points;

use symboreg::Individual;

fn main() {
    let test = Point::new(1.0, 1.0);
    let guy = Individual::new(&[test]);
    println!("{:?}", guy);
    process::exit(0);
    // ----------------------
    // PARSE ARGUMENTS
    // ----------------------
    let mut args = env::args().skip(1);

    let specs_filename = args.next()
        .unwrap_or_else( || {
            eprintln!("Please specify filename with simulation specifications. \
            \n USAGE: cargo run ./specs.csv /cities.csv > output.csv");
            process::exit(1)
            }
        );

    let city_filename = args.next()
        .unwrap_or_else( || {
            eprintln!("Please specify filename  with cities. \
            \n USAGE: cargo run ./specs.csv /cities.csv > output.csv");
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
         });

    let contents = helper::read_file(&city_filename);
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
