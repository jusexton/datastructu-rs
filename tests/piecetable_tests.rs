use datastructu_rs::piecetable::{PieceTable, PieceTableError};

#[test]
fn test_inserts_to_beginning() {
    let mut piece_table = PieceTable::from("test");
    piece_table.insert(0, "test ").unwrap();
    piece_table.insert(0, "test ").unwrap();

    assert_eq!(piece_table.read(), "test test test");
    assert_eq!(piece_table.len(), 14);
}

#[test]
fn test_inserts_to_end() {
    let mut piece_table = PieceTable::from("test");
    piece_table.insert(4, " test").unwrap();
    piece_table.insert(9, " test").unwrap();

    assert_eq!(piece_table.read(), "test test test");
    assert_eq!(piece_table.len(), 14);
}

#[ignore]
#[test]
fn test_inserts_to_middle() {
    let mut piece_table = PieceTable::from("test");
    piece_table.insert(2, "st te").unwrap();
    piece_table.insert(5, "test ").unwrap();

    assert_eq!(piece_table.read(), "test test test");
    assert_eq!(piece_table.len(), 14);
}

#[test]
fn test_insert_with_index_past_original_text() {
    let mut piece_table = PieceTable::from("test");
    let result = piece_table.insert(10, "test ");

    assert_eq!(result, Err(PieceTableError::IndexOutOfBounds));
    assert_eq!(piece_table.len(), 4);
}

#[test]
fn test_insert_with_empty_text() {
    let mut piece_table = PieceTable::from("test");
    let result = piece_table.insert(0, "");

    assert_eq!(result, Err(PieceTableError::EmptyText));
    assert_eq!(piece_table.len(), 4);
}

#[test]
fn test_default() {
    let piece_table = PieceTable::default();

    assert!(piece_table.is_empty());
}