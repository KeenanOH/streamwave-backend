//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "translation_type")]
pub enum TranslationType {
    #[sea_orm(string_value = "dub")]
    Dub,
    #[sea_orm(string_value = "sub")]
    Sub,
}
