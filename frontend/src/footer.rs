use yew::{function_component, html, Html};

#[function_component]
pub fn FooterComponent() -> Html {
    html! {
        <footer class="d-flex flex-wrap justify-content-between align-items-center p-3 my-0 border-top position-absolute w-100 bottom-0">
            <div class="col-md-4 d-flex align-items-center">
                <a href="/" class="mb-3 me-2 mb-md-0 text-muted text-decoration-none lh-1">
                    <i class="bi bi-book"></i>
                </a>
                <span class="text-muted">{"Version XXXX"}</span>
            </div>

            <ul class="nav col-md-4 justify-content-end list-unstyled d-flex">
                <li class="ms-3">
                    <a class="text-muted" href="#">
                        <i class="bi bi-github"></i>
                    </a>
                </li>
            </ul>
        </footer>
    }
}
