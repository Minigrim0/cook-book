mod ingredient_list;
mod services;

pub use ingredient_list::IngredientList;

mod recipe_list;
mod unit_list;

pub use recipe_list::AdminRecipeList;
pub use unit_list::AdminUnitList;

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum AdminPage {
    Ingredients,
    Recipes,
    Units,
}

/// Root component for the admin section
pub struct AdminRoot {
    current_page: AdminPage,
}

pub enum Msg {
    ChangePage(AdminPage),
}

impl Component for AdminRoot {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_page: AdminPage::Ingredients,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangePage(page) => {
                self.current_page = page;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="admin-root">
                <h1>{"Admin Panel"}</h1>
                <nav>
                    <ul>
                        <li>
                            <a onclick={ctx.link().callback(|_| Msg::ChangePage(AdminPage::Ingredients))}>
                                {"Ingredients"}
                            </a>
                        </li>
                        <li>
                            <a onclick={ctx.link().callback(|_| Msg::ChangePage(AdminPage::Recipes))}>
                                {"Recipes"}
                            </a>
                        </li>
                        <li>
                            <a onclick={ctx.link().callback(|_| Msg::ChangePage(AdminPage::Units))}>
                                {"Units"}
                            </a>
                        </li>
                    </ul>
                </nav>
                {self.render_current_page()}
            </div>
        }
    }
}

impl AdminRoot {
    fn render_current_page(&self) -> Html {
        match self.current_page {
            AdminPage::Ingredients => html! { <IngredientList /> },
            AdminPage::Recipes => html! { <AdminRecipeList /> },
            AdminPage::Units => html! { <AdminUnitList /> },
        }
    }
}
