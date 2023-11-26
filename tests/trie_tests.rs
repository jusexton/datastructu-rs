use datastructu_rs::trie::Trie;

#[test]
fn test_insert() {
    let trie = Trie::from_iter(["test", "text"]);

    assert_eq!(trie.len(), 2);
    assert!(trie.exact_match("test"));
    assert!(trie.exact_match("text"));
}

#[ignore]
#[test]
fn test_prefix_match() {
    let trie = Trie::from_iter(["test", "text"]);

    let matches = trie.prefix_match("te");
    assert_eq!(matches.count(), 2)
}

#[test]
fn test_is_empty() {
    let trie = Trie::new();

    assert!(trie.is_empty())
}