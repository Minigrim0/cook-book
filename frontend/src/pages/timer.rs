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
    ToggleTimer(i32),
    StopTimer(i32),
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
            TimerPageMessage::ToggleTimer(id) => {
                info!("Toggling timer: {:?}", id);
                ctx.props().timer_update_callback.emit(id);
                true
            }
            TimerPageMessage::StopTimer(id) => {
                info!("Stopping timer: {:?}", id);
                ctx.props().timer_stop_callback.emit(id);
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

            let hours = form
                .get_with_name("timer-hours")
                .map_or(0, |e| e
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<u32>()
                    .unwrap_or(0));
            let minutes = form
                .get_with_name("timer-minutes")
                .map_or(0, |e| e
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<u32>()
                    .unwrap_or(0));
            let seconds = form
                .get_with_name("timer-seconds")
                .map_or(0, |e| e
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<u32>()
                    .unwrap_or(0));

            let duration = (hours * 3600) + (minutes * 60) + seconds;

            TimerPageMessage::AddTimer(Timer {
                id: next_id,
                name,
                duration,
                is_running: true,  // Start the timer immediately
                elapsed_time: 0,
            })
        });

        html! {
            <>
                <div>
                    <h1>{"Timers"}</h1>
                    <div class="timer-list h-75">
                        {ctx.props().timers.borrow().iter().map(|timer| {
                            let timer_id = timer.id;
                            let on_delete = ctx.link().callback(move |_| TimerPageMessage::RemoveTimer(timer_id));
                            let on_toggle = ctx.link().callback(move |_| TimerPageMessage::ToggleTimer(timer_id));
                            let on_stop = ctx.link().callback(move |_| TimerPageMessage::StopTimer(timer_id));

                            html! {
                                <TimerComponent timer={timer.clone()} on_toggle={on_toggle} on_delete={on_delete} on_stop={on_stop} />

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
                        <div class={classes!("mb-3", "form-group", "d-flex")}>
                            <input type="number" name="timer-hours" class={classes!("form-control", "me-2")} placeholder="Hours" min="0" />
                            <input type="number" name="timer-minutes" class={classes!("form-control", "me-2")} placeholder="Minutes" min="0" max="59" />
                            <input type="number" name="timer-seconds" class={classes!("form-control")} placeholder="Seconds" min="0" max="59" />
                        </div>

                        <button type="submit" class={classes!("btn", "btn-primary")}>{"Add timer"}</button>
                    </form>
                </div>
            </>
        }
    }
}
