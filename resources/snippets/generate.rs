#[macro_use]
extern crate permutation_rs;

use std::collections::HashMap;
use permutation_rs::group::GroupElement;
use permutation_rs::group::permutation::Permutation;
use permutation_rs::group::calculation::elements_generated_by;

fn main() {
    let t = permute!(0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 0);
    let s = permute!(0, 2, 1, 1, 2, 0, 3, 3, 4, 4, 5, 5);

    let elements = elements_generated_by(vec!(t, s));
    for element in elements {
        println!("{}", element);
    }
}
