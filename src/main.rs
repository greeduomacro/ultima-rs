extern crate byteorder;
extern crate glutin;

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;

mod resources;

use gfx::traits::FactoryExt;
use gfx::Device;
use resources::{Resources};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_vertex_struct!(Vertex {
    pos: [f32; 2] = "a_Pos",
    uv: [f32; 2] = "a_Uv",
});

gfx_pipeline!(pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out: gfx::RenderTarget<ColorFormat> = "o_Color",
});

/*
fn load_texture<R, F>(factory: &mut F, data: &[u8])
    -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String> where R: gfx::Resources, F: gfx::Factory<R> {
    use std::io::Cursor;
    use gfx::tex as t;
    let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
    let (_, view) = factory.create_texture_const_u8::<ColorFormat>(kind, &[&img]).unwrap();
    Ok(view)
}
*/

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Ultima Online RS".to_string())
        .with_dimensions(800, 600);

    let (window, mut device, mut factory, main_color, _main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let command_buffer = factory.create_command_buffer();
    let mut encoder: gfx::Encoder<_, _> = command_buffer.into();
    let mut resources = Resources::new();

    let quad: [Vertex; 4] = [
        Vertex { pos: [ -0.5, -0.5 ], uv: [0.0, 1.0] },
        Vertex { pos: [ -0.5,  0.5 ], uv: [0.0, 0.0] },
        Vertex { pos: [  0.5, -0.5 ], uv: [1.0, 1.0] },
        Vertex { pos: [  0.5,  0.5 ], uv: [1.0, 0.0] },
    ];

    //let no_tile = load_texture(&mut factory, &include_bytes!("../assets/textures/no_tile.png")[..]).unwrap();
    let no_tile = resources.get_land(&mut factory, 100 as usize);
    let sampler = factory.create_sampler_linear();

    let shader_set = factory.create_shader_set(
        include_bytes!("../assets/shaders/tile_150_v.glsl"),
        include_bytes!("../assets/shaders/tile_150_f.glsl"),
    ).unwrap();

    let pso = factory.create_pipeline_state(
        &shader_set,
        gfx::Primitive::TriangleStrip,
        gfx::state::Rasterizer::new_fill(),
        pipe::new()
    ).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&quad, ());
    let data = pipe::Data {
        vbuf: vertex_buffer,
        texture: (no_tile, sampler),
        out: main_color,
    };

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        encoder.clear(&data.out, [0.1, 0.2, 0.3, 1.0]);
        encoder.draw(&slice, &pso, &data);
        window.swap_buffers().unwrap();
        device.cleanup();
        encoder.flush(&mut device);
    }
}

/*
extern crate sdl2;
extern crate byteorder;

mod resources;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode;

use resources::{Resources};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Ultima Online RS", 1280, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    renderer.set_blend_mode(BlendMode::Blend);
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    let mut resources = Resources::new();
    
    for screen_y in 0..10 {
        for screen_x in 0..10 {
            //let map = resources.get_map_block(199 + screen_x, 210 + screen_y); // Britain
            let map = resources.get_map_block(182 + screen_x, 106 + screen_y);

            for (i, cell) in map.cells.iter().enumerate() {
                let y = i as i32 / 8;
                let x = i as i32 % 8;
                let loc_y = y * 44 + screen_y * 176;
                let loc_x = x * 44 + screen_x * 176;
                let mut iso_y = (loc_x + loc_y) / 2;
                let mut iso_x = (loc_x - loc_y) / 2;
                iso_y = iso_y - 500 + (cell.z as i32 * 4);
                iso_x = iso_x + 1000;

                let tile = resources.get_land(&renderer, cell.tile_id as usize);
                renderer.copy(tile, None, Some(Rect::new(iso_x, iso_y, 44, 44)));
            }
        }
    }

    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
*/
