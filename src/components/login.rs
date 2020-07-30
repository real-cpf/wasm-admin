use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use yew::callback::Callback;
use yew::format::{Json, Nothing,Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html,Bridge,Bridged,Properties,Component, ComponentLink, Html, ShouldRender,InputData,MouseEvent,FocusEvent};
use yew_router::{ prelude::*,route::Route,agent::RouteRequest::ChangeRoute, switch::Permissive, Switch};

use std::fmt::{Formatter,Display};

use crate::util::set_token;
use crate::routes::Routes;

#[derive(Default,Serialize, Deserialize,PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo{
    pub loginid:String,
    pub passwd:String,
}

impl Display for LoginInfo{
    fn fmt(&self,f: &mut Formatter<'_>)->std::fmt::Result {
        write!(f, "({}, {})", self.loginid, self.passwd)
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub callback:Callback<LoginInfo>,
}

pub struct LoginForm{
    request:LoginInfo,
    response:Callback<Result<LoginInfo,Error>>,
    task: Option<FetchTask>,
    link:ComponentLink<Self>,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    props:Props,
    message:String,
}

pub enum Msg{
    LoginRequest,
    LoginResponse(Result<LoginInfo,Error>),
    Ignore,
    UpdateLoginId(String),
    UpdatePassWd(String),
}



impl Component for LoginForm{
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{

        LoginForm{
            request:Default::default(),
            response:link.callback(Msg::LoginResponse),
            task:None,
            route_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            message:Default::default(),
            link,
        }
    }

    fn update(&mut self,msg: Self::Message)->ShouldRender{
        match msg{
            Msg::Ignore=>{},
            Msg::LoginRequest=>{
                // let task=login_local(self.response.clone(),self.request.clone());
                use crate::util::post_json;
                let task=post_json(String::from("home/formlogin1"), self.request.clone(),self.response.clone());
                self.task = Some(task);
            },
            Msg::LoginResponse(Err(err))=>{
                self.message=format!("错误：{}",err);
            },
            Msg::LoginResponse(Ok(data))=>{
                let data_token=data.clone();
                set_token(Some(format!("{}-{}",data_token.loginid,data_token.passwd)));
                self.props.callback.emit(data.clone());
                self.message=format!("成功：{}",data.loginid);
                self.route_agent.send(ChangeRoute(Routes::Home.into()));
            },
            Msg::UpdateLoginId(loginid)=>{
                self.request.loginid=loginid;
            },
            Msg::UpdatePassWd(passwd)=>{
                self.request.passwd=passwd;
            },
        }
        true
    }
    fn change(&mut self, _props: Self::Properties)->ShouldRender{
        true
    }
    fn view(&self)->Html{
        let onsubmit = self.link.callback(|ev: FocusEvent| {ev.prevent_default(); Msg::LoginRequest});
        let oninput_loginid = self.link.callback(|ev: InputData| Msg::UpdateLoginId(ev.value));
        let oninput_password = self.link.callback(|ev: InputData| Msg::UpdatePassWd(ev.value));
        html!{
            <>
            <div>{"login"}</div>
            <form onsubmit=onsubmit>
            <input type="text" name="loginid" oninput=oninput_loginid value=&self.request.loginid placeholder="loginid" id="loginid"/>
            <input type="text" name="passwd" oninput=oninput_password value=&self.request.passwd placeholder="passwd" id="passwd"/>
            <div>{&self.message}</div>
            <input type="submit" value="login"/>
            </form>
            </>
        }
    }
}

pub fn login_local(callback: Callback<Result<LoginInfo, Error>>,login:LoginInfo) -> FetchTask {
    let url ="http://localhost:9000/home/formlogin1";
    let handler = move |response: Response<Json<Result<LoginInfo, Error>>>| {
        let (meta, Json(data)) = response.into_parts();
        if meta.status.is_success() {
            callback.emit(data)
        } else {
            callback.emit(Err(anyhow!(
                "err"
            )))
        }
    };
    let u=LoginInfo{
        loginid:login.loginid,
        passwd:login.passwd,
    }; 
    let request = Request::post(url).header("Content-Type", "application/json").body(Json(&u)).unwrap();
    FetchService::fetch(request, handler.into()).unwrap()
}

