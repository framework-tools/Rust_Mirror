use std::{fmt::Debug, ops::Range};

#[derive(Debug, PartialEq, Clone)]
pub struct TextBlock(pub StringUTF16);

#[derive(Debug, PartialEq, Clone)]
pub struct StringUTF16(pub Vec<u16>);

impl StringUTF16 {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn from_str(str: &str) -> Self {
        return Self(str.encode_utf16().collect())
    }

    pub fn to_string(self) -> String {
        return String::from_utf16(&self.0).unwrap()
    }

    pub fn splice(&mut self, range: Range<usize>, slice: Self) {
        self.0.splice(range, slice.0);
    }

    pub fn slice(&self, range: Range<usize>) -> Self {
        return Self(self.0[range].to_vec())
    }

    pub fn split(&self, index: usize) -> (Self, Self) {
        return (self.slice(0..index), self.slice(index..self.len()))
    }

    pub fn len(&self) -> usize {
        return self.0.len()
    }

    pub fn concat(self, add: Self) -> Self {
        return Self(vec![self.0, add.0].concat())
    }
}
