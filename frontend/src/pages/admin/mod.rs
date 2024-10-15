mod ingredient_detail;
mod ingredient_list;
mod recipe_list;
mod services;
mod unit_list;

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::routes::AdminRoute;

pub use ingredient_detail::AdminIngredientDetail;
pub use ingredient_list::AdminIngredientList;
pub use recipe_list::AdminRecipeList;
pub use unit_list::AdminUnitList;

/// Root component for the admin section
pub struct AdminRoot;

impl Component for AdminRoot {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="admin-root">
                <h1>{"Admin Panel"}</h1>
                <nav>
                    <ul>
                        <li>
                            <Link<AdminRoute> to={AdminRoute::Ingredients}>
                                {"Ingredients"}
                            </Link<AdminRoute>>
                        </li>
                        <li>
                            <Link<AdminRoute> to={AdminRoute::Recipes}>
                                {"Recipes"}
                            </Link<AdminRoute>>
                        </li>
                        <li>
                            <Link<AdminRoute> to={AdminRoute::Units}>
                                {"Units"}
                            </Link<AdminRoute>>
                        </li>
                    </ul>
                </nav>
            </div>
        }
    }
}
