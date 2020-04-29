# Ecosystem

A small genetic algorithms library, written in Rust.

To use this library in your project, just add the following line to its `Cargo.toml` dependencies:

```toml
ecosystem = "0.1"
```

Note that the following usage instructions assume a basic understanding of genetic algorithms. If you don't know anything about the topic, but wish to learn more, I can recommend the beginner-friendly video series by The Coding Train, which you can find [here](https://www.youtube.com/playlist?list=PLRqwX-V7Uu6bJM3VgzjNV5YxVxUwzALHV).

## Fundamentals

This library is based around two key components: organisms and ecosystems.

### Organisms

Implementing the `Organism` trait on any type will allow it to be a part of an `Ecosystem`, thus enabling genetic functionality. It requires you to fill in three methods:

1. `fitness`: this should return an indicator of how well the organism is performing the task that it has been set
2. `breed`: this should return a child organism with a mix of both parents' attributes ('genetic material')
3. `mutate`: this should randomly modify the organism, by an amount determined by the `rate` parameter

An example usage of this trait is covered in the [walkthrough example](#walkthrough-example).

### Ecosystems

An `Ecosystem` is simply a group of `Organism`s that contains functionality for the breeding of new generations.

You can create one like so:

```rust
use ecosystem::Ecosystem;

// `your_organisms` must be a vector of items which implement
// the `Organism` trait
let mut ecosystem = Ecosystem::new(your_organisms);
```

You can breed a new generation of organisms by calling the `breed_next_generation` method. This will overwrite the existing organisms, meaning that the population count will always stay the same.

```rust
// The only argument passed to the method is the mutation rate,
// a floating-point value that describes the extent to which the
// new organisms should be mutated (randomly modified)
ecosystem.breed_next_generation(0.1);
```

As it's often beneficial to know which organism in an `Ecosystem` has the highest fitness, they also include the helper method `fittest`:

```rust
let the_best = ecosystem.fittest();
```

The next section walks through a simple example scenario in which these fundamentals are put to practical use.

## Walkthrough example

This section will cover how to build an ecosystem of organisms that attempt to approximate the value of π.

### Building the foundations

There are two things we need to set up before we start adding genetic functionality.

Firstly, we need some structure to represent a 'π approximator', which will simply hold the value of its guess:

```rust
struct PiApproximator {
    value: f64,
}
```

And secondly, we need to create a vector holding a number of these approximators, which we can do using a simple iterator:

```rust
use rand::Rng;

const POPULATION_COUNT: u32 = 10;
const MAX_INITIAL_VALUE: f64 = 10.0;

fn main() {
    let mut rng = rand::thread_rng();
    let approximators: Vec<PiApproximator> = (0..POPULATION_COUNT)
        .map(|_| PiApproximator {
            value: rng.gen_range(-MAX_INITIAL_VALUE, MAX_INITIAL_VALUE),
        })
        .collect();
}
```

### Adding genetic functionality

The first thing we must do in order to add genetic functionality is to implement the `Organism` trait on the `PiApproximator` structure, which requires us to fill in the `fitness`, `breed`, and `mutate` methods.

```rust
use ecosystem::Organism;

impl Organism for PiApproximator {
    fn fitness(&self) -> f64 { ... }

    fn breed(&self, other: &Self) -> Self { ... }

    fn mutate(&mut self, rate: f64) { ... }
}
```

Let's go through each of these in order.

One way of calculating the fitness of an approximator is to take the reciprocal of the difference between its guess and the real value of π. This means that the closer an approximator's guess is, the higher its resulting fitness will be.

```rust
fn fitness(&self) -> f64 {
    let diff = (std::f64::consts::PI - self.value).abs();
    1.0 / diff
}
```

Breeding approximators is even simpler: we can just take the average of the two parents' values:

```rust
fn breed(&self, other: &Self) -> Self {
    Self {
        value: (self.value + other.value) / 2.0,
    }
}
```

Finally, we can mutate an approximator by randomly shifting its value up or down. The higher the `rate`, the greater the potential change.

```rust
fn mutate(&mut self, rate: f64) {
    let change = rand::thread_rng().gen_range(-rate, rate);
    self.value += change;
}
```

To put it all together, let's build an `Ecosystem` out of the approximators we created earlier:

```rust
use ecosystem::Ecosystem;

fn main() {
    ...

    let mut ecosystem = Ecosystem::new(approximators);
}
```

Finally, we can set up a loop that will run for a certain number of generations, printing the guess of the best approximator in each:

```rust
const GENERATIONS: u32 = 50;
const MUTATION_RATE: f64 = 0.1;

fn main() {
    ...

    for _ in 0..GENERATIONS {
        ecosystem.breed_next_generation(MUTATION_RATE);
        println!("{}", ecosystem.fittest().value);
    }
}
```

That's it! Try playing around with the constants to see what different results you can get.

The full code for the walkthrough is in the `examples` folder of this repository.
