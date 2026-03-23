use std::ops::Not;

use crate::relational::{column::Column, query::QueryBuilder};

use super::sql::Sql;
pub trait FilterType<DB: sqlx::Database>: Sync + Send {
    fn add_to_query(&self, query_builder: &mut QueryBuilder<DB>);
    fn and(self: Box<Self>, and: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>>;
    fn or(self: Box<Self>, or: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>>;
}

// --- 2. Expressions ---
pub enum Expression {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    NotLike,
    Between,
}

impl Sql for Expression {
    fn to_sql(&self) -> String {
        match self {
            Expression::Eq => "=".to_string(),
            Expression::Neq => "!=".to_string(),
            Expression::Gt => ">".to_string(),
            Expression::Gte => ">=".to_string(),
            Expression::Lt => "<".to_string(),
            Expression::Lte => "<=".to_string(),
            Expression::Like => "LIKE".to_string(),
            Expression::NotLike => "NOT LIKE".to_string(),
            Expression::Between => "BETWEEN".to_string(),
        }
    }
}

// --- 3. The Leaf Node (Condition) ---
pub struct Filters<T, V> {
    pub column: Column<T>,
    pub expression: Expression,
    pub value: V,
    pub not: bool,
}

impl<T, V, DB> FilterType<DB> for Filters<T, V>
where
    T: Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    DB: sqlx::Database,
    V: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
{
    fn and(self: Box<Self>, and: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>> {
        // Upcast Self to Box<dyn FilterType> and put it in an AND branch
        Box::new(LogicalOperator::And(vec![self as Box<dyn FilterType<DB>>, and]))
    }

    fn or(self: Box<Self>, or: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>> {
        Box::new(LogicalOperator::Or(vec![self as Box<dyn FilterType<DB>>, or]))
    }

    fn add_to_query(&self, query_builder: &mut QueryBuilder<DB>) {
        let not_str = if self.not { "NOT " } else { "" };

        let col_name = self.column.alias();
        query_builder.push(
            &format!("{}{} {} ", not_str, col_name, self.expression.to_sql()),
            Some(self.value.clone()),
        );
    }
}

// Convenience methods on Filters directly (without boxing)
impl<T, V> Filters<T, V>
where
    T: Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn boxed<DB>(self) -> Box<dyn FilterType<DB>>
    where
        DB: sqlx::Database,
        V: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    {
        Box::new(self)
    }

    pub fn and<DB>(self, and: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>>
    where
        DB: sqlx::Database,
        V: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    {
        Box::new(self).and(and)
    }

    pub fn or<DB>(self, or: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>>
    where
        DB: sqlx::Database,
        V: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    {
        Box::new(self).or(or)
    }
}

// Allow automatic conversion from Filters<T, V> to Box<dyn FilterType<DB>>
impl<T, V, DB> From<Filters<T, V>> for Box<dyn FilterType<DB>>
where
    T: Send + Sync + 'static,
    V: Clone + Send + Sync + 'static + for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    DB: sqlx::Database,
{
    fn from(f: Filters<T, V>) -> Self {
        Box::new(f)
    }
}

impl<T: Send + Sync, V> Not for Filters<T, V> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.not = !self.not;
        self
    }
}

// --- 4. The Branch Node (AND / OR) ---
pub enum LogicalOperator<DB: sqlx::Database> {
    And(Vec<Box<dyn FilterType<DB>>>),
    Or(Vec<Box<dyn FilterType<DB>>>),
}

impl<DB: sqlx::Database> FilterType<DB> for LogicalOperator<DB> {
    fn add_to_query(&self, builder: &mut QueryBuilder<DB>) {
        match self {
            LogicalOperator::And(filters) => {
                builder.push_sql("(");
                for (i, filter) in filters.iter().enumerate() {
                    if i > 0 {
                        builder.push_sql(" AND ");
                    }
                    // Recursively call children to push their SQL and bindings
                    filter.add_to_query(builder);
                }
                builder.push_sql(")");
            }
            LogicalOperator::Or(filters) => {
                builder.push_sql("(");
                for (i, filter) in filters.iter().enumerate() {
                    if i > 0 {
                        builder.push_sql(" OR ");
                    }
                    filter.add_to_query(builder);
                }
                builder.push_sql(")");
            }
        }
    }

    fn and(mut self: Box<Self>, and: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>> {
        if let LogicalOperator::And(ref mut filters) = *self {
            filters.push(and);
            self
        } else {
            Box::new(LogicalOperator::And(vec![self as Box<dyn FilterType<DB>>, and]))
        }
    }

    fn or(mut self: Box<Self>, or: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>> {
        if let LogicalOperator::Or(ref mut filters) = *self {
            filters.push(or);
            self
        } else {
            Box::new(LogicalOperator::Or(vec![self as Box<dyn FilterType<DB>>, or]))
        }
    }
}

pub struct BetweenFilter<T, V> {
    pub column: Column<T>,
    pub lower: V,
    pub upper: V,
    pub not: bool,
}

impl<T, V, DB> FilterType<DB> for BetweenFilter<T, V>
where
    T: Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    DB: sqlx::Database,
    V: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
{
    fn and(self: Box<Self>, and: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>>
    where
        DB: sqlx::Database,
        V: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    {
        Box::new(LogicalOperator::And(vec![self as Box<dyn FilterType<DB>>, and]))
    }

    fn or(self: Box<Self>, or: Box<dyn FilterType<DB>>) -> Box<dyn FilterType<DB>>
    where
        DB: sqlx::Database,
        V: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    {
        Box::new(LogicalOperator::Or(vec![self as Box<dyn FilterType<DB>>, or]))
    }

    fn add_to_query(&self, query_builder: &mut QueryBuilder<DB>) {
        let not_str = if self.not { " NOT" } else { "" };
        let col_name = self.column.alias();
        query_builder.push(
            &format!("{}{} BETWEEN ", col_name, not_str),
            Some(self.lower.clone()),
        );
        query_builder.push(
            " AND ",
            Some(self.upper.clone()),
        );
    }
}

impl<T, V, DB> From<BetweenFilter<T, V>> for Box<dyn FilterType<DB>>
where
    T: Send + Sync + 'static,
    V: Clone + Send + Sync + 'static + for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    DB: sqlx::Database,
{
    fn from(f: BetweenFilter<T, V>) -> Self {
        Box::new(f)
    }
}

impl<T, V> Not for BetweenFilter<T, V> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.not = !self.not;
        self
    }
}
