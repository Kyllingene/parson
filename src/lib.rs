use regex_lite::Regex;

#[macro_export]
macro_rules! grammar {
    ($( $vis:vis $name:ident : $t:ty = {
        $(
            $( $($bind:ident )? : $e:tt )+ => $res:expr
        ),+ $(,)?
    }; )*) => {};
}

pub trait Parser<'input>: Sized {
    type Output;

    fn parse(&self, input: &'input str) -> Option<(Self::Output, &'input str)>;

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

pub struct And<L, R>(L, R);

impl<'parser, L, R> Parser<'parser> for And<L, R>
where
    L: Parser<'parser>,
    R: Parser<'parser>,
{
    type Output = (L::Output, R::Output);

    fn parse(&self, input: &'parser str) -> Option<(Self::Output, &'parser str)> {
        let (l, input) = self.0.parse(input)?;
        let (r, output) = self.1.parse(input)?;
        Some(((l, r), output))
    }
}

pub enum Either<L, R> {
    L(L),
    R(R),
}

pub struct Or<L, R>(L, R);

impl<'parser, L, R> Parser<'parser> for Or<L, R>
where
    L: Parser<'parser>,
    R: Parser<'parser>,
{
    type Output = Either<L::Output, R::Output>;

    fn parse(&self, input: &'parser str) -> Option<(Self::Output, &'parser str)> {
        if let Some((l, output)) = self.0.parse(input) {
            Some((Either::L(l), output))
        } else {
            self.1.parse(input).map(|(r, o)| (Either::R(r), o))
        }
    }
}

impl<'parser> Parser<'parser> for () {
    type Output = ();

    fn parse(&self, input: &'parser str) -> Option<(Self::Output, &'parser str)> {
        Some(((), input))
    }
}

// macro_rules! tuple_impls {
//     ( $( $n:ident ),+ ) => {
//         impl<'parser, $($n,)+ > Parser for ($($n,)+)
//         where
//             $( $n: Parser<'input> )+
//         {
//             type Output = ($($n::Output,)+);

//             fn parse(&self, input: &'input str) -> Option<(Self::Output, &'input str)> {
//                 () $(
//                     .and()
//                 )+
//                 .parse(input)
//             }
//         }
//     };
// }

// tuple_impls! { A }
// tuple_impls! { A B }
// tuple_impls! { A B C }
// tuple_impls! { A B C D }
// tuple_impls! { A B C D E }
// tuple_impls! { A B C D E F }
// tuple_impls! { A B C D E F G }
// tuple_impls! { A B C D E F G H }
// tuple_impls! { A B C D E F G H I }
// tuple_impls! { A B C D E F G H I J }
// tuple_impls! { A B C D E F G H I J K }
// tuple_impls! { A B C D E F G H I J K L }
