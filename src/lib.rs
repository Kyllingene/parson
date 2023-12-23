mod parser;
pub mod utils;

pub use parser::Parser;

#[macro_export]
macro_rules! grammar {
    (
        $mv:vis mod $mn:ident;
        use [$( $path:tt )*];
        $( $vis:vis $name:ident : $t:ty = {
            $(
                $( $bind:ident : $e:tt )+ => $res:expr
            ),+ $(,)?
        }; )*
    ) => {
        $mv mod $mn {
            use {$( $path )*};
            pub mod parser {
                $(
                    pub struct $name;
                )*
            }
        $(
            impl<'parser> $crate::Parser<'parser> for parser::$name {
                type Output = $t;

                fn parse(&self, input: &'parser str) -> Option<(Self::Output, &'parser str)> {
                    $crate::switch! {
                        $(
                            input,
                            ($( $crate::__expr_path!($e), )+)
                                .map(|($( $bind, )+)| { $res }),
                        )+
                    }
                }
            }
        )*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __expr_path {
    ( $parser:ident ) => {
        parser::$parser
    };
    ( $other:tt ) => {
        $other
    };
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
