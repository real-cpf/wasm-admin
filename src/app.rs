use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct Model {
    link: ComponentLink<Self>,
    clicked: bool,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            clicked: false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <h1>{"hello word"}</h1>
            <h1>{"main "}</h1>
            // <div>
            //     <button onclick=self.link.callback(|_| Msg::Click)>{ "Click ( false )" }</button>
            //     <button onclick=self.link.callback(|_| Msg::Click)>{ "Click ( wasm-bindgen )" }</button>
            //     <p>{format!("Has been clicked: {}", self.clicked)}</p>
            // </div>
            </div>
            
        }
    }
}
