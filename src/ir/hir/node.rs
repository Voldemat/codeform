use super::tag::Tag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LineMode {
    SoftOrSpace,
    Soft,
    Hard,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'s> {
    Byte(u8),
    Line(LineMode),
    AsciiOnelineText(&'s str),
    Text(super::text::Text<'s>),
    Tag(Tag),
}

impl<'s> Node<'s> {
    pub fn as_tag(self: &Self) -> Option<&super::tag::Tag> {
        match self {
            Self::Tag(tag) => Some(tag),
            _ => None,
        }
    }
}
