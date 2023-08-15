#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct BlogEntry {
   #[serde(rename="metadata")]
   pub Meta: EntryMetadata,
   #[serde(rename="body")]
   pub Body: EntryBody,
}

#[derive(InputObject)]
pub struct NewBlogEntry {
   pub meta: NewEntryMetadata,
   pub body: NewEntryBody,
}

#[ComplexObject]
impl BlogEntry {
   pub async fn metadata(&self) -> EntryMetadata {
      self.Meta.clone()
   }

   pub async fn body(&self) -> EntryBody {
      self.Body.clone()
   }
}

#[graphql(complex)]
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct BlogAuthor {
   #[serde(rename="name")]
   pub Name: String,
   #[serde(rename="id")]
   pub Identifier: String,
}

#[derive(InputObject)]
pub struct NewBlogAuthor {
   pub Name: String,
   pub Identifier: String,
}

#[ComplexObject]
impl BlogAuthor {
   pub async fn name(&self) -> String {
      self.Name.clone()
   }

   pub async fn id(&self) -> String {
      self.Identifier.clone()
   }
}

#[graphql(complex)]
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
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
   pub Identifier: String,
}

#[derive(InputObject)]
pub struct NewEntryMetadata {
   pub Author: NewBlogAuthor,
   pub Title: String,
   pub Subtitle: String,
   pub Keywords: Vec<String>,
}

#[ComplexObject]
impl EntryMetadata {
   pub async fn author(&self) -> String {
      format!("{}({})", self.Author.Name, self.Author.Identifier.to_string())
   }

   pub async fn title(&self) -> String {
      self.Title.clone()
   }

   pub async fn subtitle(&self) -> String {
      self.Subtitle.clone()
   }

   pub async fn keywords(&self) -> Vec<String> {
      self.Keywords.clone()
   }

   pub async fn id(&self) -> String {
      self.Identifier.clone()
   }
}

#[graphql(complex)]
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct EntryBody {
   #[serde(rename="headers")]
   pub Headers: Vec<EntryHeader>,
   #[serde(rename="parts")]
   pub Parts: Vec<EntrySection>,
}

#[derive(InputObject)]
pub struct NewEntryBody {
   pub Headers: Vec<NewEntryHeader>,
   pub Parts: Vec<NewEntrySection>,
}

#[ComplexObject]
impl EntryBody {
   pub async fn headers(&self) -> Vec<EntryHeader> {
      let mut headers: Vec<EntryHeader> = vec![];
      for header in self.Headers.as_slice() {
         headers.push(header.clone());
      }

      headers
   }

   pub async fn sections(&self) -> Vec<EntrySection> {
      let mut parts: Vec<EntrySection> = vec![];
      for part in self.Parts.as_slice() {
         parts.push(part.clone());
      }

      parts
   }
}

/*
#[derive(Debug, Deserialize, Serialize, async_graphql::Enum)]
pub enum EntryPart {
   #[serde(rename="section")]
   Section(EntrySection),
   #[serde(rename="quote")]
   Quote(EntryQuote),
}

#[graphql_object(description="Part of a blog entry")]
impl EntryPart {
   fn section(&self) -> EntrySection {
      return self.0;
   }

   fn quote(&self) -> EntryQuote {
      return self.0;
   }
}
*/

#[graphql(complex)]
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct EntrySection {
   #[serde(rename="title")]
   pub Title: String,
   #[serde(rename="image_url")]
   pub ImageUrl: String,
   #[serde(rename="paragraphs")]
   pub Paragraphs: Vec<String>,
}

#[derive(InputObject)]
pub struct NewEntrySection {
   pub Title: String,
   pub ImageUrl: String,
   pub Paragraphs: Vec<String>,
}

#[ComplexObject]
impl EntrySection {
   pub async fn title(&self) -> String {
      self.Title.clone()
   }

   pub async fn imageUrl(&self) -> String {
      self.ImageUrl.clone()
   }

   pub async fn paragraphs(&self) -> Vec<String> {
      self.Paragraphs.clone()
   }
}

#[graphql(complex)]
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct EntryQuote {
   #[serde(rename="author")]
   pub Author: BlogAuthor,
   #[serde(rename="content")]
   pub Content: String,
}

#[ComplexObject]
impl EntryQuote {
   pub async fn author(&self) -> String {
      format!("{}({})", self.Author.Name, self.Author.Identifier.to_string())
   }

   pub async fn content(&self) -> String {
      self.Content.clone()
   }
}

#[graphql(complex)]
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct EntryHeader {
   #[serde(rename="chapter")]
   pub Chapter: i32,
   #[serde(rename="tagline")]
   pub Tagline: String,
}

#[derive(InputObject)]
pub struct NewEntryHeader {
   pub Chapter: i32,
   pub Tagline: String,
}

#[ComplexObject]
impl EntryHeader {
   pub async fn chapter(&self) -> i32 {
      self.Chapter
   }

   pub async fn tagline(&self) -> String {
      self.Tagline.clone()
   }
}

use ulid::{
   serde::*,
   Ulid,
};
