use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;

pub struct Footer{
    link:ComponentLink<Self>,
}

#[derive(Clone)]
pub enum Msg{
    Ignore,
}

impl Component for Footer{
    type Message=Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{
        Footer{
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
            <footer>
                <div>
                    <a href="/index.html"></a>
                    <span class="attribution">
                        { "© 2019.##############" }
                        <a href="#"> { "real_cpf" } </a>
                        { ". Code licensed under MIT.aaa" }
                    </span>
                </div>
            </footer>
            </div>
        }
    }



}