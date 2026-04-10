use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "keypacks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub device_id: u32,       // 关联设备的 ID
    pub version: String,      // 版本号
    pub status: String,       // 状态
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}