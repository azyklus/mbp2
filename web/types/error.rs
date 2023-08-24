#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayableError {
   pub Title: String,
   pub Message: String,
}

/// API error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

use std::collections::HashMap;
