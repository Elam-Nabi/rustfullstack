use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{about::About, contact::Contact, home::Home, not_found::NotFound};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::About => html! {<About />},
        Route::Contact => html! {<Contact />},
        Route::NotFound => html! {<NotFound />},
    }
}
