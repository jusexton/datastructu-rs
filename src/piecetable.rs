use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum PieceTableError {
    IndexOutOfBounds,
    EmptyText,
}

enum Source {
    Original,
    Add,
}

struct Piece {
    start: usize,
    length: usize,
    source: Source,
}

pub struct PieceTable {
    original: String,
    additional: String,
    pieces: VecDeque<Piece>,
    length: usize,
}

impl PieceTable {
    /// Creates a new empty PieceTable.
    pub fn new() -> Self {
        Self {
            original: String::new(),
            additional: String::new(),
            pieces: VecDeque::new(),
            length: 0,
        }
    }

    pub fn read(&self) -> String {
        self.pieces.iter()
            .map(|piece| {
                let start = piece.start;
                let end = piece.start + piece.length;
                match piece.source {
                    Source::Original => { &self.original[start..end] }
                    Source::Add => { &self.additional[start..end] }
                }
            })
            .collect::<String>()
    }

    pub fn insert(&mut self, index: usize, text: &str) -> Result<(), PieceTableError> {
        self.valid_insert(index, text)?;

        let piece = self.new_piece(text);
        if index == 0 {
            self.pieces.push_front(piece);
        } else if index == self.length {
            self.pieces.push_back(piece);
        } else {
            todo!("Middle insertion is not yet supported")
        }
        self.additional.push_str(text);
        self.length += text.len();

        Ok(())
    }

    fn valid_insert(&self, index: usize, text: &str) -> Result<(), PieceTableError> {
        if index > self.length {
            return Err(PieceTableError::IndexOutOfBounds);
        }
        if text.is_empty() {
            return Err(PieceTableError::EmptyText);
        }

        Ok(())
    }

    fn new_piece(&mut self, text: &str) -> Piece {
        Piece {
            start: self.additional.len(),
            length: text.len(),
            source: Source::Add,
        }
    }

    /// Returns the length of textual representation.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns true if the text representation is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl From<String> for PieceTable {
    /// Creates a new PieceTable from an existing String.
    fn from(s: String) -> Self {
        let length = s.len();
        Self {
            original: s,
            additional: String::new(),
            pieces: VecDeque::from([
                Piece {
                    start: 0,
                    length,
                    source: Source::Original,
                }
            ]),
            length,
        }
    }
}

impl From<&str> for PieceTable {
    /// Creates a new PieceTable from an existing &str.
    fn from(s: &str) -> Self {
        PieceTable::from(s.to_owned())
    }
}

impl Default for PieceTable {
    /// Creates an empty PieceTable.
    #[inline]
    fn default() -> Self {
        PieceTable::new()
    }
}
