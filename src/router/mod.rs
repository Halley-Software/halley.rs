use core::fmt;
use std::vec::Vec;

use crate::streams::request::Request;
use crate::streams::response::Reply;

pub type HalleyHandler = fn(req: Request, res: Reply) -> ();

pub struct Route {
    pub path: &'static str,
    pub method: &'static str,
    pub handler: HalleyHandler
}

impl fmt::Debug for Route {
    fn fmt(&self, route_struct: &mut fmt::Formatter<'_>) -> fmt::Result {
        route_struct.debug_struct("Route")
          .field("Path", &self.path)
          .field("Method", &self.method)
          .field("Handler", &"<Lambda Function>")
          .finish()
    }
}
pub struct Router {
    pub route_stack: Vec<Route>
}

fn find_route<Callback>(iterable_item: &Vec<Route>, callback: Callback) -> Option<Route>
    where Callback: Fn(&Route) -> bool
{
    let mut route_return: Option<Route> = None;

    'returns_matched_route: for route in iterable_item {
        if callback(route) {
            route_return = Some(Route {
                path: route.path,
                method: route.method,
                handler: route.handler
            });
            break 'returns_matched_route;
        }
    }

    return route_return;
}

impl Router {
    pub fn new(initial_routes: Option<Vec<Route>>) -> Router {
        if let Some(routes) = initial_routes {
            Self {route_stack: routes}
        } else {
            Self {route_stack: Vec::new()}
        }
    }

    /// Find the route that match with the callback condition passed in `find_route` function
    pub fn find_requested_route(&self, path: &str, method: &str) -> Option<Route> {
        if let Some(matched_route) = find_route(&self.route_stack, |route_matching| {
            route_matching.path == path && route_matching.method == method
        }) {
            return Some(matched_route);
        } else {
            return None;
        }

    }

    pub fn add(&mut self, new_route: Route) -> () {
        self.route_stack.push(new_route);
    }

    pub fn multiple_add(&mut self, mut new_route: Vec<Route>) -> () {
        self.route_stack.append(&mut new_route);
    }
}