/// DbConnCreate opens a connection to a MongoDB server with the client options specified
/// in the `config.json`.
pub async fn DbConnCreate(config: DbSettings) -> FieldResult<Client> {
   let mut clientOptions = ClientOptions::parse(format!("mongodb+srv://{}:{}@cluster0.jlm4ztq.mongodb.net/?retryWrites={}&retryReads={}&localThresholdMS={}&w={}",
      config.Username, config.Password, config.RetryWrites, config.RetryReads, config.LocalThreshold, config.WriteConcern))
      .expect("failure to parse client options");

   let serverApi = ServerApi::builder().version(ServerApiVersion::V1).build();
   clientOptions.server_api = Some(serverApi);

   let dbUrl: String = env::var("MONGODB_URL").expect("url must be set");
   let client: Client = Client::with_options(clientOptions).expect("failed to build client");

   // Ping the database to ensure a connection.
   client.database("admin")
      .run_command(doc!{ "ping": 1 }, None)
      .unwrap();

   return Ok(client);
}

pub struct QueryRoot;

#[graphql_object(context=DbContext)]
impl QueryRoot {
   pub fn authors(
      #[graphql(context)] ctx: &DbContext,
   ) -> Vec<BlogAuthor> {
      let database = &ctx.Db.clone();
      let collection = database.collection("authors");
      let cursor = collection.find(None, None).unwrap();

      let mut authors: Vec<BlogAuthor> = vec![];
      for result in cursor {
         authors.push(result.unwrap());
      }

      return authors;
   }

   pub fn posts(
      #[graphql(context)] ctx: &DbContext,
   ) -> Vec<BlogEntry> {
      let database = &ctx.Db.clone();
      let collection = database.collection("entries");
      let cursor = collection.find(None, None).unwrap();

      let mut posts: Vec<BlogEntry> = vec![];
      for result in cursor {
         posts.push(result.unwrap());
      }

      return posts;
   }
}

pub struct MutationRoot;

#[graphql_object(context=DbContext)]
impl MutationRoot {
   pub fn createEntry(
      #[graphql(context)] ctx: &DbContext,
      newEntry: NewBlogEntry
   ) -> BlogEntry {
      let database: &Database = &ctx.Db.clone();
      let collection: Collection<BlogEntry> = database.collection("entries");

      let mut newHeaders: Vec<EntryHeader> = vec![];
      for header in newEntry.body.Headers.as_slice() {
         newHeaders.push(EntryHeader {
            Chapter: header.Chapter,
            Tagline: header.Tagline.clone()
         });
      }

      let mut newParts: Vec<EntrySection> = vec![];
      for part in newEntry.body.Parts.as_slice() {
         newParts.push(EntrySection {
            Title: part.Title.clone(),
            ImageUrl: part.ImageUrl.clone(),
            Paragraphs: part.Paragraphs.clone()
         });
      }

      let be = BlogEntry{
         Meta: EntryMetadata {
            Author: BlogAuthor {
               Name: newEntry.meta.Author.Name,
               Identifier: Ulid::from_string(newEntry.meta.Author.Identifier.as_str()).expect("invalid ULID construction"),
            },
            Title: newEntry.meta.Title,
            Subtitle: newEntry.meta.Subtitle,
            Keywords: newEntry.meta.Keywords,
            Identifier: Ulid::new(),
         },
         Body: EntryBody {
            Headers: newHeaders,
            Parts: newParts,
         },
      };

      collection.insert_one(&be, None).expect("could not insert new entry");
      return be;
   }
}

#[derive(Clone)]
pub struct DbContext {
   pub Db: Database
}

impl Context for DbContext {}
/*
impl FromContext<DbContext> for Database {
   fn from(ctx: &DbContext) -> &Self {
      let ctx = ctx.clone();
      return &ctx.Db;
   }
}
*/
use {
   bson::doc,
   juniper::{Context, FromContext, FieldResult},
   mbp2::api::*,
   mongodb::{
      sync::{Client, Database, Collection},
      options::{ClientOptions, ServerApi, ServerApiVersion},
   },
   std::env,
   ulid::Ulid,
};
