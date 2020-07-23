use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use yew::callback::Callback;
use yew::format::{Json, Nothing,Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html,Bridge,Bridged,Properties,Component, ComponentLink, Html, ShouldRender,InputData,MouseEvent,FocusEvent};

use yewtil::fetch::{Fetch, FetchAction, FetchRequest, FetchState, Json as FetchJson, MethodBody};
use yewtil::future::LinkFuture;

use crate::http::Http;

use crate::routes::Routes;
// use crate::route::AppRoute;
use crate::app::Model;
use crate::home::HomeRoute;
// use yew_router::prelude::*;
use yew_router::{ prelude::*,route::Route,agent::RouteRequest::ChangeRoute, switch::Permissive, Switch};

use crate::http::Error as HttpError;

// #[derive(PartialEq, Properties, Clone)]
// pub struct Props {
//     /// Callback when user is logged in successfully
//     pub callback: Callback<UserInfo>,
// }

#[derive(Default, Debug, Clone)]
pub struct LoginRequest{
    pub userinfo:UserInfo ,
}


/// Login page
pub struct Login{
    login_req:Fetch<LoginRequest,UserInfo>,
    link: ComponentLink<Self>,
    request:LoginApi,
    response: Callback<Result<UserInfo, HttpError>>,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    message:String,
    // props: Props,
}




impl FetchRequest for LoginRequest{

    type RequestBody = UserInfo;
    type ResponseBody=UserInfo;
    type Format = FetchJson;
    fn url(&self) -> String {
        // Given that this is an external resource, this may fail sometime in the future.
        // Please report any regressions related to this.
        "http://localhost:9000/home/formlogin".to_string()
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        
        MethodBody::Post(&self.userinfo)
        
        // MethodBody::Post(&'a UserInfo)
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![]
    }

    fn use_cors(&self) -> bool {
        true
    }

}


 pub trait MyFetchRequest {
     type RequestBody: Serialize;
     type ResponseBody: DeserializeOwned;
     fn path(&self) -> String;
     fn method(&self) -> MethodBody<Self::RequestBody>;
 }
/// /// A wrapper to allow implementing a foreign trait generically for anything wrapped by it that meets
/// /// the specified type bounds.
/// /// This isn't ideal, and will not be required in the future after coherence rules are changed.
/// /// https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md
 pub struct LocalWrapper<T>(T);

 impl <T: MyFetchRequest> FetchRequest for LocalWrapper<T> {
     type RequestBody = T::RequestBody;
     type ResponseBody = T::ResponseBody;
     type Format = FetchJson; // Always serialize using json

     fn url(&self) -> String {
         // The origin will always be the same
         format!("{}", self.0.path())
     }

     fn method(&self) -> MethodBody<Self::RequestBody> {
         self.0.method()
     }

     fn headers(&self) -> Vec<(String, String)> {
         // Always attach the same headers.
         vec![
             ("Content-Type".to_string(), "application/json".to_string())
         ]
     }
 }

 pub struct ApplesRequest{
    pub userinfo:UserInfo,
 }
 impl MyFetchRequest for ApplesRequest {
     type RequestBody = (); type ResponseBody = ();
     fn path(&self) -> String { "apples".to_string()}
     fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }
 }









pub struct LoginApi{
    url:String,
    userinfo:UserInfo,
}
#[derive(Default,Serialize, Deserialize,PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo{
    pub loginid:String,
    pub passwd:String,
}


pub enum Msg {
    LoginFetchState(FetchAction<UserInfo>),
    Request,
    Response(Result<UserInfo, HttpError>),
    Ignore,
    UpdateEmail(String),
    UpdatePassword(String),
    GoTo,
}



    pub fn login(callback: Callback<Result<UserInfo, Error>>,login:UserInfo) -> FetchTask {
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
        let mut builder=Request::builder()
            .method("post")
            .uri(url)
            .header("Content-Type", "application/json");
        // let request=builder.body(login).unwrap();  
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
            login_req:Default::default(),
            link:link.clone(),
            request:req,
            response:link.callback(Msg::Response),
            task:None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            message:String::from("******"),
        }
    }

    fn update(&mut self, msg: Self::Message)->ShouldRender{
        match msg{
            Msg::LoginFetchState(state)=>{
                
                
                // self.login_req.request.userinfo.loginid="Str".to_owned();
                self.login_req.apply(state);
                
            },
            Msg::Request=>{
                // self.request.userinfo.loginid=String::from("ok");
                // let task=login(self.response.clone(),self.request.userinfo.clone());
                // self.task = Some(task);
                self.task=None;
                
            },
            Msg::UpdateEmail(loginid)=>{
                self.request.userinfo.loginid=loginid;
            },
            Msg::UpdatePassword(passwd)=>{
                self.request.userinfo.passwd=passwd;
            },Msg::Response(Err(err)) => {
                self.task = None;
                self.message=String::from("error");
            },
            Msg::Response(Ok(res)) => {
                self.request.userinfo.loginid=res.loginid;
                self.request.userinfo.passwd=res.passwd;
                self.message=String::from("ok");
                self.router_agent.send(ChangeRoute(Routes::Register.into()));

            },
            Msg::Ignore => {},
            Msg::GoTo=>{
                // let route = Route::new_no_state("/main.html");
                // AppRoute::switch(route);
                
                // self.link.send_future(self.login_req.fetch(Msg::LoginFetchState));
                
                // self.router_agent.send(ChangeRoute(Routes::Register.into()));


                let mut http=Http{};
                self.task= Some(http.post("home/formlogin1".to_string(), self.request.userinfo.clone(), self.response.clone()));

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
            <RouterAnchor<HomeRoute> route=HomeRoute::Main>
            { "click" }
            </RouterAnchor<HomeRoute>>
            <input type="button" onclick=btngoto value="goto"/>
            <form onsubmit=onsubmit>
            <input type="text" name="loginid" oninput=oninput_email value=&self.request.userinfo.loginid placeholder="loginid" id="loginid"/>
            <input type="password" name="passwd" oninput=oninput_password value=&self.request.userinfo.passwd placeholder="passwd" id="passwd"/>
            <div>{&self.message}</div>
            <input type="submit" value="login"/>
            </form>
            </>
        }
    }

}