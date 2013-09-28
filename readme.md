# Install

## Dependencies

 * [noise-rs](https://github.com/bjz/noise-rs)
 * [rust-png](https://github.com/mozilla-servo/rust-png)

## Compiling

Make sure `libnoise-*.so` and `libpng-*.so` and `libshim.a` are in your rust path, and

    rustc skybox.rs

## Running

    ./skybox <size> <side> <seed> <filename>

*or*

    ./gen.sh <size> # generates all sides and saves them as up/down/north/south/east/west.png

# Todo

 * More skybox types
 * More configurability
 * GPU-based rendering (takes forever currently)

