mod generator;
mod models;
extern crate rand;

fn main() {
    let citizens = (0..10).map(|_| { 
        models::Citizen {genome: generator::seed_list(10)}
    }).collect::<Vec<_>>();
    println!("{:?}", citizens);
    println!("{:?}", citizens[0].fitness());
}