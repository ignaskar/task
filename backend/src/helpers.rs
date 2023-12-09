use crate::api::contracts;
use log::warn;
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};
use validator::{Validate, ValidationErrors};

pub fn validate_request<'de, T>(request: &T) -> Result<(), ValidationError>
where
    T: Deserialize<'de> + Validate,
{
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
        self.0
            .field_errors()
            .values()
            .flat_map(|errs| {
                errs.iter().map(|err| {
                    let message = match err.clone().message {
                        None => format!("{}", err),
                        Some(msg) => msg.to_string(),
                    };

                    contracts::Error { message }
                })
            })
            .collect()
    }

    fn get_validation_error_message(&self) -> String {
        self.0
            .field_errors()
            .values()
            .flat_map(|errs| {
                errs.iter().map(|err| {
                    let message = match err.clone().message {
                        None => format!("{}", err),
                        Some(msg) => msg.to_string(),
                    };

                    format!("{}\n", message)
                })
            })
            .collect()
    }
}

impl std::error::Error for ValidationError {}

impl Debug for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_validation_error_message())
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_validation_error_message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EMAIL_VALIDATION_MESSAGE: &str = "email must be in a valid format";
    const PASSWORD_VALIDATION_MESSAGE: &str = "please enter a password";

    #[test]
    fn get_validation_errors_return_errors_when_validation_fails() {
        // Arrange

        let request = TestLoginRequest {
            email: String::from("not-a-good-format"),
            password: String::from(""),
        };

        let errors = ValidationError(request.validate().err().unwrap());
        let expected_errors = vec![
            contracts::Error {
                message: String::from(EMAIL_VALIDATION_MESSAGE),
            },
            contracts::Error {
                message: String::from(PASSWORD_VALIDATION_MESSAGE),
            },
        ];

        // Act

        let actual = errors.get_validation_errors();

        // Assert

        for expected_error in &expected_errors {
            assert!(
                actual
                    .iter()
                    .any(|actual_error| actual_error.message == expected_error.message),
                "Expected error message not found in actual errors"
            );
        }

        for actual_error in &actual {
            assert!(
                expected_errors
                    .iter()
                    .any(|expected_error| actual_error.message == expected_error.message),
                "Unexpected error message found in actual errors"
            );
        }
    }

    #[test]
    fn get_validation_error_message_returns_formatted_error_when_validation_fails() {
        // Arrange

        let request = TestLoginRequest {
            email: String::from("not-a-good-format"),
            password: String::from(""),
        };

        let errors = ValidationError(request.validate().err().unwrap());
        let expected_error_messages = [EMAIL_VALIDATION_MESSAGE, PASSWORD_VALIDATION_MESSAGE];

        // Act

        let actual = errors.get_validation_error_message();

        // Assert

        for expected_error in expected_error_messages {
            assert!(
                actual.contains(expected_error),
                "Expected error '{}' not found in actual error message",
                expected_error
            );
        }
    }

    #[test]
    fn validate_request_returns_result_with_error_when_validation_fails() {
        // Arrange

        let request = TestLoginRequest {
            email: String::from("not-a-good-format"),
            password: String::from(""),
        };

        // Act

        let actual = validate_request(&request);

        // Assert

        assert!(actual.is_err())
    }

    #[test]
    fn validate_request_returns_empty_result_when_validation_succeeds() {
        // Arrange

        let request = TestLoginRequest {
            email: String::from("goodest.email@gmail.com"),
            password: String::from("for-strong-man-no-problem"),
        };

        // Act

        let actual = validate_request(&request);

        // Assert

        assert!(actual.is_ok())
    }

    #[derive(Validate, Deserialize)]
    struct TestLoginRequest {
        #[validate(email(message = "email must be in a valid format"))]
        pub email: String,
        #[validate(length(min = 1, message = "please enter a password"))]
        pub password: String,
    }
}
