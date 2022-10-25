# rust-symboreg, "Rusty Genes"
- A Rust implementation of a genetic program to perform symbolic regression
- See [Mithi](https://github.com/mithi) Sevilla's [Medium article](https://medium.com/@mithi/genetic-algorithms-in-rust-for-autonomous-agents-an-introduction-ac182de32aee) for a detailed discussion of the [repo](https://github.com/mithi/rusty-genes) from which this was initially forked, addressing the Travelling Salesman Problem

Given the limited span of time I had to work on this project, the final result is lacking slightly. However, the program is complete enough to converge on a correct solution for certain datasets. Suggestions and contributions are welcome.

Improvements are needed in the crossover and mutations methods. Currently the amount of redundant checks with crossover is so high that the order-of-magnitudes increase in speed from disabling it makes the so-called "hill climber" the more performant approach!

# Usage
```bash
$ cargo run ./data/specs/specs0.csv ./data/datasets/set0.csv > ./output0.csv
```

# Testing
```bash
$ cargo test -- --nocapture
$ cargo build
$ cargo run ./data/specs/specs0.csv ./data/datasets/set0.csv
```

# Simulation
```
# ------------
# ./specs.csv
# ------------
debug_level, skip, iterations, population_size, crossover_probability, mutation_probability

where:
- debug_level: 0 (no output), 1 (CSV format), 2 (print champion and challenger), or 3 (print full population)
- skip: an integer >= 1, debug print is called when iteration % skip == 0
- iterations: an integer >= 1
- population_size: an even integer, divisible by ten
- crossover_probability: between 0.0 and 1.0
- mutation_probability: between 0.0 and 1.0

# ------------
# ./datasets.csv
# ------------
x1, y1
x2, y2
# . . .
xn, yn

when debug_level == 1:
# ------------
# ./output.csv
# ------------
iteration_step, champion_fitness, challenger_fitness, champion_dna, challenger_dna
# . . .
```
