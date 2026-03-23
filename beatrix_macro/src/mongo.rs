use proc_macro::TokenStream;
use quote::quote;

use syn::{Lit, Meta, MetaList, MetaNameValue, NestedMeta};

pub fn impl_mongo(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let mut entity_name = struct_name.to_string().to_lowercase();

    for option in ast.attrs.clone().into_iter() {
        let option = option.parse_meta().unwrap();
        if let Meta::List(MetaList { path, nested, .. }) = option {
            if *path.get_ident().unwrap() == "entity" {
                nested.iter().for_each(|field| {
                    if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) =
                        field
                    {
                        if *path.get_ident().unwrap() == "name" {
                            if let Lit::Str(lit) = lit {
                                entity_name = lit.value();
                            }
                        }
                    }
                });
            }
        }
    }

    let gen = quote! {
        impl #struct_name {
            fn entity_name() -> String {
                format!("{}", #entity_name).to_lowercase()
            }
        }
        impl beatrix::mongo::MongoModel for #struct_name {
            const COLLECTION_NAME: &'static str = #entity_name;
            fn set_id(&mut self, id: beatrix::mongodb::bson::oid::ObjectId) {
                self.id = Some(id);
            }

            fn id(&self) -> Option<beatrix::mongodb::bson::oid::ObjectId> {
                self.id.clone()
            }
        }
    };
    gen.into()
}
