use beatrix_macro::RelationalModel;
#[table_name = "user"]
struct UserInsert {
    id: i32,
    #[name = "custom_name"]
    name: String,
}
use beatrix::relational::select::SelectType;
impl beatrix_core::relational::table::Table for UserInsert {
    type SelectBuilder = beatrix_core::relational::select::Select<UserInsert>;
    fn table_details() -> beatrix_core::relational::table::TableDetails {
        beatrix_core::relational::table::TableDetails {
            name: "user".to_string(),
            alias: None,
        }
    }
    fn columns() -> Vec<Box<dyn beatrix_core::relational::column::ColumnType>> {
        <[_]>::into_vec(
            ::alloc::boxed::box_new([
                Box::new(Self::id())
                    as Box<dyn beatrix_core::relational::column::ColumnType>,
                Box::new(Self::name())
                    as Box<dyn beatrix_core::relational::column::ColumnType>,
            ]),
        )
    }
    fn select() -> Self::SelectBuilder {
        beatrix_core::relational::select::Select::new(
            Self::columns(),
            <[_]>::into_vec(::alloc::boxed::box_new([Self::table_details()])),
        )
    }
}
impl UserInsert {
    pub fn id() -> beatrix_core::relational::column::Column<i32> {
        beatrix_core::relational::column::Column::new("user", "id", None)
    }
    pub fn name() -> beatrix_core::relational::column::Column<String> {
        beatrix_core::relational::column::Column::new("user", "custom_name", None)
    }
}
fn main() {}
