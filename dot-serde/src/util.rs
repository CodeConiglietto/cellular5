use crate::error::{Error, Result};

pub struct Counter {
    current: u64,
}

impl Counter {
    pub fn new() -> Self {
        Self { current: 0 }
    }

    pub fn next(&mut self) -> Result<u64> {
        if let Some(next) = self.current.checked_add(1) {
            let i = self.current;
            self.current = next;
            Ok(i)
        } else {
            Err(Error::NumericOverflow)
        }
    }
}
