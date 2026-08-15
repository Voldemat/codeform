#[derive(Debug, Clone)]
pub struct MaybeWillFitState {
    pub remaining_width: u32,
    pub group_depth: u32,
}

impl MaybeWillFitState {
    pub fn with_decreased_remaining_width(
        self: &Self,
        diff_width: u32,
    ) -> Self {
        Self {
            remaining_width: self.remaining_width - diff_width,
            group_depth: self.group_depth,
        }
    }

    pub fn with_decremented_group_depth(self: &Self) -> Self {
        Self {
            remaining_width: self.remaining_width,
            group_depth: self.group_depth.saturating_sub(1),
        }
    }

    pub fn with_incremented_group_depth(self: &Self) -> Self {
        Self {
            remaining_width: self.remaining_width,
            group_depth: self.group_depth.saturating_add(1),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeFitState {
    DefinitelyFits,
    DoesNotFit,
    MaybeWillFit(MaybeWillFitState),
}

impl NodeFitState {
    pub fn to_print_mode(self: &Self) -> crate::ir::shared::PrintMode {
        match self {
            Self::DefinitelyFits | Self::MaybeWillFit(_) => {
                crate::ir::shared::PrintMode::Flat
            }
            Self::DoesNotFit => crate::ir::shared::PrintMode::Expanded,
        }
    }
}
