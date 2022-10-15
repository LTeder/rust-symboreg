# Rusty Genes
- A Rust implementation of a genetic program to perform symbolic regression
- See [Mithi](https://github.com/mithi) Sevilla's [Medium article](https://medium.com/@mithi/genetic-algorithms-in-rust-for-autonomous-agents-an-introduction-ac182de32aee) for a detailed discussion of this repo's source, which addressed the Travelling Salesman Problem

```bash
$ curl https://sh.rustup.rs -sSf | sh
$ cargo test -- --nocapture
$ cargo build
$ cargo run ./data/specs/specs0.csv ./data/datasets/cities0.csv > ./output0.csv
```

# Simulation
```
# ------------
# ./specs.csv
# ------------
debug_level, skip, iterations, population_size, crossover_probability, mutation_probability

where:
- debug_level: 0, 1, 2, 3 or 4
- skip: row in csv file will be written when iteration % skip == 0, should be unsigned integer >= 1
- iterations: unsigned integer
- population_size: should be an even integer and divisible by ten
- crossover_probability: between 0.0 and 1.0
- mutation_probability: between 0.0 and 1.0

# ------------
# ./datasets.csv
# ------------
id1, x1, y1
id2, x2, y2
# . . .
idn, xn, yn

# ------------
# ./output.csv
# ------------
iteration_step, number_of_points, champion_fitness, challenger_fitness, champion_dna, challenger_dna
# . . .
```
