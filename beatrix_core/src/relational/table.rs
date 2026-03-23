use crate::relational::{column::ColumnType, sql::SqlTable};

use super::{db::Database, helpers::format_name};

#[derive(Debug, Clone)]
pub struct TableDetails {
    pub name: String,
    pub alias: Option<String>,
}

impl TableDetails {
    pub fn table_name(&self) -> String {
        format_name(&self.name)
    }

    pub fn alias(&self) -> String {
        if self.alias.is_some() {
            self.alias.clone().unwrap()
        } else {
            self.table_name()
        }
    }

    pub fn r#as(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
}

pub trait Table {
    fn table_details() -> TableDetails;
    fn columns() -> Vec<Box<dyn ColumnType>>;
    fn select<DB: sqlx::Database>() -> crate::relational::select::Select<Self, DB>
    where
        Self: Sized;
}

impl<T: Database> SqlTable<T> for TableDetails {
    fn to_sql(&self, db: &T) -> String {
        format!(
            "{}{}{} as {}{}{}",
            db.backtick_open(),
            self.table_name(),
            db.backtick_close(),
            db.backtick_open(),
            self.alias(),
            db.backtick_close()
        )
    }
}

impl<T: Database> SqlTable<T> for Vec<TableDetails> {
    fn to_sql(&self, db: &T) -> String {
        self.iter()
            .map(|f| f.to_sql(db))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
