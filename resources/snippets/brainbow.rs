#[macro_use]
extern crate permutation_rs;

use std::collections::HashMap;
use permutation_rs::group::{GroupElement, GroupAction, Group};
use permutation_rs::group::permutation::Permutation;

fn main() {
    let t = permute!(0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 0);
    let s = permute!(0, 2, 1, 1, 2, 0, 3, 3, 4, 4, 5, 5);

    let brainbow = Group::new(vec!(0, 1, 2, 3, 4, 5), vec!(t, s));

    let state = permute!(0, 3, 1, 0, 2, 5, 3, 4, 4, 1, 5, 2);

    println!("permutation is{} a member", if !brainbow.is_member(&state) {" not"} else {""});
}
