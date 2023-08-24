pub fn SetToken(token: Option<String>) {
   if let Some(t) = token.clone() {
      LocalStorage::set(TOKEN_KEY, t);
   } else {
      LocalStorage::delete(TOKEN_KEY);
   }

   let mut token_lock = TOKEN.write();
   *token_lock = token;
}

/// Get JWT token from LocalStorage.
pub fn GetToken() -> Option<String> {
   let token_lock = TOKEN.read();
   return token_lock.clone();
}

pub async fn Request<B, T>(method: ReqMethod, url: String, body: B) -> Result<T, Error>
   where
      T: DeserializeOwned + 'static + std::fmt::Debug,
      B: Serialize + std::fmt::Debug
{
   let allowBody = method == ReqMethod::POST || method == ReqMethod::PUT;
   let url: String = format!("{}{}", API_ROOT, url);
   let mut builder = Client::new()
      .request(method, url)
      .header("Content-Type", "application/json");
   
   if let Some(token) = GetToken() {
      builder = builder.bearer_auth(token);
   }

   if allowBody {
      builder = builder.json(&body);
   }

   let response = builder.send().await;

   if let Ok(data) = response {
      if data.status().is_success() {
         let data: Result<T, _> = data.json::<T>().await;
         if let Ok(data) = data {
            log::debug!("Response: {:?}", data);
            Ok(data)
         } else {
            Err(Error::DeserializeError)
         }
      } else {
         match data.status().as_u16() {
            401 => Err(Error::Unauthorized),
            403 => Err(Error::Forbidden),
            404 => Err(Error::NotFound),
            500 => Err(Error::InternalServerError),
            422 => {
               let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
               if let Ok(data) = data {
                  Err(Error::UnprocessableEntity(data))
               } else {
                  Err(Error::DeserializeError)
               }
            }
            _ => Err(Error::RequestError)
         }
      }
   } else {
      Err(Error::RequestError)
   }
}

pub async fn RequestDelete<T>(url: String) -> Result<T, Error>
   where
      T: DeserializeOwned + 'static + std::fmt::Debug
{
   Request(ReqMethod::DELETE, url, ()).await
}

pub async fn RequestGet<T>(url: String) -> Result<T, Error>
   where
      T: DeserializeOwned + 'static + std::fmt::Debug
{
   Request(ReqMethod::GET, url, ()).await
}

pub async fn RequestPost<B, T>(url: String, body: B) -> Result<T, Error>
   where
      T: DeserializeOwned + 'static + std::fmt::Debug,
      B: Serialize + std::fmt::Debug
{
   Request(ReqMethod::GET, url, body).await
}

/// Put request with a body
pub async fn RequestPut<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    Request(ReqMethod::PUT, url, body).await
}

/// Set limit for pagination
pub fn PaginationLimit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}

const API_ROOT: &str = dotenv!("API_ROOT");
const TOKEN_KEY: &str = "yew.token";

lazy_static! {
   pub static ref TOKEN: RwLock<Option<String>> = {
      if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
         RwLock::new(Some(token))
      } else {
         RwLock::new(None)
      }
   };
}

use {
   crate::{error::Error, types::*},
   dotenv_codegen::dotenv,
   gloo::storage::{LocalStorage, Storage},
   parking_lot::RwLock,
   reqwest::{Client, Method as ReqMethod},
   serde::{de::DeserializeOwned, Serialize},
};