use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Node {
    is_terminal: bool,
    children: HashMap<char, Node>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            is_terminal: false,
            children: HashMap::new(),
        }
    }
}

pub struct Trie {
    root: Node,
    length: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node::new(),
            length: 0,
        }
    }

    pub fn insert(&mut self, value: &str) -> bool {
        let mut current_node = &mut self.root;
        let mut new_value = false;
        for c in value.chars() {
            current_node = match current_node.children.entry(c) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    new_value = true;
                    entry.insert(Node::new())
                }
            };
        }

        if new_value {
            current_node.is_terminal = true;
            self.length += 1;
        }
        new_value
    }

    pub fn exact_match(&self, value: &str) -> bool {
        self.traverse_to(value).map_or(false, |node| node.is_terminal)
    }

    fn traverse_to(&self, value: &str) -> Option<&Node> {
        let mut current_node = &self.root;

        for c in value.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return None,
            }
        }

        Some(current_node)
    }

    pub fn prefix_match<'a>(&'a self, prefix: &'a str) -> PrefixMatches {
        PrefixMatches::new(prefix, &self)
    }

    /// Returns true if no entries have been added.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of terminal nodes in the collection. In other words, the number
    /// of entries inserted into the collection.
    pub fn len(&self) -> usize {
        self.length
    }
}

pub struct PrefixMatches<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> PrefixMatches<'a> {
    fn new(prefix: &'a str, trie: &'a Trie) -> Self {
        let starting_node = trie.traverse_to(prefix);
        let stack = match starting_node {
            Some(node) => Vec::from([node]),
            None => Vec::new()
        };
        PrefixMatches { stack }
    }
}

impl<'a> Iterator for PrefixMatches<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        None
    }
}

impl<'a> FromIterator<&'a str> for Trie {
    fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
        let mut trie = Trie::new();
        for value in iter {
            trie.insert(value);
        }
        trie
    }
}