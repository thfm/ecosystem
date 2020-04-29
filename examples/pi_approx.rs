use ecosystem::{Ecosystem, Organism};
use rand::Rng;

struct PiApproximator {
    value: f64,
}

impl Organism for PiApproximator {
    fn fitness(&self) -> f64 {
        let diff = (std::f64::consts::PI - self.value).abs();
        1.0 / diff
    }

    fn breed(&self, other: &Self) -> Self {
        Self {
            value: (self.value + other.value) / 2.0,
        }
    }

    fn mutate(&mut self, rate: f64) {
        let change = rand::thread_rng().gen_range(-rate, rate);
        self.value += change;
    }
}

const POPULATION_COUNT: u32 = 10;
const GENERATIONS: u32 = 50;
const MUTATION_RATE: f64 = 0.1;

const MAX_INITIAL_VALUE: f64 = 10.0;

fn main() {
    let approximators: Vec<PiApproximator> = (0..POPULATION_COUNT)
        .map(|_| {
            let mut rng = rand::thread_rng();
            PiApproximator {
                value: rng.gen_range(-MAX_INITIAL_VALUE, MAX_INITIAL_VALUE),
            }
        })
        .collect();
    let mut ecosystem = Ecosystem::new(approximators);
    for _ in 0..GENERATIONS {
        ecosystem.breed_next_generation(MUTATION_RATE);
        println!("{}", ecosystem.fittest().value);
    }
}
