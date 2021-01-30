use crate::core::Parser;


#[derive(Debug)]
pub struct Alt2<P1, P2>(pub P1, pub P2);

impl<'b, 'a: 'b, P1, P2> Parser<'b, 'a> for Alt2<P1, P2>
    where P1: Parser<'b, 'a>,
          P2: Parser<'b, 'a, Output=P1::Output>
{
    type Output = P1::Output;
    fn call(&self, s: &'a str) -> Option<Self::Output> {
        self.0.call(&s).or_else(|| self.1.call(&s))
    }
}


#[derive(Debug)]
pub struct Seq2Fwd<P1, P2>(pub P1, pub P2);

impl<'b, 'a: 'b, P1, P2> Parser<'b, 'a> for Seq2Fwd<P1, P2>
where P1: Parser<'b, 'a>,
      P2: Parser<'b, 'a, Output=P1::Output>,
{
    type Output = (P1::Output, P2::Output);
    fn call(&self, s: &'a str) -> Option<Self::Output> {
        (0..=s.len()).filter_map(
            |i| self.0.call(&s[..i]).and_then(
                |u1| self.1.call(&s[i..]).map(|u2| (u1, u2))
            )
        ).next()
    }
}


#[derive(Debug)]
pub struct Seq2Rev<P1, P2>(pub P1, pub P2);

impl<'b, 'a: 'b, P1, P2> Parser<'b, 'a> for Seq2Rev<P1, P2>
where P1: Parser<'b, 'a>,
      P2: Parser<'b, 'a, Output=P1::Output>,
{
    type Output = (P1::Output, P2::Output);
    fn call(&self, s: &'a str) -> Option<Self::Output> {
        (0..=s.len()).rev().filter_map(
            |i| self.0.call(&s[..i]).and_then(
                |u1| self.1.call(&s[i..]).map(|u2| (u1, u2))
            )
        ).next()
    }
}


#[derive(Debug)]
pub struct Digits(pub u32);

impl<'b, 'a: 'b> Parser<'b, 'a> for Digits {
    type Output = &'a str;

    fn call(&self, s: &'a str) -> Option<&'a str> {
        if s.chars().all(|c| c.is_digit(self.0)) {
            Some(s)
        } else {
            None
        }
    }   
}


#[derive(Debug)]
pub struct Str<'b>(pub &'b str);

impl<'b, 'a: 'b> Parser<'b, 'a> for Str<'b> {
    type Output = &'a str;

    fn call(&self, s: &'a str) -> Option<&'a str> {
        if s == self.0 {
            Some(s)
        } else {
            None
        }
    }   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        assert_eq!(Str(&"Thing A").call(&"Thing A"), Some("Thing A"));
        assert_eq!(Str(&"Thing A").call(&"another thing"), None);
    }

    #[test]
    fn test_digits() {
        assert_eq!(Digits(10).call(&"some text with a 4 or w/e"), None);
        assert_eq!(Digits(10).call(&"4582"), Some("4582"));

        assert_eq!(Digits(10).call(&"458a2"), None);
        assert_eq!(Digits(16).call(&"458a2"), Some("458a2"));
    }

    #[test]
    fn test_seq2_fwd() {
        let mut parser = Seq2Fwd(Digits(10), Str(&" apples"));

        assert_eq!(parser.call(&"4 apples"), Some(("4", " apples")));
        assert_eq!(parser.call(&"7 oranges"), None);
        assert_eq!(parser.call(&"four apples"), None);

        assert_eq!(Seq2Fwd(Digits(10), Digits(10)).call(&"123"), Some(("", "123")));
    }

    #[test]
    fn test_seq2_rev() {
        let mut parser = Seq2Rev(Digits(10), Str(&" apples"));

        assert_eq!(parser.call(&"4 apples"), Some(("4", " apples")));
        assert_eq!(parser.call(&"7 oranges"), None);
        assert_eq!(parser.call(&"four apples"), None);

        assert_eq!(Seq2Rev(Digits(10), Digits(10)).call(&"123"), Some(("123", "")));
    }

    #[test]
    fn test_alt2() {
        let mut parser = Alt2(Str(&"hello"), Str(&"'ello"));

        assert_eq!(parser.call(&"hello"), Some("hello"));
        assert_eq!(parser.call(&"'ello"), Some("'ello"));
        assert_eq!(parser.call(&"bye"), None);
    }
}
