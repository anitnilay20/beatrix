use beatrix_core::relational::{column::Column, sql::Sql};
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Fields, Lit, Meta};

fn get_field_name(field: &syn::Field) -> Option<String> {
    // Check for #[name = "..."] attribute
    for attr in &field.attrs {
        if attr.path().is_ident("name") {
            if let Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        return Some(lit_str.value());
                    }
                }
            }
        }
    }
    None
}

pub fn generate_fields<T: Sql>(token: &str, table_name: &'static str) -> Vec<Column<T>> {
    let ast: TokenStream = token.parse().expect("Invalid syntax");
    let ast: DeriveInput = syn::parse2(ast).expect("Invalid syntax");

    if let Data::Struct(data) = ast.data {
        if let Fields::Named(fields) = &data.fields {
            return fields
                .named
                .iter()
                .filter_map(|field| {
                    field.ident.as_ref().map(|ident| {
                        // Use custom name from attribute if present, otherwise use field name
                        let column_name = get_field_name(field)
                            .unwrap_or_else(|| ident.to_string());
                        let column_name = Box::leak(column_name.into_boxed_str());
                        Column::new(table_name, column_name, None)
                    })
                })
                .collect();
        }
    }

    Vec::new()
}
