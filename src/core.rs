pub trait Parser<'a, U> {
    fn call(&self, s: &'a str) -> Option<U>;
}
