pub type GroupId = std::num::NonZeroU32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub id: Option<GroupId>,
    pub default_print_mode: crate::ir::shared::PrintMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub expected_mode: crate::ir::shared::PrintMode,
    pub target_group_id: GroupId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalGroup {
    pub default_print_mode: crate::ir::shared::PrintMode,
    pub condition: Condition,
}
