use layered::*;
use rng_gen::*;
use std::rand::*;

struct PlasmaContext {
    red: LayeredNoise,
    green: LayeredNoise,
    blue: LayeredNoise
}

impl PlasmaContext {
    pub fn new<T:Rng>(rng: &mut T) -> PlasmaContext {
        PlasmaContext {
            red:    LayeredNoise::new(8, &mut gen_seed(rng)),
            green:  LayeredNoise::new(8, &mut gen_seed(rng)),
            blue:   LayeredNoise::new(8, &mut gen_seed(rng))
        }
    }
    pub fn sample(&self, x: float, y: float, z: float) -> (float, float, float) {
        (self.red.sample(x,y,z), self.green.sample(x,y,z), self.blue.sample(x,y,z))
    }
}

