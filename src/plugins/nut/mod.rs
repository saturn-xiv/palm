pub mod controllers;
pub mod models;
pub mod services;
pub mod v1;

impl v1::Page {
    pub const MIN_SIZE: i64 = 5;
    pub const MAX_SIZE: i64 = 1024;
    pub fn validate(&self, total: i64) -> Self {
        let size = if self.size < Self::MIN_SIZE {
            Self::MIN_SIZE
        } else if self.size > Self::MAX_SIZE {
            Self::MAX_SIZE
        } else {
            self.size
        };

        let mut index = if total % size == 0 {
            total / size
        } else {
            (total / size) + 1
        };
        if index < 1 {
            index = 1;
        }
        Self { size, index }
    }
    pub fn offset(&self) -> i64 {
        (self.index - 1) * self.size
    }
}
