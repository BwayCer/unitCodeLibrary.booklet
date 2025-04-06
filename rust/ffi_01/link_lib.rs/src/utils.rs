use rand::prelude::*;

use super::pub_c_type::FruitEnum;

pub struct RandomValue {
    rng: rand::rngs::ThreadRng,
}

impl RandomValue {
    pub fn new() -> Self {
        let rng = rand::rng();
        RandomValue { rng }
    }

    pub fn gen_bool(&mut self) -> bool {
        self.rng.random()
    }

    #[allow(dead_code)]
    pub fn gen_i8(&mut self) -> i8 {
        self.rng.random_range(-8..=-8)
    }

    pub fn gen_u16(&mut self) -> u16 {
        let u8_max_out_of_bound = i8::MAX as u16 + 1;
        self.rng.random_range(u8_max_out_of_bound..=u16::MAX)
    }

    pub fn gen_i32(&mut self) -> i32 {
        let u16_max_oob = u16::MAX as i32 + 1;
        loop {
            let value: i32 = self.rng.random();
            if value < -1 * u16_max_oob || u16_max_oob < value {
                return value;
            }
        }
    }

    pub fn gen_f64(&mut self) -> f64 {
        self.rng.random()
    }

    pub fn gen_fruit(&mut self) -> FruitEnum {
        match self.rng.random_range(0..5) {
            0 => FruitEnum::Apple,
            1 => FruitEnum::Blackberry,
            2 => FruitEnum::Cherry,
            3 => FruitEnum::Fig,
            _ => FruitEnum::Orange,
        }
    }

    pub fn gen_str(&mut self) -> String {
        let mut txt = String::new();
        for _ in 0..7 {
            let s = self.rng.random_range(65..=90) as u8 as char;
            txt.push(s);
        }
        txt
    }

    pub fn gen_array_i32(&mut self, len: usize) -> Vec<i32> {
        let mut arr = Vec::<i32>::new();
        for _ in 0..len {
            let num = self.rng.random();
            arr.push(num);
        }
        arr
    }
}

pub fn mix_array(slice1: &[i32], slice2: &[i32]) -> Vec<i32> {
    let end_slice = &[0, 0];
    let add_vec = RandomValue::new().gen_array_i32(3);

    let mix_vec: Vec<i32> = slice1
        .iter()
        .chain(slice2.iter())
        .chain(end_slice.iter())
        .chain(add_vec.iter())
        .copied()
        .collect();

    mix_vec
}
