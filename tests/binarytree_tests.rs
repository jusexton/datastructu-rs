use datastructu_rs::binarytree::BinaryTree;

#[test]
fn test_inserts() {
    let mut binary_tree = BinaryTree::new();

    binary_tree.insert(1);
    binary_tree.insert(2);
    binary_tree.insert(3);

    assert_eq!(binary_tree.len(), 3)
}

#[test]
fn test_contains() {
    let mut binary_tree = BinaryTree::new();
    binary_tree.insert(1);
    binary_tree.insert(2);
    binary_tree.insert(3);

    assert!(binary_tree.contains(2));
    assert!(!binary_tree.contains(5));

    binary_tree.insert(4);
    binary_tree.insert(5);

    assert!(binary_tree.contains(5));
    assert_eq!(binary_tree.len(), 5)
}

#[test]
fn test_clear() {
    let mut binary_tree = BinaryTree::new();
    binary_tree.insert(1);
    binary_tree.insert(2);
    binary_tree.insert(3);

    binary_tree.clear();

    assert!(binary_tree.is_empty())

}