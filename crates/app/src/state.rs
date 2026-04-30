pub enum State<Loader, Data> {
    Loading(Box<Loader>),
    Active(Box<Data>),
}
