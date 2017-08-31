#[macro_use]
extern crate permutation_rs;

use std::collections::HashMap;
use permutation_rs::group::{Group, GroupElement, Morphism};
use permutation_rs::group::permutation::Permutation;
use permutation_rs::group::free::Word;
use permutation_rs::group::tree::SLP;
use permutation_rs::group::special::SLPPermutation;

fn main() {
    let t = permute!(0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 0);
    let s = permute!(0, 2, 1, 1, 2, 0, 3, 3, 4, 4, 5, 5);

    let p = SLPPermutation::new(SLP::Generator(0), t);
    let q = SLPPermutation::new(SLP::Generator(1), s);

    let brainbow = Group::new(vec!(0, 1, 2, 3, 4, 5), vec!(p, q));

    let state = permute!(0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5);
    let r = SLPPermutation::new(SLP::Identity, state);

    let stripped = brainbow.strip(r);

    let morphism = morphism!(0, 't', 1, 's');

    println!("{}", stripped.transform(&morhpism).inverse());

}
