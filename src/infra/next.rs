use crate::app::routing::Commands;
use crate::infra::request::Request;
use crate::infra::route::Route;

#[derive(Clone)]
pub struct Next {
    pub route: Commands,
    pub request: Option<Request>,
}

impl Next {
    pub fn new(route: Commands, request: Option<Request>) -> Self {
        Self { request, route }
    }
}
