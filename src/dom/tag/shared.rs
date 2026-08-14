#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DedentMode {
    Level,
    Root,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupMode {
    Flat,
    Expand,
}

impl From<crate::shared::PrintMode> for GroupMode {
    fn from(value: crate::shared::PrintMode) -> Self {
        match value {
            crate::shared::PrintMode::Flat => Self::Flat,
            crate::shared::PrintMode::Expanded => Self::Expand,
        }
    }
}

pub type GroupId = std::num::NonZeroU32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub id: Option<GroupId>,
    pub mode: std::cell::Cell<GroupMode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub expected_mode: crate::shared::PrintMode,
    pub target_group_id: GroupId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalGroup {
    pub mode: std::cell::Cell<GroupMode>,
    pub condition: Condition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbatimKind {
    Bogus,
    Suppressed,
    Verbatim { length: std::num::NonZeroU32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FitsExpanded {
    pub condition: Option<Condition>,
    pub propagate_expand: std::cell::Cell<bool>,
}
