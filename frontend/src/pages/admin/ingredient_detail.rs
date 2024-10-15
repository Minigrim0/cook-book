use yew::prelude::*;

pub struct AdminIngredientDetail;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

impl Component for AdminIngredientDetail {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // match msg {}
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {"Here an ingredient !"}
            </div>
        }
    }
}
