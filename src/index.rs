use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use yew_router::{prelude::*, Switch};

use yew::virtual_dom::VNode;

use crate::routes::Routes;
use crate::login::Login;
use crate::app::Model;

pub struct Index {}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {

        html! {
            <>
            
           <nav>
           <div style="height: 50px;width: 100%;border-width: 1px;border-color: red;">

           <div class="top-box">
           <RouterButton<Routes>  route=Routes::Login> {"Login"} </RouterButton<Routes>>
           </div>

           <div class="top-box">
           <RouterButton<Routes>  route=Routes::Register> {"register"} </RouterButton<Routes>>
           </div>

           </div>
           </nav>

           <div>
           <Router<Routes>
               render = Router::render(|switch: Routes| {
                   match switch {
                    Routes::Login=>html!{<Login/>},
                    Routes::Register=>html!{<Model/>},
                    Routes::Home=>html!{<div>{"home"}</div>},
                   }
               })
            //    redirect = Router::redirect(|route: Route| {
            //        AppRoute::PageNotFound(Permissive(Some(route.route)))
            //    })
           />
       </div>


           </>
        }
    }
}
