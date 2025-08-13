use crate::make_enum;
use paste::paste;
#[derive(Debug)]
pub struct ErrorContext {
    line: u32,
    column: u32,
    filename: String,
}
#[derive(Debug)]
pub struct BaseError {
    code: u16,
    message: String,
    #[cfg(debug_assertions)]
    context: ErrorContext,
}

make_enum!(AppError, BaseError, [(InputError, 1), (BoardError, 2)]);
