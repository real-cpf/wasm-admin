use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Routes;

pub struct Header{
    link:ComponentLink<Self>,
}

#[derive(Clone)]
pub enum Msg{
    Ignore,
}

impl Component for Header{
    type Message=Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{
        Header{
            link,
        }
    }

    fn update(&mut self,msg: Self::Message)->ShouldRender{
        true
    }  
    fn change(&mut self, _props: Self::Properties)->ShouldRender{
        true
    }

    fn view(&self)->Html{
        html!{
            <div>
                <header class="top-header">
                    <div class="top-logo">
                        {"logo"}
                    </div>
                    <div class="top-login">
                        <a href="#">{"login"}</a>
                    </div>
                </header>

                <header class="menu">
                    <RouterAnchor<Routes> route=Routes::Upload classes="menu menu-button">
                    { "upload" }
                    </RouterAnchor<Routes>>
                    <RouterAnchor<Routes> route=Routes::Login classes="menu menu-button">
                    { "login" }
                    </RouterAnchor<Routes>>
                    <RouterAnchor<Routes> route=Routes::Center classes="menu menu-button">
                    { "center" }
                    </RouterAnchor<Routes>>
                    <RouterAnchor<Routes> route=Routes::DBForm classes="menu menu-button">
                    { "dbform" }
                    </RouterAnchor<Routes>>
                    <RouterAnchor<Routes> route=Routes::WsPage classes="menu menu-button">
                    { "WsPage" }
                    </RouterAnchor<Routes>>
                </header>
            </div>
        }
    }



}