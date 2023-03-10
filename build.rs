#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SokolBackend {
    D3d11,
    Metal,
    Gl,
    Gles2,
    Gles3,
    Wgpu,
}

fn make_sokol() {
    let mut build = cc::Build::new();
    let tool = build.try_get_compiler().unwrap();

    let debug_info_requested = std::env::var("SOKOL_DEBUG").ok().is_some();
    let is_msvc = tool.is_like_msvc();
    const BASE_C_DIR: &str = "src/sokol/c/";

    let is_debug_build = cfg!(debug_assertions);
    if !is_debug_build {
        build.define("NDEBUG", None);
        build.opt_level(2);
    }

    let desired_backend = std::env::var("SOKOL_BACKEND")
        .ok()
        .unwrap_or("AUTO".to_owned());

    let wayland_desired = std::env::var("SOKOL_WAYLAND").is_ok();
    let force_egl = std::env::var("SOKOL_FORCE_EGL").is_ok();

    println!("cargo:rerun-if-env-changed=SOKOL_BACKEND");
    println!("cargo:rerun-if-env-changed=SOKOL_WAYLAND");
    println!("cargo:rerun-if-env-changed=SOKOL_FORCE_EGL");
    println!("cargo:rerun-if-env-changed=SOKOL_DEBUG");

    let backend = match &desired_backend[..] {
        "AUTO" => {
            if cfg!(target_os = "windows") && is_msvc {
                SokolBackend::D3d11
            } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
                SokolBackend::Metal
            } else {
                SokolBackend::Gl
            }
        }

        "D3D11" => SokolBackend::D3d11,
        "METAL" => SokolBackend::Metal,
        "GL" => SokolBackend::Gl,
        "GLES2" => SokolBackend::Gles2,
        "GLES3" => SokolBackend::Gles3,
        "WGPU" => SokolBackend::Wgpu,

        _ => panic!("Unknown SOKOL_BACKEND: {desired_backend}"),
    };

    match backend {
        SokolBackend::D3d11 => {
            build.define("SOKOL_D3D11", None);
        }
        SokolBackend::Metal => {
            build.define("SOKOL_METAL", None);
        }
        SokolBackend::Gles3 => {
            build.define("SOKOL_GLES3", None);
        }
        SokolBackend::Gles2 => {
            build.define("SOKOL_GLES2", None);
        }
        SokolBackend::Gl => {
            build.define("SOKOL_GLCORE33", None);
        }
        SokolBackend::Wgpu => {
            build.define("SOKOL_WGPU", None);
        }
    }

    let files = [
        "sokol_log.c",
        "sokol_app.c",
        "sokol_gfx.c",
        "sokol_glue.c",
        "sokol_time.c",
        "sokol_audio.c",
        "sokol_gl.c",
        "sokol_debugtext.c",
        "sokol_shape.c",
    ];

    //
    // include paths
    //
    build.include(BASE_C_DIR);

    build.define("IMPL", None);

    //
    // silence some warnings
    //
    build.flag_if_supported("-Wno-unused-parameter");
    build.flag_if_supported("-Wno-missing-field-initializers");

    for file in &files {
        let file = format!("{BASE_C_DIR}{file}");

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
    // x86_64-pc-windows-gnu: additional compile/link flags
    //
    if cfg!(target_os = "windows") {
        if !is_msvc {
            build.define("_WIN32_WINNIT", Some("0x0601"));

            build
                .flag("-D_WIN32_WINNT=0x0601")
                .flag_if_supported("-Wno-cast-function-type")
                .flag_if_supported("-Wno-sign-compare")
                .flag_if_supported("-Wno-unknown-pragmas");

            println!("cargo:rustc-link-lib=static=kernel32");
            println!("cargo:rustc-link-lib=static=user32");
            println!("cargo:rustc-link-lib=static=gdi32");
            println!("cargo:rustc-link-lib=static=ole32");

            if backend == SokolBackend::D3d11 {
                println!("cargo:rustc-link-lib=static=d3d11");
                println!("cargo:rustc-link-lib=static=dxgi");
            }

            // TODO: Something else needed here..?
        }
    }

    if debug_info_requested {
        build.define("_DEBUG", None).define("SOKOL_DEBUG", None);
    }

    println!("cargo:rustc-link-lib=static=sokol-rust");
    println!("cargo:rustc-link-search=src/sokol/c/");
    println!("cargo:rustc-link-search=target/debug/");

    //
    // MacOS: frameworks
    //
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=Audiocc::Toolbox");

        if backend == SokolBackend::Metal {
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=MetalKit");
        } else if backend == SokolBackend::Gl
            || backend == SokolBackend::Gles2
            || backend == SokolBackend::Gles3
        {
            println!("cargo:rustc-link-lib=framework=OpenGL");
        } else {
            todo!("Handle WGPU backend on macos")
        }
    }

    //
    // Linux: libs
    //
    if cfg!(target_os = "linux") {
        if force_egl {
            build.define("SOKOL_FORCE_EGL", None);
        }

        if wayland_desired {
            build.define("SOKOL_DISABLE_X11 ", None);
        } else {
            build.define("SOKOL_DISABLE_WAYLAND", None);
        }

        println!("cargo:rustc-link-lib=dylib=asound");

        if backend == SokolBackend::Gles2 {
            println!("cargo:rustc-link-lib=dylib=glesv2");
            assert!(force_egl || wayland_desired);
        } else {
            println!("cargo:rustc-link-lib=dylib=GL");
        }

        if force_egl || wayland_desired {
            println!("cargo:rustc-link-lib=dylib=egl");
        }

        if wayland_desired {
            println!("cargo:rustc-link-lib=dylib=wayland-client");
            println!("cargo:rustc-link-lib=dylib=wayland-cursor");
            println!("cargo:rustc-link-lib=dylib=wayland-egl");
            println!("cargo:rustc-link-lib=dylib=xkbcommon");
        } else {
            println!("cargo:rustc-link-lib=dylib=X11");
            println!("cargo:rustc-link-lib=dylib=Xi");
            println!("cargo:rustc-link-lib=dylib=Xcursor");
        }

        println!("cargo:rustc-link-lib=dylib=GL");
    }

    build.compile("sokol-rust");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    make_sokol();
}
