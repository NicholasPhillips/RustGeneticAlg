extern crate rand;
use rand::Rng;
use models;

pub fn seed_list(size:i32) -> Vec<models::Item> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| {
        let items = items();
        items[rng.gen_range(0, 9)]
    }).collect::<Vec<_>>()
}

pub fn items() -> Vec<models::Item> {
    vec![
     models::Item {weight:12,value:8},
        models::Item {weight:7,value:7},
        models::Item {weight:4,value:1},
        models::Item {weight:9,value:5},
        models::Item {weight:10,value:12},
        models::Item {weight:3,value:3},
        models::Item {weight:2,value:3},
        models::Item {weight:2,value:2},
        models::Item {weight:5,value:10}]
}