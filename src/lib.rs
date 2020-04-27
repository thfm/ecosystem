#![warn(missing_docs)]
//! A small genetic algorithms library.
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;

/// An interface for breeding, mutation, and fitness evaluation functionality.
pub trait Organism {
    /// Evaluates the organism's fitness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// impl Organism for Bird {
    ///     fn fitness(&self) -> f64 {
    ///         self.x.powi(2)
    ///     }
    /// }
    /// ```
    fn fitness(&self) -> f64;

    /// Creates a new child by breeding the organism with another.
    ///
    /// # Examples
    ///
    /// ```rust
    /// impl Organism for Bird {
    ///     fn breed(&self, other: &Self) -> Self {
    ///         Self {
    ///             x: 0.0,
    ///             flap_power: (self.flap_power + other.flap_power) / 2.0,
    ///         }
    ///     }
    /// }
    /// ```
    fn breed(&self, other: &Self) -> Self;

    /// Modifies (or *mutates*) the organism, based on the given rate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rand::Rng;
    ///
    /// impl Organism for Bird {
    ///     fn mutate(&mut self, rate: f64) {
    ///         let change = rand::thread_rng().gen_range(-1.0, 1.0);
    ///         self.flap_power += change * rate;
    ///     }
    /// }
    /// ```
    fn mutate(&mut self, rate: f64);
}

/// A collection of organisms.
pub struct Ecosystem<O: Organism> {
    organisms: Vec<O>,
    generation: u32,
}

impl<O: Organism + std::marker::Send + std::marker::Sync> Ecosystem<O> {
    /// Creates a new ecosystem with the given organisms.
    pub fn new(organisms: Vec<O>) -> Self {
        Self {
            organisms,
            generation: 0,
        }
    }

    /// Returns the organism in the ecosystem with the highest fitness.
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

    /// Creates the next generation of organisms through the breeding
    /// of suitable organisms.
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

    /// Selects an organism in the ecosystem that is suitable for breeding,
    /// based on fitness values.
    ///
    /// # Panics
    ///
    /// This method panics if the ecosystem contains no organisms.
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
