use crate::core::Parser;


#[derive(Debug)]
struct FractalParser<F, P>
{
    active_parser: Option<P>,
    make_parser: F,
}

impl<'a, U, F, P> FractalParser<'a, U, F, P>
    where F: Fn(&dyn Parser<'a, U>) -> P,
          P: Parser<'a, U>,
{
    fn new(maker: F) -> Self {
        Self {
            active_parser: None,
            make_parser: maker,
        }
    }
}

impl<'a, U, F, P> Parser<'a, U> for FractalParser<'a, U, F, P>
    where F: Fn(&dyn Parser<'a, U>) -> P,
          P: Parser<'a, U>,
{
    fn call(&mut self, s: &'a str) -> Option<U> {
        if let Some(parser) = self.active_parser {
            return parser(s);
        }
        self.active_parser = Some(self.maker(&self));
        let result = self.call(s);
        self.active_parser = None;
        result
    }
}