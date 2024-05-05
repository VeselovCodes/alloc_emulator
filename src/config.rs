use std::fs;
use std::io::ErrorKind;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub num_chunks: u32,
    pub buffer_size: u32,
    pub buffer_liveness: u32,
    pub avg_chunk_size: u32,
    pub min_chunk_size: u32,
    pub max_chunk_size: u32,
    pub avg_chunk_liveness: u32,
    pub min_chunk_liveness: u32,
    pub max_chunk_liveness: u32,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            num_chunks: 0,
            buffer_size: 0,
            buffer_liveness: 0,
            avg_chunk_size: 0,
            min_chunk_size: 0,
            max_chunk_size: 0,
            avg_chunk_liveness: 0,
            min_chunk_liveness: 0,
            max_chunk_liveness: 0,
        }
    }
    pub fn new(
        num_chunks: u32,
        buffer_size: u32,
        buffer_liveness: u32,
        avg_chunk_size: u32,
        min_chunk_size: u32,
        max_chunk_size: u32,
        avg_chunk_liveness: u32,
        min_chunk_liveness: u32,
        max_chunk_liveness: u32,
    ) -> Self {
        Self {
            num_chunks: num_chunks,
            buffer_size: buffer_size,
            buffer_liveness: buffer_liveness,
            avg_chunk_size: avg_chunk_size,
            min_chunk_size: min_chunk_size,
            max_chunk_size: max_chunk_size,
            avg_chunk_liveness: avg_chunk_liveness,
            min_chunk_liveness: min_chunk_liveness,
            max_chunk_liveness: max_chunk_liveness,
        }
    }
}

pub fn readConfig(config_full_path: String) -> Config {
    let mut cfg = Config::empty();
    let contents = fs::read_to_string(config_full_path).expect("no such config file");
    let lines = contents.split('\n');
    let warn_message = |line: &str| {
        println!("can't parse line: {}", line);
    };
    for l in lines {
        let substrs = l.split(':').collect::<Vec<&str>>();
        if substrs.len() != 2 {
            warn_message(l);
            continue;
        }
        let token = substrs[0].trim();
        let mut value: u32 = 0;
        match substrs[1].trim().parse::<u32>() {
            Ok(n) => value = n,
            Err(e) => {
                warn_message(l);
                continue;
            }
        }
        match token {
            "num_chunks" => cfg.num_chunks = value,
            "buffer_size" => cfg.buffer_size = value,
            "buffer_liveness" => cfg.buffer_liveness = value,
            "avg_chunk_size" => cfg.avg_chunk_size = value,
            "min_chunk_size" => cfg.min_chunk_size = value,
            "max_chunk_size" => cfg.max_chunk_size = value,
            "avg_chunk_liveness" => cfg.avg_chunk_liveness = value,
            "min_chunk_liveness" => cfg.min_chunk_liveness = value,
            "max_chunk_liveness" => cfg.max_chunk_liveness = value,
            _ => {
                warn_message(l);
                continue;
            }
        }
    }
    cfg
}
