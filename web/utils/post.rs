pub async fn GetAllPosts() -> Result<Vec<BlogPost>, Error> {
   let query: PostQuery = PostQuery::all();
   RequestGet::<Vec<BlogPost>>(format!("/api/posts?{}", query.query)).await
}

pub async fn GetPostsByAuthor(author: BlogAuthor) -> Result<Vec<BlogPost>, Error> {
   let query = PostQuery::author(author);
   RequestGet::<Vec<BlogPost>>(format!("/api/posts?{}", query.query)).await
}

use {
   super::requests::*,
   crate::{error::Error, types::*},
};