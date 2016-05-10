use std::io::prelude::*;
use std::fs::File;
use std::io::Cursor;
use std::io::SeekFrom;
use byteorder::{ReadBytesExt, LittleEndian};
use resources::file_index::get_index;
use gfx::Resources as ResourcesTrait;
use gfx::Factory;
use gfx::handle::ShaderResourceView;
use gfx::tex;
use gfx::format::Rgba8;

pub fn read_land<R, F>(factory: &mut F, id: usize) -> ShaderResourceView<R, [f32; 4]>
    where R: ResourcesTrait, F: Factory<R> {
    let land = get_index("tmp/artidx.mul", id as u64);

    let mut file = File::open("tmp/art.mul").unwrap();

    file.seek(SeekFrom::Start(land.lookup as u64)).unwrap();
    let mut land_buf = vec![0; land.size as usize];
    file.read_exact(&mut land_buf).unwrap();
    let mut land_cursor = Cursor::new(land_buf);

    let mut buffer: [u8; 7744] = [0; 7744];
    let mut x = 22;
    let mut y = 0;
    let mut line_width = 2;
    let pitch = 176;

    for _ in 0..22 {
        x -= 1;
        for i in 0..line_width {
            let offset = (y * pitch) + (x * 4) + (i * 4);
            let color_16: u16 = land_cursor.read_u16::<LittleEndian>().unwrap();
            let color_32: i32 = (((color_16 as i32 >> 10) & 0x1F) * 0xFF / 0x1F) |
                                ((((color_16 as i32 >> 5) & 0x1F) * 0xFF / 0x1F) << 8) |
                                ((( color_16 as i32 & 0x1F) * 0xFF / 0x1F) << 16);

            let bytes: [u8; 4] = [color_32 as u8 & u8::max_value(), 
                                 (color_32 >> 8) as u8 & u8::max_value(),
                                 (color_32 >> 16) as u8 & u8::max_value(),
                                 (color_32 >> 24) as u8 & u8::max_value()];
            buffer[offset] = bytes[0];
            buffer[offset + 1] = bytes[1];
            buffer[offset + 2] = bytes[2];
            buffer[offset + 3] = 255;
        }
        y += 1;
        line_width += 2;
    }

    line_width -= 2;

    for _ in 0..22 {
        for i in 0..line_width {
            let offset = (y * pitch) + (x * 4) + (i * 4);
            let color_16: u16 = land_cursor.read_u16::<LittleEndian>().unwrap();
            let color_32: u32 = (((color_16 as u32 >> 10) & 0x1F) * 0xFF / 0x1F) |
                                ((((color_16 as u32 >> 5) & 0x1F) * 0xFF / 0x1F) << 8) |
                                ((( color_16 as u32 & 0x1F) * 0xFF / 0x1F) << 16);

            let bytes: [u8; 4] = [color_32 as u8 & u8::max_value(), 
                                 (color_32 >> 8) as u8 & u8::max_value(),
                                 (color_32 >> 16) as u8 & u8::max_value(),
                                 (color_32 >> 24) as u8 & u8::max_value()];
            buffer[offset] = bytes[0];
            buffer[offset + 1] = bytes[1];
            buffer[offset + 2] = bytes[2];
            buffer[offset + 3] = 255;
        }
        x += 1;
        y += 1;
        line_width -= 2;
    }

    let kind = tex::Kind::D2(44 as tex::Size, 44 as tex::Size, tex::AaMode::Single);
    let (_, view) = factory.create_texture_const_u8::<Rgba8>(kind, &[&buffer]).unwrap();
    view
}

/*
pub fn read_land(renderer: &Renderer, id: usize) -> Texture {
    let land = get_index("tmp/artidx.mul", id as u64);

    let mut file = File::open("tmp/art.mul").unwrap();

    file.seek(SeekFrom::Start(land.lookup as u64)).unwrap();
    let mut land_buf = vec![0; land.size as usize];
    file.read_exact(&mut land_buf).unwrap();
    let mut land_cursor = Cursor::new(land_buf);

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::ARGB1555, 44, 44).unwrap();
    texture.set_blend_mode(BlendMode::Blend);
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let mut x = 22;
        let mut y = 0;
        let mut line_width = 0;

        for _ in 0..22 {
            x -= 1;
            line_width += 2;
            for i in 0..line_width {
                let offset = (y * pitch) + (x * 2) + (i * 2);
                let color_1 = land_cursor.read_u8().unwrap();
                let mut color_2 = land_cursor.read_u8().unwrap();
                color_2 = color_2 | (1 << 7);
                buffer[offset] = color_1;
                buffer[offset + 1] = color_2;
            }
            y += 1;
        }

        for _ in 0..22 {
            for i in 0..line_width {
                let offset = (y * pitch) + (x * 2) + (i * 2);
                let color_1 = land_cursor.read_u8().unwrap();
                let mut color_2 = land_cursor.read_u8().unwrap();
                color_2 = color_2 | (1 << 7);
                buffer[offset] = color_1;
                buffer[offset + 1] = color_2;
            }
            x += 1;
            y += 1;
            line_width -= 2;
        }
    }).unwrap();

    texture
}
*/
