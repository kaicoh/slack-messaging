use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

mod types;

// The main struct we get from parsing the attributes
// Ref: https://github.com/TedDriggs/darling?tab=readme-ov-file#shape-validation
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
struct Target {
    ident: syn::Ident,
    // generics: syn::Generics,
    data: darling::ast::Data<(), types::Field>,
    validate: Option<darling::Result<syn::Path>>,
}

impl Target {
    fn validate(&self) -> Option<syn::Path> {
        match self.validate.clone().transpose() {
            Ok(v) => v,
            Err(err) => {
                abort! {
                    err.span(), "Invalid attribute #[builder(validate = ...)]";
                    note = "Invalid argument for `validate` attribute. Only paths are allowed.";
                    help = "Try formating the argument like `path::to::function` or `\"path::to::function\"`";
                }
            }
        }
    }

    fn into_token_strem(self) -> TokenStream {
        if self.data.is_enum() {
            panic!("enum is not supported")
        }

        let ident = &self.ident;
        let builder = format_ident!("{ident}Builder");
        let doc_builder_method = format!("constract [`{builder}`] object.");
        let doc_builder = format!("Builder for [`{ident}`] object.");
        let doc_build_method = format!("build [`{ident}`] object.");

        let validate_across_fields = match self.validate() {
            Some(path) => quote! { #path(self) },
            None => quote! { vec![] },
        };

        let fields = self.data.clone().take_struct().unwrap().fields;
        let merge_fn = format_ident!("merge_{}", fields.len() + 1);

        let field_names = fields.iter().map(types::Field::field_name);
        let field_names_0 = field_names.clone();
        let field_names_1 = field_names.clone();
        let field_names_2 = field_names.clone();
        let field_names_3 = field_names.clone();
        let field_names_4 = field_names.clone();
        let field_names_5 = field_names.clone();

        let builder_fields = fields.iter().map(types::Field::builder_field);
        let accessors = fields.iter().map(types::Field::builder_accessors);
        let default_fields = fields.iter().map(types::Field::default_field);

        quote! {
            impl #ident {
                #[doc = #doc_builder_method]
                pub fn builder() -> #builder {
                    #builder::default()
                }
            }

            #[doc = #doc_builder]
            #[derive(Debug)]
            pub struct #builder {
                #(#builder_fields,)*
            }

            impl #builder {
                #(#accessors)*

                fn validate_across_fields(&self) -> Vec<crate::errors::ValidationErrorKind> {
                    #validate_across_fields
                }

                #[doc = #doc_build_method]
                pub fn build(self) -> ::std::result::Result<#ident, crate::errors::ValidationErrors> {
                    let v0 = crate::value::Value::<()> {
                        inner: None,
                        errors: self.validate_across_fields(),
                    };

                    let Self { #(#field_names_0),* } = self;
                    crate::value::#merge_fn(v0, #(#field_names_1),*)
                        .map(|(_, #(#field_names_2),*)| #ident {
                            #(#field_names_3),*
                        })
                        .map_err(|(e_0, #(#field_names_4),*)| {
                            let errors = [
                                #(crate::errors::ValidationError::new_single_field(stringify!(#field_names_5), #field_names_5)),*
                            ]
                            .into_iter()
                            .filter_map(|v| v)
                            .chain([crate::errors::ValidationError::new_across_fields(e_0)]
                                .into_iter()
                                .filter_map(|v| v)
                            )
                            .collect();

                            crate::errors::ValidationErrors {
                                object: ::std::borrow::Cow::Borrowed(stringify!(#ident)),
                                errors,
                            }
                        })
                }
            }

            impl Default for #builder {
                fn default() -> Self {
                    Self {
                        #(#default_fields,)*
                    }
                }
            }
        }
        .into()
    }
}

#[proc_macro_error]
#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    Target::from_derive_input(&input)
        .map(Target::into_token_strem)
        .unwrap_or_else(|e| e.write_errors().into())
}
