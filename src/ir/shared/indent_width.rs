#[derive(Debug, Clone, Copy)]
pub struct IndentWidth(std::num::NonZeroU8);

impl IndentWidth {
    pub fn from_u8(value: u8) -> Option<Self> {
        std::num::NonZeroU8::try_from(value).ok().map(|v| Self(v))
    }

    pub const fn value(&self) -> u32 {
        self.0.get() as u32
    }
}

impl TryFrom<u8> for IndentWidth {
    type Error = std::num::TryFromIntError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        std::num::NonZeroU8::try_from(value).map(Self)
    }
}

impl From<std::num::NonZeroU8> for IndentWidth {
    fn from(value: std::num::NonZeroU8) -> Self {
        Self(value)
    }
}
