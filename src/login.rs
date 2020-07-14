use anyhow::{anyhow, Error};
use serde_derive::Deserialize;
use yew::callback::Callback;
use yew::format::{Json, Nothing,Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html,Bridge,Bridged,Properties,Component, ComponentLink, Html, ShouldRender,InputData,MouseEvent,FocusEvent};

use crate::route::AppRoute;
use crate::app::Model;
// use yew_router::prelude::*;
use yew_router::{ prelude::*,route::Route,agent::RouteRequest::ChangeRoute, switch::Permissive, Switch};


// #[derive(PartialEq, Properties, Clone)]
// pub struct Props {
//     /// Callback when user is logged in successfully
//     pub callback: Callback<UserInfo>,
// }

/// Login page
pub struct Login{
    link: ComponentLink<Self>,
    request:LoginApi,
    response: Callback<Result<UserInfo, Error>>,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    // props: Props,
}






pub struct LoginApi{
    url:String,
    userinfo:UserInfo,
}
#[derive(Deserialize, Debug)]
pub struct UserInfo{
    pub loginid:String,
    pub passwd:String,
}

pub enum Msg {
    Request,
    Response(Result<UserInfo, Error>),
    Ignore,
    UpdateEmail(String),
    UpdatePassword(String),
    GoTo,
}



    pub fn login(callback: Callback<Result<UserInfo, Error>>) -> FetchTask {
        let url ="http://localhost:9000/home/formlogin";
        let handler = move |response: Response<Json<Result<UserInfo, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!(
                    "{}: error",
                    meta.status
                )))
            }
        };
        
        let request = Request::post(url).body(Nothing).unwrap();
        FetchService::fetch(request, handler.into()).unwrap()
    }


impl Component for Login{
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{
        let req=LoginApi{
            url:String::from("localhost:9000/home/formlogin"),
            userinfo:UserInfo{
                loginid:String::default(),
                passwd:String::default(),
            },
        };
        // link: ComponentLink<Self>,
        // request:LoginApi,
        // response: Callback<Result<UserInfo, Error>>,
        // task: Option<FetchTask>,
        // props: Props,
        Login{
            link:link.clone(),
            request:req,
            response:link.callback(Msg::Response),
            task:None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),

        }
    }

    fn update(&mut self, msg: Self::Message)->ShouldRender{
        match msg{
            Msg::Request=>{
                self.request.userinfo.loginid=String::from("ok");
                let task=login(self.response.clone());
                self.task = Some(task);
                
            },
            Msg::UpdateEmail(loginid)=>{
                self.request.userinfo.loginid=loginid;
            },
            Msg::UpdatePassword(passwd)=>{
                self.request.userinfo.passwd=passwd;
            },Msg::Response(Err(err)) => {
                self.task = None;
            },
            Msg::Response(Ok(res)) => {
                self.request.userinfo.loginid=res.loginid;
                self.request.userinfo.passwd=res.passwd;
                // self.router_agent.send(ChangeRoute(AppRoute::Model.into()));
                // AppRoute::switch(AppRoute::Model.into());
                let route = Route::new_no_state("/main");
                AppRoute::switch(route);
                // self.task = res;

            },
            Msg::Ignore => {},
            Msg::GoTo=>{
                // let route = Route::new_no_state("/main.html");
                // AppRoute::switch(route);
                self.router_agent.send(ChangeRoute(AppRoute::Model.into()));
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties)->ShouldRender{
        true
    }

    fn view(&self)->Html{
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default(); /* Prevent event propagation */
            Msg::Request
        });
        let oninput_email = self
        .link
        .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self
        .link
        .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let btngoto= self
        .link
        .callback(|ev: MouseEvent| Msg::GoTo);
        html!{
            <>
            <div>{"login"}</div>
            <input type="button" onclick=btngoto value="goto"/>
            <form onsubmit=onsubmit>
            <input type="text" name="loginid" oninput=oninput_email value=&self.request.userinfo.loginid placeholder="loginid" id="loginid"/>
            <input type="password" name="passwd" oninput=oninput_password value=&self.request.userinfo.passwd placeholder="passwd" id="passwd"/>
            <input type="submit" value="login"/>
            </form>
            </>
        }
    }

}