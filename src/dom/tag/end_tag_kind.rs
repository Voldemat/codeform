use super::start_tag_kind::StartTagKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndTagKind {
    Indent,
    Group,
    ConditionalGroup,
}

impl From<StartTagKind> for EndTagKind {
    fn from(value: StartTagKind) -> Self {
        match value {
            StartTagKind::Indent => Self::Indent,
            StartTagKind::Group(_) => Self::Group,
            StartTagKind::ConditionalGroup(_) => Self::ConditionalGroup,
        }
    }
}
