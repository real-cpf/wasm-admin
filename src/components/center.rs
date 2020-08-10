use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;

pub struct Center{
    link:ComponentLink<Self>,
}

#[derive(Clone)]
pub enum Msg{
    Ignore,
}

impl Component for Center{
    type Message=Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{
        Center{
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
            <div style="height:600px;widht:800px;background-color:blue">
            {
                "hello"
            }
            </div>
        }
    }



}