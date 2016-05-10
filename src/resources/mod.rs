use std::collections::HashMap;

pub mod file_index;
pub mod art;
pub mod tex_maps;
pub mod maps;

use resources::art::read_land;
//use resources::file_index::{read_index, FileIndex};
use resources::maps::{MapBlock, read_block};
use gfx::Resources as ResourcesTrait;
use gfx::Factory;
use gfx::handle::ShaderResourceView;

pub struct Resources {
    //art_index: Vec<FileIndex>,
    //land_textures: HashMap<usize, ShaderResourceView<ResourcesTrait, [f32; 4]>>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            //art_index: read_index("tmp/artidx.mul"),
            //land_textures: HashMap::new(),
        }
    }

    pub fn get_land<R, F>(&mut self, factory: &mut F, index: usize) -> ShaderResourceView<R, [f32; 4]>
        where R: ResourcesTrait, F: Factory<R> {
        //self.land_textures.entry(index).or_insert_with(|| read_land(factory, index))
        read_land(factory, index)
    }

    pub fn get_map_block(&self, x: i32, y: i32) -> MapBlock {
        read_block(x, y)
    }
}
