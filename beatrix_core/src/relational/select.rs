use crate::relational::{
    column::ColumnType, filters::FilterType, query::QueryBuilder, sql::{SqlColumn, SqlSelect, SqlTable}
};

use super::{db::Database, table::TableDetails};

#[async_trait::async_trait]
pub trait SelectType<Backend: sqlx::Database>: Send + Sync {
    fn new(columns: Vec<Box<dyn ColumnType>>, from: Vec<TableDetails>) -> Self
    where
        Self: Sized;
    fn columns(self, columns: Vec<Box<dyn ColumnType>>) -> Self;
    fn from(self, from: Vec<TableDetails>) -> Self;
    fn filter(self, filters: Box<dyn FilterType<Backend>>) -> Self;

    async fn fetch_all<A, DB>(&self, db: &DB) -> Result<Vec<A>, sqlx::Error>
    where
        A: Send + Unpin,
        DB: Database<Backend = Backend> + Sync,
        A: for<'r> sqlx::FromRow<'r, <Backend as sqlx::Database>::Row>;

    async fn fetch_one<A, DB>(&self, db: &DB) -> Result<A, sqlx::Error>
    where
        A: Send + Unpin,
        DB: Database<Backend = Backend> + Sync,
        A: for<'r> sqlx::FromRow<'r, <Backend as sqlx::Database>::Row>;
}

// Ensure T is Send + Sync if it ever holds data
pub struct Select<T, Backend: sqlx::Database> {
    pub(crate) columns: Vec<Box<dyn ColumnType>>,
    pub(crate) from: Vec<TableDetails>,
    pub(crate) filter: Option<Box<dyn FilterType<Backend>>>,
    // pub(crate) query_builder: QueryBuilder<Backend>,

    _marker: std::marker::PhantomData<T>,
}

impl<T, Db: Database> SqlSelect<Db> for Select<T, Db::Backend>
where
    Db::Backend: sqlx::Database,
{
    fn to_sql(&self, db: &Db) -> QueryBuilder<Db::Backend> {
        let query = format!(
            "select {} from {}",
            self.columns.to_sql(db),
            self.from.to_sql(db),
        );

        let mut query_builder = QueryBuilder::new(query);

        if let Some(filter) = &self.filter {
            query_builder.push_sql(" where ");
            filter.add_to_query(&mut query_builder);
        }

        // println!("{:?}", query_builder.);
        query_builder
    }
}

#[async_trait::async_trait]
impl<T: Send + Sync, Backend: sqlx::Database> SelectType<Backend> for Select<T, Backend> { // <-- Ensure the Model (T) is thread-safe
    fn new(columns: Vec<Box<dyn ColumnType>>, from: Vec<TableDetails>) -> Self
    where
        Self: Sized,
    {
        Self {
            columns,
            from,
            filter: None,
            // query_builder: QueryBuilder::new(String::new()),
            _marker: std::marker::PhantomData,
        }
    }

    fn columns(mut self, columns: Vec<Box<dyn ColumnType>>) -> Self {
        self.columns = columns;
        self
    }

    fn from(mut self, from: Vec<TableDetails>) -> Self {
        self.from = from;
        self
    }

    fn filter(mut self, filter: Box<dyn FilterType<Backend>>) -> Self {
        self.filter = Some(filter);
        self
    }

    async fn fetch_all<A, DB>(&self, db: &DB) -> Result<Vec<A>, sqlx::Error>
    where
        A: Send + Unpin,
        DB: Database<Backend = Backend> + Sync,
        A: for<'r> sqlx::FromRow<'r, <Backend as sqlx::Database>::Row>,
    {
        db.fetch_all::<A>(&self.to_sql(db)).await
    }

    async fn fetch_one<A, DB>(&self, db: &DB) -> Result<A, sqlx::Error>
    where
        A: Send + Unpin,
        DB: Database<Backend = Backend> + Sync,
        A: for<'r> sqlx::FromRow<'r, <Backend as sqlx::Database>::Row>,
    {
        db.fetch_one::<A>(&self.to_sql(db)).await
    }
}