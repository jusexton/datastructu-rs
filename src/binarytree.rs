struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BinaryTree<T: PartialOrd> {
    root: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None, length: 0 }
    }

    pub fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => BinaryTree::raw_insert(node, value),
            None => self.root = Some(BinaryTree::new_node(value))
        }
        self.length += 1;
    }

    fn raw_insert(current: &mut Box<Node<T>>, value: T) {
        if current.value == value { return; }

        let target = if value > current.value { &mut current.right } else { &mut current.left };
        match target {
            Some(target) => BinaryTree::raw_insert(target, value),
            None => *target = Some(BinaryTree::new_node(value))
        }
    }

    fn new_node(value: T) -> Box<Node<T>> {
        Box::new(Node::new(value))
    }

    pub fn contains(&self, value: T) -> bool {
        match &self.root {
            Some(node) => BinaryTree::raw_search(node, value),
            None => false
        }
    }

    fn raw_search(current: &Box<Node<T>>, value: T) -> bool {
        if current.value == value { return true; }

        let target = if value > current.value { &current.right } else { &current.left };
        match target {
            Some(target) => BinaryTree::raw_search(target, value),
            None => false
        }
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.length = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }
}


impl<T: PartialOrd> Default for BinaryTree<T> {
    fn default() -> Self {
        BinaryTree::new()
    }
}