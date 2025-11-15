use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum FormMode {
    Create,
    Edit(Uuid),
}
