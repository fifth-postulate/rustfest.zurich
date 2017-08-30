#[macro_use]
extern crate permutation_rs;

use std::collections::HashMap;
use permutation_rs::group::GroupElement;
use permutation_rs::group::permutation::Permutation;

fn main() {
    let p = permute!(0, 1, 1, 0, 2, 2);
    let q = permute!(0, 0, 1, 2, 2, 1);

    let product = p.times(&q);

    println!("{} * {} = {}", p, q, product);
}
