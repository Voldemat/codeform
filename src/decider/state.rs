#[derive(Debug, Clone)]
pub struct CurrentState {
    pub indent_level: crate::shared::IndentLevel,
    pub align_spaces: u8,
    pub print_mode: crate::shared::PrintMode,
    pub expected_end_tag_kind: crate::dom::tag::EndTagKind,
}

impl CurrentState {
    pub fn with_indent(
        mut self: Self,
        append_level: crate::shared::IndentLevel,
    ) -> Self {
        self.indent_level += append_level;
        self
    }
}

pub struct State {
    pub current_line_width: crate::shared::CurrentLineWidth,
    pub mode_stack: Vec<crate::shared::PrintMode>,
    pub states_stack: Vec<CurrentState>,
    pub group_mode_map: std::collections::HashMap<
        crate::dom::tag::shared::GroupId,
        crate::shared::PrintMode,
    >,
}

impl State {
    #[inline]
    pub fn advance(self: &mut Self, value: u32) {
        self.current_line_width += value;
    }

    #[inline]
    pub fn active_indent_level(self: &Self) -> crate::shared::IndentLevel {
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
        indent_width: crate::shared::IndentWidth,
    ) {
        self.current_line_width = indent_width.value()
            * self.active_indent_level() as u32
            + self.active_align_spaces() as u32;
    }

    #[inline]
    pub fn active_mode(&self) -> crate::shared::PrintMode {
        *self
            .mode_stack
            .last()
            .unwrap_or(&crate::shared::PrintMode::Expanded)
    }

    #[inline]
    pub fn push_mode(&mut self, mode: crate::shared::PrintMode) {
        self.mode_stack.push(mode);
    }

    #[inline]
    pub fn pop_mode(&mut self) {
        if self.mode_stack.len() > 1 {
            self.mode_stack.pop();
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            current_line_width: 0,
            states_stack: Vec::new(),
            mode_stack: Vec::new(),
            group_mode_map: std::collections::HashMap::new(),
        }
    }
}
