//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

use crate::database::ColoringType;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "coloring"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub idx: i64,
    pub txo_idx: i64,
    pub transfer_idx: i64,
    pub coloring_type: ColoringType,
    pub amount: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Idx,
    TxoIdx,
    TransferIdx,
    ColoringType,
    Amount,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Idx,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Transfer,
    Txo,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Idx => ColumnType::BigInteger.def(),
            Self::TxoIdx => ColumnType::BigInteger.def(),
            Self::TransferIdx => ColumnType::BigInteger.def(),
            Self::ColoringType => ColumnType::SmallInteger.def(),
            Self::Amount => ColumnType::String(None).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Transfer => Entity::belongs_to(super::transfer::Entity)
                .from(Column::TransferIdx)
                .to(super::transfer::Column::Idx)
                .into(),
            Self::Txo => Entity::belongs_to(super::txo::Entity)
                .from(Column::TxoIdx)
                .to(super::txo::Column::Idx)
                .into(),
        }
    }
}

impl Related<super::transfer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transfer.def()
    }
}

impl Related<super::txo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Txo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
