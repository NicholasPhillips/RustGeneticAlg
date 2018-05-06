extern crate rand;
use rand::Rng;
use models;

pub fn seed_list(size:i32) -> Vec<models::Item> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| {
        models::Item {
            weight: rng.gen_range(0, 100),
            value: rng.gen_range(0, 100)
        }
    }).collect::<Vec<_>>()
}