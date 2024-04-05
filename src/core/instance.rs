use std::collections::HashMap;
use std::collections::HashSet;

use super::peer::*;

type ResultVec = Vec<(String, String)>;

pub struct Instance {
    pub peers : HashMap<String, Peer>
}

impl Instance {
    pub fn new() -> Self {
        Self { 
            peers : HashMap::new()
        }
    }

    pub fn add_peer(&mut self, name : &str) -> () {
        self.peers.insert(name.to_owned(), Peer::new(name));
    }
    pub fn update_peer(&mut self, name : &str, opinions : &OpinionsMap) -> () {
        if !self.peers.contains_key(name){
            return;
        }
        //or insert should never be invoked, i just dont know how to fix
        let cur_peer = self.peers.get_mut(name).unwrap();
        cur_peer.set_opinions(opinions);
    }

    pub fn get_optimal_pairs(&self) -> ResultVec{
        let mut result = ResultVec::new();
        let mut seen : HashSet<String> = HashSet::new();
        for (peer_name, peer_instance) in self.peers.iter(){
            seen.insert(peer_name.to_owned());
            for(other_name, peer_to_other_val) in peer_instance.opinions.iter(){
                if seen.contains(other_name){
                    continue;
                }
                let other_to_peer_val = self.peers[other_name].get_opinion(&peer_name);
                if peer_to_other_val * other_to_peer_val > 0 {
                    result.push((peer_name.clone(), other_name.clone()));
                }
            }
        }
        return result;
    }
    
}