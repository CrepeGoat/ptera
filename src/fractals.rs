use crate::core::Parser;


#[derive(Debug)]
struct FractalParser<F, P>
{
    active_parser: Option<P>,
    make_parser: F,
}

impl<F, P> FractalParser<F, P> {
    fn new(maker: F) -> Self {
        Self {
            active_parser: None,
            make_parser: maker,
        }
    }
}

impl<'a, U, F, P> Parser<'a, U> for FractalParser<F, P>
    where F: Fn(&dyn Parser<'a, U>) -> P,
          P: Parser<'a, U>,
{
    fn call(&mut self, s: &'a str) -> Option<U> {
        if let Some(parser) = &mut self.active_parser {
            return parser.call(s);
        }
        self.active_parser = Some((self.make_parser)(self));
        let result = self.call(s);
        self.active_parser = None;
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractal_parser() {
        unimplemented!();
    }
}