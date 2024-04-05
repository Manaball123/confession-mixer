mod core;
use std::collections::HashMap;
mod utils;



fn main() {


    let mut inst = core::instance::Instance::new();
    println!("Enter number of peers: ");
    let n_peers : i32 = utils::get_input().trim().parse::<i32>().expect("Invalid input");
    for i in 0..n_peers {
        println!("Enter name of peer number {0}", i + 1);
        
        inst.add_peer(utils::get_input().trim());
    }
    for (peer_name, peer_instance) in inst.peers.iter_mut(){
        utils::cls();
        println!("{0}, enter the name of ur love interest or \"none\" if u dont have any(imagine)", &peer_name);
        let input = utils::get_input();
        if input.trim() == "none"{
            continue;
        }
        peer_instance.set_opinions(&HashMap::from([(input.trim().to_owned(), 1)]))
    }

    println!("{:?}", inst.get_optimal_pairs())
}