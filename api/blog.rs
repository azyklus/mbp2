#[derive(Debug, Deserialize, Serialize)]
pub struct BlogEntry {
   #[serde(rename="metadata")]
   pub Meta: EntryMetadata,
   #[serde(rename="body")]
   pub Body: EntryBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlogAuthor {
   #[serde(rename="name")]
   pub Name: String,
   #[serde(rename="id", with="ulid_as_u128")]
   pub Identifier: Ulid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryMetadata {
   #[serde(rename="author")]
   pub Author: BlogAuthor,
   #[serde(rename="title")]
   pub Title: String,
   #[serde(rename="subtitle")]
   pub Subtitle: String,
   #[serde(rename="keywords")]
   pub Keywords: Vec<String>,
   #[serde(rename="id")]
   pub Identifier: Ulid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryBody {
   #[serde(rename="headers")]
   pub Headers: Vec<EntryHeader>,
   #[serde(rename="parts")]
   pub Parts: Vec<EntryPart>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EntryPart {
   #[serde(rename="section")]
   Section(EntrySection),
   #[serde(rename="quote")]
   Quote(EntryQuote),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntrySection {
   #[serde(rename="title")]
   pub Title: String,
   #[serde(rename="image_url")]
   pub ImageUrl: String,
   #[serde(rename="paragraphs")]
   pub Paragraphs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryQuote {
   #[serde(rename="author")]
   pub Author: BlogAuthor,
   #[serde(rename="content")]
   pub Content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryHeader {
   #[serde(rename="chapter")]
   pub Chapter: usize,
   #[serde(rename="tagline")]
   pub Tagline: String,
}

use ulid::{
   serde::*,
   Ulid,
};
