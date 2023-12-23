use crate::Parser;

#[macro_export]
macro_rules! chain {
    ( $method:ident, $input:expr, $( $p:expr ),+  $(,)? ) => {{
        let next = ();

        $(
            let next = next.$method($p);
        )+

        next
    }};
}

#[macro_export]
macro_rules! switch {
    ( $input:expr, $( $p:expr ),+ $(,)? ) => {{
        let input = $input;
        $(
            if let Some((res, output)) = ($p).parse(input) {
                return Some((res, output));
            }
        )+
        None
    }};
}

pub struct And<L, R>(pub L, pub R);

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

pub struct Or<L, R>(pub L, pub R);

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

pub struct Map<P, F>(pub P, pub F);

impl<'parser, P, F, U> Parser<'parser> for Map<P, F>
where
    P: Parser<'parser>,
    F: Fn(P::Output) -> U,
{
    type Output = U;

    fn parse(&self, input: &'parser str) -> Option<(Self::Output, &'parser str)> {
        self.0.parse(input).map(|(t, o)| ((self.1)(t), o))
    }
}

impl<'parser> Parser<'parser> for () {
    type Output = ();

    fn parse(&self, input: &'parser str) -> Option<(Self::Output, &'parser str)> {
        Some(((), input))
    }
}

macro_rules! impl_tuple {
    ( $( $t:ident )+ ) => {
        impl<'parser, $( $t, )+> Parser<'parser> for ( $($t,)+ )
        where
            $( $t: Parser<'parser>, )+
            {
            type Output = ($( <$t as Parser<'parser>>::Output, )+);

            #[allow(nonstandard_style)]
            fn parse(&self, input: &'parser str) ->  Option<(Self::Output, &'parser str)> {
                let ($($t,)+) = self;
                $(
                    let ($t, input) = $t.parse(input)?;
                )+

                Some((($($t,)+), input))
            }
        }
    };
}

impl_tuple! { A }
impl_tuple! { A B }
impl_tuple! { A B C }
impl_tuple! { A B C D }
impl_tuple! { A B C D E }
impl_tuple! { A B C D E F }
impl_tuple! { A B C D E F G }
impl_tuple! { A B C D E F G H }
impl_tuple! { A B C D E F G H I }
impl_tuple! { A B C D E F G H I J }
impl_tuple! { A B C D E F G H I J K }
impl_tuple! { A B C D E F G H I J K L }
impl_tuple! { A B C D E F G H I J K L M }
impl_tuple! { A B C D E F G H I J K L M N }
impl_tuple! { A B C D E F G H I J K L M N O }
impl_tuple! { A B C D E F G H I J K L M N O P }
impl_tuple! { A B C D E F G H I J K L M N O P Q }
impl_tuple! { A B C D E F G H I J K L M N O P Q R }
impl_tuple! { A B C D E F G H I J K L M N O P Q R S }
impl_tuple! { A B C D E F G H I J K L M N O P Q R S T }
impl_tuple! { A B C D E F G H I J K L M N O P Q R S T U }
impl_tuple! { A B C D E F G H I J K L M N O P Q R S T U V }
impl_tuple! { A B C D E F G H I J K L M N O P Q R S T U V W }
impl_tuple! { A B C D E F G H I J K L M N O P Q R S T U V W X }
impl_tuple! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
impl_tuple! { A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }
