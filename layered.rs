use noise::perlin::*;
use std::vec;
use std::num;
use std::iter::*;
use std::rand::*;
use rng_gen::*;

struct LayeredNoise {
    contexts: ~[PerlinContext]
}

impl LayeredNoise {
    pub fn new<T:Rng>(n: uint, rng: &mut T) -> LayeredNoise {
        LayeredNoise { contexts: vec::from_fn(n, |_| PerlinContext::new(&mut gen_seed(rng))) }
    }
    pub fn sample(&self, x: float, y: float, z: float) -> float {
        let scale_n = |x:float, s:float| x * num::pow(3.0, s);
        let divisor = |i:uint, _l:uint| num::pow(2.0, i as float);
        num::clamp( self.contexts.iter()
                   .enumerate()
                   .map(|(i,c)| c.gen3(&scale_n(x, i as float), &scale_n(y, i as float), &scale_n(z, i as float)) / divisor(i, self.contexts.len()))
                   .sum() + 0.5, 0.0, 1.0)
    }
}

