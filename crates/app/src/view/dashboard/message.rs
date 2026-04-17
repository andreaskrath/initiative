#[derive(Debug, Clone)]
pub enum DashboardMessage {
    ValueChanged(String),
    ValueRemoved(usize),
    ValueAdded,
}
