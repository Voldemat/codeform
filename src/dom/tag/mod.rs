mod end_tag_kind;
pub mod shared;
mod start_tag_kind;
pub use end_tag_kind::EndTagKind;
pub use start_tag_kind::StartTagKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    Start(StartTagKind),
    End(EndTagKind),
}

impl Tag {
    pub fn as_start(self: &Self) -> Option<&StartTagKind> {
        match self {
            Self::Start(kind) => Some(kind),
            Self::End(_) => None,
        }
    }

    pub fn as_end(self: &Self) -> Option<&EndTagKind> {
        match self {
            Self::Start(_) => None,
            Self::End(kind) => Some(kind),
        }
    }
}
