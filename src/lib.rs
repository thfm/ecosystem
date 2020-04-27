use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;

pub trait Organism {
    fn fitness(&self) -> f64;
    fn breed(&self, other: &Self) -> Self;
    fn mutate(&mut self, rate: f64);
}

pub struct Ecosystem<O: Organism> {
    organisms: Vec<O>,
    generation: u32,
}

impl<O: Organism + std::marker::Send + std::marker::Sync> Ecosystem<O> {
    pub fn new(organisms: Vec<O>) -> Self {
        Self {
            organisms,
            generation: 0,
        }
    }

    pub fn fittest(&self) -> &O {
        self.organisms
            .iter()
            .fold(&self.organisms[0], |fittest, organism| {
                if organism.fitness() > fittest.fitness() {
                    organism
                } else {
                    fittest
                }
            })
    }

    pub fn breed_next_generation(&mut self, mutation_rate: f64) {
        let next_generation: Vec<_> = (0..self.organisms.len())
            .into_par_iter()
            .map(|_| {
                let mother = self.select_suitable_organism();
                let father = self.select_suitable_organism();

                let mut child = mother.breed(father);
                child.mutate(mutation_rate);
                child
            })
            .collect();

        self.organisms = next_generation;
        self.generation += 1;
    }

    fn select_suitable_organism(&self) -> &O {
        let mut rng = rand::thread_rng();
        loop {
            let organism = self
                .organisms
                .choose(&mut rng)
                .unwrap_or_else(|| panic!("there are no organisms in the ecosystem"));
            if rng.gen_range(0.0, self.fittest().fitness()) < organism.fitness() {
                break &organism;
            }
        }
    }
}
