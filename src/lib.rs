extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate nom;
extern crate rayon;
extern crate petgraph;
extern crate itertools;

pub mod day1;
pub mod day2;

aoc_runner_derive::aoc_lib!{ year = 2021 }
