use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav>
            <ul>
                <li>
                <Link<Route> to={Route::Home}>
                    {"Home"}
                </Link<Route>>
                </li>
                <li>
                <Link<Route> to={Route::About}>
                    {"About"}
                </Link<Route>>
                </li>
                <li>
                <Link<Route> to={Route::Contact}>
                    {"Contact"}
                </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}
