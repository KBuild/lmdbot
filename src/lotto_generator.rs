use rand::{seq::SliceRandom, thread_rng};

extern crate rand;

pub struct LottoGenerator {}

impl serenity::prelude::TypeMapKey for LottoGenerator {
    type Value = LottoGenerator;
}


impl LottoGenerator {
    pub fn new() -> LottoGenerator {
        LottoGenerator {}
    }

    // 동행복권 6/45
    pub fn choose_lotto_6_45(&self) -> String {
        let mut vec: Vec<u8> = (1..46).collect();
        vec.shuffle(&mut thread_rng());
        format!("Lotto Number => {}, {}, {}, {}, {}, {}", vec[0], vec[1], vec[2], vec[3], vec[4], vec[5])
    }
}