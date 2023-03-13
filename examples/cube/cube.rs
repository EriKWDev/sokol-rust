//------------------------------------------------------------------------------
//  cube/cube.rs
//
//  Renders a rotating cube.
//------------------------------------------------------------------------------

mod math;
mod shader;

use math as m;
use sokol::app as sapp;
use sokol::gfx as gl;

struct State {
    rx: f32,
    ry: f32,

    pip: gl::Pipeline,
    bind: gl::Bindings,
}

static mut STATE: State = State {
    rx: 0.0,
    ry: 0.0,
    pip: gl::Pipeline::new(),
    bind: gl::Bindings::new(),
};

extern "C" fn init() {
    let state = unsafe { &mut STATE };

    gl::setup(&gl::Desc {
        context: sokol::glue::context(),

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
    state.bind.vertex_buffers[0] = gl::make_buffer(&gl::BufferDesc {
        data: gl::slice_as_range(VERTICES),
        ..Default::default()
    });

    // create an index buffer for the cube
    #[rustfmt::skip]
    const INDICES: &[u16] = &[
        0,  1,  2,   0,  2,  3,
        6,  5,  4,   7,  6,  4,
        8,  9,  10,  8,  10, 11,
        14, 13, 12,  15, 14, 12,
        16, 17, 18,  16, 18, 19,
        22, 21, 20,  23, 22, 20,
    ];

    state.bind.index_buffer = gl::make_buffer(&gl::BufferDesc {
        data: gl::slice_as_range(INDICES),
        _type: gl::BufferType::Indexbuffer,
        ..Default::default()
    });

    // shader and pipeline object

    state.pip = gl::make_pipeline(&gl::PipelineDesc {
        shader: gl::make_shader(&shader::cube_shader_desc(gl::query_backend())),
        layout: {
            let mut layout = gl::LayoutDesc::new();
            layout.buffers[0].stride = 28;

            layout.attrs[shader::ATTR_VS_POSITION].format = gl::VertexFormat::Float3;
            layout.attrs[shader::ATTR_VS_COLOR0].format = gl::VertexFormat::Float4;

            layout
        },

        index_type: gl::IndexType::Uint16,
        cull_mode: gl::CullMode::Back,

        depth: gl::DepthState {
            write_enabled: true,
            compare: gl::CompareFunc::LessEqual,

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
    let vs_params = shader::VsParams {
        mvp: compute_mvp(state.rx, state.ry),
    };

    let mut pass_action = gl::PassAction::new();
    pass_action.colors[0] = gl::ColorAttachmentAction {
        action: gl::Action::Clear,
        value: gl::Color {
            r: 0.25,
            g: 0.5,
            b: 0.75,
            a: 1.0,
        },
    };

    gl::begin_default_pass(&pass_action, sapp::width(), sapp::height());
    gl::apply_pipeline(state.pip);
    gl::apply_bindings(&state.bind);
    gl::apply_uniforms(
        gl::ShaderStage::Vs,
        shader::SLOT_VS_PARAMS,
        &gl::value_as_range(&vs_params),
    );
    gl::draw(0, 36, 1);
    gl::end_pass();
    gl::commit();
}

pub fn compute_mvp(rx: f32, ry: f32) -> [[f32; 4]; 4] {
    let proj = m::persp_mat4(60.0, sapp::widthf() / sapp::heightf(), 0.01, 10.0);
    let view = m::lookat_mat4(m::vec3(0.0, 1.5, 6.0), m::Vec3::ZERO, m::Vec3::UP);
    let view_proj = m::mul_mat4(proj, view);
    let rxm = m::rotate_mat4(rx, m::vec3(1.0, 0.0, 0.0));
    let rym = m::rotate_mat4(ry, m::vec3(0.0, 1.0, 0.0));
    let model = m::mul_mat4(rxm, rym);

    m::mul_mat4(view_proj, model)
}

extern "C" fn cleanup() {
    gl::shutdown()
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

        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },

        ..Default::default()
    });
}
