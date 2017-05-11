//! # Quackin - API documentation
//!
//! Hi, welcome to the Quackin API documentation!
//!
//! Quackin is a recommender systems framework written in Rust focused on:
//!
//! - Facilitate data handling.
//! - Providing collaborative filtering algorithms.
//! - Being an environment to build, test and evaluate new algorithms.
//!
//! Until we get a website for the crate, this docs will be the primary
//! reference for both technical and user-side aspects of Quackin.
//!
//! Quackin was called Oozie as a reference to Apache Mahout, but then I
//! discovered that Apache Oozie already exists. Now is called Quackin because
//! I wanted a funny name and my debugger rubber duck was on my desk at that
//! moment.
//!
//! ## Usage
//! To start using Oozie just add it as a dependency to your `Cargo.toml` file:
//!
//! ```ignore
//! [dependencies]
//! quackin = "*"
//! ```
//!
//! ## Submodules
//!
//! Quackin is divided in the following submodules:
//! 1. `data`: to read and handle data
//! 2. `recommender`: to build recommenders
//! 3. `metrics`: to measure similarities, performance, etc.

extern crate csv;
extern crate rustc_serialize;

pub mod data;
