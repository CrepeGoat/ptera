pub trait Parser<'a> {
    type Output;

    fn call(&self, s: &'a str) -> Option<Self::Output>;

    fn map<V, F>(self, func: F) -> MappedParser<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> V,
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

impl<'a, V, P, F> Parser<'a> for MappedParser<P, F>
where
    P: Parser<'a>,
    F: Fn(P::Output) -> V,
{
    type Output = V;

    fn call(&self, s: &'a str) -> Option<Self::Output> {
        self.parser.call(s).map(&self.mapping)
    }
}
