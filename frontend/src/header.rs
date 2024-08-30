use yew::{function_component, html, Html};

#[function_component]
pub fn HeaderComponent() -> Html {
    html! {
        <nav class={"navbar navbar-expand-lg navbar-light bg-light"}>
            <div class={"container-fluid"}>

                <button class={"navbar-brand"} type="button" data-bs-toggle="offcanvas" data-bs-target="#offcanvasExample" aria-controls="offcanvasExample">
                    <i class="bi bi-list"></i>
                </button>

                <button class={"navbar-toggler"} type="button" data-bs-toggle="collapse" data-bs-target="#navbarMainMenu" aria-controls="navbarMainMenu" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>

                <div class="collapse navbar-collapse" id="navbarMainMenu">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <a class="nav-link active" aria-current="page" href="/">{"Home"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/timers">{"Timer"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/converter">{"Converter"}</a>
                        </li>

                        <li class="nav-item">
                            <a class="nav-link disabled" href="#" tabindex="-1" aria-disabled="true">{"Disabled"}</a>
                        </li>
                    </ul>

                    <form class="d-flex">
                        <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search" />
                        <button class="btn btn-outline-success" type="submit">{"Search"}</button>
                    </form>

                </div>
            </div>
        </nav>
    }
}
