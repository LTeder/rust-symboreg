extern crate rand;

use super::Point;
use std::fmt;
use self::rand::{thread_rng, Rng};

pub const MIN_POSITIVE: f64 = 2.2250738585072014e-308f64;

#[derive(Debug, Clone)]
pub enum Node<T> {
    Add,
    Subtract,
    Multiply,
    Divide,
    Sine,
    Cosine,
    Variable,
    Number(T)
}

pub fn get_op() -> Option<Node<f32>> {
    let mut rng = rand::thread_rng();
    let op_idx = rng.gen_range(0, 6);
    let node = match op_idx {
        0 => Node::Add,
        1 => Node::Subtract,
        2 => Node::Multiply,
        3 => Node::Divide,
        4 => Node::Sine,
        _ => Node::Cosine
    };
    Some(node)
}

pub fn get_val() -> Option<Node<f32>> {
    let mut rng = rand::thread_rng();
    let op_idx = rng.gen_range(0, 2);
    let terminal_node = match op_idx {
        0 => Node::Variable,
        _ => Node::Number(rng.gen_range(-1.0, 1.0))
    };
    Some(terminal_node)
}

pub fn length_from_depth(depth: u32) -> usize {
    2_usize.pow(depth) - 1
}

#[derive(Debug, Clone)]
pub struct SymbolicBinaryHeap<T> {
    depth: u32,
    heap: Vec<Option<Node<T>>>
}

impl fmt::Display for SymbolicBinaryHeap<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg = String::new();
        for i in 0..self.depth {
            let data = &self.heap[length_from_depth(i)..length_from_depth(i + 1)];
            msg.push_str(&format!("{data:?}\n").to_string())
        }
        write!(f, "{}", msg)
    }
}

impl SymbolicBinaryHeap<f32> {
    pub fn new(depth: u32) -> Self {
        let heap: Vec<Option<Node<f32>>> = vec![None; length_from_depth(depth)];
        SymbolicBinaryHeap {depth, heap}
    }
    
    pub fn left(&mut self, base_idx: usize) -> &mut Option<Node<f32>> {
        &mut self.heap[2 * base_idx + 1]
    }
    
    pub fn right(&mut self, base_idx: usize) -> &mut Option<Node<f32>> {
        &mut self.heap[2 * base_idx + 2]
    }
    
    fn add_to_node(&mut self, idx: usize, left: Option<Node<f32>>, right: Option<Node<f32>>) {
        *self.left(idx) = left;
        if right.is_some() {
            *self.right(idx) = right;
        }
    }
    
    fn fill_node(&mut self, idx: usize, getter: fn() -> Option<Node<f32>> ) {
        match self.heap[idx] {
            Some(Node::Add) | Some(Node::Subtract) |
            Some(Node::Multiply) | Some(Node::Divide) =>
                self.add_to_node(idx, getter(), getter()),
            Some(Node::Sine) | Some(Node::Cosine) =>
                self.add_to_node(idx, getter(), None),
            _ => ()
        };
    }

    pub fn random_instantiate(&mut self, base_idx: usize, depth: u32) {
        if depth > 1 {
            self.heap[base_idx] = get_op();
            for layer in 0..(depth - 2) {
                let nodes_in_layer = 2_usize.pow(layer);
                for i in 0..nodes_in_layer {
                    let idx = base_idx + layer as usize + i;
                    self.fill_node(idx, get_op);
                }
            }
            let nodes_in_layer = 2_usize.pow(depth - 2);
            for i in 0..nodes_in_layer {
                let idx = base_idx + nodes_in_layer - 1 + i;
                self.fill_node(idx, get_val)
            }
        } else {
            // To do: impl fn delete_from_idx
            self.heap[base_idx] = get_val();
        }
    }
    
    fn _collapse(&mut self, idx: usize, variable: f32) -> f32{
        let l = if self.left(idx).is_some() {
            self._collapse(2 * idx + 1, variable)
        } else {
            0.0
        };
        let r = if self.right(idx).is_some() {
            self._collapse(2 * idx + 2, variable)
        } else {
            0.0
        };
        match self.heap[idx] {
            Some(Node::Add) => {l + r},
            Some(Node::Subtract) => {l - r},
            Some(Node::Multiply) => {l * r},
            Some(Node::Divide) => {
                if r == 0.0 {
                    panic!("Attempted divide by zero.")
                }
                l / r
            },
            Some(Node::Sine) => l.sin(),
            Some(Node::Cosine) => l.cos(),
            Some(Node::Number(n)) => n,
            Some(Node::Variable) => variable,
            _ => 0.0
        }
    }
    
    pub fn collapse(&mut self, variable: f32) -> f32{
        self._collapse(0, variable)
    }
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub dna: SymbolicBinaryHeap<f32>,
    pub fitness: f64,
}

impl Individual {
    pub fn new(points: &[Point]) -> Self {
        let dna = points;
        let fitness = fitness(&dna, &points);
        Individual {dna, fitness}
    }

    pub fn spawn(&self, depth: usize) {
        let mut v:Vec<usize> = (0..depth).collect();
        thread_rng().shuffle(&mut v);
    }

    pub fn cross_over(&self, other: &Individual, points: &[Point]) -> (Self, Self) {
        let n = self.dna.len();
        let mut rng = thread_rng();
        let start = rng.gen_range(0, n - 1);
        let end = rng.gen_range(start + 1, n);

        let daughter_dna = crossover_dna(&self.dna, &other.dna, start, end);
        let son_dna = crossover_dna(&other.dna, &self.dna, start, end);
        
        let daughter = Individual::new(daughter_dna, points);
        let son = Individual::new(son_dna, points);
        
        (daughter, son)
    }

    pub fn mutate(&mut self, points: &[Point]) {
        let i = thread_rng().gen_range(0, self.dna.len() - 1);
        self.dna.swap(i, i + 1);
        self.fitness = fitness(&self.dna, &points);
    }
}

fn fitness(dna: &[usize], points: &[Point]) -> f64 {
    let d = dna.windows(2)
               .fold(MIN_POSITIVE, |acc, w| acc + points[w[0]].distance_squared(&points[w[1]]));
    1.0 / d
}

fn crossover_dna(mom: &[usize], dad: &[usize], start: usize, end: usize) -> Vec<usize> {
    let mom_slice = &mom[start..=end];
    let mut child: Vec<usize> = Vec::new();
    
    for i in 0..dad.len() {
        if !mom_slice.contains(&dad[i]) {
            child.push(dad[i]);
        }
    }
    
    let end_slice = &child.split_off(start);
    child.extend_from_slice(mom_slice);
    child.extend_from_slice(end_slice);
    child
}

/* 
-----------------------------
ALTERNATIVE FITNESS FUNCTION
-----------------------------

fn fitness(dna: &[usize], points: &[Point]) -> f64 {
    let length = points.len() - 1;
    let mut d = MIN_POSITIVE;

    for i in 0..length {
        let (j, k) = (dna[i], dna[i+1]);
        d += points[j].distance_squared(&points[k]);
    }
    1.0 / d
}
*/