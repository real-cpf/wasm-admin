
use yew_router::prelude::*;
use yew::callback::Callback;
use crate::components::LoginInfo;


//index要放在最后 否则会自动请求index
#[derive(Switch, Debug, Clone)]
pub enum Routes {
    #[to="/wspage"]
    WsPage,
    #[to="/upload"]
    Upload,
    #[to = "/login"]
    Login,
    #[to = "/register"]
    Register,
    #[to = "/home"]
    Home,
    #[to = "/db"]
    DBForm,
    #[to = "/"]
    Center,

}

