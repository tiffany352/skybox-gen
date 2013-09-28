use std::vec;
use std::rand::*;

pub fn gen_seed<T:Rng>(rng: &mut T) -> IsaacRng {
    let v = vec::from_fn(16, |_| rng.gen::<u8>());
    IsaacRng::new_seeded(v)
}

