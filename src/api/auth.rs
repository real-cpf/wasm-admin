use yew::callback::Callback;
use yew::services::storage::{Area, StorageService};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::{Error,anyhow};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::util::get_token;

use crate::util::post_json;
use crate::components::LoginInfo;

#[derive(Default,Serialize, Deserialize,PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo{
    pub loginid:String,
    pub passwd:String,
}
///login api
pub fn login(callback: Callback<Result<LoginInfo, Error>>,login:LoginInfo) -> FetchTask {
    post_json(String::from("home/formlogin1"), login, callback)
}

pub fn has_token()->bool{
    get_token().is_some()
}

