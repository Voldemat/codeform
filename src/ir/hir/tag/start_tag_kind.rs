#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentMode {
    Hard,
    Soft,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartTagKind {
    Indent(IndentMode),
    Group(super::shared::Group),
    ConditionalGroup(super::shared::ConditionalGroup),
}

impl StartTagKind {
    pub fn as_indent(self: &Self) -> Option<&IndentMode> {
        match self {
            Self::Indent(indent_mode) => Some(indent_mode),
            _ => None,
        }
    }

    pub fn as_group(self: &Self) -> Option<&super::shared::Group> {
        match self {
            Self::Group(group) => Some(group),
            _ => None,
        }
    }

    pub fn as_conditional_group(
        self: &Self,
    ) -> Option<&super::shared::ConditionalGroup> {
        match self {
            Self::ConditionalGroup(conditional_group) => {
                Some(conditional_group)
            }
            _ => None,
        }
    }
}
