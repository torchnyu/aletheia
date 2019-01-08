use failure::Error;

#[derive(Debug, Fail)]
pub enum AletheiaError {
    #[fail(display = "Config error: {}", message)]
    ConfigError { message: String },
    #[fail(display = "Repo date {} is after start date", date)]
    DateError { date: String },
}

pub type Result<T> = std::result::Result<T, Error>;
