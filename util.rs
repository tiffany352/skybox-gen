use std::from_str::*;
use std::num::*;

pub enum Side {
    West,
    East,
    Up,
    Down,
    North,
    South,
}

impl FromStr for Side {
    fn from_str(s: &str) -> Option<Side> {
        match s {
            "west"  => Some(West),
            "east"  => Some(East),
            "up"    => Some(Up),
            "down"  => Some(Down),
            "north" => Some(North),
            "south" => Some(South),
            _ => None
        }
    }
}

pub fn sidespace_to_worldspace(side: Side, x: float, y: float) -> (float, float, float) {
    match side {
        West  => ( -1.0,  y,    x   ),
        East  => (  1.0,  y,   -x   ),
        Up    => ( -x,    1.0, -y   ),
        Down  => (  x,   -1.0,  y   ),
        North => (  x,    y,    1.0 ),
        South => ( -x,    y,   -1.0 ),
    }
}

pub fn worldspace_to_sidespace(side: Side, x: float, y: float, z: float) -> (float, float) {
    match side {
        West  => ( -z/x,  y/x ),
        East  => (  z/x, -y/x ),
        Up    => ( -x/y, -z/y ),
        Down  => (  x/y,  z/y ),
        North => (  x/z,  y/z ),
        South => ( -x/z,  y/z ),
    }
}

pub fn to_u8(val: float) -> u8 {
    (clamp(val, 0.0, 1.0) * 255.0) as u8
}

pub fn from_u8(val: u8) -> float {
    val as float / 255.0
}

pub fn cubify(x: float, y: float, z: float) -> (float, float, float) {
    let len = max(abs(x),max(abs(y),abs(z)));
    (x/len,y/len,z/len)
}

pub fn normalize(x: float, y: float, z: float) -> (float,float,float) {
    let len = sqrt(x*x + y*y + z*z);
    (x/len, y/len, z/len)
}

pub fn normalize_unsigned(x: float, y: float, z: float) -> (float, float, float) {
    let (x,y,z) = (x-0.5, y-0.5, z-0.5);
    let len = sqrt(x*x + y*y + z*z);
    (x/len+0.5, y/len+0.5, z/len+0.5)
}

