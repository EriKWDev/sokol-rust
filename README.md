[![Rust](https://github.com/ErikWDev/sokol-rust/actions/workflows/main.yml/badge.svg)](https://github.com/ErikWDev/sokol-rust/actions/workflows/main.yml)

## Sokol-rust
Auto-generated Rust bindings for the [sokol headers](https://github.com/floooh/sokol).

Add as a dependency as such:
```toml
sokol = { version="*", git="https://github.com/ErikWDev/sokol-rust.git" }
```

## Updating the bindings
To update, place the `gen_rust.py` script inside `sokol/bindgen` and clone this repository inside there. 
Inside `gen_all.py`, import `gen_rust` like the other bindgens and call it's `gen` function the exact and
same way as the others.

```python
import gen_rust

# Rust
gen_rust.prepare()
for task in tasks:
    [c_header_path, main_prefix, dep_prefixes] = task
    gen_rust.gen(c_header_path, main_prefix, dep_prefixes)
```

I also recommend to run `cargo fmt` inside `sokol-rust` after the python script to clean up the output formatting.

## Dependencies
The rust compiler and cargo can be installed using [rustup](https://rustup.rs/)

The same dependencies apply as with sokol normally for each platform 

## Building with cargo
Cargo will compile and link the sokol headers automatically durnig compilation thanks to the buildscript `build.rs`

## Examples
Not all examples have been translated to rust yet, but you can check the onces that have been in the `examples` directory.

You can compile all examples using the following command:
```console
cargo build --all-targets
```

Build and run individual examples as such:
```console
cargo run --example clear
cargo run --example cube
cargo run --example mrt
cargo run --example userdata
```

## Shaders
Sokol's own shader compiler does not yet have support for generating helper files in rust, though they can quite easily
be translated into rust manually as have been done for the examples. See `examples/mrt/shader.rs` or `examples/cube/shader.rs`
for examples on how this has been done.

I hope to add support for rust into the shader compiler as well.
