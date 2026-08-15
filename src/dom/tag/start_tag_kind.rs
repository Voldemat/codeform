#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartTagKind {
    Indent,
    Group(super::shared::Group),
    ConditionalGroup(super::shared::ConditionalGroup),
}

impl StartTagKind {
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
