struct State {
    pass_action: sokol::gfx::PassAction,
}

static mut STATE: State = State {
    pass_action: sokol::gfx::PassAction::new(),
};

extern "C" fn init() {
    sokol::gfx::setup(&sokol::gfx::Desc {
        context: sokol::glue::context(),

        ..Default::default()
    });

    let state = unsafe { &mut STATE };

    state.pass_action.colors[0] = sokol::gfx::ColorAttachmentAction {
        action: sokol::gfx::Action::Clear,
        value: sokol::gfx::Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },

        ..Default::default()
    };

    let backend = sokol::gfx::query_backend();
    match &backend {
        sokol::gfx::Backend::Glcore33 | sokol::gfx::Backend::Gles2 | sokol::gfx::Backend::Gles3 => {
            println!("Using GL Backend!")
        }

        sokol::gfx::Backend::D3d11 => {
            println!("Using D3d11 Backend!")
        }

        sokol::gfx::Backend::MetalIos
        | sokol::gfx::Backend::MetalMacos
        | sokol::gfx::Backend::MetalSimulator => {
            println!("Using Metal Backend!")
        }

        sokol::gfx::Backend::Wgpu => {
            println!("Using Wgpu Backend!")
        }

        sokol::gfx::Backend::Dummy => {
            println!("Using Dymmy Backend!")
        }
    }

    println!("Specifically the {:?} backend!", backend);
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

    let g = state.pass_action.colors[0].value.g + 0.01;
    state.pass_action.colors[0].value.g = if g > 1.0 { 0.0 } else { g };

    let (width, height) = (sokol::app::width(), sokol::app::height());

    sokol::gfx::begin_default_pass(&state.pass_action, width, height);
    sokol::gfx::end_pass();
    sokol::gfx::commit();
}

extern "C" fn cleanup() {
    sokol::gfx::shutdown()
}

fn main() {
    let title = std::ffi::CString::new("clear").unwrap();

    sokol::app::run(&sokol::app::Desc {
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        frame_cb: Some(frame),

        width: 800,
        height: 600,
        sample_count: 4,
        window_title: title.as_ptr(),

        icon: sokol::app::IconDesc {
            sokol_default: true,
            ..Default::default()
        },

        ..Default::default()
    });
}
