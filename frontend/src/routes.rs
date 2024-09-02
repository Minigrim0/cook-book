use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/timers")]
    Timers,
    #[at("/converter")]
    Converters,
    #[at("/load")]
    Loaders,
}
