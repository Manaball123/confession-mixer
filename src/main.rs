mod core;
use std::collections::HashMap;

fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    let mut inst = core::instance::Instance::new();
    inst.add_peer("A");
    inst.add_peer("B");
    inst.add_peer("C");
    inst.update_peer("A", &HashMap::from([("B".to_owned(), 1)]));
    inst.update_peer("B", &HashMap::from([("A".to_owned(), 1)]));
    inst.update_peer("C", &HashMap::from([("B".to_owned(), 1)]));
    println!("{:?}", inst.get_optimal_pairs())
}