extern crate symboreg;

use std::process;
use std::env;

use symboreg::{Point, Simulation, helper, string_to_points};

fn main() {
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
