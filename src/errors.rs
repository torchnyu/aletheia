use failure::Error;

#[derive(Debug, Fail)]
pub enum AletheiaError {
    #[fail(display = "Config error: {}", message)]
    ConfigError { message: String },
}

pub type Result<T> = std::result::Result<T, Error>;
