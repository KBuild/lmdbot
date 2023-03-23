extern crate rand;

use rand::seq::SliceRandom;

impl serenity::prelude::TypeMapKey for BobGenerator {
    type Value = BobGenerator;
}

pub struct BobGenerator {
    list: Vec<String>,
}

impl BobGenerator {
    pub fn new(file_path: &str) -> Self {
        let contents = std::fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let bob_list = contents.lines().into_iter().filter(|str| !str.is_empty()).map(|str| str.to_string()).collect();
        
        #[cfg(debug_assertions)]
        println!("Now I got a list of bobs: {:?}", bob_list);

        BobGenerator {
            list: bob_list,
        }
    }

    pub fn pop(&self) -> &String {
        self.list.choose(&mut rand::thread_rng()).unwrap()
    }
}
