// 1. Import the macro using the crate name
use beatrix_macro::RelationalModel;

// 2. Apply it to your struct
#[derive(RelationalModel)]
#[table_name = "user"]
struct UserInsert {
    id: i32,
    #[name = "custom_name"]
    name: String,
}

// You need a main function because macrotest compiles this file as a standalone script
fn main() {}