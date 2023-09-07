/// DbConnCreate opens a connection to a MongoDB server with the client options specified
/// in the `config.json`.
pub async fn DbConnCreate(config: DbSettings) -> FieldResult<Client> {
   let mut clientOptions = ClientOptions::parse_async(format!(
      "mongodb+srv://{}:{}@cluster0.jlm4ztq.mongodb.net/?retryWrites={}&retryReads={}&localThresholdMS={}&w={}",
      config.Username, config.Password, config.RetryWrites, config.RetryReads, config.LocalThreshold, config.WriteConcern
   ))
   .await
   .expect("failure to parse client options");

   let serverApi = ServerApi::builder().version(ServerApiVersion::V1).build();
   clientOptions.server_api = Some(serverApi);

   let client: Client = Client::with_options(clientOptions).expect("failed to build client");
   return Ok(client);
}

/// GraphQLSchema is the schema for our GQL/Mongo setup.
///
/// - [`QueryRoot`] holds our queries;
/// - [`MutationRoot`] handles our mutations;
/// - and [`EmptySubscription`], since we have no subscriptions.
///
/// [`QueryRoot`]: crate::service::QueryRoot
/// [`MutationRoot`]: crate::service::MutationRoot
/// [`EmptySubscription`]: async_graphql::EmptySubscription
pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// QueryRoot contains our GQL queries.
///
/// - [`authors`] is our query returning a [`Vec<BlogAuthor>`] containing our authors.
/// - [`posts`] is our query returning a [`Vec<BlogEntry>`] containing our entries.
///
/// [`authors`]: crate::service::QueryRoot::authors
/// [`posts`]: crate::service::QueryRoot::posts
pub struct QueryRoot;

#[Object]
impl QueryRoot {
   pub async fn authors(&self, ctx: &Context<'_>) -> Vec<BlogAuthor> {
      let database = ctx.data::<Database>().unwrap();
      let collection = database.collection("authors");
      let mut cursor: Cursor<_> = collection.find(None, None).await.unwrap();

      let mut authors: Vec<BlogAuthor> = vec![];
      while let Some(author) = cursor.try_next().await.unwrap() {
         authors.push(author);
      }

      authors
   }

   pub async fn posts(&self, ctx: &Context<'_>) -> Vec<BlogEntry> {
      let database = ctx.data::<Database>().expect("database data not present");
      let collection = database.collection("entries");
      let mut cursor: Cursor<_> = collection.find(None, None).await.unwrap();

      let mut posts: Vec<BlogEntry> = vec![];
      while let Some(post) = cursor.try_next().await.unwrap() {
         posts.push(post);
      }

      posts
   }
}

/// MutationRoot contains our GQL mutations.
///
/// - [`createEntry`] is our mutation for creating a new blog entry in our database collections.
/// - [`removeEntry`] is for deleting entries from the database.
/// - [`addAuthor`] pushes a new [`BlogAuthor`] into our authors collection.
///
/// [`createEntry`]: crate::service::MutationRoot::createEntry
/// [`removeEntry`]: crate::service::MutationRoot::removeEntry
/// [`addAuthor`]: crate::service::MutationRoot::addAuthor
/// [`BlogAuthor`]: mbp2::api::BlogAuthor
pub struct MutationRoot;

#[Object]
impl MutationRoot {
   /// Create a blog entry in our MongoDB collection ("otherskies.entries").
   /// Takes a `&Context` receiver and a parameter for the new blog entry.
   pub async fn createEntry(
      &self,
      ctx: &Context<'_>,
      #[graphql(desc = "The new entry to be created.")]
      newEntry: NewBlogEntry,
   ) -> BlogEntry {
      let database: &Database = ctx.data::<Database>().expect("failed to fetch database");
      let collection: Collection<BlogEntry> = database.collection("entries");

      let mut newHeaders: Vec<EntryHeader> = vec![];
      for header in newEntry.body.Headers.as_slice() {
         newHeaders.push(EntryHeader {
            Chapter: header.Chapter,
            Tagline: header.Tagline.clone(),
         });
      }

      let mut newParts: Vec<EntrySection> = vec![];
      for part in newEntry.body.Parts.as_slice() {
         newParts.push(EntrySection {
            Title: part.Title.clone(),
            ImageUrl: part.ImageUrl.clone(),
            Paragraphs: part.Paragraphs.clone(),
         });
      }

      let blogEntry = BlogEntry{
         Meta: EntryMetadata{
            Author: BlogAuthor{
               Name: newEntry.meta.Author.Name,
               Identifier: Ulid::from_string(newEntry.meta.Author.Identifier.as_str())
                  .expect("invalid ULID construction")
                  .to_string(),
            },
            Title: newEntry.meta.Title,
            Subtitle: newEntry.meta.Subtitle,
            Keywords: newEntry.meta.Keywords,
            Identifier: Ulid::new().to_string(),
         },
         Body: EntryBody{
            Headers: newHeaders,
            Parts: newParts,
         },
      };

      collection.insert_one(&blogEntry, None).await.expect("could not insert new entry");
      blogEntry
   }

   /// Add an author to our MongoDB collection ("otherskies.authors").
   /// Takes a `&Context` receiver and a parameter for the new author.
   pub async fn addAuthor(
      &self,
      ctx: &Context<'_>,
      #[graphql(desc="The author to add to the collection.")]
      newAuthor: NewBlogAuthor,
   ) -> BlogAuthor {
      let database: &Database = ctx.data::<Database>().expect("failed to fetch database");
      let collection: Collection<BlogAuthor> = database.collection("authors");

      let blogAuthor: BlogAuthor = BlogAuthor{
         Name: newAuthor.Name.clone(),
         Identifier: Ulid::from_string(newAuthor.Identifier.as_str())
            .expect("invalid ULID construction")
            .to_string(),
      };

      collection.insert_one(&blogAuthor, None).await.expect("could not insert new author");
      blogAuthor
   }

   /// removeEntry removes a blog entry from the database collection ("otherskies.entries").
   /// Takes a `&Context` receiver, a `&str` for an identifier, and a [`BlogEntry`] for the
   /// entry to be deleted.
   pub async fn removeEntry(
      &self,
      ctx: &Context<'_>,
      #[graphql(desc="The identifier of the entry.")]
      identifier: String,
      #[graphql(desc="The blog entry to be deleted.")]
      entry: NewBlogEntry,
   ) -> Vec<BlogEntry> {
      // Fetch our database and select the collection we need to modify.
      let database: &Database = ctx.data::<Database>().expect("failed to fetch database");
      let collection: Collection<BlogEntry> = database.collection("entries");

      // Set the filters for the entry we need to delete.
      let oid: ObjectId = ObjectId::parse_str(identifier.as_str()).expect("invalid identifier");
      let filter = doc!{
         "_id": oid,
         "meta": {
            "author": {
               "name": entry.meta.Author.Name.to_owned(),
               "identifier": entry.meta.Author.Identifier.to_owned(),
            },
            "title": entry.meta.Title.to_owned(),
            "subtitle": entry.meta.Subtitle.to_owned(),
            "keywords": entry.meta.Keywords.as_slice(),
         },
      };

      // Delete the entry.
      collection.delete_one(filter, None).await.expect("failed to delete entry");

      // Retrieve and return the entries we have left.
      let mut cursor = collection.find(None, None).await.unwrap();
      let mut entries: Vec<BlogEntry> = vec![];
      while let Some(entry) = cursor.try_next().await.unwrap() {
         entries.push(entry);
      }

      entries
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
   crate::models::*,
   async_graphql::{Context, EmptySubscription, FieldResult, Schema},
   bson::{doc, oid::ObjectId},
   futures::TryStreamExt,
   mbp2::api::*,
   mongodb::{
      options::{ClientOptions, ServerApi, ServerApiVersion},
      Client, Collection, Cursor, Database,
   },
   ulid::Ulid,
};
