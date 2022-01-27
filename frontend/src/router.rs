use std::collections::VecDeque;
use zoon::{println, *};
use crate::{Page, page_id, set_page_id, get_user};

// ------ route_history ------

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| {
        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => {
                return set_page_id(Page::Unknown);
            }
        };

        match route {
            Route::Login => {
                if get_user().map(Option::is_some) {
                    return router().replace(Route::Root)
                }
                set_page_id(Page::Login)
            }
            Route::Root => {
                set_page_id(Page::Home);
            }
        }
    })
}

// ------ Route ------

#[route]
#[derive(Clone)]
pub enum Route {
    #[route("login")]
    Login,
    #[route()]
    Root,
}