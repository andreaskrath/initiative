pub mod content;
pub mod dashboard;
pub mod request;
pub mod spell;

enum State<Loader, Data> {
    Loading(Box<Loader>),
    Active(Box<Data>),
}
