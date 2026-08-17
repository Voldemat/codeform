#[derive(Debug, Clone)]
pub struct CurrentState {
    pub indent_level: crate::ir::shared::IndentLevel,
    pub align_spaces: u8,
    pub print_mode: crate::ir::shared::PrintMode,
    pub enabled: bool,
    pub expected_end_tag_kind: Option<crate::ir::hir::tag::EndTagKind>,
}

impl CurrentState {
    pub fn with_indent(
        mut self: Self,
        append_level: crate::ir::shared::IndentLevel,
    ) -> Self {
        self.indent_level += append_level;
        self
    }

    pub fn with_print_mode(
        mut self: Self,
        print_mode: crate::ir::shared::PrintMode,
    ) -> Self {
        self.print_mode = print_mode;
        self
    }

    pub fn with_expected_end_tag_kind(
        mut self: Self,
        expected_end_tag_kind: Option<crate::ir::hir::tag::EndTagKind>,
    ) -> Self {
        self.expected_end_tag_kind = expected_end_tag_kind;
        self
    }
}

impl Default for CurrentState {
    fn default() -> Self {
        Self {
            indent_level: 0,
            align_spaces: 0,
            print_mode: crate::ir::shared::PrintMode::Expanded,
            enabled: true,
            expected_end_tag_kind: None,
        }
    }
}

pub struct State {
    pub current_line_width: crate::ir::shared::CurrentLineWidth,
    pub states_stack: Vec<CurrentState>,
    pub group_mode_map: std::collections::HashMap<
        crate::ir::hir::tag::GroupId,
        crate::ir::shared::PrintMode,
    >,
}

impl State {
    #[inline]
    pub fn advance(self: &mut Self, value: u32) {
        self.current_line_width += value;
    }

    #[inline]
    pub fn active_indent_level(self: &Self) -> crate::ir::shared::IndentLevel {
        self.states_stack
            .last()
            .map(|s| s.indent_level)
            .unwrap_or(0)
    }

    #[inline]
    pub fn active_align_spaces(self: &Self) -> u8 {
        self.states_stack
            .last()
            .map(|s| s.align_spaces)
            .unwrap_or(0)
    }

    #[inline]
    pub fn reset_line(
        self: &mut Self,
        indent_width: crate::ir::shared::IndentWidth,
    ) {
        self.current_line_width = indent_width.value()
            * self.active_indent_level() as u32
            + self.active_align_spaces() as u32;
    }

    #[inline]
    pub fn active_mode(&self) -> crate::ir::shared::PrintMode {
        self.states_stack.last().unwrap().print_mode
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_line_width: 0,
            states_stack: vec![CurrentState::default()],
            group_mode_map: std::collections::HashMap::new(),
        }
    }
}
