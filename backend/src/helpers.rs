use std::fmt::{Debug, Display, Formatter};
use log::warn;
use serde::Deserialize;
use validator::{Validate, ValidationErrors};
use crate::api::contracts;

pub fn validate_request<'de, T>(request: &T) -> Result<(), ValidationError>
    where T: Deserialize<'de> + Validate {
    if let Err(e) = request.validate() {
        let err = ValidationError(e);
        warn!("failed to validate incoming request. reason: {:?}", err);
        return Err(err);
    }

    Ok(())
}

pub struct ValidationError(ValidationErrors);

impl ValidationError {
    pub fn get_validation_errors(&self) -> Vec<contracts::Error> {
        self.0.field_errors().values().flat_map(|errs| {
            errs
                .iter()
                .map(|err| {
                    let message = match err.clone().message {
                        None => format!("{}", err),
                        Some(msg) => msg.to_string()
                    };

                    contracts::Error {
                        message
                    }
                })
        })
            .collect()
    }

    fn get_validation_error_message(&self) -> String {
        self.0.field_errors().values().flat_map(|errs| {
            errs
                .iter()
                .map(|err| {
                    let message = match err.clone().message {
                        None => format!("{}", err),
                        Some(msg) => msg.to_string()
                    };

                    format!("{}\n", message)
                })
        })
            .collect()
    }
}

impl std::error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl Debug for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_validation_error_message())
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "request validation failed")
    }
}