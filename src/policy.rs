pub struct Policy {
    start: i32,
    end: i32,
}

impl Policy {
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}