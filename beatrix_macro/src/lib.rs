extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

// #[cfg(feature = "mongo")]
// mod mongo;

#[cfg(feature = "postgres")]
mod relational;

// #[cfg(feature = "mongo")]
// #[proc_macro_derive(MongoModel, attributes(entity))]
// pub fn derive_mongo_model(input: TokenStream) -> TokenStream {
//     mongo::impl_mongo(&parse_macro_input!(input as DeriveInput))
// }

#[cfg(feature = "postgres")]
#[proc_macro_derive(RelationalModel, attributes(table_name, name))]
pub fn derive_relational_model(input: TokenStream) -> TokenStream {
    relational::impl_relational_model(&parse_macro_input!(input as DeriveInput)).into()
}
