use std::{
    convert::TryInto,
    env, f32,
    ops::Deref,
    path,
    time::{Duration, Instant},
};

use gfx::{self, texture, traits::FactoryExt};
use ggez::{graphics, Context};
use image::RgbaImage;
use ndarray::ArrayView3;

// ColorFormat and DepthFormat are hardwired into ggez's drawing code,
// and there isn't a way to easily change them, so for the moment we just have
// to know what they are and use the same settings.
type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 4] = "aPos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),

        resolution: gfx::Global<[f32; 3]> = "iResolution",
        time: gfx::Global<f32> = "iTime",
        tex: gfx::TextureSampler<[f32; 4]> = "iTexture",

        out_color: gfx::RenderTarget<ColorFormat> = "outColor",
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    fn new(p: [i8; 3]) -> Vertex {
        Vertex {
            pos: [f32::from(p[0]), f32::from(p[1]), f32::from(p[2]), 1.0],
        }
    }
}

pub struct GfxRenderer {
    data: pipe::Data<gfx_device_gl::Resources>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    slice: gfx::Slice<gfx_device_gl::Resources>,

    start_time: Instant,
}

impl GfxRenderer {
    pub fn new_shadertoy(ctx: &mut Context, shadertoy_shader: &str) -> Self {
        Self::new_glsl(ctx, &shadertoy_wrap(shadertoy_shader).into_bytes())
    }

    pub fn new_glsl(ctx: &mut Context, fragment_shader: &[u8]) -> Self {
        let (w, h) = ggez::graphics::drawable_size(ctx);
        let (factory, _device, _encoder, depth_view, color_view) = graphics::gfx_objects(ctx);

        let vertex_shader =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/vertex.glsl"));

        let vertex_data = [
            // Top left
            Vertex::new([-1, -1, 0]),
            // Top right
            Vertex::new([1, -1, 0]),
            // Bottom left
            Vertex::new([-1, 1, 0]),
            // Bottom right
            Vertex::new([1, 1, 0]),
        ];

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let index_data: &[u16] = &[
            0, 1, 3,
            0, 2, 3,
        ];

        // Create vertex buffer
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

        // Create pipeline state object
        let pso = factory
            .create_pipeline_simple(vertex_shader, fragment_shader, pipe::new())
            .unwrap();

        let texture = gfx_load_texture(factory, 1, 1, &[255; 4]);
        let sampler = factory.create_sampler_linear();

        let data = pipe::Data {
            vbuf,
            time: 0.0,
            resolution: [w, h, 1.0],
            tex: (texture, sampler),

            // We use the (undocumented-but-useful) gfx::memory::Typed here
            // to convert ggez's raw render and depth buffers into ones with
            // compile-time type information.
            out_color: gfx::memory::Typed::new(color_view),
            out_depth: gfx::memory::Typed::new(depth_view),
        };

        GfxRenderer {
            start_time: Instant::now(),
            data,
            pso,
            slice,
        }
    }

    pub fn set_image_from_cell_array(&mut self, ctx: &mut Context, cell_array: ArrayView3<u8>) {
        let (height, width, depth) = cell_array.dim();
        assert_eq!(depth, 4);

        let (factory, _device, _encoder, _depthview, _colorview) = graphics::gfx_objects(ctx);

        let texture = gfx_load_texture(
            factory,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            cell_array.as_slice().unwrap(),
        );
        let sampler = factory.create_sampler_linear();

        self.data.tex = (texture, sampler);
    }

    pub fn set_image_from_image(&mut self, ctx: &mut Context, image: &RgbaImage) {
        let (width, height) = image.dimensions();

        let (factory, _device, _encoder, _depthview, _colorview) = graphics::gfx_objects(ctx);

        let texture = gfx_load_texture(
            factory,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            image.as_flat_samples().samples.deref(),
        );
        let sampler = factory.create_sampler_linear();

        self.data.tex = (texture, sampler);
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let (w, h) = ggez::graphics::drawable_size(ctx);
        let (_factory, device, encoder, _depthview, _colorview) = graphics::gfx_objects(ctx);

        self.data.time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.data.resolution[0] = w;
        self.data.resolution[1] = h;

        encoder.clear(&self.data.out_color, [0.0, 0.0, 0.0, 1.0]);

        encoder.clear_depth(&self.data.out_depth, 1.0);

        encoder.draw(&self.slice, &self.pso, &self.data);
        encoder.flush(device);
    }
}

fn shadertoy_wrap(shader: &str) -> String {
    format!(
        r#"#version 450 core
uniform vec3 iResolution;
uniform float iTime;
uniform sampler2D iTexture;

void mainImage(out vec4 fragColor, in vec2 fragCoord);

{}

out vec4 outColor;

void main(){{
    vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
    mainImage(color, gl_FragCoord.xy);
    color.w = 1.0;
    outColor = color;
}}"#,
        shader
    )
}

fn gfx_load_texture<F, R>(
    factory: &mut F,
    width: u16,
    height: u16,
    image_data: &[u8],
) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
where
    F: gfx::Factory<R>,
    R: gfx::Resources,
{
    let (_, texture_view) = factory
        .create_texture_immutable_u8::<gfx::format::Rgba8>(
            texture::Kind::D2(width, height, texture::AaMode::Single),
            texture::Mipmap::Provided,
            &[image_data],
        )
        .unwrap();
    texture_view
}
