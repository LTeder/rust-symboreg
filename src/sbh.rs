extern crate rand;

use std::fmt;
use self::rand::{Rng, thread_rng};

pub const MAX_DEPTH: u32 = 6;
pub const MAX_IDX: usize = length_from_depth(MAX_DEPTH);
pub const MAX_NUMBER_NODE: f32 = 15.0;
pub const MIN_NUMBER_NODE: f32 = -15.0;

#[derive(Debug, Clone, Copy)]
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
        _ => Node::Number(rng.gen_range(MIN_NUMBER_NODE, MAX_NUMBER_NODE))
    };
    Some(terminal_node)
}

pub const fn length_from_depth(depth: u32) -> usize {
    2_usize.pow(depth) - 1
}

#[derive(Debug, Clone)]
pub struct SymbolicBinaryHeap<T> {
    pub heap: Vec<Option<Node<T>>>,
    rng: rand::ThreadRng
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
        SymbolicBinaryHeap {heap, rng: thread_rng() }
    }

    /// Construct a heap using a premade Some(Node) vector
    pub fn new_from(heap: Vec<Option<Node<f32>>>) -> Self {
        let heap: Vec<Option<Node<f32>>> = heap;
        SymbolicBinaryHeap {heap, rng: thread_rng()}
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
            self.heap[base_idx] = get_val();
        }
    }

    /// Here, complexity is defined as the number of nodes in the heap
    pub fn complexity(&mut self) -> u32 {
        let mut complexity: u32 = 0;
        for node in &self.heap {
            if node.is_some() {
                complexity += 1;
            }
        }
        complexity
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

    /// Returns the index in the heap of a randomized parent node for swapping
    pub fn get_swapping_index(&mut self) -> usize {
        let depth = self.depth();
        let target_depth = self.rng.gen_range(1, depth);
        let mut depth_idx: usize = 0;
        let mut seek_idx: usize = 0;
        // This will fail because random descent doesn't guarantee target_depth
        while depth_idx < target_depth as usize {
            let use_right: bool = self.rng.gen();
            let right_idx = 2 * seek_idx + 2;
            let right_exists = self.heap[right_idx].is_some();
            if use_right && right_exists {
                seek_idx = right_idx;
            } else {
                seek_idx = right_idx - 1;
            }
            depth_idx += 1;
        }
        seek_idx
    }

    fn get_terminal_idxs(&mut self) -> Vec<usize> {
        let mut terminals: Vec<usize> = Vec::new();
        for (i, node) in self.heap.iter().enumerate() {
            match node {
                Some(Node::Variable) | Some(Node::Number(_)) => terminals.push(i),
                _ => ()
            };
        }
        terminals
    }

    fn get_op_idxs(&mut self) -> Vec<usize> {
        let mut ops: Vec<usize> = Vec::new();
        for (i, node) in self.heap.iter().enumerate() {
            match node {
                Some(Node::Variable) | Some(Node::Number(_)) | None => (),
                _ => ops.push(i)
            };
        }
        ops
    }
    
    /// Alter a random terminal node with a constant
    pub fn mutate_constant(&mut self) {
        let terminals: Vec<usize> = self.get_terminal_idxs();
        let mut choice;
        let mut spawn_depth: u32 = 2;
        loop {
            let idx = self.rng.gen_range(1, terminals.len());
            choice = terminals[idx];
            if 2 * idx + 2 < MAX_IDX {
                if 4 * idx + 6 < MAX_IDX {
                    spawn_depth = 3
                }
                break;
            }
        }
        self.random_instantiate(choice, spawn_depth);
    }
    
    /// Replace a random operation node with a terminal node
    pub fn mutate_clip(&mut self) {
        let terminals: Vec<usize> = self.get_terminal_idxs();
        let choice: usize = terminals[self.rng.gen_range(1, terminals.len())];
        self.delete_from_idx(choice);
        self.heap[choice] = get_val();
    }
    
    /// Swap two random branches
    pub fn mutate_swap(&mut self) {
        // Get parent node indicies
        let full_depth = self.depth();
        let mut ops: Vec<usize> = self.get_op_idxs();
        let choice1: usize = ops.swap_remove(self.rng.gen_range(0, ops.len()));
        let choice1_depth = self.heap_at_idx(choice1).depth();
        let mut choice2: usize;
        loop {
            choice2 = ops.swap_remove(self.rng.gen_range(0, ops.len()));
            let choice2_depth = self.heap_at_idx(choice2).depth();
            if full_depth - choice1_depth + choice2_depth <= MAX_DEPTH {
                break;
            }
        }
        // Swap starting at the parent node
        let mut swap_idxs: Vec<(usize, usize)> = Vec::new();
        swap_idxs.push((choice1, choice2));
        while let Some((idx1, idx2)) = swap_idxs.pop() {
            self.heap.swap(idx1, idx2);
            let daughter_left_idx = 2 * idx1 + 1;
            let daughter_right_idx = 2 * idx1 + 2;
            let son_left_idx = 2 * idx2 + 1;
            let son_right_idx = 2 * idx2 + 2;
            if daughter_left_idx <= MAX_IDX && son_left_idx <= MAX_IDX {
                swap_idxs.push((daughter_left_idx, son_left_idx));
            }
            if daughter_right_idx <= MAX_IDX && son_right_idx <= MAX_IDX {
                swap_idxs.push((daughter_right_idx, son_right_idx));
            }
        }
    }
    
    /// Replace a random operation node with a similar operation
    pub fn mutate_similar(&mut self) {
        let mut ops: Vec<(usize, Node<f32>)> = Vec::new();
        for (i, node) in self.heap.iter().enumerate() {
            match node {
                Some(Node::Add) => ops.push((i, Node::Subtract)),
                Some(Node::Subtract) => ops.push((i, Node::Add)),
                Some(Node::Multiply) => ops.push((i, Node::Divide)),
                Some(Node::Divide) => ops.push((i, Node::Multiply)),
                Some(Node::Sine) => ops.push((i, Node::Cosine)),
                Some(Node::Cosine) => ops.push((i, Node::Sine)),
                Some(Node::Number(_)) =>
                    ops.push((i, Node::Number(self.rng.gen_range(MIN_NUMBER_NODE, MAX_NUMBER_NODE)))),
                _ => ()
            };
        }
        let choice: (usize, Node<f32>) = ops[self.rng.gen_range(0, ops.len())];
        self.heap[choice.0] = Some(choice.1);
    }
    
    /// Recurses into child nodes to determine heap's result for variable
    fn _collapse(&mut self, idx: usize, variable: f32) -> f32{
        let l = if self.left(idx).is_some() {
            self._collapse(2 * idx + 1, variable)
        } else {
            f32::MIN_POSITIVE
        };
        let r = if self.right(idx).is_some() {
            self._collapse(2 * idx + 2, variable)
        } else {
            f32::MIN_POSITIVE
        };
        match self.heap[idx] {
            Some(Node::Add) => {l + r},
            Some(Node::Subtract) => {l - r},
            Some(Node::Multiply) => {l * r},
            Some(Node::Divide) => {
                assert!(r != 0.0, "Attempted divide by zero.");
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
