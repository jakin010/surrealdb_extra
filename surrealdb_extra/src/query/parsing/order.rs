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

#[derive(Debug, Clone)]
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

        let mut order = Order::default();
        order.order = idiom.0;
        order.random = false;
        order.collate = false;
        order.numeric = false;
        order.direction = direction;

        Self(order)
    }
}

impl From<(String, OrderDirection)> for ExtraOrder {
    fn from(value: (String, OrderDirection)) -> Self {
        let idiom = ExtraIdiom::from(value.0);
        let direction = value.1.to_bool();

        let mut order = Order::default();
        order.order = idiom.0;
        order.random = false;
        order.collate = false;
        order.numeric = false;
        order.direction = direction;

        Self(order)
    }
}

impl From<(Idiom, OrderDirection)> for ExtraOrder {
    fn from(value: (Idiom, OrderDirection)) -> Self {
        let direction = value.1.to_bool();

        let mut order = Order::default();
        order.order = value.0;
        order.random = false;
        order.collate = false;
        order.numeric = false;
        order.direction = direction;

        Self(order)
    }
}
