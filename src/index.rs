use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use yew_router::{prelude::*, Switch};

use yew::virtual_dom::VNode;

use crate::components::LoginForm;
use crate::components::LoginInfo;
use crate::routes::Routes;
// use crate::login::Login;
use crate::app::Model;

pub struct Index {
    userinfo:Option<LoginInfo>,
    link: ComponentLink<Self>,
}

pub enum Msg{
    IsLogin(LoginInfo),
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index {
            userinfo:Default::default(),
            link:link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::IsLogin(userinfo)=>{
                self.userinfo=Some(userinfo);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback_login = self.link.callback(Msg::IsLogin);
        html! {
            <>
            <div style="color: red;">{show_header(self.userinfo.clone())}</div>
            <LoginForm callback=callback_login/>
        //    <nav>
        //    <div style="height: 50px;width: 100%;border-width: 1px;border-color: red;">

        //    <div class="top-box">
        //    <RouterButton<Routes>  route=Routes::Login> {"Login"} </RouterButton<Routes>>
        //    </div>

        //    <div class="top-box">
        //    <RouterButton<Routes>  route=Routes::Register> {"register"} </RouterButton<Routes>>
        //    </div>

        //    <div class="top-box">
        //    {show_header(self.userinfo.clone())}
        //    </div>
        //    </div>
        //    </nav>

        //    <div style="text-align: center">
        //    <Router<Routes>
        //        render = Router::render(  |switch: Routes| {
        //            match switch {
        //             Routes::Login=>{
        //                 let new_call=callback_login.clone();
        //                 html!{<LoginForm callback=new_call />}
        //             },
        //             // Routes::Login=>html!{<div>{"home"}</div>},
        //             Routes::Register=>html!{<Model/>},
        //             Routes::Home=>html!{<div>{"home"}</div>},
        //            }
        //        })
          //  //    redirect = Router::redirect(|route: Route| {
           // //        AppRoute::PageNotFound(Permissive(Some(route.route)))
          //  //    })
    //        />
    //    </div>


           </>
        }
    }

}
fn show_header(userinfo:Option<LoginInfo>)->VNode{
    if !userinfo.is_some(){
        html!{
            <>
            {"未登录"}
            </>
        }
    }else{
        html!{
            <>
            {format!("用户：{}，欢迎你！",userinfo.unwrap().loginid)}
            </>
        }
    }
}
