use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

use crate::timer::Timer;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_create_timer: Callback<Timer>,
    pub next_id: i32,
}

#[function_component(AddTimerModal)]
pub fn add_timer_modal(props: &Props) -> Html {
    let name_ref = use_node_ref();
    let hours_ref = use_node_ref();
    let minutes_ref = use_node_ref();
    let seconds_ref = use_node_ref();

    let on_submit = {
        let name_ref = name_ref.clone();
        let hours_ref = hours_ref.clone();
        let minutes_ref = minutes_ref.clone();
        let seconds_ref = seconds_ref.clone();
        let on_create_timer = props.on_create_timer.clone();
        let next_id = props.next_id;

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            let hours = hours_ref.cast::<HtmlInputElement>().unwrap().value().parse::<u32>().unwrap_or(0);
            let minutes = minutes_ref.cast::<HtmlInputElement>().unwrap().value().parse::<u32>().unwrap_or(0);
            let seconds = seconds_ref.cast::<HtmlInputElement>().unwrap().value().parse::<u32>().unwrap_or(0);
            
            let total_seconds = hours * 3600 + minutes * 60 + seconds;
            
            if !name.is_empty() && total_seconds > 0 {
                on_create_timer.emit(Timer {
                    id: next_id,
                    name,
                    duration: total_seconds,
                    is_running: true,  // Start the timer immediately
                    elapsed_time: 0,
                });
            }
        })
    };

    html! {
        <div class="modal fade" id="addTimerModal" tabindex="-1" aria-labelledby="addTimerModalLabel" aria-hidden="true">
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="addTimerModalLabel">{"Add New Timer"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <form onsubmit={on_submit}>
                        <div class="modal-body">
                            <div class="mb-3">
                                <label for="timerName" class="form-label">{"Timer Name"}</label>
                                <input type="text" class="form-control" id="timerName" placeholder="Enter timer name" ref={name_ref} required=true />
                            </div>
                            <div class="mb-3">
                                <label class="form-label">{"Duration"}</label>
                                <div class="row">
                                    <div class="col">
                                        <input type="number" class="form-control" placeholder="Hours" min="0" ref={hours_ref} required=true />
                                    </div>
                                    <div class="col">
                                        <input type="number" class="form-control" placeholder="Minutes" min="0" max="59" ref={minutes_ref} required=true />
                                    </div>
                                    <div class="col">
                                        <input type="number" class="form-control" placeholder="Seconds" min="0" max="59" ref={seconds_ref} required=true />
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                            <button type="submit" class="btn btn-primary">{"Add Timer"}</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}