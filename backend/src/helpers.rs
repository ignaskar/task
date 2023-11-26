use serde::Deserialize;
use validator::Validate;
use crate::errors::Error;

pub fn validate_request<'de, T>(request: &T) -> Result<(), Error>
    where T: Deserialize<'de> + Validate {
    if let Err(e) = request.validate() {
        return Err(Error::Validation(e))
    }

    Ok(())
}