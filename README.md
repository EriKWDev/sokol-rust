[![Rust](https://github.com/ErikWDev/sokol-rust/actions/workflows/main.yml/badge.svg)](https://github.com/ErikWDev/sokol-rust/actions/workflows/main.yml)

## Sokol-rust
Auto-generated Rust bindings for the [sokol headers](https://github.com/floooh/sokol).

Add `sokol-rust` as a dependency to your `Cargo.toml` as such:
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
cargo run --example debugtext
cargo run --example sgl-context
cargo run --example sgl-points
cargo run --example blend
cargo run --example userdata
```

## Shaders
Sokol's own shader compiler does not yet have support for generating helper files in rust, though they can quite easily
be translated into rust manually as have been done for the examples. See `examples/mrt/shader.rs` or `examples/cube/shader.rs`
for examples on how this has been done.

I hope to add support for rust into the shader compiler as well.

A work-in-progress version of the shader compiler can be found in `sokolrust.cc`. If you place it inside the `src/shdc/` folder
of the [sokol-tools](https://github.com/floooh/sokol-tools), you just need to make some slight modifications to `main.cc` and re-compile
the shader-compiler in order to add experimental support for rust shaders as well.

```cpp
// .. in main.cc switch case, add a case for rust
case format_t::SOKOL_RUST:
    output_err = sokolrust_t::gen(args, inp, spirvcross, bytecode);
    break;

// .. in shdc.h, add SOKOL_RUST as a format and implement the string conversions
case SOKOL_RUST:    return "sokol_rust";
// ..
else if (str == "sokol_rust") {
    return SOKOL_RUST;
}

// .. in util.cc, add a simple helper function
std::string to_upper_case(const std::string& str) {
    return pystring::upper(str);
}
// .. and declare it in shdc.h's namespace util { .. }
std::string to_upper_case(const std::string& str);
```
