use std::rc::Rc;
use std::cell::RefCell;

use yew::{function_component, html, Html, Properties, Callback, MouseEvent};
use web_sys::{EventTarget, HtmlButtonElement, HtmlElement};
use wasm_bindgen::JsCast;
use log::error;

use crate::timer::Timer;
use crate::components::AddTimerModal;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub timers: Rc<RefCell<Vec<Timer>>>,
    pub on_create_timer: Callback<Timer>,
    pub on_delete_timer: Callback<i32>,
    pub on_toggle_timer: Callback<i32>,
    pub on_stop_timer: Callback<i32>,
}

#[function_component(ActiveTimersDropdown)]
pub fn active_timers_dropdown(props: &Props) -> Html {
    let timers = props.timers.clone();

    let on_toggle_timer = {
        let on_toggle_timer = props.on_toggle_timer.clone();
        Callback::from(move |event: MouseEvent| {
            event.stop_propagation();
            let target: Option<EventTarget> = event.target();
            let button = target.and_then(|t| t.dyn_into::<HtmlElement>().ok())
            .and_then(|el| el.closest("button").ok().flatten())
            .and_then(|el| el.dyn_into::<HtmlButtonElement>().ok());

            if let Some(button) = button {
                let timer_id = button.value().parse::<i32>().unwrap();
                on_toggle_timer.emit(timer_id);
            } else {
                error!("Failed to toggle timer");
            }
        })
    };

    let on_stop_timer = {
        let on_stop_timer = props.on_stop_timer.clone();
        Callback::from(move |event: MouseEvent| {
            event.stop_propagation();
            let target: Option<EventTarget> = event.target();
            let button = target.and_then(|t| t.dyn_into::<HtmlElement>().ok())
                .and_then(|el| el.closest("button").ok().flatten())
                .and_then(|el| el.dyn_into::<HtmlButtonElement>().ok());

            if let Some(button) = button {
                match button.value().parse::<i32>() {
                    Ok(timer_id) => on_stop_timer.emit(timer_id),
                    Err(e) => error!("Failed to parse timer ID: {}", e),
                }
            } else {
                error!("Failed to find stop timer button");
            }
        })
    };

    let on_delete_timer = {
        let on_delete_timer = props.on_delete_timer.clone();
        Callback::from(move |event: MouseEvent| {
            event.stop_propagation();
            let target: Option<EventTarget> = event.target();
            let button: Option<HtmlButtonElement> = target.and_then(|t| t.dyn_into::<HtmlElement>().ok())
            .and_then(|el| el.closest("button").ok().flatten())
            .and_then(|el| el.dyn_into::<HtmlButtonElement>().ok());

            if let Some(button) = button {
                let timer_id: i32 = button.value().parse().unwrap();
                on_delete_timer.emit(timer_id);
            } else {
                error!("Failed to delete timer");
            }
        })
    };

    html! {
        <div class="dropdown">
            <button class="btn btn-outline-secondary dropdown-toggle" type="button" id="activeTimersDropdown" data-bs-toggle="dropdown" aria-expanded="false">
                <i class="bi bi-clock me-2"></i>
                {"Active Timers"}
            </button>
            <ul class="dropdown-menu" aria-labelledby="activeTimersDropdown">
                {timers.borrow().iter().map(|timer| {
                    let progress = (timer.elapsed_time as f32 / timer.duration as f32) * 100.0;

                    let time_left = if timer.duration - timer.elapsed_time > 3600 {
                        format!(
                            "{}:{}:{}",
                            (timer.duration - timer.elapsed_time) / 3600,
                            (timer.duration - timer.elapsed_time) % 3600 / 60,
                            (timer.duration - timer.elapsed_time) % 60
                        )
                    } else if timer.duration - timer.elapsed_time > 0 {
                        format!("{}:{}", (timer.duration - timer.elapsed_time) / 60, (timer.duration - timer.elapsed_time) % 60)
                    } else {
                        format!("{}s", (timer.duration - timer.elapsed_time) as f32)
                    };

                    html!{
                        <li>
                            <div class="dropdown-item">
                                <div class="d-flex justify-content-between align-items-center">
                                    <span>{&timer.name}</span>
                                    <span>{&time_left}</span>
                                </div>
                                <div class="d-flex align-items-center mt-2">
                                    <div class="progress flex-grow-1" style="height: 5px;">
                                        <div class="progress-bar" role="progressbar" style={format!("width: {}%;", progress)} aria-valuenow={progress.to_string()} aria-valuemin="0" aria-valuemax="100"></div>
                                    </div>
                                    <div class="ms-2">
                                        <button onclick={&on_toggle_timer} value={timer.id.to_string()} class="btn btn-sm btn-outline-secondary p-0" style="width: 20px; height: 20px;">
                                            {if timer.is_running {
                                                html! {<i class="bi bi-pause-fill"></i>}
                                            } else {
                                                html! {<i class="bi bi-play-fill"></i>}
                                            }}
                                        </button>
                                        <button onclick={&on_stop_timer} value={timer.id.to_string()} class="btn btn-sm btn-outline-secondary p-0 ms-1" style="width: 20px; height: 20px;">
                                            <i class="bi bi-stop-fill"></i>
                                        </button>
                                        <button onclick={&on_delete_timer} value={timer.id.to_string()} class="btn btn-sm btn-outline-danger p-0 ms-1" style="width: 20px; height: 20px;">
                                            <i class="bi bi-x"></i>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </li>
                    }
                }).collect::<Html>()}
                if timers.borrow().is_empty() {
                    <li>
                        <div class="dropdown-item">
                            <span>{"No active timers"}</span>
                        </div>
                    </li>
                }
                <li><hr class="dropdown-divider" /></li>
                <li>
                    <div class="dropdown-item">
                        <button class="btn btn-outline-success w-100" type="button" data-bs-toggle="modal" data-bs-target="#addTimerModal">
                            <i class="bi bi-plus-circle me-2"></i>{"Add Timer"}
                        </button>
                    </div>
                </li>
            </ul>

        <AddTimerModal on_create_timer={props.on_create_timer.clone()} next_id={props.timers.borrow().len() as i32} />
        </div>
    }
}