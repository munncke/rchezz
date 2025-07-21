#[macro_export]
macro_rules! make_enum {
    ($name:ident, $payload:ty, [$( $variant:ident ),*]) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant($payload),
            )*
        }
    };
}
