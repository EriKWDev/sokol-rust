//------------------------------------------------------------------------------
//  cube/cube.rs
//
//  Renders a rotating cube.
//------------------------------------------------------------------------------

mod shader;

use sokol::{app as sapp, gfx as sg, glue as sglue};

pub struct State {
    pub rx: f32,
    pub ry: f32,

    pub pip: sg::Pipeline,
    pub bind: sg::Bindings,
}

static mut STATE: State = State { rx: 0.0, ry: 0.0, pip: sg::Pipeline::new(), bind: sg::Bindings::new() };

extern "C" fn init() {
    let state = unsafe { &mut STATE };

    sg::setup(&sg::Desc {
        context: sglue::context(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // cube vertex buffer
    #[rustfmt::skip]
    const VERTICES: &[f32] = &[
        -1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
         1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
         1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
        -1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,

        -1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
         1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
         1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
        -1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,

        -1.0, -1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0,  1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0,  1.0,  1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0, 0.0, 1.0, 1.0,

        1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
        1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
        1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,
        1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,

        -1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
         1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
         1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,

        -1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
        -1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
         1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
         1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
    ];
    state.bind.vertex_buffers[0] =
        sg::make_buffer(&sg::BufferDesc { data: sg::slice_as_range(VERTICES), ..Default::default() });

    // create an index buffer for the cube
    #[rustfmt::skip]
    const INDICES: &[u16] = &[
         0,  1,  2,   0,  2,  3,
         6,  5,  4,   7,  6,  4,
         8,  9, 10,   8, 10, 11,
        14, 13, 12,  15, 14, 12,
        16, 17, 18,  16, 18, 19,
        22, 21, 20,  23, 22, 20,
    ];

    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(INDICES),
        _type: sg::BufferType::Indexbuffer,
        ..Default::default()
    });

    // shader and pipeline object

    state.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::cube_shader_desc(sg::query_backend())),
        layout: {
            let mut layout = sg::LayoutDesc::new();
            layout.buffers[0].stride = 28;

            layout.attrs[shader::ATTR_VS_POSITION].format = sg::VertexFormat::Float3;
            layout.attrs[shader::ATTR_VS_COLOR0].format = sg::VertexFormat::Float4;

            layout
        },
        index_type: sg::IndexType::Uint16,
        cull_mode: sg::CullMode::Back,
        depth: sg::DepthState {
            write_enabled: true,
            compare: sg::CompareFunc::LessEqual,
            ..Default::default()
        },
        ..Default::default()
    });
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

    let t = (sapp::frame_duration() * 60.0) as f32;
    state.rx += 1.0 * t;
    state.ry += 2.0 * t;

    // vertex shader uniform with model-view-projection matrix
    let vs_params = shader::VsParams { mvp: compute_mvp(state.rx, state.ry) };

    let mut pass_action = sg::PassAction::new();
    pass_action.colors[0] = sg::ColorAttachmentAction {
        action: sg::Action::Clear,
        value: sg::Color { r: 0.25, g: 0.5, b: 0.75, a: 1.0 },
    };

    sg::begin_default_pass(&pass_action, sapp::width(), sapp::height());
    sg::apply_pipeline(state.pip);
    sg::apply_bindings(&state.bind);
    sg::apply_uniforms(sg::ShaderStage::Vs, shader::SLOT_VS_PARAMS, &sg::value_as_range(&vs_params));
    sg::draw(0, 36, 1);
    sg::end_pass();
    sg::commit();
}

pub fn compute_mvp(rx: f32, ry: f32) -> glam::Mat4 {
    let proj = glam::Mat4::perspective_rh_gl(60.0, sapp::widthf() / sapp::heightf(), 0.01, 10.0);
    let view = glam::Mat4::look_at_rh(glam::vec3(0.0.1 .5, 6.0), glam::Vec3::ZERO, glam::vec3(0.0, 1.0, 0.0));
    let view_proj = proj * view;
    let rxq = glam::Quat::from_rotation_x(rx);
    let ryq = glam::Quat::from_rotation_y(ry);
    let rotation = ryq * rxq;

    // let proj = m::persp_mat4(60.0, sapp::widthf() / sapp::heightf(), 0.01, 10.0);
    // let view = m::lookat_mat4(m::vec3(0.0, 1.5, 6.0), m::Vec3::ZERO, m::Vec3::UP);
    // let view_proj = m::mul_mat4(proj, view);
    // let rxm = m::rotate_mat4(rx, m::vec3(1.0, 0.0, 0.0));
    // let rym = m::rotate_mat4(ry, m::vec3(0.0, 1.0, 0.0));
    // let model = m::mul_mat4(rxm, rym);
    // m::mul_mat4(view_proj, model)
    rotation * view_proj
}

extern "C" fn cleanup() {
    sg::shutdown()
}

fn main() {
    let window_title = b"cube\0".as_ptr() as _;

    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        frame_cb: Some(frame),
        cleanup_cb: Some(cleanup),
        width: 800,
        height: 600,
        sample_count: 4,
        window_title,
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },

        ..Default::default()
    });
}
