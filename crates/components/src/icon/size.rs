#[derive(Debug, Clone)]
pub enum IconSize {
    /// 12px.
    Small,

    /// 16px.
    Medium,

    /// 20 px.
    Large,
}

impl IconSize {
    pub(super) fn wh(self) -> (u32, u32) {
        match self {
            IconSize::Small => (12, 12),
            IconSize::Medium => (16, 16),
            IconSize::Large => (20, 20),
        }
    }
}
