use std::pin::Pin;

use crate::core::Parser;


#[derive(Debug)]
struct FractalParser<P>{parser: Option<P>}

impl<'a, P> FractalParser<P>
where
    P: Parser<'a> + std::marker::Unpin
{
    fn new<F>(maker: F) -> Self
    where
        F: Fn(Pin<&dyn Parser<'a, Output = P::Output>>) -> P,
    {
        let mut self_ = Self {parser: None};
        let self_pin = Pin::new(&self_);
        let parser = maker(self_pin);
        self_.parser = Some(parser);

        self_
    }
}

impl<'a, P> Parser<'a> for FractalParser<P>
where
    P: Parser<'a>,
{
    type Output = P::Output;

    fn call(&self, s: &'a str) -> Option<Self::Output> {
        if let Some(parser) = &self.parser {
            return parser.call(s);
        }
        unreachable!();
    }
}

impl<'a, O> Parser<'a> for Pin<&dyn Parser<'a, Output = O>>
{
    type Output = O;

    fn call(&self, s: &'a str) -> Option<Self::Output> {
        self.call(s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{Alt2, Seq2Fwd, Seq2Rev, Digits, Str};

    #[test]
    fn test_fractal_parser() {
        let mut parser = FractalParser::new(|fractal|
            Alt2(
                Digits(10).post(|opt| opt.and_then(|s| s.parse::<u32>().ok())),
                Alt2(
                    Seq2Rev(
                        fractal,
                        Seq2Rev(
                            Str(&" * "),
                            fractal,
                        ),
                    ).post(|opt| opt.map(|(x1, (s, x2))| x1*x2)),
                    Seq2Rev(
                        fractal,
                        Seq2Rev(
                            Str(&" + "),
                            fractal,
                        ),
                    ).post(|opt| opt.map(|(x1, (s, x2))| x1+x2)),
                ),
            )
        );

        assert_eq!(parser.call(&"1 + 2 * 3 + 4"), Some(11));
    }
}
