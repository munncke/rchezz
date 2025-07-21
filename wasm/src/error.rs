use crate::make_enum;
#[derive(Debug)]
struct ErrorContext {
    line: usize,
    column: usize,
    filename: String,
}
#[derive(Debug)]
struct BaseError {
    code: u16,
    message: String,
    #[cfg(debug_assertions)]
    context: ErrorContext,
}


make_enum!(AppError, BaseError,[InputError]);

