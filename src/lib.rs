//! A blazingly fast Chess Engine and Chess AI.
//!
//! This package is seperated into two parts. Firstly, the board representation & associated functions. and Secondly,
//! the AI implementations.
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/pleco) and can be
//! used by adding `pleco` to the dependencies in your project's `Cargo.toml`.
//!
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(test, allow(dead_code))]

#![feature(integer_atomics)]
#![feature(test)]
#![allow(dead_code)]
#![feature(integer_atomics)]
#![feature(unique)]
#![feature(allocator_api)]


#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

extern crate test;
extern crate rayon;
extern crate num_cpus;
extern crate rand;

pub mod board;

pub mod timer;
pub mod engine;
pub mod tools;
pub mod uci;
pub mod tt;
pub mod bot_prelude;


pub mod core;

pub mod bots;


pub use board::Board;
pub use core::piece_move::BitMove;
pub use core::templates::{Player,Piece,BitBoard,SQ};


