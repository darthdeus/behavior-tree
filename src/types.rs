#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Initialized,
    Success,
    Failure,
    Running,
}

impl core::default::Default for Status {
    fn default() -> Self {
        Self::Initialized
    }
}
