use std::rc::Rc;
use std::cell::RefCell;

use yew::prelude::*;
use yew_router::prelude::*;
use gloo_timers::callback::Interval;

mod components;
mod pages;
mod routes;
mod timer;
mod switch;

mod glue;
use glue::*;

use switch::switch;
use timer::Timer;
use components::{FooterComponent, HeaderComponent, SidebarComponent};
use routes::{Route, ToolsRoute};

#[derive(Properties, PartialEq, Clone)]
pub struct AppProps {
    pub timer_add_callback: Callback<Timer>,
    pub timer_remove_callback: Callback<i32>,
    pub timer_update_callback: Callback<(i32, bool)>,
    pub timers: Rc<RefCell<Vec<Timer>>>,
}

pub struct App {
    timers: Rc<RefCell<Vec<Timer>>>,
    _interval: Interval,
}

pub enum AppMsg {
    AddTimer(Timer),
    RemoveTimer(i32),
    UpdateTimer((i32, bool)),
    Tick,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let interval = Interval::new(1000, move || link.send_message(AppMsg::Tick));
        
        Self {
            timers: Rc::new(RefCell::new(Vec::new())),
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::AddTimer(timer) => {
                self.timers.borrow_mut().push(timer);
                true
            }
            AppMsg::RemoveTimer(id) => {
                let mut timers = self.timers.borrow_mut();
                if let Some(index) = timers.iter().position(|t| t.id == id) {
                    timers.remove(index);
                    true
                } else {
                    false
                }
            }
            AppMsg::UpdateTimer((id, is_running)) => {
                let mut timers = self.timers.borrow_mut();
                if let Some(timer) = timers.iter_mut().find(|t| t.id == id) {
                    timer.is_running = is_running;
                    true
                } else {
                    false
                }
            }
            AppMsg::Tick => {
                let mut timers = self.timers.borrow_mut();
                for timer in timers.iter_mut() {
                    if timer.is_running {
                        if timer.elapsed_time < timer.duration {
                            timer.elapsed_time += 1;
                        } else {
                            timer.is_running = false;
                        }
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_timer_added = ctx.link().callback(AppMsg::AddTimer);
        let on_timer_removed = ctx.link().callback(AppMsg::RemoveTimer);
        let on_timer_updated = ctx.link().callback(AppMsg::UpdateTimer);

        let props = AppProps {
            timers: Rc::clone(&self.timers),
            timer_add_callback: on_timer_added,
            timer_remove_callback: on_timer_removed,
            timer_update_callback: on_timer_updated,
        };

        html! {
            <div class={"content"}>
                <BrowserRouter>
                    <HeaderComponent />
                    <SidebarComponent />
                    <div class="container-fluid flex-fluid" style="min-height: 50vh">
                        <Switch<Route> render={move |route| switch(route, props.clone())} />
                    </div>
                    <FooterComponent />
                </BrowserRouter>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
