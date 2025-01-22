use super::bacterium_genome::*;
use rand::Rng;

pub fn calculate_fitness(genome: Genome) -> f64 {
    let greed = get_greedy(genome) as f64;
    let flagella = get_flagella(genome) as f64;
    let mut fitness = greed;
    fitness *= flagella;
    fitness
}

pub fn roulette_wheel_selection(pop: &[(Genome, f64)], nb: usize) -> Vec<Genome> {
    let mut rng = rand::thread_rng();
    let total_fitness: f64 = pop.iter().map(|(_, f)| f).sum();
    let mut selected = Vec::new();
    for _ in 0..nb {
        let pick = rng.gen_range(0.0..total_fitness);
        let mut accum = 0.0;
        for (ind, fit) in pop {
            accum += fit;
            if accum >= pick {
                selected.push(*ind);
                break;
            }
        }
    }
    selected
}

pub fn random_crossover(parent_a: &Genome, parent_b: &Genome) -> (Genome, Genome) {
    let mut rng = rand::thread_rng();
    let point = rng.gen_range(1..32);
    let mask: u32 = (1 << point) - 1; // point bits à 1
    let child_a = (parent_a & !mask) | (parent_b & mask);
    let child_b = (parent_b & !mask) | (parent_a & mask);
    (child_a, child_b)
}

pub fn mutate(individual: &mut Genome, mutation_rate: f64) {
    let mut rng = rand::thread_rng();
    for bit_pos in 0..32 {
        if rng.gen_bool(mutation_rate) {
            let mask = 1 << bit_pos;
            *individual ^= mask; // flip du bit
        }
    }
}
