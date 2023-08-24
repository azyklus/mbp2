#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlogAuthor {
   pub name: String,
   pub id: String,
}

impl BlogAuthor {
   pub fn fromId(id: String) -> BlogAuthor {
      return BlogAuthor{
         name: "".to_string(),
         id: id,
      };
   }
}
