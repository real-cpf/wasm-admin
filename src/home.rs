use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;

// use crate::login::Login;
use crate::app::Model;

pub struct HomePage {
    route_service: RouteService<()>,
    route: Route<()>,
    link: ComponentLink<Self>,
}


#[derive(Clone)]
pub enum Msg {
    Ignore,
    RouteChanged(Route<()>),
    GoTo(HomeRoute),
}

#[derive(Debug, Switch, Clone)]
pub enum HomeRoute {
    #[to = "/login"]
    Login,
    #[to = "/main"]
    Main
}


impl HomePage{
    fn go_to(&self,home_route:HomeRoute) -> Callback<MouseEvent>{
        self.link.callback(move |_|{
            let route=home_route.clone();
            Msg::GoTo(route)
        })
    }
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);
        HomePage {
            route,
            route_service,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoTo(route)=>{
                self.route = route.into();
                self.route_service.set_route(&self.route.route, ());
            },
            Msg::RouteChanged(route) => {
                self.route = route
            },
            Msg::Ignore=>{}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        html! {
           <>
           <div style="height: 50px;width: 100%;border-width: 1px;border-color: red;">
           <input class="top-box" type="button" onclick=&self.go_to(HomeRoute::Login)  value="login"/>
           <input class="top-box" type="button" onclick=&self.go_to(HomeRoute::Main) value="register"/>
            </div>
           
            <div style="">
            {
                match HomeRoute::switch(self.route.clone()) {
                    // Some(HomeRoute::Login)=>html!{<Login/>},
                    Some(HomeRoute::Login)=>html!{ <div>{"404"}</div>},
                    Some(HomeRoute::Main)=>html!{<Model/>},
                    None=>html!{
                        <div>{"404"}</div>
                    }
                }
            }
            </div>
            
            </>
        }
    }
}
