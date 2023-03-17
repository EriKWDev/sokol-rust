[![Rust](https://github.com/ErikWDev/sokol-rust/actions/workflows/main.yml/badge.svg)](https://github.com/ErikWDev/sokol-rust/actions/workflows/main.yml)

## Sokol-rust
Auto-generated Rust bindings for the [sokol headers](https://github.com/floooh/sokol).

Add `sokol-rust` as a dependency to your `Cargo.toml` as such:
```toml
sokol = { version="*", git="https://github.com/ErikWDev/sokol-rust.git" }
```

Check out the `examples/` folder for more examples. Here is `examples/clear/clear.rs`:
```rust
use sokol::app as sapp;
use sokol::gfx as sg;

struct State {
    pass_action: sg::PassAction,
}

static mut STATE: State = State {
    pass_action: sg::PassAction::new(),
};

extern "C" fn init() {
    let state = unsafe { &mut STATE };

    sg::setup(&sg::Desc {
        context: sokol::glue::context(),
        ..Default::default()
    });

    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        action: sg::Action::Clear,
        value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

    let g = state.pass_action.colors[0].value.g + 0.01;
    state.pass_action.colors[0].value.g = if g > 1.0 { 0.0 } else { g };

    let (width, height) = (sapp::width(), sapp::height());

    sg::begin_default_pass(&state.pass_action, width, height);
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    sg::shutdown()
}

fn main() {
    let window_title = b"clear\0".as_ptr() as _;

    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        frame_cb: Some(frame),
        window_title,
        width: 800,
        height: 600,
        sample_count: 4,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
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

To run the imgui example, you need to go into it's directory:
```console
cd examples/imgui
cargo run
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
