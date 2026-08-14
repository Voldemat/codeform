use super::Width;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextWidth {
    Width(Width),
    Multiline,
}

impl TextWidth {
    pub fn from_text<C: Fn(u8) -> u32>(
        text: &str,
        indent_width: crate::shared::IndentWidth,
        compute_unicode_width: C,
    ) -> TextWidth {
        text.bytes().fold(
            TextWidth::Width(Width::new(0)),
            |current_text_width, byte| {
                let TextWidth::Width(current_width) = current_text_width else {
                    return current_text_width;
                };
                if byte == b'\n' {
                    return TextWidth::Multiline;
                };
                let byte_width = match byte {
                    b'\t' => indent_width.value(),
                    ascii_byte if matches!(ascii_byte, b' '..=b'~') => 1,
                    unicode_byte => compute_unicode_width(unicode_byte),
                };
                TextWidth::Width(current_width + byte_width)
            },
        )
    }

    pub fn width(self) -> Option<Width> {
        match self {
            TextWidth::Width(width) => Some(width),
            TextWidth::Multiline => None,
        }
    }

    pub fn is_multiline(self) -> bool {
        matches!(self, TextWidth::Multiline)
    }
}
