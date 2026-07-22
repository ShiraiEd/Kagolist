use rapina::schemars::{self, JsonSchema};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, JsonSchema, Validate)]
pub struct CreateItem {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(range(min = 1, max = 100))]
    pub quantity: i32,
    #[validate(range(min = 0, max = 100000000))]
    pub price: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub price: Option<i64>,
}
