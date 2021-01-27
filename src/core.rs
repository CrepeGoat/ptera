pub trait Parser<'a> {
    type Output;

    fn call(&self, s: &'a str) -> Option<Self::Output>;

    fn map<V, P, F>(self, func: F) -> MappedParser<P, F>
        where P: Parser<'a>,
              F: Fn(U) -> V,
    {
        MappedParser::new(self, func)
    }
}


struct MappedParser<P, F> {
    parser: P,
    mapping: F,
}

impl<P, F> MappedParser<P, F> {
    fn new(parser: P, func: F) -> Self {
        MappedParser {parser, mapping: func}
    }
}


impl<'a, V, P, F> Parser<'a> for MappedParser<F, P>
    where P: Parser<'a>,
          F: Fn(P::Output) -> V,
{
    type Output = V;

    fn call(&self, s: &'a str) -> Option<Self::Output> {
        self.parser.call(s).map(self.mapping)
    }
}
