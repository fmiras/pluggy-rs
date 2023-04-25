use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub parameter: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidationResult {
    pub parameters: HashMap<String, String>,
    pub errors: Vec<ValidationError>,
}
