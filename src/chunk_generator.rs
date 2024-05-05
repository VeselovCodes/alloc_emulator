use crate::chunk::Chunk;
use crate::config::Config;

use rand_distr::{Beta, Distribution};
use std::cmp;

fn get_rand_val(min: f64, avg: f64, max: f64) -> f64 {
    let diff = max - min;
    let rel_avg = (avg - min) / diff;
    // find alpha and beta to apply beta distribution
    // mean is a/(a+b) = rel_avg
    let b = 2.0;
    let a = rel_avg * b / (1.0 - rel_avg);
    let beta = Beta::new(a, b).unwrap();
    beta.sample(&mut rand::thread_rng()) * diff + min
}

pub fn generate_chunks(conf: &Config) -> Vec<Chunk> {
    let _buf_size = conf.buffer_size;
    let buf_liveness = conf.buffer_liveness;
    let num_chunks = conf.num_chunks as usize;
    let max_size = conf.max_chunk_size;
    let avg_size = conf.avg_chunk_size;
    let min_size = conf.min_chunk_size;
    let max_liveness = conf.max_chunk_liveness;
    let avg_liveness = conf.avg_chunk_liveness;
    let min_liveness = conf.min_chunk_liveness;
    let mut chunks: Vec<Chunk> = Vec::with_capacity(num_chunks);
    let mut cur_start = 0;
    for _ in 0..num_chunks {
        // Liveness and size are created according to beta distribution for now
        let size = get_rand_val(min_size as f64, avg_size as f64, max_size as f64) as u32;
        let liveness = get_rand_val(
            min_liveness as f64,
            avg_liveness as f64,
            max_liveness as f64,
        ) as u32;
        // just use these params for now
        // TODO: build random dataflow
        chunks.push(Chunk::new(
            size,
            cur_start,
            cmp::min(liveness, buf_liveness - cur_start),
        ));
        cur_start = (cur_start + liveness) % buf_liveness;
    }
    chunks
}
