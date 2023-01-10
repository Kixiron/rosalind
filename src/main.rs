#![feature(portable_simd, vec_into_raw_parts)]

mod dna;
mod fasta;
mod problems;

use crate::dna::Dna;
use std::{env, fs};

fn main() {
    let dataset = env::args().nth(1).unwrap();

    let data = fs::read_to_string(dataset).unwrap();

    // let (first, second) = data.split_once('\n').unwrap();
    // let first = Dna::from_str(first.trim());
    // let second = Dna::from_str(second.trim());

    let dataset = fasta::parse_fasta(&data);
    problems::gc::run(&dataset);
}
