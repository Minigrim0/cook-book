use yew::{function_component, html, Html, Properties};
use yew::{Callback, MouseEvent};

use crate::timer;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub timer: timer::Timer,
    pub on_toggle: Callback<MouseEvent>,
    pub on_delete: Callback<MouseEvent>,
}

#[function_component]
pub fn Timer(props: &Props) -> Html {
    let (progress, remaining) = {
        let elapsed = if props.timer.duration > 0 {
            (chrono::Utc::now().timestamp() as u32 - props.timer.start_time) % props.timer.duration
        } else {
            0
        };
        let progress = if props.timer.duration > 0 {
            (elapsed as f32 / props.timer.duration as f32) * 100.0
        } else {
            0.0
        };
        let remaining = props.timer.duration - elapsed;
        (progress, remaining)
    };

    html! {
        <div class="timer-item">
            <div class="circular-progress" style={format!("--progress: {}deg", progress * 3.6)}>
                <div class="inner-circle">
                    <span class="timer-name">{&props.timer.name}</span>
                    <span class="timer-duration">{format!("{}s", remaining)}</span>
                </div>
            </div>
            <div class="timer-controls">
                <button class="toggle-btn" onclick={props.on_toggle.clone()}>
                    {if props.timer.is_running { "Stop" } else { "Resume" }}
                </button>
                <button class="delete-btn" onclick={props.on_delete.clone()}>{"Delete"}</button>
            </div>
        </div>
    }
}