use super::SchoolAdapter;

#[derive(Debug)]
pub struct Adapter {}

impl Adapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl SchoolAdapter for Adapter {}
