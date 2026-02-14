use std::marker::PhantomData;

use super::{
    db::Database,
    filters::{Expression, Filters},
};
use crate::relational::sql::SqlColumn;
use crate::{impl_column_for_characters, impl_comparison_for_column};

pub trait ColumnType: Sync + Send {
    fn table_name(&self) -> &'static str;
    fn column_name(&self) -> &'static str;
    fn alias(&self) -> String;
}

impl<T: ?Sized + ColumnType, DB: Database> SqlColumn<DB> for T {
    fn to_sql(&self, db: &DB) -> String {
        format!(
            "{}{}{} as {}{}{}",
            db.backtick_open(),
            self.column_name(),
            db.backtick_close(),
            db.backtick_open(),
            self.alias(),
            db.backtick_close()
        )
    }
}
impl<DB: Database> SqlColumn<DB> for Box<dyn ColumnType> {
    fn to_sql(&self, db: &DB) -> String {
        format!(
            "{}{}{} as {}{}{}\n",
            db.backtick_open(),
            self.column_name(),
            db.backtick_close(),
            db.backtick_open(),
            self.alias(),
            db.backtick_close()
        )
    }
}

impl<DB: Database> SqlColumn<DB> for Vec<Box<dyn ColumnType>> {
    fn to_sql(&self, db: &DB) -> String {
        self.iter()
            .map(|f| f.to_sql(db))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
impl<T: ColumnType + 'static, DB: Database> SqlColumn<DB> for Vec<Box<T>> {
    fn to_sql(&self, db: &DB) -> String {
        self.iter()
            .map(|f| f.to_sql(db))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[derive(Clone)]
pub struct Column<T> {
    pub table_name: &'static str,
    pub column_name: &'static str,
    pub alias: Option<String>,
    pub _marker: PhantomData<T>,
}

impl<T> Column<T> {
    pub fn new(table_name: &'static str, column_name: &'static str, alias: Option<String>) -> Self {
        Self {
            table_name,
            column_name,
            alias,
            _marker: PhantomData,
        }
    }

    pub fn column_name(&self) -> &str {
        self.column_name
    }

    pub fn alias(&self) -> &str {
        if let Some(alias) = &self.alias {
            alias
        } else {
            self.column_name()
        }
    }

    pub fn r#as(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }
}

impl<T> ColumnType for Column<T>
where
    T: Sync + Send,
{
    fn table_name(&self) -> &'static str {
        self.table_name
    }

    fn column_name(&self) -> &'static str {
        self.column_name
    }

    fn alias(&self) -> String {
        if let Some(alias) = &self.alias {
            alias.clone()
        } else {
            self.column_name().to_string()
        }
    }
}

impl<T> Column<T>
where
    T: Send + Sync + 'static + Clone,
{
    pub fn eq(self, other: T) -> Filters<T, T> {
        Filters {
            column: self,
            not: false,
            value: other,
            expression: Expression::Eq,
        }
    }

    pub fn neq(self, other: T) -> Filters<T, T> {
        Filters {
            column: self,
            not: true,
            value: other,
            expression: Expression::Neq,
        }
    }
}

impl_comparison_for_column!(i8, i16, i32, i64, f32, f64);
impl_column_for_characters!(char, String);
