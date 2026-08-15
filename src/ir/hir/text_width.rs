#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextWidth {
    Width(super::width::Width),
    Multiline,
}

impl TextWidth {
    pub fn from_text<C: Fn(char) -> usize>(
        text: &str,
        indent_width: crate::ir::shared::IndentWidth,
        compute_char_width: C,
    ) -> TextWidth {
        text.chars().fold(
            TextWidth::Width(super::width::Width::new(0)),
            |current_text_width, c| {
                let TextWidth::Width(current_width) = current_text_width else {
                    return current_text_width;
                };
                match c {
                    '\n' => TextWidth::Multiline,
                    '\t' => {
                        TextWidth::Width(current_width + indent_width.value())
                    }
                    other_char => TextWidth::Width(
                        current_width + compute_char_width(other_char) as u32,
                    ),
                }
            },
        )
    }

    pub fn width(self) -> Option<super::width::Width> {
        match self {
            TextWidth::Width(width) => Some(width),
            TextWidth::Multiline => None,
        }
    }

    pub fn is_multiline(self) -> bool {
        matches!(self, TextWidth::Multiline)
    }
}
