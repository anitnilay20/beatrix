use crate::relational::{db::Database, query::QueryBuilder};

pub trait Sql {
    fn to_sql(&self) -> String;
}

pub trait SqlColumn<T: Database> {
    fn to_sql(&self, db: &T) -> String;
}

pub trait SqlTable<T: Database> {
    fn to_sql(&self, db: &T) -> String;
}

pub trait SqlSelect<T: Database> {
    fn to_sql(&self, db: &T) -> QueryBuilder<T::Backend>;
}

pub trait SqlFilter<T: Database> {
    fn to_sql(&self, db: &T, index: usize) -> String;
}
