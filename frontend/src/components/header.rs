use log::info;
use yew::{classes, function_component, html, Html};
use yew_router::history::Location;
use yew_router::prelude::{use_location, Link};
use yew_router::Routable;

use crate::Route;

/// Returns the "active" class string if the provided route matches the location
fn active(route: Route, location: &Option<Location>) -> String {
    if let Some(location) = location {
        if route.to_path() == location.path() {
            "active".into()
        } else {
            "".into()
        }
    } else {
        "".into()
    }
}

#[function_component]
pub fn HeaderComponent() -> Html {
    let location: Option<Location> = if let Some(location) = use_location() {
        info!("{:?}", location.path());
        Some(location)
    } else {
        None
    };

    html! {
        <nav class={"navbar navbar-expand-lg navbar-light bg-light"}>
            <div class={"container-fluid"}>

                <button class={classes!("navbar-brand")} type="button" data-bs-toggle="offcanvas" data-bs-target="#offcanvasExample" aria-controls="offcanvasExample">
                    <i class="bi bi-list"></i>
                </button>

                <button class={"navbar-toggler"} type="button" data-bs-toggle="collapse" data-bs-target="#navbarMainMenu" aria-controls="navbarMainMenu" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>

                <div class="collapse navbar-collapse" id="navbarMainMenu">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <Link<Route> classes={classes!("nav-link", active(Route::Home, &location))} to={Route::Home}>{"Home"}</Link<Route>>
                        </li>
                        <li class="nav-item">
                            <Link<Route> classes={classes!("nav-link", active(Route::Timers, &location))} to={Route::Timers}>{"Timer"}</Link<Route>>
                        </li>
                        <li class="nav-item">
                            <Link<Route> classes={classes!("nav-link", active(Route::Converters, &location))} to={Route::Converters}>{"Converter"}</Link<Route>>
                        </li>
                    </ul>

                    <form class="d-flex">
                        <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search" />
                        <button class="btn btn-outline-success" type="submit">{"Search"}</button>
                    </form>

                </div>
            </div>
        </nav>
    }
}
