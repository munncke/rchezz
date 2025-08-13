#[macro_export]
macro_rules! make_enum {
    ($name:ident, $payload:ty,[$( ($variant:ident, $code:expr) ),*]) => {
        #[derive(Debug)]
        pub enum $name { $( $variant($payload) ),* }
        paste! {
            impl $name {
        $(
        pub fn[<$variant:snake>] (
            message: &str,
        ) -> Self {
            Self::$variant($payload {
                code: $code,
                message: message.to_string(),
                #[cfg(debug_assertions)]
                context: $crate::error::ErrorContext {
                     line: line!(),
                     column: column!(),
                     filename: file!().to_string(),
                    },
                })
            }
        )*
            }
        }
    };
}
