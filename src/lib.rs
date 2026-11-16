//! Braveheart's self-play data: datagen plays the engine against itself and
//! stores games and positions, relabel re-scores stored positions, and the
//! datagen audit checks a generation before anything trains on it.
#![warn(missing_docs)]

pub mod datagen;
pub mod relabel;
pub mod rng;
