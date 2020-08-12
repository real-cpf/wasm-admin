use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::{html, ChangeData, Component, ComponentLink, Html, ShouldRender};
use yew::callback::Callback;
use anyhow::{anyhow, Error};
use web_sys::Blob;
use yew::services::console::ConsoleService;

use serde::{Deserialize, Serialize};
use crate::util::{FileForm,post_file,post_json};
use crate::components::LoginInfo;
pub struct Upload {
    link: ComponentLink<Upload>,
    response:Callback<Result<LoginInfo,Error>>,
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    files: Vec<String>,
    by_chunks: bool,
}


// impl Into<FileForm> for FileData{
//     fn into(self)->FileForm{
//         FileForm{
//             file:self.content,
//         }
//     }
// }


type Chunks = bool;

pub enum Msg {
    Ignore,
    Loaded(FileData),
    Chunk(Option<FileChunk>),
    Files(Vec<File>, Chunks),
    ToggleByChunks,
    UploadResponse(Result<LoginInfo,Error>),
}

impl Component for Upload {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Upload {
            reader: ReaderService::new(),
            response:link.callback(Msg::UploadResponse),
            link,
            tasks: vec![],
            files: vec![],
            by_chunks: false,
            
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded(file) => {
                let info = format!("file: {:?}", file);
                self.files.push(info);
                let f=FileForm{
                    file:file.content,
                };
                
                post_json(String::from("files/upload"),f , self.response.clone());
                // post_json(String::from("files/upload"),File::from(f.clone().file.unwrap()), self.response.clone());
            }
            Msg::Chunk(Some(chunk)) => {
                let info = format!("chunk: {:?}", chunk);
                self.files.push(info);
            }
            Msg::Files(files, chunks) => {
                
                for file in files.into_iter() {
                    ConsoleService::log("hello");
                    ConsoleService::debug(format!("{:?}",file).as_str());
                    // JsValue::
                    // post_json(String::from("files/upload"),f, self.response.clone());  
                    let task = {
                        if chunks {
                            let callback = self.link.callback(Msg::Chunk);
                            self.reader.read_file_by_chunks(file, callback, 10).unwrap()
                        } else {
                            
                            let callback = self.link.callback(Msg::Loaded);
                            self.reader.read_file(file, callback).unwrap()
                        }
                    };
                    self.tasks.push(task);
                }
            }
            Msg::ToggleByChunks => {
                self.by_chunks = !self.by_chunks;
            }
            _ => return false,
        }
        true
    }

    fn view(&self) -> Html {
        let flag = self.by_chunks;
        html! {
            <div>
                <div>
                    <p>{"Choose a file to upload to see the uploaded bytes"}</p>
                    <input type="file" multiple=false onchange=self.link.callback(move |value| {
                        //Files(FileList { obj: Object { obj: JsValue(FileList) } })
                        ConsoleService::debug(format!("{:?}",value).as_str());
                        
                            let mut result = Vec::new();
                            if let ChangeData::Files(files) = value {
                                let files = js_sys::try_iter(&files)
                                    .unwrap()
                                    .unwrap()
                                    .into_iter()
                                    .map(|v| File::from(v.unwrap()));
                                result.extend(files);
                            }
                            Msg::Files(result, flag)
                            // Msg::Ignore
                        })/>
                </div>
                <div>
                    <label>{ "By chunks" }</label>
                    <input type="checkbox" checked=flag onclick=self.link.callback(|_| Msg::ToggleByChunks) />
                </div>
                <ul>
                    { for self.files.iter().map(|f| self.view_file(f)) }
                </ul>
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Upload {
    fn view_file(&self, data: &str) -> Html {
        html! {
            <li>{ data }</li>
        }
    }
}
