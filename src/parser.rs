use regex_lite::Regex;

use crate::utils::{And, Map, Or};

pub trait Parser<'input>: Sized {
    type Output;

    fn parse(&self, input: &'input str) -> Option<(Self::Output, &'input str)>;

    fn map<F, U>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Output) -> U,
    {
        Map(self, f)
    }

    fn and<P>(self, other: P) -> And<Self, P>
    where
        P: Parser<'input>,
    {
        And(self, other)
    }

    fn or<P>(self, other: P) -> Or<Self, P>
    where
        P: Parser<'input>,
    {
        Or(self, other)
    }
}

impl<'input, T, F> Parser<'input> for F
where
    F: Fn(&'input str) -> Option<(T, &'input str)>,
{
    type Output = T;

    fn parse(&self, input: &'input str) -> Option<(Self::Output, &'input str)> {
        (self)(input)
    }
}

impl<'input> Parser<'input> for &'input str {
    type Output = &'input str;

    fn parse(&self, input: &'input str) -> Option<(Self::Output, &'input str)> {
        let re = Regex::new(self).expect("regex failed to compile");
        let end = re.find_at(input, 0)?.end();

        Some((&input[..end], &input[end..]))
    }
}

impl<'input> Parser<'input> for char {
    type Output = char;

    fn parse(&self, input: &'input str) -> Option<(Self::Output, &'input str)> {
        input.strip_prefix(|ch| ch == *self).map(|s| (*self, s))
    }
}
