macro_rules! string_error {
    ($name:ident, $prefix:literal $(, from: $($source:ty),+ $(,)?)?) => {
        #[derive(Debug, Clone)]
        pub struct $name(String);

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, concat!($prefix, "{}"), self.0)
            }
        }

        impl std::error::Error for $name {}

        $($(
            impl From<$source> for $name {
                fn from(error: $source) -> Self {
                    $name(error.to_string())
                }
            }
        )+)?
    };
}

pub(crate) use string_error;
