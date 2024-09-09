use log::error;
use yew::{classes, html, Component, Context, Html, Properties};

use models::CompleteRecipe;

use super::services::load_recipe;

pub struct RecipeDetailsPage {
    recipe: Option<CompleteRecipe>,
}

pub enum Msg {
    LoadRecipe,
    RecipeLoaded(Result<CompleteRecipe, String>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub recipe_id: i32,
}

impl Component for RecipeDetailsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadRecipe);

        RecipeDetailsPage { recipe: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadRecipe => {
                let callback = ctx.link().callback(Msg::RecipeLoaded);
                load_recipe(ctx.props().recipe_id, callback);
                false
            }
            Msg::RecipeLoaded(result) => {
                match result {
                    Ok(recipe) => {
                        self.recipe = Some(recipe);
                    }
                    Err(e) => error!("Backend: {}", e.to_string()),
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class={classes!("row", "p-2")}>
                    if let Some(recipe) = &self.recipe {
                        <h1>{&recipe.name}</h1>
                    } else {
                        <div class={classes!("position-absolute", "top-50", "start-50", "translate-middle", "text-muted", "text-center")}>
                            {"Unable to load :/"}
                        </div>
                    }
                </div>
            </div>
        }
    }
}