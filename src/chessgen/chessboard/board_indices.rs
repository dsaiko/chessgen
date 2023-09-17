use crate::bitboard::Index;

use super::{Color, Piece};

/// Conversion of Color to an array index.
impl<T> std::ops::Index<Color> for [T] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

/// Conversion of Color to an array index.
impl<T> std::ops::IndexMut<Color> for [T] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

/// Conversion of Piece to an array index.
impl<T> std::ops::Index<Piece> for [T] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

/// Conversion of Piece to an array index.
impl<T> std::ops::IndexMut<Piece> for [T] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

/// Conversion of Index to an array index.
impl<T> std::ops::Index<Index> for [T] {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        &self[index.index]
    }
}

/// Conversion of Index to an array index.
impl<T> std::ops::IndexMut<Index> for [T] {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self[index.index]
    }
}

/// Conversion of Index to a Vec index.
impl<T> std::ops::Index<Index> for Vec<T> {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        &self[index.index]
    }
}

/// Conversion of Index to a Vec index.
impl<T> std::ops::IndexMut<Index> for Vec<T> {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self[index.index]
    }
}

/// Conversion of Color to a Vec index.
impl<T> std::ops::Index<Color> for Vec<T> {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

/// Conversion of Color to a Vec index.
impl<T> std::ops::IndexMut<Color> for Vec<T> {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

/// Conversion of Piece to a Vec index.
impl<T> std::ops::Index<Piece> for Vec<T> {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

/// Conversion of Piece to a Vec index.
impl<T> std::ops::IndexMut<Piece> for Vec<T> {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
