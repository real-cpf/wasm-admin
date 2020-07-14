use crate::app::Model;
use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "#/"]
    Login,
    #[to = "/model"]
    Model,
}