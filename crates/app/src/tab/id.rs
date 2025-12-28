use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TabId(u64);

impl TabId {
    /// Generate and return a new unique `TabId`.
    pub fn unique() -> TabId {
        TabId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    /// Calculate the numeric difference between two `TabId`s.
    ///
    /// Given that the raw value of a `TabId` is `u64`, the difference is absolute to avoid underflow issues.
    pub fn difference(&self, other: TabId) -> u64 {
        self.raw().abs_diff(other.raw())
    }

    fn raw(&self) -> u64 {
        self.0
    }
}
