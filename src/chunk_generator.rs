use crate::chunk::Chunk;
use crate::config::Config;

pub fn generateChunks(conf: &Config) -> Vec<Chunk> {
    let buf_size = conf.buffer_size;
    let buf_liveness = conf.buffer_liveness;
    let num_chunks = conf.num_chunks;
    let max_size = conf.max_chunk_size;
    let avg_size = conf.avg_chunk_size;
    let min_size = conf.min_chunk_size;
    let max_liveness = conf.max_chunk_liveness;
    let avg_liveness = conf.avg_chunk_liveness;
    let min_liveness = conf.min_chunk_liveness;
    let mut chunks: Vec<Chunk> = Vec::new();
    chunks
}
