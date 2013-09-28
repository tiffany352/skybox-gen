use std::rand::Rng;
use std::num::*;
use util::*;
use std::vec;
use blur::*;
use blend::*;

struct Star {
    x: float,
    y: float,
    intensity: float,
    redshift: float
}

struct StarsContext {
    stars: ~[Star],
    blurs: ~[BlurContext]
}

impl StarsContext {
    pub fn new<T:Rng>(rng: &mut T) -> StarsContext {
        // φ = cos-1(2x - 1)
        let mut list = ~[];
        for _ in range(0, 1000) {
            list.push(Star {
                x: rng.gen::<float>() * Real::pi() * 2.0, 
                y: acos(2.0 * rng.gen::<float>() - 1.0),
                intensity: rng.gen::<float>(),
                redshift: rng.gen::<float>()
            });
        }
        StarsContext { 
            stars: list,
            blurs: ~[
                BlurContext::new(1.0, 1),
                BlurContext::new(1.0, 3),
                BlurContext::new(0.5, 8)
            ]
        }
    }
    pub fn render(&self, size: uint, side: Side) -> ~[u8] {
        let mut img = vec::from_elem((size*size*3) as uint, 0u8);
        for &Star {x: sx, y: sy, intensity: intensity, redshift: redshift} in self.stars.iter() {
            let (x,y,z) = normalize(sin(sx) * sin(sy), cos(sy), cos(sx) * sin(sy));
            let (x,y) = worldspace_to_sidespace(side, x, y, z);
            if x > -1.0 && x < 1.0 && y > -1.0 && y < 1.0 {
                let (x,y) = (((x/2.0 + 0.5)*(size-1) as float) as uint, 
                             ((y/2.0 + 0.5)*(size-1) as float) as uint);
                img[3 * y * size + 3 * x + 0] = to_u8(intensity * (1.0 - redshift));
                img[3 * y * size + 3 * x + 1] = to_u8(intensity);
                img[3 * y * size + 3 * x + 2] = to_u8(intensity * redshift);
            }
        }
        for blur in self.blurs.iter() {
            img = additive_blend(img, blur.render(img, size, size), size, size);
        }
        img
    }
}

