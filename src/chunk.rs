#[derive(Debug, Clone, Copy)]
pub struct Chunk {
    _size: u32,
    _liveness_start: u32,
    _liveness: u32,
}

impl Chunk {
    pub fn new(size: u32, liveness_start: u32, liveness: u32) -> Self {
        Self {
            _size: size,
            _liveness_start: liveness_start,
            _liveness: liveness,
        }
    }
}
