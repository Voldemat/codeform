#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Width(std::num::NonZeroU32);

impl Width {
    pub fn new(width: u32) -> Self {
        Width(std::num::NonZeroU32::MIN.saturating_add(width))
    }

    pub fn value(self) -> u32 {
        self.0.get() - 1
    }
}

impl std::ops::Add<u32> for Width {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        let _ = self.0.saturating_add(rhs);
        self
    }
}
