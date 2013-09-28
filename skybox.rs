extern mod noise;
extern mod png;
extern mod extra;

use png::*;
use std::os;
use std::vec;
use std::from_str::*;
use plasma::*;
use layered::*;
use stars::*;
use util::*;
use blend::*;
use extra::time::*;
use std::rand::*;

pub mod plasma;
pub mod layered;
pub mod stars;
pub mod util;
pub mod blur;
pub mod blend;
pub mod rng_gen;

fn usage() {
    println!("Usage: {} <size> <side> <seed> <filename>", os::args()[0]);
}

fn profile(start: Timespec, thing: &str) {
    let stop = get_time();
    let diff = Timespec {
        sec: stop.sec - start.sec,
        nsec: stop.nsec - start.nsec
    };
    println!("Finished {}: {} seconds", thing, diff.sec as float + diff.nsec as float / 1000000000.0);
}

fn main() {
    let args = os::args();
    if args.len() < 5 {
        usage();
        return
    }
    let (size, side): (uint, Side) = match (from_str(args[1]), from_str(args[2]))  {
        (Some(x), Some(y)) => (x,y),
        _ => {
            usage();
            return
        }
    };
    let file = PosixPath(args[4]);
    let mut rng = IsaacRng::new_seeded(args[3].as_bytes());
    let pc = PlasmaContext::new(&mut rng);
    let nc = LayeredNoise::new(6, &mut rng);
    let sc = StarsContext::new(&mut rng);
    println!("Generating {}x{} {:?} face: {}", size, size, side, args[4]);
    let gen_start = get_time();
    println("Rendering nebulae...");
    let start = get_time();
    let data = do vec::from_fn(size*size) |pix| {
        let fx = (pix % size) as float / size as float;
        let fy = (pix / size) as float / size as float;
        let (x, y, z) = sidespace_to_worldspace(side, fx*2.0 - 1.0, fy*2.0 - 1.0);
        let (x, y, z) = normalize(x, y, z);
        //~[to_u8(x/2.0 + 0.5), to_u8(y/2.0 + 0.5), to_u8(z/2.0 + 0.5)]
        let (r,g,b) = pc.sample(x,y,z);
        let sample = nc.sample(x,y,z) * 0.2; 
        let (r,g,b) = (r*sample, g*sample, b*sample);
        let (r,g,b) = (to_u8(r as float), to_u8(g as float), to_u8(b as float));
        //let sample = to_u8(sample);
        ~[r,g,b]
    };
    let data = vec::concat(data);
    profile(start, "rendering nebulae");
    println("Rendering stars...");
    let start = get_time();
    let star_data = sc.render(size as uint, side);
    let data: ~[u8] = additive_blend(data, star_data, size as uint, size as uint);
    profile(start, "rendering stars");
    let img = Image {
        width: size as u32,
        height: size as u32,
        color_type: RGB8,
        pixels: data
    };
    store_png(&img, &file);
    profile(gen_start, "rendering face");
}

