

pub mod token {
}#[macro_export]
macro_rules! build_lexer {
    ( $( $token:expr ),* ) => {
        ars::lexer::Lexer::new(vec![ $( $token ),* ])
    };
}

#[macro_export]
macro_rules! build_kinds {
    ( $name:ident, $( $kind:ident ),* ) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $( $kind ),*
        }

        impl $name {
            pub fn kind(&self) -> u32 {
                *self as u32
            }

            pub fn to_string(&self) -> String {
                match self {
                    $( $name::$kind => stringify!($kind).to_string(), )*
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", self.to_string(), self.kind())
            }
        }
    };
}