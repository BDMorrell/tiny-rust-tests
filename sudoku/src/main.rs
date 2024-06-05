#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
//! A hopefully quick Sudoku solver.
//!
//! Notes to self:
//!
//! * I heard that Sudoku puzzle solving is so sipmle that it is hard to program something that is faster than
//!   brute-force, so all overhead is bad.
//! * Try to avoid conditional brnaching.
//! * Try to write test code (maybe even just documentation.)
//!
//! # NOTE
//! There may be a good argument to have Sudoku have values `1..=9`, instead of `1..=8`. This would change:
//!   * [`b9`] would still be `u16`, and bit 0 could be a padding bit. This would keep left-shifting a 1 by the value
//!     as a viable bit selector.
//!     * [`B9_MASK`] would be suffixed with the new padding bit.
//!   * A [`SudokuState::cells`] could have it's 'empty' value be 0. (And a zero flag is a thing processors have.) This
//!     would probably make more sense, and I could even have Rust's `None` by using a `std::num::NonZeroI8`. for
//!     [`SudokuState::cells`].
//!     * This would also allow bit-packing two cells into a single byte, thoubh if I use an `Option`, it would be
//!       unsafe, and I'd have to add tests to make sure that the representation I'm expecting it to be is the same as
//!       the actual compiled representation. Though this can get bad if using optimizations or not changes the
//!       representation.
//!   * If I'm still using the bitmask to represent what's already in the set, than this could make the whole empty
//!   default be a bunch of zeros! If I'm feeling like using unsafe code, I could initalize the value with
//!   `std::mem::zeroed`.

/// 9-bit-wide bitflag set
///
/// Type that contians bitflags for the potential or exstance of Sudoku values `0..=8`, stored as `1 << [0..=8]`
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
///
/// This structure is over a hundred bytes, which seems a bit large for it to be implementing the [`Copy`] trait.
#[derive(Clone, Copy, Debug)]
pub struct SudokuState {
    /// The values of the cells, or a value of \[TODO\] if the cell has not yet been deduced.
    ///
    /// TODO:
    /// * an enum (like `Optional`) could be a good idea…
    /// * if I'm not using an enum, than what sould be the set of all valid values?
    ///   * it might be useful to have Sudoku have symbols 0...=8. (This requires a minimum of 4 bits; where 9..16 are
    ///     free for other use.) If I use symbols 0..=8, than it seems logical to use either 9 or 0xF to represent the
    ///     "empty" (unset) value.
    ///     * Right now, "empty" is represented using 0xF. This is done in [`BLANK_SUDOKU_STATE`].
    /// * Should I pack the cell values, so that I use packed `u4`s?
    cells: [u8; 81],
    /// Consiss of 3 `[b9; 9]` sets for the
    /// 1. rows (`idx` ∈ [0..9) )
    /// 2. columns (`idx` ∈ [9..18) )
    /// 3. boxes (`idx` ∈ [18..27) )
    ///
    /// The compliment of the union described is the possible values that a cell can have. This 'negative logic' may
    /// change if it is too confusing or is less useful in bitwise operation. This is decision reflected in
    /// [`BLANK_SUDOKU_STATE`].
    ///
    /// The union of these three sets represents the unavailability (or prior existence of) finalized cells.
    ///
    /// TODO: make lookup tables for indicies, or see if doing the maths to generate the values is faster.
    sets: [b9; 27],
}

/// The state of a blank sudoku board, to be sent with [`Default::default()`].
///
/// The reasoning for the values provided is in the documentation for [`SudokuState::cells`] and [`SudokuState::sets`].
const BLANK_SUDOKU_STATE: SudokuState = SudokuState {
    sets: [0; 27],
    cells: [0x0F; 81],
};

impl Default for SudokuState {
    fn default() -> Self {
        BLANK_SUDOKU_STATE
    }
}

/// The function to invoke to start this program.
fn main() {
    println!("Hello, world!");
}
