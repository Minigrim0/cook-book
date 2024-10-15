use log::{error, info};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlButtonElement};
use yew::prelude::*;
use yew::MouseEvent;
use yew_router::prelude::*;

use crate::components::PaginatedNavbar;
use crate::routes::AdminRoute;

use models::models::Ingredient;
use models::PaginatedIngredients;

const ITEMS_PER_PAGE: usize = 20;

pub struct AdminIngredientList {
    ingredients: Vec<Ingredient>,
    current_page: usize,
    total_ingredients: usize,
}

pub enum Msg {
    LoadIngredients(usize),
    IngredientsLoaded(Vec<Ingredient>, usize),
    UpdateIngredient(usize, String),
    PageDown,
    PageUp,
    GoToPage(MouseEvent),
}

impl Component for AdminIngredientList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadIngredients(1));
        Self {
            ingredients: Vec::new(),
            current_page: 1,
            total_ingredients: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadIngredients(page) => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match crate::glue::filter_ingredients(
                        String::new(),
                        ITEMS_PER_PAGE as i32,
                        ((page - 1) * ITEMS_PER_PAGE) as i32,
                    )
                    .await
                    {
                        Ok(paginated_ingredients) => {
                            let data: Result<PaginatedIngredients, String> =
                                serde_wasm_bindgen::from_value(paginated_ingredients)
                                    .map_err(|e| e.to_string());

                            match data {
                                Ok(result) => {
                                    link.send_message(Msg::IngredientsLoaded(result.0, result.1))
                                }
                                Err(e) => log::error!("Unable to convert JsValue, {}", e),
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to load ingredients: {:?}", e);
                        }
                    }
                });
                false
            }
            Msg::IngredientsLoaded(ingredients, total) => {
                self.ingredients = ingredients;
                self.total_ingredients = total;
                true
            }
            Msg::UpdateIngredient(index, new_name) => {
                if let Some(ingredient) = self.ingredients.get_mut(index) {
                    ingredient.name = new_name;
                    // Here you would typically call a service function to update the ingredient in the backend
                    // For now, we'll just update it locally
                }
                true
            }
            Msg::PageUp | Msg::PageDown | Msg::GoToPage(_) => {
                self.current_page = match msg {
                    Msg::PageUp => self.current_page + 1,
                    Msg::PageDown => self.current_page - 1,
                    Msg::GoToPage(e) => {
                        // Get the page number from the button's value field
                        let button_opt = e.target().map(|t| {
                            t.dyn_into::<HtmlButtonElement>().map(|b| {
                                b.value()
                                    .parse::<usize>()
                                    .map_err(|e| {
                                        error!(
                                            "Unable to parse value from button: {}",
                                            e.to_string()
                                        )
                                    })
                                    .unwrap_or(0)
                            })
                        });

                        if let Some(button) = button_opt {
                            match button {
                                Ok(value) => {
                                    info!("Go to page: {}", value);
                                    value
                                }
                                Err(e) => {
                                    error!("Unable to get button value; {}", e.to_string());
                                    0
                                }
                            }
                        } else {
                            error!("Unable to get an event target");
                            0
                        }
                    }
                    _ => {
                        error!("Invalid state");
                        0
                    }
                };

                ctx.link()
                    .send_message(Msg::LoadIngredients(self.current_page));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let previous_cb = ctx.link().callback(|_| Msg::PageDown);
        let next_cb = ctx.link().callback(|_| Msg::PageUp);
        let button_cb = ctx.link().callback(Msg::GoToPage);

        html! {
            <div class="ingredient-list">
                <h2>{"Ingredients"}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{"ID"}</th>
                            <th>{"Name"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {self.ingredients.iter().map(|ingredient| {
                            html! {
                                <tr key={ingredient.id}>
                                    <td>
                                        <Link<AdminRoute> to={AdminRoute::IngredientsDetail { id: ingredient.id }}>
                                            {ingredient.id}
                                        </Link<AdminRoute>>
                                    </td>
                                    <td>
                                        {ingredient.name.clone()}
                                    </td>
                                </tr>
                            }
                        }).collect::<Html>()}
                    </tbody>
                </table>
                <PaginatedNavbar
                    previous_callback={previous_cb.clone()}
                    next_callback={next_cb.clone()}
                    number_callback={button_cb.clone()}
                    current_page={self.current_page}
                    num_pages={self.num_pages}
                />
            </div>
        }
    }
}
