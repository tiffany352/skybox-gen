use std::iter::*;

pub fn additive_blend(data1: &[u8], data2: &[u8], _w: uint, _h: uint) -> ~[u8] {
    data1.iter().zip(data2.iter()).map(|(&c1, &c2)| {
        if 255 - c1 < c2 {
            255
        } else {
            c1+c2
        }
    }).to_owned_vec()
}

