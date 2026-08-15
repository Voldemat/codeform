#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineMode {
    Normal,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'s> {
    Byte(u8),
    Line(LineMode),
    Text(&'s str),
    Tag(super::tag::Tag),
}
