/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
   pub Err: String,
}

impl Display for FetchError {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      Debug::fmt(&self.Err, f)
   }
}

impl Error for FetchError{}

/// The possible states a fetch request can be in.
#[derive(Debug, Clone, PartialEq)]
pub enum FetchState<T> {
   NotFetching,
   Fetching,
   Success(T),
   Failed(FetchError),
}

pub enum FetchStateMsg<T> {
   SetDataFetchState(FetchState<T>),
   GetData,
}

use std::{
   error::Error,
   fmt::{self, Debug, Display, Formatter},
};
