use std::cmp::{max, min};
use yew::{classes, function_component, html, Html, Properties};
use yew::{Callback, MouseEvent};

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Callback for the button "previous"
    pub previous_callback: Callback<MouseEvent>,
    /// Callback for the button "next"
    pub next_callback: Callback<MouseEvent>,
    /// Callback for the numbered buttons
    pub number_callback: Callback<MouseEvent>,
    pub current_page: i32,
    pub num_pages: i32,
}

#[function_component]
pub fn PaginatedNavbar(props: &Props) -> Html {
    html! {
        <nav aria-label="Page navigation example">
            <ul class="pagination">
                <li class="page-item">
                    <button
                        class={classes!("page-link", if props.current_page > 0 {""} else {"disabled"})}
                        onclick={&props.previous_callback}
                    >
                        {"Previous"}
                    </button>
                </li>
                {
                    (max(0, props.current_page-3)..props.current_page)
                        .map(|index| {
                            html! {
                                <li class="page-item">
                                    <button
                                        class={classes!("page-link")}
                                        onclick={props.number_callback.clone()}
                                        value={index.to_string()}
                                    >
                                        {index + 1}
                                    </button>
                                </li>
                            }
                        })
                        .collect::<Html>()
                }
                <li class="page-item"><button class={classes!("page-link", "active")}>{props.current_page + 1}</button></li>
                {
                    ((props.current_page+1)..min(props.num_pages, props.current_page+4))
                        .map(|index| {
                            html! {
                                <li class="page-item">
                                    <button
                                        class={classes!("page-link")}
                                        onclick={props.number_callback.clone()}
                                        value={index.to_string()}
                                    >
                                        {index + 1}
                                    </button>
                                </li>
                            }
                        })
                        .collect::<Html>()
                }
                <li class="page-item">
                    <button
                        class={classes!("page-link", if props.current_page < props.num_pages - 1 {""} else {"disabled"})}
                        onclick={&props.next_callback}
                    >
                        {"Next"}
                    </button>
                </li>
            </ul>
        </nav>
    }
}
