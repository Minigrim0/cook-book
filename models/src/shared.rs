use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
/// Include metadata on recipes, amount of recipe, amout of cuisines, ...
pub struct RecipeMeta {
    pub recipe_amount: i32,
    pub cuisine_amout: i32,
    pub ingredients_amount: i32,
}
