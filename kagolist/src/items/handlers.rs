use rapina::database::{Db, DbError};
use rapina::prelude::*;
use rapina::sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

use crate::entity::Item;
use crate::entity::item::{ActiveModel, Model};

use super::dto::{CreateItem, UpdateItem};
use super::error::ItemError;

#[get("/items")]
#[errors(ItemError)]
pub async fn list_items(db: Db) -> Result<Json<Vec<Model>>> {
    let items = Item::find().all(db.conn()).await.map_err(DbError)?;
    Ok(Json(items))
}

#[get("/items/:id")]
#[errors(ItemError)]
pub async fn get_item(db: Db, id: Path<i32>) -> Result<Json<Model>> {
    let id = id.into_inner();
    let item = Item::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| Error::not_found(format!("Item {} not found", id)))?;
    Ok(Json(item))
}

#[post("/items")]
#[errors(ItemError)]
pub async fn create_item(db: Db, body: Validated<Json<CreateItem>>) -> Result<Json<Model>> {
    let input = body.into_inner();
    let item = ActiveModel {
        name: Set(input.name.clone()),
        quantity: Set(input.quantity),
        price: Set(input.price),
        ..Default::default()
    };
    let result = item.insert(db.conn()).await.map_err(DbError)?;
    Ok(Json(result))
}

#[put("/items/:id")]
#[errors(ItemError)]
pub async fn update_item(db: Db, id: Path<i32>, body: Json<UpdateItem>) -> Result<Json<Model>> {
    let id = id.into_inner();
    let item = Item::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(DbError)?
        .ok_or_else(|| Error::not_found(format!("Item {} not found", id)))?;

    let update = body.into_inner();
    let mut active: ActiveModel = item.into_active_model();
    if let Some(val) = update.name {
        active.name = Set(val);
    }
    if let Some(val) = update.quantity {
        active.quantity = Set(val);
    }
    if let Some(val) = update.price {
        active.price = Set(val);
    }

    let result = active.update(db.conn()).await.map_err(DbError)?;
    Ok(Json(result))
}

#[delete("/items/:id")]
#[errors(ItemError)]
pub async fn delete_item(db: Db, id: Path<i32>) -> Result<Json<serde_json::Value>> {
    let id = id.into_inner();
    let result = Item::delete_by_id(id)
        .exec(db.conn())
        .await
        .map_err(DbError)?;
    if result.rows_affected == 0 {
        return Err(Error::not_found(format!("Item {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "deleted": id })))
}
