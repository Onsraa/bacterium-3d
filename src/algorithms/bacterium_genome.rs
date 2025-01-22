use rand::Rng;

/// Un génome est un `u32` de 32 bits
pub type Genome = u32;

/*
Nouvelle répartition (32 bits) :

 - Bits [0..1]   : feeding mode (2 bits) => 0..=3
 - Bits [2..5]   : tails (4 bits)        => 0..=15
 - Bits [6..10]  : greed (5 bits)       => 0..=31
 - Bits [11..20] : énergie (10 bits)    => 0..=1023
 - Bits [21..31] : vie (11 bits)        => 0..=2047

Total = 2 + 4 + 5 + 10 + 11 = 32 bits
*/

/// Décalages (shifts) et masks
pub const FEEDING_MODE_SHIFT: u32 = 0; // 2 bits
pub const FEEDING_MODE_MASK: u32 = 0b11; // [0..3]

pub const TAILS_SHIFT: u32 = 2; // 4 bits
pub const TAILS_MASK: u32 = 0b1111; // [0..15]

pub const GREED_SHIFT: u32 = 6; // 5 bits
pub const GREED_MASK: u32 = 0b1_1111; // [0..31] => 0b11111 = 31

pub const ENERGY_SHIFT: u32 = 11; // 10 bits
pub const ENERGY_MASK: u32 = 0x3FF; // [0..1023] => 10 bits

pub const LIFE_SHIFT: u32 = 21; // 11 bits => [21..31]
pub const LIFE_MASK: u32 = 0x7FF; // [0..2047] => 11 bits

/// Fonctions GET
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

/// Petit utilitaire interne pour factoriser le code.
fn set_bits(genome: &mut Genome, shift: u32, mask: u32, val: u32) {
    *genome &= !(mask << shift); // on efface la zone
    *genome |= (val & mask) << shift; // on insère la valeur
}

/// Fonctions SET
pub fn set_feeding_mode(genome: &mut Genome, val: u32) {
    set_bits(genome, FEEDING_MODE_SHIFT, FEEDING_MODE_MASK, val);
}
pub fn set_flagella(genome: &mut Genome, val: u32) {
    set_bits(genome, TAILS_SHIFT, TAILS_MASK, val);
}
pub fn set_greed(genome: &mut Genome, val: u32) {
    set_bits(genome, GREED_SHIFT, GREED_MASK, val);
}
pub fn set_energy(genome: &mut Genome, val: u32) {
    set_bits(genome, ENERGY_SHIFT, ENERGY_MASK, val);
}
pub fn set_life(genome: &mut Genome, val: u32) {
    set_bits(genome, LIFE_SHIFT, LIFE_MASK, val);
}

/// Génome Aléatoire :
/// - feeding_mode : 2 bits => [0..3]
/// - tails        : 4 bits => [0..15]
/// - greed        : 5 bits => [0..31]
/// - energy       : 10 bits => [0..1023]
/// - life         : 11 bits => [0..2047]
pub fn random_genome() -> Genome {
    let mut rng = rand::thread_rng();
    let mut g: Genome = 0;

    set_feeding_mode(&mut g, rng.gen_range(0..=3));
    set_flagella(&mut g, rng.gen_range(0..=15));
    set_greed(&mut g, rng.gen_range(0..=31));
    set_energy(&mut g, rng.gen_range(0..=1023));
    set_life(&mut g, rng.gen_range(0..=2047));

    g
}
