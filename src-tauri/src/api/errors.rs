use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeoCodingError {
    #[error("No network connection")]
    NetworkError,
    #[error("Invalid location")]
    InvalidLocationError,
    #[error("Coordinates missing")]
    MissingCoordinatesError,
}
