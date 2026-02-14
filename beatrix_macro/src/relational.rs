use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Lit, Meta};

// --- 1. Helper Functions (Compile-Time Only) ---

/// Extracts a string value from a named attribute (e.g., #[name = "foo"])
fn get_attr_string(attrs: &[syn::Attribute], attr_ident: &str) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if attr.path().is_ident(attr_ident) {
            if let Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        return Some(lit_str.value());
                    }
                }
            }
        }
        None
    })
}

/// Grabs all named fields from the struct
fn get_struct_fields(ast: &DeriveInput) -> Vec<&Field> {
    if let Data::Struct(data) = &ast.data {
        if let Fields::Named(fields) = &data.fields {
            return fields.named.iter().collect();
        }
    }
    vec![]
}

fn get_table_name(ast: &DeriveInput) -> String {
    get_attr_string(&ast.attrs, "table_name").unwrap_or_else(|| ast.ident.to_string())
}

// --- 2. Code Generators ---

/// Generates the individual `pub fn id() -> Column<i32>` methods
fn gen_field_methods(ast: &DeriveInput, table_name: &str) -> TokenStream {
    let fields = get_struct_fields(ast);

    let field_fns = fields.into_iter().filter_map(|field| {
        let ident = field.ident.as_ref()?;
        let field_type = &field.ty;

        // Use custom name or fallback to the struct field's name
        let col_name = get_attr_string(&field.attrs, "name")
            .unwrap_or_else(|| ident.to_string());

        // We use quote! to generate hardcoded string literals (&'static str)
        Some(quote! {
            pub fn #ident() -> beatrix_core::relational::column::Column<#field_type> {
                beatrix_core::relational::column::Column::new(#table_name, #col_name, None)
            }
        })
    });

    quote! { #(#field_fns)* }
}

/// Generates the `fn columns()` method for the Table trait
fn gen_trait_columns_method(ast: &DeriveInput) -> TokenStream {
    let fields = get_struct_fields(ast);

    let col_instantiations = fields.into_iter().filter_map(|field| {
        let ident = field.ident.as_ref()?;
        // We reuse the generated field methods (e.g., Self::id()) to build the array
        Some(quote! {
            Box::new(Self::#ident()) as Box<dyn beatrix_core::relational::column::ColumnType>
        })
    });

    quote! {
        fn columns() -> Vec<Box<dyn beatrix_core::relational::column::ColumnType>> {
            vec![
                #(#col_instantiations),*
            ]
        }
    }
}

fn gen_from_row_impl(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = get_struct_fields(ast);

    let field_extracts = fields.into_iter().filter_map(|field| {
        let field_ident = field.ident.as_ref()?;
        let col_name = get_attr_string(&field.attrs, "name")
            .unwrap_or_else(|| field_ident.to_string());

        Some(quote! {
            #field_ident: row.get(#col_name)
        })
    });

    quote! {
        #[cfg(feature = "postgres")]
        impl<'r> beatrix_core::sqlx::FromRow<'r, beatrix_core::sqlx::postgres::PgRow> for #struct_name {
            fn from_row(row: &'r beatrix_core::sqlx::postgres::PgRow) -> Result<Self, beatrix_core::sqlx::Error> {
                use beatrix_core::sqlx::Row;
                Ok(Self {
                    #(#field_extracts),*
                })
            }
        }
    }
}

// --- 3. The Main Macro Entry Point ---

pub fn impl_relational_model(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let table_name = get_table_name(ast);

    let field_methods = gen_field_methods(ast, &table_name);
    let trait_columns_method = gen_trait_columns_method(ast);
    let from_row_impl = gen_from_row_impl(ast);

    quote! {
        use beatrix_core::relational::select::SelectType;

        impl beatrix_core::relational::table::Table for #struct_name {
            fn table_details() -> beatrix_core::relational::table::TableDetails {
                beatrix_core::relational::table::TableDetails {
                    name: #table_name.to_string(),
                    alias: None,
                }
            }

            // Insert the generated columns() function here
            #trait_columns_method

            fn select<DB: beatrix_core::sqlx::Database>() -> beatrix_core::relational::select::Select<Self, DB>
            where
                Self: Sized,
            {
                beatrix_core::relational::select::Select::new(Self::columns(), vec![Self::table_details()])
            }
        }

        impl #struct_name {
            // Insert the generated pub fn id(), pub fn name() here
            #field_methods
        }

        #from_row_impl
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import your impl_relational_model function
    use syn::parse_str;

    #[test]
    fn test_macro_expansion() {
        // 1. Define the input code exactly as a user would write it
        let input_code = r#"
            #[table_name = "users"]
            struct UserInsert {
                id: i32,
                #[name = "custom_name"]
                name: String,
            }
        "#;

        // 2. Parse the string into a syn::DeriveInput AST
        let ast = parse_str::<DeriveInput>(input_code).expect("Failed to parse input");

        // 3. Run your generator function
        let expanded_tokens = impl_relational_model(&ast);

        // 4. Convert the generated TokenStream back to a string
        let expanded_string = expanded_tokens.to_string();

        // Print it to the console so you can inspect it visually
        // Run with: cargo test -- --nocapture
        println!("=== GENERATED CODE ===");
        println!("{}", expanded_string);
        println!("======================");

        // Optional: Assert that specific strings exist in the output
        // Should generate generic SelectBuilder with Postgres as default DB
        let output = expanded_string.clone();
        assert!(output.contains("type SelectBuilder") && output.contains("Select"),
            "Generated output should contain SelectBuilder and Select");
        assert!(expanded_string.contains(r#"name : "users" . to_string ()"#));
    }
}