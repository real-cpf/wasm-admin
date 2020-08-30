use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
type AsBinary = bool;

pub struct WsPage{
    link:ComponentLink<Self>,
    ws: Option<WebSocketTask>,
    fetching: bool,
    data:Option<u32>,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}


pub enum Msg{
    Ignore,
    Connect,
    SendData(AsBinary),
    Disconnect,
    Lost,
    WsReady(Result<WsResponse, Error>),
}

impl Component for WsPage{
    type Message=Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{
        WsPage{
            link,
            fetching:false,
            data:Some(0),
            ws:None,
        }
    }

    fn update(&mut self,msg: Self::Message)->ShouldRender{
        match msg{
            Msg::Connect=>{
                let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
                let notification = self.link.callback(|status| match status {
                        WebSocketStatus::Opened => Msg::Ignore,
                        WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Lost.into(),
                    });
                let task =WebSocketService::connect("ws://localhost:9001/", callback, notification)
                            .unwrap();
                self.ws = Some(task);
            },
            Msg::SendData(binary)=>{
                let request = WsRequest { value: 321 };
                if binary {
                    self.ws.as_mut().unwrap().send_binary(Json(&request));
                } else {
                    self.ws.as_mut().unwrap().send(Json(&request));
                }
            },
            Msg::Disconnect=>{
                self.data=Some(0);
                self.ws.take();
            },Msg::Lost=>{
                self.data=Some(0);
                self.ws=None;
            },
            Msg::WsReady(response)=>{
                self.data=response.map(|data| data.value).ok();
            },
            Msg::Ignore => {
                return false;
            }

        }
        true
    }  
    fn change(&mut self, _props: Self::Properties)->ShouldRender{
        true
    }

    fn view(&self)->Html{
        html!{
            <div>
            <div>{format!("now data is {}",self.data.unwrap())}</div>
                <button disabled=self.ws.is_some()
                onclick=self.link.callback(|_| Msg::Connect)>
                { "Connect To WebSocket" }
                </button>
                <button disabled=self.ws.is_none()
                        onclick=self.link.callback(|_| Msg::SendData(false))>
                    { "Send To WebSocket" }
                </button>
                <button disabled=self.ws.is_none()
                        onclick=self.link.callback(|_| Msg::SendData(true))>
                    { "Send To WebSocket [binary]" }
                </button>
                <button disabled=self.ws.is_none()
                        onclick=self.link.callback(|_| Msg::Disconnect)>
                    { "Close WebSocket connection" }
                </button>
            </div>
        }
    }



}