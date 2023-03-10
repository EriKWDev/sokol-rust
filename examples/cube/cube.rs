extern "C" fn init() {
    sokol::gfx::setup(&sokol::gfx::Desc {
        context: sokol::glue::context(),

        ..Default::default()
    });
}

extern "C" fn frame() {
    let pass_action = sokol::gfx::PassAction::new();
    let (width, height) = (sokol::app::width(), sokol::app::height());

    sokol::gfx::begin_default_pass(&pass_action, width, height);
    sokol::gfx::end_pass();
    sokol::gfx::commit();
}

extern "C" fn cleanup() {
    sokol::gfx::shutdown()
}

fn main() {
    let title = std::ffi::CString::new("cube").unwrap();

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

        // TODO
        // logger: sokol::app::Logger {
        //     func: Some(sokol::log::slog_func),
        //     ..Default::default()
        // },
        ..Default::default()
    });
}
