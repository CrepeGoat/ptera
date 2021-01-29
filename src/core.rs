pub trait Parser<'a> {
    type Output;

    fn call(&self, s: &'a str) -> Option<Self::Output>;

    fn post<V, F>(self, func: F) -> PostProcessedParser<Self, F>
    where
        Self: Sized,
        F: Fn(Option<Self::Output>) -> Option<V>,
    {
        PostProcessedParser::new(self, func)
    }
}


pub struct PostProcessedParser<P, F> {
    parser: P,
    mapping: F,
}

impl<P, F> PostProcessedParser<P, F> {
    fn new(parser: P, func: F) -> Self {
        PostProcessedParser {parser, mapping: func}
    }
}

impl<'a, V, P, F> Parser<'a> for PostProcessedParser<P, F>
where
    P: Parser<'a>,
    F: Fn(Option<P::Output>) -> Option<V>,
{
    type Output = V;

    fn call(&self, s: &'a str) -> Option<Self::Output> {
        (&self.mapping)(self.parser.call(s))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::Digits;

    #[test]
    fn test_parser_map() {
        assert_eq!(
            Digits(10).post(|opt| opt.and_then(|s| s.parse::<u32>().ok())).call(&"123"),
            Some(123u32),
        );
    }
}

