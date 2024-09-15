use web_sys::SubmitEvent;
use yew::{html, Component, Context, Html, classes};
use wasm_bindgen::JsCast;
use log::info;
use gloo_timers::callback::Interval;

use crate::AppProps;
use crate::timer::Timer;
use crate::components::Timer as TimerComponent;

pub struct TimerPage {
    _interval: Option<Interval>,
}

pub enum TimerPageMessage {
    AddTimer(Timer),
    RemoveTimer(i32),
    UpdateTimer(i32, bool),
    Tick,
}

impl Component for TimerPage {
    type Message = TimerPageMessage;
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let interval = Interval::new(500, move || link.send_message(TimerPageMessage::Tick));
        Self {
            _interval: Some(interval),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimerPageMessage::AddTimer(timer) => {
                info!("Adding timer: {:?}", timer);
                ctx.props().timer_add_callback.emit(timer);
                true
            }
            TimerPageMessage::RemoveTimer(id) => {
                info!("Removing timer: {:?}", id);
                ctx.props().timer_remove_callback.emit(id);
                true
            }
            TimerPageMessage::UpdateTimer(id, is_running) => {
                info!("Updating timer: {:?} to {:?}", id, is_running);
                ctx.props().timer_update_callback.emit((id, is_running));
                true
            }
            TimerPageMessage::Tick => {
                // Force a re-render every second
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let next_id = ctx.props().timers.borrow().len() as i32;
        let on_submit = ctx.link().callback(move |form_data: SubmitEvent| {
            // Prevent the default form submission behavior
            form_data.prevent_default();
            let form: web_sys::HtmlFormElement = form_data.target().unwrap().dyn_into().unwrap();
            let name = if let Some(name_element) = form.get_with_name("timer-name") {
                name_element.dyn_into::<web_sys::HtmlInputElement>().unwrap().value()
            } else {
                "New timer".to_string()
            };
            let duration = if let Some(duration_element) = form.get_with_name("timer-duration") {
                duration_element.dyn_into::<web_sys::HtmlInputElement>().unwrap().value().parse::<u32>().unwrap_or(0)
            } else {
                0
            };
            TimerPageMessage::AddTimer(Timer {
                id: next_id,
                name: name,
                duration: duration,
                is_running: false,
                start_time: chrono::Utc::now().timestamp() as u32,
            })
        });

        html! {
            <>
                <div>
                    <h1>{"Timers"}</h1>
                    <div class="timer-list h-75">
                        {ctx.props().timers.borrow().iter().map(|timer| {
                            let timer_id = timer.id;
                            let new_state = !timer.is_running;
                            let on_delete = ctx.link().callback(move |_| TimerPageMessage::RemoveTimer(timer_id));
                            let on_toggle = ctx.link().callback(move |_| TimerPageMessage::UpdateTimer(timer_id, new_state));

                            html! {
                                <TimerComponent timer={timer.clone()} on_toggle={on_toggle} on_delete={on_delete} />
                            }
                        }).collect::<Html>()}
                        if ctx.props().timers.borrow().is_empty() {
                            <p>{"No timers yet"}</p>
                        }
                    </div>

                    <form onsubmit={on_submit} class="mt-4">
                        <div class={classes!("mb-3", "form-group")}>
                            <input type="text" name="timer-name" class={classes!("form-control")} placeholder="Timer name" />
                        </div>
                        <div class={classes!("mb-3", "form-group")}>
                            <input type="number" name="timer-duration" class={classes!("form-control")} placeholder="Timer duration" />
                        </div>
                        <button type="submit" class={classes!("btn", "btn-primary")}>{"Add timer"}</button>
                    </form>
                </div>
            </>
        }
    }
}
