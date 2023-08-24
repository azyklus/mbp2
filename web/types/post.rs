#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostQuery {
   pub query: String,
}

impl PostQuery {
   pub fn new(id: String) -> PostQuery {
      let query: String = format!("query {{ post {{ meta({}) {{ author {{ name id }} title subtitle keywords }} body {{ headers sections }} }} }}", id);
      return PostQuery{ query };
   }

   pub fn all() -> PostQuery {
      let query: String = "query { posts { meta { author { name id } title subtitle keywords id } body { headers sections } } }".to_string();
      return PostQuery{ query };
   }

   pub fn author(author: BlogAuthor) -> PostQuery {
      let query: String = format!("query {{ post {{ meta {{ author({}, {}) title subtitle keywords }} body {{ headers sections }} }} }}", author.name, author.id);
      return PostQuery{ query };
   }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostListInfo {
   pub posts: Vec<PostMetadata>,
   pub count: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlogPost {
   pub meta: PostMetadata,
   pub body: PostBody,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostMetadata {
   pub author: BlogAuthor,
   pub title: String,
   pub subtitle: String,
   pub keywords: Vec<String>,
   pub id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostBody {
   pub headers: Vec<PostHeader>,
   pub sections: Vec<PostSection>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostHeader {
   pub chapter: i32,
   pub tagline: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostSection {
   pub title: String,
   pub imageUrl: String,
   pub paragraphs: Vec<String>,
}

use crate::types::BlogAuthor;