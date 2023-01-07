#[derive(Debug, Deserialize, Serialize)]
pub struct HomeModel {
   #[serde(rename="title")]
   pub Title: String,
   #[serde(rename="parent")]
   pub Parent: String,
}
