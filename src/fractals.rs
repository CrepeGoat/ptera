use crate::core::Parser;


#[derive(Debug)]
struct FractalParser<P, F>
{
    active_parser: Option<P>,
    make_parser: F,
}

impl<F, P> FractalParser<P, F> {
    fn new(maker: F) -> Self {
        Self {
            active_parser: None,
            make_parser: maker,
        }
    }
}

impl<'a, P, F> Parser<'a> for FractalParser<P, F>
where
    P: Parser<'a>,
    F: Fn(&dyn Parser<'a, Output=P::Output>) -> P,
{
    type Output = P::Output;

    fn call(&mut self, s: &'a str) -> Option<Self::Output> {
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
    use crate::parsers::{Alt2, Seq2Fwd, Seq2Rev, Digits, Str};

    #[test]
    fn test_fractal_parser() {
        let mut parser = FractalParser::new(|fractal|
            Alt2(
                Digits(10).map(|s| s.parse::<u32>().unwrap()),
                Alt2(
                    Seq2Rev(
                        fractal,
                        Seq2Rev(
                            Str(&" * "),
                            fractal,
                        ),
                    ).map(|(x1, (s, x2))| x1*x2),
                    Seq2Rev(
                        fractal,
                        Seq2Rev(
                            Str(&" + "),
                            fractal,
                        ),
                    ).map(|(x1, (s, x2))| x1+x2),
                ),
            )
        );

        assert_eq!(parser.call(&"1 + 2 * 3 + 4"), Some(11));
    }
}