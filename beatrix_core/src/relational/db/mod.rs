use sqlx::{Error, FromRow, PgPool};

use crate::relational::query::QueryBuilder;

#[cfg(feature = "postgres")]
pub(crate) mod postgres;

pub enum DBType<'a> {
    Postgres(&'a PgPool),
}


#[async_trait::async_trait]
pub trait Database: Sync
{
    type Backend: sqlx::Database;
    /// Opening backtick character to surround identifiers, such as column and table names.
    const C_BACKTICK_OPEN: &'static str;
    /// Closing backtick character to surround identifiers, such as column and table names.
    const C_BACKTICK_CLOSE: &'static str;
    /// Wildcard character to be used in `LIKE` queries.
    const C_WILDCARD: &'static str;

    fn backtick_open(&self) -> &'static str {
        Self::C_BACKTICK_OPEN
    }

    fn backtick_close(&self) -> &'static str {
        Self::C_BACKTICK_CLOSE
    }

    fn wildcard(&self) -> &'static str {
        Self::C_WILDCARD
    }

    async fn new(connection_string: &str, pool_size: Option<u32>) -> Result<Self, Error>
    where
        Self: Sized;

    fn client(&self) -> DBType<'_>;
    
    async fn fetch_all<T>(
        &self,
        query_builder: &QueryBuilder<Self::Backend>,
    ) -> Result<Vec<T>, sqlx::Error>
    where
    T: Send + Unpin,
    T: for<'r> FromRow<'r, <Self::Backend as sqlx::Database>::Row>;

    async fn fetch_one<T>(
        &self,
        query_builder: &QueryBuilder<Self::Backend>,
    ) -> Result<T, sqlx::Error>
    where
    T: Send + Unpin,
    T: for<'r> FromRow<'r, <Self::Backend as sqlx::Database>::Row>;
}
