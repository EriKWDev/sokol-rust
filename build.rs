fn select_sokol_gfx_renderer(build: &mut cc::Build, is_msvc: bool, is_impl: bool) {
    //
    // select sokol_gfx renderer, defaults to:
    // - Windows: D3D11 with MSVC, GLCORE33 otherwise
    // - MacOS: Metal
    // - Linux: GLCORE33
    //
    if cfg!(target_os = "windows") && is_msvc {
        build.flag("-DSOKOL_D3D11");
    } else if cfg!(target_os = "macos") {
        build.flag("-DSOKOL_METAL");
    } else {
        build.flag("-DSOKOL_GLCORE33");
    }

    if is_impl {
        if cfg!(target_os = "windows") && is_msvc {
            println!("cargo:rustc-cfg=gfx=\"d3d11\"");
        } else if cfg!(target_os = "macos") {
            println!("cargo:rustc-cfg=gfx=\"metal\"");
        } else {
            println!("cargo:rustc-cfg=gfx=\"glcore33\"");
        }
    }
}

fn make_sokol() {
    let mut build = cc::Build::new();
    let tool = build.try_get_compiler().unwrap();

    let is_debug = std::env::var("DEBUG").ok().is_some();
    let is_msvc = tool.is_like_msvc();

    let files = [
        "src/sokol/c/sokol_log.c",
        "src/sokol/c/sokol_app.c",
        "src/sokol/c/sokol_gfx.c",
        "src/sokol/c/sokol_glue.c",
        "src/sokol/c/sokol_time.c",
        "src/sokol/c/sokol_audio.c",
        "src/sokol/c/sokol_gl.c",
        "src/sokol/c/sokol_debugtext.c",
        "src/sokol/c/sokol_shape.c",
    ];

    //
    // include paths
    //
    build.include("src/sokol/c/");

    for file in &files {
        println!("cargo:rerun-if-changed={}", file);

        //
        // MacOS: need ARC, so compile sokol.m with -fobjc-arc
        //
        if cfg!(target_os = "macos") {
            build.flag("-fobjc-arc");
        }
        build.file(file);
    }

    //
    // select sokol_gfx renderer
    //
    select_sokol_gfx_renderer(&mut build, is_msvc, true);

    //
    // silence some warnings
    //
    build.flag_if_supported("-Wno-unused-parameter");

    //
    // x86_64-pc-windows-gnu: additional compile/link flags
    //
    if cfg!(target_os = "windows") {
        if !is_msvc {
            build
                .flag("-D_WIN32_WINNT=0x0601")
                .flag_if_supported("-Wno-cast-function-type")
                .flag_if_supported("-Wno-sign-compare")
                .flag_if_supported("-Wno-unknown-pragmas");

            println!("cargo:rustc-link-lib=static=gdi32");
            println!("cargo:rustc-link-lib=static=ole32");
        }
    }
    if is_debug {
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    build.compile("sokol-rust");
    println!("cargo:rustc-link-lib=static=sokol-rust");
    println!("cargo:rustc-link-search=src/sokol/c/");
    println!("cargo:rustc-link-search=target/debug/");

    //
    // MacOS: frameworks
    //
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=Audiocc::Toolbox");
    }

    //
    // Linux: libs
    //
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=asound");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    make_sokol();
}
