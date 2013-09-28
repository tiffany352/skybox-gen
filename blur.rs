use std::num::*;
use std::vec;
use util::*;

struct BlurContext {
    matrix: ~[float],
    r: uint
}

fn make_gaussian(N: float, sigma: float) -> ~fn(float)->float {
    /*def make_gauss(N, sigma, mu):
    k = N / (sigma * math.sqrt(2*math.pi))
    s = -1.0 / (2 * sigma * sigma)
    def f(x):
        return k * math.exp(s * (x - mu)*(x - mu))*/
    let k = N / (sigma * sqrt(2.0 * Real::pi()));
    let s = -1.0 / (2.0 * sigma * sigma);
    |x| {
        k * exp(s * x * x)
    }
}

impl BlurContext {
    pub fn new(N: float, radius: uint) -> BlurContext {
        let diam = radius * 2 + 1;
        let fdiam = diam as float;
        let gaus = make_gaussian(N, fdiam / 6.0);
        BlurContext {
            matrix: vec::from_fn(diam*diam, |p| {
                let x = (p%diam) as int - radius as int;
                let y = (p as int/diam as int) - radius as int;
                gaus(x as float) * gaus(y as float)
            }),
            r: radius
        }
        /*for y in range(0, diam) {
            for x in range(0, diam) {
                print!("{} ", c.matrix[y*diam + x]);
            }
            println("");
        }*/
    }
    pub fn render(&self, img: &[u8], w: uint, h: uint) -> ~[u8] {
        let mut buf = ~[];
        let sample = |x: uint, y: uint| (img[3*y*w + 3*x + 0],
                                         img[3*y*w + 3*x + 1],
                                         img[3*y*w + 3*x + 2]);
        let wi = w as int;
        let hi = h as int;
        let radi = self.r as int;
        let diam = radi*2 + 1;
        for y in range(0, h) {
            for x in range(0, w) {
                let xi = x as int;
                let yi = y as int;
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                for by in range(max(0, yi - radi) - yi, min(hi, yi + radi) - yi) {
                    for bx in range(max(0, xi - radi) - xi, min(wi, xi + radi) - xi) {
                        let sx = (bx + xi) as uint;
                        let sy = (by + yi) as uint;
                        let (nr,ng,nb) = sample(sx, sy);
                        let p = ((by+radi)*diam + bx+radi) as uint;
                        r += self.matrix[p] * from_u8(nr);
                        g += self.matrix[p] * from_u8(ng);
                        b += self.matrix[p] * from_u8(nb);
                    }
                }
                buf.push(~[to_u8(r),to_u8(g),to_u8(b)]);
            }
        }
        vec::concat(buf)
    }
}

