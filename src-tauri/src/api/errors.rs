use custom_error::custom_error;

custom_error! {pub GeoCodingError
    NetworkError = "No network connection",
    InvalidLocationError= "Invalid location",
    MissingCoordinatesError = "Coordinates missing"
}