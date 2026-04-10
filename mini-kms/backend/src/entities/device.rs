use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "devices")] // 对应表名
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(unique)]
    pub serial: String,       // 序列号
    pub name: String,         // 设备名称
    pub status: String,       // 状态
    pub last_seen_at: Option<DateTime>, // 最近在线时间
    pub created_at: DateTime,  // 创建时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 稍后在密钥包模块会用到与 keypacks 的关联
}

impl ActiveModelBehavior for ActiveModel {}