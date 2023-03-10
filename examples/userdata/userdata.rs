#[derive(Debug)]
pub struct UserData {
    data: u8,

    map: std::collections::HashMap<u8, u8>,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            data: 0,
            map: Default::default(),
        }
    }
}

extern "C" fn init() {
    sokol::gfx::setup(&sokol::gfx::Desc {
        context: sokol::glue::context(),

        ..Default::default()
    });
}

extern "C" fn frame() {
    /*
        NOTE: We then need to turn the raw c pointer into a mutable reference to the same
              type as we created in main. This is safe as long as the data was indeed provided
              and the type is the same.
    */
    let state: &mut UserData = unsafe { &mut *(sokol::app::userdata() as *mut _) };

    /*
        NOTE: Just randomly modifying the data here for demonstration
    */
    state.data = state.data.wrapping_add(1);
    if state.data % 13 == 0 {
        let val = (state.data.wrapping_mul(13)) / 3;
        state.map.insert(state.data, val);
    }
    if state.data % 12 == 0 && state.data % 15 == 0 {
        state.map.clear();
    }
    println!("{state:?}");

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
    let window_title = b"test\0".as_ptr() as _;

    let mut user_data = UserData::default();

    sokol::app::run(&sokol::app::Desc {
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        frame_cb: Some(frame),

        /*
            NOTE: 'user_data' is allocated on the stack in the main function and we take a
                  mutable reference to it which we cast to a pointer and pass to sokol
        */
        user_data: &mut user_data as *mut _ as _,

        width: 800,
        height: 600,
        sample_count: 4,
        window_title,

        icon: sokol::app::IconDesc {
            sokol_default: true,
            ..Default::default()
        },

        ..Default::default()
    });
}
