use thiserror::*;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Current level is higher or equal to the desired level.")]
    SmallerLevel,
    #[error("Please insert a number")]
    InvalidValue,
}
