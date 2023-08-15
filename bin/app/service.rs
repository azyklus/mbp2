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
      .await.unwrap();

   return Ok(client);
}

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
   pub async fn authors<'ctx>(
      &self,
      ctx: &Context<'ctx>,
   ) -> Vec<BlogAuthor> {
      let database = ctx.data::<Database>().unwrap();
      let collection = database.collection("authors");
      let cursor = collection.find(None, None).await.unwrap();

      let mut authors: Vec<BlogAuthor> = vec![];
      for result in cursor {
         authors.push(result.unwrap());
      }

      return authors;
   }

   pub async fn posts<'ctx>(
      &self,
      ctx: &Context<'ctx>,
   ) -> Vec<BlogEntry> {
      let database = ctx.data::<Database>().expect("database data not present");
      let collection = database.collection("entries");
      let cursor = collection.find(None, None).await.unwrap();

      let mut posts: Vec<BlogEntry> = vec![];
      for result in cursor {
         posts.push(result.unwrap());
      }

      return posts;
   }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
   pub async fn createEntry<'ctx>(
      &self,
      ctx: &Context<'ctx>,
      newEntry: NewBlogEntry
   ) -> BlogEntry {
      let database: &Database = ctx.data::<Database>().expect("database data not present");
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
               Identifier: Ulid::from_string(newEntry.meta.Author.Identifier.as_str()).expect("invalid ULID construction").to_string(),
            },
            Title: newEntry.meta.Title,
            Subtitle: newEntry.meta.Subtitle,
            Keywords: newEntry.meta.Keywords,
            Identifier: Ulid::new().to_string(),
         },
         Body: EntryBody {
            Headers: newHeaders,
            Parts: newParts,
         },
      };

      collection.insert_one(&be, None).await.expect("could not insert new entry");
      return be;
   }
}

/*
#[derive(Clone)]
pub struct DbContext {
   pub Db: Database
}

impl Context for DbContext {}

impl FromContext<DbContext> for Database {
   fn from(ctx: &DbContext) -> &Self {
      let ctx = ctx.clone();
      return &ctx.Db;
   }
}
*/

use {
   bson::doc,
   async_graphql::{Context, FieldResult, EmptySubscription, Schema},
   mbp2::api::*,
   mongodb::{
      Client, Database, Collection,
      options::{ClientOptions, ServerApi, ServerApiVersion},
   },
   std::env,
   ulid::Ulid,
};
