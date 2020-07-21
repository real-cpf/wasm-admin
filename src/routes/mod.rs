
use yew_router::prelude::*;


#[derive(Switch, Debug, Clone)]
pub enum Routes {
    #[to = "/login"]
    Login,
    #[to = "/register"]
    Register,
    #[to = "/home"]
    Home,
}

