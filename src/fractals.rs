use std::pin::Pin;
use std::marker::PhantomPinned;

use crate::core::Parser;


#[derive(Debug)]
struct FractalParser<P>{
    parser: Option<P>,
    _pin: PhantomPinned,
}

impl<'a, P: 'static> FractalParser<P>
where
    P: Parser<'a>
{
    fn new<F>(maker: F) -> Pin<Box<Self>>
    where
        F: Fn(ParserRef<'a, P::Output>) -> P,
    {
        let mut pinned_self = Box::pin(Self {parser: None, _pin: PhantomPinned});
        let parser = maker(ParserRef(&(*pinned_self) as *const FractalParser<P>));
        unsafe {
            pinned_self.as_mut().get_unchecked_mut().parser = Some(parser);
        }

        pinned_self
    }

    fn source(&self) -> &P {
        if let Some(parser) = &self.parser {
            parser
        } else {
            unreachable!();
        }
    }
}

impl<'a, P: 'static> Parser<'a> for FractalParser<P>
where
    P: Parser<'a>,
{
    type Output = P::Output;
    fn call(&self, s: &'a str) -> Option<Self::Output> {
        self.source().call(s)
    }

    fn min_len(&self) -> usize {self.source().min_len()}
    fn max_len(&self) -> usize {self.source().max_len()}
}

#[derive(Copy, Clone)]
struct ParserRef<'a, O>(*const dyn Parser<'a, Output = O>);

impl<'a, O> ParserRef<'a, O> {
    unsafe fn source_ref(&self) -> &dyn Parser<'a, Output = O> {
        match self.0.as_ref() {
            Some(x) => x,
            None => unreachable!(),
        }
    }
}

impl<'a, O> Parser<'a> for ParserRef<'a, O>
{
    type Output = O;

    fn call(&self, s: &'a str) -> Option<Self::Output> {
        unsafe {self.source_ref()}.call(s)
    }
    fn min_len(&self) -> usize {0}
    fn max_len(&self) -> usize {usize::MAX}
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
                    Seq2Rev(fractal, Seq2Rev(Str(&" + "), fractal))
                        .post(|opt| opt.map(|(x1, (_s, x2))| x1+x2)),
                    Seq2Rev(fractal, Seq2Rev(Str(&" * "), fractal))
                        .post(|opt| opt.map(|(x1, (_s, x2))| x1*x2)),
                ),
            )
        );

        assert_eq!(parser.call(&"1 + 2 * 3 + 4"), Some(11));
    }
}
