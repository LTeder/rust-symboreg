# rust-symboreg, "Rusty Genes"
- A Rust implementation of a genetic program to perform symbolic regression
- See [Mithi](https://github.com/mithi) Sevilla's [Medium article](https://medium.com/@mithi/genetic-algorithms-in-rust-for-autonomous-agents-an-introduction-ac182de32aee) for a detailed discussion of the [repo](https://github.com/mithi/rusty-genes) from which this was initially forked, addressing the Travelling Salesman Problem

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
