use yew::{function_component, html, Html};

#[function_component(ActiveTimersDropdown)]
pub fn active_timers_dropdown() -> Html {
    html! {
        <div class="dropdown">
            <button class="btn btn-outline-secondary dropdown-toggle" type="button" id="activeTimersDropdown" data-bs-toggle="dropdown" aria-expanded="false">
                <i class="bi bi-clock me-2"></i>
                {"Active Timers"}
            </button>
            <ul class="dropdown-menu" aria-labelledby="activeTimersDropdown">
                <li>
                    <div class="dropdown-item">
                        <div class="d-flex justify-content-between align-items-center">
                            <span>{"Pasta"}</span>
                            <span>{"8:00"}</span>
                        </div>
                        <div class="d-flex align-items-center mt-2">
                            <div class="progress flex-grow-1" style="height: 5px;">
                                <div class="progress-bar" role="progressbar" style="width: 75%;" aria-valuenow="75" aria-valuemin="0" aria-valuemax="100"></div>
                            </div>
                            <div class="ms-2">
                                <button class="btn btn-sm btn-outline-secondary p-0" style="width: 20px; height: 20px;"><i class="bi bi-pause-fill"></i></button>
                                <button class="btn btn-sm btn-outline-secondary p-0 ms-1" style="width: 20px; height: 20px;"><i class="bi bi-stop-fill"></i></button>
                                <button class="btn btn-sm btn-outline-danger p-0 ms-1" style="width: 20px; height: 20px;"><i class="bi bi-x"></i></button>
                            </div>
                        </div>
                    </div>
                </li>
                <li>
                    <div class="dropdown-item">
                        <div class="d-flex justify-content-between align-items-center">
                            <span>{"Rice"}</span>
                            <span>{"15:00"}</span>
                        </div>
                        <div class="d-flex align-items-center mt-2">
                            <div class="progress flex-grow-1" style="height: 5px;">
                                <div class="progress-bar" role="progressbar" style="width: 50%;" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                            </div>
                            <div class="ms-2">
                                <button class="btn btn-sm btn-outline-secondary p-0" style="width: 20px; height: 20px;"><i class="bi bi-pause-fill"></i></button>
                                <button class="btn btn-sm btn-outline-secondary p-0 ms-1" style="width: 20px; height: 20px;"><i class="bi bi-stop-fill"></i></button>
                                <button class="btn btn-sm btn-outline-danger p-0 ms-1" style="width: 20px; height: 20px;"><i class="bi bi-x"></i></button>
                            </div>
                        </div>
                    </div>
                </li>
                <li>
                    <div class="dropdown-item">
                        <div class="d-flex justify-content-between align-items-center">
                            <span>{"Chicken"}</span>
                            <span>{"25:00"}</span>
                        </div>
                        <div class="d-flex align-items-center mt-2">
                            <div class="progress flex-grow-1" style="height: 5px;">
                                <div class="progress-bar" role="progressbar" style="width: 25%;" aria-valuenow="25" aria-valuemin="0" aria-valuemax="100"></div>
                            </div>
                            <div class="ms-2">
                                <button class="btn btn-sm btn-outline-secondary p-0" style="width: 20px; height: 20px;"><i class="bi bi-pause-fill"></i></button>
                                <button class="btn btn-sm btn-outline-secondary p-0 ms-1" style="width: 20px; height: 20px;"><i class="bi bi-stop-fill"></i></button>
                                <button class="btn btn-sm btn-outline-danger p-0 ms-1" style="width: 20px; height: 20px;"><i class="bi bi-x"></i></button>
                            </div>
                        </div>
                    </div>
                </li>
            </ul>
        </div>
    }
}