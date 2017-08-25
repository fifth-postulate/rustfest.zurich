use std::collections::HashMap;

fn main() {
    let mut t: HashMap<u64, u64> = HashMap::new();
    t.insert(0, 1);
    t.insert(1, 2);
    t.insert(2, 3);
    t.insert(3, 4);
    t.insert(4, 5);
    t.insert(5, 0);

    let original = 0;
    let image = t.get(&original).unwrap();

    println!("{} -> {}", original, image);
}
