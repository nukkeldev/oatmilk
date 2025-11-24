use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, LitStr, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(SQLiteCompat, attributes(sqlite_compat))]
pub fn sqlite_compat_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut table_name = None;
    for attr in input.attrs {
        if attr.path().is_ident("sqlite_compat") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("table_name") {
                    let value: LitStr = meta.value()?.parse()?;
                    table_name = Some(value.value());
                }
                Ok(())
            });
        }
    }

    if table_name.is_none() {
        panic!("A SQL table name must be provided with #[sqlite_compat(table_name = \"...\")]!");
    }

    let fields = match input.data {
        Data::Struct(ref s) => &s.fields,
        _ => panic!("SQLiteCompat can only be derived for structs"),
    };

    let mut skip = 0;
    if let Some(Some(ident)) = fields.iter().nth(0).map(|f| &f.ident) {
        if ident.to_string() == "id" {
            skip = 1;
        }
    }

    let field_names: Vec<_> = fields
        .iter()
        .skip(skip)
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    let mut commad_field_names = Vec::new();
    let mut field_indices = Vec::new();

    for (i, f) in fields.iter().skip(skip).enumerate() {
        let name = f.ident.as_ref().unwrap().to_string();
        let index = LitStr::new(format!("${}", i + 1).as_str(), f.span());
        if i > 0 {
            commad_field_names.push(quote! { ", " });
            field_indices.push(quote! { ", " });
        }
        commad_field_names.push(quote! { #name });
        field_indices.push(quote! { #index });
    }

    let expanded = quote! {
        impl<'a> SQLiteCompat<'a> for #name {
            const TABLE_NAME: &'static str = #table_name;

            fn insert(self) -> Query<'a, Sqlite, <sqlx::Sqlite as sqlx::Database>::Arguments<'a>> {
                sqlx::query(concat!("INSERT INTO ", #table_name, "(", #( #commad_field_names ),*, ") VALUES (", #( #field_indices ),*, ")"))
                    #( .bind(self.#field_names) )*
            }
        }
    };

    TokenStream::from(expanded)
}
