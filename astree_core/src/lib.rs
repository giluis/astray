#![feature(let_chains)]

pub mod parse;
pub mod iter;
pub mod impl_std;
pub mod error;
// mod reddit_post;

pub use parse::*;
pub use iter::*;
pub use impl_std::*;
pub use error::*;

#[cfg(test)]
mod tests_iter;

