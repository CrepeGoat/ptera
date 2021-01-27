pub trait Parser<'a, U> {
	fn call(&mut self, s: &'a str) -> Option<U>;
}
