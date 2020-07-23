///like this code from repository
///https://github.com/jetli/rust-yew-realworld-example-app/blob/master/crates/conduit-wasm/src/services/requests.rs

use serde::{Deserialize, Serialize};
use yew::callback::Callback;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};
use log::debug;

use thiserror::Error as HttpError;
#[derive(HttpError, Clone, Debug)]
pub enum Error{
    #[error("error")]
    HttpError,
}

#[derive(Default, Debug)]
pub struct Http {}

impl Http {
    pub fn new() -> Self {
        Self {}
    }

    /// build all kinds of http request: post/get/delete etc.
    pub fn builder<B, T>(
        &mut self,
        method: &str,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Into<Text> + std::fmt::Debug,
    {
        let handler = move |response: Response<Text>| {
            if let (meta, Ok(data)) = response.into_parts() {
                debug!("Response: {:?}", data);
                if meta.status.is_success() {
                    let data: Result<T, _> = serde_json::from_str(&data);
                    if let Ok(data) = data {
                        callback.emit(Ok(data))
                    } else {
                        callback.emit(Err(Error::HttpError))
                    }
                } else {
                    match meta.status.as_u16() {
                        _ => callback.emit(Err(Error::HttpError)),
                    }
                }
            } else {
                callback.emit(Err(Error::HttpError))
            }
        };

        let url = format!("http://localhost:9000/{}", url);
        let mut builder = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("Content-Type", "application/json");
        let request = builder.body(body).unwrap();
        debug!("Request: {:?}", request);

        FetchService::fetch(request, handler.into()).unwrap()
    }

    /// Delete request
    pub fn delete<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'a> T: Deserialize<'a> + 'static + std::fmt::Debug,
    {
        self.builder("DELETE", url, Nothing, callback)
    }

    /// Get request
    pub fn get<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'a> T: Deserialize<'a> + 'static + std::fmt::Debug,
    {
        self.builder("GET", url, Nothing, callback)
    }

    /// Post request with a body
    pub fn post<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'a> T: Deserialize<'a> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("POST", url, body, callback)
    }

    /// Put request with a body
    pub fn put<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'a> T: Deserialize<'a> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("PUT", url, body, callback)
    }
}

