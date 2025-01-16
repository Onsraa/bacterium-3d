use crate::components::diet::Diet;
use bevy::prelude::Color;
use rand::Rng;

/// Un génome est simplement un u32
pub type Genome = u32;

/// Répartition simplifiée :
/// - Bits [0..1]   : mode de nourrissage (2 bits)
/// - Bits [2..3]   : nombre de flagelles (2 bits)
/// - Bits [4..7]   : gourmandise (4 bits) => influence la tendance à se nourrir
/// - Bits [8..17]  : énergie (10 bits)
/// - Bits [18..27] : points de vie (10 bits)
///
/// On laissera les bits [28..31] (4 bits) libres pour d'éventuelles évolutions.

pub const FEEDING_MODE_SHIFT: u32 = 0;
pub const FEEDING_MODE_MASK: u32 = 0b11;
pub const TAILS_SHIFT: u32 = 2;
pub const TAILS_MASK: u32 = 0b11;
pub const GREED_SHIFT: u32 = 4;
pub const GREED_MASK: u32 = 0b1111;
pub const ENERGY_SHIFT: u32 = 8;
pub const ENERGY_MASK: u32 = 0x3FF;
pub const LIFE_SHIFT: u32 = 18;
pub const LIFE_MASK: u32 = 0x3FF;

pub fn get_feeding_mode(genome: Genome) -> u32 {
    (genome >> FEEDING_MODE_SHIFT) & FEEDING_MODE_MASK
}

pub fn get_flagella(genome: Genome) -> u32 {
    (genome >> TAILS_SHIFT) & TAILS_MASK
}

pub fn get_greedy(genome: Genome) -> u32 {
    (genome >> GREED_SHIFT) & GREED_MASK
}

pub fn get_energy(genome: Genome) -> u32 {
    (genome >> ENERGY_SHIFT) & ENERGY_MASK
}

pub fn get_health(genome: Genome) -> u32 {
    (genome >> LIFE_SHIFT) & LIFE_MASK
}

pub fn set_feeding_mode(genome: &mut Genome, val: u32) {
    let val = val & FEEDING_MODE_MASK;
    *genome &= !(FEEDING_MODE_MASK << FEEDING_MODE_SHIFT);
    *genome |= val << FEEDING_MODE_SHIFT;
}

pub fn set_tails(genome: &mut Genome, val: u32) {
    let val = val & TAILS_MASK;
    *genome &= !(TAILS_MASK << TAILS_SHIFT);
    *genome |= val << TAILS_SHIFT;
}

pub fn set_greed(genome: &mut Genome, val: u32) {
    let val = val & GREED_MASK;
    *genome &= !(GREED_MASK << GREED_SHIFT);
    *genome |= val << GREED_SHIFT;
}

pub fn set_energy(genome: &mut Genome, val: u32) {
    let val = val & ENERGY_MASK;
    *genome &= !(ENERGY_MASK << ENERGY_SHIFT);
    *genome |= val << ENERGY_SHIFT;
}

pub fn set_life(genome: &mut Genome, val: u32) {
    let val = val & LIFE_MASK;
    *genome &= !(LIFE_MASK << LIFE_SHIFT);
    *genome |= val << LIFE_SHIFT;
}

pub fn random_genome() -> Genome {
    let mut rng = rand::thread_rng();
    let mut g: Genome = 0;
    set_feeding_mode(&mut g, rng.gen_range(0..=3));
    set_tails(&mut g, rng.gen_range(0..=3));
    set_greed(&mut g, rng.gen_range(0..=15));
    set_energy(&mut g, rng.gen_range(0..=1023));
    set_life(&mut g, rng.gen_range(0..=1023));
    g
}
