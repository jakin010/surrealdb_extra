use surrealdb::sql::{Idiom, Order, OrderList, Ordering};
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
pub struct ExtraOrdering(pub Ordering);

#[derive(Debug, Clone)]
pub struct ExtraOrder(pub Order);

impl From<Ordering> for ExtraOrdering {
    fn from(value: Ordering) -> Self {
        Self(value)
    }
}

impl From<OrderList> for ExtraOrdering {
    fn from(value: OrderList) -> Self {
        Self(Ordering::Order(value))
    }
}

impl From<Vec<Order>> for ExtraOrdering {
    fn from(value: Vec<Order>) -> Self {
        let mut list = OrderList::default();
        list.0 = value;

        Self(Ordering::Order(list))
    }
}

impl From<Vec<ExtraOrder>> for ExtraOrdering {
    fn from(value: Vec<ExtraOrder>) -> Self {
        let value = value.into_iter().map(|x| x.0).collect();
        let mut list = OrderList::default();
        list.0 = value;

        Self(Ordering::Order(list))
    }
}

impl From<(&str, OrderDirection)> for ExtraOrder {
    fn from(value: (&str, OrderDirection)) -> Self {
        let idiom = ExtraIdiom::from(value.0);
        let direction = value.1.to_bool();

        let mut order = Order::default();
        order.value = idiom.0;
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
        order.value = idiom.0;
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
        order.value = value.0;
        order.collate = false;
        order.numeric = false;
        order.direction = direction;

        Self(order)
    }
}

#[macro_export]
macro_rules! order_vec {
    ($($x:expr),+ $(,)?) => [
        $crate::query::parsing::order::ExtraOrdering::from(
            std::vec::Vec::<$crate::query::parsing::order::ExtraOrder>::from([$($crate::query::parsing::order::ExtraOrder::from($x)),+])
        )
    ];
}