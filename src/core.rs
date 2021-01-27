pub trait Parser<'a> {
    type Output;

    fn call(&self, s: &'a str) -> Option<Self::Output>;
}
