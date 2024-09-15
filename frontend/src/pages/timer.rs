use web_sys::SubmitEvent;
use yew::{html, Component, Context, Html};
use wasm_bindgen::JsCast;
use log::info;

use crate::AppProps;
use crate::timer::Timer;

pub struct TimerPage;

pub enum TimerPageMessage {
    AddTimer(Timer),
    RemoveTimer(i32),
    UpdateTimer(i32, bool),
    GetTimers,
}



impl Component for TimerPage {
    type Message = TimerPageMessage;
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(TimerPageMessage::GetTimers);

        Self
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
            // Load timers from inside the browser's local storage
            TimerPageMessage::GetTimers => {
                // let timers = get_timers();
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
            let name = form.get_attribute("name").unwrap_or("New timer".to_string());
            let duration = form.get_attribute("duration").unwrap_or("0".to_string()).parse::<u32>().unwrap_or(0);
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
                            let (progress, remaining) = if timer.is_running {
                                let elapsed = if timer.duration > 0 {
                                    (chrono::Utc::now().timestamp() as u32 - timer.start_time) % timer.duration
                                } else {
                                    0
                                };
                                let progress = if timer.duration > 0 {
                                    (elapsed as f32 / timer.duration as f32) * 100.0
                                } else {
                                    0.0
                                };
                                let remaining = timer.duration - elapsed;
                                (progress, remaining)
                            } else {
                                (0.0, timer.duration)
                            };

                            html! {
                                <div class="timer-item">
                                    <div class="circular-progress" style={format!("--progress: {}deg", progress * 3.6)}>
                                        <div class="inner-circle">
                                            <span class="timer-name">{&timer.name}</span>
                                            <span class="timer-duration">{format!("{}s", remaining)}</span>
                                        </div>
                                    </div>
                                    <div class="timer-controls">
                                        <button class="toggle-btn" onclick={on_toggle}>
                                            {if timer.is_running { "Stop" } else { "Resume" }}
                                        </button>
                                        <button class="delete-btn" onclick={on_delete}>{"Delete"}</button>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()}
                        if ctx.props().timers.borrow().is_empty() {
                            <p>{"No timers yet"}</p>
                        }
                    </div>

                    <form onsubmit={on_submit} class="mt-4">
                        <div class="mb-3">
                            <input type="text" class="form-control" placeholder="Timer name" />
                        </div>
                        <div class="mb-3">
                            <input type="number" class="form-control" placeholder="Timer duration" />
                        </div>
                        <button type="submit" class="btn btn-primary">{"Add timer"}</button>
                    </form>
                </div>
            </>
        }
    }
}
