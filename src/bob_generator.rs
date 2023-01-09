extern crate rand;

use rand::Rng;

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
        BobGenerator {
            list: contents.split("\r\n").into_iter().map(|str| str.to_string()).collect(),
        }
    }

    pub fn pop(&self) -> &String {
       &self.list[rand::thread_rng().gen_range(0, self.list.len())]
    }
}