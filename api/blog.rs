#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlogEntry {
   #[serde(rename="metadata")]
   pub Meta: EntryMetadata,
   #[serde(rename="body")]
   pub Body: EntryBody,
}

#[derive(GraphQLInputObject)]
pub struct NewBlogEntry {
   pub meta: NewEntryMetadata,
   pub body: NewEntryBody,
}

#[graphql_object(description="A blog entry")]
impl BlogEntry {
   fn metadata(&self) -> EntryMetadata {
      return self.Meta.clone();
   }

   fn body(&self) -> EntryBody {
      return self.Body.clone();
   }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlogAuthor {
   #[serde(rename="name")]
   pub Name: String,
   #[serde(rename="id", with="ulid_as_u128")]
   pub Identifier: Ulid,
}

#[derive(GraphQLInputObject)]
pub struct NewBlogAuthor {
   pub Name: String,
   pub Identifier: String,
}

#[graphql_object(description="Author of a blog entry")]
impl BlogAuthor {
   fn name(&self) -> String {
      return self.Name.clone();
   }

   fn id(&self) -> String {
      return self.Identifier.to_string();
   }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(GraphQLInputObject)]
pub struct NewEntryMetadata {
   pub Author: NewBlogAuthor,
   pub Title: String,
   pub Subtitle: String,
   pub Keywords: Vec<String>,
}

#[graphql_object(description="Metadata of a blog entry")]
impl EntryMetadata {
   pub fn author(&self) -> String {
      format!("{}({})", self.Author.Name, self.Author.Identifier.to_string())
   }

   pub fn title(&self) -> String {
      return self.Title.clone();
   }

   pub fn subtitle(&self) -> String {
      return self.Subtitle.clone();
   }

   pub fn keywords(&self) -> Vec<String> {
      return self.Keywords.clone();
   }

   pub fn id(&self) -> String {
      return self.Identifier.to_string();
   }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryBody {
   #[serde(rename="headers")]
   pub Headers: Vec<EntryHeader>,
   #[serde(rename="parts")]
   pub Parts: Vec<EntrySection>,
}

#[derive(GraphQLInputObject)]
pub struct NewEntryBody {
   pub Headers: Vec<NewEntryHeader>,
   pub Parts: Vec<NewEntrySection>,
}

#[graphql_object(description="Body of a blog entry")]
impl EntryBody {
   fn headers(&self) -> Vec<EntryHeader> {
      let mut headers: Vec<EntryHeader> = vec![];
      for header in self.Headers.as_slice() {
         headers.push(header.clone());
      }

      return headers;
   }

   fn sections(&self) -> Vec<EntrySection> {
      let mut parts: Vec<EntrySection> = vec![];
      for part in self.Parts.as_slice() {
         parts.push(part.clone());
      }

      return parts;
   }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EntryPart {
   #[serde(rename="section")]
   Section(EntrySection),
   #[serde(rename="quote")]
   Quote(EntryQuote),
}
/*
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntrySection {
   #[serde(rename="title")]
   pub Title: String,
   #[serde(rename="image_url")]
   pub ImageUrl: String,
   #[serde(rename="paragraphs")]
   pub Paragraphs: Vec<String>,
}

#[derive(GraphQLInputObject)]
pub struct NewEntrySection {
   pub Title: String,
   pub ImageUrl: String,
   pub Paragraphs: Vec<String>,
}

#[graphql_object(description="Section of a blog entry")]
impl EntrySection {
   fn title(&self) -> String {
      return self.Title.clone();
   }

   fn imageUrl(&self) -> String {
      return self.ImageUrl.clone();
   }

   fn paragraphs(&self) -> Vec<String> {
      return self.Paragraphs.clone();
   }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryQuote {
   #[serde(rename="author")]
   pub Author: BlogAuthor,
   #[serde(rename="content")]
   pub Content: String,
}

#[graphql_object(description="A quote from a blog post")]
impl EntryQuote {
   fn author(&self) -> String {
      format!("{}({})", self.Author.Name, self.Author.Identifier.to_string())
   }

   fn content(&self) -> String {
      return self.Content.clone();
   }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryHeader {
   #[serde(rename="chapter")]
   pub Chapter: i32,
   #[serde(rename="tagline")]
   pub Tagline: String,
}

#[derive(GraphQLInputObject)]
pub struct NewEntryHeader {
   pub Chapter: i32,
   pub Tagline: String,
}

#[graphql_object(description="The header of a chapter")]
impl EntryHeader {
   fn chapter(&self) -> i32 {
      return self.Chapter;
   }

   fn tagline(&self) -> String {
      return self.Tagline.clone();
   }
}

use ulid::{
   serde::*,
   Ulid,
};
