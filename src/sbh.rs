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
    let node = match thread_rng().gen_range(0, 6) {
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
    let terminal_node = match rng.gen_range(0, 2) {
        0 => Node::Variable,
        _ => Node::Number(rng.gen_range(MIN_NUMBER_NODE / 2.0, MAX_NUMBER_NODE / 2.0))
    };
    Some(terminal_node)
}

pub const fn length_from_depth(depth: u32) -> usize {
    2_usize.pow(depth) - 1
}

pub fn depth_from_idx(idx: usize) -> u32 {
    let mut idx: usize = idx;
    let mut depth: u32 = 1;
    while idx > 0 {
        depth += 1;
        idx = (idx - 1) / 2;
    }
    depth
}

/// Returns whether idx1 is a parent node of idx2, or vice versa
pub fn check_related(idx1: usize, idx2: usize) -> bool {
    if idx1 == 0 || idx2 == 0 {
        return true;
    }
    let mut seek_idx = idx1;
    while seek_idx > 0 && seek_idx >= idx2 {
        if seek_idx == idx2 {
            return true;
        }
        seek_idx = (seek_idx - 1) / 2;
    }
    seek_idx = idx2;
    while seek_idx > 0 && seek_idx >= idx1 {
        if seek_idx == idx1 {
            return true;
        }
        seek_idx = (seek_idx - 1) / 2;
    }
    false
}

#[derive(Debug, Clone)]
pub struct SymbolicBinaryHeap<T> {
    pub heap: Vec<Option<Node<T>>>,
    pub rng: rand::ThreadRng
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

    // Performs random_instantiate on an empty heap
    pub fn spawn(&mut self) {
        let depth: u32 = self.rng.gen_range(2, MAX_DEPTH);
        self.random_instantiate(0, depth);
    }
    
    /// Returns the left child node (the only child of Sine/Cosine nodes)
    pub fn left(&mut self, base_idx: usize) -> &mut Option<Node<f32>> {
        let left_idx = 2 * base_idx + 1;
        if left_idx > MAX_IDX {
            panic!("Attempted to find child at {}, below MAX_DEPTH.\nself:\n{}", left_idx, self)
        }
        &mut self.heap[left_idx]
    }
    
    /// Returns the right child node (the only child of Sine/Cosine nodes)
    pub fn right(&mut self, base_idx: usize) -> &mut Option<Node<f32>> {
        let right_idx = 2 * base_idx + 2;
        if right_idx > MAX_IDX {
            panic!("Attempted to find child at {}, below MAX_DEPTH.\nself:\n{}", right_idx, self)
        }
        &mut self.heap[right_idx]
    }
    
    /// Returns the parent node
    pub fn parent(&mut self, idx: usize) -> &mut Option<Node<f32>> {
        if idx < 1 {
            panic!("Attempted to find parent at invalid index {}.\nself:\n{}", idx, self)
        }
        &mut self.heap[(idx - 1) / 2]
    }

    pub fn has_variable(&mut self) -> bool {
        let mut has_variable = false;
        for node in self.heap.iter() {
            match node {
                Some(Node::Variable) => {
                    has_variable = true; 
                    break; },
                _ => ()
            }
        }
        has_variable
    }

    /// Recursively delete child nodes
    fn _delete_from_idx(&mut self, idx: usize) {
        let left_idx = 2 * idx + 1;
        let right_idx = left_idx + 1;
        if left_idx < MAX_IDX {
            self.heap[left_idx] = None;
            self._delete_from_idx(left_idx)
        }
        if right_idx <= MAX_IDX {
            self.heap[right_idx] = None;
            self._delete_from_idx(right_idx)
        }
    }

    /// Construct a node for idx with a variable or a constant of the node solved at 1
    fn op_to_terminal(&mut self, idx: usize) -> Node<f32> {
        let use_variable: bool = self.rng.gen();
        if use_variable {
            Node::Variable
        } else {
            let num: f32 = self._collapse(idx, 1.0);
            Node::Number(num)
        }
    }

    /// Replace the node at idx op_to_terminal
    /// Then recursively delete any potential child nodes
    pub fn delete_from_idx(&mut self, idx: usize) {
        let parent_idx = (idx - 1) / 2;
        // Defines a mapping between parent node and number of children
        self.heap[idx] = match self.heap[parent_idx] {
            Some(Node::Sine) | Some(Node::Cosine) =>
                if parent_idx == idx / 2 { // if idx is a left child node
                    Some(self.op_to_terminal(idx))
                } else {None},
            Some(Node::Add) | Some(Node::Subtract) |
            Some(Node::Multiply) | Some(Node::Divide) =>
                Some(self.op_to_terminal(idx)),
            _ => None // includes Node::Variable and Node::Number
        };
        self._delete_from_idx(idx)
    }

    /// Adds self.heap[idx] to heap then recurses into children if possible
    fn _heap_at_idx(&mut self, idx: usize, self_idx: usize, heap: &mut Vec<Option<Node<f32>>>){
        heap[idx] = self.heap[self_idx].clone();
        let left_idx = 2 * idx + 1;
        let right_idx = 2 * idx + 2;
        let left_self_idx = 2 * self_idx + 1;
        let right_self_idx = 2 * self_idx + 2;
        // self_idx >= idx always since idx starts at 0
        if left_self_idx <= MAX_IDX {
            self._heap_at_idx(left_idx, left_self_idx, heap);
        }
        if right_self_idx <= MAX_IDX {
            self._heap_at_idx(right_idx, right_self_idx, heap);
        }
    }

    /// Returns a new heap built starting from the idx of this object
    pub fn heap_at_idx(&mut self, base_idx: usize) -> Self {
        let mut heap: Vec<Option<Node<f32>>> = vec![None; MAX_IDX + 1];
        self._heap_at_idx(0, base_idx, &mut heap);
        SymbolicBinaryHeap::new_from(heap)
    }
    
    /// Determines how many children a node can take and applies them
    /// Helper function for random SymbolicBinaryHeap generation
    fn fill_node(&mut self, idx: usize, getter1: fn() -> Option<Node<f32>>,
                                        getter2: fn() -> Option<Node<f32>>) {
        assert!(idx <= (MAX_IDX - 1) / 2,
            "Attempted to fill node at invalid index {}.\nself:\n{}", idx, self);
        // Defines a mapping between parent node and number of children
        match self.heap[idx] {
            Some(Node::Add) | Some(Node::Subtract) |
            Some(Node::Multiply) | Some(Node::Divide) => {
                *self.left(idx) = getter1();
                *self.right(idx) = getter2() },
            Some(Node::Sine) | Some(Node::Cosine) => {
                *self.left(idx) = getter1();
                *self.right(idx) = None; },
            _ => { // Includes Variable and Number(_)
                *self.left(idx) = None;
                *self.right(idx) = None; }
        };
    }

    /// Generates a SymbolicBinaryHeap of a given depth
    /// A new heap should be instantiated at base_idx = 0
    /// A non-zero base_idx is used as a potential mutation
    pub fn random_instantiate(&mut self, base_idx: usize, depth: u32) {
        self._delete_from_idx(base_idx);
        assert!(depth_from_idx(base_idx) + depth - 1 <= MAX_DEPTH,
            "Attempted to random_instantiate from index {} with depth {}.\nself:\n{}",
            base_idx, depth, self);
        if base_idx == 0 {
            if depth > 1 {
                self.heap[base_idx] = get_op();
                // Randomly fill all possible operation nodes
                for layer in 0..(depth - 2) {
                    let nodes_in_layer = 2_usize.pow(layer);
                    for i in 0..nodes_in_layer {
                        let idx = base_idx + layer as usize + i;
                        let use_op: bool = self.rng.gen();
                        assert!(depth_from_idx(idx) < MAX_DEPTH - 1,
                            "Tried to place an operation at the lowest depth.\nself:\n{}",
                            self);
                        let getter: fn() -> Option<Node<f32>> =
                            if use_op {get_op} else {get_val};
                        if self.rng.gen() {
                            self.fill_node(idx, get_op, getter);
                        } else {
                            self.fill_node(idx, getter, get_op);
                        }
                    }
                }
                // Index into the last layer created (or 0) and add terminals
                let nodes_in_layer = 2_usize.pow(depth - 2);
                let offset = base_idx + nodes_in_layer - 1;
                for i in 0..nodes_in_layer {
                    self.fill_node(offset + i, get_val, get_val);
                }
            } else {
                panic!("Tried to instantiate at 0 with bad depth {}.\nself:\n{}\n",
                       depth, self);
            }
        } else {
            if depth > 1 {
                let mut source = SymbolicBinaryHeap::<f32>::new();
                source.random_instantiate(0, depth);
                self._delete_from_idx(base_idx);
                self._swap_from(Some(source), base_idx, 0);    
            } else if depth == 1 {
                self.heap[base_idx] = get_val();
            } else {
                panic!("Tried to instantiate at {} with bad depth {}.\nself:\n{}\n",
                       base_idx, depth, self);
            }
        }
    }

    pub fn _swap_from(&mut self, ext_src: Option<Self>, choice1: usize, choice2: usize) {
        // Swap starting at the parent node
        let do_swap: bool = ext_src.is_none();
        let mut swap_idxs: Vec<(usize, usize)> = Vec::new();
        swap_idxs.push((choice1, choice2));
        while let Some((idx1, idx2)) = swap_idxs.pop() {
            if do_swap {
                self.heap.swap(idx1, idx2);
            } else {
                self.heap[idx1] = ext_src.as_ref().unwrap().heap[idx2];
            }
            let idx1_left = 2 * idx1 + 1;
            let idx1_right = 2 * idx1 + 2;
            let idx2_left = 2 * idx2 + 1;
            let idx2_right = 2 * idx2 + 2;
            if idx1_left < MAX_IDX && idx2_left < MAX_IDX {
                swap_idxs.push((idx1_left, idx2_left));
            }
            if idx1_right <= MAX_IDX && idx2_right <= MAX_IDX {
                swap_idxs.push((idx1_right, idx2_right));
            }
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
    /// Assumes each operation node has at least one terminal node below it
    pub fn depth(&mut self) -> u32 {
        depth_from_idx(*self.get_checked_terminals().last().unwrap())
    }

    /// Performs self.get_terminal_idxs() and checks if empty
    fn get_checked_terminals(&mut self) -> Vec<usize> {
        let mut node_idxs = self.get_terminal_idxs();
        if node_idxs.len() == 0 { // No terminal nodes
            node_idxs = self.get_op_idxs();
            if node_idxs.len() <= 1 { // And only the top operation node
                let depth = self.rng.gen_range(2, 4);
                self.random_instantiate(0, depth);
            } else if node_idxs.len() > 1 { // Multiple operators present
                let choice = node_idxs[self.rng.gen_range(1, node_idxs.len())];
                self.delete_from_idx(choice);
            }
            node_idxs = self.get_terminal_idxs();
        }
        if self.heap[1].is_some() {
            for _ in 0..node_idxs.len() {
                if self.parent(*node_idxs.last().unwrap()).is_none() {
                    self._delete_from_idx(node_idxs.pop().unwrap());
                }
            }
        }
        node_idxs
    }

    /// Returns the index in the heap of a randomized parent node for swapping
    pub fn get_swap_idx(&mut self) -> usize {
        let depth = self.depth();
        let target_depth = self.rng.gen_range(1, depth);
        let mut node_idxs: Vec<usize>;
        if target_depth > 1 {
            node_idxs = self.get_op_idxs();
            let mut choice: usize;
            for _ in 0..node_idxs.len() {
                choice = node_idxs.swap_remove(self.rng.gen_range(0, node_idxs.len()));
                if self.heap_at_idx(choice).depth() == target_depth {
                    return choice;
                }
            }
        }
        node_idxs = self.get_checked_terminals();
        node_idxs[self.rng.gen_range(0, node_idxs.len())]
    }

    pub fn check_swap_idx(&mut self, idx: usize) {
        assert!(self.heap[idx].is_some(), "Bad swap index {}.\nself:\n{}\n", idx, self);
        let parent_idx = (idx - 1) / 2;
        match self.parent(idx) {
            Some(Node::Sine) | Some(Node::Cosine) =>
                if parent_idx == idx / 2 && self.right(parent_idx).is_some() {
                    self._delete_from_idx(idx + 1);
                    self.heap[idx + 1] = None;
                } else if self.left(parent_idx).is_some() {
                    self._delete_from_idx(idx - 1);
                    self.heap[idx - 1] = None;
                },
            Some(Node::Add) | Some(Node::Subtract) |
            Some(Node::Multiply) | Some(Node::Divide) => (),
            _ => { // includes Node::Variable and Node::Number
                let node: Option<Node<f32>>;
                if parent_idx == idx / 2 && idx + 1 <= MAX_IDX {
                    if self.right(parent_idx).is_some() {
                        node = match self.rng.gen_range(0, 4) {
                            0 => Some(Node::Add),
                            1 => Some(Node::Subtract),
                            2 => Some(Node::Multiply),
                            _ => Some(Node::Divide)
                        };
                    } else {
                        node = match self.rng.gen_range(0, 2) {
                            0 => Some(Node::Sine),
                            _ => Some(Node::Cosine)
                        };
                    }
                } else {
                    if self.left(parent_idx).is_none() {
                        match self.parent(parent_idx) {
                            Some(Node::Sine) | Some(Node::Cosine) => (),
                            None => panic!("Lost terminal node.\nself:\n{}\n", self),
                            _ => *self.left(parent_idx) = get_val()
                        };
                    }
                    node = get_op();
                }
                *self.parent(idx) = node; }
        };
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

    /// Alter a Node::Number value from a Vec of indicies to terminal nodes
    fn _mutate_number(&mut self, terminals: &mut Vec<usize>) {
        assert!(terminals.len() > 0,
            "Tried to mutate a number without terminals.\nself:\n{}\n", self);
        let idx = self.rng.gen_range(0, terminals.len());
        let choice = terminals[idx];
        let mut num;
        match self.heap[choice] {
            Some(Node::Variable) => {
                self.heap[choice] = Some(Node::<f32>::Variable);
                return; },
            Some(Node::Number(n)) => { num = n; },
            _ => { panic!("Tried to mutate {:?} at {}.\nself:\n{}",
                self.heap[choice], choice, self) }
        };
        let factor = self.rng.gen_range(-1.5, 1.5);
        let do_add: bool = self.rng.gen();
        num = if do_add {num + factor} else {num * factor};
        // "Rebound" with greater error
        if num > MAX_NUMBER_NODE {
            num -= 1.2 * (num - MAX_NUMBER_NODE);
        } else if num < MIN_NUMBER_NODE {
            num -= 1.2 * (num - MIN_NUMBER_NODE);
        }
        num = num.clamp(MIN_NUMBER_NODE, MAX_NUMBER_NODE);
        self._delete_from_idx(choice); // Ensure it has no children
        self.heap[choice] = Some(Node::Number(num));
    }
    
    /// Alter a random terminal node with a constant, increasing depth
    pub fn mutate_constant(&mut self) {
        let mut terminals: Vec<usize> = self.get_checked_terminals();
        let mut idx: usize;
        if self.depth() < MAX_DEPTH {
            let mut choice: usize;
            let mut max_spawn_depth: u32;
            let mut iter_terminals = terminals.to_vec();
            for _ in 0..terminals.len() {
                idx = self.rng.gen_range(0, iter_terminals.len());
                choice = iter_terminals.swap_remove(idx);
                max_spawn_depth = MAX_DEPTH - depth_from_idx(choice) + 1;
                if max_spawn_depth > 2 {
                    let spawn_depth = self.rng.gen_range(2, max_spawn_depth);
                    self.random_instantiate(choice, spawn_depth);
                    return;
                }
            }
        }
        self._mutate_number(&mut terminals);
    }
    
    /// Replace a random operation node with a terminal node
    pub fn mutate_clip(&mut self) {
        let ops: Vec<usize> = self.get_op_idxs();
        assert!(ops.len() > 1, "Tried to clip the top (operation) node.\nself:\n{}\n", self);
        let choice = ops[self.rng.gen_range(1, ops.len())];
        self.delete_from_idx(choice);
    }
    
    /// Swap two random branches
    pub fn mutate_swap(&mut self) {
        // Get parent node indicies
        let full_depth = self.depth();
        let mut nodes: Vec<usize> = Vec::new();
        for (i, node) in self.heap.iter().enumerate() {
            match node {
                None => (),
                _ => nodes.push(i)
            };
        }
        nodes.swap_remove(0); // Remove the top operation node
        let choice1: usize = nodes.swap_remove(self.rng.gen_range(0, nodes.len()));
        let choice1_depth = self.heap_at_idx(choice1).depth();
        let mut choice2: usize = MAX_IDX + 1;
        for _ in 0..nodes.len() {
            choice2 = nodes.swap_remove(self.rng.gen_range(0, nodes.len()));
            let choice2_depth = self.heap_at_idx(choice2).depth();
            if !check_related(choice1, choice2) &&
                    full_depth - choice1_depth + choice2_depth <= MAX_DEPTH && 
                    full_depth - choice2_depth + choice1_depth <= MAX_DEPTH {
                break;
            }
            choice2 = MAX_IDX + 1;
        }
        if choice2 == MAX_IDX + 1 { // Can't swap, so instead mutate a constant
            self.mutate_constant();
        } else { // Swap starting at the parent node
            self._swap_from(None, choice1, choice2);
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
                Some(Node::Number(n)) => ops.push((i, Node::Number(*n))),
                _ => ()
            };
        }
        let choice: (usize, Node<f32>) = ops[self.rng.gen_range(0, ops.len())];
        match choice.1 {
            Node::Add | Node::Subtract | Node::Multiply |
            Node::Divide | Node::Sine | Node::Cosine =>
                self.heap[choice.0] = Some(choice.1),
            Node::Number(_) =>
                self._mutate_number(&mut vec![choice.0]),
            _ => ()
        };
    }
    
    /// Recurses into child nodes to determine heap's result for variable
    fn _collapse(&mut self, idx: usize, variable: f32) -> f32{
        let left_idx = 2 * idx + 1;
        let right_idx = left_idx + 1;
        let (mut l, mut r) = (f32::MIN_POSITIVE, f32::MIN_POSITIVE);
        if left_idx < MAX_IDX && self.heap[left_idx].is_some() {
            l += self._collapse(left_idx, variable);
        }
        if right_idx <= MAX_IDX && self.heap[right_idx].is_some() {
            r += self._collapse(right_idx, variable);
        }
        match self.heap[idx] {
            Some(Node::Add) => l + r,
            Some(Node::Subtract) => l - r,
            Some(Node::Multiply) => l * r,
            Some(Node::Divide) => if r != 0.0 {l / r} else {l / f32::MIN_POSITIVE},
            Some(Node::Sine) => (l + r).sin(),
            Some(Node::Cosine) => (l + r).cos(),
            Some(Node::Number(n)) => n,
            Some(Node::Variable) => variable,
            _ => 0.0
        }
    }
    
    pub fn collapse(&mut self, variable: f32) -> f32{
        self._collapse(0, variable)
    }
}
