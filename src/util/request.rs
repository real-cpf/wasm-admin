use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};
use yew::callback::Callback;
use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::format::{Json, Nothing,Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::{Error,anyhow};

const BASE_PATH: &str = dotenv!("BASE_PATH");

#[derive(Default,Serialize, Deserialize,PartialEq, Clone, Debug)]
pub struct FileForm{
   pub file:Vec<u8>,
}



// pub fn post_json<N,M>(path:String,body:N,callback:Callback<Result<M,Error>>)->FetchTask
//     where for <'a> M:Deserialize<'a>+ 'static,N:Into<Text>+std::fmt::Debug,
//     {
//         let handler = move |response: Response<Text>| {
//             let (meta, Ok(data)) = response.into_parts();
//             if meta.status.is_success() {
//                 let data: Result<M, _> = serde_json::from_str(&data);
//                 if let Ok(data) = data {
//                     callback.emit(Ok(data))
//                 } else {
//                     callback.emit(Err(anyhow!("type error")))
//                 }
//             } else {
//                 callback.emit(Err(anyhow!{
//                     format!("code : {}",meta.status.as_u16())
//                 }))
//             }
//         };
//         let mut builder = Request::builder()
//         .method("POST")
//         .uri(format!("{}/{}",BASE_PATH,path))
//         .header("Content-Type", "application/json");
//         let request = builder.body(body).unwrap();
//         // let request = Request::post(format!("{}/{}",BASE_PATH,path)).header("Content-Type", "application/json").body(Json(&body)).unwrap();
//         FetchService::fetch(request, handler.into()).unwrap()
//     }

pub fn post_file<FileForm,M>(path:String,body:FileForm,callback:Callback<Result<M,Error>>)->FetchTask
    where for <'a> M:Deserialize<'a>+ 'static,FileForm:Serialize+Into<Text>+ std::fmt::Debug,
    {
        let handler = move |response: Response<Json<Result<M,Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!{
                    format!("code : {}",meta.status.as_u16())
                }))
            }
        };
        let request = Request::post(format!("{}/{}",BASE_PATH,path))
        .header("Content-Type", "multipart/form-data").body(body).unwrap();
        FetchService::fetch(request, handler.into()).unwrap()
}


pub fn post_json<N,M>(path:String,body:N,callback:Callback<Result<M,Error>>)->FetchTask
    where for <'a> M:Deserialize<'a>+ 'static,N:Serialize+ std::fmt::Debug,
    {
        let handler = move |response: Response<Json<Result<M,Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!{
                    format!("code : {}",meta.status.as_u16())
                }))
            }
        };
        let request = Request::post(format!("{}/{}",BASE_PATH,path)).header("Content-Type", "multipart/form-data").body(Json(&body)).unwrap();
        FetchService::fetch(request, handler.into()).unwrap()
}
