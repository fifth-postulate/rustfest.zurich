pub fn fact(m: u64) -> u64 {
    (1..m)
        .map(|n| n + 1)
        .fold(1u64, |acc, n| acc * n)
}

fn main() {
    for m in 1..10 {
        println!("{:2}! = {}", m, fact(m));
    }
}
