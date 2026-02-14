use sqlx::{postgres::PgPoolOptions, Error, FromRow, PgPool};

use crate::relational::query::QueryBuilder;

use super::{DBType, Database};

#[derive(Debug)]
pub struct Postgres(PgPool);

// Explicitly implement Sync for Postgres
// PgPool is Sync, so Postgres is Sync
unsafe impl Sync for Postgres {}

impl Postgres {
    pub fn query_builder<'a>(
        &self,
        query_builder: &'a crate::relational::query::QueryBuilder<sqlx::Postgres>,
    ) -> sqlx::QueryBuilder<'a, sqlx::Postgres> {
        let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(query_builder.query());

        for (sql, arg) in query_builder.build().iter() {
            qb.push(sql);

            if let Some(bindable) = arg {
                bindable.bind_to(&mut qb);
            }
        }
        
        println!("Built SQL Query: {}", qb.sql());
        qb
    }
}

#[async_trait::async_trait]
impl Database for Postgres {
    type Backend = sqlx::Postgres;

    /// Opening backtick character to surround identifiers, such as column and table names.
    const C_BACKTICK_OPEN: &'static str = "\"";
    /// Closing backtick character to surround identifiers, such as column and table names.
    const C_BACKTICK_CLOSE: &'static str = "\"";
    /// Wildcard character to be used in `LIKE` queries.
    const C_WILDCARD: &'static str = "%";

    async fn new(connection_string: &str, pool_size: Option<u32>) -> Result<Self, Error> {
        let mut pool = PgPoolOptions::new();
        if let Some(size) = pool_size {
            pool = pool.max_connections(size);
        } else {
            pool = pool.max_connections(1);
        }

        Ok(Self(pool.connect(connection_string).await?))
    }

    fn client(&self) -> DBType<'_> {
        DBType::Postgres(&self.0)
    }

    async fn fetch_all<T: Send + Unpin + for<'r> FromRow<'r, sqlx::postgres::PgRow>>(
        &self,
        query_builder: &QueryBuilder<sqlx::Postgres>,
    ) -> Result<Vec<T>, Error> {
        self.query_builder(query_builder)
            .build_query_as::<T>()
            .fetch_all(&self.0)
            .await
    }

    async fn fetch_one<T: Send + Unpin + for<'r> FromRow<'r, sqlx::postgres::PgRow>>(
        &self,
        query_builder: &QueryBuilder<sqlx::Postgres>,
    ) -> Result<T, Error> {
        self.query_builder(query_builder)
            .build_query_as::<T>()
            .fetch_one(&self.0)
            .await
    }
}
