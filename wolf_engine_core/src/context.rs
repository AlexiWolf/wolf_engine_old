pub struct Context<D> {
    pub data: D,
}

impl<D> From<D> for Context<D> {
    fn from(data: D) -> Self {
        Self { data }
    }
}
