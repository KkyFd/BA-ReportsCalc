use thiserror::*;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Current level is higher or equal to the desired level.")]
    SmallerLevel,
    #[error("Please insert a number")]
    InvalidValue,
}

/* impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::SmallerLevel => write!(f, "C"),
            AppError::InvalidValue => write!(f, ""),
        }
    }
} */
