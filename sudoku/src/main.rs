#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
//! A hopefully quick Sudoku solver.
//!
//! Notes to self:
//!
//! * I heard that SUdoku puzzle solving is so sipmle that it is hard to program something that is faster than brute force, so all overhead is bad.
//! * Try to avoid conditional brnaching.
//! * Try to write test code (maybe even just documentation.)

/// 9-bit-wide bitflag set
///
/// Type that contians bitflags for the potential or exstance of Sudoku values `0..=8`, stored as `2 << [0..=8]`
///
///
/// # TODO:
/// In Sudoku, every cell is an element for a row, column, and a box.
#[allow(non_camel_case_types)]
pub type b9 = u16;

/// The set of all of the bits used for the `b9` type.
///
/// Every `b9` should equal itself `AND [B9_MASK]`.
pub const B9_MASK: b9 = 0b_111_111_111;

/// A data structure specifically designed to help solve Sudoku puzzles.
///
/// Setting the value of a cell should also update the three relevant sets.
/// (TODO: check to see if there can be a simple implementation for `self.unmark_cell(...)`)
pub struct SudokuState {
    /// The values of the cells, or a value of \[TODO\] if the cell has not yet been deduced.
    ///
    /// TODO:
    /// * an enum (like `Optional`) could be a good idea…
    /// * Should I pack the cell values, so that I use `u4`s?
    cells: [u8; 81],
    /// Consiss of 3 `[b9; 9]` sets for the
    /// 1. rows (`idx` ∈ [0..9) )
    /// 2. columns (`idx` ∈ [9..18) )
    /// 3. boxes (`idx` ∈ [18..27) )
    ///
    /// The union of these three sets represents the unavailability (or prior existence of) finalized cells.
    /// The compliment of the union described is the possible values that a cell can have.
    ///
    /// TODO: make lookup tables for indicies, or see if doing the maths to generate the values is faster.
    sets: [b9; 27],
}

/// The function to invoke to start this program.
fn main() {
    println!("Hello, world!");
}
