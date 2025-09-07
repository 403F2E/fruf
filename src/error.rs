use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum AppError {
    FileNotFound,
    FileDoNotOpen,
    ExistProblem(std::io::Error),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AppError::FileNotFound => write!(f, "File Error: There is no file in the given path."),
            AppError::FileDoNotOpen => write!(f, "File Error: Not able to open the file."),
            Self::ExistProblem(e) => write!(f, "{}", e),
        }
    }
}
