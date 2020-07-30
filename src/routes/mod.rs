
use yew_router::prelude::*;
use yew::callback::Callback;
use crate::components::LoginInfo;

#[derive(Switch, Debug, Clone)]
pub enum Routes {
    #[to="/upload"]
    Upload,
    #[to = "/login"]
    Login,
    #[to = "/register"]
    Register,
    #[to = "/home"]
    Home,
}

