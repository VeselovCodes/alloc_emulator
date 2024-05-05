use crate::buffer::BufferState;
use crate::chunk::Chunk;
use crate::config::Config;

pub fn allocate(_chunks: Vec<Chunk>, _conf: &Config) -> Vec<BufferState> {
    let states: Vec<BufferState> = Vec::new();
    states
}
