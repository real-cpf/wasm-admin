use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use yew_router::{prelude::*, Switch};

use yew::virtual_dom::VNode;

use crate::home::HomePage;
use crate::components::LoginForm;
use crate::components::Upload;
use crate::components::LoginInfo;
use crate::routes::Routes;
// use crate::login::Login;
use crate::app::Model;
use crate::util::get_token;

pub struct Index {
    current_route: Option<Routes>,
    userinfo:Option<LoginInfo>,
    link: ComponentLink<Self>,
}

pub enum Msg{
    IsLogin(LoginInfo),
    RouteChange(Route),
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service: RouteService = RouteService::new();
        let mut route = route_service.get_route();
        Index {
            current_route:Routes::switch(route),
            userinfo:Default::default(),
            link:link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::IsLogin(userinfo)=>{
                self.userinfo=Some(userinfo);
            },
            Msg::RouteChange(route)=>{
                self.current_route=Routes::switch(route);
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback_login = self.link.callback(Msg::IsLogin);    
                    html!{
                        <>
                      <div>
                     { show_user_header(get_token(),callback_login)}


                                <Router<Routes>
                                render = Router::render(  |switch: Routes| {
                                    match switch {
                                        // Routes::Login=>{
                                        //     let new_call=callback_login.clone();
                                        //     html!{<LoginForm callback=new_call />}
                                        // },
                                        Routes::Upload=>html!{<Upload/>},
                                        Routes::Login=>html!{<div>{"home"}</div>},
                                        Routes::Register=>html!{<Model/>},
                                        Routes::Home=>html!{<div>{"home"}</div>},
                                    }
                                })
                            //    redirect = Router::redirect(|route: Route| {
                            //        AppRoute::PageNotFound(Permissive(Some(route.route)))
                            //    })
                            />
                      </div>
                       
                    
                
            
            // <div style="color: red;">{show_user_header(get_token(),callback_login.clone())}</div>
            // <RouterAnchor<Routes> route=Routes::Login classes="nav-link">
            // { "login" }
            // </RouterAnchor<Routes>>
            // <a href="/login">{"aaa"}</a>
            // {
            //     if let Some(route)=&self.current_route{
            //         match route{
            //             Routes::Home=>html!{
            //                 <HomePage/>
            //             },
            //             Routes::Login=>html!{
            //                 <LoginForm  callback=callback_login />
            //             },
            //             Routes::Register=>html!{
            //                 <Model/>
            //             },
            //         }
            //     }else{
            //         html! { "No child component available" }
            //     }
            // }
            // <LoginForm callback=callback_login/>
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
        //    />
    //    </div>


           </>
        }
    }

}


fn show_user_header(userinfo:Option<String>,_callback:Callback<LoginInfo>)->VNode{
    if !userinfo.is_some(){
        html!{
            <>
            <LoginForm callback=_callback/>
            </>
        }
    }else{
        html!{
            <>
            <RouterAnchor<Routes> route=Routes::Upload>
            { "upload test" }
            </RouterAnchor<Routes>>
            {format!("用户：{}，欢迎你！",userinfo.unwrap())}

            </>
        }
    }
}
