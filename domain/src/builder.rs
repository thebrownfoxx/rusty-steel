#[derive(Debug)]
pub struct Builder<Impl>(Impl);

impl<Impl> Builder<Impl> {
    pub fn new(base: Impl) -> Self {
        Self(base)
    }

    pub fn reimplement<New>(self, function: impl FnOnce(Impl) -> New) -> Builder<New> {
        Builder(function(self.0))
    }

    pub fn build(self) -> Impl {
        self.0
    }
}
