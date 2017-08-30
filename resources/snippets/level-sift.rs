#[macro_use]
extern crate permutation_rs;

use std::collections::HashMap;
use permutation_rs::group::{GroupElement, GroupAction};
use permutation_rs::group::permutation::Permutation;

fn main() {
    let base = 0;
    let mut transversal = HashMap::new();
    transversal.insert(0, permute!(0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5));
    transversal.insert(1, permute!(0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 0));
    transversal.insert(2, permute!(0, 2, 1, 3, 2, 4, 3, 5, 4, 0, 5, 1));
    transversal.insert(3, permute!(0, 3, 1, 4, 2, 5, 3, 0, 4, 1, 5, 2));
    transversal.insert(4, permute!(0, 4, 1, 5, 2, 0, 3, 1, 4, 2, 5, 3));
    transversal.insert(5, permute!(0, 5, 1, 0, 2, 1, 3, 2, 4, 3, 5, 4));

    let state = permute!(0, 3, 1, 0, 2, 5, 3, 4, 4, 1, 5, 2);

    let image = state.act_on(&base);

    println!("{}^{} = {}", &base, &state, &image);
}
