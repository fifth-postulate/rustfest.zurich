extern crate permutation_rs;

use permutation_rs::group::GroupElement;
use permutation_rs::group::free::Word;

fn main() {
    let t = Word::generator('t');
    let s = Word::generator('s');

    let product = t.times(&s).times(&t.inverse().times(s.invers()));

    println!("{}", product);
}
