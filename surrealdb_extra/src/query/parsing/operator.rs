/// Usage: op!(||) = Operator::Or, op!(OR) = Operator::Or, op!(or) = Operator::Or
/// 
/// Special cases: 
///     - op!("∋") == Operator::Contain (for the rest of contain double qoutes are needed)
///     - op!(--) == Operator::Neg (Minus sign and Negative sign are normaly the same sign. Here not -- is Negative and - is Minus. 
///                                 Reason being macro rules pattern matching (Could not find a better solution suggestion are welcome))
/// 
/// For contain key words use double quotes with the symbol
#[macro_export]
macro_rules! op {
    (--) => { ::surrealdb::sql::Operator::Neg };
    (!) => { ::surrealdb::sql::Operator::Not };

    (||) => { ::surrealdb::sql::Operator::Or };
    (&&) => { ::surrealdb::sql::Operator::And };
    (?) => { ::surrealdb::sql::Operator::Tco };
    (??) => { ::surrealdb::sql::Operator::Nco };

    (+) => { ::surrealdb::sql::Operator::Add };
    (-) => { ::surrealdb::sql::Operator::Sub };
    (*) => { ::surrealdb::sql::Operator::Mul };
    (/) => { ::surrealdb::sql::Operator::Div };
    (**) => { ::surrealdb::sql::Operator::Pow };
    (+=) => { ::surrealdb::sql::Operator::Inc };
    (-=) => { ::surrealdb::sql::Operator::Dec };
    (+?=) => { ::surrealdb::sql::Operator::Ext };
    
    (=) => { ::surrealdb::sql::Operator::Equal };
    (==) => { ::surrealdb::sql::Operator::Exact };
    (!=) => { ::surrealdb::sql::Operator::NotEqual };
    (*=) => { ::surrealdb::sql::Operator::AllEqual };
    (?=) => { ::surrealdb::sql::Operator::AnyEqual };

    (~) => { ::surrealdb::sql::Operator::Like };
    (!~) => { ::surrealdb::sql::Operator::NotLike };
    (*~) => { ::surrealdb::sql::Operator::AllLike };
    (?~) => { ::surrealdb::sql::Operator::AnyLike };
    (@$x:tt@) => { ::surrealdb::sql::Operator::Matches(Some($x)) };
    (@@) => { ::surrealdb::sql::Operator::Matches(None) };

    (<) => { ::surrealdb::sql::Operator::LessThan };
    (<=) => { ::surrealdb::sql::Operator::LessThanOrEqual };
    (>) => { ::surrealdb::sql::Operator::MoreThan };
    (>=) => { ::surrealdb::sql::Operator::MoreThanOrEqual };

    ("∋") => { ::surrealdb::sql::Operator::Contain };
    ("∌") => { ::surrealdb::sql::Operator::NotContain };
    ("⊇") => { ::surrealdb::sql::Operator::ContainAll };
    ("⊃") => { ::surrealdb::sql::Operator::ContainAny };
    ("⊅") => { ::surrealdb::sql::Operator::ContainNone };
    ("∈") => { ::surrealdb::sql::Operator::Inside };
    ("∉") => { ::surrealdb::sql::Operator::NotInside };
    ("⊆") => { ::surrealdb::sql::Operator::AllInside };
    ("⊂") => { ::surrealdb::sql::Operator::AnyInside };
    ("⊄") => { ::surrealdb::sql::Operator::NoneInside };

    (<$x:tt>) => { ::surrealdb::sql::Operator::Knn($x) };

    ($x:tt) => { 
        ::paste::item!(
            ::surrealdb::sql::Operator::[<$x:camel>]
        )
     };
}

#[cfg(test)]
mod test {
    use surrealdb::sql::Operator;

    #[test]
    fn op_and() {
        assert_eq!(op!(AND), Operator::And);
        assert_eq!(op!(and), Operator::And);
        assert_eq!(op!(&&), Operator::And);
    }

    #[test]
    fn op_contain() {
        assert_eq!(op!(Contain), Operator::Contain);
        assert_eq!(op!(contain), Operator::Contain);
        assert_eq!(op!("∋"), Operator::Contain);
    }

    #[test]
    fn op_matches() {
        assert_eq!(op!(@@), Operator::Matches(None));
        assert_eq!(op!(@88@), Operator::Matches(Some(88)));
    }

    #[test]
    fn op_knn() {
        assert_eq!(op!(<3>), Operator::Knn(3));
    }
}