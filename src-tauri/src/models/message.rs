use sea_orm::ActiveValue::Set;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub session_uuid: String,
    pub order: i32,
    pub content: String,
    pub itype: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// 保存消息
pub async fn save(db: &DatabaseConnection, message: Model) -> Result<Model, String> {
    let am = ActiveModel {
        id: NotSet,
        session_uuid: Set(message.session_uuid.clone()),
        order: Set(message.order),
        content: Set(message.content.clone()),
        itype: Set(message.itype),
    };
    let a: Model;

    a = am.insert(db).await.unwrap();

    Ok(a)
}
