pub mod button;
pub mod icon;
pub mod rule;
pub mod text;
pub mod theme;
pub mod typography;

pub use theme::ThemeVariant;
pub use typography::Typography;

/// A scale used to adjust alpha for muted styles.
const MUTED_SCALE: f32 = 0.6;
