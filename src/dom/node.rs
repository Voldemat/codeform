#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LineMode {
    SoftOrSpace,
    Soft,
    Hard,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BestFittingMode {
    FirstLine,
    AllLines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text<'s> {
    pub text: &'s str,
    pub width: crate::shared::TextWidth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DOMNode<'s> {
    Space,
    Line(LineMode),
    Token(&'s str),
    Text(Text<'s>),
    Tag(super::tag::Tag),
}

impl<'s> DOMNode<'s> {
    pub fn as_tag(self: &Self) -> Option<&super::tag::Tag> {
        match self {
            Self::Tag(tag) => Some(tag),
            _ => None,
        }
    }
}
