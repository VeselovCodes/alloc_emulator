use std::env;

use alloc_driver::allocate;
use buffer::BufferState;
use chunk::Chunk;
use chunk_generator::generate_chunks;
use config::read_config;
use config::Config;
use correctness_checker::check_buffer_states;

mod alloc_driver;
mod buffer;
mod chunk;
mod chunk_generator;
mod config;
mod correctness_checker;

// imput: -params of static buffer
//        -alloc functor
//        -chunk generator
fn main() {
    let config_full_path: String = {
        // Default config
        let mut conf_arg = String::from("./init.cfg");
        let args: Vec<String> = env::args().collect();
        if args.len() >= 2 {
            for i in 1..(args.len() - 2) {
                if args[i] == "-i" {
                    conf_arg = args[i + 1].clone();
                    break;
                }
            }
        }
        conf_arg
    };

    println!("Read config from file: {config_full_path}");
    let config: Config = read_config(config_full_path);
    let config_ref: &Config = &config;
    let chunks: Vec<Chunk> = generate_chunks(config_ref);
    let buffer_states: Vec<BufferState> = allocate(chunks, config_ref);
    check_buffer_states(buffer_states, config_ref);
}
