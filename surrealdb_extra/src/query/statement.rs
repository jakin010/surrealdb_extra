use std::marker::PhantomData;
use surrealdb::{Connection, Surreal};
use crate::query::create::CreateBuilder;
use crate::query::relate::RelateBuilder;
use crate::query::select::SelectBuilder;
use crate::query::states::{NoCond, NoData, NoFields, NoRelation, NoWhat};
use crate::query::update::UpdateBuilder;

#[deprecated(since="0.10.0", note="please use `(Builder)::new()` instead")]
pub trait StatementBuilder {
    #[deprecated(since="0.10.0", note="please use `SelectBuilder::new()` instead")]
    fn select_builder(&self) -> SelectBuilder<NoWhat, NoFields, NoCond>;
    #[deprecated(since="0.10.0", note="please use `UpdateBuilder::new()` instead")]
    fn update_builder(&self) -> UpdateBuilder<NoWhat, NoData, NoCond>;
    #[deprecated(since="0.10.0", note="please use `RelateBuilder::new()` instead")]
    fn relate_builder(&self) -> RelateBuilder<NoRelation, NoData>;
    #[deprecated(since="0.10.0", note="please use `CreateBuilder::new()` instead")]
    fn create_builder(&self) -> CreateBuilder<NoWhat, NoData>;
}

impl<Client: Connection> StatementBuilder for Surreal<Client>
    where Client: Connection
{
    fn select_builder(&self) -> SelectBuilder<NoWhat, NoFields, NoCond> {
        SelectBuilder {
            statement: Default::default(),
            what_state: PhantomData,
            fields_state: PhantomData,
            cond_state: PhantomData,
        }
    }

    fn update_builder(&self) -> UpdateBuilder<NoWhat, NoData, NoCond> {
        UpdateBuilder {
            statement: Default::default(),
            what_state: PhantomData,
            data_state: PhantomData,
            cond_state: PhantomData,
        }
    }

    fn relate_builder(&self) -> RelateBuilder<NoRelation, NoData> {
        RelateBuilder {
            statement: Default::default(),
            relate_state: PhantomData,
            data_state: PhantomData,
        }
    }

    fn create_builder(&self) -> CreateBuilder<NoWhat, NoData> {
        CreateBuilder {
            statement: Default::default(),
            what_state: PhantomData,
            data_state: PhantomData,
        }
    }
}
