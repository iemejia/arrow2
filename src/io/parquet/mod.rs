//! APIs to read from and write to Parquet format.
use crate::error::Error;

pub mod read;
pub mod write;

const ARROW_SCHEMA_META_KEY: &str = "ARROW:schema";

impl From<parquet2::error::Error> for Error {
    fn from(error: parquet2::error::Error) -> Self {
        match error {
            parquet2::error::Error::FeatureNotActive(_, _) => {
                let message = "Failed to read a compressed parquet file. \
                    Use the cargo feature \"io_parquet_compression\" to read compressed parquet files."
                    .to_string();
                Error::ExternalFormat(message)
            }
            _ => Error::ExternalFormat(error.to_string()),
        }
    }
}

impl From<Error> for parquet2::error::Error {
    fn from(error: Error) -> Self {
        parquet2::error::Error::General(error.to_string())
    }
}
