#[derive(Debug)]
pub struct Citizen {
    pub genome: Vec<Item>
}

trait Fitness {
    fn fitness(&self) -> i32;
}

impl Citizen {
    pub fn fitness(&self) -> i32 {
        self.genome.iter().map(|g| {
            g.value
        }).fold(0, |a, b:i32| a + b)
    }
}

#[derive(Debug)]
pub struct Item {
    pub weight: i32,
    pub value: i32
}