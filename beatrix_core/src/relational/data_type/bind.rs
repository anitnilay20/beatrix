use sqlx::{Database, Encode, Type};
// use super::Value;

// 1. The Trait definition
pub trait BindValue<DB: Database>: Send + Sync {
    fn bind_to<'args>(&'args self, builder: &mut sqlx::QueryBuilder<'args, DB>);
    fn box_clone(&self) -> Box<dyn BindValue<DB>>;
}

impl<DB: Database> Clone for Box<dyn BindValue<DB>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl<T, DB> BindValue<DB> for T
where
    DB: Database,
    T: for<'q> Encode<'q, DB> + Type<DB>,
    T: Send + Sync + Clone + 'static,
{
    fn bind_to<'args>(&'args self, builder: &mut sqlx::QueryBuilder<'args, DB>) {
        builder.push_bind(self.clone());
    }

    fn box_clone(&self) -> Box<dyn BindValue<DB>> {
        Box::new(self.clone())
    }
}
