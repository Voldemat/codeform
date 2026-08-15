#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text<'s> {
    pub text: &'s str,
    pub width: super::text_width::TextWidth,
}
