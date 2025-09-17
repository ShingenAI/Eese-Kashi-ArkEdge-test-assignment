#[derive(Debug)]
pub struct Command {
    pub index: usize,
    pub start: u32,
    pub end: u32,
}

impl Command {
    pub fn new(index: usize, start: u32, duration: u32) -> Self {
        Self {
            index,
            start,
            end: start + duration,
        }
    }
}