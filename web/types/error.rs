#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayableError {
   pub Title: String,
   pub Message: String,
}