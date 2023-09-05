use surrealdb::sql::{Idiom, Order};
use crate::query::parsing::idiom::ExtraIdiom;

pub enum OrderDirection {
    ASC,
    DESC
}

impl OrderDirection {
    pub fn to_bool(&self) -> bool {
        match self {
            Self::ASC => true,
            Self::DESC => false
        }
    }
}

pub struct ExtraOrder(pub Order);

impl From<Order> for ExtraOrder {
    fn from(value: Order) -> Self {
        Self(value)
    }
}

impl From<(&str, OrderDirection)> for ExtraOrder {
    fn from(value: (&str, OrderDirection)) -> Self {
        let idiom = ExtraIdiom::from(value.0);
        let direction = value.1.to_bool();

        let order = Order {
            order: idiom.0,
            random: false,
            collate: false,
            numeric: false,
            direction,
        };

        Self(order)
    }
}

impl From<(String, OrderDirection)> for ExtraOrder {
    fn from(value: (String, OrderDirection)) -> Self {
        let idiom = ExtraIdiom::from(value.0);
        let direction = value.1.to_bool();

        let order = Order {
            order: idiom.0,
            random: false,
            collate: false,
            numeric: false,
            direction,
        };

        Self(order)
    }
}

impl From<(Idiom, OrderDirection)> for ExtraOrder {
    fn from(value: (Idiom, OrderDirection)) -> Self {
        let direction = value.1.to_bool();

        let order = Order {
            order: value.0,
            random: false,
            collate: false,
            numeric: false,
            direction,
        };

        Self(order)
    }
}
