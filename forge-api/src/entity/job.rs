use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "jobs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub job_id: Uuid,
    pub job_type: String,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(nullable)]
    pub started_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(nullable)]
    pub finished_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", nullable)]
    pub error: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub payload: Option<Json>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub result: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}


