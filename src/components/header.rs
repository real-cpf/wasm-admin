use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;

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
                    <a href="#" class="menu menu-button">{"menuA"}</a>
                    <a href="#" class="menu menu-button">{"menuV"}</a>
                    <a href="#" class="menu menu-button">{"menuX"}</a>
                    <a href="#" class="menu menu-button">{"menuZ"}</a>
                    <a href="#" class="menu menu-button">{"menuS"}</a>
                </header>
            </div>
        }
    }



}