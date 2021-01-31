use std::rc::Rc;
use std::pin::Pin;
use std::marker::PhantomPinned;

use crate::core::Parser;


#[derive(Debug)]
struct FractalParser<P>{
    parser: Option<P>,
    _pin: PhantomPinned,
}

impl<'a, P> FractalParser<P>
where
    P: Parser<'a>
{
    fn new<F>(maker: F) -> Pin<Rc<Self>>
    where
        F: Fn(Pin<Rc<dyn Parser<'a, Output = P::Output>>>) -> P,
    {
        let mut pinned_self = Rc::pin(Self {parser: None, _pin: PhantomPinned});

        let parser = maker(pinned_self);
        unsafe {
            pinned_self.as_mut().get_unchecked_mut().parser = Some(parser);
        }

        pinned_self
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

impl<'a, O> Parser<'a> for Pin<Rc<dyn Parser<'a, Output = O>>>
{
    type Output = O;

    fn call(&self, s: &'a str) -> Option<Self::Output> {
        self.call(s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{Alt2, Seq2Rev, Digits, Str};

    #[test]
    fn test_fractal_parser() {
        let parser = FractalParser::new(|fractal|
            Alt2(
                Digits(10).post(|opt| opt.and_then(|s| s.parse::<u32>().ok())),
                Alt2(
                    Seq2Rev(
                        fractal,
                        Seq2Rev(
                            Str(&" * "),
                            fractal,
                        ),
                    ).post(|opt| opt.map(|(x1, (_s, x2))| x1*x2)),
                    Seq2Rev(
                        fractal,
                        Seq2Rev(
                            Str(&" + "),
                            fractal,
                        ),
                    ).post(|opt| opt.map(|(x1, (_s, x2))| x1+x2)),
                ),
            )
        );

        assert_eq!(parser.call(&"1 + 2 * 3 + 4"), Some(11));
    }
}
