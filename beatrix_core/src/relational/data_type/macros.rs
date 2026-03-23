#[macro_export]
macro_rules! impl_into_value {
    ($($t:ty => $variant:ident),*) => {
        $(
            impl $crate::relational::data_type::IntoValue for $t {
                fn into_value(self) -> $crate::relational::data_type::Value {
                    $crate::relational::data_type::Value::$variant(self)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_sql_for_character {
    ($($t:ty),*) => {
        $(
            impl $crate::relational::sql::Sql for $t {
                fn to_sql(&self) -> String {
                    format!("'{}'", self)
                }
            }
        )*
    };
}