extern crate rand;

use std::fmt;
use std::mem::swap;
use self::rand::{Rng, thread_rng};

use super::Point;

pub const MAX_DEPTH: u32 = 6;
pub const MAX_IDX: usize = length_from_depth(MAX_DEPTH);
pub const MAX_NUMBER_NODE: f32 = 10.0;
pub const MIN_NUMBER_NODE: f32 = -10.0;

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
    let mut rng = thread_rng();
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
    let mut rng = thread_rng();
    let op_idx = rng.gen_range(0, 2);
    let terminal_node = match op_idx {
        0 => Node::Variable,
        _ => Node::Number(rng.gen_range(-2.0, 2.0))
    };
    Some(terminal_node)
}

pub const fn length_from_depth(depth: u32) -> usize {
    2_usize.pow(depth) - 1
}

#[derive(Debug, Clone)]
pub struct SymbolicBinaryHeap<T> {
    heap: Vec<Option<Node<T>>>
}

/// Prints each level on a new line
impl fmt::Display for SymbolicBinaryHeap<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg = String::new();
        for i in 0..MAX_DEPTH {
            let data: &[Option<Node<f32>>] =
                &self.heap[length_from_depth(i)..length_from_depth(i + 1)];
            msg.push_str(&format!("{data:?}\n").to_string())
        }
        write!(f, "{}", msg)
    }
}

impl SymbolicBinaryHeap<f32> {
    /// Default constructor, creates an empty heap
    pub fn new() -> Self {
        let heap: Vec<Option<Node<f32>>> = vec![None; MAX_IDX + 1];
        SymbolicBinaryHeap {heap}
    }

    /// Construct a heap using a premade Some(Node) vector
    pub fn new_from(heap: Vec<Option<Node<f32>>>) -> Self {
        let heap: Vec<Option<Node<f32>>> = heap;
        SymbolicBinaryHeap {heap}
    }
    
    /// Returns the left child node (the only child of Sine/Cosine nodes)
    pub fn left(&mut self, base_idx: usize) -> &mut Option<Node<f32>> {
        let left_idx = 2 * base_idx + 1;
        if left_idx > self.heap.len() - 1 {
            panic!("Attempted to find child below MAX_DEPTH.\nself: {:?}", self)
        }
        &mut self.heap[left_idx]
    }
    
    /// Returns the right child node (the only child of Sine/Cosine nodes)
    pub fn right(&mut self, base_idx: usize) -> &mut Option<Node<f32>> {
        let right_idx = 2 * base_idx + 2;
        if right_idx > self.heap.len() - 1 {
            panic!("Attempted to find child below MAX_DEPTH.\nself: {:?}", self)
        }
        &mut self.heap[right_idx]
    }
    
    /// Returns the parent node
    pub fn parent(&mut self, idx: usize) -> &mut Option<Node<f32>> {
        if idx == 0 {
            panic!("Attempted to find parent at invalid index{}.\nself: {:?}", idx, self)
        }
        &mut self.heap[(idx - 1) / 2]
    }

    /// Set an element on the heap to None and recurse into its children if possible
    pub fn delete_from_idx(&mut self, idx: usize) {
        self.heap[idx] = None;
        let left_idx = 2 * idx + 1;
        let right_idx = 2 * idx + 2;
        if left_idx <= MAX_IDX {
            self.delete_from_idx(left_idx)
        }
        if right_idx <= MAX_IDX {
            self.delete_from_idx(right_idx)
        }
    }

    /// Adds self.heap[idx] to heap then recurses into children if possible
    fn _heap_at_idx(&mut self, idx: usize, heap: &mut Vec<Option<Node<f32>>>){
        heap[idx] = self.heap[idx].clone();
        let left_idx = 2 * idx + 1;
        let right_idx = 2 * idx + 2;
        if left_idx <= MAX_IDX {
            self._heap_at_idx(left_idx, heap);
        }
        if right_idx <= MAX_IDX {
            self._heap_at_idx(right_idx, heap);
        }
    }

    /// Returns a new heap built starting from the idx of this object
    pub fn heap_at_idx(&mut self, base_idx: usize) -> Self {
        let mut heap: Vec<Option<Node<f32>>> = vec![None; MAX_IDX + 1];
        self._heap_at_idx(base_idx, &mut heap);
        SymbolicBinaryHeap::new_from(heap)
    }
    
    /// Helper function for random SymbolicBinaryHeap generation
    fn add_to_node(&mut self, idx: usize,
                   left: Option<Node<f32>>, right: Option<Node<f32>>) {
        *self.left(idx) = left;
        if right.is_some() {
            *self.right(idx) = right;
        }
    }
    
    /// Determines how many children a node can take and applies them
    /// Helper function for random SymbolicBinaryHeap generation
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

    /// Generates a SymbolicBinaryHeap of a given depth
    /// A new heap should be instantiated at base_idx = 0
    /// A non-zero base_idx is used as a potential mutation
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

    /// Returns the depth of the deepest node in the binary heap
    pub fn depth(&mut self) -> u32 {
        // Get the largest index in the heap containing a node
        let mut largest_idx: usize = 0;
        for (i, node) in self.heap.iter().enumerate() {
            if node.is_some() {
                largest_idx = i;
            }
        }
        // Iterate over node parents back to the beginning of the heap
        let mut seek_idx = largest_idx;
        let mut depth: u32 = 1;
        while seek_idx > 0 && self.parent(seek_idx).is_some() {
            depth += 1;
            seek_idx = (seek_idx - 1) / 2;
        }
        depth
    }
    
    /// Recurses into child nodes to determine heap's result for variable
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
    pub fitness: f32,
}

impl Individual {
    pub fn new(points: &[Point]) -> Self {
        let mut dna = SymbolicBinaryHeap::<f32>::new(); 
        let fitness = fitness(&mut dna, &points);
        Individual {dna, fitness}
    }

    pub fn new_from(dna: SymbolicBinaryHeap<f32>, points: &[Point]) -> Self {
        let mut dna = dna; 
        let fitness = fitness(&mut dna, &points);
        Individual {dna, fitness}
    }

    pub fn spawn(&mut self, depth: u32) {
        self.dna.random_instantiate(0, depth);
    }

    /// Perform a random mutation from an array of possible actions
    pub fn mutate(&mut self, points: &[Point]) {
        let i = thread_rng().gen_range(0, MAX_DEPTH - 1);
        //self.dna.swap(i);
        self.fitness = fitness(&mut self.dna, &points);
    }
}

/// Returns the index in the heap of a randomized parent node for swapping
pub fn get_swapping_index(mut sbh: SymbolicBinaryHeap<f32>) -> usize {
    let mut rng = thread_rng();
    let target_depth = rng.gen_range(1, sbh.depth());
    let mut depth_idx: usize = 0;
    let mut seek_idx: usize = 0;
    // This will fail because random descent doesn't guarantee target_depth
    while depth_idx < target_depth as usize {
        let use_right: bool = rng.gen();
        let right_idx = 2 * seek_idx + 2;
        let right_exists = sbh.heap[right_idx].is_some();
        if use_right && right_exists {
            seek_idx = right_idx;
        } else {
            seek_idx = right_idx - 1;
        }
        depth_idx += 1;
    }
    seek_idx
}

fn _cross_over(daughter_idx: usize, son_idx: usize,
               mut daughter: SymbolicBinaryHeap<f32>, mut son: SymbolicBinaryHeap<f32>) {
    swap(&mut daughter.heap[daughter_idx], &mut son.heap[son_idx]);
    let daughter_left_idx = 2 * daughter_idx + 1;
    let daughter_right_idx = 2 * daughter_idx + 2;
    let son_left_idx = 2 * son_idx + 1;
    let son_right_idx = 2 * son_idx + 2;
    if daughter_left_idx <= MAX_IDX && son_left_idx <= MAX_IDX {
        _cross_over(daughter_left_idx, son_left_idx, daughter, son)
    }
    if daughter_right_idx <= MAX_IDX && son_right_idx <= MAX_IDX {
        _cross_over(daughter_left_idx, son_left_idx, daughter, son)
    }
}

/// Choose a random target depth from each parent, and swap a random branch at that depth
/// Of these four individuals, remove the worst performers or the most complicated
/// Gendering of the individuals is done for clarity and dark humor
pub fn cross_over(mother: &Individual, father: &Individual,
                  points: &[Point]) -> (Individual, Individual) {
    let mom_swap_idx: usize = get_swapping_index(mother.dna.clone());
    let dad_swap_idx: usize = get_swapping_index(father.dna.clone());
    let daughter_dna = mother.dna.clone();
    let son_dna = father.dna.clone();
    _cross_over(mom_swap_idx, dad_swap_idx, daughter_dna, son_dna);
    let daughter = Individual::new_from(daughter_dna, points);
    let son = Individual::new_from(son_dna, points);
    (daughter, son)
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
