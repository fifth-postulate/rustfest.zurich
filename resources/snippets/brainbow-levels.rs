#[macro_use]
extern crate permutation_rs;

use std::collections::HashMap;
use permutation_rs::group::{GroupElement, GroupAction, BaseStrongGeneratorLevel};
use permutation_rs::group::permutation::Permutation;

fn main() {
    let t = permute!(0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 0);
    let s = permute!(0, 2, 1, 1, 2, 0, 3, 3, 4, 4, 5, 5);

    let level = BaseStrongGeneratorLevel::new(0, vec!(t, s));

    let u = permute!(0, 0, 1, 3, 2, 2, 3, 1, 4, 4, 5, 5);
    let v = permute!(0, 0, 1, 1, 2, 4, 3, 3, 4, 2, 5, 5);
    let w = permute!(0, 0, 1, 1, 2, 2, 3, 5, 4, 4, 5, 3);
}
