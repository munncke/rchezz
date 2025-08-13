use crate::error::AppError;

#[derive(Debug)]
pub struct Board {
    height: u8,
    width: u8,
}
impl Board {
    pub fn new(height: u8, width: u8) -> Result<Board, AppError> {
        if height > 20 || height < 8 {
            return Err(AppError::board_error(
                "Height must be between 8 or 20",
            ));
        }
        if width > 20 || width < 8 {
            return Err(AppError::board_error("Width must be between 8 or 20"));
        }
        Ok(Board { height, width })
    }
}
