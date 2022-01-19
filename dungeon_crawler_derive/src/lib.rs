use proc_macro::TokenStream;
use quote::{quote, ToTokens, __private::Span};
use syn::{self, Data, Fields, Ident, Path, Type};

#[proc_macro_derive(Convar)]
pub fn convar_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_convar_macro(&ast)
}

fn impl_convar_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let Data::Struct(st) = &ast.data {
        if let Fields::Unnamed(fields) = &st.fields {
            let command_name = pascal_to_snake(name.to_string());
            if let Type::Path(path) = &fields
                .unnamed
                .iter()
                .next()
                .expect("Atleast one type expected")
                .ty
            {
                let ty = path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<String>();
                let ty_str = &Ident::new(ty.as_str(), Span::call_site());
                let gen = quote! {
                    impl Convar for #name {
                        type Item = #ty_str;
                        fn change(&mut self, item: Self::Item) {
                            self.0 = item;
                        }

                        fn command_name(&self) -> &'static str {
                            #command_name
                        }
                    }
                };

                return gen.into();
            }
        }
    }

    panic!("Only Tuple structs allowed.")
}

fn pascal_to_snake(pascal: String) -> String {
    let indices = pascal
        .match_indices(|char: char| char.is_uppercase())
        .skip(1)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();

    let mut snake = String::new();

    pascal.chars().enumerate().for_each(|(idx, char)| {
        if indices.contains(&idx) {
            snake.push('_');
        }
        snake.push(char.to_ascii_lowercase())
    });

    snake
}
