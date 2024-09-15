use log::{info, warn};
use yew::{classes, html, Component, Context, Html};
use gloo_timers::callback::Interval;
use models::JobWithLogs;

use crate::glue::{recipe_load_path, get_job_status};

pub enum Msg {
    FolderSelect,
    FileSelect,
    UrlEntered,
    AddJobId(i32),
    UpdateJobs,
    SetJobs(Vec<JobWithLogs>),
}

pub struct LoaderPage {
    job_ids: Vec<i32>,
    jobs: Vec<JobWithLogs>,
    _interval: Interval,
}

impl Component for LoaderPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let interval = Interval::new(5000, move || link.send_message(Msg::UpdateJobs));
        Self { 
            job_ids: Vec::new(),
            jobs: Vec::new(),
            _interval: interval,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FolderSelect => {
                info!("Loading from a folder");
                ctx.link().send_future(async {
                    match recipe_load_path().await {
                        Ok(response) => {
                            let id: i32 = match serde_wasm_bindgen::from_value(response)
                                .map_err(|e| format!("Conversion error: {}", e.to_string())) {
                                Ok(id) => id,
                                Err(e) => {
                                    warn!("Error loading recipe: {}", e);
                                    return Msg::AddJobId(-1); // Use a sentinel value for error
                                }
                            };
                            Msg::AddJobId(id)
                        }
                        Err(err) => {
                            warn!("{:?}", err);
                            Msg::AddJobId(-1) // Use a sentinel value for error
                        }
                    }
                });
                false
            }
            Msg::FileSelect => {
                info!("Loading from a file");
                false
            }
            Msg::UrlEntered => {
                info!("Loading from a url");
                false
            }
            Msg::AddJobId(id) => {
                if id != -1 {
                    self.job_ids.push(id);
                    ctx.link().send_message(Msg::UpdateJobs);
                }
                true
            }
            Msg::UpdateJobs => {
                let job_ids = self.job_ids.clone();
                ctx.link().send_future(async move {
                    let mut jobs = Vec::new();
                    for id in job_ids {
                        if let Ok(response) = get_job_status(id).await {
                            if let Ok(job) = serde_wasm_bindgen::from_value(response) {
                                jobs.push(job);
                            }
                        }
                    }
                    Msg::SetJobs(jobs)
                });
                false
            }
            Msg::SetJobs(jobs) => {
                self.jobs = jobs;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_folder_select = ctx.link().callback(|_| Msg::FolderSelect);
        let on_file_select = ctx.link().callback(|_| Msg::FileSelect);
        let on_url_entered = ctx.link().callback(|_| Msg::UrlEntered);

        html! {
            <div class={classes!("position-absolute", "top-50", "start-50", "translate-middle", "w-75")}>
                <h3 class={classes!("w-100", "text-center")}>{"Loader"}</h3>
                <div class={classes!("w-100", "text-center")}>
                    <small class={classes!("text-muted")}>{"Use this tool to load recipes in"}</small>
                </div>
                <div class={classes!("row", "col-12", "p-2")}>
                    <p class={classes!("col-10")}>{"Load from a folder: "}</p>
                    <button class={classes!("col")} onclick={on_folder_select}>{"select folder"}</button>
                </div>
                <div class={classes!("row", "col-12", "p-2")}>
                    <p class={classes!("col-10")}>{"Load from a json file: "}</p>
                    <button class={classes!("col")} onclick={on_file_select}>{"select file"}</button>
                </div>
                <div class={classes!("row", "col-12", "p-2")}>
                    <p class={classes!("col-5")}>{"Load from a url: "}</p>
                    <input  class={classes!("col-5")} placeholder="Enter url here" />
                    <button class={classes!("col")} onclick={on_url_entered}>{"load"}</button>
                </div>
                <div class={classes!("row", "col-12", "p-2")}>
                    <h4>{"Current Jobs:"}</h4>
                    {for self.jobs.iter().map(|job| html! {
                        <div class={classes!("row", "col-12", "p-2")}>
                            <p class={classes!("col-3")}>{"Job ID: "}{job.job.id}</p>
                            <p class={classes!("col-3")}>{"Status: "}{&job.job.status}</p>
                            <div class={classes!("col-6", "progress")}>
                                <div class={classes!("progress-bar")} role="progressbar" 
                                    style={format!("width: {}%", job.job.progress * 100.0)} 
                                    aria-valuenow={job.job.progress.to_string()} aria-valuemin="0" aria-valuemax="100">
                                    {format!("{:.0}%", job.job.progress * 100.0)}
                                </div>
                            </div>
                        </div>
                    })}
                </div>
            </div>
        }
    }
}
