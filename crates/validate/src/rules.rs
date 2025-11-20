#[derive(Debug)]
pub enum Rule {
    Required,
    Minimum(usize),
    Maximum(usize),
    Between(usize, usize),
}
