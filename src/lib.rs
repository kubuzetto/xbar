#![crate_name = "xbar"]

//! Rust implementation of the algorithm described in the conference paper
//! _"[A locality preserving one-sided binary tree - crossbar switch wiring
//! design algorithm](https://ieeexplore.ieee.org/document/7086839)"_ published
//! in [2015 49th Annual Conference on Information Sciences and Systems (CISS)]
//! (https://ieeexplore.ieee.org/xpl/mostRecentIssue.jsp?punumber=7075844).
//!
//! > _"__One-sided crossbar switches__ allow for a simple implementation of
//! complete `K_n` graphs. However, designing these circuits is a cumbersome
//! process and can be automated._
//! > _We present an algorithm that allows designing automatic one-sided binary
//! tree - crossbar switches which __do not exceed `floor(n/2)` columns__, and
//! achieves `K_n` graph without connecting any wires between any three adjacent
//! blocks, thus preserving locality in connections."_
//!
//! # Example usage:
//!
//! ```rust
//! use xbar::Crossbar as X;
//! pub fn main() {
//! 	let n = 5;
//! 	println!("Crossbar for {} terminals has {} rows, \
//! 		formed into {} blocks; and {} columns",
//! 		n, X::rows(n), X::blocks(n), X::columns(n));
//! 	println!("Connections of the crossbar:");
//!     for con in X::new(n) {
//! 		println!("{:#?}", con);
//!     }
//! }
//! ```
//! produces the output:
//! ```text
//! Crossbar for 5 terminals has 20 rows, formed into 4 blocks; and 2 columns
//! Connections of the crossbar:
//! Connection {
//!     start: Position {
//!         block_idx: 0,
//!         row_idx: 0,
//!         abs_idx: 0
//!     },
//!     end: Position {
//!         block_idx: 0,
//!         row_idx: 1,
//!         abs_idx: 1
//!     },
//!     col_idx: 0
//! }
//! ...
//! ```
//!
//! # Reference
//!
//! Sahin, Devrim. "A locality preserving one-sided binary tree - crossbar switch
//! wiring design algorithm." _Information Sciences and Systems (CISS), 2015 49th
//! Annual Conference on_. IEEE, 2015.
//!
//! ## Bibtex
//! ```tex
//! @inproceedings{dsahin2015crossbar,
//!   title={A locality preserving one-sided binary tree - crossbar switch wiring design algorithm},
//!   author={{\c{S}}ahin, Devrim},
//!   booktitle={Information Sciences and Systems (CISS), 2015 49th Annual Conference on},
//!   pages={1--4},
//!   year={2015},
//!   organization={IEEE}
//! }
//! ```

use std::cmp::min;

/// A `Position` depicts the location of a row.
#[derive(Debug)]
pub struct Position {
    /// Index of the block that the `Position` is in.
    pub block_idx: usize,
    /// Row offset within the block.
    pub row_idx: usize,
    /// Absolute index. `abs_idx` always
    /// equals to `block_idx * N + row_idx`.
    /// This field is still calculated and
    /// kept for convenience.
    pub abs_idx: usize,
}

/// A `Connection` is a (vertical) link
/// between two rows (terminals).
#[derive(Debug)]
pub struct Connection {
    /// Start position of the connection.
    pub start: Position,
    /// End position of the connection.
    pub end: Position,
    /// Column index of the connection.
    pub col_idx: usize,
}

/// Core struct of the crate.
#[derive(Debug)]
pub struct Crossbar {
    /// Number of terminals.
    pub count: usize,
    inner_idx: usize,
    outer_idx: usize,
}

impl Position {
    fn new(block_idx: usize, row_idx: usize, count: usize) -> Self {
        Self {
            block_idx,
            row_idx,
            abs_idx: row_idx + block_idx * count,
        }
    }
}

impl Crossbar {
    /// Returns an iterator of crossbar connections.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of terminals in the crossbar.
    ///
    /// # Example
    ///
    /// ```rust
    /// let it = xbar::Crossbar::new(10);
    /// std::assert_eq!(45, it.count(),
    /// 	"Function should return an \
    /// 	 iterator with 45 items");
    /// ```
    pub fn new(count: usize) -> Self {
        Self {
            count,
            inner_idx: 0,
            outer_idx: 1,
        }
    }

    /// Returns how many _blocks_ a crossbar
    /// with `count` terminals would have.
    ///
    /// A block is a group of terminals that
    /// contains one of each terminal.
    ///
    /// A crossbar switch with `N` terminals
    /// contains `N - 1` blocks.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of terminals
    /// in the crossbar.
    ///
    /// # Example
    ///
    /// ```rust
    /// std::assert_eq!(9, xbar::Crossbar::blocks(10),
    /// 	"A crossbar switch with 10 terminals contains 9 blocks.");
    /// ```
    #[inline]
    pub fn blocks(count: usize) -> usize {
        count - 1
    }

    /// Returns how many _rows_ a crossbar
    /// with `count` terminals would have.
    ///
    /// A crossbar switch with `N` terminals
    /// contains `N * (N - 1)` rows (`N - 1`
    /// blocks with `N` terminals each).
    ///
    /// # Arguments
    ///
    /// * `count` - Number of terminals
    /// in the crossbar.
    ///
    /// # Example
    ///
    /// ```rust
    /// std::assert_eq!(90, xbar::Crossbar::rows(10),
    /// 	"A crossbar switch with 10 terminals contains 90 rows.");
    /// ```
    #[inline]
    pub fn rows(count: usize) -> usize {
        count * (count - 1)
    }

    /// Returns how many _columns_ a crossbar
    /// with `count` terminals would have.
    ///
    /// A crossbar switch with `N` terminals
    /// contains `floor(N / 2)` rows.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of terminals
    /// in the crossbar.
    ///
    /// # Example
    ///
    /// ```rust
    /// std::assert_eq!(5, xbar::Crossbar::columns(10),
    /// 	"A crossbar switch with 10 terminals contains 5 columns.");
    /// ```
    #[inline]
    pub fn columns(count: usize) -> usize {
        count / 2
    }

    #[inline]
    fn b2i(b: bool) -> usize {
        if b {
            1
        } else {
            0
        }
    }

    #[inline]
    fn full_block_reverse(n: usize, i: usize, j: usize) -> Connection {
        let block = 2 * i - 1;
        let col = if j < 3 * i {
            j % i
        } else {
            i + min(j % i, j + i - n)
        };
        Connection {
            start: Position::new(block, i + j - n, n),
            end: Position::new(block, j, n),
            col_idx: col,
        }
    }

    #[inline]
    fn full_block_forward(n: usize, i: usize, j: usize, is_odd: bool, is_wrap: bool) -> Connection {
        let block = 2 * i - 2;
        Connection {
            start: Position::new(block + Self::b2i(is_odd), j, n),
            end: Position::new(block + Self::b2i(is_odd || is_wrap), (i + j) % n, n),
            col_idx: j % i,
        }
    }

    #[inline]
    fn full_block(n: usize, i: usize, j: usize) -> Connection {
        let is_odd = ((j / i) & 1) == 1;
        let is_wrap = i + j >= n;
        if is_odd && is_wrap {
            Self::full_block_reverse(n, i, j)
        } else {
            Self::full_block_forward(n, i, j, is_odd, is_wrap)
        }
    }

    #[inline]
    fn half_block(n: usize, i: usize, j: usize) -> Connection {
        let block = 2 * i - 2;
        Connection {
            start: Position::new(block, j, n),
            end: Position::new(block, i + j, n),
            col_idx: j,
        }
    }

    #[inline]
    fn step(&mut self, inner_lim: usize) {
        self.inner_idx += 1;
        if self.inner_idx >= inner_lim {
            self.inner_idx = 0;
            self.outer_idx += 1;
        }
    }
}

impl Iterator for Crossbar {
    type Item = Connection;
    fn next(&mut self) -> Option<Self::Item> {
        let rem = 2 * self.outer_idx;
        if rem > self.count {
            None
        } else if rem == self.count {
            let o = self.outer_idx;
            let conn = Self::half_block(self.count, o, self.inner_idx);
            self.step(o);
            Some(conn)
        } else {
            let c = self.count;
            let conn = Self::full_block(c, self.outer_idx, self.inner_idx);
            self.step(c);
            Some(conn)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        for n in 1..100 {
            println!("testing n = {}", n);
            let w = n * (n - 1);
            let mut buf: Vec<Vec<i32>> = vec![vec![0; w]; n / 2];
            let mut mat: Vec<Vec<i32>> = vec![vec![0; n]; n];
            for (i, row) in mat.iter_mut().enumerate().take(n) {
                row[i] = 1;
            }
            for val in Crossbar::new(n) {
                assert_eq!(
                    val.start.abs_idx,
                    val.start.row_idx + n * val.start.block_idx
                );
                assert_eq!(val.end.abs_idx, val.end.row_idx + n * val.end.block_idx);
                mat[val.start.row_idx][val.end.row_idx] += 1;
                mat[val.end.row_idx][val.start.row_idx] += 1;
                for k in val.start.abs_idx..val.end.abs_idx {
                    assert_eq!(buf[val.col_idx][k], 0);
                    buf[val.col_idx][k] += 1;
                }
                for row in buf.iter().take(val.col_idx) {
                    let mut empty_line = true;
                    for elem in row.iter().take(val.end.abs_idx).skip(val.start.abs_idx) {
                        if elem != &0 {
                            empty_line = false;
                            break;
                        }
                    }
                    assert_eq!(empty_line, false);
                }
            }
            for row in mat.iter().take(n) {
                for elem in row.iter().take(n) {
                    assert_eq!(elem, &1);
                }
            }
        }
    }
}
