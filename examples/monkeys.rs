use ecosystem::{Ecosystem, Organism};
use rand::{seq::SliceRandom, Rng};

const LETTERS: &[char] = &[
    ' ', '!', ',', '.', '?', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

struct Monkey {
    phrase: String,
}

impl Monkey {
    const TARGET_PHRASE: &'static str = "To be or not to be?";

    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            phrase: (0..Self::TARGET_PHRASE.len())
                .map(|_| *LETTERS.choose(&mut rng).unwrap())
                .collect(),
        }
    }
}

impl Organism for Monkey {
    fn fitness(&self) -> f64 {
        let fitness: u32 = self
            .phrase
            .chars()
            .zip(Self::TARGET_PHRASE.chars())
            .map(|(guess, target)| if guess == target { 1 } else { 0 })
            .sum();
        f64::from(fitness.pow(2))
    }

    fn breed(&self, other: &Self) -> Self {
        let partition = rand::thread_rng().gen_range(0, self.phrase.len());
        Self {
            phrase: format!(
                "{}{}",
                &self.phrase[..partition],
                &other.phrase[partition..]
            ),
        }
    }

    fn mutate(&mut self, rate: f64) {
        let mut rng = rand::thread_rng();
        self.phrase = self
            .phrase
            .chars()
            .map(|letter| {
                if rng.gen_bool(rate) {
                    *LETTERS.choose(&mut rng).unwrap()
                } else {
                    letter
                }
            })
            .collect();
    }
}

const POPULATION_COUNT: u32 = 500;
const MUTATION_RATE: f64 = 0.01;

fn main() {
    let monkeys: Vec<Monkey> = (0..POPULATION_COUNT).map(|_| Monkey::new()).collect();
    let mut ecosystem = Ecosystem::new(monkeys);

    while ecosystem.fittest().phrase != String::from(Monkey::TARGET_PHRASE) {
        ecosystem.breed_next_generation(MUTATION_RATE);
        println!("{}", ecosystem.fittest().phrase);
    }
}
